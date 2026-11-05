//! malibx: a tiny window/pixel library with a MiniLibX-style API, usable from Python.

mod events;
mod icon;
mod image;
mod mlx;
mod mouse;
mod platform;
mod text;
mod window;
mod xpm;
mod xpm_colors;

use pyo3::prelude::*;

#[pymodule]
fn malibx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<mlx::Mlx>()?;
    m.add_class::<window::Window>()?;
    m.add_class::<image::Image>()?;

    m.add_function(wrap_pyfunction!(mlx::init, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::loop_hook, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::loop_end, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::run_loop, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::do_sync, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::do_key_autorepeatoff, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::do_key_autorepeaton, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::destroy_display, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::get_screen_size, m)?)?;
    m.add_function(wrap_pyfunction!(mlx::get_color_value, m)?)?;

    m.add_function(wrap_pyfunction!(window::new_window, m)?)?;
    m.add_function(wrap_pyfunction!(window::destroy_window, m)?)?;
    m.add_function(wrap_pyfunction!(window::clear_window, m)?)?;
    m.add_function(wrap_pyfunction!(window::pixel_put, m)?)?;
    m.add_function(wrap_pyfunction!(text::string_put, m)?)?;
    m.add_function(wrap_pyfunction!(text::set_font, m)?)?;

    m.add_function(wrap_pyfunction!(events::hook, m)?)?;
    m.add_function(wrap_pyfunction!(events::key_hook, m)?)?;
    m.add_function(wrap_pyfunction!(events::mouse_hook, m)?)?;
    m.add_function(wrap_pyfunction!(events::expose_hook, m)?)?;

    m.add_function(wrap_pyfunction!(mouse::mouse_get_pos, m)?)?;
    m.add_function(wrap_pyfunction!(mouse::mouse_move, m)?)?;
    m.add_function(wrap_pyfunction!(mouse::mouse_hide, m)?)?;
    m.add_function(wrap_pyfunction!(mouse::mouse_show, m)?)?;

    m.add_function(wrap_pyfunction!(image::new_image, m)?)?;
    m.add_function(wrap_pyfunction!(image::get_data_addr, m)?)?;
    m.add_function(wrap_pyfunction!(image::put_image_to_window, m)?)?;
    m.add_function(wrap_pyfunction!(image::destroy_image, m)?)?;
    m.add_function(wrap_pyfunction!(xpm::xpm_to_image, m)?)?;
    m.add_function(wrap_pyfunction!(xpm::xpm_file_to_image, m)?)?;
    Ok(())
}
