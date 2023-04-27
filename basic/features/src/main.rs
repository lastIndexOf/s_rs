use advanced_trait::run_advanced_trait;
use advanced_type::run_advanced_type;
use marco::run_marco;
use unsafe_rust::run_unsafe_rust;

mod advanced_trait;
mod advanced_type;
mod marco;
mod unsafe_rust;

fn main() {
    run_unsafe_rust();
    run_advanced_trait();
    run_advanced_type();
    run_marco();
}
