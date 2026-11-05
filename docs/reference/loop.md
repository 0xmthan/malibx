# loop

```python
malibx.loop(mlx: Mlx) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_loop</code></p>

Runs the main loop. It shows your windows, calls your hooks about 60 times a
second, and returns when there's nothing left to do.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |

## Returns

`None`. `loop` returns when:

- [`loop_end`](loop_end.md) was called; the current frame finishes first, or
- every window of this `mlx` has been closed.

## Raises

| Exception | When |
|---|---|
| `KeyboardInterrupt` | ++ctrl+c++ was pressed in the terminal. It's checked every frame. |
| *anything* | An exception raised by one of your hooks propagates out of `loop` unchanged. |
| `RuntimeError` | The window system failed to show a window (rare). |

## What one frame does

1. Calls the [loop hook](loop_hook.md), if one is set.
2. For each window: if the user closed it, removes it and queues its
   DestroyNotify (event 17). Otherwise it copies the window's pixels to the
   screen, then collects input since the last frame.
3. Calls the event hooks for everything collected, in order.
4. Returns if `loop_end` was called or no windows are left. Otherwise it
   starts the next frame.

Showing a window waits as needed to keep the loop at **60 fps** at most. A
slow loop hook makes frames slower; nothing is skipped. See
[The main loop](../getting-started/the-loop.md) for a full walkthrough.

## Notes

- With no open windows, `loop` returns immediately.
- `loop` can be called again after it returns. Windows that are still open
  keep their pixels and hooks.
- Your hooks may call any malibx function, including `new_window`,
  `loop_hook` and `loop_end`.

## Example

```python
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 640, 480, "loop")
malibx.loop(mlx)          # blocks until the window is closed
print("window closed, loop returned")
```

## See also

[`loop_hook`](loop_hook.md) · [`loop_end`](loop_end.md) ·
[The main loop](../getting-started/the-loop.md)
