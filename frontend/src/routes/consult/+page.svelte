<script lang="ts">
  import { onMount } from 'svelte';
  import { consult, demoCases, type ConsultResponse, type ConsultationRequest, type DemoCase } from '$lib/api';

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
      <pre class="response-text">{result.assistant_response}</pre>
      <p class="disclaimer">{result.disclaimer}</p>
    {:else}
      <div class="empty-state">
        <h2>Result appears here</h2>
        <p>The app checks red flags first, asks whether care is reachable, then frames local herbal guidance as reference or temporary harm-reduction support.</p>
      </div>
    {/if}
  </aside>
</main>
