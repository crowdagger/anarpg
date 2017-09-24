#[macro_use]
extern crate log;
extern crate anarpg;
extern crate simplelog;

use anarpg::{Character, Stats, Cooldown};
use simplelog::{Config, TermLogger, CombinedLogger, LogLevelFilter};

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Debug, Config::default()).unwrap(),
        ]
    ).unwrap();
    let character = Character::new("Lev", "Gouine motarde");
    let mut stats = Stats::new();
    let zero = Stats::zero();
    info!("{:?}", stats);
    let cooldown = Cooldown::new()
        .with_fixed(1.0)
        .with_badassness(1.0);
    info!("cooldown (bare): {}", cooldown.adjusted_by(&zero));
    info!("cooldown (default): {}", cooldown.adjusted_by(&stats));
    stats.badassness = 20;
    info!("cooldown (badass): {}", cooldown.adjusted_by(&stats));
}
