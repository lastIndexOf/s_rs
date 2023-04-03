use state::test_state;
use state_oop::test_state_oop;
use trait_object::test_trait_object;

mod state;
mod state_oop;
mod trait_object;

fn main() {
    test_trait_object();
    test_state();
    test_state_oop();
}
