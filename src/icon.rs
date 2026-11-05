//! Default window icon: the malibx skull (`assets/skull.xpm`), set
//! automatically by `new_window`. Not part of the MiniLibX API.
//!
//! - macOS: the app's Dock icon, through `NSApplication` (minifb doesn't
//!   support icons on macOS).
//! - Linux X11: the window icon, through minifb (`_NET_WM_ICON`).
//! - Wayland and Windows: no icon (minifb can't set one at runtime there).

#[cfg(any(target_os = "macos", target_os = "linux", test))]
use crate::xpm;

#[cfg(any(target_os = "macos", target_os = "linux", test))]
const SKULL: &str = include_str!("../assets/skull.xpm");

/// The skull as (width, height, BGRA pixels); alpha 0xFF is transparent.
#[cfg(any(target_os = "macos", target_os = "linux", test))]
fn skull() -> Option<(usize, usize, Vec<u8>)> {
    xpm::parse(&xpm::strings(SKULL)).ok()
}

/// Gives a freshly created window the default icon.
pub(crate) fn apply(window: &mut minifb::Window) {
    #[cfg(target_os = "macos")]
    {
        let _ = window;
        macos::set_app_icon();
    }
    #[cfg(target_os = "linux")]
    linux::set_window_icon(window);
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let _ = window;
}

#[cfg(target_os = "macos")]
mod macos {
    use std::sync::Once;

    use objc2::{AnyThread, MainThreadMarker};
    use objc2_app_kit::{NSApplication, NSBitmapImageRep, NSDeviceRGBColorSpace, NSImage};
    use objc2_foundation::NSSize;

    /// Dock icons are drawn at up to 512 px; the skull is scaled up with
    /// nearest neighbour so it stays pixel-crisp, with a margin around it.
    const SIZE: usize = 512;

    static ONCE: Once = Once::new();

    pub(super) fn set_app_icon() {
        ONCE.call_once(|| {
            // AppKit may only be used from the main thread.
            let Some(mtm) = MainThreadMarker::new() else {
                return;
            };
            let Some((w, h, bgra)) = super::skull() else {
                return;
            };
            let scale = (SIZE * 7 / 8) / w.max(h);
            let (ox, oy) = ((SIZE - w * scale) / 2, (SIZE - h * scale) / 2);

            // SAFETY: null planes makes AppKit allocate the pixel buffer; the
            // sizes describe a SIZE x SIZE, 8-bit RGBA, non-planar image.
            let rep = unsafe {
                NSBitmapImageRep::initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
                    NSBitmapImageRep::alloc(),
                    std::ptr::null_mut(),
                    SIZE as isize,
                    SIZE as isize,
                    8,
                    4,
                    true,
                    false,
                    NSDeviceRGBColorSpace,
                    (SIZE * 4) as isize,
                    32,
                )
            };
            let Some(rep) = rep else {
                return;
            };
            let data = rep.bitmapData();
            if data.is_null() {
                return;
            }
            // SAFETY: AppKit allocated SIZE * SIZE * 4 bytes for this rep above.
            let rgba = unsafe { std::slice::from_raw_parts_mut(data, SIZE * SIZE * 4) };
            rgba.fill(0);
            for y in 0..h * scale {
                for x in 0..w * scale {
                    let s = ((y / scale) * w + x / scale) * 4;
                    if bgra[s + 3] == 0xFF {
                        continue; // transparent
                    }
                    let d = ((y + oy) * SIZE + x + ox) * 4;
                    rgba[d..d + 4].copy_from_slice(&[bgra[s + 2], bgra[s + 1], bgra[s], 0xFF]);
                }
            }

            let image = NSImage::initWithSize(NSImage::alloc(), NSSize::new(SIZE as f64, SIZE as f64));
            image.addRepresentation(&rep);
            let app = NSApplication::sharedApplication(mtm);
            // SAFETY: a valid NSImage, set from the main thread.
            unsafe { app.setApplicationIconImage(Some(&image)) };
        });
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use minifb::Icon;
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    pub(super) fn set_window_icon(window: &mut minifb::Window) {
        // minifb panics if asked to set an icon on Wayland, so only do X11.
        let x11 = window
            .window_handle()
            .is_ok_and(|h| matches!(h.as_raw(), RawWindowHandle::Xlib(_)));
        if !x11 {
            return;
        }
        let Some((w, h, bgra)) = super::skull() else {
            return;
        };
        // _NET_WM_ICON: width, height, then one ARGB pixel per `long`.
        let mut data: Vec<u64> = vec![w as u64, h as u64];
        data.extend(bgra.as_chunks::<4>().0.iter().map(|p| {
            if p[3] == 0xFF {
                0
            } else {
                0xFF00_0000 | (p[2] as u64) << 16 | (p[1] as u64) << 8 | p[0] as u64
            }
        }));
        // XChangeProperty copies the data, so it may be dropped right after.
        if let Ok(icon) = Icon::try_from(&data[..]) {
            window.set_icon(icon);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn built_in_skull_parses() {
        let (w, h, px) = super::skull().expect("assets/skull.xpm must be valid");
        assert_eq!((w, h, px.len()), (48, 48, 48 * 48 * 4));
    }
}
