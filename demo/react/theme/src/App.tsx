export function App() {
  return (
    <main style={{ minHeight: "100vh", display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))", gap: "24px", padding: "32px", background: "linear-gradient(180deg, #e9eef7 0%, #ffffff 100%)" }}>
      <section className="f-surface f-stack">
        <h2 className="f-text-primary">Win11 preset</h2>
        <p>Fluent direction with restrained radius and soft neutral surface.</p>
        <button className="focus:f-ring hover:f-bg-primary">Fluent button</button>
      </section>
      <section className="m-surface f-stack dark:m-elevation-1">
        <h2 className="m-text-primary">Google preset</h2>
        <p>Material direction with stronger blue emphasis and elevated press state.</p>
        <button className="m-bg-primary m-text-on-primary active:m-shadow-2">Material button</button>
      </section>
    </main>
  );
}
