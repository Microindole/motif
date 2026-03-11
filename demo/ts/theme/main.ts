const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <div style="min-height: 100vh; display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 24px; padding: 32px; background: radial-gradient(circle at top, #dfe7f3 0%, #ffffff 60%);">
    <section class="f-surface f-stack">
      <span class="f-text-muted f-body">Win11 preset</span>
      <h2 class="f-text-primary f-title">Mica shell with acrylic inner layer</h2>
      <p class="f-text-muted f-body">The outer surface stays soft and quiet, while the inner layer feels more like a lifted acrylic pane.</p>
      <div class="f-surface-alt f-stack">
        <p class="f-text-muted f-body">Acrylic-like panel</p>
        <button style="border:0;padding:0.85rem 1.25rem;cursor:pointer;" class="focus:f-ring hover:f-bg-primary">Fluent button</button>
      </div>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <span class="m-text-muted m-body">Google preset</span>
      <h2 class="m-text-primary m-title">Surface plus clear primary container</h2>
      <p class="m-text-muted m-body">The main card stays neutral while the inner container and primary button make hierarchy feel more explicit.</p>
      <div class="m-surface-variant f-stack">
        <p class="m-text-muted m-body">Surface container</p>
        <button style="border:0;padding:0.85rem 1.25rem;cursor:pointer;" class="m-bg-primary-container">Primary container</button>
        <button style="border:0;padding:0.85rem 1.25rem;cursor:pointer;" class="m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
      </div>
    </section>
  </div>
`;
