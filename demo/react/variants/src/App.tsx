export function App() {
  return (
    <main
      style={{
        minHeight: "100vh",
        display: "grid",
        placeItems: "center",
        background: "linear-gradient(180deg, #eef4ff 0%, #ffffff 100%)",
      }}
    >
      <section className="m-surface f-stack dark:m-elevation-1 ui-pad-xl ui-radius-lg ui-density-comfortable">
        <h1 className="f-text-primary">motif React variants</h1>
        <p className="ui-text-xs">
          Preset controls now keep Material styling while allowing separate size
          tuning through <code>ui-*</code> utilities.
        </p>
        <button className="focus:m-ring hover:m-bg-hover-primary active:m-shadow-press ui-control-lg ui-radius-pill ui-density-compact">
          React variants
        </button>
        <div className="ui-stack-inline ui-gap-xl">
          <input type="checkbox" className="f-checkbox" aria-label="Fluent checkbox" />
          <input type="radio" className="m-radio" aria-label="Material radio" />
          <button className="m-switch" aria-label="Material switch" />
        </div>
        <div className="ui-stack-center ui-text-xs">Preset + ui layer</div>
      </section>
    </main>
  );
}

