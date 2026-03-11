const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <div style="min-height: 100vh; display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 24px; padding: 32px; background: radial-gradient(circle at top, #eef3fb 0%, #ffffff 60%);">
    <section class="f-surface f-stack">
      <span class="f-text-muted f-body">Win11 preset</span>
      <h2 class="f-text-primary f-title">Mica-like surface, restrained corners</h2>
      <p class="f-text-muted f-body">Softer surface, clearer focus ring, and quieter desktop-oriented feel.</p>
      <button style="border:0;padding:0.85rem 1.25rem;cursor:pointer;" class="focus:f-ring hover:f-bg-primary">Fluent button</button>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <span class="m-text-muted m-body">Google preset</span>
      <h2 class="m-text-primary m-title">Bolder color roles, fuller rounded action</h2>
      <p class="m-text-muted m-body">Stronger brand button, more explicit hierarchy, more rounded action shape.</p>
      <button style="border:0;padding:0.85rem 1.25rem;cursor:pointer;" class="m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
    </section>
  </div>
`;
