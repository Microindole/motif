const app = document.querySelector<HTMLDivElement>("#app");

if (!app) {
  throw new Error("#app not found");
}

app.innerHTML = `
  <main style="min-height: 100vh; display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 24px; padding: 32px; background: linear-gradient(180deg, #ecf2fb 0%, #f8fbff 100%);">
    <section class="f-surface f-stack">
      <aside class="f-drawer f-stack">
        <div class="f-persona">
          <div class="f-avatar">A</div>
          <div>
            <div class="f-label">Alex</div>
            <nav class="f-breadcrumb">
              <div class="f-breadcrumb-item">Workspace</div>
              <div class="f-breadcrumb-item">Inbox</div>
            </nav>
          </div>
        </div>
        <input class="f-search-field" value="Search drafts" />
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button class="f-segmented-button">Focused</button>
          <button class="f-segmented-button">Planned</button>
        </div>
        <div class="f-toast">
          <span>Draft synced to desktop shell</span>
          <button class="f-icon-button" aria-label="Dismiss sync toast">x</button>
        </div>
      </aside>
        <div class="f-banner">
          <span class="f-label">Windows workspace</span>
          <div style="display:flex;gap:8px;flex-wrap:wrap;">
            <span class="f-badge">Synced</span>
            <span class="f-chip">Desktop</span>
          </div>
        </div>
      <div style="display:flex;gap:12px;flex-wrap:wrap;">
        <button class="f-nav-item">Inbox</button>
        <button class="f-nav-item">Archive</button>
        <button class="f-icon-button" aria-label="Open quick actions">+</button>
      </div>
      <span class="f-label">Windows workspace</span>
      <h2 class="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
      <p class="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
      <div class="f-divider"></div>
      <div style="display:flex;gap:12px;flex-wrap:wrap;">
        <button class="f-tab">Mail</button>
        <button class="f-tab">Review</button>
      </div>
      <div class="f-panel f-stack">
        <label class="f-label" style="display:grid;gap:10px;">
          Subject
          <input class="f-field focus:f-border-focus focus:f-ring" value="Quarterly sync notes" />
        </label>
        <label class="f-label" style="display:grid;gap:10px;">
          Summary
          <textarea class="f-textarea focus:f-border-focus focus:f-ring" rows="4">Tighten the new preset rules and review the AI output guardrails.</textarea>
        </label>
        <label class="f-label" style="display:grid;gap:10px;">
          Folder
          <select class="f-select focus:f-border-focus focus:f-ring">
            <option>Inbox</option>
            <option>Follow up</option>
          </select>
        </label>
        <div class="f-list-item">
          <span>Share with review group</span>
          <input type="checkbox" class="f-checkbox" aria-label="Share with review group" />
        </div>
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button style="cursor:pointer;" class="f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Preview panel</button>
          <button style="cursor:pointer;" class="f-action-primary hover:f-bg-hover-primary active:f-shadow-press">Send update</button>
        </div>
          <div class="f-dialog f-stack">
            <span class="f-label">Quick menu</span>
            <button class="f-menu-item hover:f-bg-hover-subtle">Move to archive</button>
            <button class="f-menu-item hover:f-bg-hover-subtle">Assign owner</button>
            <div class="f-tooltip">Fluent tooltip tone for calm guidance</div>
          </div>
          <div class="f-progress" aria-hidden="true"></div>
          <div style="display:flex;align-items:center;gap:12px;">
            <div class="f-spinner" aria-hidden="true"></div>
            <span class="f-label">Syncing shell surfaces</span>
          </div>
          <div class="f-skeleton" style="min-height: 4rem;" aria-hidden="true"></div>
          <div class="f-empty-state">
            <span class="f-title">No pinned layouts yet</span>
            <span class="f-body">Save a workspace snapshot to reuse this Fluent shell later.</span>
          </div>
          <aside class="f-sheet-side f-stack">
            <span class="f-label">Inspector sheet</span>
            <span class="f-body f-text-muted">Review density, radius, and preset overrides without leaving the canvas.</span>
          </aside>
          <aside class="f-sheet-bottom f-stack">
            <span class="f-label">Bottom tray</span>
            <span class="f-body f-text-muted">Queued shell actions and follow-up suggestions stay docked below the canvas.</span>
          </aside>
          <div class="f-accordion-item-open">
            <div class="f-accordion-header">
              <span class="f-label">Advanced routing rules</span>
              <span class="f-tag">Expanded</span>
            </div>
            <div class="f-accordion-content">
              <span class="f-body f-text-muted">Expand to review archive, follow-up, and desktop notification behavior.</span>
            </div>
          </div>
          <div class="f-table">
            <div class="f-table-header">
              <span class="f-label">Workflow</span>
              <span class="f-label">Status</span>
              <span class="f-label">Open</span>
            </div>
            <div class="f-table-row-selected">
              <span class="f-table-cell">Inbox sync</span>
              <div style="display:flex;gap:8px;flex-wrap:wrap;">
                <span class="f-label">Pending</span>
                <span class="f-tag">Desktop only</span>
              </div>
              <button class="f-icon-button" aria-label="Open inbox sync row">+</button>
            </div>
          </div>
        </div>
    </section>
    <section class="m-surface f-stack dark:m-elevation-1">
      <aside class="m-drawer f-stack">
        <div class="m-persona">
          <div class="m-avatar">R</div>
          <div>
            <div class="m-label">Riley</div>
            <nav class="m-breadcrumb">
              <div class="m-breadcrumb-item">Workspace</div>
              <div class="m-breadcrumb-item">Board</div>
            </nav>
          </div>
        </div>
        <input class="m-search-field" value="Search tasks" />
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button class="m-segmented-button">Board</button>
          <button class="m-segmented-button">Calendar</button>
        </div>
        <div class="m-toast">
          <span>Task board synced with mobile</span>
          <button class="m-icon-button" aria-label="Dismiss sync toast">x</button>
        </div>
      </aside>
        <div class="m-banner">
          <span class="m-label">Google workspace</span>
          <div style="display:flex;gap:8px;flex-wrap:wrap;">
            <span class="m-badge">Ready</span>
            <span class="m-chip">Mobile</span>
          </div>
        </div>
      <div style="display:flex;gap:12px;flex-wrap:wrap;">
        <button class="m-nav-item">Board</button>
        <button class="m-nav-item">Backlog</button>
        <button class="m-icon-button" aria-label="Open quick actions">+</button>
      </div>
      <span class="m-label">Google workspace</span>
      <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
      <p class="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
      <div class="m-divider"></div>
      <div style="display:flex;gap:12px;flex-wrap:wrap;">
        <button class="m-tab">Board</button>
        <button class="m-tab">Timeline</button>
      </div>
      <div class="m-surface-container f-stack">
        <label class="m-label" style="display:grid;gap:10px;">
          Search
          <input class="m-field focus:m-border-focus focus:m-ring" value="Ship the preset charter and workspace demo" />
        </label>
        <label class="m-label" style="display:grid;gap:10px;">
          Highlight
          <textarea class="m-textarea focus:m-border-focus focus:m-ring" rows="4">Surface container carries secondary actions</textarea>
        </label>
        <label class="m-label" style="display:grid;gap:10px;">
          Status
          <select class="m-select focus:m-border-focus focus:m-ring">
            <option>Ready to ship</option>
            <option>Needs review</option>
          </select>
        </label>
        <div class="m-list-item">
          <span>Sync with mobile owners</span>
          <button class="m-switch" aria-label="Sync with mobile owners"></button>
        </div>
        <div style="display:flex;gap:12px;flex-wrap:wrap;">
          <button style="cursor:pointer;" class="m-action-outlined hover:m-bg-hover-surface active:m-shadow-press">Open filter</button>
          <button style="cursor:pointer;" class="m-action-tonal hover:m-bg-hover-container active:m-shadow-press">Save draft</button>
          <button style="cursor:pointer;" class="m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Create task</button>
        </div>
          <div class="m-dialog f-stack">
            <span class="m-label">Quick actions</span>
            <button class="m-menu-item hover:m-bg-hover-surface">Open detail panel</button>
            <button class="m-menu-item hover:m-bg-hover-surface">Duplicate task</button>
            <div class="m-tooltip">Material tooltip tone for direct guidance</div>
          </div>
          <div class="m-progress" aria-hidden="true"></div>
          <div style="display:flex;align-items:center;gap:12px;">
            <div class="m-spinner" aria-hidden="true"></div>
            <span class="m-label">Refreshing board metrics</span>
          </div>
          <div class="m-skeleton" style="min-height: 4rem;" aria-hidden="true"></div>
          <div class="m-empty-state">
            <span class="m-title">No suggested tasks</span>
            <span class="m-body m-text-muted">Create a new issue or import work from another Google-style project board.</span>
          </div>
          <aside class="m-sheet-side f-stack">
            <span class="m-label">Details sheet</span>
            <span class="m-body m-text-muted">Secondary actions and metadata stay close without overpowering the primary flow.</span>
          </aside>
          <aside class="m-sheet-bottom f-stack">
            <span class="m-label">Bottom tray</span>
            <span class="m-body m-text-muted">Suggested follow-ups and recent board activity stay anchored below the main content.</span>
          </aside>
          <div class="m-accordion-item-open">
            <div class="m-accordion-header">
              <span class="m-label">Delivery constraints</span>
              <span class="m-tag">Expanded</span>
            </div>
            <div class="m-accordion-content">
              <span class="m-body m-text-muted">Expand to inspect owner rules, channel routing, and approval coverage.</span>
            </div>
          </div>
          <div class="m-table">
            <div class="m-table-header">
              <span class="m-label">Workflow</span>
              <span class="m-label">Status</span>
              <span class="m-label">Open</span>
            </div>
            <div class="m-table-row-selected">
              <span class="m-table-cell">Board publish</span>
              <div style="display:flex;gap:8px;flex-wrap:wrap;">
                <span class="m-label">Today</span>
                <span class="m-tag">Shared</span>
              </div>
              <button class="m-icon-button" aria-label="Open board publish row">+</button>
            </div>
          </div>
        </div>
    </section>
  </main>
`;





