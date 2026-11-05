# MiniLibX reference

What malibx has to match. Everything here is taken from the [official source](https://github.com/42Paris/minilibx-linux).
X11 event codes and keysyms: [xorgproto](https://gitlab.freedesktop.org/xorg/proto/xorgproto).

## 1. All functions (30)

### Setup / teardown
| Function | Signature | Notes |
|---|---|---|
| `mlx_init` | `void *mlx_init()` | Connects to the display. Returns `NULL` on failure. |
| `mlx_destroy_display` | `int (void *mlx_ptr)` | Closes the connection. |
| `mlx_get_screen_size` | `int (void *mlx_ptr, int *sizex, int *sizey)` | Screen resolution. |

### Window
| Function | Signature | Notes |
|---|---|---|
| `mlx_new_window` | `void *(void *mlx_ptr, int size_x, int size_y, char *title)` | Returns `NULL` on failure. Multiple windows allowed. |
| `mlx_clear_window` | `int (void *mlx_ptr, void *win_ptr)` | Clears the window **in black**. |
| `mlx_destroy_window` | `int (void *mlx_ptr, void *win_ptr)` | |

### Drawing directly to a window
| Function | Signature | Notes |
|---|---|---|
| `mlx_pixel_put` | `int (void *mlx_ptr, void *win_ptr, int x, int y, int color)` | One pixel. (0,0) = top-left, x right, y down. Nothing drawn outside the window. |
| `mlx_string_put` | `int (void *mlx_ptr, void *win_ptr, int x, int y, int color, char *string)` | Text at (x, y). Implemented with `XDrawString`. |
| `mlx_set_font` | `void (void *mlx_ptr, void *win_ptr, char *name)` | Font for `mlx_string_put`. Only fixed-width bitmap fonts. |
| `mlx_get_color_value` | `int (void *mlx_ptr, int color)` | Converts an RGB int to the display's pixel format. The man page says it returns `unsigned int`. |

### Images (off-screen buffers)
| Function | Signature | Notes |
|---|---|---|
| `mlx_new_image` | `void *(void *mlx_ptr, int width, int height)` | Returns `NULL` on failure. |
| `mlx_get_data_addr` | `char *(void *img_ptr, int *bits_per_pixel, int *size_line, int *endian)` | Raw pixel buffer. See section 3. |
| `mlx_put_image_to_window` | `int (void *mlx_ptr, void *win_ptr, void *img_ptr, int x, int y)` | Draws the image at (x, y). Respects XPM transparency. |
| `mlx_xpm_to_image` | `void *(void *mlx_ptr, char **xpm_data, int *width, int *height)` | Image from XPM data in memory. Fills in width/height. |
| `mlx_xpm_file_to_image` | `void *(void *mlx_ptr, char *filename, int *width, int *height)` | Image from an `.xpm` file. Fills in width/height. |
| `mlx_destroy_image` | `int (void *mlx_ptr, void *img_ptr)` | |

### Events / loop
| Function | Signature | Notes |
|---|---|---|
| `mlx_key_hook` | `int (void *win_ptr, int (*f)(int keycode, void *param), void *param)` | Fires on key **release** (see section 2). |
| `mlx_mouse_hook` | `int (void *win_ptr, int (*f)(int button, int x, int y, void *param), void *param)` | Fires on button **press**. |
| `mlx_expose_hook` | `int (void *win_ptr, int (*f)(void *param), void *param)` | The window needs redrawing. |
| `mlx_hook` | `int (void *win_ptr, int x_event, int x_mask, int (*f)(), void *param)` | Generic hook for any X11 event (see section 2). |
| `mlx_loop_hook` | `int (void *mlx_ptr, int (*f)(void *param), void *param)` | Called when no events are pending. This is the "every frame" function. |
| `mlx_loop` | `int (void *mlx_ptr)` | Runs the event loop. Only returns after `mlx_loop_end` or once every window is destroyed. |
| `mlx_loop_end` | `int (void *mlx_ptr)` | Makes `mlx_loop` return. |
| `mlx_do_sync` | `int (void *mlx_ptr)` | Flushes pending drawing commands (`XSync`). |
| `mlx_do_key_autorepeatoff` | `int (void *mlx_ptr)` | Disables key auto-repeat (system-wide, `XAutoRepeatOff`). |
| `mlx_do_key_autorepeaton` | `int (void *mlx_ptr)` | Re-enables it. |

### Mouse
| Function | Signature | Notes |
|---|---|---|
| `mlx_mouse_get_pos` | `int (void *mlx_ptr, void *win_ptr, int *x, int *y)` | Position relative to the window. (0,0) if the pointer is on another screen. |
| `mlx_mouse_move` | `int (void *mlx_ptr, void *win_ptr, int x, int y)` | Moves the pointer. |
| `mlx_mouse_hide` | `int (void *mlx_ptr, void *win_ptr)` | |
| `mlx_mouse_show` | `int (void *mlx_ptr, void *win_ptr)` | |

## 2. How events work

### Loop
From `mlx_loop.c`: while at least one window exists and `mlx_loop_end` hasn't been
called:
1. Handle all pending X events, calling the hook registered for each one.
2. If a loop hook is set, call it once.

With no loop hook, the loop just blocks waiting for events.

### Hook argument formats
Taken from `mlx_int_param_event.c`. The `x_event` number is from X11's `X.h`.

| X event | Code | Callback gets | Set by |
|---|---|---|---|
| KeyPress | 2 | `(int keysym, void *param)` | `mlx_hook(win, 2, 1L<<0, f, p)` |
| KeyRelease | 3 | `(int keysym, void *param)` | `mlx_key_hook`, or `mlx_hook(win, 3, 1L<<1, f, p)` |
| ButtonPress | 4 | `(int button, int x, int y, void *param)` | `mlx_mouse_hook` |
| ButtonRelease | 5 | `(int button, int x, int y, void *param)` | `mlx_hook` |
| MotionNotify | 6 | `(int x, int y, void *param)` | `mlx_hook` |
| Expose | 12 | `(void *param)`, only for the last expose in a series | `mlx_expose_hook` |
| any other (e.g. DestroyNotify 17) | | `(void *param)` | `mlx_hook` |

Important details:
- `mlx_key_hook` is wired to **KeyRelease**, not KeyPress (`mlx_key_hook.c`).
  Games that need "is key held" register KeyPress (2) and KeyRelease (3) with
  `mlx_hook` and track the state themselves.
- **Keycodes are X11 keysyms** (from `keysymdef.h`), not raw scancodes. The
  ones Pac-Man needs:

  | Key | Keysym |
  |---|---|
  | Escape | `0xff1b` |
  | Return | `0xff0d` |
  | BackSpace | `0xff08` |
  | Left / Up / Right / Down | `0xff51` / `0xff52` / `0xff53` / `0xff54` |
  | `a`–`z` | `0x61`–`0x7a` (ASCII) |
  | `0`–`9` | `0x30`–`0x39` (ASCII) |
  | Space | `0x20` |

- Closing the window with the red X is **DestroyNotify (17)**, used as
  `mlx_hook(win, 17, 0, f, p)`.
- `param` is passed back untouched and never read by MiniLibX.

## 3. Colors and image memory

**Color int** (from the `mlx_pixel_put` man page): `0xAARRGGBB`. Blue is the lowest byte.

| Bits | 31..24 | 23..16 | 15..8 | 7..0 |
|---|---|---|---|---|
| Channel | α | R | G | B |

Alpha is inverted: `0x00` = opaque, `0xFF` = transparent (MLX README).

**Image buffer** (from `mlx_get_data_addr`):
- Returns a pointer to the first pixel of the first row.
- `bits_per_pixel` is the size of one pixel (32 on normal displays).
- `size_line` is the bytes per row. It can be larger than `width * bpp / 8`, so
  always step rows with `size_line`.
- `endian`: 0 means little endian, 1 means big endian.
- The pixel at (x, y) is at `addr + y * size_line + x * (bits_per_pixel / 8)`.

**Transparency:** XPM pixels marked `None` are stored as `0xFF000000`
(`mlx_xpm.c`), i.e. alpha `0xFF`, which the README defines as fully transparent.
The old clip-mask code in `mlx_xpm.c` is commented out, so transparency relies
on that alpha value.
