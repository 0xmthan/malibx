//! The connection handle and the main loop (`man mlx`, `man mlx_loop`).

use std::cell::RefCell;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::window::{WinState, Window};

pub(crate) struct Inner {
    pub(crate) windows: Vec<Option<WinState>>,
    loop_hook: Option<(Py<PyAny>, Py<PyAny>)>,
    end_loop: bool,
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
        let mut pending = Vec::new();
        {
            let mut inner = mlx.inner.borrow_mut();
            for slot in inner.windows.iter_mut() {
                if let Some(w) = slot {
                    if !w.window.is_open() {
                        w.poll_closed(py, &mut pending);
                        *slot = None;
                        continue;
                    }
                    w.present()?;
                    w.poll(py, &mut pending);
                }
            }
        }
        for call in pending {
            call.fire(py)?;
        }

        let inner = mlx.inner.borrow();
        if inner.end_loop || inner.windows.iter().all(Option::is_none) {
            return Ok(());
        }
    }
}
