//! The pointer (`man mlx`, mlx_mouse_* in mlx.h): read its position, move,
//! hide and show it.

use minifb::MouseMode;
use pyo3::prelude::*;

use crate::mlx::Mlx;
use crate::window::Window;

/// mlx_mouse_get_pos: the pointer's (x, y) relative to the window's top-left.
/// Outside the window the values can be negative or past the edges; (0, 0)
/// if the position isn't known.
///
/// Docs: https://0xmthan.github.io/malibx/reference/mouse_get_pos/
#[pyfunction]
pub(crate) fn mouse_get_pos(mlx: PyRef<'_, Mlx>, win: PyRef<'_, Window>) -> PyResult<(i64, i64)> {
    let mut inner = mlx.inner.borrow_mut();
    let w = inner.win(&win)?;
    Ok(w.window
        .get_mouse_pos(MouseMode::Pass)
        .map_or((0, 0), |(x, y)| (x.floor() as i64, y.floor() as i64)))
}

/// mlx_mouse_move: moves the pointer to (x, y) in the window. Does nothing on
/// Wayland, which doesn't allow it.
///
/// Docs: https://0xmthan.github.io/malibx/reference/mouse_move/
#[pyfunction]
pub(crate) fn mouse_move(
    mlx: PyRef<'_, Mlx>,
    win: PyRef<'_, Window>,
    x: i64,
    y: i64,
) -> PyResult<()> {
    let mut inner = mlx.inner.borrow_mut();
    let w = inner.win(&win)?;
    crate::platform::warp_pointer(&w.window, x, y);
    Ok(())
}

/// mlx_mouse_hide: hides the pointer while it's over the window.
///
/// Docs: https://0xmthan.github.io/malibx/reference/mouse_hide/
#[pyfunction]
pub(crate) fn mouse_hide(mlx: PyRef<'_, Mlx>, win: PyRef<'_, Window>) -> PyResult<()> {
    set_cursor_hidden(&mlx, &win, true)
}

/// mlx_mouse_show: shows the pointer again after `mouse_hide`.
///
/// Docs: https://0xmthan.github.io/malibx/reference/mouse_show/
#[pyfunction]
pub(crate) fn mouse_show(mlx: PyRef<'_, Mlx>, win: PyRef<'_, Window>) -> PyResult<()> {
    set_cursor_hidden(&mlx, &win, false)
}

fn set_cursor_hidden(mlx: &Mlx, win: &Window, hidden: bool) -> PyResult<()> {
    let mut inner = mlx.inner.borrow_mut();
    let w = inner.win(win)?;
    // Only on a change: Windows counts hide/show calls, so repeating one
    // would need as many of the other to undo.
    if w.cursor_hidden != hidden {
        w.window.set_cursor_visibility(!hidden);
        w.cursor_hidden = hidden;
    }
    Ok(())
}
