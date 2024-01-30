use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
// #[serde(rename_all = "lowercase")]
pub enum Environment {
    LOCAL,
    TEST,
    STAGE,
    PROD,
}

impl Environment {
    fn default() -> Self { Environment::LOCAL }
}

#[derive(Deserialize, Debug, PartialEq)]
// #[serde(rename_all = "lowercase")]
pub enum LogLevel {
    CRITICAL,
    ERROR,
    WARNING,
    INFO,
    DEBUG,
}

impl LogLevel {
    fn default() -> Self { LogLevel::DEBUG }
}

fn game_service() -> String {
    "game-service".to_string()
}

fn max_connections() -> usize {
    4
}

fn connections_overflow() -> usize {
    6
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "game_service")]
    pub app_name: String,
    #[serde(default = "Environment::default")]
    pub environment: Environment,
    #[serde(default = "LogLevel::default")]
    pub log_level: LogLevel,
    pub sentry_url: String,
    pub yugabyte_dsn: String,
    #[serde(default = "max_connections")]
    pub yugabyte_max_connections: usize,
    #[serde(default = "connections_overflow")]
    pub yugabyte_connections_overflow: usize,

    pub idp_url: String,

    pub scylladb_dsn: String,
    pub redis_dsn: String,
}
