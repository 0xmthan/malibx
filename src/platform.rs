//! The two things minifb can't do, per platform: read the screen size
//! (`mlx_get_screen_size`) and move the pointer (`mlx_mouse_move`).
//!
//! - macOS: AppKit for the screen, CoreGraphics to warp the pointer.
//! - Linux X11: Xlib, loaded at runtime like minifb does. Wayland doesn't
//!   let apps move the pointer, so `warp_pointer` does nothing there.
//! - Windows: user32.

/// The main screen's size in pixels (points on macOS), or `None` if there's
/// no display to ask.
pub(crate) fn screen_size() -> Option<(usize, usize)> {
    imp::screen_size()
}

/// Moves the pointer to (x, y) in the window's drawing area. Returns `false`
/// if the platform can't (Wayland, or no window handle).
pub(crate) fn warp_pointer(window: &minifb::Window, x: i64, y: i64) -> bool {
    imp::warp_pointer(window, x, y)
}

#[cfg(target_os = "macos")]
mod imp {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSScreen, NSView};
    use objc2_core_graphics::{CGAssociateMouseAndMouseCursorPosition, CGWarpMouseCursorPosition};
    use objc2_foundation::NSPoint;
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    pub(super) fn screen_size() -> Option<(usize, usize)> {
        let mtm = MainThreadMarker::new()?;
        let frame = NSScreen::mainScreen(mtm)?.frame();
        Some((frame.size.width as usize, frame.size.height as usize))
    }

    pub(super) fn warp_pointer(window: &minifb::Window, x: i64, y: i64) -> bool {
        let Some(mtm) = MainThreadMarker::new() else {
            return false;
        };
        let Ok(handle) = window.window_handle() else {
            return false;
        };
        let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
            return false;
        };
        // SAFETY: minifb's ns_view is a live NSView for as long as `window` is.
        let view: &NSView = unsafe { handle.ns_view.cast().as_ref() };
        let Some(ns_window) = view.window() else {
            return false;
        };

        // View coordinates -> screen coordinates (origin bottom-left of the
        // primary screen) -> CoreGraphics coordinates (origin top-left).
        let (x, y) = (x as f64, y as f64);
        let y = if view.isFlipped() {
            y
        } else {
            view.bounds().size.height - y
        };
        let in_window = view.convertPoint_toView(NSPoint::new(x, y), None);
        let on_screen = ns_window.convertPointToScreen(in_window);
        let Some(primary) = NSScreen::screens(mtm).firstObject() else {
            return false;
        };
        let top = primary.frame().size.height;
        CGWarpMouseCursorPosition(NSPoint::new(on_screen.x, top - on_screen.y));
        // Without this, macOS ignores mouse movement for a moment after a warp.
        CGAssociateMouseAndMouseCursorPosition(true);
        true
    }
}

#[cfg(target_os = "linux")]
mod imp {
    use std::ffi::c_int;
    use std::ptr;

    use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
    use x11_dl::xlib::Xlib;

    pub(super) fn screen_size() -> Option<(usize, usize)> {
        let xlib = Xlib::open().ok()?;
        // SAFETY: plain Xlib calls on a display we open and close here.
        unsafe {
            let display = (xlib.XOpenDisplay)(ptr::null());
            if display.is_null() {
                return None;
            }
            let screen = (xlib.XDefaultScreen)(display);
            let size = (
                (xlib.XDisplayWidth)(display, screen) as usize,
                (xlib.XDisplayHeight)(display, screen) as usize,
            );
            (xlib.XCloseDisplay)(display);
            Some(size)
        }
    }

    pub(super) fn warp_pointer(window: &minifb::Window, x: i64, y: i64) -> bool {
        let (Ok(win), Ok(display)) = (window.window_handle(), window.display_handle()) else {
            return false;
        };
        // Only X11: Wayland doesn't allow moving the pointer.
        let (RawWindowHandle::Xlib(win), RawDisplayHandle::Xlib(display)) =
            (win.as_raw(), display.as_raw())
        else {
            return false;
        };
        let (Some(display), Ok(xlib)) = (display.display, Xlib::open()) else {
            return false;
        };
        let display = display.as_ptr().cast();
        // SAFETY: minifb's display and window stay valid while `window` lives.
        unsafe {
            (xlib.XWarpPointer)(display, 0, win.window, 0, 0, 0, 0, x as c_int, y as c_int);
            (xlib.XFlush)(display);
        }
        true
    }
}

#[cfg(windows)]
mod imp {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use winapi::shared::windef::{HWND, POINT};
    use winapi::um::winuser::{
        ClientToScreen, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN, SetCursorPos,
    };

    pub(super) fn screen_size() -> Option<(usize, usize)> {
        // SAFETY: GetSystemMetrics has no preconditions.
        let (w, h) = unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };
        (w > 0 && h > 0).then_some((w as usize, h as usize))
    }

    pub(super) fn warp_pointer(window: &minifb::Window, x: i64, y: i64) -> bool {
        let Ok(handle) = window.window_handle() else {
            return false;
        };
        let RawWindowHandle::Win32(handle) = handle.as_raw() else {
            return false;
        };
        let mut p = POINT {
            x: x as i32,
            y: y as i32,
        };
        // SAFETY: minifb's HWND is valid while `window` lives.
        unsafe {
            ClientToScreen(handle.hwnd.get() as HWND, &mut p) != 0 && SetCursorPos(p.x, p.y) != 0
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
mod imp {
    pub(super) fn screen_size() -> Option<(usize, usize)> {
        None
    }

    pub(super) fn warp_pointer(_: &minifb::Window, _: i64, _: i64) -> bool {
        false
    }
}
