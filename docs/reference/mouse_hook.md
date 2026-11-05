# mouse_hook

```python
malibx.mouse_hook(win: Window, f: Callable[[int, int, int, T], Any], param: T = None) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_mouse_hook</code></p>

Calls `f(button, x, y, param)` when a mouse button is **pressed** in the
window, or the wheel scrolls.

## Parameters

| Name | Type | Description |
|---|---|---|
| `win` | `Window` | The window to listen on |
| `f` | `Callable[[int, int, int, T], Any]` | Called as `f(button, x, y, param)` |
| `param` | `T` | Passed to `f`. Defaults to `None`. |

`f` receives:

| Argument | Meaning |
|---|---|
| `button` | `1` left, `2` middle, `3` right, `4` scroll up, `5` scroll down |
| `x`, `y` | Pointer position in window pixels, from the top-left |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- It's the same as `hook(win, 4, 1 << 2, f, param)`, and the two replace
  each other.
- For button releases use `hook(win, 5, …)`. For pointer movement use
  `hook(win, 6, …)`.

## Example

```python
def on_click(button, x, y, _):
    if button == 1:
        malibx.pixel_put(mlx, win, x, y, 0xFFFFFF)
    elif button in (4, 5):
        print("scrolled", "up" if button == 4 else "down")

malibx.mouse_hook(win, on_click)
```

## See also

[`hook`](hook.md) · [Keyboard and mouse](../getting-started/input.md#mouse)
