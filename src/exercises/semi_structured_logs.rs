use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level.to_string().to_uppercase(), message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

#[test]
fn emits_info() {
    assert_eq!(info("Timezone changed"), "[INFO]: Timezone changed");
}

#[test]
#[ignore]
fn emits_warning() {
    assert_eq!(warn("Timezone not set"), "[WARNING]: Timezone not set");
}

#[test]
#[ignore]
fn emits_error() {
    assert_eq!(error("Disk full"), "[ERROR]: Disk full");
}

#[test]
#[ignore]
fn log_emits_info() {
    assert_eq!(
        log(LogLevel::Info, "Timezone changed"),
        "[INFO]: Timezone changed"
    );
}

#[test]
#[ignore]
fn log_emits_warning() {
    assert_eq!(
        log(LogLevel::Warning, "Timezone not set"),
        "[WARNING]: Timezone not set"
    );
}

#[test]
#[ignore]
fn log_emits_error() {
    assert_eq!(log(LogLevel::Error, "Disk full"), "[ERROR]: Disk full");
}

#[test]
#[cfg(feature = "add-a-variant")]
#[ignore]
fn add_a_variant() {
    // this test won't even compile until the enum is complete, which is why it is feature-gated.
    assert_eq!(
        log(LogLevel::Debug, "reached line 123"),
        "[DEBUG]: reached line 123",
    );
}
