//! Static build info for Baby Audio Tiny.

pub const BUILD_VERSION: &str  = "v4.3.17";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "8b67a736cfa9";
pub const BUILD_TIMESTAMP: u64 = 1777307624;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Baby Audio Tiny", BUILD_VERSION, BUILD_CHANNEL)
}
