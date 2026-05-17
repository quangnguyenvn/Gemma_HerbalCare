use crate::{
    db::{retrieve_herbs, save_consultation},
    models::{ConsultResponse, Consultation, ConsultationRequest, DemoCase, HerbQuery},
    safety::{standard_disclaimer, triage, urgent_guidance},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use thiserror::Error;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/herbs", get(get_herbs))
        .route("/api/triage", post(post_triage))
        .route("/api/consult", post(post_consult))
        .route("/api/consultations/:id", get(get_consultation))
        .route("/api/demo-cases", get(demo_cases))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "Gemma HerbalCare" }))
}

async fn get_herbs(
    State(state): State<AppState>,
    Query(query): Query<HerbQuery>,
) -> Result<Json<Vec<crate::models::HerbSummary>>, ApiError> {
    let herbs = retrieve_herbs(
        &state.pool,
        query.country.as_deref(),
        query.region.as_deref(),
        query.symptom.as_deref(),
    )
    .await?;
    Ok(Json(herbs))
}

async fn post_triage(Json(input): Json<ConsultationRequest>) -> Json<crate::models::TriageResult> {
    Json(triage(&input))
}

async fn post_consult(
    State(state): State<AppState>,
    Json(input): Json<ConsultationRequest>,
) -> Result<Json<ConsultResponse>, ApiError> {
    let triage_result = triage(&input);
    let herbs = if matches!(triage_result.risk_level.as_str(), "emergency" | "urgent") {
        vec![]
    } else {
        retrieve_herbs(
            &state.pool,
            Some(&input.country),
            input.region.as_deref(),
            Some(&input.symptoms),
        )
        .await?
    };

    let assistant_response = if triage_result.risk_level == "emergency" {
        urgent_guidance()
    } else {
        state.gemma.complete(&input, &triage_result, &herbs).await?
    };

    let location = format!(
        "{}, {}, {}",
        input.city.clone().unwrap_or_default(),
        input.region.clone().unwrap_or_default(),
        input.country
    );
    let saved = save_consultation(
        &state.pool,
        &location,
        &serde_json::to_string(&input)?,
        &serde_json::to_string(&triage_result)?,
        &serde_json::to_string(&herbs)?,
        &assistant_response,
    )
    .await?;

    Ok(Json(ConsultResponse {
        consultation_id: Some(saved.last_insert_rowid()),
        triage: triage_result,
        local_herbs: herbs,
        assistant_response,
        disclaimer: standard_disclaimer(),
    }))
}

async fn get_consultation(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Consultation>, ApiError> {
    let consultation =
        sqlx::query_as::<_, Consultation>("SELECT * FROM consultations WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or(ApiError::NotFound)?;
    Ok(Json(consultation))
}

async fn demo_cases() -> Json<Vec<DemoCase>> {
    Json(vec![
        DemoCase {
            title: "Child diarrhea after unsafe water".to_string(),
            request: ConsultationRequest {
                country: "Nigeria".to_string(),
                region: Some("Kano".to_string()),
                city: Some("Kano".to_string()),
                symptoms: "child has mild diarrhea and loose stool after drinking untreated well water, still drinking, no visible blood, no dehydration signs".to_string(),
                age_group: "child".to_string(),
                care_accessible: false,
                pregnant: false,
                known_conditions: vec![],
                current_medicines: vec![],
                allergies: vec![],
                duration_days: Some(1),
            },
        },
        DemoCase {
            title: "Fever and suspected malaria".to_string(),
            request: ConsultationRequest {
                country: "Nigeria".to_string(),
                region: Some("Kano".to_string()),
                city: Some("Kano".to_string()),
                symptoms: "fever, chills, headache and suspected malaria after mosquito bites".to_string(),
                age_group: "child".to_string(),
                care_accessible: false,
                pregnant: false,
                known_conditions: vec![],
                current_medicines: vec![],
                allergies: vec![],
                duration_days: Some(2),
            },
        },
        DemoCase {
            title: "Possible worms in a child".to_string(),
            request: ConsultationRequest {
                country: "Nigeria".to_string(),
                region: Some("Kano".to_string()),
                city: Some("Kano".to_string()),
                symptoms: "child has abdominal discomfort, poor appetite, itchy bottom at night, and possible worms in stool".to_string(),
                age_group: "child".to_string(),
                care_accessible: true,
                pregnant: false,
                known_conditions: vec![],
                current_medicines: vec![],
                allergies: vec![],
                duration_days: Some(7),
            },
        },
        DemoCase {
            title: "How can we make water safer?".to_string(),
            request: ConsultationRequest {
                country: "Nigeria".to_string(),
                region: Some("Kano".to_string()),
                city: Some("Kano".to_string()),
                symptoms: "family only has cloudy well water and asks how to make drinking water safer with basic household tools".to_string(),
                age_group: "adult".to_string(),
                care_accessible: false,
                pregnant: false,
                known_conditions: vec![],
                current_medicines: vec![],
                allergies: vec![],
                duration_days: Some(0),
            },
        },
        DemoCase {
            title: "Long-term food source plan".to_string(),
            request: ConsultationRequest {
                country: "Nigeria".to_string(),
                region: Some("Kano".to_string()),
                city: Some("Kano".to_string()),
                symptoms: "family asks for a simple long-term food source plan such as growing sweet potato, moringa, and raising chickens with limited money".to_string(),
                age_group: "adult".to_string(),
                care_accessible: false,
                pregnant: false,
                known_conditions: vec![],
                current_medicines: vec![],
                allergies: vec![],
                duration_days: Some(0),
            },
        },
        DemoCase {
            title: "Poor breathing after indoor smoke".to_string(),
            request: ConsultationRequest {
                country: "Nigeria".to_string(),
                region: Some("Kano".to_string()),
                city: Some("Kano".to_string()),
                symptoms: "child has difficulty breathing, fast breathing, and coughing after indoor cooking smoke".to_string(),
                age_group: "child".to_string(),
                care_accessible: false,
                pregnant: false,
                known_conditions: vec![],
                current_medicines: vec![],
                allergies: vec![],
                duration_days: Some(0),
            },
        },
    ])
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("not found")]
    NotFound,
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}
