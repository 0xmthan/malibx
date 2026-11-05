# init

```python
malibx.init() -> Mlx
```

<p class="mx-mlx">MiniLibX: <code>mlx_init</code></p>

Creates the connection handle that the rest of malibx works through.

## Returns

An [`Mlx`](index.md#mlx) object. It holds your windows and your loop hook,
and you pass it as the first argument to most other functions.

## Notes

- Call it **once** at the start of your program. Each `Mlx` is independent:
  its [`loop`](loop.md) only runs its own windows, so a second one is
  rarely useful.
- Unlike `mlx_init`, it never fails. Nothing talks to the windowing system
  until you call [`new_window`](new_window.md), so display errors show up
  there.
- An `Mlx` and its windows belong to the thread that created them, so use
  them only from that thread. Run malibx on your program's main thread: macOS
  requires windows to live there.

## Example

```python
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 640, 480, "hello")
malibx.loop(mlx)
```

## See also

[`new_window`](new_window.md) · [`loop`](loop.md)
