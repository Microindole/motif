export function App() {
  return (
    <main style={{ minHeight: "100vh", display: "grid", placeItems: "center", background: "linear-gradient(180deg, #eef4ff 0%, #ffffff 100%)" }}>
      <section className="m-surface f-stack dark:m-elevation-1">
        <h1 className="f-text-primary">motif React variants</h1>
        <p>Generate <code>motif.css</code>, then run <code>npm install</code> and <code>npm run dev</code>.</p>
        <button className="focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">React variants</button>
      </section>
    </main>
  );
}


