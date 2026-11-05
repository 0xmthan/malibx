//! XPM image loading (`man mlx_new_image`, "XPM IMAGES"), following
//! MiniLibX's own parser in `mlx_xpm.c`:
//! - colors come from the `c` key: `#RRGGBB`, `None`, or an X11 color name;
//! - `None` pixels are stored as 0xFF000000 (alpha 0xFF = transparent);
//! - unknown colors and unknown pixel keys become black, like MiniLibX.

use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::image::Image;
use crate::mlx::Mlx;
use crate::xpm_colors::COLORS;

/// Keys that can follow a color in an XPM color line.
const KEYS: [&str; 5] = ["c", "m", "s", "g", "g4"];

const TRANSPARENT: [u8; 4] = [0, 0, 0, 0xFF];

/// Parses XPM data (the contents of its quoted strings) into BGRA pixels.
fn parse(lines: &[String]) -> Result<(usize, usize, Vec<u8>), String> {
    let mut lines = lines.iter();
    let mut next = |what: &str| lines.next().ok_or_else(|| format!("xpm: missing {what}"));

    let header: Vec<usize> = next("header")?
        .split_whitespace()
        .take(4)
        .map(|n| n.parse().unwrap_or(0))
        .collect();
    let [w, h, nc, cpp] = header[..] else {
        return Err("xpm: header needs `width height colors chars_per_pixel`".into());
    };
    if w == 0 || h == 0 || nc == 0 || cpp == 0 {
        return Err("xpm: width, height, colors and chars_per_pixel must be > 0".into());
    }

    let mut palette: HashMap<&[u8], [u8; 4]> = HashMap::with_capacity(nc);
    for _ in 0..nc {
        let line = next("color line")?;
        let key = line
            .as_bytes()
            .get(..cpp)
            .ok_or_else(|| format!("xpm: color line too short: {line:?}"))?;
        palette.insert(key, color_value(&line[cpp..]));
    }

    let mut pixels = Vec::with_capacity(w * h * 4);
    for _ in 0..h {
        let row = next("pixel row")?.as_bytes();
        if row.len() < w * cpp {
            return Err(format!("xpm: pixel row shorter than {w} pixels"));
        }
        for key in row.chunks_exact(cpp).take(w) {
            pixels.extend_from_slice(palette.get(key).unwrap_or(&[0, 0, 0, 0]));
        }
    }
    Ok((w, h, pixels))
}

/// The BGRA value of an XPM color line (without its pixel key).
fn color_value(spec: &str) -> [u8; 4] {
    let tokens: Vec<&str> = spec.split_whitespace().collect();
    let Some(start) = tokens.iter().position(|t| *t == "c") else {
        return [0, 0, 0, 0];
    };
    let name: Vec<&str> = tokens[start + 1..]
        .iter()
        .take_while(|t| !KEYS.contains(t))
        .copied()
        .collect();
    let name = name.join(" ");
    if name.eq_ignore_ascii_case("none") {
        return TRANSPARENT;
    }
    let rgb = match name.strip_prefix('#') {
        Some(hex) => parse_hex(hex),
        None => COLORS
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(&name))
            .map(|(_, c)| *c),
    }
    .unwrap_or(0);
    let [b, g, r, _] = rgb.to_le_bytes();
    [b, g, r, 0]
}

/// `#RGB`, `#RRGGBB` or `#RRRRGGGGBBBB` -> 0xRRGGBB.
fn parse_hex(hex: &str) -> Option<u32> {
    let per = match hex.len() {
        3 | 6 | 12 => hex.len() / 3,
        _ => return None,
    };
    let mut rgb = 0;
    for i in 0..3 {
        let part = u32::from_str_radix(&hex[i * per..(i + 1) * per], 16).ok()?;
        let byte = match per {
            1 => part * 0x11,
            2 => part,
            _ => part >> 8,
        };
        rgb = rgb << 8 | byte;
    }
    Some(rgb)
}

/// The quoted strings of an XPM file, with `/* */` and `//` comments removed.
fn strings(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => out.push(chars.by_ref().take_while(|&c| c != '"').collect()),
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut prev = ' ';
                for c in chars.by_ref() {
                    if prev == '*' && c == '/' {
                        break;
                    }
                    prev = c;
                }
            }
            '/' if chars.peek() == Some(&'/') => {
                chars.by_ref().find(|&c| c == '\n');
            }
            _ => {}
        }
    }
    out
}

fn load(py: Python<'_>, lines: &[String]) -> PyResult<(Image, usize, usize)> {
    let (w, h, pixels) = parse(lines).map_err(PyValueError::new_err)?;
    Ok((Image::from_bgra(py, w, h, &pixels), w, h))
}

/// mlx_xpm_to_image: XPM data (its strings, as a list) -> (image, width, height).
#[pyfunction]
pub(crate) fn xpm_to_image(
    mlx: PyRef<'_, Mlx>,
    py: Python<'_>,
    xpm_data: Vec<String>,
) -> PyResult<(Image, usize, usize)> {
    let _ = mlx; // kept for the MiniLibX signature
    load(py, &xpm_data)
}

/// mlx_xpm_file_to_image: an `.xpm` file -> (image, width, height).
#[pyfunction]
pub(crate) fn xpm_file_to_image(
    mlx: PyRef<'_, Mlx>,
    py: Python<'_>,
    filename: std::path::PathBuf,
) -> PyResult<(Image, usize, usize)> {
    let _ = mlx; // kept for the MiniLibX signature
    let bytes = std::fs::read(&filename)?;
    load(py, &strings(&String::from_utf8_lossy(&bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_colors_names_and_none() {
        let (w, h, px) = parse(&lines(&[
            "3 2 4 1",
            "r c #FF0000",
            "g c lime green",
            ". c None",
            "b c Blue m black",
            "rg.",
            "b?r",
        ]))
        .unwrap();
        assert_eq!((w, h), (3, 2));
        let px: Vec<[u8; 4]> = px.chunks(4).map(|c| c.try_into().unwrap()).collect();
        assert_eq!(px[0], [0x00, 0x00, 0xFF, 0]); // red
        assert_eq!(px[1], [0x32, 0xCD, 0x32, 0]); // lime green = 0x32cd32
        assert_eq!(px[2], TRANSPARENT);
        assert_eq!(px[3], [0xFF, 0x00, 0x00, 0]); // blue, `m black` ignored
        assert_eq!(px[4], [0, 0, 0, 0]); // unknown key -> black
        assert_eq!(px[5], [0x00, 0x00, 0xFF, 0]);
    }

    #[test]
    fn two_chars_per_pixel() {
        let (_, _, px) = parse(&lines(&["2 1 2 2", "aa c #010203", "bb c #ABC", "bbaa"])).unwrap();
        assert_eq!(px, [0xCC, 0xBB, 0xAA, 0, 0x03, 0x02, 0x01, 0]);
    }

    #[test]
    fn bad_input_is_an_error() {
        assert!(parse(&lines(&[])).is_err());
        assert!(parse(&lines(&["2 2 1"])).is_err());
        assert!(parse(&lines(&["2 1 1 1", "a c #000000", "a"])).is_err());
        assert!(parse(&lines(&["1 2 1 1", "a c #000000", "a"])).is_err());
    }

    #[test]
    fn strings_skips_comments() {
        let text = "/* XPM */\nstatic char *x[] = {\n// \"no\"\n\"1 1 1 1\", /* \"no\" */\n\"a c #000000\",\n\"a\"};";
        assert_eq!(strings(text), ["1 1 1 1", "a c #000000", "a"]);
    }
}
