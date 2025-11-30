mod PrimeSearch;

use std::time::Instant;
use clap::Parser;// for cmdline arg parsing
use log::info;


fn main() {
    //env_logger::init();
    env_logger::Builder::new() // Build logger using the chosen level
        .filter_level(log::LevelFilter::Debug)
        .init();

    let args = CommandLineArgs::parse();
    log::info!("Command line args : {:#?}", args);

    struct ThreadParams {
        starting_number:u32,
        count:u32,
        name:String,
    };

    let mut thread_params = std::vec::Vec::new();
    let mut join_handles = Vec::new();

    let mut s_number = args.starting_number;
    for i in 1..=args.thread_count {
        let tp = ThreadParams{starting_number: s_number, count: args.count, name: i.to_string() };
        s_number += args.start_delta_between_thread;
        thread_params.push(tp);
    }

    log::info!("Starting search with {} threads", thread_params.len());
    let start_time = Instant::now();

    for tp in thread_params{

        let th = std::thread::spawn(move || {
            log::info!("Starting thread with start number and count: {} {}", tp.starting_number,tp.count);
            let results = PrimeSearch::find_primes(tp.starting_number,tp.count);
            log::info!("Thread {} ended", tp.name);
            results
        });

        join_handles.push(th);

    }

    for handle in join_handles{
        let results = handle.join().unwrap();
        log::info!("Results: {:?}", results);
    }

    let duration = start_time.elapsed();
    info!("Finished search in {} ms ", duration.as_millis());


}


#[derive(Parser, Debug)]
struct CommandLineArgs {


    /// Starting number
    #[arg(short, long, default_value = "100000000")]
    starting_number: u32,

    /// Used for setting other threads starting number
    #[arg(short = 'd', long, default_value = "1000")]
    start_delta_between_thread: u32,

    /// Number of primes per thread
    #[arg(short, long, default_value_t = 10)]
    count: u32,

    /// Thread count
    #[arg(short, long, default_value_t = 4)]
    thread_count: u8,

    /// Activate verbose mode
    #[arg(short = 'v', long)]
    verbose: bool,
}