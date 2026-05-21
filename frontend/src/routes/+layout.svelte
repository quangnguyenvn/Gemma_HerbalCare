<script lang="ts">
  import { onMount } from 'svelte';
  import { connectionState, startConnectionMonitor } from '$lib/connection';
  import { isLocale, languageOptions, locale, navCopy, type Locale } from '$lib/i18n';
  import '../app.css';

  $: t = navCopy[$locale];

  function setDocumentLanguage(nextLocale: Locale) {
    if (typeof document !== 'undefined') {
      document.documentElement.lang = nextLocale;
      localStorage.setItem('gemma-herbalcare-locale', nextLocale);
    }
  }

  onMount(() => {
    const saved = localStorage.getItem('gemma-herbalcare-locale');
    if (isLocale(saved)) {
      locale.set(saved);
      setDocumentLanguage(saved);
    } else {
      setDocumentLanguage($locale);
    }

    return startConnectionMonitor();
  });
</script>

<svelte:head>
  <title>Gemma HerbalCare</title>
  <meta
    name="description"
    content="Safety-first local herbal guidance for low-resource communities"
  />
</svelte:head>

<div class="shell">
  <header class="topbar">
    <a class="brand" href="/">
      <span class="brand-mark">GH</span>
      <span>Gemma HerbalCare</span>
    </a>
    <div class="topbar-actions">
      <nav aria-label={t.navLabel}>
        <a href="/">{t.home}</a>
        <a href="/consult">{t.consult}</a>
        <a href="/herbs">{t.herbs}</a>
        <a href="/about">{t.safety}</a>
      </nav>
      <label class="language-switcher">
        <span>{t.language}</span>
        <select
          value={$locale}
          on:change={(event) => {
            const nextLocale = event.currentTarget.value;
            if (isLocale(nextLocale)) {
              locale.set(nextLocale);
              setDocumentLanguage(nextLocale);
            }
          }}
        >
          {#each languageOptions as option}
            <option value={option.code}>{option.label}</option>
          {/each}
        </select>
      </label>
      <div class:low-connection={$connectionState.mode === 'low'} class="connection-pill">
        <strong>{$connectionState.label}</strong>
        <small>
          {$connectionState.detail}
          {#if $connectionState.latencyMs}
            ({$connectionState.latencyMs} ms)
          {/if}
        </small>
      </div>
    </div>
  </header>
  <slot />
</div>
