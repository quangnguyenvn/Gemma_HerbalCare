use crate::models::{ConsultationRequest, TriageResult};

const RED_FLAGS: &[&str] = &[
    "chest pain",
    "difficulty breathing",
    "blue lips",
    "confusion",
    "unconscious",
    "seizure",
    "severe dehydration",
    "blood in vomit",
    "blood in stool",
    "fever above 39.5",
    "fever lasting more than 3 days",
    "severe abdominal pain",
    "stiff neck",
    "pregnancy with bleeding",
    "infant under 3 months with fever",
    "suspected cancer",
    "hiv/aids without clinical care",
    "stroke symptoms",
    "severe allergic reaction",
];

const SERIOUS_DISEASES: &[&str] = &[
    "cancer",
    "hiv",
    "aids",
    "tuberculosis",
    "tb",
    "malaria",
    "stroke",
    "heart attack",
    "sepsis",
    "kidney failure",
    "liver failure",
    "uncontrolled diabetes",
];

pub fn triage(input: &ConsultationRequest) -> TriageResult {
    let combined = format!(
        "{} {} {}",
        input.symptoms,
        input.known_conditions.join(" "),
        input.allergies.join(" ")
    )
    .to_lowercase();

    let mut red_flags: Vec<String> = RED_FLAGS
        .iter()
        .filter(|flag| combined.contains(**flag))
        .map(|flag| (*flag).to_string())
        .collect();

    if input.pregnant && (combined.contains("bleeding") || combined.contains("severe pain")) {
        red_flags.push("pregnancy with bleeding or severe pain".to_string());
    }
    if input.age_group.to_lowercase().contains("infant") && combined.contains("fever") {
        red_flags.push("infant under 3 months with fever".to_string());
    }
    if input.age_group.to_lowercase().contains("child")
        && (combined.contains("lethargic")
            || combined.contains("very weak")
            || combined.contains("cannot drink")
            || combined.contains("unable to drink"))
    {
        red_flags.push("child with possible severe dehydration or severe illness".to_string());
    }
    if input.duration_days.unwrap_or_default() > 3 && combined.contains("fever") {
        red_flags.push("fever lasting more than 3 days".to_string());
    }
    if combined.contains("39.6") || combined.contains("40") || combined.contains("104f") {
        red_flags.push("fever above 39.5C".to_string());
    }

    red_flags.sort();
    red_flags.dedup();
    if !red_flags.is_empty() {
        return TriageResult {
            risk_level: "emergency".to_string(),
            red_flags,
            reason:
                "One or more emergency red flags were detected. Herbal suggestions are suppressed."
                    .to_string(),
        };
    }

    if combined.contains("malaria") {
        return TriageResult {
            risk_level: "urgent".to_string(),
            red_flags: vec![],
            reason: "Suspected malaria needs prompt testing and antimalarial medicine. Quinine originally came from Cinchona bark, but raw bark or self-sourced quinine should not be used for malaria because dosing and toxicity are unsafe without clinical care.".to_string(),
        };
    }

    if SERIOUS_DISEASES
        .iter()
        .any(|disease| combined.contains(disease))
        || combined.contains("cure")
    {
        return TriageResult {
            risk_level: "urgent".to_string(),
            red_flags: vec![],
            reason: "The request involves a serious condition or cure claim. The app can only encourage professional care and discuss supportive comfort, not cures.".to_string(),
        };
    }

    let risk_level = if input.pregnant
        || input.age_group.to_lowercase().contains("child")
        || !input.current_medicines.is_empty()
        || !input.known_conditions.is_empty()
        || !input.allergies.is_empty()
    {
        "caution"
    } else {
        "low"
    };

    TriageResult {
        risk_level: risk_level.to_string(),
        red_flags: vec![],
        reason: "No emergency red flags detected from the provided information.".to_string(),
    }
}

pub fn urgent_guidance() -> String {
    "This may need urgent medical care. Please seek emergency help now, call your local emergency number if available, or go to the nearest clinic or hospital. Do not use herbs or home remedies to delay care for these symptoms.".to_string()
}

pub fn standard_disclaimer() -> String {
    "Gemma HerbalCare is an educational herbal knowledge navigator. It is not medical advice, not a diagnosis, and not a prescription.".to_string()
}
