# hook

```python
malibx.hook(win: Window, x_event: int, x_mask: int, f: Callable[..., Any], param: Any = None) -> None
```

<p class="mx-mlx">MiniLibX: <code>mlx_hook</code></p>

Calls `f` whenever the X11 event `x_event` happens in a window. This is the
general version of [`key_hook`](key_hook.md), [`mouse_hook`](mouse_hook.md)
and [`expose_hook`](expose_hook.md).

## Parameters

| Name | Type | Description |
|---|---|---|
| `win` | `Window` | The window to listen on |
| `x_event` | `int` | X11 event number, see below |
| `x_mask` | `int` | X11 event mask. Accepted for compatibility and **ignored**. |
| `f` | `Callable` | Called with the event's arguments, then `param` |
| `param` | `Any` | Passed as the last argument to `f`. Defaults to `None`. |

## Events

| `x_event` | Name | Usual `x_mask` | `f` is called as | Fires when |
|---|---|---|---|---|
| `2` | KeyPress | `1 << 0` | `f(keysym, param)` | a key goes down, then again on auto-repeat while it's held (see [`do_key_autorepeatoff`](do_key_autorepeatoff.md)) |
| `3` | KeyRelease | `1 << 1` | `f(keysym, param)` | a key goes up |
| `4` | ButtonPress | `1 << 2` | `f(button, x, y, param)` | a mouse button goes down, or the wheel scrolls |
| `5` | ButtonRelease | `1 << 3` | `f(button, x, y, param)` | a mouse button goes up, or the wheel scrolls |
| `6` | MotionNotify | `1 << 6` | `f(x, y, param)` | the pointer moves inside the window |
| `12` | Expose | `1 << 15` | `f(param)` | the window is shown for the first time |
| `17` | DestroyNotify | `0` | `f(param)` | the user closed the window |

- `keysym` is an X11 keysym. See the
  [key table](../getting-started/input.md#keys).
- `button` is `1` left, `2` middle, `3` right, `4` scroll up, `5` scroll down.
  A wheel step gives a press **and** a release of 4 or 5.
- `x`, `y` are window pixels from the top-left. For button events they're
  clamped to the window.

## Returns

`None`.

## Raises

| Exception | When |
|---|---|
| `ValueError` | `x_event` is outside `0`–`35`: `x_event must be between 0 and 35` |
| `RuntimeError` | The window was closed: `window is closed` |

## Notes

- **One function per event per window.** Registering the same event again
  replaces the previous function. `key_hook` uses slot 3, `mouse_hook` slot 4
  and `expose_hook` slot 12, so they replace (and are replaced by) `hook`
  calls for those numbers.
- Events other than the seven above are accepted, so MiniLibX code runs, but
  they never fire.
- `x_mask` exists only so MiniLibX code works unchanged. malibx always
  watches every supported event.
- Hooks run after the window is shown in each frame. See
  [The main loop](../getting-started/the-loop.md#frames).
- **Close button (17):** the window is already closed when `f` runs, so
  don't draw to it. If it was the last window, [`loop`](loop.md) returns
  after `f`. Closing a window from code with
  [`destroy_window`](destroy_window.md) doesn't call this hook.
- An exception in `f` stops the loop and propagates out of `loop`.

## Example

```python
ESC = 0xFF1B
state = {"held": set(), "mouse": (0, 0)}

def key_down(key, st):
    if key == ESC:
        malibx.loop_end(mlx)
    st["held"].add(key)

def key_up(key, st):
    st["held"].discard(key)

def moved(x, y, st):
    st["mouse"] = (x, y)

def closed(st):
    print("bye")

malibx.hook(win, 2, 1 << 0, key_down, state)
malibx.hook(win, 3, 1 << 1, key_up, state)
malibx.hook(win, 6, 1 << 6, moved, state)
malibx.hook(win, 17, 0, closed, state)
```

## See also

[`key_hook`](key_hook.md) · [`mouse_hook`](mouse_hook.md) ·
[Keyboard and mouse](../getting-started/input.md)
