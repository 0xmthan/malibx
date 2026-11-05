//! malibx: a tiny window/pixel library with a MiniLibX-style API, usable from Python.

use std::cell::RefCell;

use minifb::{Window as MfbWindow, WindowOptions};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

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

/// Connection handle, like the `void *mlx_ptr` returned by `mlx_init`.
#[pyclass(unsendable)]
struct Mlx {
    inner: RefCell<Inner>,
}

/// Window handle, like the `void *win_ptr` returned by `mlx_new_window`.
#[pyclass(frozen)]
struct Window {
    #[allow(dead_code)]
    id: usize,
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
        // Present each window and pump its events; drop the ones the user closed.
        {
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
        // Call the loop hook without holding the borrow, so it can call malibx itself.
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
    }
}

#[pymodule]
fn malibx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mlx>()?;
    m.add_class::<Window>()?;
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(new_window, m)?)?;
    m.add_function(wrap_pyfunction!(loop_hook, m)?)?;
    m.add_function(wrap_pyfunction!(loop_end, m)?)?;
    m.add_function(wrap_pyfunction!(run_loop, m)?)?;
    Ok(())
}
