//! The connection handle and the main loop (`man mlx`, `man mlx_loop`).

use std::cell::RefCell;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::events::Pending;
use crate::window::{WinState, Window};

pub(crate) struct Inner {
    pub(crate) windows: Vec<Option<WinState>>,
    loop_hook: Option<(Py<PyAny>, Py<PyAny>)>,
    end_loop: bool,
    /// KeyPress repeats while a key is held (`do_key_autorepeaton/off`).
    autorepeat: bool,
    /// Events collected by `do_sync`, fired at the loop's next event step.
    synced: Vec<Pending>,
}

impl Inner {
    /// The state of an open window, or an error if it was closed.
    pub(crate) fn win(&mut self, win: &Window) -> PyResult<&mut WinState> {
        self.windows
            .get_mut(win.id)
            .and_then(Option::as_mut)
            .ok_or_else(|| PyRuntimeError::new_err("window is closed"))
    }
}

/// Connection handle, like the `void *mlx_ptr` returned by `mlx_init`.
#[pyclass(unsendable)]
pub(crate) struct Mlx {
    pub(crate) inner: RefCell<Inner>,
}

/// mlx_init
///
/// Docs: https://0xmthan.github.io/malibx/reference/init/
#[pyfunction]
pub(crate) fn init() -> Mlx {
    Mlx {
        inner: RefCell::new(Inner {
            windows: Vec::new(),
            loop_hook: None,
            end_loop: false,
            autorepeat: true,
            synced: Vec::new(),
        }),
    }
}

/// mlx_loop_hook: `f(param)` is called once per frame.
///
/// Docs: https://0xmthan.github.io/malibx/reference/loop_hook/
#[pyfunction]
#[pyo3(signature = (mlx, f, param=None))]
pub(crate) fn loop_hook(
    mlx: PyRef<'_, Mlx>,
    py: Python<'_>,
    f: Py<PyAny>,
    param: Option<Py<PyAny>>,
) {
    let param = param.unwrap_or_else(|| py.None());
    mlx.inner.borrow_mut().loop_hook = Some((f, param));
}

/// mlx_loop_end: makes `loop` return after the current frame.
///
/// Docs: https://0xmthan.github.io/malibx/reference/loop_end/
#[pyfunction]
pub(crate) fn loop_end(mlx: PyRef<'_, Mlx>) {
    mlx.inner.borrow_mut().end_loop = true;
}

/// mlx_loop: runs until loop_end is called or every window is closed.
///
/// Docs: https://0xmthan.github.io/malibx/reference/loop/
#[pyfunction]
#[pyo3(name = "loop")]
pub(crate) fn run_loop(mlx: PyRef<'_, Mlx>, py: Python<'_>) -> PyResult<()> {
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

        // Present each window and collect its events; drop the ones the user
        // closed. Hooks are called after the borrow ends, so they can call
        // malibx too.
        let pending = {
            let mut inner = mlx.inner.borrow_mut();
            let mut pending = std::mem::take(&mut inner.synced);
            present_all(&mut inner, py, &mut pending)?;
            pending
        };
        for call in pending {
            call.fire(py)?;
        }

        let inner = mlx.inner.borrow();
        if inner.end_loop || inner.windows.iter().all(Option::is_none) {
            return Ok(());
        }
    }
}

/// Shows every open window and queues its events; drops windows the user
/// closed (queueing their DestroyNotify).
fn present_all(inner: &mut Inner, py: Python<'_>, out: &mut Vec<Pending>) -> PyResult<()> {
    let autorepeat = inner.autorepeat;
    for slot in inner.windows.iter_mut() {
        if let Some(w) = slot {
            if !w.window.is_open() {
                w.poll_closed(py, out);
                *slot = None;
                continue;
            }
            w.present()?;
            w.poll(py, out, autorepeat);
        }
    }
    Ok(())
}

/// mlx_do_sync: shows everything drawn so far right away, without waiting
/// for the loop. Input that arrives meanwhile is kept for the loop's hooks.
///
/// Docs: https://0xmthan.github.io/malibx/reference/do_sync/
#[pyfunction]
pub(crate) fn do_sync(mlx: PyRef<'_, Mlx>, py: Python<'_>) -> PyResult<()> {
    let mut inner = mlx.inner.borrow_mut();
    let mut synced = std::mem::take(&mut inner.synced);
    let result = present_all(&mut inner, py, &mut synced);
    inner.synced = synced;
    result
}

/// mlx_do_key_autorepeatoff: KeyPress (event 2) fires once per key press,
/// not again while the key is held. Only affects malibx windows.
///
/// Docs: https://0xmthan.github.io/malibx/reference/do_key_autorepeatoff/
#[pyfunction]
pub(crate) fn do_key_autorepeatoff(mlx: PyRef<'_, Mlx>) {
    mlx.inner.borrow_mut().autorepeat = false;
}

/// mlx_do_key_autorepeaton: KeyPress (event 2) repeats while a key is held
/// (the default).
///
/// Docs: https://0xmthan.github.io/malibx/reference/do_key_autorepeaton/
#[pyfunction]
pub(crate) fn do_key_autorepeaton(mlx: PyRef<'_, Mlx>) {
    mlx.inner.borrow_mut().autorepeat = true;
}

/// mlx_destroy_display: closes every window and forgets the loop hook.
///
/// Docs: https://0xmthan.github.io/malibx/reference/destroy_display/
#[pyfunction]
pub(crate) fn destroy_display(mlx: PyRef<'_, Mlx>) {
    let mut inner = mlx.inner.borrow_mut();
    inner.windows.iter_mut().for_each(|slot| *slot = None);
    inner.loop_hook = None;
    inner.synced.clear();
}

/// mlx_get_screen_size: the main screen's (width, height).
///
/// Docs: https://0xmthan.github.io/malibx/reference/get_screen_size/
#[pyfunction]
pub(crate) fn get_screen_size(mlx: PyRef<'_, Mlx>) -> PyResult<(usize, usize)> {
    let _ = mlx; // kept for the MiniLibX signature
    crate::platform::screen_size()
        .ok_or_else(|| PyRuntimeError::new_err("get_screen_size failed: no display"))
}

/// mlx_get_color_value: the color as the window expects it. malibx windows
/// always use 0xRRGGBB, so this just drops the top byte.
///
/// Docs: https://0xmthan.github.io/malibx/reference/get_color_value/
#[pyfunction]
pub(crate) fn get_color_value(mlx: PyRef<'_, Mlx>, color: i64) -> i64 {
    let _ = mlx; // kept for the MiniLibX signature
    color & 0x00FF_FFFF
}
