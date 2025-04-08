mod runner;
mod persistence;

pub(crate) mod system_info;

pub use runner::*;
pub use persistence::*;

const BENCHMARKS_TARGET_DIR: &str = "target/benchmarks";
const USER_BENCHMARK_SERVER_URL: &str = if cfg!(debug_assertions) {
    // development
    "http://localhost:8000/"
} else {
    // production
    "https://user-benchmark-server-gvtbw64teq-nn.a.run.app/"
};

#[cfg(test)]
const USER_BENCHMARK_WEBSITE_URL: &str = "http://localhost:4321/";
#[cfg(not(test))]
const USER_BENCHMARK_WEBSITE_URL: &str = "https://burn.dev/";
