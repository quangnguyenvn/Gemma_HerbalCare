<script lang="ts">
  import { onMount } from 'svelte';
  import { consult, demoCases, type ConsultResponse, type ConsultationRequest, type DemoCase } from '$lib/api';

  type VisualGuide = {
    key: string;
    title: string;
    caption: string;
  };

  let form: ConsultationRequest = {
    country: 'India',
    region: 'Bihar',
    city: 'Gaya',
    symptoms: 'mild cough, sore throat, runny nose, temperature 37.8C',
    age_group: 'adult',
    care_accessible: false,
    pregnant: false,
    known_conditions: [],
    current_medicines: [],
    allergies: [],
    duration_days: 1
  };
  let result: ConsultResponse | null = null;
  let cases: DemoCase[] = [];
  let loading = false;
  let error = '';

  onMount(() => {
    demoCases().then((items) => (cases = items)).catch(() => (cases = []));
  });

  const splitList = (value: string) =>
    value
      .split(',')
      .map((item) => item.trim())
      .filter(Boolean);

  let knownConditionsText = '';
  let medicinesText = '';
  let allergiesText = '';
  $: responseBlocks = result ? buildResponseBlocks(result.assistant_response) : [];
  $: symptomsLower = form.symptoms.toLowerCase();
  $: hasDiarrhea = symptomsLower.includes('diarrhea') || symptomsLower.includes('loose stool');
  $: asksWater = symptomsLower.includes('water');
  $: asksFood = symptomsLower.includes('food') || symptomsLower.includes('sweet potato') || symptomsLower.includes('chicken');
  $: hasBreathingDanger =
    symptomsLower.includes('difficulty breathing') ||
    symptomsLower.includes('fast breathing') ||
    symptomsLower.includes('poor breathing') ||
    symptomsLower.includes('indoor cooking smoke');

  async function submit() {
    loading = true;
    error = '';
    result = null;
    try {
      result = await consult({
        ...form,
        known_conditions: splitList(knownConditionsText),
        current_medicines: splitList(medicinesText),
        allergies: splitList(allergiesText),
        duration_days: Number(form.duration_days ?? 0)
      });
    } catch (err) {
      error = err instanceof Error ? err.message : 'Consultation failed';
    } finally {
      loading = false;
    }
  }

  function loadCase(item: DemoCase) {
    form = { ...item.request };
    knownConditionsText = item.request.known_conditions.join(', ');
    medicinesText = item.request.current_medicines.join(', ');
    allergiesText = item.request.allergies.join(', ');
    result = null;
  }

  const visualCatalog: Record<string, VisualGuide> = {
    sweet_potato: {
      key: 'sweet_potato',
      title: 'Start with a small plot',
      caption: 'Try sweet potato vines in one small bed before expanding.'
    },
    moringa: {
      key: 'moringa',
      title: 'Add familiar leafy food',
      caption: 'Moringa can support meals where people already know the plant.'
    },
    chicken_coop: {
      key: 'chicken_coop',
      title: 'Keep chickens only if ready',
      caption: 'Water, shade, feed scraps, and protection must come first.'
    },
    ors: {
      key: 'ors',
      title: 'Small sips again and again',
      caption: 'ORS and safe fluids come before herbs for diarrhea.'
    },
    safe_water: {
      key: 'safe_water',
      title: 'Settle, filter, then boil',
      caption: 'Use clean cloth and a covered container after boiling.'
    },
    fresh_air: {
      key: 'fresh_air',
      title: 'Move away from smoke',
      caption: 'Breathing trouble needs fresh air and urgent help.'
    },
    clinic: {
      key: 'clinic',
      title: 'Go to proven care',
      caption: 'Testing, medicine, and emergency care should not wait for herbs.'
    },
    wash_hands: {
      key: 'wash_hands',
      title: 'Clean hands protect children',
      caption: 'Handwashing and sanitation help prevent repeated illness.'
    },
    plant_check: {
      key: 'plant_check',
      title: 'Confirm the plant first',
      caption: 'Use trusted local confirmation before preparing any herb.'
    }
  };

  function buildResponseBlocks(response: string) {
    return response
      .split(/\n{2,}/)
      .map((text) => text.trim())
      .filter(Boolean)
      .map((text) => ({
        text,
        visuals: visualsForText(text)
      }));
  }

  function visualsForText(text: string): VisualGuide[] {
    const lower = text.toLowerCase();
    const keys: string[] = [];
    const add = (key: string) => {
      if (!keys.includes(key)) keys.push(key);
    };

    if (lower.includes('sweet potato') || lower.includes('vines')) add('sweet_potato');
    if (lower.includes('moringa')) add('moringa');
    if (lower.includes('chicken') || lower.includes('birds') || lower.includes('eggs')) add('chicken_coop');
    if (lower.includes('ors') || lower.includes('hydration') || lower.includes('small sips')) add('ors');
    if (lower.includes('safe water') || lower.includes('boil') || lower.includes('clean cloth')) add('safe_water');
    if (lower.includes('breathing') || lower.includes('smoke') || lower.includes('fresh air')) add('fresh_air');
    if (lower.includes('urgent') || lower.includes('clinic') || lower.includes('testing') || lower.includes('emergency')) add('clinic');
    if (lower.includes('worms') || lower.includes('sanitation') || lower.includes('hand')) add('wash_hands');
    if (lower.includes('herb') || lower.includes('plant')) add('plant_check');

    return keys.slice(0, 3).map((key) => visualCatalog[key]);
  }
</script>

<main class="page two-column">
  <section>
    <p class="eyebrow">Consultation</p>
    <h1>Start with safety</h1>
    <div class="demo-strip">
      {#each cases as item}
        <button class="small-button" on:click={() => loadCase(item)}>{item.title}</button>
      {/each}
    </div>

    <form class="field-guide-form" on:submit|preventDefault={submit}>
      <div class="form-row">
        <label>Country <input bind:value={form.country} required /></label>
        <label>Region <input bind:value={form.region} /></label>
        <label>City <input bind:value={form.city} /></label>
      </div>
      <label>Symptoms or known concern <textarea bind:value={form.symptoms} rows="4" required></textarea></label>
      <div class="form-row">
        <label>
          Age group
          <select bind:value={form.age_group}>
            <option>adult</option>
            <option>child</option>
            <option>infant under 3 months</option>
            <option>older adult</option>
          </select>
        </label>
        <label>Duration days <input type="number" min="0" bind:value={form.duration_days} /></label>
        <label class="checkbox"><input type="checkbox" bind:checked={form.pregnant} /> Pregnant</label>
      </div>
      <label class="access-toggle">
        <input type="checkbox" bind:checked={form.care_accessible} />
        <span>
          Clinic, pharmacy, or community health worker reachable within 24 hours
          <small>{form.care_accessible ? 'Herbal guidance will be framed as reference only.' : 'The response will include temporary harm-reduction guidance and risk warnings.'}</small>
        </span>
      </label>
      <label>Known diseases <input bind:value={knownConditionsText} placeholder="comma separated" /></label>
      <label>Current medicines <input bind:value={medicinesText} placeholder="comma separated" /></label>
      <label>Allergies <input bind:value={allergiesText} placeholder="comma separated" /></label>
      <button class="button primary" disabled={loading}>{loading ? 'Checking...' : 'Run safety-first consult'}</button>
      {#if error}<p class="error">{error}</p>{/if}
    </form>
  </section>

  <aside class="result-panel">
    {#if result}
      <div class="seal {result.triage.risk_level}">{result.triage.risk_level}</div>
      <h2>Safety check</h2>
      <p>{result.triage.reason}</p>
      <p class="care-access-note">
        {form.care_accessible
          ? 'Care access: reachable. Treat herbal information as supportive reference; proven care comes first.'
          : 'Care access: not reachable right now. This guidance is for the gap before care and carries risk.'}
      </p>
      <section class="care-plan">
        <div class="badge-row">
          <span class="evidence-badge clinical">care access plan</span>
          {#if hasDiarrhea}<span class="evidence-badge lifesaving">life-saving: ORS</span>{/if}
          {#if result.local_herbs.length}<span class="evidence-badge supportive">supportive only: herb</span>{/if}
        </div>
        <h2>{form.care_accessible ? 'If care is reachable' : 'If care is not reachable right now'}</h2>
        {#if form.care_accessible}
          <p>Use this consultation as preparation for a clinic, pharmacy, or community health worker. Bring the symptoms, duration, age, medicines, allergies, and any herbs already used.</p>
        {:else}
          <p>Use this as temporary harm-reduction guidance while still trying to reach a clinic, pharmacy, community health worker, emergency transport, or someone who can get proven medicine.</p>
        {/if}
      </section>
      {#if hasDiarrhea}
        <section class="survival-card">
          <div class="badge-row"><span class="evidence-badge lifesaving">life-saving: ORS</span></div>
          <h2>Homemade ORS</h2>
          <p>Mix 1 liter of safe water with 6 level teaspoons of sugar and 1/2 level teaspoon of salt. Stir until dissolved. The taste should be no saltier than tears.</p>
          <p>Give small frequent sips. If the child vomits, wait a few minutes and try again slowly. Keep breastfeeding or giving simple food if the child can eat.</p>
          <p class="warning-line">Too much salt can be dangerous. If you cannot measure, look for ORS packets, pharmacy help, or a health worker.</p>
        </section>
      {/if}
      {#if asksWater}
        <section class="survival-card">
          <div class="badge-row"><span class="evidence-badge lifesaving">life-saving: safe water</span></div>
          <h2>Make water safer</h2>
          <p>Let cloudy water settle, pour the clearer water through a clean cloth, then boil it at a rolling boil for 1 minute and let it cool in a covered clean container.</p>
          <p class="warning-line">Boiling does not make water safe if it contains fuel, pesticides, heavy metals, or other chemicals.</p>
        </section>
      {/if}
      {#if asksFood}
        <section class="survival-card">
          <div class="badge-row"><span class="evidence-badge clinical">long-term resilience</span></div>
          <h2>Food source plan</h2>
          <p>Start with low-cost, familiar options: sweet potato vines for tubers and leaves, moringa as a leafy food, and chickens only if water, shade, protection, and feed scraps are realistic.</p>
          <p>Choose one small trial plot or a few birds first, then expand after the family sees what survives locally.</p>
        </section>
      {/if}
      {#if hasBreathingDanger}
        <section class="survival-card">
          <div class="badge-row"><span class="evidence-badge lifesaving">emergency: breathing</span></div>
          <h2>Breathing danger</h2>
          <p>Move the person away from smoke, dust, or cooking fire into fresh air if it is safe. Keep them sitting upright and loosen tight clothing.</p>
          <p class="warning-line">Difficulty breathing, blue lips, confusion, chest pain, or a child who is too weak to drink needs urgent help now. Do not delay care for herbs.</p>
        </section>
      {/if}
      {#if result.triage.red_flags.length}
        <h3>Red flags</h3>
        <ul>
          {#each result.triage.red_flags as flag}<li>{flag}</li>{/each}
        </ul>
      {/if}

      {#if result.local_herbs.length}
        <h2>Local herbs</h2>
        <div class="herb-stack">
          {#each result.local_herbs as herb}
            <article class="herb-card">
              {#if herb.image_url}
                <img class="herb-photo" src={herb.image_url} alt={`${herb.common_name} identification reference`} loading="lazy" />
              {/if}
              <div class="herb-card-header">
                <div>
                  <h3>{herb.common_name}</h3>
                  <p class="latin">{herb.latin_name}</p>
                </div>
                <span class="nearby-badge">near patient</span>
              </div>
              {#if herb.local_availability}
                <p class="availability-note">{herb.local_availability}</p>
              {/if}
              {#if herb.identification_features.length}
                <h4>How to recognize it</h4>
                <ul class="identification-list">
                  {#each herb.identification_features as feature}<li>{feature}</li>{/each}
                </ul>
              {/if}
              <p>{herb.why_relevant}</p>
              <p><strong>Evidence:</strong> {herb.evidence_level}</p>
              <p><strong>Safety:</strong> {herb.safety_summary}</p>
              <p><strong>Contraindications:</strong> {herb.contraindications.join('; ') || 'none listed'}</p>
              <p><strong>Interactions:</strong> {herb.interactions.join('; ') || 'none listed'}</p>
              <p><strong>Finding safely:</strong> {herb.preparation_note}</p>
              {#if herb.use_guidance}
                <p><strong>How to use safely:</strong> {herb.use_guidance}</p>
              {/if}
              {#if herb.image_source_url}
                <a class="source-link" href={herb.image_source_url} target="_blank" rel="noreferrer">Image reference</a>
              {/if}
            </article>
          {/each}
        </div>
        <p class="field-warning">
          Visual identification is only a guide. Do not forage or give a plant to a child unless a trusted local health worker, experienced grower, or market source confirms it.
        </p>
      {/if}

      <h2>Gemma response</h2>
      <div class="response-story">
        {#each responseBlocks as block}
          <section class="response-step">
            <p>{block.text}</p>
            {#if block.visuals.length}
              <div class="visual-guide-grid">
                {#each block.visuals as visual}
                  <article class="visual-guide-card">
                    <div class="visual-frame" aria-hidden="true">
                      <svg viewBox="0 0 120 86" role="img">
                        {#if visual.key === 'sweet_potato'}
                          <path d="M12 69 C34 58, 62 58, 108 69" />
                          <path d="M20 72 H104" />
                          <path d="M35 64 C42 55, 55 53, 66 61" />
                          <path d="M39 60 C35 50, 41 43, 51 44 C49 53, 47 58, 39 60 Z" />
                          <path d="M58 59 C57 47, 67 41, 78 46 C72 55, 67 59, 58 59 Z" />
                          <path d="M45 72 C42 78, 51 81, 58 76 C62 72, 54 68, 45 72 Z" />
                          <circle cx="23" cy="41" r="7" />
                          <path d="M23 48 V63 M15 55 H31 M19 68 H27" />
                        {:else if visual.key === 'moringa'}
                          <path d="M62 74 V24" />
                          <path d="M62 38 C46 30, 38 28, 27 31" />
                          <path d="M62 48 C80 39, 89 39, 100 43" />
                          <path d="M62 57 C47 52, 38 54, 28 61" />
                          <circle cx="28" cy="31" r="4" />
                          <circle cx="39" cy="29" r="4" />
                          <circle cx="49" cy="32" r="4" />
                          <circle cx="99" cy="43" r="4" />
                          <circle cx="88" cy="40" r="4" />
                          <circle cx="78" cy="42" r="4" />
                          <circle cx="28" cy="61" r="4" />
                          <circle cx="39" cy="55" r="4" />
                          <circle cx="50" cy="55" r="4" />
                          <path d="M45 74 H79" />
                        {:else if visual.key === 'chicken_coop'}
                          <path d="M18 70 H104 V37 L61 18 18 37 Z" />
                          <path d="M29 70 V43 H92 V70" />
                          <path d="M42 54 H78" />
                          <circle cx="45" cy="62" r="5" />
                          <circle cx="61" cy="62" r="5" />
                          <path d="M87 63 C82 57, 89 49, 97 53 C102 55, 101 64, 94 66" />
                          <path d="M96 53 L101 49" />
                          <circle cx="93" cy="55" r="1.5" />
                        {:else if visual.key === 'ors'}
                          <path d="M34 22 H75 L69 72 H40 Z" />
                          <path d="M39 42 H70" />
                          <path d="M80 31 H101 V70 H80 Z" />
                          <path d="M86 41 H95 M86 50 H95" />
                          <path d="M21 62 C28 51, 35 51, 42 62" />
                          <path d="M24 68 H39" />
                          <circle cx="55" cy="55" r="4" />
                        {:else if visual.key === 'safe_water'}
                          <path d="M19 59 C36 50, 53 68, 70 59 C83 52, 94 55, 105 61" />
                          <path d="M28 31 H58 L53 67 H33 Z" />
                          <path d="M31 46 H55" />
                          <path d="M68 23 H99 V67 H68 Z" />
                          <path d="M73 31 H94 M73 39 H94 M73 47 H94" />
                          <path d="M47 18 C47 18, 40 26, 47 31 C54 26, 47 18, 47 18 Z" />
                        {:else if visual.key === 'fresh_air'}
                          <circle cx="35" cy="36" r="8" />
                          <path d="M35 44 V67 M25 53 H45 M30 76 H40" />
                          <path d="M55 30 C72 22, 84 25, 100 18" />
                          <path d="M55 45 C72 37, 83 42, 101 34" />
                          <path d="M58 60 C72 55, 83 58, 99 52" />
                          <path d="M17 70 C22 64, 27 64, 32 70" />
                        {:else if visual.key === 'clinic'}
                          <path d="M24 70 H96 V34 H24 Z" />
                          <path d="M32 34 L60 16 L88 34" />
                          <path d="M56 44 H64 V53 H73 V61 H64 V70 H56 V61 H47 V53 H56 Z" />
                          <path d="M17 75 C32 66, 44 66, 56 75" />
                          <path d="M71 75 C81 68, 91 68, 103 75" />
                        {:else if visual.key === 'wash_hands'}
                          <path d="M30 48 C42 37, 50 37, 61 48" />
                          <path d="M34 51 L53 70" />
                          <path d="M55 51 L36 70" />
                          <path d="M68 44 H98" />
                          <path d="M73 36 H93" />
                          <circle cx="41" cy="28" r="4" />
                          <circle cx="57" cy="24" r="3" />
                          <circle cx="75" cy="25" r="4" />
                        {:else}
                          <circle cx="60" cy="45" r="18" />
                          <path d="M60 63 V74" />
                          <path d="M60 45 C50 35, 40 36, 34 44 C44 48, 54 48, 60 45 Z" />
                          <path d="M60 45 C70 35, 82 36, 88 44 C78 48, 68 48, 60 45 Z" />
                          <path d="M38 73 H82" />
                          <path d="M31 19 L38 26 M38 19 L31 26" />
                        {/if}
                      </svg>
                    </div>
                    <h3>{visual.title}</h3>
                    <p>{visual.caption}</p>
                  </article>
                {/each}
              </div>
            {/if}
          </section>
        {/each}
      </div>
      <p class="disclaimer">{result.disclaimer}</p>
    {:else}
      <div class="empty-state">
        <h2>Result appears here</h2>
        <p>The app checks red flags first, asks whether care is reachable, then frames local herbal guidance as reference or temporary harm-reduction support.</p>
      </div>
    {/if}
  </aside>
</main>
