use config::{Config, File};
use serde_derive::{Deserialize, Serialize};
use std::env::var as env_var;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct AgentData {
    pub latest_version: String,
    pub minimal_version: String,
}

#[derive(Debug, Deserialize)]
pub struct FileListerConfig {
    pub dir: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct FileWatcherConfig {
    pub dir: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpConfig {
    pub host: String,
    pub auth_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HubConfig {
    pub host: String,
    pub port: String,
    pub protocol: String,
    pub auth_path: String,
    pub disconnect_path: String,
    pub connection_attempt_limit: u32,
}

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    pub term_level: String,
    pub file_level: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct MyFilesConfiguration {
    pub db_path: String,
    pub drop_db_on_start: bool,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub agent_data: AgentData,
    pub file_lister_config: FileListerConfig,
    pub file_watcher_config: FileWatcherConfig,
    pub server_config: ServerConfig,
    pub logger_config: LoggerConfig,
    pub my_files_config: MyFilesConfiguration,
    pub hub_config: HubConfig,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            agent_data: AgentData {
                latest_version: String::new(),
                minimal_version: String::new(),
            },
            file_lister_config: FileListerConfig {
                dir: vec![[r"tests", "assets", "test_folder"].iter().collect()],
            },
            file_watcher_config: FileWatcherConfig {
                dir: vec![[r"tests", "assets", "test_folder"].iter().collect()],
            },
            server_config: ServerConfig {
                address: String::from("0.0.0.0:8111"),
                log_level: String::from("info"),
            },
            hub_config: HubConfig {
                host: String::from("localhost"),
                port: String::from("7001"),
                protocol: String::from("http"),
                auth_path: String::from("/gateway/auth/AOTH"),
                disconnect_path: String::from("/gateway/auth/AOTH/{agent_id}/disconnect"),
                connection_attempt_limit: 30,
            },
            logger_config: LoggerConfig {
                term_level: String::from("debug"),
                file_level: String::from("warn"),
            },
            my_files_config: MyFilesConfiguration {
                db_path: String::from("my_files.db"),
                drop_db_on_start: false,
            },
        }
    }
}

impl Configuration {
    pub fn init() -> Self {
        let env = env_var("TIDY_ENV").unwrap_or_else(|_| "development".into());

        info!("Loading configuration for environment: {}", env);

        let builder = Config::builder()
            .add_source(File::from(Path::new("config/default.json")))
            .add_source(File::with_name(&format!("config/{env}.json")).required(false))
            .build()
            .unwrap();
        let config: Configuration = builder.try_deserialize().unwrap_or_default();
        config
    }
}
