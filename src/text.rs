//! Text drawing (`man mlx_pixel_put`, `mlx_string_put`), with the public
//! domain 8x8 font from the `font8x8` crate.

use font8x8::{BASIC_FONTS, LATIN_FONTS, UnicodeFonts};
use pyo3::prelude::*;

use crate::mlx::Mlx;
use crate::window::Window;

/// Glyph size in pixels. Rows 0..ASCENT sit above the baseline.
const GLYPH: i64 = 8;
const ASCENT: i64 = 7;

/// Draws `text` into a 0x00RRGGBB buffer. Like X11, `y` is the baseline.
fn draw_text(buf: &mut [u32], w: usize, h: usize, x: i64, y: i64, color: u32, text: &str) {
    let top = y - ASCENT;
    for (i, ch) in text.chars().enumerate() {
        let Some(glyph) = BASIC_FONTS.get(ch).or_else(|| LATIN_FONTS.get(ch)) else {
            continue;
        };
        let left = x + i as i64 * GLYPH;
        for (row, bits) in glyph.iter().enumerate() {
            let py = top + row as i64;
            if !(0..h as i64).contains(&py) {
                continue;
            }
            for col in 0..GLYPH {
                let px = left + col;
                if bits >> col & 1 == 1 && (0..w as i64).contains(&px) {
                    buf[py as usize * w + px as usize] = color;
                }
            }
        }
    }
}

/// mlx_string_put: draws `string` at (x, y) in `color` (0xRRGGBB).
/// `y` is the text baseline, like MiniLibX. Each character is 8x8 pixels.
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
    draw_text(&mut w.buffer, w.width, w.height, x, y, color, string);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(text: &str, w: usize, h: usize, x: i64, y: i64) -> Vec<String> {
        let mut buf = vec![0; w * h];
        draw_text(&mut buf, w, h, x, y, 1, text);
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
}
