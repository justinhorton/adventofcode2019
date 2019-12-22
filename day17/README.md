# Day 17

## Annotated Grid

```
    ###########
    #    10   #
    #         #
    #         #
    # 8       #
    #         #
    #         #
    #         #
    ##########O##
              # # 4
              # #
              # #   8
              ##O########
                #       #
                #       # 4
                #       #           12
                ########O##   ############^
                        # #   #
                        # #   #
                        # #   #
                        # #   #
                        # #   # 10
                        # #   #
                    8   # #   #
                ######### #   #
        8       #         #   #
    #########   #     ####O####
    #       #   #     #   #
    #       # ##O#####O####
    #       # # #     #
  8 #       # # #     #
    #       # # #     #
    #       # # #     #
    #       # # #     # 10
    ########O#O##     #
            # #       #
############# #       #
#      12     #       #
# 12          #########
#      8          8
# #########
# #
# #
# #
# #
# #
# #
# #
##O########
  #       #
  #       #
  #       #
  ########O##
      8   # #
          # # 4
          # #
          ##O##########
            #         #
            #         #
            #         #
            #         # 8
            #         #
            #         #
            #   10    #
            ###########
```

## Part 2

I didn't solve this programmatically. I used the following strategy on the above grid:

1. Never turn at intersections; always go to the next corner/dead end.
2. Write out the path from the start to the final dead end in the lower part of the grid.
3. Factor out common sequences into routines. We know from the grid that one routine must start with `L,12,L,10` (2 turns at the start) and one must end with `R,12,R,8` (2 turns at the end).

### Routines

| Routine      | Movements             |
| -----------  | -----------           |
| A            | `L,12,L,10,R,8,L,12`  |
| B            | `L,10,R,12,R,8`       |
| Main         | `A,B,A,B,C,C,B,A,B,C` |

---

```
(L,12,L,10,R,8,L,12)A,

(R,8,R,10,R,12)B,

(L,12,L,10,R,8,L,12)A,

(R,8,R,10,R,12)B,

(L,10,R,12,R,8)C

(L,10,R,12,R,8)C,

(R,8,R,10,R,12)B

(L,12,L,10,R,8,L,12)A,

(R,8,R,10,R,12)B,

(L,10,R,12,R,8)C
```

