export const API_BASE =
  import.meta.env.VITE_API_BASE ?? (import.meta.env.DEV ? 'http://localhost:8080' : '');

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

export const fallbackDemoCases: DemoCase[] = [
  {
    title: 'Child diarrhea after unsafe water',
    request: {
      country: 'Nigeria',
      region: 'Kano',
      city: 'Kano',
      symptoms:
        'child has mild diarrhea and loose stool after drinking untreated well water, still drinking, no visible blood, no dehydration signs',
      age_group: 'child',
      care_accessible: false,
      pregnant: false,
      known_conditions: [],
      current_medicines: [],
      allergies: [],
      duration_days: 1
    }
  },
  {
    title: 'Fever and suspected malaria',
    request: {
      country: 'Nigeria',
      region: 'Kano',
      city: 'Kano',
      symptoms: 'fever, chills, headache and suspected malaria after mosquito bites',
      age_group: 'child',
      care_accessible: false,
      pregnant: false,
      known_conditions: [],
      current_medicines: [],
      allergies: [],
      duration_days: 2
    }
  },
  {
    title: 'Possible worms in a child',
    request: {
      country: 'Nigeria',
      region: 'Kano',
      city: 'Kano',
      symptoms:
        'child has abdominal discomfort, poor appetite, itchy bottom at night, and possible worms in stool',
      age_group: 'child',
      care_accessible: true,
      pregnant: false,
      known_conditions: [],
      current_medicines: [],
      allergies: [],
      duration_days: 7
    }
  },
  {
    title: 'How can we make water safer?',
    request: {
      country: 'Nigeria',
      region: 'Kano',
      city: 'Kano',
      symptoms:
        'family only has cloudy well water and asks how to make drinking water safer with basic household tools',
      age_group: 'adult',
      care_accessible: false,
      pregnant: false,
      known_conditions: [],
      current_medicines: [],
      allergies: [],
      duration_days: 0
    }
  },
  {
    title: 'Long-term food source plan',
    request: {
      country: 'Nigeria',
      region: 'Kano',
      city: 'Kano',
      symptoms:
        'family asks for a simple long-term food source plan such as growing sweet potato, moringa, and raising chickens with limited money',
      age_group: 'adult',
      care_accessible: false,
      pregnant: false,
      known_conditions: [],
      current_medicines: [],
      allergies: [],
      duration_days: 0
    }
  },
  {
    title: 'Poor breathing after indoor smoke',
    request: {
      country: 'Nigeria',
      region: 'Kano',
      city: 'Kano',
      symptoms: 'child has difficulty breathing, fast breathing, and coughing after indoor cooking smoke',
      age_group: 'child',
      care_accessible: false,
      pregnant: false,
      known_conditions: [],
      current_medicines: [],
      allergies: [],
      duration_days: 0
    }
  }
];

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
  try {
    const res = await fetch(`${API_BASE}/api/demo-cases`);
    if (!res.ok) throw new Error(await res.text());
    return res.json();
  } catch {
    return fallbackDemoCases;
  }
}

export async function health(signal?: AbortSignal): Promise<void> {
  const res = await fetch(`${API_BASE}/health`, {
    cache: 'no-store',
    signal
  });
  if (!res.ok) throw new Error(await res.text());
}
