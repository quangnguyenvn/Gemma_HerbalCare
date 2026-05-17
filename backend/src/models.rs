use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Herb {
    pub id: i64,
    pub common_name: String,
    pub latin_name: String,
    pub local_names_json: String,
    pub regions_json: String,
    pub plant_part: String,
    pub traditional_uses_json: String,
    pub symptom_tags_json: String,
    pub evidence_level: String,
    pub safety_summary: String,
    pub contraindications_json: String,
    pub interactions_json: String,
    pub preparation_note: String,
    pub use_guidance: String,
    pub source_url: String,
    pub image_url: String,
    pub image_source_url: String,
    pub identification_features_json: String,
    pub local_availability: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Condition {
    pub id: i64,
    pub name: String,
    pub severity_class: String,
    pub symptom_tags_json: String,
    pub red_flags_json: String,
    pub safe_response_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Availability {
    pub id: i64,
    pub herb_id: i64,
    pub country: String,
    pub region: String,
    pub confidence: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Consultation {
    pub id: i64,
    pub created_at: String,
    pub location: String,
    pub user_input_json: String,
    pub triage_result_json: String,
    pub retrieved_herbs_json: String,
    pub gemma_response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultationRequest {
    pub country: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub symptoms: String,
    pub age_group: String,
    #[serde(default = "default_care_accessible")]
    pub care_accessible: bool,
    pub pregnant: bool,
    pub known_conditions: Vec<String>,
    pub current_medicines: Vec<String>,
    pub allergies: Vec<String>,
    pub duration_days: Option<i64>,
}

fn default_care_accessible() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageResult {
    pub risk_level: String,
    pub red_flags: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HerbSummary {
    pub id: i64,
    pub common_name: String,
    pub latin_name: String,
    pub why_relevant: String,
    pub evidence_level: String,
    pub safety_summary: String,
    pub contraindications: Vec<String>,
    pub interactions: Vec<String>,
    pub preparation_note: String,
    pub use_guidance: String,
    pub source_url: String,
    pub image_url: String,
    pub image_source_url: String,
    pub identification_features: Vec<String>,
    pub local_availability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsultResponse {
    pub consultation_id: Option<i64>,
    pub triage: TriageResult,
    pub local_herbs: Vec<HerbSummary>,
    pub assistant_response: String,
    pub disclaimer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoCase {
    pub title: String,
    pub request: ConsultationRequest,
}

#[derive(Debug, Deserialize)]
pub struct HerbQuery {
    pub country: Option<String>,
    pub region: Option<String>,
    pub symptom: Option<String>,
}
