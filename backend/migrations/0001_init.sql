-- The demo backend auto-runs this schema in Rust for hackathon simplicity.
-- This file mirrors the local SQLite data model for reviewers and future SQLx migrations.
CREATE TABLE IF NOT EXISTS herbs (
  id INTEGER PRIMARY KEY,
  common_name TEXT NOT NULL,
  latin_name TEXT NOT NULL,
  local_names_json TEXT NOT NULL,
  regions_json TEXT NOT NULL,
  plant_part TEXT NOT NULL,
  traditional_uses_json TEXT NOT NULL,
  symptom_tags_json TEXT NOT NULL,
  evidence_level TEXT NOT NULL,
  safety_summary TEXT NOT NULL,
  contraindications_json TEXT NOT NULL,
  interactions_json TEXT NOT NULL,
  preparation_note TEXT NOT NULL,
  source_url TEXT NOT NULL
);

