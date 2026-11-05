# loop_hook

```python
malibx.loop_hook(mlx: Mlx, f: Callable[[T], Any], param: T = None) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_loop_hook</code></p>

Sets a function to call at the start of every frame of [`loop`](loop.md).
Do your per-frame updating and drawing here.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `f` | `Callable[[T], Any]` | Called as `f(param)` once per frame. Its return value is ignored. |
| `param` | `T` | Passed to `f` each time. Defaults to `None`. |

## Returns

`None`.

## Notes

- There's **one** loop hook per `mlx`. Calling `loop_hook` again replaces it.
- The hook runs *before* the windows are shown, so anything it draws appears
  on screen in the same frame.
- Windows aren't cleared between frames. Call
  [`clear_window`](clear_window.md), or draw a full-window background, if
  things move.
- An exception in `f` stops the loop and propagates out of
  [`loop`](loop.md).
- `f` isn't checked when you register it. If it isn't callable, or takes the
  wrong number of arguments, you get a `TypeError` from `loop` on the first
  frame.

## Example

```python
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 320, 100, "counter")

def frame(st):
    malibx.clear_window(mlx, win)
    malibx.string_put(mlx, win, 10, 20, 0xFFFFFF, f"frame {st['n']}")
    st["n"] += 1

malibx.loop_hook(mlx, frame, {"n": 0})
malibx.loop(mlx)
```

## See also

[`loop`](loop.md) · [`hook`](hook.md) ·
[Passing state with param](../getting-started/the-loop.md#passing-state-with-param)
