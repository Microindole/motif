const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <div style="min-height: 100vh; display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 24px; padding: 32px; background: linear-gradient(180deg, #e9eef7 0%, #ffffff 100%);">
    <section class="f-surface f-stack">
      <h2 class="f-text-primary">Win11 preset</h2>
      <p>Fluent direction with softer radius and Win11-like blue emphasis.</p>
      <button class="focus:f-ring hover:f-bg-primary">Fluent button</button>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <h2 class="m-text-primary">Google preset</h2>
      <p>Material direction with stronger primary button and elevation.</p>
      <button class="m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
    </section>
  </div>
`;
