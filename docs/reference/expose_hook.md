# expose_hook

```python
malibx.expose_hook(win: Window, f: Callable[[T], Any], param: T = None) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_expose_hook</code></p>

Calls `f(param)` when the window is shown on screen for the first time.

## Parameters

| Name | Type | Description |
|---|---|---|
| `win` | `Window` | The window to listen on |
| `f` | `Callable[[T], Any]` | Called as `f(param)` |
| `param` | `T` | Passed to `f`. Defaults to `None`. |

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- It fires **once**, during the window's first frame in [`loop`](loop.md).
  Register it before that frame, or it never fires.
- In X11, Expose also fires when a covered window is uncovered, and programs
  redraw then. malibx keeps every window's pixels and redraws them itself, so
  you never need to handle that.
- It's the same as `hook(win, 12, 1 << 15, f, param)`, and the two replace
  each other.

## Example

```python
def shown(_):
    print("window is on screen")

malibx.expose_hook(win, shown)
```

## See also

[`hook`](hook.md)
