const html = `
  <section class="f-surface f-stack">
    <span class="f-label">Windows workspace</span>
    <h2 class="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
    <div class="f-divider"></div>
    <input class="f-field focus:f-ring" value="Quarterly sync notes" />
    <button class="f-action-primary hover:f-bg-primary">Send update</button>
  </section>
  <section class="m-surface f-stack dark:m-elevation-1">
    <span class="m-label">Google workspace</span>
    <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
    <div class="m-divider"></div>
    <input class="m-field" value="Ship the preset charter and workspace demo" />
    <button class="m-action-tonal">Save draft</button>
    <button class="m-action-primary active:m-shadow-2">Create task</button>
  </section>
`;

console.log(html);
