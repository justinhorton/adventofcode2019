# Day 21

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
