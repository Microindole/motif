const clusterStyle = { display: "flex", gap: "12px", flexWrap: "wrap" } as const;
const compactClusterStyle = { display: "flex", gap: "8px", flexWrap: "wrap" } as const;
const rowStyle = { display: "grid", gap: "10px" } as const;
const spinnerRowStyle = { display: "flex", alignItems: "center", gap: "12px" } as const;

export function MaterialWorkspaceSection() {
  return (
    <section className="m-surface f-stack dark:m-elevation-1">
      <aside className="m-drawer f-stack ui-gap-lg">
        <div className="m-persona">
          <div className="m-avatar">R</div>
          <div>
            <div className="m-label">Riley</div>
            <nav className="m-breadcrumb">
              <div className="m-breadcrumb-item">Workspace</div>
              <div className="m-breadcrumb-item">Board</div>
            </nav>
          </div>
        </div>
        <input className="m-search-field" defaultValue="Search tasks" />
        <div style={clusterStyle}>
          <button className="m-segmented-button">Board</button>
          <button className="m-segmented-button">Calendar</button>
        </div>
        <div className="m-toast">
          <span>Task board synced with mobile</span>
          <button className="m-icon-button" aria-label="Dismiss sync toast">x</button>
        </div>
      </aside>
      <div className="m-banner">
        <span className="m-label">Google workspace</span>
        <div style={compactClusterStyle}>
          <span className="m-badge">Ready</span>
          <span className="m-chip">Mobile</span>
        </div>
      </div>
      <div style={clusterStyle}>
        <button className="m-nav-item">Board</button>
        <button className="m-nav-item">Backlog</button>
        <button className="m-icon-button" aria-label="Open quick actions">+</button>
      </div>
      <span className="m-label">Google workspace</span>
      <h2 className="m-title m-text-primary">Task board with explicit action hierarchy</h2>
      <p className="m-body m-text-muted">
        The container system is clearer, the label scale is tighter, and the
        actions feel more direct.
      </p>
      <div className="m-divider"></div>
      <div style={clusterStyle}>
        <button className="m-tab">Board</button>
        <button className="m-tab">Timeline</button>
      </div>
      <div className="m-surface-container f-stack">
        <label className="m-label" style={rowStyle}>
          Search
          <input
            className="m-field focus:m-border-focus focus:m-ring"
            defaultValue="Ship the preset charter and workspace demo"
          />
        </label>
        <label className="m-label" style={rowStyle}>
          Highlight
          <textarea
            className="m-textarea focus:m-border-focus focus:m-ring"
            rows={4}
            defaultValue="Surface container carries secondary actions"
          />
        </label>
        <label className="m-label" style={rowStyle}>
          Status
          <select className="m-select focus:m-border-focus focus:m-ring" defaultValue="ship">
            <option value="ship">Ready to ship</option>
            <option value="review">Needs review</option>
          </select>
        </label>
        <div className="m-list-item ui-gap-md">
          <span>Sync with mobile owners</span>
          <button className="m-switch" aria-label="Sync with mobile owners" />
        </div>
        <div style={clusterStyle}>
          <button
            style={{ cursor: "pointer" }}
            className="m-action-outlined hover:m-bg-hover-surface active:m-shadow-press"
          >
            Open filter
          </button>
          <button
            style={{ cursor: "pointer" }}
            className="m-action-tonal hover:m-bg-hover-container active:m-shadow-press"
          >
            Save draft
          </button>
          <button
            style={{ cursor: "pointer" }}
            className="m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press"
          >
            Create task
          </button>
        </div>
        <div className="m-dialog f-stack">
          <span className="m-label">Quick actions</span>
          <button className="m-menu-item hover:m-bg-hover-surface">Open detail panel</button>
          <button className="m-menu-item hover:m-bg-hover-surface">Duplicate task</button>
          <div className="m-tooltip">Material tooltip tone for direct guidance</div>
        </div>
        <div className="m-progress" aria-hidden="true"></div>
        <div style={spinnerRowStyle}>
          <div className="m-spinner" aria-hidden="true"></div>
          <span className="m-label">Refreshing board metrics</span>
        </div>
        <div className="m-skeleton" style={{ minHeight: "4rem" }} aria-hidden="true"></div>
        <div className="m-empty-state">
          <span className="m-title">No suggested tasks</span>
          <span className="m-body m-text-muted">
            Create a new issue or import work from another Google-style project board.
          </span>
        </div>
        <aside className="m-sheet-side f-stack">
          <span className="m-label">Details sheet</span>
          <span className="m-body m-text-muted">
            Secondary actions and metadata stay close without overpowering the primary flow.
          </span>
        </aside>
        <aside className="m-sheet-bottom f-stack">
          <span className="m-label">Bottom tray</span>
          <span className="m-body m-text-muted">
            Suggested follow-ups and recent board activity stay anchored below the main content.
          </span>
        </aside>
        <div className="m-accordion-item-open">
          <div className="m-accordion-header">
            <span className="m-label">Delivery constraints</span>
            <span className="m-tag">Expanded</span>
          </div>
          <div className="m-accordion-content">
            <span className="m-body m-text-muted">
              Expand to inspect owner rules, channel routing, and approval coverage.
            </span>
          </div>
        </div>
        <div className="m-table">
          <div className="m-table-header">
            <span className="m-label">Workflow</span>
            <span className="m-label">Status</span>
            <span className="m-label">Open</span>
          </div>
          <div className="m-table-row-selected">
            <span className="m-table-cell">Board publish</span>
            <div style={compactClusterStyle}>
              <span className="m-label">Today</span>
              <span className="m-tag">Shared</span>
            </div>
            <button className="m-icon-button" aria-label="Open board publish row">+</button>
          </div>
        </div>
      </div>
    </section>
  );
}
