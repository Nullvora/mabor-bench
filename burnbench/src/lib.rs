pub mod __private;
mod persistence;
mod runner;

pub(crate) mod system_info;

pub use persistence::*;
pub use runner::*;
pub use system_info::*;

const BENCHMARKS_TARGET_DIR: &str = "target/benchmarks";
const USER_BENCHMARK_SERVER_URL: &str = if cfg!(debug_assertions) {
    // development
    "http://localhost:8000/"
} else {
    // production
    "https://user-benchmark-server-812794505978.northamerica-northeast1.run.app/v1/"
};

#[cfg(test)]
const USER_BENCHMARK_WEBSITE_URL: &str = "http://localhost:4321/";
#[cfg(not(test))]
const USER_BENCHMARK_WEBSITE_URL: &str = "https://burn.dev/";
