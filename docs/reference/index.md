# API reference

Every function in malibx, one page each. The names are MiniLibX's without the
`mlx_` prefix, and the arguments are in the same order.

## Setup and loop

| Function | Does |
|---|---|
| [`init()`](init.md) | Creates the connection handle |
| [`loop(mlx)`](loop.md) | Runs the main loop until the windows close or `loop_end` is called |
| [`loop_hook(mlx, f, param=None)`](loop_hook.md) | Calls `f(param)` once per frame |
| [`loop_end(mlx)`](loop_end.md) | Makes `loop` return |

## Windows and drawing

| Function | Does |
|---|---|
| [`new_window(mlx, size_x, size_y, title)`](new_window.md) | Opens a window |
| [`clear_window(mlx, win)`](clear_window.md) | Fills a window with black |
| [`pixel_put(mlx, win, x, y, color)`](pixel_put.md) | Sets one pixel |
| [`string_put(mlx, win, x, y, color, string)`](string_put.md) | Draws text; `y` is the baseline |
| [`set_font(mlx, win, name)`](set_font.md) | Sets the text size: `"8x8"` to `"64x64"` |

## Images

| Function | Does |
|---|---|
| [`new_image(mlx, width, height)`](new_image.md) | Creates a black off-screen image |
| [`get_data_addr(img)`](get_data_addr.md) | Returns `(buffer, bits_per_pixel, size_line, endian)` |
| [`put_image_to_window(mlx, win, img, x, y)`](put_image_to_window.md) | Draws an image into a window |
| [`destroy_image(mlx, img)`](destroy_image.md) | Frees an image |
| [`xpm_file_to_image(mlx, filename)`](xpm_file_to_image.md) | Loads an `.xpm` file → `(img, width, height)` |
| [`xpm_to_image(mlx, xpm_data)`](xpm_to_image.md) | Same, from a list of strings |

## Events

| Function | Does |
|---|---|
| [`hook(win, x_event, x_mask, f, param=None)`](hook.md) | Calls `f` on any supported X11 event |
| [`key_hook(win, f, param=None)`](key_hook.md) | `f(keysym, param)` on key **release** |
| [`mouse_hook(win, f, param=None)`](mouse_hook.md) | `f(button, x, y, param)` on button press |
| [`expose_hook(win, f, param=None)`](expose_hook.md) | `f(param)` when the window is first shown |

## Types

malibx has three handle types. You get them from functions; you can't
create them directly.

### Mlx

The connection, returned by [`init`](init.md). It's MiniLibX's
`void *mlx_ptr` and owns the windows and the loop hook.

### Window

A window, returned by [`new_window`](new_window.md). It's MiniLibX's
`void *win_ptr`. A window remembers its `Mlx`, so the hook functions only need
the window. Once the user closes it, functions that use it raise
`RuntimeError: window is closed`.

### Image

An off-screen image, returned by [`new_image`](new_image.md),
[`xpm_file_to_image`](xpm_file_to_image.md) and
[`xpm_to_image`](xpm_to_image.md). It's MiniLibX's `void *img_ptr`.
Its pixels are reached through [`get_data_addr`](get_data_addr.md).

## Not in malibx

These MiniLibX functions aren't implemented:

| MiniLibX | Instead |
|---|---|
| `mlx_destroy_window` | The user closes windows. To stop from code, call [`loop_end`](loop_end.md). |
| `mlx_destroy_display` | Not needed; Python cleans up on exit. |
| `mlx_get_screen_size` | — |
| `mlx_get_color_value` | Not needed; colors are always `0xRRGGBB`. |
| `mlx_do_sync` | Not needed; windows are shown every frame. |
| `mlx_do_key_autorepeatoff` / `on` | Track held keys in a `set` (see [Smooth movement](../getting-started/input.md#smooth-movement)). |
| `mlx_mouse_get_pos` | Store the position from a MotionNotify [`hook`](hook.md) (event 6). |
| `mlx_mouse_move`, `mlx_mouse_hide`, `mlx_mouse_show` | — |
