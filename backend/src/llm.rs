use crate::models::{ConsultationRequest, HerbSummary, TriageResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::{env, sync::Arc};

pub type DynGemmaProvider = Arc<dyn GemmaProvider + Send + Sync>;

#[async_trait]
pub trait GemmaProvider {
    async fn complete(
        &self,
        input: &ConsultationRequest,
        triage: &TriageResult,
        herbs: &[HerbSummary],
    ) -> Result<String>;
}

pub fn build_gemma_provider() -> Box<dyn GemmaProvider + Send + Sync> {
    match env::var("GEMMA_PROVIDER")
        .unwrap_or_else(|_| "mock".to_string())
        .as_str()
    {
        "http" => Box::new(HttpGemmaProvider {
            client: Client::new(),
            api_url: env::var("GEMMA_API_URL")
                .unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string()),
            api_key: env::var("GEMMA_API_KEY").ok(),
            model: env::var("GEMMA_MODEL").unwrap_or_else(|_| "gemma4".to_string()),
        }),
        _ => Box::new(MockGemmaProvider),
    }
}

pub struct MockGemmaProvider;

#[async_trait]
impl GemmaProvider for MockGemmaProvider {
    async fn complete(
        &self,
        input: &ConsultationRequest,
        triage: &TriageResult,
        herbs: &[HerbSummary],
    ) -> Result<String> {
        if triage.risk_level == "emergency" {
            return Ok(format!(
                "Safety check: your symptoms include red flags. Please seek emergency care now. I will not suggest herbs for this situation because home remedies could delay life-saving care.\n\nCare access near you: look for the nearest clinic, hospital, pharmacy, community health worker, NGO field worker, or trusted transport helper in or near {}. If you can call first, ask where emergency care is available now.\n\nBring or show this short referral summary: symptoms: {}; duration: {} day(s) if known; age group: {}; medicines: {}; allergies: {}; pregnancy: {}.\n\nThis is not medical advice or a diagnosis.",
                location_phrase(input),
                input.symptoms,
                input
                    .duration_days
                    .map(|days| days.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                input.age_group,
                list_or_none(&input.current_medicines),
                list_or_none(&input.allergies),
                if input.pregnant { "yes" } else { "no" }
            ));
        }

        if triage.risk_level == "urgent" {
            if input.symptoms.to_lowercase().contains("malaria")
                || input
                    .known_conditions
                    .iter()
                    .any(|condition| condition.to_lowercase().contains("malaria"))
            {
                let access_note = if input.care_accessible {
                    "Because you said care is reachable, use any herbal information only as background. Please prioritize a malaria rapid test or blood test and age/weight-appropriate antimalarial medicine from the nearest clinic, pharmacy, or community health worker."
                } else {
                    "Because you said care is not reachable right now, this is harm-reduction guidance for the gap before care. It carries risk. Keep trying to reach a clinic, pharmacy, community health worker, emergency transport, or someone who can obtain a malaria rapid test and proper antimalarial medicine."
                };
                return Ok(format!("Safety check: suspected malaria can become dangerous quickly, especially in children. {access_note}\n\nCare access near you: search or ask for the nearest clinic, pharmacy, community health worker, malaria testing point, NGO field worker, or trusted transport helper in or near {}. Bring or show this summary: fever/chills after mosquito bites or suspected malaria; duration: {} day(s) if known; age group: {}; medicines: {}; allergies: {}; pregnancy: {}.\n\nEducational note: quinine was originally derived from Cinchona bark, but raw bark or self-sourced quinine is not safe for treating malaria. The dose can be wrong, side effects can be serious, and malaria treatment depends on the parasite type, resistance patterns, age, weight, and pregnancy status.\n\nWhile getting help, keep the child drinking safe fluids if they can drink, use fever comfort measures, avoid more mosquito bites, and watch for danger signs: confusion, seizures, difficulty breathing, extreme weakness, repeated vomiting, inability to drink, or very dark urine. Those signs need emergency care now.\n\nThis is not medical advice or a diagnosis.",
                    location_phrase(input),
                    input
                        .duration_days
                        .map(|days| days.to_string())
                        .unwrap_or_else(|| "unknown".to_string()),
                    input.age_group,
                    list_or_none(&input.current_medicines),
                    list_or_none(&input.allergies),
                    if input.pregnant { "yes" } else { "no" }
                ));
            }
            return Ok(format!(
                "Safety check: this sounds like a serious health concern or a request for a cure. Herbs should not be used to replace clinical care, cancer treatment, antibiotics, insulin, antiretroviral therapy, anticoagulants, chemotherapy, or emergency care.\n\nCare access near you: contact the nearest clinic, hospital, pharmacy, community health worker, NGO field worker, or referral contact in or near {}. Bring or show this summary: symptoms: {}; duration: {} day(s) if known; age group: {}; medicines: {}; allergies: {}; pregnancy: {}.\n\nI can only discuss general comfort support and safety questions to bring to a clinician. Please contact a qualified health worker as soon as possible.\n\nThis is not medical advice or a diagnosis.",
                location_phrase(input),
                input.symptoms,
                input
                    .duration_days
                    .map(|days| days.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                input.age_group,
                list_or_none(&input.current_medicines),
                list_or_none(&input.allergies),
                if input.pregnant { "yes" } else { "no" }
            ));
        }

        let care_access_note = if input.care_accessible {
            "Care is reachable. Herbs are only extra support. Use clinic, pharmacy, or health worker first."
        } else {
            "Care is not reachable now. These steps are only for the waiting time. There is risk. Keep trying to reach a clinic, pharmacy, health worker, or trusted helper."
        };
        let mut response = format!(
            "Safety check: I do not see emergency danger signs in this text: {}. I may be wrong. This is not a diagnosis.\n\n{}\n\nWhat may help a little:\n",
            input.symptoms,
            care_access_note
        );
        let symptom_lc = input.symptoms.to_lowercase();
        if symptom_lc.contains("diarrhea") || symptom_lc.contains("loose stool") {
            response.push_str("\nMost important: ORS is more important than herbs. If you have no ORS packet, mix 1 liter safe water + 6 level teaspoons sugar + 1/2 level teaspoon salt. Give small sips again and again.\n");
        }
        if symptom_lc.contains("water") {
            response.push_str("\nSafe water: let dirty water sit. Pour the clearer top water through clean cloth. Boil hard for 1 minute. Cool with a cover on. Do not drink water with fuel, pesticide, or chemical smell.\n");
        }
        if symptom_lc.contains("food")
            || symptom_lc.contains("sweet potato")
            || symptom_lc.contains("chicken")
        {
            response.push_str("\nFood plan: start small. Try sweet potato vines or moringa first. Chickens help only if you have water, shade, safety from theft/animals, and feed scraps.\n");
        }
        for herb in herbs {
            response.push_str(&format!(
                "- {} ({}): support only. Not a cure. Proof level: {}. Be careful: {} Do not use if: {} Medicine clash: {} Safe use: {}\n",
                herb.common_name,
                herb.latin_name,
                herb.evidence_level,
                herb.safety_summary,
                list_or_none(&herb.contraindications),
                list_or_none(&herb.interactions),
                if herb.use_guidance.is_empty() {
                    herb.preparation_note.as_str()
                } else {
                    herb.use_guidance.as_str()
                }
            ));
        }
        response.push_str("\nAvoid using herbs as a replacement for prescribed medicines. Stop and seek care if symptoms worsen, fever becomes high or lasts more than 3 days, breathing becomes difficult, there is blood in stool or vomit, severe pain occurs, or you feel confused or faint.\n\nReminder: Gemma HerbalCare is educational only, not medical advice, not a diagnosis, and not a prescription.");
        Ok(response)
    }
}

pub struct HttpGemmaProvider {
    client: Client,
    api_url: String,
    api_key: Option<String>,
    model: String,
}

#[async_trait]
impl GemmaProvider for HttpGemmaProvider {
    async fn complete(
        &self,
        input: &ConsultationRequest,
        triage: &TriageResult,
        herbs: &[HerbSummary],
    ) -> Result<String> {
        anyhow::ensure!(
            !self.api_url.is_empty(),
            "GEMMA_API_URL is required for GEMMA_PROVIDER=http"
        );
        let prompt = build_prompt(input, triage, herbs)?;
        let mut request = self.client.post(&self.api_url).json(&json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false
        }));
        if let Some(key) = &self.api_key {
            request = request.bearer_auth(key);
        }
        let value: serde_json::Value = request
            .send()
            .await
            .context("Gemma endpoint request failed")?
            .error_for_status()
            .context("Gemma endpoint returned an error status")?
            .json()
            .await
            .context("Gemma endpoint did not return JSON")?;

        Ok(value
            .get("text")
            .or_else(|| value.get("response"))
            .or_else(|| value.pointer("/choices/0/text"))
            .and_then(|v| v.as_str())
            .unwrap_or("The Gemma endpoint returned no text.")
            .to_string())
    }
}

pub fn build_prompt(
    input: &ConsultationRequest,
    triage: &TriageResult,
    herbs: &[HerbSummary],
) -> Result<String> {
    Ok(format!(
        r#"SYSTEM:
You are Gemma HerbalCare, a safety-first herbal knowledge navigator. You are not a doctor. You do not diagnose or prescribe. You help users understand traditional herbal knowledge, safety risks, and when to seek professional care. You must only use the retrieved herb records. Do not invent herbs, cures, dosages, or medical claims.

SAFETY POLICY:
- If red_flags are present, do not suggest herbs.
- Serious diseases require professional medical care.
- Never claim herbs cure cancer, HIV/AIDS, tuberculosis, malaria, stroke, heart attack, or severe infection.
- Never advise stopping prescribed medication.
- Always include safety warnings, contraindications, and evidence level.
- If identification features or location availability are provided, explain that they are only aids and that unknown wild plants should be confirmed by a trusted local health worker, experienced grower, or safe market source before use.
- For suspected malaria, you may mention that quinine originally came from Cinchona bark as historical medicine-origin context, but never advise the user to find, boil, chew, dose, buy OTC, or use Cinchona bark or self-sourced quinine. Say malaria needs prompt testing and appropriate antimalarial medicine from a clinic, pharmacy, or community health worker.
- The user context includes care_accessible. If true, frame herbs as reference/supportive background only. If false, provide a short "if care is not reachable right now" harm-reduction section, state that this carries risk, and keep urging any route to clinic, pharmacy, community health worker, or proven medicine.
- For urgent or emergency cases, include a "care access near you" suggestion using the user's city, region, and country. Do not invent a hospital name or exact address unless retrieved facility data is provided. Suggest nearest clinic, hospital, pharmacy, community health worker, NGO field worker, emergency contact, referral directory, or trusted transport helper.
- For urgent or emergency cases, include a short referral summary the user can show to a health worker: symptoms, duration, age group, medicines, allergies, pregnancy status, and red flags.
- For diarrhea, emphasize ORS above herbs. If no ORS packet is available, say: 1 liter safe water + 6 level teaspoons sugar + 1/2 level teaspoon salt. Warn that too much salt can be dangerous.
- For water safety questions, explain basic steps: settle cloudy water, filter through clean cloth, boil at a rolling boil for 1 minute, cool covered. Warn that boiling/filtering does not remove chemical contamination such as fuel, pesticides, or heavy metals.
- For long-term food questions, give simple local resilience guidance such as starting small with sweet potato, moringa, and chickens only where water, shade, protection, and feed are realistic.
- If a retrieved herb has use_guidance, include a "how to use safely" note. Do not provide precise medicinal dosing for children or serious disease; prefer food-level or traditional-support wording from the retrieved record.
- Write for a very low-literacy reader. Use short sentences. Use simple words. Explain like: "Do this. Why. Danger sign." Avoid jargon. If you must use a hard word, explain it in plain words.
- Prefer cautious language: "may help comfort" or "support only" rather than "treats" or "cures".

USER CONTEXT:
{}

TRIAGE RESULT:
{}

RETRIEVED HERB RECORDS:
{}

TASK:
Write a clear, compassionate response for a low-resource user. Use simple language. Structure:
1. Safety check
2. Care access plan
3. Do this first
4. Support-only local herbs or food plants
5. What to avoid
6. Danger signs
7. Reminder that this is not medical advice
"#,
        serde_json::to_string_pretty(input)?,
        serde_json::to_string_pretty(triage)?,
        serde_json::to_string_pretty(herbs)?
    ))
}

fn list_or_none(items: &[String]) -> String {
    if items.is_empty() {
        "none listed".to_string()
    } else {
        items.join("; ")
    }
}

fn location_phrase(input: &ConsultationRequest) -> String {
    let location_parts = [
        input.city.as_deref(),
        input.region.as_deref(),
        Some(input.country.as_str()),
    ]
    .into_iter()
    .flatten()
    .filter(|part| !part.trim().is_empty())
    .collect::<Vec<_>>();

    if location_parts.is_empty() {
        "your current area".to_string()
    } else {
        location_parts.join(", ")
    }
}
