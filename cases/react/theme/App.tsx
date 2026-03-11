export function App() {
  return (
    <>
      <section className="f-surface f-stack">
        <span className="f-text-muted f-body">Win11 preset</span>
        <h2 className="f-text-primary f-title">Mica-like surface, restrained corners</h2>
        <p className="f-text-muted f-body">Softer surface, clearer focus ring, and quieter desktop-oriented feel.</p>
        <button className="focus:f-ring hover:f-bg-primary">Fluent button</button>
      </section>
      <section className="m-surface f-stack dark:m-elevation-1">
        <span className="m-text-muted m-body">Google preset</span>
        <h2 className="m-text-primary m-title">Bolder color roles, fuller rounded action</h2>
        <p className="m-text-muted m-body">Stronger brand button, more explicit hierarchy, more rounded action shape.</p>
        <button className="m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
      </section>
    </>
  );
}
