///------------------------------------------------------------------------ 
/// Lotto Parallel (Longshot) - a Lotto 6/49 simulator showing the futility of lotteries
/// This is the same basic program as Lotto Longshot, but with multi-threaded simulatiion
/// Created By: David Young, January, 2023 - using Rust 1.65.0 on Ubuntu 22.04
/// This requires crates:  
///    scan-rules="^0.1"
///    rand = "0.8.5"
///    chrono = "0.4"
///    thousands = "0.2.0"
///    crossbeam = "0.8"
///    num_cpus = "1.0"
///------------------------------------------------------------------------

// Modules used
#[macro_use] extern crate scan_rules;
extern crate num_cpus;
use rand::Rng;
use std::time::Instant;
use chrono::prelude::*;
use thousands::Separable;

// global constants
const MIN_BALL: usize = 1;  // lowest number you can pick
const MAX_BALL: usize = 49;  // highest number you can pick
const MAX_PICKS: usize = 6;  // maximum number of numbers you can pick
const RESULT_SCENARIOS: usize = 7;  // can score from 0 - 6 right = 7 scenarios
const PAYOFF_RATES: [u64; 7] = [0, 0, 3, 10, 80, 2_500, 
                               9_000_000];  // payoff per balls
const COST_PER_TICKET: u64 = 3;  // like it says

struct Simulation {  // Structure to hold all variables for a simulation run
    quick_picks_choice: char,  // y or other
    start_instant: Instant,  // for calculating runtime
    finish_instant: Instant, // as above
    runtime_seconds: f64,  // as above
    my_picks_idx: [usize; MAX_PICKS],  // which numbers user picked, 0-index
    num_games_to_run: u64,  // how many lottery games to simulate
    count_results: [u64; RESULT_SCENARIOS]  // tally each result, e.g. guessed 0..6
}

fn main() {
    // Initialize global variables for a simulation run
    // Note that arrays are zero-indexed, so ball label would be index + 1
    let now = Instant::now();
    let mut sim_run = Simulation {
        quick_picks_choice: 'y',
        start_instant: now,
        finish_instant: now,
        runtime_seconds: 0.0,
        my_picks_idx: [0; MAX_PICKS],
        num_games_to_run: 0,
        count_results: [0; RESULT_SCENARIOS]
    };
        
    // run the program
    get_user_input(&mut sim_run);  // get simulation paramters from user via standard input
    run_simulation(&mut sim_run);  // run all the games
    report_results(&mut sim_run);  // report on the total wins by type and earnings
}  // main

fn get_user_input(this_run: &mut Simulation) {
    // Get user's paramters for the simulation from standard input
    println!("\nWelcome to Lotto Parallel (Longshot) - a lesson in futility!");
    println!("------------------------------------------------------------");
    println!("\nThis simulates a Lotto 6/49 lottery to see how lucky you are (not).");
    println!("\nThis version is optimized to use all CPU's on your computer - super fast.");
    println!("\nType 'y' + Enter for a random quick pick, else any other letter + Enter");
    let user_choice: char;
    let mut picks: [usize; MAX_PICKS];
    let mut valid: bool;
    loop {
        let result = try_readln! {
            (let c: char) => (c)
        };
        match result {
            Ok(c) => {
                user_choice = c;
                break;
            },
            Err(_) => {
                println!("Type a single character and press enter ");
                continue;
            }
        }
    } // loop
    if user_choice == 'y' {
        picks = do_quick_pick();
    }
    else {
        loop { 
            (picks, valid) = get_user_picks();
            if valid {
                break;
            }
        }
    }
    picks.sort();  // sort the chosen balls
    this_run.quick_picks_choice = user_choice;
    this_run.my_picks_idx = picks;
    let cpus = num_cpus::get();
    println!("How many games do you want to simulate?");
    println!("  Note: Games will be divided evenly among {} CPU's.", cpus);
    loop {
        let result = try_readln! {
            (let n: u64) => (n)
        };
        match result {
            Ok(n) => {
                this_run.num_games_to_run = n;
                break;
            },
            Err(_) => {
                println!("Type a positive integer number and press enter, butthead");
                continue;
            }
        }
    }  // loop
    let remainder : u64 = this_run.num_games_to_run % cpus as u64;
    if remainder != 0 {
        this_run.num_games_to_run -= remainder;
    }
}  // get_user_input

fn do_quick_pick() -> [usize; MAX_PICKS] {
    // Get and return random ball picks rather than letting user choose
    let mut picks: [usize; MAX_PICKS] = [0; MAX_PICKS];
    // use the game's ball-drawing to get a random set
    let balls_array: [bool; MAX_BALL] = draw_balls();  
    let mut num_picked: usize = 0;
    let mut n: usize = 0;
    while num_picked < MAX_PICKS {
        if balls_array [n] {
            picks [num_picked] = n;
            num_picked += 1;
        }
        n += 1;
    }
    return picks;
}  // do_quick_pick

fn draw_balls() -> [bool; MAX_BALL] {
    // Simulate a lottery game draw; return array of booleans
    //   representing which balls were pulled (true's) from all possible values
    let mut balls_array: [bool; MAX_BALL] = [false; MAX_BALL];
    let mut nballs_picked: usize = 0;
    let mut test_ball: usize;
    while nballs_picked < MAX_PICKS {  //i.e, from index 0 to MAX_PCKS - 1
        test_ball = rand::thread_rng().gen_range(0..MAX_BALL);  // to max_ball - 1
        if !balls_array [test_ball] {  // if this ball hasn't already been generated
            balls_array [test_ball] = true;
            nballs_picked += 1;
        }
    }
    return balls_array;
}  // draw_balls

fn get_user_picks() -> ([usize; MAX_PICKS], bool) {
    // Get user's choice of balls, check validity, and return
    //   an array of the balls picked (if valid) plus a 'valid' boolean.
    let mut picks: [usize; MAX_PICKS] = [0; MAX_PICKS];
    println!("Enter {} numbers from {} to {}", MAX_PICKS, MIN_BALL, MAX_BALL);
    loop {
        let result = try_readln! {  // ugly hardcoding but readln! doesn't do arrays
            (let n0: usize, let n1: usize, let n2: usize, let n3: usize,
            let n4: usize, let n5: usize) => (n0, n1, n2, n3, n4, n5)
        };
        match result {
            Ok((n0, n1, n2, n3, n4, n5)) => {
                picks[0] = n0;  // will later change pick to index of picks
                picks[1] = n1;
                picks[2] = n2;
                picks[3] = n3;
                picks[4] = n4;
                picks[5] = n5;
                break;
            },
            Err(_) => {
                println!("Enter {} numbers from {} to {}", MAX_PICKS, MIN_BALL, MAX_BALL);
                continue;
            }
        }  // match
    }  // loop
    // check for errors
    for n in 0..MAX_PICKS {  // loop from 0 to (MAX_PICKS - 1)
        if (picks[n] < MIN_BALL) | (picks[n] > MAX_BALL) {
            println!("You chose {} but numbers must be from {} to {}", 
                picks[n], MIN_BALL, MAX_BALL);
            return (picks, false);
        };  // if
        for m in 0..MAX_PICKS {  // loop from 0 to MAX_PICKS - 1
            if (n != m) & (picks[n] == picks[m]) {
                println!("Duplicate numbers: {}", picks[n]);
                return (picks, false);
            }  // if
        }  // for
    }  // for
    // valid, so change the chosen numbers to index values
    for n in 0..MAX_PICKS {  // change from ball label to index (ie, - 1)
        picks[n] -= 1;
    }
    return (picks, true);
}  // get_user_picks

fn run_simulation(this_run: &mut Simulation) {
    // Simulate all the lottery games, and accumulate statistics
    let nowx = Local::now();
    let show_date_time = nowx.format("%Y-%m-%d %H:%M:%S");  // Printable date / time
    let mut big_number_str : String = this_run.num_games_to_run.separate_with_commas();
    println!("Running simulation for {} games at {}...", 
        big_number_str, show_date_time);
    // Create a displayable set of picked balls
    let mut picks_display = this_run.my_picks_idx;
    for n in 0..MAX_PICKS {
        picks_display[n] += 1;
    }
    println!("Numbers chosen : {:?}", picks_display);
    this_run.start_instant = Instant::now();
    this_run.count_results = [0; RESULT_SCENARIOS];
    let my_picks = this_run.my_picks_idx;
    // Set up for multi-threaded
    let cpus = num_cpus::get();
    let n_threads : usize = cpus;
    let games_per_thread : u64 = this_run.num_games_to_run / n_threads as u64;
    big_number_str = games_per_thread.separate_with_commas();
    println!("Number of threads : {}  at  {} games per thread", 
        n_threads, big_number_str);
    let mut all_scores : Vec<u64> = vec![0; RESULT_SCENARIOS * n_threads];  // scores for all threads
    let thread_scores : Vec<&mut [u64]> = 
        all_scores.chunks_mut(RESULT_SCENARIOS).collect();
    // Split into multiple threads and run the simulations
    crossbeam::scope(|spawner| {
        for (i, strm) in thread_scores.into_iter().enumerate() {
            spawner.spawn(move |_| {
                let mut num_right : usize;
                let mut big_number_str : String;
                for g in 1..=games_per_thread {
                    num_right = run_a_game(my_picks);  // run / score a single game
                    strm[num_right] += 1;  // increment global score
                    if g % 1_000_000 == 0 {  // print every xxx games as progress indicator
                        big_number_str = g.separate_with_commas();
                        println!("Thread {} : Running Game {}...", i, big_number_str);
                    } // if
                } // for
            });  // spawner
        }  // for
    }).unwrap();
    // Coalesce all thread score vector slices into the overall simulation scores
    for n in 0..RESULT_SCENARIOS { // for each number-right index 0 - 6
        for t in 0..n_threads {  // for each thread
            this_run.count_results[n] += all_scores[n + (t * RESULT_SCENARIOS)];  // sum all
        }  // for
    }  // for
}  // run_simulation

fn run_a_game(picks: [usize; MAX_PICKS]) -> usize {
    // Run 1 game against the user's picks, and return number right
    let balls_array: [bool; MAX_BALL] = draw_balls();
    let mut num_right: usize = 0;
    for n in 0..MAX_PICKS {  // loop from 0 to (MAX_PICKS - 1)
        if balls_array[picks[n]] {
            num_right += 1;
        }  // if
    } // for
    return num_right;
}  // run_a_game

fn report_results(this_run: &mut Simulation) {
    // Print a summary of the overall simulation results
    let nowx = Local::now();
    let show_date_time = nowx.format("%Y-%m-%d %H:%M:%S");  // Printable date / time
    let mut big_number_str : String = this_run.num_games_to_run.separate_with_commas();
    println!("Finished simulation for {} games at {}...", 
        big_number_str, show_date_time);
    let now2 = Instant::now();
    let run_time = now2.duration_since(this_run.start_instant);
    let run_seconds: f64 = (run_time.as_micros() as f64) / 1000000.0f64;
    let runs_per_second : f64 = this_run.num_games_to_run as f64 / run_seconds;
    big_number_str = (runs_per_second as u64).separate_with_commas();
    println!("Run time = {} seconds", run_seconds);
    println!("Runs per second = {}\n", big_number_str);
    this_run.finish_instant = now2;
    this_run.runtime_seconds = run_seconds;
    // Print how many games resulted in which outcomes, and accumulate totals
    let mut total_payoff : u64 = 0;
    let mut this_payoff : u64;
    for n in 0..RESULT_SCENARIOS { // from 0 to scenarios - 1
        this_payoff = this_run.count_results[n] * PAYOFF_RATES[n];
        total_payoff += this_payoff;
        big_number_str = this_run.count_results[n].separate_with_commas();
        print!("You picked {} correct {} times", n, big_number_str);
        big_number_str = this_payoff.separate_with_commas();
        println!("  --> Payoff = ${}", big_number_str);
    } // for
    // Print the overall totals
    let total_cost : u64 = this_run.num_games_to_run * COST_PER_TICKET;
    let total_profit : i64 = (total_payoff as i64) - (total_cost as i64);
    let mut profit_pct : f64 = 0.0;
    big_number_str = total_cost.separate_with_commas();
    println!("\nTotal cost of tickets : ${}", big_number_str);
    big_number_str = total_payoff.separate_with_commas();
    println!("Total money won : ${}", big_number_str);
    big_number_str = total_profit.separate_with_commas();
    println!("Total profit / loss : ${}", big_number_str);
    if total_cost != 0 {  // can't divide by zero
        profit_pct = ((total_profit as f64) / (total_cost as f64)) * 100.0f64;    
    }
    println!("Percent profit / loss : {:.2} %", profit_pct);
    if profit_pct < 0.0 {
        println!("*** Loser!!! I hope you learned something from this! ***");
    }
    else {
        println!("*** Winner!!! Pure fluke though, don't make this a habit ***");
    }
    println!("\n*************** END SIMULATION ***************\n");
}  // report_results
