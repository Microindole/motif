<template>
  <main class="shell">
    <section class="f-surface f-stack">
      <span class="f-label">Windows workspace</span>
      <h2 class="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
      <p class="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
      <div class="f-divider"></div>
      <div class="f-panel f-stack">
        <label class="f-label row">
          Subject
          <input class="entry f-field focus:f-border-focus focus:f-ring" value="Quarterly sync notes" />
        </label>
        <label class="f-label row">
          Summary
          <textarea class="entry f-field focus:f-border-focus focus:f-ring" rows="4">Tighten the new preset rules and review the AI output guardrails.</textarea>
        </label>
        <div class="cluster">
          <button class="button-reset f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Preview panel</button>
          <button class="button-reset f-action-primary hover:f-bg-hover-primary active:f-shadow-press">Send update</button>
        </div>
      </div>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <span class="m-label">Google workspace</span>
      <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
      <p class="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
      <div class="m-divider"></div>
      <div class="m-surface-container f-stack">
        <label class="m-label row">
          Search
          <input class="entry m-field focus:m-border-focus focus:m-ring" value="Ship the preset charter and workspace demo" />
        </label>
        <label class="m-label row">
          Highlight
          <input class="entry m-field focus:m-border-focus focus:m-ring" value="Surface container carries secondary actions" />
        </label>
        <div class="cluster">
          <button class="button-reset m-action-outlined hover:m-bg-hover-surface active:m-shadow-press">Open filter</button>
          <button class="button-reset m-action-tonal hover:m-bg-hover-container active:m-shadow-press">Save draft</button>
          <button class="button-reset m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Create task</button>
        </div>
      </div>
    </section>
  </main>
</template>

<style>
.shell {
  min-height: 100vh;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 24px;
  padding: 32px;
  background: linear-gradient(180deg, #ecf2fb 0%, #f8fbff 100%);
}

.row {
  display: grid;
  gap: 10px;
}

.cluster {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.entry {
  width: 100%;
  box-sizing: border-box;
}

.button-reset {
  cursor: pointer;
}
</style>





