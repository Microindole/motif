<template>
  <main class="shell">
    <section class="f-surface f-stack">
      <span class="f-text-muted f-body">Win11 preset</span>
      <h2 class="f-text-primary f-title">Mica-like surface, restrained corners</h2>
      <p class="f-text-muted f-body">Softer surface, clearer focus ring, and quieter desktop-oriented feel.</p>
      <button class="hero-button focus:f-ring hover:f-bg-primary">Fluent button</button>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <span class="m-text-muted m-body">Google preset</span>
      <h2 class="m-text-primary m-title">Bolder color roles, fuller rounded action</h2>
      <p class="m-text-muted m-body">Stronger brand button, more explicit hierarchy, more rounded action shape.</p>
      <button class="hero-button m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
    </section>
  </main>
</template>

<style>
.shell {
  min-height: 100vh;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 24px;
  padding: 32px;
  background: radial-gradient(circle at top, #eef3fb 0%, #ffffff 60%);
}

.hero-button {
  border: 0;
  padding: 0.85rem 1.25rem;
  cursor: pointer;
}
</style>
