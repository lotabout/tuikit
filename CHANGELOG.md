# Change Logs

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
