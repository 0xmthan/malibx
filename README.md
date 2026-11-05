# malibx

A tiny window and pixel library for Python, written in Rust, with a
[MiniLibX](https://github.com/42Paris/minilibx-linux)-style API.
It runs natively on macOS, Linux and Windows.

**Documentation: <https://0xmthan.github.io/malibx/>**

## Install

```sh
pip install malibx
```

Wheels for Linux (x86_64, arm64), macOS (arm64, x86_64) and Windows (x64), on
any Python 3.11+. They're also attached to each
[GitHub release](https://github.com/0xmthan/malibx/releases).

## Usage

```python
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 640, 480, "hello")
malibx.loop_hook(mlx, lambda param: None)
malibx.loop(mlx)
```

## Functions

Same names as MiniLibX.

| malibx | MiniLibX | Does |
|---|---|---|
| `init()` | `mlx_init` | Returns the connection handle |
| `new_window(mlx, w, h, title)` | `mlx_new_window` | Opens a window, returns a window handle |
| `clear_window(mlx, win)` | `mlx_clear_window` | Fills the window with black |
| `pixel_put(mlx, win, x, y, color)` | `mlx_pixel_put` | Draws one pixel (`0xRRGGBB`) |
| `new_image(mlx, w, h)` | `mlx_new_image` | Creates a black image |
| `get_data_addr(img)` | `mlx_get_data_addr` | Returns `(buffer, bits_per_pixel, size_line, endian)` |
| `put_image_to_window(mlx, win, img, x, y)` | `mlx_put_image_to_window` | Draws the image at (x, y) |
| `destroy_image(mlx, img)` | `mlx_destroy_image` | Frees the image |
| `xpm_file_to_image(mlx, path)` | `mlx_xpm_file_to_image` | Loads an `.xpm` file, returns `(img, width, height)` |
| `xpm_to_image(mlx, xpm_data)` | `mlx_xpm_to_image` | Same, from a list of the XPM's strings |
| `string_put(mlx, win, x, y, color, string)` | `mlx_string_put` | Draws text; `y` is the baseline |
| `set_font(mlx, win, name)` | `mlx_set_font` | Text size for the window: `"8x8"` (default), `"16x16"`, … `"64x64"` |
| `hook(win, x_event, x_mask, f, param=None)` | `mlx_hook` | Calls `f` on an X11 event (see below) |
| `key_hook(win, f, param=None)` | `mlx_key_hook` | `f(keysym, param)` on key **release** |
| `mouse_hook(win, f, param=None)` | `mlx_mouse_hook` | `f(button, x, y, param)` on button press |
| `expose_hook(win, f, param=None)` | `mlx_expose_hook` | `f(param)` when the window is first shown |
| `loop_hook(mlx, f, param=None)` | `mlx_loop_hook` | Calls `f(param)` once per frame |
| `loop(mlx)` | `mlx_loop` | Runs until `loop_end` is called or all windows are closed |
| `loop_end(mlx)` | `mlx_loop_end` | Makes `loop` return |

Events for `hook`:

| `x_event` | Event | `f` gets |
|---|---|---|
| 2 / 3 | KeyPress / KeyRelease | `(keysym, param)` |
| 4 / 5 | ButtonPress / ButtonRelease | `(button, x, y, param)` |
| 6 | MotionNotify | `(x, y, param)` |
| 12 | Expose | `(param)` |
| 17 | DestroyNotify (close button) | `(param)` |

- The loop runs at 60 fps. Each frame: your loop hook, then the window is
  shown, then the event hooks for that frame.
- Keys are X11 keysyms like in MiniLibX: arrows `0xff51`–`0xff54`, Esc
  `0xff1b`, Enter `0xff0d`, letters and digits are their lowercase ASCII code.
- Mouse buttons: 1 left, 2 middle, 3 right, 4/5 scroll up/down.
- `x_mask` is accepted but ignored; every supported event is always reported.
- Clicking the close button closes the window right away; hook 17 runs just after.
- `buffer` is a writable `memoryview`: 32 bits per pixel, little endian, so
  pixel (x, y) is 4 bytes `B, G, R, A` at `y * size_line + x * 4`.
- Transparency works like MiniLibX: pixels with alpha `0xFF` are not drawn,
  anything else is drawn opaque. XPM `None` pixels get alpha `0xFF`.
- XPM colors: `#RGB`, `#RRGGBB`, `#RRRRGGGGBBBB`, `None` or an X11 color name
  (`white`, `light blue`, …). Unknown colors become black, like MiniLibX.
- Windows get the malibx skull as their icon: the Dock icon on macOS, the
  window icon on Linux X11. (Wayland and Windows keep the system default.)
- Text uses a built-in 8x8 font. `set_font` only scales it: `"16x16"` draws
  each character 16 px wide, and so on in steps of 8 up to `"64x64"`.

See [mlx-reference.md](mlx-reference.md) for the full MiniLibX API this library
follows. Type hints are included.

## Development

Needs Rust and [uv](https://docs.astral.sh/uv/).

| Command | Does |
|---|---|
| `make` | Create `.venv` and build and install malibx into it |
| `make run` | Run `examples/hello.py` (`EXAMPLE=draw`, `keys` or `sprites` for the others) |
| `make wheel` | Build a release wheel into `dist/` |
| `make docs` / `docs-serve` | Build the docs into `site/` / preview them at http://127.0.0.1:8000 |
| `make test` | Run the Rust unit tests |
| `make lint` | `cargo fmt --check`, `clippy`, tests and `cargo audit` |
| `make clean` / `fclean` | Remove build files / also remove `.venv` |

## Releasing

Bump `version` in `Cargo.toml` and push to `main`. CI runs the lint, test and
audit checks, and if no release exists for that version yet, it builds the wheels and
publishes them to PyPI and as GitHub release `v<version>`.

## License

MIT, see [LICENSE](LICENSE).
