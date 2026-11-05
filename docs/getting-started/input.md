# Keyboard and mouse

malibx tells you about input by calling **hooks**: functions you register on
a window. They run during the [event step](the-loop.md#frames) of each frame.

## The hook functions

| Function | When `f` is called | `f` gets |
|---|---|---|
| `key_hook(win, f, param=None)` | A key is **released** | `(keysym, param)` |
| `mouse_hook(win, f, param=None)` | A mouse button is **pressed** | `(button, x, y, param)` |
| `expose_hook(win, f, param=None)` | The window is first shown | `(param)` |
| `hook(win, x_event, x_mask, f, param=None)` | Any [supported event](#any-event-with-hook) | depends on the event |

!!! warning "`key_hook` fires on key **release**"
    This matches MiniLibX, but it's a surprise for many people. For games,
    where things should react when a key goes **down**, use
    `hook(win, 2, 1 << 0, f, param)` (KeyPress) instead. See
    [Smooth movement](#smooth-movement).

## Keys

Keys are reported as **X11 keysyms**, the same numbers MiniLibX gives you on
Linux. This keeps code portable: malibx reports these same values on macOS
and Windows too.

- **Letters** are their *lowercase* ASCII code, even with ++shift++ held:
  `ord("a")` is `0x61`. Use `ord("w")` rather than memorizing numbers.
- **Digits** are their ASCII code: `ord("0")` to `ord("9")`.
- **Special keys** have codes in the `0xFFxx` range:

| Key | Keysym | | Key | Keysym |
|---|---|---|---|---|
| ++esc++ | `0xFF1B` | | ++arrow-left++ | `0xFF51` |
| ++enter++ | `0xFF0D` | | ++arrow-up++ | `0xFF52` |
| ++space++ | `0x0020` | | ++arrow-right++ | `0xFF53` |
| ++tab++ | `0xFF09` | | ++arrow-down++ | `0xFF54` |
| ++backspace++ | `0xFF08` | | ++shift++ (left / right) | `0xFFE1` / `0xFFE2` |
| ++del++ | `0xFFFF` | | ++ctrl++ (left / right) | `0xFFE3` / `0xFFE4` |
| ++f1++ … ++f12++ | `0xFFBE` … `0xFFC9` | | ++alt++ (left / right) | `0xFFE9` / `0xFFEA` |

A handy set of constants to copy into your program:

```python title="keys.py"
ESC, ENTER, SPACE, TAB = 0xFF1B, 0xFF0D, 0x20, 0xFF09
LEFT, UP, RIGHT, DOWN = 0xFF51, 0xFF52, 0xFF53, 0xFF54
W, A, S, D = map(ord, "wasd")
```

!!! tip "Find any key's code"
    Not sure what a key sends? Print it:

    ```python
    malibx.hook(win, 2, 1 << 0, lambda k, _: print(hex(k)), None)
    ```

    The `keys` example in the repository (`make run EXAMPLE=keys`) prints
    every key and mouse event.

### Quit on Esc

```python
def on_key(keysym, param):
    if keysym == ESC:
        malibx.loop_end(mlx)

malibx.key_hook(win, on_key)
```

## Mouse

```python
def on_click(button, x, y, param):
    print(f"button {button} at ({x}, {y})")

malibx.mouse_hook(win, on_click)
```

`x` and `y` are in window pixels, with `(0, 0)` at the top-left corner, the
same coordinates you draw with. Buttons are numbered like X11:

| `button` | Meaning |
|---|---|
| `1` | Left |
| `2` | Middle (wheel click) |
| `3` | Right |
| `4` | Scroll up |
| `5` | Scroll down |

Each step of the scroll wheel is reported as a press **and** release of
button 4 or 5.

### Click to paint

```python title="paint.py" linenums="1"
import malibx

mlx = malibx.init()
win = malibx.new_window(mlx, 640, 480, "paint (left: dot, right: clear)")

def on_click(button, x, y, _):
    if button == 1:
        for dy in range(-3, 4):
            for dx in range(-3, 4):
                malibx.pixel_put(mlx, win, x + dx, y + dy, 0xB040FF)
    elif button == 3:
        malibx.clear_window(mlx, win)

malibx.mouse_hook(win, on_click)
malibx.loop(mlx)
```

There's no loop hook and no redrawing here. The window keeps its pixels, so
each dot stays until you clear it.

## Any event with `hook`

```python
malibx.hook(win, x_event, x_mask, f, param=None)
```

`hook` is the general version of the functions above. `x_event` is an X11
event number. These are the ones malibx reports:

| `x_event` | Event | `f` gets | Fires when |
|---|---|---|---|
| `2` | KeyPress | `(keysym, param)` | a key goes down, and again on auto-repeat while it's held |
| `3` | KeyRelease | `(keysym, param)` | a key goes up |
| `4` | ButtonPress | `(button, x, y, param)` | a mouse button goes down |
| `5` | ButtonRelease | `(button, x, y, param)` | a mouse button goes up |
| `6` | MotionNotify | `(x, y, param)` | the mouse moves inside the window |
| `12` | Expose | `(param)` | the window is first shown |
| `17` | DestroyNotify | `(param)` | the user closes the window |

`x_mask` is accepted so that MiniLibX code works unchanged, but malibx
**ignores it**. Pass the usual X11 mask (`1 << 0` for KeyPress, `1 << 1` for
KeyRelease, …) or `0`. It makes no difference.

!!! info "One hook per event per window"
    Each window holds one function per event number. Registering again for
    the same event **replaces** the old function. The shortcut functions use
    the same slots: `key_hook` is event 3, `mouse_hook` is event 4 and
    `expose_hook` is event 12. So `key_hook(win, f)` replaces a
    `hook(win, 3, …)`, and the other way round.

Event numbers from 0 to 35 are accepted, so MiniLibX code that registers other
events still runs, but those hooks never fire. Numbers outside that range
raise `ValueError`.

### The close button

When the user clicks the window's close button, malibx **closes the window
right away**, then calls your event 17 hook (if any). To close a window from
code, use [`destroy_window`](../reference/destroy_window.md); that doesn't
call the hook. Use the hook for
cleanup or for a goodbye message. You can't cancel the close, and the window
is already gone, so don't draw to it from this hook.

```python
def on_close(param):
    print("saving…")
    malibx.loop_end(mlx)       # also stop if other windows are still open

malibx.hook(win, 17, 0, on_close)
```

If this was the last open window, `loop` returns on its own after the hook.

## Smooth movement

To move something smoothly while a key is held, **track which keys are
down** and move in the loop hook. Moving inside the KeyPress hook makes
movement stutter: the hook only fires once, then pauses until the operating
system's key repeat starts.

```python title="move.py" linenums="1"
import malibx

W, H, SIZE, SPEED = 640, 480, 32, 4
LEFT, UP, RIGHT, DOWN, ESC = 0xFF51, 0xFF52, 0xFF53, 0xFF54, 0xFF1B
DIRS = {LEFT: (-1, 0), RIGHT: (1, 0), UP: (0, -1), DOWN: (0, 1),
        ord("a"): (-1, 0), ord("d"): (1, 0), ord("w"): (0, -1), ord("s"): (0, 1)}

mlx = malibx.init()
win = malibx.new_window(mlx, W, H, "move me")

player = malibx.new_image(mlx, SIZE, SIZE)
buf, _, _, _ = malibx.get_data_addr(player)
buf[:] = (0xFFD700).to_bytes(4, "little") * (SIZE * SIZE)

state = {"x": W // 2, "y": H // 2, "held": set()}


def key_down(key, st):
    if key == ESC:
        malibx.loop_end(mlx)
    st["held"].add(key)                     # (1)!


def key_up(key, st):
    st["held"].discard(key)


def frame(st):
    for key in st["held"]:                  # (2)!
        dx, dy = DIRS.get(key, (0, 0))
        st["x"] = min(max(st["x"] + dx * SPEED, 0), W - SIZE)
        st["y"] = min(max(st["y"] + dy * SPEED, 0), H - SIZE)
    malibx.clear_window(mlx, win)
    malibx.put_image_to_window(mlx, win, player, st["x"], st["y"])


malibx.hook(win, 2, 1 << 0, key_down, state)    # KeyPress
malibx.hook(win, 3, 1 << 1, key_up, state)      # KeyRelease
malibx.loop_hook(mlx, frame, state)
malibx.loop(mlx)
```

1. A `set` ignores auto-repeat: adding a key that's already held changes
   nothing. For the opposite (one action per press, however long it's
   held) call [`do_key_autorepeatoff`](../reference/do_key_autorepeatoff.md).
2. Every held key moves the player once per frame. Holding two arrows moves
   diagonally.

## Following the mouse

MotionNotify (event 6) reports the pointer position whenever it moves inside
the window:

```python
def on_move(x, y, st):
    st["mouse"] = (x, y)

malibx.hook(win, 6, 1 << 6, on_move, state)
```

Store the position in your state and use it in the loop hook, the same way
as held keys. Or skip the hook and ask for it whenever you need it:

```python
def frame(st):
    mx, my = malibx.mouse_get_pos(mlx, win)
```

You can also move the pointer with
[`mouse_move`](../reference/mouse_move.md), and hide it with
[`mouse_hide`](../reference/mouse_hide.md), for example to draw your own
cursor or for first-person mouse-look.

## Summary

- `key_hook` is **key release**; use `hook(win, 2, …)` for key press.
- Keys are X11 keysyms: lowercase ASCII for letters, `0xFFxx` for special keys.
- Mouse buttons 1/2/3 are left/middle/right, and 4/5 are the scroll wheel.
- One hook per event per window. Registering again replaces it.
- The close button closes the window first, then calls event 17.
- For smooth movement, track held keys in a `set` and move in the loop hook.

---

**You've finished Getting started.** You know how to open windows, run the
loop, draw with pixels, text and images, and respond to input. The
[examples](https://github.com/0xmthan/malibx/tree/main/examples) in the
repository put all of it together. `sprites.py`, an animated XPM skull with
glowing eyes, is a good one to read next.
