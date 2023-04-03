use channel::{test_channel, test_channel_multiple, test_multiple_producer_signal_consumer};
use child_thread::{basic, move_thread};
use mutual_exclusion::test_mutual_exclusion;

mod channel;
mod child_thread;
mod mutual_exclusion;

fn main() {
    basic();
    move_thread();
    test_channel();
    test_channel_multiple();
    test_multiple_producer_signal_consumer();
    test_mutual_exclusion();
}
