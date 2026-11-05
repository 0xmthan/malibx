//! Text drawing (`man mlx_string_put`, `mlx_set_font`), with the public
//! domain 8x8 font from the `font8x8` crate, optionally scaled up.

use font8x8::{BASIC_FONTS, LATIN_FONTS, UnicodeFonts};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::mlx::Mlx;
use crate::window::Window;

/// Glyph size in pixels. Rows 0..ASCENT sit above the baseline.
const GLYPH: i64 = 8;
const ASCENT: i64 = 7;

/// Largest scale `set_font` accepts ("64x64").
const MAX_SCALE: i64 = 8;

/// Draws `text` into a 0x00RRGGBB buffer, each font pixel as a
/// `scale` x `scale` block. Like X11, `y` is the baseline.
#[allow(clippy::too_many_arguments)]
fn draw_text(
    buf: &mut [u32],
    w: usize,
    h: usize,
    x: i64,
    y: i64,
    color: u32,
    text: &str,
    scale: i64,
) {
    let top = y - ASCENT * scale;
    for (i, ch) in text.chars().enumerate() {
        let Some(glyph) = BASIC_FONTS.get(ch).or_else(|| LATIN_FONTS.get(ch)) else {
            continue;
        };
        let left = x + i as i64 * GLYPH * scale;
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..GLYPH {
                if bits >> col & 1 == 0 {
                    continue;
                }
                for dy in 0..scale {
                    let py = top + row as i64 * scale + dy;
                    if !(0..h as i64).contains(&py) {
                        continue;
                    }
                    for dx in 0..scale {
                        let px = left + col * scale + dx;
                        if (0..w as i64).contains(&px) {
                            buf[py as usize * w + px as usize] = color;
                        }
                    }
                }
            }
        }
    }
}

/// Parses a font name "NxN" where N is 8, 16, ... 64 into a scale.
fn parse_font(name: &str) -> Option<i64> {
    let (a, b) = name.split_once('x')?;
    let n: i64 = a.parse().ok()?;
    (a == b && n % GLYPH == 0 && (1..=MAX_SCALE).contains(&(n / GLYPH))).then_some(n / GLYPH)
}

/// mlx_set_font: picks the text size for `string_put` in this window.
/// Only the built-in font exists, so `name` is its size: "8x8" (default),
/// "16x16", "24x24", ... up to "64x64".
#[pyfunction]
pub(crate) fn set_font(mlx: PyRef<'_, Mlx>, win: PyRef<'_, Window>, name: &str) -> PyResult<()> {
    let scale = parse_font(name).ok_or_else(|| {
        PyValueError::new_err(format!(
            "unknown font {name:?}: use \"8x8\", \"16x16\", ... up to \"64x64\""
        ))
    })?;
    mlx.inner.borrow_mut().win(&win)?.font_scale = scale;
    Ok(())
}

/// mlx_string_put: draws `string` at (x, y) in `color` (0xRRGGBB).
/// `y` is the text baseline, like MiniLibX. Each character is 8x8 pixels,
/// or bigger after `set_font`.
#[pyfunction]
pub(crate) fn string_put(
    mlx: PyRef<'_, Mlx>,
    win: PyRef<'_, Window>,
    x: i64,
    y: i64,
    color: i64,
    string: &str,
) -> PyResult<()> {
    let mut inner = mlx.inner.borrow_mut();
    let w = inner.win(&win)?;
    let color = color as u32 & 0x00FF_FFFF;
    let scale = w.font_scale;
    draw_text(&mut w.buffer, w.width, w.height, x, y, color, string, scale);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(text: &str, w: usize, h: usize, x: i64, y: i64) -> Vec<String> {
        render_scaled(text, w, h, x, y, 1)
    }

    fn render_scaled(text: &str, w: usize, h: usize, x: i64, y: i64, s: i64) -> Vec<String> {
        let mut buf = vec![0; w * h];
        draw_text(&mut buf, w, h, x, y, 1, text, s);
        buf.chunks(w)
            .map(|r| r.iter().map(|&p| if p == 1 { '#' } else { '.' }).collect())
            .collect()
    }

    #[test]
    fn baseline_is_y() {
        // Like X11, glyphs sit on top of the baseline: the foot of 'L' is row y - 1.
        let rows = render("L", 8, 10, 0, 8);
        assert_eq!(rows[1], "####....");
        assert_eq!(rows[7], "#######.");
        assert_eq!(rows[8], "........");
    }

    #[test]
    fn clips_and_advances() {
        // The first L is off-screen to the left; the second one starts at x + 8 = 4.
        let rows = render("LL", 12, 4, -4, 7);
        assert_eq!(rows[0], "....####....");
        assert!(render("L", 4, 4, -100, -100).iter().all(|r| r == "...."));
    }

    #[test]
    fn scales_glyphs() {
        // At 2x the foot of 'L' is two rows above the baseline, 14 px wide.
        let rows = render_scaled("LL", 32, 18, 0, 16, 2);
        assert_eq!(
            rows[14],
            format!("{}..{}..", "#".repeat(14), "#".repeat(14))
        );
        assert_eq!(rows[15], rows[14]);
        assert_eq!(rows[16], ".".repeat(32));
        assert_eq!(
            rows[2],
            format!(
                "{}{}{}{}",
                "#".repeat(8),
                ".".repeat(8),
                "#".repeat(8),
                ".".repeat(8)
            )
        );
    }

    #[test]
    fn font_names() {
        assert_eq!(parse_font("8x8"), Some(1));
        assert_eq!(parse_font("16x16"), Some(2));
        assert_eq!(parse_font("64x64"), Some(8));
        for bad in ["", "8", "16x8", "12x12", "0x0", "72x72", "-8x-8", "fixed"] {
            assert_eq!(parse_font(bad), None, "{bad}");
        }
    }
}
