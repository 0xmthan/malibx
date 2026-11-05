# mouse_show

```python
malibx.mouse_show(mlx: Mlx, win: Window) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_mouse_show</code></p>

Shows the mouse pointer again after [`mouse_hide`](mouse_hide.md).

## Parameters

| Name | Type | Description |
|---|---|---|
| `mlx` | `Mlx` | The connection from [`init`](init.md) |
| `win` | `Window` | The window to show the pointer in |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- Showing a pointer that's already visible does nothing.
- The pointer is visible by default in every new window.

## Example

```python
def toggle_pause(st):
    st["paused"] = not st["paused"]
    if st["paused"]:
        malibx.mouse_show(mlx, win)   # let the player click menus
    else:
        malibx.mouse_hide(mlx, win)
```

## See also

[`mouse_hide`](mouse_hide.md)
