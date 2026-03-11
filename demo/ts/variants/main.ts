const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <section class="m-surface f-stack dark:m-elevation-1">
    <h1 class="f-text-primary">motif TypeScript variants</h1>
    <p>Compile <code>main.ts</code> to <code>dist/main.js</code>, then open <code>index.html</code>.</p>
    <button class="focus:f-ring hover:f-bg-primary active:m-shadow-2">TS variants</button>
  </section>
`;
