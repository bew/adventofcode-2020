# AdventOfCode 2020 - Learning Rust :heart:

This is my take on the [advent of code challenges](https://adventofcode.com/2020/) of 2020, using them as an excuse to write my first lines of Rust, long due after reading a few existing Rust codebases, and reading about it for so long!

**Challenges covered**:

(might be outdated, check in [src/](src/) for an up-to-date list)

- day01
- day02
- day03

**My code only**:

NOTE that no external crate (Rust's libraries) have been used so far. I'd like to make everything myself to better understand the language, before using code written by other people.

My only fear with this choice is _input parsing_. At the time of writing, I had only day 1, 2 and 3 so the input parsing was pretty basic. I hope the next challenges inputs won't be too hard to parse!

_I had a pretty bad time (with looong errors when compiling) while trying to wrap my mind around proper error handling. I was suggested to use the crate anyhow, but since I want to understand all the code, I had to re-implement a simpler version of anyhow._ (it is available in [src/common.rs](src/common.rs))

## Build it | Run it

**Using Nix**:

```sh
$ nix build
$ ./result all
```

**Using cargo**:

```sh
$ cargo build
$ ./target/debug/adventofcode-2020 all
```

## CLI

The resulting binary has the solutions for all the covered challenges. It offers a simple CLI to either run a single challenge with the default or a custom input, or all the challenges with their default input.

**Usage**:
```sh
$ prog <day_id-or-all> [<custom_input_path>]
```

**Example**:
```sh
$ ./result day03
--- day03
Part1: 218
Part2: 3847183340
```

NOTE: errors during the computation of a solution will be visible:
```sh
$ ./result day03 inputs/day03_example_grid.txt.fail
--- day03
Part1 Error: Failed to load input: Invalid char '~' in input grid at x:2 y:2
Part2 Error: Failed to load input: Invalid char '~' in input grid at x:2 y:2
```
