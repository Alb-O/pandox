//! Shared logging configuration for Pandox crates.

use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the logging system with a global readable format.
///
/// Defaults to `warn` globally, but `debug` for `pandox_macros`, `pandox_pandoc`,
/// `pandox_log` and `pandox`.
/// Uses `RUST_LOG` env var if set.
pub fn init_tracing() {
	let _ = fmt()
		.with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
			// Default to warn globally, but debug for our internal crates
			EnvFilter::new(
				"warn,pandox_macros=debug,pandox_pandoc=debug,pandox=debug,pandox_log=debug",
			)
		}))
		.without_time()
		.pretty()
		.compact()
		.try_init();
}
