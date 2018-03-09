## countdown-numbers

CLI program that finds all solutions to a numbers round from the popular British tv show Countdown.

### Rules
The rules of the Countdown Numbers Game are as follow:

The contestant chooses six numbers from two groups of, 20 small numbers and 4 large numbers. The numbers consist of two each of numbers 1 through 10. The 4 large numbers are 25, 50, 75 and 100. The contestant decides how many large numbers are to be used, from none to all four, the rest will be small numbers.

A random three-digit target is generated. The contestants have 30 seconds
to work out a sequence of calculations with the numbers whose final result
is as close to the target number as possible. They may use only the four
basic operations of addition, subtraction, multiplication and division,
and do not have to use all six numbers. Fractions are not allowed, and only
positive integers may be obtained as a result at any stage of the calculation.

### Usage
```
$ countdown-numbers 952 25 50 75 100 8 9
Starting numbers: [25, 50, 75, 100, 8, 9], target: 952
1259782 Valid expressions, found 10 Solutions in 0.665940267 seconds
((25 + 9) * (((100 + 75) * 8) / 50)) = 952
((((100 + 75) * 8) * (25 + 9)) / 50) = 952
((((100 + 75) * (25 + 9)) / 50) * 8) = 952
((((100 + 75) * (25 + 9)) * 8) / 50) = 952
((((25 + 9) * 8) * (100 + 75)) / 50) = 952
(((100 + 25) - ((50 * 9) / 75)) * 8) = 952
((((75 * (50 - 9)) - 100) / 25) * 8) = 952
((((75 * (50 - 9)) - 100) * 8) / 25) = 952
(((100 - ((50 * 9) / 75)) + 25) * 8) = 952
```

```
$ countdown-numbers --help
countdown-numbers 0.1.0
Henning Ottesen <henning@live.no>
Countdown Numbers Game Solver

USAGE:
    countdown-numbers.exe [FLAGS] [OPTIONS] <TARGET> <NUMBER>...

FLAGS:
    -h, --help       Prints help information
        --rules      Prints the rules of the Countdown Numbers Game
    -V, --version    Prints version information

OPTIONS:
    -r <NUM_BIG_NUMS>        Randomly choose the numbers and the target,
                             overrides provided numbers and target.
                             Takes number of big numbers as value, from 0 to 4.

ARGS:
    <TARGET>       Target number
    <NUMBER>...    Starting numbers, at least two numbers must be provided
```


### Build and Run
1. Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org/) installed
2. Clone the project `$ git clone https://github.com/henninglive/countdown-numbers/ && cd countdown-numbers`
3. Build the project `$ cargo build --release` (NOTE: There is a large performance differnce when compiling without optimizations, so I recommend alwasy using `--release` to enable to them)
4. Once complete, the binary will be located at `target/release/countdown-numbers`
5. Use `$ cargo run --release --  952 25 50 75 100 8 9` to build and then run, in one step
