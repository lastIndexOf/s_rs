mod mod_a;

pub use mod_a::mod_b;

fn main() {
    println!("Hello, world! {}", mod_a::mod_b::MODULE_B_CONST);
}
