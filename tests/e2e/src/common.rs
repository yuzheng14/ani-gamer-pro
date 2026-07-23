use std::sync::OnceLock;

use tracing_subscriber::EnvFilter;

static TRACING: OnceLock<()> = OnceLock::new();

pub fn init_test_tracing() {
    TRACING.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_err| EnvFilter::new("info")),
            )
            .with_target(true)
            .with_line_number(true)
            .try_init()
            .expect("tracing subscriber init error");
    });
}
