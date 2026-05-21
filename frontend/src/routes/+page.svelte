<script lang="ts">
  import { homeCopy, locale } from '$lib/i18n';

  $: t = homeCopy[$locale];
</script>

<main class="home">
  <section class="hero">
    <div class="hero-copy">
      <p class="eyebrow">{t.eyebrow}</p>
      <h1>Gemma HerbalCare</h1>
      <p class="subtitle">{t.subtitle}</p>
      <div class="hero-actions">
        <a class="button primary" href="/consult">{t.start}</a>
        <a class="button secondary" href="/about">{t.safetyPolicy}</a>
      </div>
    </div>
    <div class="botanical-plate" aria-hidden="true">
      <div class="stem"></div>
      <div class="leaf leaf-a"></div>
      <div class="leaf leaf-b"></div>
      <div class="leaf leaf-c"></div>
      <div class="flower"></div>
      <span>Zingiber officinale</span>
    </div>
  </section>

  <section class="manuscript-grid">
    <article>
      <h2>{t.triageTitle}</h2>
      <p>{t.triageBody}</p>
    </article>
    <article>
      <h2>{t.recordsTitle}</h2>
      <p>{t.recordsBody}</p>
    </article>
    <article>
      <h2>{t.gemmaTitle}</h2>
      <p>{t.gemmaBody}</p>
    </article>
  </section>

  <section class="mobile-app-panel">
    <div>
      <p class="eyebrow">{t.mobileEyebrow}</p>
      <h2>{t.mobileTitle}</h2>
      <p>{t.mobileBody}</p>
    </div>
    <div class="mobile-app-visual" aria-hidden="true">
      <div class="phone-illustration">
        <div class="phone-speaker"></div>
        <div class="phone-screen">
          <div class="phone-status"></div>
          <div class="phone-title">{t.phoneTitle}</div>
          <div class="phone-card">
            <span class="phone-leaf"></span>
            <div>
              <strong>Moringa</strong>
              <small>{t.phoneMoringaPack}</small>
            </div>
          </div>
          <div class="phone-card">
            <span class="phone-leaf alt"></span>
            <div>
              <strong>Ginger</strong>
              <small>{t.phoneGingerPack}</small>
            </div>
          </div>
          <div class="phone-chip">{t.phoneOffline}</div>
        </div>
        <div class="phone-button"></div>
      </div>
      <div class="mobile-feature-list">
        <p><strong>{t.offlineTitle}</strong> {t.offlineBody}</p>
        <p><strong>{t.updatesTitle}</strong> {t.updatesBody}</p>
        <p><strong>{t.fieldTitle}</strong> {t.fieldBody}</p>
      </div>
    </div>
  </section>
</main>
