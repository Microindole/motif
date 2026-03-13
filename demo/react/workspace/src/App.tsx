export function App() {
  return (
    <main style={{ minHeight: "100vh", display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(320px, 1fr))", gap: "24px", padding: "32px", background: "linear-gradient(180deg, #ecf2fb 0%, #f8fbff 100%)" }}>
      <section className="f-surface f-stack">
        <span className="f-label">Windows workspace</span>
        <h2 className="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
        <p className="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
        <div className="f-divider"></div>
        <div className="f-panel f-stack">
          <label className="f-label" style={{ display: "grid", gap: "10px" }}>
            Subject
            <input className="f-field focus:f-border-focus focus:f-ring" defaultValue="Quarterly sync notes" />
          </label>
          <label className="f-label" style={{ display: "grid", gap: "10px" }}>
            Summary
            <textarea className="f-field focus:f-border-focus focus:f-ring" rows={4} defaultValue="Tighten the new preset rules and review the AI output guardrails." />
          </label>
          <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
            <button style={{ cursor: "pointer" }} className="f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Preview panel</button>
            <button style={{ cursor: "pointer" }} className="f-action-primary hover:f-bg-hover-primary active:f-shadow-press">Send update</button>
          </div>
        </div>
      </section>
      <section className="m-surface f-stack dark:m-elevation-1">
        <span className="m-label">Google workspace</span>
        <h2 className="m-title m-text-primary">Task board with explicit action hierarchy</h2>
        <p className="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
        <div className="m-divider"></div>
        <div className="m-surface-container f-stack">
          <label className="m-label" style={{ display: "grid", gap: "10px" }}>
            Search
            <input className="m-field focus:m-border-focus focus:m-ring" defaultValue="Ship the preset charter and workspace demo" />
          </label>
          <label className="m-label" style={{ display: "grid", gap: "10px" }}>
            Highlight
            <input className="m-field focus:m-border-focus focus:m-ring" defaultValue="Surface container carries secondary actions" />
          </label>
          <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
            <button style={{ cursor: "pointer" }} className="m-action-outlined hover:m-bg-hover-surface active:m-shadow-press">Open filter</button>
            <button style={{ cursor: "pointer" }} className="m-action-tonal hover:m-bg-hover-container active:m-shadow-press">Save draft</button>
            <button style={{ cursor: "pointer" }} className="m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Create task</button>
          </div>
        </div>
      </section>
    </main>
  );
}





