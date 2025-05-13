use clap::Parser;
use reqwest::{StatusCode, Url};
use statrs::statistics::{Data, Distribution, OrderStatistics};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: Url,

    /// Calculate latency 90th percentile, median and mean
    #[arg(short, long, default_value_t = false)]
    latency: bool,

    /// Check uptime
    #[arg(short = 'U', long, default_value_t = false)]
    uptime: bool,
}

fn main() {
    let args = Args::parse();

    let attempts: usize = 100;
    let responses = get_responses(args.url, attempts);
    let (latencies, uptimes): (Vec<f64>, Vec<StatusCode>) = responses.iter().cloned().unzip();
    let mut latency: Data<_> = Data::new(latencies);
    if args.latency {
        println!(
            "latency:\np90: {:?},\nmedian: {:?},\nAverage: {:?}",
            latency.percentile(90),
            latency.median(),
            latency.mean()
        );
    }
    if args.uptime {
        let success_count = uptimes.iter().filter(|code| code.is_success()).count() as f64;

        println!("Success rate: {}/{}", success_count, uptimes.len());
    }
}

fn get_responses(url: Url, amount: usize) -> Vec<(f64, StatusCode)> {
    let mut responses: Vec<(f64, StatusCode)> = Vec::with_capacity(amount);
    for _ in 0..amount {
        let start = Instant::now();
        match reqwest::blocking::get(url.clone()) {
            Ok(resp) => {
                responses.push((start.elapsed().as_millis() as f64, resp.status()));
            }
            Err(e) => println!("{:?}", e),
        }
    }
    responses
}
