# Installation

malibx needs **Python 3.11 or newer**. Prebuilt wheels are published for:

| Platform | Architectures |
|---|---|
| :material-apple: macOS | Apple Silicon (arm64), Intel (x86_64) |
| :material-linux: Linux | x86_64, arm64 |
| :material-microsoft-windows: Windows | x64 |

The wheels use Python's stable ABI, so one wheel works with every Python from
3.11 onwards.

## Install

=== "pip"

    ```sh
    python -m pip install malibx
    ```

=== "uv"

    ```sh
    uv add malibx            # in a uv project
    uv pip install malibx    # or into the active environment
    ```

=== "From a GitHub release"

    Download the `.whl` for your platform from the
    [releases page](https://github.com/0xmthan/malibx/releases), then:

    ```sh
    python -m pip install malibx-*.whl
    ```

!!! tip "Use a virtual environment"
    Install into a virtual environment (`python -m venv .venv`, or let uv
    handle it) rather than your system Python. On recent macOS and Linux
    distributions, installing into the system Python is often blocked anyway.

## Check that it works

Save this as `check.py` and run it with `python check.py`:

```python
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 320, 120, "malibx works")
malibx.string_put(mlx, win, 16, 64, 0xFFFFFF, "it works! close me.")
malibx.loop(mlx)
print("bye")
```

A small black window with white text should open. Close it with the window's
close button, and `bye` is printed in the terminal.

## Editor support

malibx comes with type hints (a `malibx.pyi` stub). VS Code, PyCharm and
other editors autocomplete every function and its parameters, and type
checkers like mypy and Pyright check your calls.

## Troubleshooting

??? question "`pip` says *no matching distribution found for malibx*"
    There's no wheel for your Python version or platform. Check that:

    - `python --version` shows **3.11 or newer**;
    - you're on one of the platforms in the table above. On Linux, wheels are
      built for glibc (manylinux), so Alpine and other musl-based distributions
      aren't covered.

    You can still [build from source](#building-from-source).

??? question "The window doesn't open on Linux"
    malibx needs a graphical desktop session, X11 or Wayland. It can't open
    windows over a plain SSH session or in a headless container. On WSL, use
    Windows 11 with WSLg, or install malibx with Windows' own Python instead.

??? question "Nothing happens and my terminal is stuck"
    `malibx.loop` blocks until every window is closed. That's intended: it's
    the program's main loop. Close the window, or press ++ctrl+c++ in the
    terminal to stop it. See [The main loop](the-loop.md).

## Building from source

You'll need a [Rust toolchain](https://rustup.rs) and
[uv](https://docs.astral.sh/uv/).

```sh
git clone https://github.com/0xmthan/malibx
cd malibx
make          # creates .venv and builds malibx into it
make run      # runs examples/hello.py
```

To build a wheel you can install anywhere, run `make wheel`. The wheel is
written to `dist/`.

---

Installed? Next, [open your first window :material-arrow-right:](first-window.md)
