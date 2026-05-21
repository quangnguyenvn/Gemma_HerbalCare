<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';

  type Locale = 'en' | 'sw' | 'hi' | 'zh' | 'ko';

  const languageOptions: Array<{ code: Locale; label: string }> = [
    { code: 'en', label: 'English' },
    { code: 'sw', label: 'Swahili' },
    { code: 'hi', label: 'हिन्दी' },
    { code: 'zh', label: '中文' },
    { code: 'ko', label: '한국어' }
  ];

  const copy: Record<
    Locale,
    {
      navLabel: string;
      home: string;
      consult: string;
      herbs: string;
      safety: string;
      language: string;
    }
  > = {
    en: {
      navLabel: 'Primary navigation',
      home: 'Home',
      consult: 'Consult',
      herbs: 'Herb library',
      safety: 'Safety',
      language: 'Language'
    },
    sw: {
      navLabel: 'Urambazaji mkuu',
      home: 'Nyumbani',
      consult: 'Ushauri',
      herbs: 'Mimea tiba',
      safety: 'Usalama',
      language: 'Lugha'
    },
    hi: {
      navLabel: 'मुख्य नेविगेशन',
      home: 'होम',
      consult: 'सलाह',
      herbs: 'जड़ी-बूटी सूची',
      safety: 'सुरक्षा',
      language: 'भाषा'
    },
    zh: {
      navLabel: '主导航',
      home: '首页',
      consult: '咨询',
      herbs: '草药库',
      safety: '安全',
      language: '语言'
    },
    ko: {
      navLabel: '기본 탐색',
      home: '홈',
      consult: '상담',
      herbs: '허브 라이브러리',
      safety: '안전',
      language: '언어'
    }
  };

  let locale: Locale = 'en';
  $: t = copy[locale];

  function setDocumentLanguage(nextLocale: Locale) {
    if (typeof document !== 'undefined') {
      document.documentElement.lang = nextLocale;
      localStorage.setItem('gemma-herbalcare-locale', nextLocale);
    }
  }

  onMount(() => {
    const saved = localStorage.getItem('gemma-herbalcare-locale');
    if (saved && saved in copy) {
      locale = saved as Locale;
    }
    setDocumentLanguage(locale);
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
        <select bind:value={locale} on:change={() => setDocumentLanguage(locale)}>
          {#each languageOptions as option}
            <option value={option.code}>{option.label}</option>
          {/each}
        </select>
      </label>
    </div>
  </header>
  <slot />
</div>
