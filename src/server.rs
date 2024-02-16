use crate::agent_data::AgentData;
use crate::my_files;
use crate::my_files::{ConfigurationPresent, ConnectionManagerPresent, Sealed};
use crate::http::routes::{MyFilesState, AgentDataState, get_files, get_status, hello_world};
use crate::http::protocol::{HttpResponse};
use axum::{routing::get, Json, Router};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::{error, info, Level};
// use std::env;
use futures::future::BoxFuture;

lazy_static! {
    static ref AGENT_LOGGING_LEVEL: HashMap<String, Level> = {
        let mut m = HashMap::new();
        m.insert("trace".to_owned(), Level::TRACE);
        m.insert("debug".to_owned(), Level::DEBUG);
        m.insert("info".to_owned(), Level::INFO);
        m.insert("warn".to_owned(), Level::WARN);
        m.insert("error".to_owned(), Level::ERROR);
        m
    };
}

#[derive(Default)]
pub struct Server {
    address: String,
    router: Router,
}

#[derive(Clone, Default)]
pub struct ServerBuilder {
    router: Router,
    my_files_builder:
    my_files::MyFilesBuilder<ConfigurationPresent, ConnectionManagerPresent, Sealed>,
}

trait ServerConfig {
    fn host(&self) -> &str;
    fn route(&self) -> &str;
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder::default()
    }

    pub fn my_files_builder(
        mut self,
        my_files_builder: my_files::MyFilesBuilder<
            ConfigurationPresent,
            ConnectionManagerPresent,
            Sealed,
        >,
    ) -> Self {
        self.my_files_builder = my_files_builder;
        self
    }

    pub fn build(
        self,
        latest_version: String,
        minimal_version: String,
        dirs_watch: Vec<PathBuf>,
        address: String,
        logging_level: String,
    ) -> Server {
        let my_files_instance = self.my_files_builder.build().unwrap();
        info!("MyFiles instance successfully created for HTTP Server");
        let my_files_state = MyFilesState {
            my_files: Arc::new(Mutex::new(my_files_instance)),
        };
        let agent_data_state = AgentDataState {
            agent_data: Arc::new(Mutex::new(AgentData::build(
                latest_version,
                minimal_version,
                dirs_watch,
            ))),
        };

        let server_logging_level: Level = AGENT_LOGGING_LEVEL.get(&logging_level).map_or_else(
            || {
                error!(
                    "Invalid logging level: {}. Defaulting to info.",
                    logging_level
                );
                Level::INFO
            },
            |level| *level,
        );

        let router = self
            .router
            .route("/", get(hello_world))
            .route("/get_files", get(get_files).with_state(my_files_state))
            .route("/get_status", get(get_status).with_state(agent_data_state))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(server_logging_level))
                    .on_response(trace::DefaultOnResponse::new().level(server_logging_level))
                    .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
            );

        Server { address, router }
    }
}

pub trait Protocol {
    fn handle_post(&self, body: String) -> BoxFuture<'static, Json<HttpResponse>>;
}

impl Server {
    pub async fn start<T>(self, protocol: T)
        where
            T: Protocol {
        let addr: SocketAddr = match self.address.parse() {
            Ok(addr) => addr,
            Err(_) => {
                let default_config: Server = Server::default();
                error!(
                    "Invalid host or port: {}, defaulting to {}",
                    self.address, default_config.address
                );
                default_config.address.parse().unwrap()
            }
        };
        let tcp_listener = match TcpListener::bind::<SocketAddr>(addr).await {
            Ok(tcp_listener) => tcp_listener,
            Err(e) => {
                error!("Failed to bind to {}: {}", addr.to_string(), e);
                return;
            }
        };

        let future_response: Box<dyn Future<Output = Json<HttpResponse>> + Send> = Box::new(protocol.handle_post("test".to_string()));

        //TODO sending request to hub
        // let uuid = response_body.uuid.clone();
        // env::set_var("AGENT_UUID", &uuid);
        // info!("Set AGENT_UUID env var with value : {:?}", &uuid);

        info!("Http Server running at {}", addr.to_string());
        axum::serve(tcp_listener, self.router).await.unwrap();
    }
}