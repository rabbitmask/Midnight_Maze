use std::thread;
use std::time::Duration;
use rand::Rng;


pub(crate) fn random_sleep(min_seconds: u64, max_seconds: u64) {
    let sleep_time = rand::thread_rng().gen_range(min_seconds..=max_seconds);
    thread::sleep(Duration::from_secs(sleep_time));
}

