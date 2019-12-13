# Day 13

## Part 2

The problem instructs us on how to hack the game to play for free. In that same vein, we hack the game to avoid having to move the joystick at all.

This only requires changing a single instruction (the one that calculates whether the ball is past the paddle in the main program loop). If the ball is never past the paddle, the game continues on its own until the user wins. We just have to provide _some_ valid joystick input at each **INPUT** instruction.

### Line Diff of the Hack

```bash
$ diff day13-lines.txt day13-hacked-lines.txt
366,367c366,367
< 1007
< 389
---
> 1107
> 0
```

This corresponds to changing memory addresses 365 and 366. This turns a `LT m[389], 23 -> m[381]` instruction into a `LT 0,23,381 -> m[381]` instruction, which always stores 1, allowing the game to continue.
