const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <main style="min-height: 100vh; display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 24px; padding: 32px; background: linear-gradient(180deg, #ecf2fb 0%, #f8fbff 100%);">
    <section class="f-surface f-stack">
      <span class="f-label">Windows workspace</span>
      <h2 class="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
      <p class="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
      <div class="f-divider"></div>
      <div class="f-panel f-stack">
        <label class="f-label" style="display:grid;gap:10px;">
          Subject
          <input class="f-field focus:f-border-focus focus:f-ring" value="Quarterly sync notes" />
        </label>
        <label class="f-label" style="display:grid;gap:10px;">
          Summary
          <textarea class="f-field focus:f-border-focus focus:f-ring" rows="4">Tighten the new preset rules and review the AI output guardrails.</textarea>
        </label>
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button style="cursor:pointer;" class="f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Preview panel</button>
          <button style="cursor:pointer;" class="f-action-primary hover:f-bg-hover-primary active:f-shadow-press">Send update</button>
        </div>
      </div>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <span class="m-label">Google workspace</span>
      <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
      <p class="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
      <div class="m-divider"></div>
      <div class="m-surface-container f-stack">
        <label class="m-label" style="display:grid;gap:10px;">
          Search
          <input class="m-field focus:m-border-focus focus:m-ring" value="Ship the preset charter and workspace demo" />
        </label>
        <label class="m-label" style="display:grid;gap:10px;">
          Highlight
          <input class="m-field focus:m-border-focus focus:m-ring" value="Surface container carries secondary actions" />
        </label>
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button style="cursor:pointer;" class="m-action-outlined hover:m-bg-hover-surface active:m-shadow-press">Open filter</button>
          <button style="cursor:pointer;" class="m-action-tonal hover:m-bg-hover-container active:m-shadow-press">Save draft</button>
          <button style="cursor:pointer;" class="m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Create task</button>
        </div>
      </div>
    </section>
  </main>
`;





