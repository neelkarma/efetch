#[derive(Debug)]
pub enum Error {
    ConfigParseError,
    ConfigReadError,
    NoHomeEnvError,
}
