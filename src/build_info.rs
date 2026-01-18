use serde::Serialize;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const COMMIT: &str = env!("BUILD_COMMIT");
pub const BUILD_DATE: &str = env!("BUILD_DATE");
pub const VERSION_STRING: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (commit: ",
    env!("BUILD_COMMIT"),
    ", built: ",
    env!("BUILD_DATE"),
    ")"
);

#[derive(Debug, Serialize)]
pub struct BuildInfo {
    pub version: &'static str,
    pub commit: &'static str,
    pub build_date: &'static str,
}

impl BuildInfo {
    pub fn new() -> Self {
        Self {
            version: VERSION,
            commit: COMMIT,
            build_date: BUILD_DATE,
        }
    }
}

impl Default for BuildInfo {
    fn default() -> Self {
        Self::new()
    }
}
