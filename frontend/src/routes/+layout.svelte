<script lang="ts">
  import '../app.css';
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
    <nav aria-label="Primary navigation">
      <a href="/consult">Consult</a>
      <a href="/herbs">Herb library</a>
      <a href="/about">Safety</a>
    </nav>
  </header>
  <slot />
</div>

