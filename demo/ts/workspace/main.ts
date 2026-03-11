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
      <div class="f-surface-alt f-stack">
        <label class="f-label" style="display:grid;gap:10px;">
          Subject
          <input class="f-field focus:f-ring" value="Quarterly sync notes" />
        </label>
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button style="cursor:pointer;" class="f-action-primary hover:f-bg-primary">Send update</button>
          <button style="cursor:pointer;" class="f-surface-alt focus:f-ring">Preview</button>
        </div>
      </div>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <span class="m-label">Google workspace</span>
      <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
      <p class="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
      <div class="m-divider"></div>
      <div class="m-surface-variant f-stack">
        <label class="m-label" style="display:grid;gap:10px;">
          Search
          <input class="m-field" value="Ship the preset charter and workspace demo" />
        </label>
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button style="cursor:pointer;" class="m-action-tonal">Save draft</button>
          <button style="cursor:pointer;" class="m-action-primary active:m-shadow-2">Create task</button>
        </div>
      </div>
    </section>
  </main>
`;
