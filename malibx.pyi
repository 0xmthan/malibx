"""A tiny window and pixel library with a MiniLibX-style API.

Docs: https://0xmthan.github.io/malibx/
"""

import os
from typing import Any, Callable, TypeVar, final

_T = TypeVar("_T")

@final
class Mlx:
    """The connection handle returned by `init()` (MiniLibX's `mlx_ptr`).

    Owns your windows and the loop hook. Pass it as the first argument to
    most malibx functions.

    Docs: https://0xmthan.github.io/malibx/reference/#mlx
    """

@final
class Window:
    """A window returned by `new_window()` (MiniLibX's `win_ptr`).

    Once the user closes it, functions that use it raise
    `RuntimeError: window is closed`.

    Docs: https://0xmthan.github.io/malibx/reference/#window
    """

@final
class Image:
    """An off-screen image (MiniLibX's `img_ptr`).

    Created by `new_image()`, `xpm_file_to_image()` or `xpm_to_image()`.
    Reach its pixels with `get_data_addr()`, draw it with
    `put_image_to_window()`.

    Docs: https://0xmthan.github.io/malibx/reference/#image
    """

# Setup and loop

def init() -> Mlx:
    """Create the connection handle. Like `mlx_init`.

    Call once at the start of your program and pass the result to
    `new_window()`, `loop()` and the drawing functions. Never fails: display
    errors show up in `new_window()`.

    Returns:
        The `Mlx` connection.

    Example:
        mlx = malibx.init()
        win = malibx.new_window(mlx, 640, 480, "hello")
        malibx.loop(mlx)

    Docs: https://0xmthan.github.io/malibx/reference/init/
    """

def loop(mlx: Mlx) -> None:
    """Run the main loop until every window is closed or `loop_end()` is called.

    Like `mlx_loop`. Each frame, at up to 60 fps:
    1. calls the loop hook (see `loop_hook()`),
    2. shows every window's pixels on screen,
    3. calls the event hooks for input since the last frame.

    Args:
        mlx: The connection from `init()`.

    Raises:
        KeyboardInterrupt: Ctrl+C was pressed in the terminal.
        Exception: Any exception raised by one of your hooks propagates out.

    Docs: https://0xmthan.github.io/malibx/reference/loop/
    """

def loop_hook(mlx: Mlx, f: Callable[[_T], Any], param: _T = ...) -> None:
    """Call `f(param)` at the start of every frame. Like `mlx_loop_hook`.

    Update and draw here. Windows are not cleared between frames, so call
    `clear_window()` (or draw a full background) if things move. There is one
    loop hook per `mlx`; calling this again replaces it.

    Args:
        mlx: The connection from `init()`.
        f: Called as `f(param)` once per frame.
        param: Passed to `f`. Defaults to `None`.

    Example:
        def frame(st):
            malibx.clear_window(mlx, win)
            malibx.put_image_to_window(mlx, win, player, st["x"], st["y"])

        malibx.loop_hook(mlx, frame, {"x": 0, "y": 0})

    Docs: https://0xmthan.github.io/malibx/reference/loop_hook/
    """

def loop_end(mlx: Mlx) -> None:
    """Make the running `loop()` return after the current frame. Like `mlx_loop_end`.

    Windows stay open; you can call `loop()` again later.

    Args:
        mlx: The connection whose loop should stop.

    Example:
        def on_key(keysym, _):
            if keysym == 0xFF1B:  # Esc
                malibx.loop_end(mlx)

    Docs: https://0xmthan.github.io/malibx/reference/loop_end/
    """

# Windows and drawing

def new_window(mlx: Mlx, size_x: int, size_y: int, title: str) -> Window:
    """Open a window with a black drawing area. Like `mlx_new_window`.

    The window isn't resizable, and is shown once `loop()` runs. Anything
    drawn before then appears on its first frame.

    Args:
        mlx: The connection from `init()`.
        size_x: Width of the drawing area, in pixels.
        size_y: Height of the drawing area, in pixels.
        title: Text in the title bar.

    Returns:
        A `Window` handle.

    Raises:
        RuntimeError: The window couldn't be created (e.g. no display).

    Docs: https://0xmthan.github.io/malibx/reference/new_window/
    """

def clear_window(mlx: Mlx, win: Window) -> None:
    """Fill the whole window with black. Like `mlx_clear_window`.

    Args:
        mlx: The connection from `init()`.
        win: The window to clear.

    Raises:
        RuntimeError: The window was closed.

    Docs: https://0xmthan.github.io/malibx/reference/clear_window/
    """

def pixel_put(mlx: Mlx, win: Window, x: int, y: int, color: int) -> None:
    """Set one pixel of a window. Like `mlx_pixel_put`.

    (0, 0) is the top-left corner; y grows downwards. Pixels outside the
    window are ignored. For large areas, draw into an image instead: it's
    much faster.

    Args:
        mlx: The connection from `init()`.
        win: The window to draw in.
        x: Column, 0 = left edge.
        y: Row, 0 = top edge.
        color: 0xRRGGBB, e.g. 0xFF0000 for red.

    Raises:
        RuntimeError: The window was closed.

    Docs: https://0xmthan.github.io/malibx/reference/pixel_put/
    """

def string_put(mlx: Mlx, win: Window, x: int, y: int, color: int, string: str) -> None:
    """Draw text with the built-in 8x8 font. Like `mlx_string_put`.

    `y` is the BASELINE, not the top: at the default size glyphs rise 7 px
    above it. Each character is 8 px wide (times the `set_font()` scale).
    Only the glyph pixels are drawn. ASCII and Latin-1 are supported; `\\n` is
    not a line break.

    Args:
        mlx: The connection from `init()`.
        win: The window to draw in.
        x: Left edge of the first character.
        y: Baseline of the text.
        color: 0xRRGGBB.
        string: The text.

    Raises:
        RuntimeError: The window was closed.

    Docs: https://0xmthan.github.io/malibx/reference/string_put/
    """

def set_font(mlx: Mlx, win: Window, name: str) -> None:
    """Set the text size for `string_put()` in this window. Like `mlx_set_font`.

    The built-in font is scaled by a whole number, so text stays sharp. The
    size stays until changed; new windows start at "8x8".

    Args:
        mlx: The connection from `init()`.
        win: The window whose text size to set.
        name: "8x8", "16x16", "24x24", "32x32", "40x40", "48x48", "56x56"
            or "64x64".

    Raises:
        ValueError: `name` is not one of the sizes above.
        RuntimeError: The window was closed.

    Docs: https://0xmthan.github.io/malibx/reference/set_font/
    """

# Images

def new_image(mlx: Mlx, width: int, height: int) -> Image:
    """Create an off-screen image, all opaque black. Like `mlx_new_image`.

    Fill it through `get_data_addr()`, draw it with `put_image_to_window()`.

    Args:
        mlx: The connection from `init()`.
        width: Width in pixels, at least 1.
        height: Height in pixels, at least 1.

    Returns:
        The new `Image`.

    Raises:
        ValueError: `width` or `height` is 0.

    Docs: https://0xmthan.github.io/malibx/reference/new_image/
    """

def get_data_addr(img: Image) -> tuple[memoryview, int, int, int]:
    """Get writable access to an image's pixels. Like `mlx_get_data_addr`.

    Each pixel is 4 bytes B, G, R, A; pixel (x, y) starts at
    `y * size_line + x * 4`. Write with `color.to_bytes(4, "little")`.
    Alpha 0xFF = transparent when drawn, anything else = opaque.

    Args:
        img: The image.

    Returns:
        `(buffer, bits_per_pixel, size_line, endian)`: a writable memoryview,
        always 32, width * 4, and 0 (little endian).

    Raises:
        RuntimeError: The image was destroyed.

    Example:
        buf, bpp, size_line, endian = malibx.get_data_addr(img)
        i = y * size_line + x * 4
        buf[i:i + 4] = (0xFF0000).to_bytes(4, "little")  # red pixel

    Docs: https://0xmthan.github.io/malibx/reference/get_data_addr/
    """

def put_image_to_window(mlx: Mlx, win: Window, img: Image, x: int, y: int) -> None:
    """Draw an image into a window, top-left at (x, y). Like `mlx_put_image_to_window`.

    Pixels with alpha 0xFF are skipped (transparent); all others are opaque.
    The image is clipped to the window, so x and y may be negative. Pixels
    are copied now: later changes to the image need another call.

    Args:
        mlx: The connection from `init()`.
        win: The window to draw in.
        img: The image to draw.
        x: Window column of the image's left edge.
        y: Window row of the image's top edge.

    Raises:
        RuntimeError: The window was closed or the image was destroyed.

    Docs: https://0xmthan.github.io/malibx/reference/put_image_to_window/
    """

def destroy_image(mlx: Mlx, img: Image) -> None:
    """Free an image's pixels now. Like `mlx_destroy_image`.

    Usually unnecessary: Python frees images when they're no longer used.
    Using the image afterwards raises `RuntimeError`.

    Args:
        mlx: The connection from `init()`.
        img: The image to free.

    Docs: https://0xmthan.github.io/malibx/reference/destroy_image/
    """

def xpm_to_image(mlx: Mlx, xpm_data: list[str]) -> tuple[Image, int, int]:
    """Create an image from XPM data in memory. Like `mlx_xpm_to_image`.

    `xpm_data` is the XPM's strings: the header "W H NCOLORS CPP", then the
    color lines, then the pixel rows. `None` colors become transparent.

    Args:
        mlx: The connection from `init()`.
        xpm_data: The XPM strings, in order.

    Returns:
        `(image, width, height)`.

    Raises:
        ValueError: The XPM data is malformed.

    Example:
        img, w, h = malibx.xpm_to_image(mlx, [
            "3 2 2 1", ". c None", "r c #FF0000",
            "r.r",
            ".r.",
        ])

    Docs: https://0xmthan.github.io/malibx/reference/xpm_to_image/
    """

def xpm_file_to_image(mlx: Mlx, filename: str | os.PathLike[str]) -> tuple[Image, int, int]:
    """Load an .xpm file into an image. Like `mlx_xpm_file_to_image`.

    Colors can be #RGB, #RRGGBB, #RRRRGGGGBBBB, an X11 name ("light blue") or
    None (transparent). Unknown colors become black. Relative paths are
    resolved from the current working directory, not the script's.

    Args:
        mlx: The connection from `init()`.
        filename: Path to the .xpm file.

    Returns:
        `(image, width, height)`.

    Raises:
        FileNotFoundError: The file doesn't exist.
        ValueError: The XPM data is malformed.

    Docs: https://0xmthan.github.io/malibx/reference/xpm_file_to_image/
    """

# Events

def hook(win: Window, x_event: int, x_mask: int, f: Callable[..., Any], param: Any = ...) -> None:
    """Call `f` on an X11 event in this window. Like `mlx_hook`.

    Supported `x_event` values and how `f` is called:
        2  KeyPress       f(keysym, param)        (repeats while held)
        3  KeyRelease     f(keysym, param)
        4  ButtonPress    f(button, x, y, param)
        5  ButtonRelease  f(button, x, y, param)
        6  MotionNotify   f(x, y, param)
        12 Expose         f(param)                (first frame only)
        17 DestroyNotify  f(param)                (close button)

    One function per event per window: registering again replaces it.
    `x_mask` is accepted for compatibility and ignored.

    Args:
        win: The window to listen on.
        x_event: X11 event number (see above).
        x_mask: Ignored. Pass the usual X11 mask or 0.
        f: Called with the event's arguments, then `param`.
        param: Passed as the last argument to `f`. Defaults to `None`.

    Raises:
        ValueError: `x_event` is outside 0-35.
        RuntimeError: The window was closed.

    Example:
        malibx.hook(win, 2, 1 << 0, on_key_down, state)
        malibx.hook(win, 17, 0, on_close, state)

    Docs: https://0xmthan.github.io/malibx/reference/hook/
    """

def key_hook(win: Window, f: Callable[[int, _T], Any], param: _T = ...) -> None:
    """Call `f(keysym, param)` when a key is RELEASED. Like `mlx_key_hook`.

    For key press, use `hook(win, 2, 1 << 0, f, param)`. Keysyms are X11:
    letters are lowercase ASCII (ord("a")), Esc 0xFF1B, Enter 0xFF0D,
    arrows 0xFF51 (left) 0xFF52 (up) 0xFF53 (right) 0xFF54 (down).

    Args:
        win: The window to listen on.
        f: Called as `f(keysym, param)`.
        param: Passed to `f`. Defaults to `None`.

    Raises:
        RuntimeError: The window was closed.

    Docs: https://0xmthan.github.io/malibx/reference/key_hook/
    """

def mouse_hook(win: Window, f: Callable[[int, int, int, _T], Any], param: _T = ...) -> None:
    """Call `f(button, x, y, param)` on a mouse button press. Like `mlx_mouse_hook`.

    Buttons: 1 left, 2 middle, 3 right, 4 scroll up, 5 scroll down.
    x and y are window pixels from the top-left.

    Args:
        win: The window to listen on.
        f: Called as `f(button, x, y, param)`.
        param: Passed to `f`. Defaults to `None`.

    Raises:
        RuntimeError: The window was closed.

    Docs: https://0xmthan.github.io/malibx/reference/mouse_hook/
    """

def expose_hook(win: Window, f: Callable[[_T], Any], param: _T = ...) -> None:
    """Call `f(param)` when the window is first shown. Like `mlx_expose_hook`.

    Fires once, on the window's first frame in `loop()`.

    Args:
        win: The window to listen on.
        f: Called as `f(param)`.
        param: Passed to `f`. Defaults to `None`.

    Raises:
        RuntimeError: The window was closed.

    Docs: https://0xmthan.github.io/malibx/reference/expose_hook/
    """
