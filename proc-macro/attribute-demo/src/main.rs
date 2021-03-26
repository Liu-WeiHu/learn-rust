use std::{thread, time::Duration};

use attribute_demo_lib::run_time;

fn main() {
    deco(3);
}

#[run_time]
fn deco(t: u64) {
    let secs = Duration::from_secs(t);
    thread::sleep(secs);
}