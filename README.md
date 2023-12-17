# AOC 2023

My attempts at the Advent of Code '23, using over-engineered Rust solutions.


# Notes

## Day 6
- already expected the brute-force approach to fail for whatever part 2 would be
- ideas for part 2:
    - bisection
    - directly solve (need to deal with the fact that the solution is an integer, though -> added a bunch of assertions to be sure there's no off-by-one error)

## Day 7
- learned about the `HashMap.entry()` API for reasonably concise insert-or-update
- code really blown up by just duplicating everything for part 2, but didn't bother to clean this up
- in general, things could probably by shortened a lot by depending on lexsorting of tuples even more

## Day 9

Part 1
- input: one time series per line
- task: produce one extrapolation step
    - produce finite differences
    - iterate until observing a constant derivative
    - extrapolate from there
- output: sum of all extrapolated values

Part 2
- same same
