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

    let responses = get_responses(args.url, 10);
    let mut responses: Data<_> = Data::new(
        responses
            .iter()
            .map(|(latency, _)| *latency)
            .collect::<Vec<_>>(),
    );
    if args.latency {
        println!(
            "latency:\np90: {:?},\nmedian: {:?},\nAverage: {:?}",
            responses.percentile(90),
            responses.median(),
            responses.mean()
        );
    }
    println!("response times: {:#?}", responses);
    //TODO: count response codes
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
