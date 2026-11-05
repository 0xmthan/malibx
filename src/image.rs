//! Off-screen images (`man mlx_new_image`).

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyMemoryView};

use crate::mlx::Mlx;
use crate::window::Window;

/// Bytes per pixel in images: 32 bits, little endian (B, G, R, A in memory),
/// like MiniLibX on a normal 32-bit display.
const BPP: usize = 4;

/// Image handle, like the `void *img_ptr` returned by `mlx_new_image`.
/// The pixels live in a Python bytearray so `get_data_addr` can hand out a
/// writable memoryview over them.
#[pyclass]
pub(crate) struct Image {
    data: Option<Py<PyByteArray>>,
    width: usize,
    height: usize,
}

impl Image {
    /// An image filled with the given BGRA pixels (used by the XPM loader).
    pub(crate) fn from_bgra(py: Python<'_>, width: usize, height: usize, pixels: &[u8]) -> Image {
        Image {
            data: Some(PyByteArray::new(py, pixels).unbind()),
            width,
            height,
        }
    }

    fn data(&self) -> PyResult<&Py<PyByteArray>> {
        self.data
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("image is destroyed"))
    }
}

/// mlx_new_image: a black width x height image.
///
/// Docs: https://0xmthan.github.io/malibx/reference/new_image/
#[pyfunction]
pub(crate) fn new_image(
    mlx: PyRef<'_, Mlx>,
    py: Python<'_>,
    width: usize,
    height: usize,
) -> PyResult<Image> {
    let _ = mlx; // kept for the MiniLibX signature
    if width == 0 || height == 0 {
        return Err(PyValueError::new_err("image size must be at least 1x1"));
    }
    let data = PyByteArray::new(py, &vec![0u8; width * height * BPP]);
    Ok(Image {
        data: Some(data.unbind()),
        width,
        height,
    })
}

/// mlx_get_data_addr: returns (buffer, bits_per_pixel, size_line, endian).
/// `buffer` is a writable memoryview; pixel (x, y) starts at
/// `y * size_line + x * (bits_per_pixel // 8)` and is stored as B, G, R, A.
///
/// Docs: https://0xmthan.github.io/malibx/reference/get_data_addr/
#[pyfunction]
pub(crate) fn get_data_addr<'py>(
    img: PyRef<'py, Image>,
    py: Python<'py>,
) -> PyResult<(Bound<'py, PyMemoryView>, usize, usize, u8)> {
    let view = PyMemoryView::from(img.data()?.bind(py).as_any())?;
    Ok((view, BPP * 8, img.width * BPP, 0))
}

/// mlx_put_image_to_window: copies the image into the window at (x, y),
/// clipped to the window edges. Pixels with alpha 0xFF are transparent.
///
/// Docs: https://0xmthan.github.io/malibx/reference/put_image_to_window/
#[pyfunction]
pub(crate) fn put_image_to_window(
    mlx: PyRef<'_, Mlx>,
    py: Python<'_>,
    win: PyRef<'_, Window>,
    img: PyRef<'_, Image>,
    x: i64,
    y: i64,
) -> PyResult<()> {
    let mut inner = mlx.inner.borrow_mut();
    let w = inner.win(&win)?;
    let data = img.data()?.bind(py);
    // SAFETY: no Python code runs while we hold this slice, so the bytearray
    // can't be resized or freed under us.
    let bytes = unsafe { data.as_bytes() };
    blit(
        &mut w.buffer,
        w.width,
        w.height,
        bytes,
        img.width,
        img.height,
        x,
        y,
    );
    Ok(())
}

/// mlx_destroy_image: frees the image. Using it afterwards raises an error.
///
/// Docs: https://0xmthan.github.io/malibx/reference/destroy_image/
#[pyfunction]
pub(crate) fn destroy_image(mlx: PyRef<'_, Mlx>, mut img: PyRefMut<'_, Image>) {
    let _ = mlx; // kept for the MiniLibX signature
    img.data = None;
}

/// Copies a BGRA image into a 0x00RRGGBB buffer at (x, y), clipped to its edges.
/// Alpha 0xFF means transparent (MiniLibX's convention); anything else is opaque.
#[allow(clippy::too_many_arguments)]
fn blit(dst: &mut [u32], dw: usize, dh: usize, src: &[u8], sw: usize, sh: usize, x: i64, y: i64) {
    let (iw, ih) = (sw as i64, sh as i64);
    let (ww, wh) = (dw as i64, dh as i64);
    let (x0, x1) = (x.max(0), (x + iw).min(ww));
    if x0 >= x1 {
        return;
    }
    let n = (x1 - x0) as usize;
    for row in y.max(0) - y..(wh - y).min(ih) {
        let s = ((row * iw + x0 - x) as usize) * BPP;
        let d = ((y + row) * ww + x0) as usize;
        for (px, bgra) in dst[d..d + n]
            .iter_mut()
            .zip(src[s..s + n * BPP].as_chunks::<BPP>().0)
        {
            if bgra[3] != 0xFF {
                *px = u32::from_le_bytes([bgra[0], bgra[1], bgra[2], 0]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sw x sh image where pixel (x, y) has color 0x00_00_yy_xx (+1 so none is 0).
    fn image(sw: usize, sh: usize) -> Vec<u8> {
        let mut v = Vec::new();
        for y in 0..sh {
            for x in 0..sw {
                v.extend_from_slice(&[(x + 1) as u8, (y + 1) as u8, 0, 0]);
            }
        }
        v
    }

    /// Reference implementation: per-pixel bounds check.
    fn naive(dw: usize, dh: usize, sw: usize, sh: usize, x: i64, y: i64) -> Vec<u32> {
        let src = image(sw, sh);
        let mut dst = vec![0u32; dw * dh];
        for iy in 0..sh as i64 {
            for ix in 0..sw as i64 {
                let (wx, wy) = (x + ix, y + iy);
                if (0..dw as i64).contains(&wx) && (0..dh as i64).contains(&wy) {
                    let i = ((iy * sw as i64 + ix) * 4) as usize;
                    dst[wy as usize * dw + wx as usize] =
                        u32::from_le_bytes([src[i], src[i + 1], src[i + 2], 0]);
                }
            }
        }
        dst
    }

    #[test]
    fn blit_matches_naive_at_every_offset() {
        let (dw, dh, sw, sh) = (7, 5, 3, 4);
        for y in -6..8 {
            for x in -6..10 {
                let mut dst = vec![0u32; dw * dh];
                blit(&mut dst, dw, dh, &image(sw, sh), sw, sh, x, y);
                assert_eq!(dst, naive(dw, dh, sw, sh, x, y), "at ({x}, {y})");
            }
        }
    }

    #[test]
    fn blit_alpha_ff_is_transparent_anything_else_opaque() {
        let mut dst = vec![7u32; 3];
        let src = [
            0x56, 0x34, 0x12, 0x00, 0x56, 0x34, 0x12, 0x80, 0x56, 0x34, 0x12, 0xFF,
        ];
        blit(&mut dst, 3, 1, &src, 3, 1, 0, 0);
        assert_eq!(dst, [0x0012_3456, 0x0012_3456, 7]);
    }
}
