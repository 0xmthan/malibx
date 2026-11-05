//! Event hooks (`man mlx_loop`, `mlx_hook`): turns minifb input into
//! MiniLibX-style X11 events and calls the registered Python hooks.

use std::collections::HashMap;

use minifb::{Key, KeyRepeat, MouseButton, MouseMode};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::window::{WinState, Window};

// X11 event numbers (X.h).
pub(crate) const KEY_PRESS: i32 = 2;
pub(crate) const KEY_RELEASE: i32 = 3;
pub(crate) const BUTTON_PRESS: i32 = 4;
pub(crate) const BUTTON_RELEASE: i32 = 5;
pub(crate) const MOTION_NOTIFY: i32 = 6;
pub(crate) const EXPOSE: i32 = 12;
pub(crate) const DESTROY_NOTIFY: i32 = 17;
const LAST_EVENT: i32 = 36;

/// A registered hook: `f` and the `param` it gets back.
pub(crate) struct Hook {
    f: Py<PyAny>,
    param: Py<PyAny>,
}

/// Per-window input state, used to turn "is it down now" into press/release events.
#[derive(Default)]
pub(crate) struct InputState {
    exposed: bool,
    mouse_down: [bool; 3],
    mouse_pos: Option<(i64, i64)>,
}

/// What a hook gets called with, before `param` (see mlx_int_param_event.c).
enum Args {
    Plain,
    Key(u32),
    Button(u8, i64, i64),
    Motion(i64, i64),
}

/// A hook call waiting to run once the window state is no longer borrowed.
pub(crate) struct Pending {
    f: Py<PyAny>,
    param: Py<PyAny>,
    args: Args,
}

impl Pending {
    pub(crate) fn fire(self, py: Python<'_>) -> PyResult<()> {
        let p = self.param;
        match self.args {
            Args::Plain => self.f.call1(py, (p,)),
            Args::Key(k) => self.f.call1(py, (k, p)),
            Args::Button(b, x, y) => self.f.call1(py, (b, x, y, p)),
            Args::Motion(x, y) => self.f.call1(py, (x, y, p)),
        }?;
        Ok(())
    }
}

fn queue(
    hooks: &HashMap<i32, Hook>,
    py: Python<'_>,
    out: &mut Vec<Pending>,
    event: i32,
    args: Args,
) {
    if let Some(h) = hooks.get(&event) {
        out.push(Pending {
            f: h.f.clone_ref(py),
            param: h.param.clone_ref(py),
            args,
        });
    }
}

impl WinState {
    /// Queues DestroyNotify for a window the user just closed.
    pub(crate) fn poll_closed(&self, py: Python<'_>, out: &mut Vec<Pending>) {
        queue(&self.hooks, py, out, DESTROY_NOTIFY, Args::Plain);
    }

    /// Queues the events since the last frame. Call right after `present`.
    pub(crate) fn poll(&mut self, py: Python<'_>, out: &mut Vec<Pending>) {
        let (hooks, win, input) = (&self.hooks, &self.window, &mut self.input);

        if !input.exposed {
            input.exposed = true;
            queue(hooks, py, out, EXPOSE, Args::Plain);
        }

        for key in win.get_keys_pressed(KeyRepeat::Yes) {
            if let Some(k) = keysym(key) {
                queue(hooks, py, out, KEY_PRESS, Args::Key(k));
            }
        }
        for key in win.get_keys_released() {
            if let Some(k) = keysym(key) {
                queue(hooks, py, out, KEY_RELEASE, Args::Key(k));
            }
        }

        let pos = win
            .get_mouse_pos(MouseMode::Discard)
            .map(|(x, y)| (x as i64, y as i64));
        if let Some((x, y)) = pos
            && pos != input.mouse_pos
        {
            queue(hooks, py, out, MOTION_NOTIFY, Args::Motion(x, y));
        }
        input.mouse_pos = pos;

        let (x, y) = win
            .get_mouse_pos(MouseMode::Clamp)
            .map_or((0, 0), |(x, y)| (x as i64, y as i64));
        let buttons = [MouseButton::Left, MouseButton::Middle, MouseButton::Right];
        for (i, button) in buttons.into_iter().enumerate() {
            let down = win.get_mouse_down(button);
            let n = i as u8 + 1; // X11: 1 left, 2 middle, 3 right
            if down && !input.mouse_down[i] {
                queue(hooks, py, out, BUTTON_PRESS, Args::Button(n, x, y));
            } else if !down && input.mouse_down[i] {
                queue(hooks, py, out, BUTTON_RELEASE, Args::Button(n, x, y));
            }
            input.mouse_down[i] = down;
        }

        // X11 reports the scroll wheel as buttons 4 (up) and 5 (down).
        if let Some((_, dy)) = win.get_scroll_wheel()
            && dy != 0.0
        {
            let n = if dy > 0.0 { 4 } else { 5 };
            queue(hooks, py, out, BUTTON_PRESS, Args::Button(n, x, y));
            queue(hooks, py, out, BUTTON_RELEASE, Args::Button(n, x, y));
        }
    }
}

fn set_hook(
    win: &Window,
    py: Python<'_>,
    event: i32,
    f: Py<PyAny>,
    param: Option<Py<PyAny>>,
) -> PyResult<()> {
    if !(0..LAST_EVENT).contains(&event) {
        return Err(PyValueError::new_err(format!(
            "x_event must be between 0 and {}",
            LAST_EVENT - 1
        )));
    }
    let param = param.unwrap_or_else(|| py.None());
    let mlx = win.mlx.bind(py).borrow();
    let mut inner = mlx.inner.borrow_mut();
    inner.win(win)?.hooks.insert(event, Hook { f, param });
    Ok(())
}

/// mlx_hook: calls `f` on X11 event `x_event`. `x_mask` is accepted for
/// compatibility; malibx always listens to every supported event.
///
/// Docs: https://0xmthan.github.io/malibx/reference/hook/
#[pyfunction]
#[pyo3(signature = (win, x_event, x_mask, f, param=None))]
pub(crate) fn hook(
    win: PyRef<'_, Window>,
    py: Python<'_>,
    x_event: i32,
    x_mask: i64,
    f: Py<PyAny>,
    param: Option<Py<PyAny>>,
) -> PyResult<()> {
    let _ = x_mask;
    set_hook(&win, py, x_event, f, param)
}

/// mlx_key_hook: `f(keysym, param)` on key release.
///
/// Docs: https://0xmthan.github.io/malibx/reference/key_hook/
#[pyfunction]
#[pyo3(signature = (win, f, param=None))]
pub(crate) fn key_hook(
    win: PyRef<'_, Window>,
    py: Python<'_>,
    f: Py<PyAny>,
    param: Option<Py<PyAny>>,
) -> PyResult<()> {
    set_hook(&win, py, KEY_RELEASE, f, param)
}

/// mlx_mouse_hook: `f(button, x, y, param)` on mouse button press.
///
/// Docs: https://0xmthan.github.io/malibx/reference/mouse_hook/
#[pyfunction]
#[pyo3(signature = (win, f, param=None))]
pub(crate) fn mouse_hook(
    win: PyRef<'_, Window>,
    py: Python<'_>,
    f: Py<PyAny>,
    param: Option<Py<PyAny>>,
) -> PyResult<()> {
    set_hook(&win, py, BUTTON_PRESS, f, param)
}

/// mlx_expose_hook: `f(param)` when the window is first shown.
///
/// Docs: https://0xmthan.github.io/malibx/reference/expose_hook/
#[pyfunction]
#[pyo3(signature = (win, f, param=None))]
pub(crate) fn expose_hook(
    win: PyRef<'_, Window>,
    py: Python<'_>,
    f: Py<PyAny>,
    param: Option<Py<PyAny>>,
) -> PyResult<()> {
    set_hook(&win, py, EXPOSE, f, param)
}

/// minifb key -> X11 keysym, values from xorgproto's keysymdef.h.
/// Letters are always lowercase, like MiniLibX.
fn keysym(key: Key) -> Option<u32> {
    Some(match key {
        Key::Key0 => 0x0030,           // XK_0
        Key::Key1 => 0x0031,           // XK_1
        Key::Key2 => 0x0032,           // XK_2
        Key::Key3 => 0x0033,           // XK_3
        Key::Key4 => 0x0034,           // XK_4
        Key::Key5 => 0x0035,           // XK_5
        Key::Key6 => 0x0036,           // XK_6
        Key::Key7 => 0x0037,           // XK_7
        Key::Key8 => 0x0038,           // XK_8
        Key::Key9 => 0x0039,           // XK_9
        Key::A => 0x0061,              // XK_a
        Key::B => 0x0062,              // XK_b
        Key::C => 0x0063,              // XK_c
        Key::D => 0x0064,              // XK_d
        Key::E => 0x0065,              // XK_e
        Key::F => 0x0066,              // XK_f
        Key::G => 0x0067,              // XK_g
        Key::H => 0x0068,              // XK_h
        Key::I => 0x0069,              // XK_i
        Key::J => 0x006a,              // XK_j
        Key::K => 0x006b,              // XK_k
        Key::L => 0x006c,              // XK_l
        Key::M => 0x006d,              // XK_m
        Key::N => 0x006e,              // XK_n
        Key::O => 0x006f,              // XK_o
        Key::P => 0x0070,              // XK_p
        Key::Q => 0x0071,              // XK_q
        Key::R => 0x0072,              // XK_r
        Key::S => 0x0073,              // XK_s
        Key::T => 0x0074,              // XK_t
        Key::U => 0x0075,              // XK_u
        Key::V => 0x0076,              // XK_v
        Key::W => 0x0077,              // XK_w
        Key::X => 0x0078,              // XK_x
        Key::Y => 0x0079,              // XK_y
        Key::Z => 0x007a,              // XK_z
        Key::F1 => 0xffbe,             // XK_F1
        Key::F2 => 0xffbf,             // XK_F2
        Key::F3 => 0xffc0,             // XK_F3
        Key::F4 => 0xffc1,             // XK_F4
        Key::F5 => 0xffc2,             // XK_F5
        Key::F6 => 0xffc3,             // XK_F6
        Key::F7 => 0xffc4,             // XK_F7
        Key::F8 => 0xffc5,             // XK_F8
        Key::F9 => 0xffc6,             // XK_F9
        Key::F10 => 0xffc7,            // XK_F10
        Key::F11 => 0xffc8,            // XK_F11
        Key::F12 => 0xffc9,            // XK_F12
        Key::F13 => 0xffca,            // XK_F13
        Key::F14 => 0xffcb,            // XK_F14
        Key::F15 => 0xffcc,            // XK_F15
        Key::Down => 0xff54,           // XK_Down
        Key::Left => 0xff51,           // XK_Left
        Key::Right => 0xff53,          // XK_Right
        Key::Up => 0xff52,             // XK_Up
        Key::Apostrophe => 0x0027,     // XK_apostrophe
        Key::Backquote => 0x0060,      // XK_grave
        Key::Backslash => 0x005c,      // XK_backslash
        Key::Comma => 0x002c,          // XK_comma
        Key::Equal => 0x003d,          // XK_equal
        Key::LeftBracket => 0x005b,    // XK_bracketleft
        Key::Minus => 0x002d,          // XK_minus
        Key::Period => 0x002e,         // XK_period
        Key::RightBracket => 0x005d,   // XK_bracketright
        Key::Semicolon => 0x003b,      // XK_semicolon
        Key::Slash => 0x002f,          // XK_slash
        Key::Backspace => 0xff08,      // XK_BackSpace
        Key::Delete => 0xffff,         // XK_Delete
        Key::End => 0xff57,            // XK_End
        Key::Enter => 0xff0d,          // XK_Return
        Key::Escape => 0xff1b,         // XK_Escape
        Key::Home => 0xff50,           // XK_Home
        Key::Insert => 0xff63,         // XK_Insert
        Key::Menu => 0xff67,           // XK_Menu
        Key::PageDown => 0xff56,       // XK_Page_Down
        Key::PageUp => 0xff55,         // XK_Page_Up
        Key::Pause => 0xff13,          // XK_Pause
        Key::Space => 0x0020,          // XK_space
        Key::Tab => 0xff09,            // XK_Tab
        Key::NumLock => 0xff7f,        // XK_Num_Lock
        Key::CapsLock => 0xffe5,       // XK_Caps_Lock
        Key::ScrollLock => 0xff14,     // XK_Scroll_Lock
        Key::LeftShift => 0xffe1,      // XK_Shift_L
        Key::RightShift => 0xffe2,     // XK_Shift_R
        Key::LeftCtrl => 0xffe3,       // XK_Control_L
        Key::RightCtrl => 0xffe4,      // XK_Control_R
        Key::NumPad0 => 0xffb0,        // XK_KP_0
        Key::NumPad1 => 0xffb1,        // XK_KP_1
        Key::NumPad2 => 0xffb2,        // XK_KP_2
        Key::NumPad3 => 0xffb3,        // XK_KP_3
        Key::NumPad4 => 0xffb4,        // XK_KP_4
        Key::NumPad5 => 0xffb5,        // XK_KP_5
        Key::NumPad6 => 0xffb6,        // XK_KP_6
        Key::NumPad7 => 0xffb7,        // XK_KP_7
        Key::NumPad8 => 0xffb8,        // XK_KP_8
        Key::NumPad9 => 0xffb9,        // XK_KP_9
        Key::NumPadDot => 0xffae,      // XK_KP_Decimal
        Key::NumPadSlash => 0xffaf,    // XK_KP_Divide
        Key::NumPadAsterisk => 0xffaa, // XK_KP_Multiply
        Key::NumPadMinus => 0xffad,    // XK_KP_Subtract
        Key::NumPadPlus => 0xffab,     // XK_KP_Add
        Key::NumPadEnter => 0xff8d,    // XK_KP_Enter
        Key::LeftAlt => 0xffe9,        // XK_Alt_L
        Key::RightAlt => 0xffea,       // XK_Alt_R
        Key::LeftSuper => 0xffeb,      // XK_Super_L
        Key::RightSuper => 0xffec,     // XK_Super_R
        Key::Unknown | Key::Count => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keysyms_match_x11() {
        assert_eq!(keysym(Key::Left), Some(0xff51));
        assert_eq!(keysym(Key::Up), Some(0xff52));
        assert_eq!(keysym(Key::Right), Some(0xff53));
        assert_eq!(keysym(Key::Down), Some(0xff54));
        assert_eq!(keysym(Key::Escape), Some(0xff1b));
        assert_eq!(keysym(Key::Enter), Some(0xff0d));
        assert_eq!(keysym(Key::Backspace), Some(0xff08));
        assert_eq!(keysym(Key::Space), Some(0x20));
        assert_eq!(keysym(Key::Unknown), None);
    }

    #[test]
    fn letters_and_digits_are_lowercase_ascii() {
        let letters = [
            Key::A,
            Key::B,
            Key::C,
            Key::D,
            Key::E,
            Key::F,
            Key::G,
            Key::H,
            Key::I,
            Key::J,
            Key::K,
            Key::L,
            Key::M,
            Key::N,
            Key::O,
            Key::P,
            Key::Q,
            Key::R,
            Key::S,
            Key::T,
            Key::U,
            Key::V,
            Key::W,
            Key::X,
            Key::Y,
            Key::Z,
        ];
        for (i, key) in letters.into_iter().enumerate() {
            assert_eq!(keysym(key), Some(b'a' as u32 + i as u32));
        }
        let digits = [
            Key::Key0,
            Key::Key1,
            Key::Key2,
            Key::Key3,
            Key::Key4,
            Key::Key5,
            Key::Key6,
            Key::Key7,
            Key::Key8,
            Key::Key9,
        ];
        for (i, key) in digits.into_iter().enumerate() {
            assert_eq!(keysym(key), Some(b'0' as u32 + i as u32));
        }
    }
}
