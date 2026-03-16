const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <section class="m-surface f-stack dark:m-elevation-1 ui-pad-xl ui-radius-lg ui-density-comfortable">
    <h1 class="f-text-primary">motif TypeScript variants</h1>
    <p class="ui-text-xs">Compile <code>main.ts</code> to <code>dist/main.js</code>, then open <code>index.html</code>.</p>
    <button class="focus:m-ring hover:m-bg-hover-primary active:m-shadow-press ui-control-lg ui-radius-pill ui-density-compact">TS variants</button>
    <div class="ui-stack-inline ui-gap-xl">
      <input type="checkbox" class="f-checkbox" aria-label="Fluent checkbox" />
      <input type="radio" class="m-radio" aria-label="Material radio" />
      <button class="m-switch" aria-label="Material switch"></button>
    </div>
    <div class="ui-stack-center ui-text-xs">Preset + ui layer</div>
  </section>
`;



