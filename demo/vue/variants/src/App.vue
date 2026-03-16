<template>
  <main class="shell">
    <section class="m-surface f-stack dark:m-elevation-1 ui-pad-xl ui-radius-lg ui-density-comfortable">
      <h1 class="f-text-primary">motif Vue variants</h1>
      <p class="ui-text-xs">
        This scenario keeps Material styling intact while letting size and radius
        shift through the neutral <code>ui-*</code> layer.
      </p>
      <button class="focus:m-ring hover:m-bg-hover-primary active:m-shadow-press ui-control-lg ui-radius-pill ui-density-compact">
        Vue variants
      </button>
      <div class="ui-stack-inline ui-gap-xl">
        <input type="checkbox" class="f-checkbox" aria-label="Fluent checkbox" />
        <input type="radio" class="m-radio" aria-label="Material radio" />
        <button class="m-switch" aria-label="Material switch"></button>
      </div>
      <div class="ui-stack-center ui-text-xs">Preset + ui layer</div>
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



