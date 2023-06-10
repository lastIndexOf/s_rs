use s_log::{hello, hello_tracing, init_env_log, init_tracing};

fn main() {
    match std::env::var("TRACING") {
        Ok(val) if val == "true" => {
            init_tracing();
            hello_tracing();
        }
        _ => {
            init_env_log();
            hello();
        }
    }
}
