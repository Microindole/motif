<template>
  <div class="f-surface f-stack">
    <span class="f-label">Windows workspace</span>
    <h2 class="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
    <p class="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
    <div class="f-divider"></div>
    <div class="f-panel f-stack">
      <label class="f-label">
        Subject
        <input class="f-field focus:f-ring" value="Quarterly sync notes" />
      </label>
      <button class="f-action-subtle focus:f-ring">Preview panel</button>
      <button class="f-action-primary hover:f-bg-primary">Send update</button>
    </div>
  </div>
  <div class="m-surface f-stack dark:m-elevation-1">
    <span class="m-label">Google workspace</span>
    <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
    <p class="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
    <div class="m-divider"></div>
    <div class="m-surface-container f-stack">
      <label class="m-label">
        Search
        <input class="m-field" value="Ship the preset charter and workspace demo" />
      </label>
      <button class="m-action-outlined">Open filter</button>
      <button class="m-action-tonal">Save draft</button>
      <button class="m-action-primary active:m-shadow-2">Create task</button>
    </div>
  </div>
</template>
