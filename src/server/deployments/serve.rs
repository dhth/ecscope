use super::super::utils::find_open_port_in_range;
use crate::common::{DeploymentState, Environment};
use crate::config::{ClusterConfig, ConfigSource};
use crate::domain::{DeploymentDetails, DeploymentError};
use crate::service::get_deployments;
use aws_sdk_ecs::Client as ECSClient;
use axum::Json;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::response::{Html, IntoResponse};
use axum::{Router, routing::get};
use rand::Rng;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::sync::Arc;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

const ENV_VAR_PORT: &str = "ECSCOPE_PORT";
const ROOT_HTML: &str = include_str!("client/index.html");
const DEPS_JS: &str = include_str!("client/priv/static/deps.mjs");
const DEPS_CSS: &str = include_str!("client/priv/static/deps.css");
const DEPS_CUSTOM_CSS: &str = include_str!("client/priv/static/custom.css");
const DEPS_FAVICON: &[u8] = include_bytes!("client/priv/static/favicon.png");

#[derive(serde::Serialize)]
struct GetDeploymentsResponse {
    deployments: Vec<DeploymentDetails>,
    errors: Vec<DeploymentError>,
}

#[derive(serde::Serialize)]
struct ApiError {
    error: String,
}

impl IntoResponse for GetDeploymentsResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServeDeploymentsError {
    #[error("couldn't find open port")]
    CouldntFindOpenPort,
    #[error("incorrect port provided: {0}")]
    IncorrectPortProvided(String),
    #[error("couldn't bind to address: {0}")]
    CouldntBindToAddress(IOError),
    #[error("couldn't start server: {0}")]
    CouldntStartServer(IOError),
}

pub async fn serve_deployments(
    clusters: Vec<ClusterConfig>,
    clients_map: Arc<HashMap<ConfigSource, ECSClient>>,
    state: Option<DeploymentState>,
    skip_opening: bool,
    env: Environment,
) -> Result<(), ServeDeploymentsError> {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let router = Router::new()
        .route("/", get(move || root_get(env)))
        .route("/priv/static/deps.mjs", get(move || js_get(env)))
        .route("/priv/static/deps.css", get(move || css_get(env)))
        .route("/priv/static/custom.css", get(move || css_custom_get(env)))
        .route("/priv/static/favicon.png", get(favicon_get))
        .route("/dev/api/deps", get(fake_deployments_get))
        .route(
            "/api/deps",
            get({
                let clients_map = Arc::clone(&clients_map);
                move || deployments_get(clusters, clients_map, state)
            }),
        )
        .layer(cors);

    let port = match std::env::var(ENV_VAR_PORT) {
        Ok(port_str) => match port_str.parse::<u16>() {
            Ok(p) => Ok(p),
            Err(e) => Err(ServeDeploymentsError::IncorrectPortProvided(e.to_string())),
        },
        Err(e) => match e {
            std::env::VarError::NotPresent => find_open_port_in_range(4500, 5000)
                .ok_or(ServeDeploymentsError::CouldntFindOpenPort),
            std::env::VarError::NotUnicode(s) => Err(ServeDeploymentsError::IncorrectPortProvided(
                s.to_string_lossy().to_string(),
            )),
        },
    }?;

    let address = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(ServeDeploymentsError::CouldntBindToAddress)?;

    let http_address = format!("http://{}", &address);

    if !skip_opening {
        if open::that(&http_address).is_err() {
            eprintln!(
                "couldn't open your browser, please open the following address manually:\n{}",
                &http_address
            )
        } else {
            println!("serving results on {}", &http_address);
        }
    } else {
        println!("serving results on {}", &http_address);
    }

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(ServeDeploymentsError::CouldntStartServer)?;

    Ok(())
}

async fn js_get(env: Environment) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    #[allow(clippy::unwrap_used)]
    headers.insert("Content-Type", "text/javascript".parse().unwrap());
    let js = match env {
        Environment::Dev =>
        {
            #[allow(clippy::unwrap_used)]
            tokio::fs::read_to_string("src/server/deployments/client/priv/static/deps.mjs")
                .await
                .unwrap()
        }
        Environment::Prod => DEPS_JS.to_string(),
    };
    (headers, js)
}

async fn css_get(env: Environment) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    #[allow(clippy::unwrap_used)]
    headers.insert("Content-Type", "text/css".parse().unwrap());
    let css = match env {
        Environment::Dev =>
        {
            #[allow(clippy::unwrap_used)]
            tokio::fs::read_to_string("src/server/deployments/client/priv/static/deps.css")
                .await
                .unwrap()
        }
        Environment::Prod => DEPS_CSS.to_string(),
    };
    (headers, css)
}

async fn css_custom_get(env: Environment) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    #[allow(clippy::unwrap_used)]
    headers.insert("Content-Type", "text/css".parse().unwrap());
    let css = match env {
        Environment::Dev =>
        {
            #[allow(clippy::unwrap_used)]
            tokio::fs::read_to_string("src/server/deployments/client/priv/static/custom.css")
                .await
                .unwrap()
        }
        Environment::Prod => DEPS_CUSTOM_CSS.to_string(),
    };
    (headers, css)
}

async fn favicon_get() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    #[allow(clippy::unwrap_used)]
    headers.insert("Content-Type", "image/png".parse().unwrap());
    (headers, DEPS_FAVICON)
}

async fn root_get(env: Environment) -> impl IntoResponse {
    match env {
        Environment::Dev => {
            #[allow(clippy::unwrap_used)]
            let html = tokio::fs::read_to_string("src/server/deployments/client/index.html")
                .await
                .unwrap();
            Html(html)
        }
        Environment::Prod => Html(ROOT_HTML.to_string()),
    }
}

async fn deployments_get(
    clusters: Vec<ClusterConfig>,
    clients_map: Arc<HashMap<ConfigSource, ECSClient>>,
    state: Option<DeploymentState>,
) -> Result<GetDeploymentsResponse, ApiError> {
    let (deployments, errors) = get_deployments(clusters, clients_map, state)
        .await
        .map_err(|error| ApiError { error })?;

    let response = GetDeploymentsResponse {
        deployments,
        errors,
    };

    Ok(response)
}

async fn fake_deployments_get() -> Result<GetDeploymentsResponse, ApiError> {
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    let mut rng = rand::rng();

    if rng.random_bool(1.0 / 10.0) {
        return Err(ApiError {
            error: "Something went wrong".to_string(),
        });
    }

    let num_services = rng.random_range(1..=5);

    let services = (0..num_services)
        .map(|i| format!("service-{}", i))
        .collect::<Vec<_>>();

    let mut deployments = vec![];
    let envs = ["qa", "staging", "prod"];

    services.iter().for_each(|s| {
        envs.iter().for_each(|env| {
            let dep = if rng.random_bool(0.6) {
                DeploymentDetails::dummy_running(s, env)
            } else {
                let choices = [
                    DeploymentDetails::dummy_pending(s, env),
                    DeploymentDetails::dummy_active(s, env),
                    DeploymentDetails::dummy_failing(s, env),
                    DeploymentDetails::dummy_draining(s, env),
                ];
                choices[rng.random_range(0..choices.len())].clone()
            };

            deployments.push(dep);
        });
    });

    let num_errors = rng.random_range(1..=5);
    let errors = (0..num_errors)
        .map(|i| DeploymentError {
            service_name: format!("service-{}", i),
            error: "couldn't get access token\nLine 2\nLine 3\nLine 4".to_string(),
            cluster_arn: format!("cluster-{}", i),
            keys: "qa".to_string(),
        })
        .collect::<Vec<_>>();

    Ok(GetDeploymentsResponse {
        deployments,
        errors,
    })
}

#[allow(clippy::expect_used)]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("\nbye 👋");
}
