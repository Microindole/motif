<template>
  <main class="shell">
    <section class="f-surface f-stack">
      <h2 class="f-text-primary">Win11 preset</h2>
      <p>Soft Fluent-like surface and restrained accent.</p>
      <button class="focus:f-ring hover:f-bg-primary">Fluent button</button>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <h2 class="m-text-primary">Google preset</h2>
      <p>Material-like primary action with stronger emphasis and elevation.</p>
      <button class="m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
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
  background: linear-gradient(180deg, #e9eef7 0%, #ffffff 100%);
}
</style>
