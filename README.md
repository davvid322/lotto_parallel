# lotto_parallel
This is a small program that simulates a lottery, similar to the Lotto 6/49 in Canada.
I created this for two reasons:
1 - To learn how to create multi-threaded software in the Rust programming language, and
2 - To show the statistical futility of playing lotteries.
###### This is a multi-threaded version of my lotto_longshot program.

### Usage Notes
- This is a text-only program that runs in a terminal.
- The user gets to choose 6 numbers between 1 and 49, or they can let the system
do a 'quick pick' to generate a random selection for them.
- The user can specify how many games to simulate, and the system will keep
track of how many times they got 0, 1, 2...6 correct.
- The system will detect the number of logical CPU's on the running computer, and
create one parallel thread for each CPU, dividing the number of runs evenly.
- At the end the program will produce some summary statistics.
- No per-game details are shown, as this version of the original lotto_longshot is intended for
huge numbers of game simulations.
- There is no limit to the number of games to simulate (unsigned 64-bit integer).
- This was created using Rust 1.65.0 on Ubuntu Linux 22.04.
