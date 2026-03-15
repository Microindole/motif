const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <section class="m-surface f-stack dark:m-elevation-1 ui-pad-lg ui-radius-lg">
    <h1 class="f-text-primary">motif TypeScript variants</h1>
    <p class="ui-text-sm">Compile <code>main.ts</code> to <code>dist/main.js</code>, then open <code>index.html</code>.</p>
    <button class="focus:m-ring hover:m-bg-hover-primary active:m-shadow-press ui-control-lg ui-radius-pill">TS variants</button>
    <div style="display:flex;gap:12px;align-items:center;flex-wrap:wrap;">
      <input type="checkbox" class="f-checkbox" aria-label="Fluent checkbox" />
      <input type="radio" class="m-radio" aria-label="Material radio" />
      <button class="m-switch" aria-label="Material switch"></button>
    </div>
  </section>
`;


