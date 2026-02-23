//! HTTP API server for CADSP
//! Exposes repository scanning, analysis, and code synthesis endpoints

use axum::{
    extract::{Json, State},
    response::Json as JsonResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use cadsp_core::*;

#[derive(Clone)]
struct AppState {
    github_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScanRequest {
    repo_url: String,
    analysis_depth: Option<String>,
    sandbox_mode: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScanResponse {
    scan_id: String,
    status: String,
    metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisRequest {
    repo_id: String,
    code_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisResponse {
    discoveries: Vec<DiscoveredObject>,
    neuro_path: NeuroNodePath,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let github_token = std::env::var("GITHUB_TOKEN")
        .unwrap_or_else(|_| {
            tracing::warn!("GITHUB_TOKEN not set, some features limited");
            String::new()
        });

    let state = AppState { github_token };

    let app = Router::new()
        .route("/api/v1/repositories/scan", post(scan_repository))
        .route("/api/v1/analysis/discover", post(analyze_code))
        .route("/health", axum::routing::get(health_check))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind port 8080");

    tracing::info!("🧠 CADSP API listening on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn scan_repository(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ScanRequest>,
) -> JsonResponse<ScanResponse> {
    tracing::info!("Scan request for: {}", req.repo_url);

    let scanner = RepositoryScanner::new(state.github_token.clone());

    match scanner.scan(&req.repo_url).await {
        Ok(metadata) => {
            tracing::info!("Scan completed: {}", metadata.scan_id);
            JsonResponse(ScanResponse {
                scan_id: metadata.scan_id.clone(),
                status: "completed".to_string(),
                metadata: serde_json::to_value(&metadata).unwrap_or(serde_json::json!({})),
            })
        }
        Err(e) => {
            tracing::error!("Scan failed: {}", e);
            JsonResponse(ScanResponse {
                scan_id: "ERROR".to_string(),
                status: format!("failed: {}", e),
                metadata: serde_json::json!({}),
            })
        }
    }
}

async fn analyze_code(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<AnalysisRequest>,
) -> JsonResponse<AnalysisResponse> {
    tracing::info!("Analysis request for repo: {}", req.repo_id);

    match biophysical_patterns::PatternDetector::detect(&req.code_content) {
        Ok(discoveries) => {
            let objects: Vec<_> = discoveries
                .iter()
                .map(|d| (d.id.clone(), d.confidence_score))
                .collect();

            match neuro_node_path::NeuroNodePathEngine::compute_path(&req.repo_id, &objects) {
                Ok(neuro_path) => {
                    tracing::info!("Analysis completed: {} discoveries", discoveries.len());
                    JsonResponse(AnalysisResponse {
                        discoveries,
                        neuro_path,
                    })
                }
                Err(e) => {
                    tracing::error!("Path computation failed: {}", e);
                    JsonResponse(AnalysisResponse {
                        discoveries: vec![],
                        neuro_path: Default::default(),
                    })
                }
            }
        }
        Err(e) => {
            tracing::error!("Pattern detection failed: {}", e);
            JsonResponse(AnalysisResponse {
                discoveries: vec![],
                neuro_path: Default::default(),
            })
        }
    }
}

async fn health_check() -> &'static str {
    "✓ CADSP API operational"
}
