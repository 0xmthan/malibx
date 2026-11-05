# destroy_window

```python
malibx.destroy_window(mlx: Mlx, win: Window) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_destroy_window</code></p>

Closes a window from your code.

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to close |

## Returns

`None`.

## Notes

- The window disappears immediately, together with its pixels and hooks.
- Its DestroyNotify hook (event 17) is **not** called. That hook only fires
  when the *user* clicks the close button. This means you can safely call
  `destroy_window` from inside that hook, which is a common MiniLibX pattern.
- Afterwards, functions that take this window raise
  `RuntimeError: window is closed`.
- Destroying a window that's already closed does nothing.
- If it was the last open window, [`loop`](loop.md) returns at the end of the
  current frame.

## Example

```python
ESC = 0xFF1B

def on_key(keysym, _):
    if keysym == ESC:
        malibx.destroy_window(mlx, win)   # last window: loop returns

malibx.hook(win, 2, 1 << 0, on_key)
malibx.loop(mlx)
```

## See also

[`destroy_display`](destroy_display.md) · [`loop_end`](loop_end.md) ·
[`hook`](hook.md) (event 17)
