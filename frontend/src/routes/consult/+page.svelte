<script lang="ts">
  import { onMount } from 'svelte';
  import { consult, demoCases, type ConsultResponse, type ConsultationRequest, type DemoCase } from '$lib/api';
  import { isOfflineConnection } from '$lib/connection';

  type VisualGuide = {
    key: string;
    title: string;
    caption: string;
    imageSrc: string;
    imageAlt: string;
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
  let imageInput: HTMLInputElement;
  let selectedImageName = '';
  let selectedImageUrl = '';
  let speaking = false;
  let speechSupported = false;
  let offlineDraftNote = '';

  onMount(() => {
    demoCases().then((items) => (cases = items)).catch(() => (cases = []));
    speechSupported = typeof window !== 'undefined' && 'speechSynthesis' in window;

    return () => {
      if (selectedImageUrl) URL.revokeObjectURL(selectedImageUrl);
      if (speechSupported) window.speechSynthesis.cancel();
    };
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
    offlineDraftNote = '';
    result = null;
    const request = {
      ...form,
      known_conditions: splitList(knownConditionsText),
      current_medicines: splitList(medicinesText),
      allergies: splitList(allergiesText),
      duration_days: Number(form.duration_days ?? 0)
    };

    if (isOfflineConnection()) {
      saveOfflineDraft(request);
      offlineDraftNote =
        'Offline mode: consultation is saved on this device. Review danger signs below, keep trying to reach care, and run the consult again when the local backend or internet is reachable.';
      loading = false;
      return;
    }

    try {
      result = await consult(request);
    } catch (err) {
      saveOfflineDraft(request);
      error =
        'Consultation service is not reachable. I saved this consultation draft on this device; try again when the local backend or internet is reachable.';
    } finally {
      loading = false;
    }
  }

  function saveOfflineDraft(request: ConsultationRequest) {
    try {
      localStorage.setItem(
        'consult-offline-draft',
        JSON.stringify({
          saved_at: new Date().toISOString(),
          request
        })
      );
    } catch {
      // Ignore storage failures; the visible form still remains on screen.
    }
  }

  function loadCase(item: DemoCase) {
    form = { ...item.request };
    knownConditionsText = item.request.known_conditions.join(', ');
    medicinesText = item.request.current_medicines.join(', ');
    allergiesText = item.request.allergies.join(', ');
    result = null;
  }

  function triggerImageUpload() {
    imageInput?.click();
  }

  function handleImageUpload(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    const file = target.files?.[0];
    if (!file) return;
    if (selectedImageUrl) URL.revokeObjectURL(selectedImageUrl);
    selectedImageName = file.name;
    selectedImageUrl = URL.createObjectURL(file);
  }

  function clearImage() {
    if (selectedImageUrl) URL.revokeObjectURL(selectedImageUrl);
    selectedImageName = '';
    selectedImageUrl = '';
    if (imageInput) imageInput.value = '';
  }

  function resultSpeechText() {
    if (!result) return '';
    return [
      `Safety check. ${result.triage.reason}`,
      form.care_accessible
        ? 'Care is reachable. Use herbs only as extra support.'
        : 'Care is not reachable now. This guidance is only for the waiting time and has risk.',
      result.assistant_response,
      result.disclaimer
    ].join('\n\n');
  }

  function toggleReadAloud() {
    if (!speechSupported || !result) return;
    if (speaking) {
      window.speechSynthesis.cancel();
      speaking = false;
      return;
    }
    const utterance = new SpeechSynthesisUtterance(resultSpeechText());
    utterance.rate = 0.86;
    utterance.pitch = 0.95;
    utterance.onend = () => (speaking = false);
    utterance.onerror = () => (speaking = false);
    speaking = true;
    window.speechSynthesis.cancel();
    window.speechSynthesis.speak(utterance);
  }

  const visualCatalog: Record<string, VisualGuide> = {
    sweet_potato: {
      key: 'sweet_potato',
      title: 'Start with a small plot',
      caption: 'Try sweet potato vines in one small bed before expanding.',
      imageSrc: '/visual-guides/food-plan.svg',
      imageAlt: 'Farmer watering a small sweet potato plot near moringa and chickens'
    },
    moringa: {
      key: 'moringa',
      title: 'Add familiar leafy food',
      caption: 'Moringa can support meals where people already know the plant.',
      imageSrc: '/visual-guides/food-plan.svg',
      imageAlt: 'Moringa tree beside a small food plot and chicken coop'
    },
    chicken_coop: {
      key: 'chicken_coop',
      title: 'Keep chickens only if ready',
      caption: 'Water, shade, feed scraps, and protection must come first.',
      imageSrc: '/visual-guides/food-plan.svg',
      imageAlt: 'Simple protected chicken coop beside a food plot'
    },
    ors: {
      key: 'ors',
      title: 'Small sips again and again',
      caption: 'ORS and safe fluids come before herbs for diarrhea.',
      imageSrc: '/visual-guides/ors.svg',
      imageAlt: 'Caregiver giving ORS to a child'
    },
    safe_water: {
      key: 'safe_water',
      title: 'Settle, filter, then boil',
      caption: 'Use clean cloth and a covered container after boiling.',
      imageSrc: '/visual-guides/safe-water.svg',
      imageAlt: 'Water being filtered, boiled, and stored in a clean container'
    },
    fresh_air: {
      key: 'fresh_air',
      title: 'Move away from smoke',
      caption: 'Breathing trouble needs fresh air and urgent help.',
      imageSrc: '/visual-guides/breathing-care.svg',
      imageAlt: 'Caregiver moving a child away from smoke toward a clinic'
    },
    clinic: {
      key: 'clinic',
      title: 'Go to proven care',
      caption: 'Testing, medicine, and emergency care should not wait for herbs.',
      imageSrc: '/visual-guides/breathing-care.svg',
      imageAlt: 'Caregiver and child going toward a clinic building'
    },
    wash_hands: {
      key: 'wash_hands',
      title: 'Clean hands protect children',
      caption: 'Handwashing and sanitation help prevent repeated illness.',
      imageSrc: '/visual-guides/safe-water.svg',
      imageAlt: 'Clean water steps for safer household hygiene'
    },
    plant_check: {
      key: 'plant_check',
      title: 'Confirm the plant first',
      caption: 'Use trusted local confirmation before preparing any herb.',
      imageSrc: '/visual-guides/plant-check.svg',
      imageAlt: 'Community elder helping confirm a plant before use'
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
      <section class="accessibility-panel" aria-label="Accessibility and phase 2 tools">
        <div>
          <p class="panel-kicker">Accessibility</p>
          <h2>For users who cannot read or type</h2>
          <p>
            These tools are for low-literacy users, older adults, people with poor vision, or community workers helping someone else.
          </p>
        </div>
        <div class="assistive-tools">
          <button
            class="icon-button"
            type="button"
            on:click={triggerImageUpload}
            title="Phase 2 visual support: attach a plant, water, medicine label, rash, or wound photo for local OCR and visual triage support. This prototype previews the photo only."
            aria-label="Upload image for future visual support"
          >
            CAM
          </button>
          <button
            class="icon-button"
            type="button"
            title="Phase 2 voice input: local speech-to-text for users who cannot type. Planned for offline use with Gemma multimodal or a local speech model."
            aria-label="Voice input planned"
          >
            MIC
          </button>
          <div class="phase-note">
            <strong>Phase 2 roadmap</strong>
            <span>Camera and microphone support will use local OCR, local speech-to-text, and visual triage support. It will not diagnose from photos.</span>
          </div>
        </div>
        <input
          class="visually-hidden"
          type="file"
          accept="image/*"
          bind:this={imageInput}
          on:change={handleImageUpload}
        />
      </section>
      {#if selectedImageUrl}
        <figure class="image-preview">
          <img src={selectedImageUrl} alt="Uploaded visual support preview" />
          <button class="small-button" type="button" on:click={clearImage}>Remove image</button>
          <figcaption>
            <strong>{selectedImageName}</strong>
            <span>Preview only. Phase 2 will use local OCR and visual triage support, not diagnosis.</span>
          </figcaption>
        </figure>
      {/if}
      <button class="button primary" disabled={loading}>{loading ? 'Checking...' : 'Run safety-first consult'}</button>
      {#if error}<p class="error">{error}</p>{/if}
      {#if offlineDraftNote}<p class="offline-note">{offlineDraftNote}</p>{/if}
    </form>
  </section>

  <aside class="result-panel">
    {#if offlineDraftNote}
      <div class="seal urgent">offline</div>
      <h2>Offline safety fallback</h2>
      <p>
        This page can still help you prepare. It cannot complete a Gemma consult until the local backend or internet is reachable.
      </p>
      <section class="care-plan">
        <div class="badge-row"><span class="evidence-badge clinical">saved draft</span></div>
        <h2>What to do now</h2>
        <p>
          If there is chest pain, trouble breathing, severe dehydration, blood in stool or vomit, seizure, confusion,
          pregnancy bleeding, very high fever, or suspected malaria, seek the nearest clinic, pharmacy, community health
          worker, emergency transport, or trusted local helper now.
        </p>
        <p>
          Bring this summary: {form.city}, {form.region}, {form.country}; symptoms: {form.symptoms}; age group:
          {form.age_group}; duration: {form.duration_days ?? 'unknown'} day(s).
        </p>
      </section>
    {:else if result}
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
                <img class="herb-photo" src={herb.image_url} alt={`${herb.common_name} identification reference`} loading="eager" />
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
      <div class="response-tools" aria-label="Read-aloud response controls">
        <button
          class="icon-button"
          type="button"
          on:click={toggleReadAloud}
          disabled={!speechSupported}
          title="Read the response aloud for users with low literacy, poor vision, or tired eyes. Uses browser text-to-speech in this prototype."
          aria-label={speaking ? 'Stop reading response aloud' : 'Read response aloud'}
        >
          {speaking ? 'STOP' : 'VOL'}
        </button>
        <div class="phase-note">
          <strong>Read aloud</strong>
          <span>{speechSupported ? 'Works now with browser text-to-speech. Local-language voices are a phase 2 target.' : 'Read aloud is not supported by this browser.'}</span>
        </div>
      </div>
      <div class="response-story">
        {#each responseBlocks as block}
          <section class="response-step">
            <p>{block.text}</p>
            {#if block.visuals.length}
              <div class="visual-guide-grid">
                {#each block.visuals as visual}
                  <article class="visual-guide-card">
                    <div class="visual-frame">
                      <img src={visual.imageSrc} alt={visual.imageAlt} loading="lazy" />
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
