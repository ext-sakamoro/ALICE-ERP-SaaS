// ALICE-ERP-SaaS core-engine
// License: AGPL-3.0-or-later

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Default, Clone, Serialize)]
struct Stats {
    total_requests: u64,
    inventory_requests: u64,
    production_requests: u64,
    payroll_requests: u64,
    courses_requests: u64,
}

#[derive(Clone)]
struct AppState {
    stats: Arc<Mutex<Stats>>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(Deserialize)]
struct InventoryUpdateRequest {
    sku: String,
    quantity_delta: i64,
    warehouse: Option<String>,
}

#[derive(Serialize)]
struct InventoryItem {
    id: String,
    sku: String,
    quantity_on_hand: i64,
    warehouse: String,
    reorder_point: i64,
}

#[derive(Deserialize)]
struct ProductionOrderRequest {
    product_id: String,
    quantity: u32,
    due_date: Option<String>,
}

#[derive(Serialize)]
struct ProductionOrder {
    id: String,
    product_id: String,
    quantity: u32,
    status: &'static str,
    estimated_completion: &'static str,
}

#[derive(Deserialize)]
struct PayrollRequest {
    employee_id: String,
    period: String,
}

#[derive(Serialize)]
struct PayrollRecord {
    id: String,
    employee_id: String,
    period: String,
    gross_pay: f64,
    deductions: f64,
    net_pay: f64,
}

#[derive(Deserialize)]
struct CourseRequest {
    title: Option<String>,
    employee_id: Option<String>,
}

#[derive(Serialize)]
struct Course {
    id: String,
    title: String,
    enrolled: u32,
    completion_rate: f64,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "alice-erp-core-engine",
        version: "0.1.0",
    })
}

async fn erp_inventory(
    State(state): State<AppState>,
    body: Option<Json<InventoryUpdateRequest>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.inventory_requests += 1;
    if let Some(Json(req)) = body {
        info!("erp/inventory POST sku={} delta={}", req.sku, req.quantity_delta);
        let item = InventoryItem {
            id: Uuid::new_v4().to_string(),
            sku: req.sku,
            quantity_on_hand: 100 + req.quantity_delta,
            warehouse: req.warehouse.unwrap_or_else(|| "MAIN".to_string()),
            reorder_point: 20,
        };
        Ok(Json(serde_json::to_value(item).unwrap()))
    } else {
        info!("erp/inventory GET");
        Ok(Json(serde_json::json!({ "items": [], "total_skus": 0 })))
    }
}

async fn erp_production(
    State(state): State<AppState>,
    body: Option<Json<ProductionOrderRequest>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.production_requests += 1;
    if let Some(Json(req)) = body {
        info!("erp/production POST product_id={} qty={}", req.product_id, req.quantity);
        let order = ProductionOrder {
            id: Uuid::new_v4().to_string(),
            product_id: req.product_id,
            quantity: req.quantity,
            status: "scheduled",
            estimated_completion: "2026-03-16T00:00:00Z",
        };
        Ok(Json(serde_json::to_value(order).unwrap()))
    } else {
        info!("erp/production GET");
        Ok(Json(serde_json::json!({ "orders": [], "total": 0 })))
    }
}

async fn erp_payroll(
    State(state): State<AppState>,
    body: Option<Json<PayrollRequest>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.payroll_requests += 1;
    if let Some(Json(req)) = body {
        info!("erp/payroll POST employee_id={} period={}", req.employee_id, req.period);
        let record = PayrollRecord {
            id: Uuid::new_v4().to_string(),
            employee_id: req.employee_id,
            period: req.period,
            gross_pay: 5_000.0,
            deductions: 1_200.0,
            net_pay: 3_800.0,
        };
        Ok(Json(serde_json::to_value(record).unwrap()))
    } else {
        info!("erp/payroll GET");
        Ok(Json(serde_json::json!({ "records": [], "total": 0 })))
    }
}

async fn erp_courses(
    State(state): State<AppState>,
    body: Option<Json<CourseRequest>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut stats = state.stats.lock().unwrap();
    stats.total_requests += 1;
    stats.courses_requests += 1;
    if let Some(Json(req)) = body {
        let title = req.title.unwrap_or_else(|| "New Course".to_string());
        info!("erp/courses POST title={}", title);
        let course = Course {
            id: Uuid::new_v4().to_string(),
            title,
            enrolled: 0,
            completion_rate: 0.0,
        };
        Ok(Json(serde_json::to_value(course).unwrap()))
    } else {
        info!("erp/courses GET");
        Ok(Json(serde_json::json!({ "courses": [], "total": 0 })))
    }
}

async fn erp_stats(State(state): State<AppState>) -> Json<Stats> {
    let stats = state.stats.lock().unwrap().clone();
    Json(stats)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let state = AppState {
        stats: Arc::new(Mutex::new(Stats::default())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/erp/inventory",  get(erp_inventory).post(erp_inventory))
        .route("/api/v1/erp/production", get(erp_production).post(erp_production))
        .route("/api/v1/erp/payroll",    get(erp_payroll).post(erp_payroll))
        .route("/api/v1/erp/courses",    get(erp_courses).post(erp_courses))
        .route("/api/v1/erp/stats",      get(erp_stats))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9145").await.unwrap();
    info!("alice-erp-core-engine listening on :9145");
    axum::serve(listener, app).await.unwrap();
}
