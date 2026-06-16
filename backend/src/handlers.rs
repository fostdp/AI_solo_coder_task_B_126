use crate::db::Database;
use crate::message_bus::{BusMessage, SensorToCastingTx, SensorToAcousticTx, SensorToDtuTx};
use crate::models::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub tx_to_dtu: SensorToDtuTx,
    pub tx_to_casting: SensorToCastingTx,
    pub tx_to_acoustic: SensorToAcousticTx,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: Option<usize>,
}

pub async fn health_check() -> impl IntoResponse {
    Json(ApiResponse::ok(serde_json::json!({
        "status": "ok",
        "service": "bell-casting-backend",
        "version": "1.0.0",
        "timestamp": Utc::now().to_rfc3339(),
    })))
}

pub async fn get_bells(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.get_all_bells().await {
        Ok(bells) => Json(ApiResponse::ok(bells)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<Bell>>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn get_bell(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.db.get_bell(id).await {
        Ok(Some(bell)) => Json(ApiResponse::ok(bell)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Bell>::err("钟体不存在")),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Bell>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn post_sensor_reading(
    State(state): State<AppState>,
    Json(input): Json<SensorReadingIn>,
) -> impl IntoResponse {
    let reading = SensorReading {
        reading_id: Uuid::new_v4(),
        bell_id: input.bell_id,
        timestamp: Utc::now(),
        temp_celsius: input.temp_celsius,
        temp_gradient: input.temp_gradient,
        wall_thickness_mm: input.wall_thickness_mm,
        thickness_deviation: input.thickness_deviation,
        alloy_cu: input.alloy_cu,
        alloy_sn: input.alloy_sn,
        alloy_pb: input.alloy_pb,
        alloy_zn: input.alloy_zn,
        alloy_other: input.alloy_other,
        acoustic_freq_hz: input.acoustic_freq_hz,
        acoustic_amplitude: input.acoustic_amplitude,
        acoustic_decay: input.acoustic_decay,
        acoustic_harmonics: serde_json::to_string(&input.acoustic_harmonics).unwrap_or_default(),
    };

    let bell_opt = state.db.get_bell(reading.bell_id).await.ok().flatten();
    let reading_id = reading.reading_id;

    let msg = BusMessage::SensorReadingReceived {
        reading,
        bell: bell_opt,
    };

    match state.tx_to_dtu.send(msg).await {
        Ok(_) => Json(ApiResponse::ok(serde_json::json!({
            "reading_id": reading_id,
            "status": "submitted_to_dtu_receiver",
        })))
        .into_response(),
        Err(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiResponse::<serde_json::Value>::err(format!(
                "DTU接收器不可用: {}",
                e
            ))),
        )
            .into_response(),
    }
}

pub async fn get_sensor_readings(
    State(state): State<AppState>,
    Path(bell_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let limit = pagination.limit.unwrap_or(100);
    match state.db.get_sensor_readings(bell_id, limit).await {
        Ok(readings) => Json(ApiResponse::ok(readings)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<SensorReading>>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn run_casting_simulation(
    State(state): State<AppState>,
    Json(req): Json<CastingSimRequest>,
) -> impl IntoResponse {
    let bell_opt = state.db.get_bell(req.bell_id).await.ok().flatten();
    let req_clone = req.clone();

    let msg = BusMessage::CastingSimRequested {
        req,
        bell: bell_opt,
    };

    match state.tx_to_casting.send(msg).await {
        Ok(_) => Json(ApiResponse::ok(serde_json::json!({
            "status": "submitted_to_casting_simulator",
            "request": req_clone,
        })))
        .into_response(),
        Err(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiResponse::<serde_json::Value>::err(format!(
                "铸造仿真器不可用: {}",
                e
            ))),
        )
            .into_response(),
    }
}

pub async fn get_casting_simulations(
    State(state): State<AppState>,
    Path(bell_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let limit = pagination.limit.unwrap_or(20);
    match state.db.get_casting_simulations(bell_id, limit).await {
        Ok(sims) => Json(ApiResponse::ok(sims)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<CastingSimulation>>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn run_acoustic_simulation(
    State(state): State<AppState>,
    Json(req): Json<AcousticSimRequest>,
) -> impl IntoResponse {
    let bell_opt = state.db.get_bell(req.bell_id).await.ok().flatten();
    let req_clone = req.clone();

    let msg = BusMessage::AcousticSimRequested {
        req,
        bell: bell_opt,
    };

    match state.tx_to_acoustic.send(msg).await {
        Ok(_) => Json(ApiResponse::ok(serde_json::json!({
            "status": "submitted_to_acoustic_simulator",
            "request": req_clone,
        })))
        .into_response(),
        Err(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiResponse::<serde_json::Value>::err(format!(
                "声学仿真器不可用: {}",
                e
            ))),
        )
            .into_response(),
    }
}

pub async fn get_acoustic_simulations(
    State(state): State<AppState>,
    Path(bell_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let limit = pagination.limit.unwrap_or(20);
    match state.db.get_acoustic_simulations(bell_id, limit).await {
        Ok(sims) => Json(ApiResponse::ok(sims)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<AcousticSimulation>>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn get_active_alerts(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.get_active_alerts().await {
        Ok(alerts) => Json(ApiResponse::ok(alerts)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<Alert>>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn resolve_alert(
    State(state): State<AppState>,
    Path(alert_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.db.resolve_alert(alert_id).await {
        Ok(_) => Json(ApiResponse::ok(serde_json::json!({
            "alert_id": alert_id,
            "resolved": true,
        })))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<serde_json::Value>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn post_casting_process(
    State(state): State<AppState>,
    Json(input): Json<CastingProcess>,
) -> impl IntoResponse {
    let process = CastingProcess {
        process_id: Uuid::new_v4(),
        timestamp: Utc::now(),
        ..input
    };
    match state.db.insert_casting_process(&process).await {
        Ok(_) => Json(ApiResponse::ok(process)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<CastingProcess>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

pub async fn get_casting_process(
    State(state): State<AppState>,
    Path(bell_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let limit = pagination.limit.unwrap_or(50);
    match state.db.get_casting_process(bell_id, limit).await {
        Ok(processes) => Json(ApiResponse::ok(processes)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<CastingProcess>>::err(format!("{}", e))),
        )
            .into_response(),
    }
}

// ========== Feature 1: 合金配比音质对比分析 ==========

pub async fn post_alloy_comparison(
    State(state): State<AppState>,
    Json(req): Json<AlloyComparisonRequest>,
) -> impl IntoResponse {
    let bell = if let Some(id) = req.bell_id {
        state.db.get_bell(id).await.ok().flatten()
    } else {
        None
    };
    let result = crate::simulation::compare_alloys(&req, bell.as_ref());
    Json(ApiResponse::ok(result))
}

#[derive(Debug, Deserialize)]
pub struct AlloySuggestionQuery {
    pub target_freq_hz: f64,
    pub max_deviation_cents: Option<f64>,
}

pub async fn get_alloy_suggestion(
    Query(query): Query<AlloySuggestionQuery>,
) -> impl IntoResponse {
    let max_dev = query.max_deviation_cents.unwrap_or(25.0);
    let suggestion = crate::simulation::get_alloy_composition_suggestion(query.target_freq_hz, max_dev);
    Json(ApiResponse::ok(suggestion))
}

// ========== Feature 2: 古代vs现代铸造工艺对比 ==========

pub async fn post_casting_method_comparison(
    Json(req): Json<CastingMethodRequest>,
) -> impl IntoResponse {
    let result = crate::simulation::compare_casting_methods(&req);
    Json(ApiResponse::ok(result))
}

pub async fn get_casting_method_list() -> impl IntoResponse {
    let list = crate::simulation::get_casting_method_key_list();
    Json(ApiResponse::ok(list))
}

#[derive(Debug, Deserialize)]
pub struct RecommendMethodQuery {
    pub bell_weight_tons: f64,
    pub acoustic_requirement: Option<f64>,
    pub budget_per_kg: Option<f64>,
    pub need_complex_artwork: Option<bool>,
}

pub async fn get_recommended_method(
    Query(query): Query<RecommendMethodQuery>,
) -> impl IntoResponse {
    let acq = query.acoustic_requirement.unwrap_or(8.0);
    let budget = query.budget_per_kg.unwrap_or(200.0);
    let artwork = query.need_complex_artwork.unwrap_or(false);
    let method = crate::simulation::get_recommended_method_for_bell(
        query.bell_weight_tons,
        acq,
        budget,
        artwork,
    );
    Json(ApiResponse::ok(serde_json::json!({
        "recommended_method_key": method
    })))
}

// ========== Feature 3: 钟楼建筑声学传播模拟 ==========

pub async fn post_tower_acoustic_sim(
    Json(req): Json<TowerAcousticRequest>,
) -> impl IntoResponse {
    let result = crate::simulation::simulate_tower_acoustics(&req);
    Json(ApiResponse::ok(result))
}

pub async fn get_preset_tower_configs() -> impl IntoResponse {
    let configs = crate::simulation::get_preset_tower_configs();
    Json(ApiResponse::ok(configs))
}

// ========== Feature 4: 虚拟敲钟体验 ==========

pub async fn post_virtual_strike(
    State(state): State<AppState>,
    Json(params): Json<VirtualStrikeParams>,
) -> impl IntoResponse {
    let bell = state.db.get_bell(params.bell_id).await.ok().flatten();
    let result = crate::simulation::compute_strike_impact(&params, bell.as_ref());
    Json(ApiResponse::ok(result))
}

pub async fn get_strike_options() -> impl IntoResponse {
    let positions: Vec<serde_json::Value> = crate::simulation::get_position_options()
        .into_iter()
        .map(|(key, desc, factor)| {
            serde_json::json!({
                "key": key,
                "description": desc,
                "amplitude_factor": factor
            })
        })
        .collect();
    let mallets: Vec<serde_json::Value> = crate::simulation::get_mallet_options()
        .into_iter()
        .map(|(key, name, desc)| {
            serde_json::json!({
                "key": key,
                "name": name,
                "description": desc
            })
        })
        .collect();
    Json(ApiResponse::ok(serde_json::json!({
        "positions": positions,
        "mallets": mallets
    })))
}

pub async fn get_strike_tutorial() -> impl IntoResponse {
    let tutorial = crate::simulation::generate_strike_tutorial();
    Json(ApiResponse::ok(tutorial))
}
