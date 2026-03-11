export function App() {
  return (
    <>
      <section className="f-surface f-stack">
        <h2 className="f-text-primary">Win11 preset</h2>
        <button className="focus:f-ring hover:f-bg-primary">Fluent button</button>
      </section>
      <section className="m-surface f-stack dark:m-elevation-1">
        <h2 className="m-text-primary">Google preset</h2>
        <button className="m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
      </section>
    </>
  );
}
