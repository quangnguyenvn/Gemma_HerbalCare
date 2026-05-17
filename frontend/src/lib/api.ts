const API_BASE = import.meta.env.VITE_API_BASE ?? 'http://localhost:8080';

export type ConsultationRequest = {
  country: string;
  region?: string;
  city?: string;
  symptoms: string;
  age_group: string;
  care_accessible: boolean;
  pregnant: boolean;
  known_conditions: string[];
  current_medicines: string[];
  allergies: string[];
  duration_days?: number;
};

export type TriageResult = {
  risk_level: 'low' | 'caution' | 'urgent' | 'emergency';
  red_flags: string[];
  reason: string;
};

export type HerbSummary = {
  id: number;
  common_name: string;
  latin_name: string;
  why_relevant: string;
  evidence_level: string;
  safety_summary: string;
  contraindications: string[];
  interactions: string[];
  preparation_note: string;
  use_guidance: string;
  source_url: string;
  image_url: string;
  image_source_url: string;
  identification_features: string[];
  local_availability: string;
};

export type ConsultResponse = {
  consultation_id?: number;
  triage: TriageResult;
  local_herbs: HerbSummary[];
  assistant_response: string;
  disclaimer: string;
};

export type DemoCase = {
  title: string;
  request: ConsultationRequest;
};

export async function consult(request: ConsultationRequest): Promise<ConsultResponse> {
  const res = await fetch(`${API_BASE}/api/consult`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(request)
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export async function herbs(params: {
  country?: string;
  region?: string;
  symptom?: string;
}): Promise<HerbSummary[]> {
  const search = new URLSearchParams();
  Object.entries(params).forEach(([key, value]) => {
    if (value) search.set(key, value);
  });
  const res = await fetch(`${API_BASE}/api/herbs?${search}`);
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export async function demoCases(): Promise<DemoCase[]> {
  const res = await fetch(`${API_BASE}/api/demo-cases`);
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}
