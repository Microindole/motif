const clusterStyle = { display: "flex", gap: "12px", flexWrap: "wrap" } as const;
const compactClusterStyle = { display: "flex", gap: "8px", flexWrap: "wrap" } as const;
const rowStyle = { display: "grid", gap: "10px" } as const;
const spinnerRowStyle = { display: "flex", alignItems: "center", gap: "12px" } as const;

export function FluentWorkspaceSection() {
  return (
    <section className="f-surface f-stack">
      <aside className="f-drawer f-stack">
        <div className="f-persona">
          <div className="f-avatar">A</div>
          <div>
            <div className="f-label">Alex</div>
            <nav className="f-breadcrumb">
              <div className="f-breadcrumb-item">Workspace</div>
              <div className="f-breadcrumb-item">Inbox</div>
            </nav>
          </div>
        </div>
        <input className="f-search-field" defaultValue="Search drafts" />
        <div style={clusterStyle}>
          <button className="f-segmented-button">Focused</button>
          <button className="f-segmented-button">Planned</button>
        </div>
        <div className="f-toast">
          <span>Draft synced to desktop shell</span>
          <button className="f-icon-button" aria-label="Dismiss sync toast">x</button>
        </div>
      </aside>
      <div className="f-banner">
        <span className="f-label">Windows workspace</span>
        <div style={compactClusterStyle}>
          <span className="f-badge">Synced</span>
          <span className="f-chip">Desktop</span>
        </div>
      </div>
      <div style={clusterStyle}>
        <button className="f-nav-item">Inbox</button>
        <button className="f-nav-item">Archive</button>
        <button className="f-icon-button" aria-label="Open quick actions">+</button>
      </div>
      <span className="f-label">Windows workspace</span>
      <h2 className="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
      <p className="f-body f-text-muted">
        The shell stays soft, the divider stays subtle, and the primary action
        stays restrained until you need it.
      </p>
      <div className="f-divider"></div>
      <div style={clusterStyle}>
        <button className="f-tab">Mail</button>
        <button className="f-tab">Review</button>
      </div>
      <div className="f-panel f-stack">
        <label className="f-label" style={rowStyle}>
          Subject
          <input
            className="f-field focus:f-border-focus focus:f-ring"
            defaultValue="Quarterly sync notes"
          />
        </label>
        <label className="f-label" style={rowStyle}>
          Summary
          <textarea
            className="f-textarea focus:f-border-focus focus:f-ring"
            rows={4}
            defaultValue="Tighten the new preset rules and review the AI output guardrails."
          />
        </label>
        <label className="f-label" style={rowStyle}>
          Folder
          <select className="f-select focus:f-border-focus focus:f-ring" defaultValue="inbox">
            <option value="inbox">Inbox</option>
            <option value="follow-up">Follow up</option>
          </select>
        </label>
        <div className="f-list-item">
          <span>Share with review group</span>
          <input type="checkbox" className="f-checkbox" aria-label="Share with review group" />
        </div>
        <div style={clusterStyle}>
          <button
            style={{ cursor: "pointer" }}
            className="f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press"
          >
            Preview panel
          </button>
          <button
            style={{ cursor: "pointer" }}
            className="f-action-primary hover:f-bg-hover-primary active:f-shadow-press"
          >
            Send update
          </button>
        </div>
        <div className="f-dialog f-stack">
          <span className="f-label">Quick menu</span>
          <button className="f-menu-item hover:f-bg-hover-subtle">Move to archive</button>
          <button className="f-menu-item hover:f-bg-hover-subtle">Assign owner</button>
          <div className="f-tooltip">Fluent tooltip tone for calm guidance</div>
        </div>
        <div className="f-progress" aria-hidden="true"></div>
        <div style={spinnerRowStyle}>
          <div className="f-spinner" aria-hidden="true"></div>
          <span className="f-label">Syncing shell surfaces</span>
        </div>
        <div className="f-skeleton" style={{ minHeight: "4rem" }} aria-hidden="true"></div>
        <div className="f-empty-state">
          <span className="f-title">No pinned layouts yet</span>
          <span className="f-body">
            Save a workspace snapshot to reuse this Fluent shell later.
          </span>
        </div>
        <aside className="f-sheet-side f-stack">
          <span className="f-label">Inspector sheet</span>
          <span className="f-body f-text-muted">
            Review density, radius, and preset overrides without leaving the canvas.
          </span>
        </aside>
        <aside className="f-sheet-bottom f-stack">
          <span className="f-label">Bottom tray</span>
          <span className="f-body f-text-muted">
            Queued shell actions and follow-up suggestions stay docked below the canvas.
          </span>
        </aside>
        <div className="f-accordion-item-open">
          <div className="f-accordion-header">
            <span className="f-label">Advanced routing rules</span>
            <span className="f-tag">Expanded</span>
          </div>
          <div className="f-accordion-content">
            <span className="f-body f-text-muted">
              Expand to review archive, follow-up, and desktop notification behavior.
            </span>
          </div>
        </div>
        <div className="f-table">
          <div className="f-table-header">
            <span className="f-label">Workflow</span>
            <span className="f-label">Status</span>
            <span className="f-label">Open</span>
          </div>
          <div className="f-table-row-selected">
            <span className="f-table-cell">Inbox sync</span>
            <div style={compactClusterStyle}>
              <span className="f-label">Pending</span>
              <span className="f-tag">Desktop only</span>
            </div>
            <button className="f-icon-button" aria-label="Open inbox sync row">+</button>
          </div>
        </div>
      </div>
    </section>
  );
}
