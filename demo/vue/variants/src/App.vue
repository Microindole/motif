<template>
  <main class="shell">
    <section class="m-surface f-stack dark:m-elevation-1">
      <h1 class="f-text-primary">motif Vue variants</h1>
      <p>This scenario keeps focus, hover, active and dark classes in one place.</p>
      <button class="focus:m-ring hover:f-bg-hover-primary active:m-shadow-2">Vue variants</button>
    </section>
  </main>
</template>

<style>
.shell {
  min-height: 100vh;
  display: grid;
  place-items: center;
  background: linear-gradient(180deg, #eef5ff 0%, #ffffff 100%);
}
</style>

