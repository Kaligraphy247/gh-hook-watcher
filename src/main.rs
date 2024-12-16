#![allow(unused_imports, dead_code)]
use axum::{
    extract::Json,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct GithubPushEvent {
    #[serde(rename = "ref")] // GitHub uses "ref" which is a keyword in Rust
    reference: String,
    repository: Repository,
    commits: Vec<Commit>,
}

#[derive(Debug, Deserialize)]
struct Repository {
    name: String,      // Repository Name
    full_name: String, // Full repository name (owner/repo)
}

#[derive(Debug, Deserialize)]
struct Commit {
    id: String,
    message: String,
    author: Author,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .compact()
        .init();

    // Create app router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/webhook", post(handle_webhook));

    // Set up server address
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(address)
        .await
        .expect("failed to bind the address");
    info!("Server starting on {}", address);

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

async fn handle_webhook(
    Json(payload): Json<GithubPushEvent>, //automatically parse JSON into GithubPushEvent
) -> impl IntoResponse {
    if payload.reference == "refs/heads/main" {
        //  Log repo information
        info!(
            "Received push to main branch in repository: {}",
            payload.repository.full_name
        );

        debug!("Full Payload: {:#?}", payload);

        // Log info about each commit in the push
        for commit in payload.commits {
            info!(
                "Commit: {} by {} ({})\nMessage: {}",
                &commit.id[..7],
                commit.author.name,
                commit.author.email,
                commit.message
            )
        }

        // Custom Logic to do when webhook is triggered
        // for now, log debug msg
        debug!("Action Triggered will run just below here");
        debug!("Should be non-blocking, and return immediately");
        debug!("Actions, include CI/CD, send notifications, run automated tests");
        debug!("end");
        StatusCode::OK // Return 200 if everything is successful
    } else {
        info!("Received push to non-main branch: {}", payload.reference);
        StatusCode::OK
    };
}


async fn root_handler() -> impl IntoResponse {
    info!("Root handler called");
    debug!("Debug working?");
    let body = serde_json::json!({
        "code": 200,
        "msg": "Github Hooks Watcher",
        "hasError": false,
        "error": Option::<String>::None,
    })
    .to_string();
    let mut headers = HeaderMap::new();
    headers.append(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    (StatusCode::OK, headers, body)
}
