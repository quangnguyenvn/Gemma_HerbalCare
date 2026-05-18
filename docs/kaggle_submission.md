# Kaggle Submission Pack

Competition: The Gemma 4 Good Hackathon

## Form Fields

Title:

Gemma HerbalCare: Safety-first Herbal Knowledge Navigator

Subtitle:

A Gemma 4 powered prototype that helps low-resource communities explore local herbal knowledge without delaying urgent medical care.

Suggested tracks:

- Main Track
- Impact Track: Health and Sciences
- Impact Track: Safety and Trust
- Optional Special Technology Track: Ollama, only if the submission emphasizes the local Gemma/Ollama HTTP provider

Thumbnail:

`assets/gemma_herbalcare_thumbnail.jpg`

Video demo:

`assets/Gemma_herbalCare_submission_3min.mp4`

Public video URL after push:

https://raw.githubusercontent.com/quangnguyenvn/Gemma_HerbalCare/main/assets/Gemma_herbalCare_submission_3min.mp4

Public code repo:

https://github.com/quangnguyenvn/Gemma_HerbalCare

Functional prototype / demo instructions:

https://github.com/quangnguyenvn/Gemma_HerbalCare#run-locally

Project description, kept under 250 words:

Gemma HerbalCare is a safety-first herbal knowledge navigator for low-resource communities. In many places, families use local herbs before they can reach a clinic, but generic AI can confidently invent cures, doses, or false reassurance. This prototype changes the order of the conversation: check danger first, retrieve only curated local records second, then ask Gemma 4 to explain those records in plain, cautious language.

The app detects red flags such as chest pain, difficulty breathing, pregnancy bleeding, severe fever, suspected malaria, cancer cure requests, HIV/AIDS without care, and other serious conditions. In those cases it suppresses herbs and gives urgent care guidance. For lower-risk situations, it retrieves local herb and food-plant records with evidence levels, contraindications, interaction warnings, source URLs, and preparation notes. Gemma 4 acts as a community health translator, not as the source of truth.

The prototype includes a Rust/Axum backend, SQLite retrieval, safety triage, consultation logging, a SvelteKit frontend, demo cases, and a Gemma-compatible HTTP provider with Ollama-style local inference support. It is educational only, not medical advice, but it demonstrates a practical path toward safer local health navigation with clinician-reviewed data and community health partners.

## Writeup Body

### The Problem

In many low-resource communities, herbal medicine is not a wellness trend. It is often what people have nearby while clinics are far away, transport is expensive, or health information is unreliable.

That reality deserves respect, but it also needs guardrails. A helpful system can support hydration, comfort, nutrition, and earlier care-seeking. A careless chatbot can delay treatment for malaria, sepsis, pregnancy complications, heart attack, severe dehydration, cancer, or dangerous medicine interactions.

Gemma HerbalCare was built around one product principle:

The safest herbal recommendation is sometimes no herbal recommendation.

### What We Built

Gemma HerbalCare is a working web prototype that helps users ask about symptoms, location, care access, medicines, pregnancy, allergies, and local herbal options. It returns either:

- urgent care guidance with no herbs, when safety red flags are detected
- a serious-condition boundary response, when the user asks for cures or describes high-risk disease
- a retrieval-grounded educational response, when the situation is low-risk enough for support-only local herbal knowledge

The app is not a doctor, not a diagnosis tool, and not a prescription engine. It is a safer bridge between "I feel sick" and "I can reach care."

### Why Gemma 4 Matters Here

Gemma 4 is used as a careful community health translator. It does not decide whether herbs are safe for an emergency. The backend decides that first.

The model receives:

- the user context
- the triage result
- retrieved herb records, if herbs are allowed

The prompt instructs Gemma to explain only from those records. It must not invent herbs, cures, dosages, or medical claims. It must include warnings, contraindications, evidence levels, and care-seeking advice.

This makes the project more than a chatbot wrapper. The safety-critical decisions happen before generation.

### Architecture

The pipeline is intentionally small and auditable:

1. User submits symptoms, location, age group, pregnancy status, medicines, allergies, symptom duration, and whether care is reachable.
2. The Rust/Axum backend runs deterministic safety triage.
3. Emergency or urgent cases suppress herb retrieval.
4. Low and caution cases retrieve curated SQLite herb records by location and symptom.
5. Records are ranked by local availability, evidence, and safety.
6. A Gemma-compatible provider generates a plain-language response.
7. The consultation is logged in SQLite for review and evaluation.

Core files:

- `backend/src/safety.rs`: red flags, serious-condition boundaries, triage
- `backend/src/db.rs`: SQLite schema, seed data, herb retrieval, consultation logging
- `backend/src/llm.rs`: Gemma provider trait, mock provider, HTTP provider, prompt
- `backend/src/routes.rs`: API handlers
- `frontend/src/routes`: SvelteKit interface and demo cases

### Safety Design

Gemma HerbalCare suppresses herb recommendations for emergency or serious conditions, including:

- chest pain or difficulty breathing
- pregnancy bleeding or severe pain
- fever above 39.5C or fever lasting more than 3 days
- suspected malaria
- cancer cure requests
- HIV/AIDS, tuberculosis, stroke, heart attack, sepsis, kidney failure, liver failure, and uncontrolled diabetes

It also refuses to advise stopping or replacing prescribed medicines such as antibiotics, insulin, antiretroviral therapy, chemotherapy, anticoagulants, or emergency care.

For diarrhea, it emphasizes ORS before herbs. For unsafe water, it explains settle, cloth-filter, boil, and covered cooling steps while warning that boiling does not remove chemical contamination. For long-term food resilience, it can discuss local food plants such as sweet potato and moringa as food-support options.

### Demo Scenarios

The prototype includes demo cases designed to test both helpfulness and refusal behavior:

- Child diarrhea after unsafe water: ORS first, safe-water guidance, support-only local plants.
- Fever and suspected malaria: urgent testing and antimalarial care, no raw Cinchona or self-dosing.
- Possible worms in a child: caution-level support and care-seeking guidance.
- Cloudy well water: practical household safety steps.
- Long-term food source plan: local food resilience guidance.
- Poor breathing after indoor smoke: emergency red flag, no herbs shown.

### Technical Execution

Backend:

- Rust
- Axum
- Tokio
- SQLx with SQLite
- Serde
- reqwest

Frontend:

- SvelteKit
- TypeScript
- responsive CSS

Model layer:

- mock Gemma provider by default for zero-key judging
- HTTP Gemma-compatible provider via environment variables
- default local endpoint compatible with Ollama-style `/api/generate`

The app can run locally with:

```bash
cd backend
cargo run
```

```bash
cd frontend
npm install
npm run dev
```

Then open:

```text
http://localhost:5173
```

To use an HTTP Gemma endpoint:

```bash
cd backend
GEMMA_PROVIDER=http GEMMA_MODEL=gemma4 cargo run
```

### Validation

Before submission:

- `cargo test` passes for the backend.
- `npm run check` passes for the frontend with 0 errors and 0 warnings.
- The public GitHub repository contains source code, documentation, demo script, safety policy, architecture notes, and video assets.

### Impact

The target user is someone making health decisions before professional care is reachable. That is a dangerous moment for AI. The most valuable behavior is not always generating more information. Sometimes it is recognizing danger early, refusing unsafe cure claims, and explaining the next step in simple language.

With clinician-reviewed datasets, multilingual voice flows, and local health worker partnerships, this architecture could support safer community health navigation across regions where traditional medicine and limited care access coexist.

### Limitations and Future Work

This is an educational hackathon prototype. Real deployment would require:

- clinician review of regional herb datasets
- local regulatory and public health review
- multilingual and low-literacy validation
- stronger medicine interaction checks
- offline deployment bundles for clinics and community health workers
- evaluation tests for refusal accuracy, retrieval grounding, and response quality

### Links

Code:

https://github.com/quangnguyenvn/Gemma_HerbalCare

Demo video:

https://raw.githubusercontent.com/quangnguyenvn/Gemma_HerbalCare/main/assets/Gemma_herbalCare_submission_3min.mp4

Run locally:

https://github.com/quangnguyenvn/Gemma_HerbalCare#run-locally

Architecture:

https://github.com/quangnguyenvn/Gemma_HerbalCare/blob/main/docs/architecture.md

Safety policy:

https://github.com/quangnguyenvn/Gemma_HerbalCare/blob/main/docs/safety_policy.md

Demo script:

https://github.com/quangnguyenvn/Gemma_HerbalCare/blob/main/docs/demo_script.md
