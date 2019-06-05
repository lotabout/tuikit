# Change Logs

## v0.2.8: 2019-06-05

Update dependenncy `nix` to `0.14`.

## v0.2.7: 2019-06-04

Features:
- Implement `From` trait for variaous struct
    * `From<Color> for Attr`
    * `From<Effect> for Attr`
    * `From<char> for Cell`
- `win/split` now accept `Into<...>` struct. Previously when initializing
    splits, you need to write `split.basis(10.into())`,
    now it's just `split.basis(10)`.
- Implement builder pattern for `Attr`. We could now do
    `Attr::default().fg(...).bg(...)`.
- Add two user defined event(`User1` and `User2`). Use it for your own need.

Bug fixes:
- fix compilation error on FreeBSD.

## v0.2.6: 2019-03-28

Reduce CPU usage on idle.

## v0.2.5: 2019-03-28

Clear screen on resize

## v0.2.4: 2019-03-22

Fix: ESC key not working

## v0.2.3: 2019-03-23

- Support more alt keys
- impl `Draw` for `Box<T: Draw>`

## v0.2.2: 2019-03-19

API change: `Draw::content_size` -> `Draw::size_hint` and returns
`Option<usize>`. So that `None` could indicates "I don't know".

## v0.2.1: 2019-03-17

- fix: build failed with rust 2018 (1.31.0)

## v0.2.0: 2019-03-17

Feature:
- Support layout(e.g. `HSplit`, `VSplit`)
- `term.send_event` to inject event to `Term`'s event loop
- `use tuikit::prelude::*` to simplify import

## v0.1.5: 2019-03-02

Fix: Synchronize the pause and restart event.

## v0.1.4: 2019-02-25

Fix: output will replace raw ESC(`\x1b`) with `?` so that terminal won't mess up.

## v0.1.3: 2019-02-24

Fix: report cursor position (0, 0) on terminals that doesn't support CPR.

## v0.1.2: 2019-02-24

Features:
- support specifying `min-height` and `max-height`
- screen: able to iterate over all cells
- attr: add `extend` method for composing `Attr`.

Bug Fixes:
- #1 Increase timeout(to 300ms) on initialize to support slow terminals
- #3 erase contents on exit
- screen: fix panic on height/width of `0`
- fix some key parsing error
