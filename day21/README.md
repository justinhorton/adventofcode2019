# Day 21

## Part 2

Turns out we can get by with a simpler strategy (and therefore script) for part 1:

```
instead of:

D && (!A && B && C ||  A && B && !C || !B)

we can use:

D && (!A || !B || !C)
=> D && !(A && B && C)

OR  A J
AND B J
AND C J
NOT J J
AND D J
```

As it turns out, registers are initialized to 0, so to populate a register upfront with non-negated `A`, we can just use `OR A J` rather than `NOT A J; NOT J J` (as I used in my first part 1 solution below).

Running with this script for part 2, the robot falls into the hole 5 tiles away when jumping from this state:

```
.................
.................
..@..............
#####.#.##..#.###

...

.................
.................
......@..........
#####.#.##..#.###
```

So, add a check that `E` mut also be a floor:

```
OR  A J
AND B J
AND C J
NOT J J
AND D J

OR  E T
AND T J
```

Then the little robot dies _not_ jumping here:

```
.................
.................
...@.............
#####..#.########

...

.................
.................
.................
#####@.#.########
```

It should've jumped to land on the middle `#` and immediately jumped again. The part 1 logic would have made it to jump here, but the new check that `E` is a floor prevented that.

It's ok to jump from the above state if there's a floor 8 tiles away (`H`), even though there's no floor 5 tiles away (`E`).

```
OR  A J
AND B J
AND C J
NOT J J
AND D J

OR  E T
OR  H T
AND T J
```

And the above allows the robot to make it through.

## Part 1

For any jump, D must always be floor as the drone jumps 4 spaces.

```
gap of 1:
    !A && B && C
    A && !B && C
    A && B && !C

gap of 2:
    !A && !B && C
    A && !B && !C

gap of 3:
    !A && !B && !C
```

```
=> !A && B && C || A && !B && C || A && B && !C || !A && !B && C || A && !B && !C || !A && !B && !C

factor out !B

=> !A && B && C ||  A && B && !C || !B && (A && C || !A && C || A && !C || !A && !B)

last parenthesized term is always true

=> !A && B && C ||  A && B && !C || !B

combine with the assertion re: D

=> D && (!A && B && C ||  A && B && !C || !B)

gives us the routine:

NOT A J
AND B J
AND C J

NOT C T
AND A T
AND B T
OR  T J

NOT B T
OR  T J

NOT D T
NOT T T
AND T J
```
