const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <section class="m-surface f-stack">
    <h1 class="f-text-primary">motif TypeScript demo</h1>
    <p>Compile <code>main.ts</code> to <code>dist/main.js</code>, then open <code>index.html</code>.</p>
    <button class="f-action-primary focus:f-ring hover:f-bg-hover-primary active:f-shadow-press">TS basic</button>
  </section>
`;


