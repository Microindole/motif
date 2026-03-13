export function App() {
  return (
    <main style={{ minHeight: "100vh", display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(320px, 1fr))", gap: "24px", padding: "32px", background: "radial-gradient(circle at top, #dfe7f3 0%, #ffffff 60%)" }}>
      <section className="f-surface f-stack">
        <span className="f-text-muted f-body">Win11 preset</span>
        <h2 className="f-text-primary f-title">Mica shell with acrylic inner layer</h2>
        <p className="f-text-muted f-body">The outer surface stays soft and quiet, while the inner layer feels more like a lifted acrylic pane.</p>
        <div className="f-surface-alt f-stack">
          <p className="f-text-muted f-body">Acrylic-like panel</p>
          <button style={{ border: 0, padding: "0.85rem 1.25rem", cursor: "pointer" }} className="focus:f-ring hover:f-bg-hover-primary active:f-shadow-press">Fluent button</button>
        </div>
      </section>
      <section className="m-surface f-stack dark:m-elevation-1">
        <span className="m-text-muted m-body">Google preset</span>
        <h2 className="m-text-primary m-title">Surface plus clear primary container</h2>
        <p className="m-text-muted m-body">The main card stays neutral while the inner container and primary button make hierarchy feel more explicit.</p>
        <div className="m-surface-variant f-stack">
          <p className="m-text-muted m-body">Surface container</p>
          <button style={{ border: 0, padding: "0.85rem 1.25rem", cursor: "pointer" }} className="m-bg-primary-container">Primary container</button>
          <button style={{ border: 0, padding: "0.85rem 1.25rem", cursor: "pointer" }} className="m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Material button</button>
        </div>
      </section>
    </main>
  );
}



