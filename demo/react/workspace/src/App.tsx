export function App() {
  return (
    <main style={{ minHeight: "100vh", display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(320px, 1fr))", gap: "24px", padding: "32px", background: "linear-gradient(180deg, #ecf2fb 0%, #f8fbff 100%)" }}>
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
          <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
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
          <span className="f-badge">Synced</span>
        </div>
        <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
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
        <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
          <button className="f-tab">Mail</button>
          <button className="f-tab">Review</button>
        </div>
        <div className="f-panel f-stack">
          <label className="f-label" style={{ display: "grid", gap: "10px" }}>
            Subject
            <input
              className="f-field focus:f-border-focus focus:f-ring"
              defaultValue="Quarterly sync notes"
            />
          </label>
          <label className="f-label" style={{ display: "grid", gap: "10px" }}>
            Summary
            <textarea
              className="f-textarea focus:f-border-focus focus:f-ring"
              rows={4}
              defaultValue="Tighten the new preset rules and review the AI output guardrails."
            />
          </label>
          <label className="f-label" style={{ display: "grid", gap: "10px" }}>
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
          <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
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
          <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
            <div className="f-spinner" aria-hidden="true"></div>
            <span className="f-label">Syncing shell surfaces</span>
          </div>
          <div className="f-skeleton" style={{ minHeight: "4rem" }} aria-hidden="true"></div>
          <div className="f-empty-state">
            <span className="f-title">No pinned layouts yet</span>
            <span className="f-body">Save a workspace snapshot to reuse this Fluent shell later.</span>
          </div>
          <aside className="f-sheet f-stack">
            <span className="f-label">Inspector sheet</span>
            <span className="f-body f-text-muted">Review density, radius, and preset overrides without leaving the canvas.</span>
          </aside>
          <div className="f-accordion-item">
            <span className="f-label">Advanced routing rules</span>
            <span className="f-body f-text-muted">Expand to review archive, follow-up, and desktop notification behavior.</span>
          </div>
          <div className="f-table-row">
            <span className="f-body">Inbox sync</span>
            <span className="f-label">Pending</span>
            <button className="f-icon-button" aria-label="Open inbox sync row">+</button>
          </div>
        </div>
      </section>
      <section className="m-surface f-stack dark:m-elevation-1">
        <aside className="m-drawer f-stack">
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
          <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
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
          <span className="m-badge">Ready</span>
        </div>
        <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
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
        <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
          <button className="m-tab">Board</button>
          <button className="m-tab">Timeline</button>
        </div>
        <div className="m-surface-container f-stack">
          <label className="m-label" style={{ display: "grid", gap: "10px" }}>
            Search
            <input
              className="m-field focus:m-border-focus focus:m-ring"
              defaultValue="Ship the preset charter and workspace demo"
            />
          </label>
          <label className="m-label" style={{ display: "grid", gap: "10px" }}>
            Highlight
            <textarea
              className="m-textarea focus:m-border-focus focus:m-ring"
              rows={4}
              defaultValue="Surface container carries secondary actions"
            />
          </label>
          <label className="m-label" style={{ display: "grid", gap: "10px" }}>
            Status
            <select className="m-select focus:m-border-focus focus:m-ring" defaultValue="ship">
              <option value="ship">Ready to ship</option>
              <option value="review">Needs review</option>
            </select>
          </label>
          <div className="m-list-item">
            <span>Sync with mobile owners</span>
            <button className="m-switch" aria-label="Sync with mobile owners" />
          </div>
          <div style={{ display: "flex", gap: "12px", flexWrap: "wrap" }}>
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
          <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
            <div className="m-spinner" aria-hidden="true"></div>
            <span className="m-label">Refreshing board metrics</span>
          </div>
          <div className="m-skeleton" style={{ minHeight: "4rem" }} aria-hidden="true"></div>
          <div className="m-empty-state">
            <span className="m-title">No suggested tasks</span>
            <span className="m-body m-text-muted">Create a new issue or import work from another Google-style project board.</span>
          </div>
          <aside className="m-sheet f-stack">
            <span className="m-label">Details sheet</span>
            <span className="m-body m-text-muted">Secondary actions and metadata stay close without overpowering the primary flow.</span>
          </aside>
          <div className="m-accordion-item">
            <span className="m-label">Delivery constraints</span>
            <span className="m-body m-text-muted">Expand to inspect owner rules, channel routing, and approval coverage.</span>
          </div>
          <div className="m-table-row">
            <span className="m-body">Board publish</span>
            <span className="m-label">Today</span>
            <button className="m-icon-button" aria-label="Open board publish row">+</button>
          </div>
        </div>
      </section>
    </main>
  );
}





