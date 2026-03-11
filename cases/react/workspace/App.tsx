export function WorkspaceCase() {
  return (
    <>
      <section className="f-surface f-stack">
        <span className="f-label">Windows workspace</span>
        <h2 className="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
        <div className="f-divider"></div>
        <input className="f-field focus:f-ring" defaultValue="Quarterly sync notes" />
        <button className="f-action-primary hover:f-bg-primary">Send update</button>
      </section>
      <section className="m-surface f-stack dark:m-elevation-1">
        <span className="m-label">Google workspace</span>
        <h2 className="m-title m-text-primary">Task board with explicit action hierarchy</h2>
        <div className="m-divider"></div>
        <input className="m-field" defaultValue="Ship the preset charter and workspace demo" />
        <button className="m-action-tonal">Save draft</button>
        <button className="m-action-primary active:m-shadow-2">Create task</button>
      </section>
    </>
  );
}
