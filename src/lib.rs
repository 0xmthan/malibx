//! malibx: a tiny window/pixel library with a MiniLibX-style API, usable from Python.

use std::cell::RefCell;

use minifb::{Window as MfbWindow, WindowOptions};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyMemoryView};

/// Bytes per pixel in images: 32 bits, little endian (B, G, R, A in memory),
/// like MiniLibX on a normal 32-bit display.
const BPP: usize = 4;

/// One open window and its pixel buffer (0x00RRGGBB per pixel).
struct WinState {
    window: MfbWindow,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

struct Inner {
    windows: Vec<Option<WinState>>,
    loop_hook: Option<(Py<PyAny>, Py<PyAny>)>,
    end_loop: bool,
}

impl Inner {
    fn win(&mut self, win: &Window) -> PyResult<&mut WinState> {
        self.windows
            .get_mut(win.id)
            .and_then(Option::as_mut)
            .ok_or_else(|| PyRuntimeError::new_err("window is closed"))
    }
}

/// Connection handle, like the `void *mlx_ptr` returned by `mlx_init`.
#[pyclass(unsendable)]
struct Mlx {
    inner: RefCell<Inner>,
}

/// Window handle, like the `void *win_ptr` returned by `mlx_new_window`.
#[pyclass(frozen)]
struct Window {
    id: usize,
}

/// Image handle, like the `void *img_ptr` returned by `mlx_new_image`.
/// The pixels live in a Python bytearray so `get_data_addr` can hand out a
/// writable memoryview over them.
#[pyclass]
struct Image {
    data: Option<Py<PyByteArray>>,
    width: usize,
    height: usize,
}

impl Image {
    fn data(&self) -> PyResult<&Py<PyByteArray>> {
        self.data
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("image is destroyed"))
    }
}

/// mlx_init
#[pyfunction]
fn init() -> Mlx {
    Mlx {
        inner: RefCell::new(Inner {
            windows: Vec::new(),
            loop_hook: None,
            end_loop: false,
        }),
    }
}

/// mlx_new_window
#[pyfunction]
fn new_window(mlx: PyRef<'_, Mlx>, size_x: usize, size_y: usize, title: &str) -> PyResult<Window> {
    let mut window = MfbWindow::new(title, size_x, size_y, WindowOptions::default())
        .map_err(|e| PyRuntimeError::new_err(format!("new_window failed: {e}")))?;
    window.set_target_fps(60);
    let mut inner = mlx.inner.borrow_mut();
    inner.windows.push(Some(WinState {
        window,
        buffer: vec![0; size_x * size_y],
        width: size_x,
        height: size_y,
    }));
    Ok(Window {
        id: inner.windows.len() - 1,
    })
}

/// mlx_clear_window: fills the window with black.
#[pyfunction]
fn clear_window(mlx: PyRef<'_, Mlx>, win: PyRef<'_, Window>) -> PyResult<()> {
    mlx.inner.borrow_mut().win(&win)?.buffer.fill(0);
    Ok(())
}

/// mlx_pixel_put: color is 0xRRGGBB. Pixels outside the window are ignored.
#[pyfunction]
fn pixel_put(
    mlx: PyRef<'_, Mlx>,
    win: PyRef<'_, Window>,
    x: i64,
    y: i64,
    color: i64,
) -> PyResult<()> {
    let mut inner = mlx.inner.borrow_mut();
    let w = inner.win(&win)?;
    if (0..w.width as i64).contains(&x) && (0..w.height as i64).contains(&y) {
        w.buffer[y as usize * w.width + x as usize] = color as u32 & 0x00FF_FFFF;
    }
    Ok(())
}

/// mlx_new_image: a black width x height image.
#[pyfunction]
fn new_image(mlx: PyRef<'_, Mlx>, py: Python<'_>, width: usize, height: usize) -> PyResult<Image> {
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
#[pyfunction]
fn get_data_addr<'py>(
    img: PyRef<'py, Image>,
    py: Python<'py>,
) -> PyResult<(Bound<'py, PyMemoryView>, usize, usize, u8)> {
    let view = PyMemoryView::from(img.data()?.bind(py).as_any())?;
    Ok((view, BPP * 8, img.width * BPP, 0))
}

/// mlx_put_image_to_window: copies the image into the window at (x, y),
/// clipped to the window edges.
#[pyfunction]
fn put_image_to_window(
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

/// Copies a BGRA image into a 0x00RRGGBB buffer at (x, y), clipped to its edges.
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
            *px = u32::from_le_bytes([bgra[0], bgra[1], bgra[2], 0]);
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
                v.extend_from_slice(&[(x + 1) as u8, (y + 1) as u8, 0, 0xFF]);
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
    fn blit_ignores_alpha_and_keeps_rgb() {
        let mut dst = vec![0u32; 1];
        blit(&mut dst, 1, 1, &[0x56, 0x34, 0x12, 0xFF], 1, 1, 0, 0);
        assert_eq!(dst[0], 0x0012_3456);
    }
}

/// mlx_destroy_image: frees the image. Using it afterwards raises an error.
#[pyfunction]
fn destroy_image(mlx: PyRef<'_, Mlx>, mut img: PyRefMut<'_, Image>) {
    let _ = mlx; // kept for the MiniLibX signature
    img.data = None;
}

/// mlx_loop_hook: `f(param)` is called once per frame.
#[pyfunction]
#[pyo3(signature = (mlx, f, param=None))]
fn loop_hook(mlx: PyRef<'_, Mlx>, py: Python<'_>, f: Py<PyAny>, param: Option<Py<PyAny>>) {
    let param = param.unwrap_or_else(|| py.None());
    mlx.inner.borrow_mut().loop_hook = Some((f, param));
}

/// mlx_loop_end: makes `loop` return after the current frame.
#[pyfunction]
fn loop_end(mlx: PyRef<'_, Mlx>) {
    mlx.inner.borrow_mut().end_loop = true;
}

/// mlx_loop: runs until loop_end is called or every window is closed.
#[pyfunction]
#[pyo3(name = "loop")]
fn run_loop(mlx: PyRef<'_, Mlx>, py: Python<'_>) -> PyResult<()> {
    mlx.inner.borrow_mut().end_loop = false;
    loop {
        // Call the loop hook first so what it draws shows up this frame.
        // The borrow is released first, so the hook can call malibx itself.
        let hook = mlx
            .inner
            .borrow()
            .loop_hook
            .as_ref()
            .map(|(f, p)| (f.clone_ref(py), p.clone_ref(py)));
        if let Some((f, p)) = hook {
            f.call1(py, (p,))?;
        }
        py.check_signals()?;

        // Present each window and pump its events; drop the ones the user closed.
        let mut inner = mlx.inner.borrow_mut();
        for slot in inner.windows.iter_mut() {
            if let Some(w) = slot {
                if !w.window.is_open() {
                    *slot = None;
                    continue;
                }
                w.window
                    .update_with_buffer(&w.buffer, w.width, w.height)
                    .map_err(|e| PyRuntimeError::new_err(format!("loop failed: {e}")))?;
            }
        }
        if inner.end_loop || inner.windows.iter().all(Option::is_none) {
            return Ok(());
        }
    }
}

#[pymodule]
fn malibx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mlx>()?;
    m.add_class::<Window>()?;
    m.add_class::<Image>()?;
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(new_window, m)?)?;
    m.add_function(wrap_pyfunction!(clear_window, m)?)?;
    m.add_function(wrap_pyfunction!(pixel_put, m)?)?;
    m.add_function(wrap_pyfunction!(new_image, m)?)?;
    m.add_function(wrap_pyfunction!(get_data_addr, m)?)?;
    m.add_function(wrap_pyfunction!(put_image_to_window, m)?)?;
    m.add_function(wrap_pyfunction!(destroy_image, m)?)?;
    m.add_function(wrap_pyfunction!(loop_hook, m)?)?;
    m.add_function(wrap_pyfunction!(loop_end, m)?)?;
    m.add_function(wrap_pyfunction!(run_loop, m)?)?;
    Ok(())
}
