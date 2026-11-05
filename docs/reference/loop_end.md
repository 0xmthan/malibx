# loop_end

```python
malibx.loop_end(mlx: Mlx) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_loop_end</code></p>

Makes the running [`loop`](loop.md) return once the current frame is done.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection whose loop should stop |

## Returns

`None`.

## Notes

- The frame still finishes: event hooks already queued for it still run.
- Windows stay open after `loop` returns. You can draw into them and call
  `loop` again.
- Called outside a running loop, it does nothing. The next `loop` starts
  normally.

## Example

```python
import malibx

ESC = 0xFF1B
mlx = malibx.init()
win = malibx.new_window(mlx, 320, 200, "press Esc")

def on_key(keysym, _):
    if keysym == ESC:
        malibx.loop_end(mlx)

malibx.key_hook(win, on_key)
malibx.loop(mlx)
print("bye")
```

## See also

[`loop`](loop.md) · [`hook`](hook.md) (event 17: close button)
