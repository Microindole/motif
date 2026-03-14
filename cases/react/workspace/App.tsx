export function WorkspaceCase() {
  return (
    <>
      <div className="f-drawer f-stack"><div className="f-persona"><div className="f-avatar">A</div><div><div className="f-label">Alex</div><nav className="f-breadcrumb"><div className="f-breadcrumb-item">Workspace</div><div className="f-breadcrumb-item">Inbox</div></nav></div></div><input className="f-search-field" defaultValue="Search drafts" /><button className="f-segmented-button">Focused</button><div className="f-toast">Synced</div></div>
      <div className="f-banner"><span className="f-badge">Synced</span><button className="f-icon-button">+</button></div>
      <button className="f-nav-item">Inbox</button>
      <div className="f-surface f-stack">
        <span className="f-label">Windows workspace</span>
        <h2 className="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
        <p className="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
        <div className="f-divider"></div>
        <button className="f-tab">Mail</button>
        <div className="f-panel f-stack">
          <label className="f-label">
            Subject
            <input className="f-field focus:f-border-focus focus:f-ring" defaultValue="Quarterly sync notes" />
          </label>
          <textarea className="f-textarea focus:f-border-focus focus:f-ring" defaultValue="Draft body"></textarea>
          <select className="f-select focus:f-border-focus focus:f-ring" defaultValue="inbox">
            <option value="inbox">Inbox</option>
          </select>
          <div className="f-list-item"><span>Share</span><input type="checkbox" className="f-checkbox" /></div>
          <button className="f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Preview panel</button>
          <button className="f-action-primary hover:f-bg-hover-primary active:f-shadow-press">Send update</button>
          <div className="f-dialog f-stack"><button className="f-menu-item">Move</button></div>
          <div className="f-tooltip">Tip</div>
          <div className="f-progress"></div>
          <div className="f-spinner"></div>
          <div className="f-skeleton"></div>
          <div className="f-empty-state">No pinned layouts</div>
          <aside className="f-sheet f-stack">Inspector</aside>
          <div className="f-accordion-item-open">Rules</div>
          <div className="f-table-row-selected"><span>Inbox sync</span><span>Pending</span><button className="f-icon-button">+</button></div>
        </div>
      </div>
      <div className="m-banner"><span className="m-badge">Ready</span><button className="m-icon-button">+</button></div>
      <div className="m-drawer f-stack"><div className="m-persona"><div className="m-avatar">R</div><div><div className="m-label">Riley</div><nav className="m-breadcrumb"><div className="m-breadcrumb-item">Workspace</div><div className="m-breadcrumb-item">Board</div></nav></div></div><input className="m-search-field" defaultValue="Search tasks" /><button className="m-segmented-button">Board</button><div className="m-toast">Ready</div></div>
      <button className="m-nav-item">Board</button>
      <div className="m-surface f-stack dark:m-elevation-1">
        <span className="m-label">Google workspace</span>
        <h2 className="m-title m-text-primary">Task board with explicit action hierarchy</h2>
        <p className="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
        <div className="m-divider"></div>
        <button className="m-tab">Board</button>
        <div className="m-surface-container f-stack">
          <label className="m-label">
            Search
            <input className="m-field focus:m-border-focus focus:m-ring" defaultValue="Ship the preset charter and workspace demo" />
          </label>
          <textarea className="m-textarea focus:m-border-focus focus:m-ring" defaultValue="Board note"></textarea>
          <select className="m-select focus:m-border-focus focus:m-ring" defaultValue="ship">
            <option value="ship">Ready</option>
          </select>
          <div className="m-list-item"><span>Sync</span><button className="m-switch">Toggle</button></div>
          <button className="m-action-outlined hover:m-bg-hover-surface active:m-shadow-press">Open filter</button>
          <button className="m-action-tonal hover:m-bg-hover-container active:m-shadow-press">Save draft</button>
          <button className="m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Create task</button>
          <div className="m-dialog f-stack"><button className="m-menu-item">Duplicate</button></div>
          <div className="m-tooltip">Tip</div>
          <div className="m-progress"></div>
          <div className="m-spinner"></div>
          <div className="m-skeleton"></div>
          <div className="m-empty-state">No suggested tasks</div>
          <aside className="m-sheet f-stack">Details</aside>
          <div className="m-accordion-item-open">Constraints</div>
          <div className="m-table-row-selected"><span>Board publish</span><span>Today</span><button className="m-icon-button">+</button></div>
        </div>
      </div>
    </>
  );
}





