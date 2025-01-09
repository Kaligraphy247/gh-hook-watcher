use serde_json::json;
use ring::hmac::{self, Key};
use tracing::{debug, info, Level};
use dotenv::dotenv;
use hex::FromHexError;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::FmtSubscriber;
use axum::{
    extract::Json,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use gh_hook_watcher::GitHubPayloadBody;

#[tokio::main]
async fn main() {
    dotenv().ok();
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
    headers: HeaderMap,
    Json(payload): Json<GitHubPayloadBody>,
) -> impl IntoResponse {
    // Verify webhook signature
    let webhook_secret = match std::env::var("GH_WEBHOOK_SECRET") {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                json!({"msg": e.to_string()}).to_string(),
            )
        }
    };
    let secret_key = Key::new(hmac::HMAC_SHA256, webhook_secret.as_bytes());
    let signature = match get_signature_from_header(&headers) {
        Ok(sig) => sig,
        Err(e) => return (StatusCode::BAD_REQUEST, json!({"msg": e}).to_string()),
    };
    let message = match payload.convert_to_json_string() {
        Ok(msg) => msg,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                json!({"msg": "failed to convert payload"}).to_string(),
            )
        }
    };
    let verified = verify_signature(&secret_key, message.as_bytes(), &signature);

    if verified {
        // what to listen for
        if payload.reference == "refs/heads/main" {
            //  Log repo information
            info!(
                "Received push to main branch in repository: {}",
                payload.repository.full_name
            );

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
            (StatusCode::OK, json!({"msg": "Success"}).to_string())
        } else {
            info!(
                "Received push to non-main branch: {:?}",
                payload.convert_to_json_string()
            );
            (
                StatusCode::OK,
                json!({"msg": "Received push on non-main branch"}).to_string(),
            )
        }
    } else {
        (
            StatusCode::BAD_REQUEST,
            json!({"msg": "Signature verification failed"}).to_string(),
        )
    }
}

async fn root_handler(_headers: HeaderMap) -> impl IntoResponse {
    info!("Root handler called");
    let body = json!({
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

fn get_signature_from_header(headers: &HeaderMap) -> Result<Vec<u8>, &'static str> {
    let signature = headers
        .get("X-Hub-Signature-256")
        .ok_or("Missing X-Hub-Signature-256 header")?
        .to_str()
        .map_err(|_| "Invalid X-Hub-Signature-256 header format")?
        .strip_prefix("sha256=")
        .ok_or("X-Hub-Signature-256 header missing prefix")?;

    hex::decode(signature).map_err(|_: FromHexError| "Invalid X-Hub-Signature-256 hex value")
}

fn verify_signature(secret_key: &Key, message: &[u8], expected_signature: &[u8]) -> bool {
    let verified = hmac::verify(secret_key, message, expected_signature);
    verified.is_ok()
}
