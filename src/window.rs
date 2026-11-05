//! Windows and drawing straight into them (`man mlx_new_window`, `man mlx_pixel_put`).

use std::collections::HashMap;

use minifb::{Window as MfbWindow, WindowOptions};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::events::{Hook, InputState};
use crate::mlx::Mlx;

/// One open window and its pixel buffer (0x00RRGGBB per pixel).
pub(crate) struct WinState {
    pub(crate) window: MfbWindow,
    pub(crate) buffer: Vec<u32>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) hooks: HashMap<i32, Hook>,
    pub(crate) input: InputState,
}

impl WinState {
    /// Shows the buffer on screen and pumps the window's events.
    pub(crate) fn present(&mut self) -> PyResult<()> {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .map_err(|e| PyRuntimeError::new_err(format!("loop failed: {e}")))
    }
}

/// Window handle, like the `void *win_ptr` returned by `mlx_new_window`.
/// It keeps its `Mlx` so hooks can take just the window, like MiniLibX.
#[pyclass(frozen)]
pub(crate) struct Window {
    pub(crate) id: usize,
    pub(crate) mlx: Py<Mlx>,
}

/// mlx_new_window
#[pyfunction]
pub(crate) fn new_window(
    mlx: &Bound<'_, Mlx>,
    size_x: usize,
    size_y: usize,
    title: &str,
) -> PyResult<Window> {
    let mut window = MfbWindow::new(title, size_x, size_y, WindowOptions::default())
        .map_err(|e| PyRuntimeError::new_err(format!("new_window failed: {e}")))?;
    window.set_target_fps(60);
    crate::icon::apply(&mut window);
    let state = mlx.borrow();
    let mut inner = state.inner.borrow_mut();
    inner.windows.push(Some(WinState {
        window,
        buffer: vec![0; size_x * size_y],
        width: size_x,
        height: size_y,
        hooks: HashMap::new(),
        input: InputState::default(),
    }));
    Ok(Window {
        id: inner.windows.len() - 1,
        mlx: mlx.clone().unbind(),
    })
}

/// mlx_clear_window: fills the window with black.
#[pyfunction]
pub(crate) fn clear_window(mlx: PyRef<'_, Mlx>, win: PyRef<'_, Window>) -> PyResult<()> {
    mlx.inner.borrow_mut().win(&win)?.buffer.fill(0);
    Ok(())
}

/// mlx_pixel_put: color is 0xRRGGBB. Pixels outside the window are ignored.
#[pyfunction]
pub(crate) fn pixel_put(
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
