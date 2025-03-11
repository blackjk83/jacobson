use std::sync::Arc;
use axum::{
    routing::get,
    Router,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use prometheus::{
    Registry, Gauge, Encoder,
    register_gauge_with_registry,
};
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::{MetricsCollector, SystemMetrics, SystemInfoError};

#[derive(Clone)]
pub struct AppState {
    collector: Arc<RwLock<MetricsCollector>>,
    registry: Arc<Registry>,
    metrics: Arc<PrometheusMetrics>,
}

pub struct PrometheusMetrics {
    cpu_usage: Gauge,
    memory_used: Gauge,
    memory_total: Gauge,
}

impl PrometheusMetrics {
    fn new() -> Result<(Self, Registry), SystemInfoError> {
        let registry = Registry::new();

        let cpu_usage = register_gauge_with_registry!(
            "system_cpu_usage",
            "CPU usage percentage",
            registry
        ).map_err(|e| SystemInfoError::ServiceError(e.to_string()))?;

        let memory_used = register_gauge_with_registry!(
            "system_memory_used_bytes",
            "Memory used in bytes",
            registry
        ).map_err(|e| SystemInfoError::ServiceError(e.to_string()))?;

        let memory_total = register_gauge_with_registry!(
            "system_memory_total_bytes",
            "Total memory in bytes",
            registry
        ).map_err(|e| SystemInfoError::ServiceError(e.to_string()))?;

        Ok((Self {
            cpu_usage,
            memory_used,
            memory_total,
        }, registry))
    }

    fn update(&self, metrics: &SystemMetrics) {
        self.cpu_usage.set(metrics.cpu.usage as f64);
        self.memory_used.set(metrics.memory.used as f64);
        self.memory_total.set(metrics.memory.total as f64);
    }
}

pub async fn run_service(port: u16, update_interval: std::time::Duration) -> Result<(), SystemInfoError> {
    // Initialize tracing
    tracing_subscriber::fmt().init();

    // Initialize shared state
    let (prometheus_metrics, registry) = PrometheusMetrics::new()?;
    
    let state = AppState {
        collector: Arc::new(RwLock::new(MetricsCollector::new(update_interval))),
        registry: Arc::new(registry),
        metrics: Arc::new(prometheus_metrics),
    };

    // Build router
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/api/v1/metrics", get(json_metrics_handler))
        .route("/health", get(health_handler))
        .with_state(state.clone());

    // Start background metrics collection
    let collector_state = state.clone();
    tokio::spawn(async move {
        loop {
            match collector_state.collector.write().await.collect().await {
                Ok(metrics) => {
                    collector_state.metrics.update(&metrics);
                    info!("Metrics collected successfully");
                }
                Err(e) => {
                    warn!("Failed to collect metrics: {}", e);
                }
            }
            tokio::time::sleep(collector_state.collector.read().await.update_interval).await;
        }
    });

    // Start server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await
        .map_err(|e| SystemInfoError::ServiceError(e.to_string()))?;
    
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| SystemInfoError::ServiceError(e.to_string()))?;

    Ok(())
}

async fn metrics_handler(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
    let mut buffer = vec![];
    let encoder = prometheus::TextEncoder::new();
    encoder.encode(&state.registry.gather(), &mut buffer)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    String::from_utf8(buffer)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn json_metrics_handler(State(state): State<AppState>) -> Result<Json<SystemMetrics>, (StatusCode, String)> {
    match state.collector.write().await.collect().await {
        Ok(metrics) => Ok(Json(metrics)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
