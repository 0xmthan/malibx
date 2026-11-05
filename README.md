# malibx

A tiny window and pixel library for Python, written in Rust, with a
[MiniLibX](https://github.com/42Paris/minilibx-linux)-style API.
It runs natively on macOS and Linux.

## Install

Download the wheel for your platform from
[Releases](https://github.com/0xmthan/malibx/releases), then:

```sh
pip install malibx-<version>-cp311-abi3-<platform>.whl
```

Wheels: Linux x86_64, macOS arm64 and macOS x86_64. They work on any Python 3.11+.

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
| `loop_hook(mlx, f, param=None)` | `mlx_loop_hook` | Calls `f(param)` once per frame |
| `loop(mlx)` | `mlx_loop` | Runs until `loop_end` is called or all windows are closed |
| `loop_end(mlx)` | `mlx_loop_end` | Makes `loop` return |

The loop runs at 60 fps. See [mlx-reference.md](mlx-reference.md) for the full
MiniLibX API this library follows.

## Development

Needs Rust and [uv](https://docs.astral.sh/uv/).

| Command | Does |
|---|---|
| `make` | Create `.venv` and build and install malibx into it |
| `make run` | Run `examples/hello.py` |
| `make wheel` | Build a release wheel into `dist/` |
| `make lint` | `cargo fmt --check`, `clippy` and `cargo audit` |
| `make clean` / `fclean` | Remove build files / also remove `.venv` |

## Releasing

Bump `version` in `Cargo.toml` and push to `main`. CI runs the lint and audit
checks, and if no release exists for that version yet, it builds the wheels and
publishes them as release `v<version>`.

## License

MIT, see [LICENSE](LICENSE).
