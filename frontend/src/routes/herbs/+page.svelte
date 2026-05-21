<script lang="ts">
  import { onMount } from 'svelte';
  import { herbs, type HerbSummary } from '$lib/api';
  import { isLowConnection } from '$lib/connection';

  let country = 'India';
  let region = 'Bihar';
  let symptom = '';
  let items: HerbSummary[] = [];
  let loading = false;
  let error = '';
  let cacheNote = '';

  function cacheKey() {
    return `herb-cache:${country.trim().toLowerCase()}:${region
      .trim()
      .toLowerCase()}:${symptom.trim().toLowerCase()}`;
  }

  function readCache(): HerbSummary[] | null {
    try {
      const raw = localStorage.getItem(cacheKey());
      if (!raw) return null;
      return JSON.parse(raw) as HerbSummary[];
    } catch {
      return null;
    }
  }

  function writeCache(nextItems: HerbSummary[]) {
    try {
      localStorage.setItem(cacheKey(), JSON.stringify(nextItems));
    } catch {
      // Storage can fail in private mode or when quota is full; live results still work.
    }
  }

  async function search() {
    loading = true;
    error = '';
    cacheNote = '';
    try {
      if (isLowConnection()) {
        const cached = readCache();
        if (cached?.length) {
          items = cached;
          cacheNote = 'Low connection mode: showing cached regional records.';
        }
      }

      const freshItems = await herbs({ country, region, symptom });
      items = freshItems;
      writeCache(freshItems);
      cacheNote = '';
    } catch (err) {
      const cached = readCache();
      if (cached?.length) {
        items = cached;
        cacheNote = 'Network request failed. Showing cached regional records from this device.';
      } else {
        error = err instanceof Error ? err.message : 'Search failed';
      }
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    void search();
  });
</script>

<main class="page">
  <p class="eyebrow">Herb library</p>
  <h1>Curated field-guide records</h1>
  <form class="library-filter" on:submit|preventDefault={search}>
    <label>Country <input bind:value={country} /></label>
    <label>Region <input bind:value={region} /></label>
    <label>Symptom <input bind:value={symptom} placeholder="optional filter" /></label>
    <button class="button primary" disabled={loading}>{loading ? 'Searching...' : 'Search'}</button>
  </form>
  {#if error}<p class="error">{error}</p>{/if}
  {#if cacheNote}<p class="cache-note">{cacheNote}</p>{/if}
  <section class="library-grid">
    {#each items as herb}
      <article class="herb-card botanical">
        {#if herb.image_url}
          <img
            class="record-guide-image"
            src={herb.image_url}
            alt={`${herb.common_name} identification reference`}
            loading="eager"
          />
        {:else}
          <div class="sprig" aria-hidden="true"></div>
        {/if}
        <h2>{herb.common_name}</h2>
        <p class="latin">{herb.latin_name}</p>
        <p>{herb.why_relevant}</p>
        <dl>
          <dt>Evidence</dt><dd>{herb.evidence_level}</dd>
          <dt>Safety</dt><dd>{herb.safety_summary}</dd>
          <dt>Contraindications</dt><dd>{herb.contraindications.join('; ') || 'none listed'}</dd>
          <dt>Interactions</dt><dd>{herb.interactions.join('; ') || 'none listed'}</dd>
        </dl>
        <a href={herb.source_url} target="_blank" rel="noreferrer">Source</a>
      </article>
    {/each}
  </section>
</main>
