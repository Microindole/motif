export function App() {
  return (
    <main style={{ minHeight: "100vh", display: "grid", placeItems: "center", background: "linear-gradient(180deg, #f5f5f5 0%, #ffffff 100%)" }}>
      <section className="m-surface f-stack">
        <h1 className="f-text-primary">motif React demo</h1>
        <p>Generate <code>motif.css</code>, then run <code>npm install</code> and <code>npm run dev</code>.</p>
        <button className="f-action-primary focus:f-ring hover:f-bg-hover-primary active:f-shadow-press">React basic</button>
      </section>
    </main>
  );
}


