<template>
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
      <div class="cluster">
        <button class="button-reset m-segmented-button">Board</button>
        <button class="button-reset m-segmented-button">Calendar</button>
      </div>
      <div class="m-toast">
        <span>Task board synced with mobile</span>
        <button class="button-reset m-icon-button" aria-label="Dismiss sync toast">x</button>
      </div>
    </aside>
    <div class="m-banner">
      <span class="m-label">Google workspace</span>
      <div class="cluster compact-cluster">
        <span class="m-badge">Ready</span>
        <span class="m-chip">Mobile</span>
      </div>
    </div>
    <div class="cluster">
      <button class="button-reset m-nav-item">Board</button>
      <button class="button-reset m-nav-item">Backlog</button>
      <button class="button-reset m-icon-button" aria-label="Open quick actions">+</button>
    </div>
    <span class="m-label">Google workspace</span>
    <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
    <p class="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
    <div class="m-divider"></div>
    <div class="cluster">
      <button class="button-reset m-tab">Board</button>
      <button class="button-reset m-tab">Timeline</button>
    </div>
    <div class="m-surface-container f-stack">
      <label class="m-label row">
        Search
        <input class="entry m-field focus:m-border-focus focus:m-ring" value="Ship the preset charter and workspace demo" />
      </label>
      <label class="m-label row">
        Highlight
        <textarea class="entry m-textarea focus:m-border-focus focus:m-ring" rows="4">Surface container carries secondary actions</textarea>
      </label>
      <label class="m-label row">
        Status
        <select class="entry m-select focus:m-border-focus focus:m-ring">
          <option>Ready to ship</option>
          <option>Needs review</option>
        </select>
      </label>
      <div class="m-list-item">
        <span>Sync with mobile owners</span>
        <button class="button-reset m-switch" aria-label="Sync with mobile owners"></button>
      </div>
      <div class="cluster">
        <button class="button-reset m-action-outlined hover:m-bg-hover-surface active:m-shadow-press">Open filter</button>
        <button class="button-reset m-action-tonal hover:m-bg-hover-container active:m-shadow-press">Save draft</button>
        <button class="button-reset m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Create task</button>
      </div>
      <div class="m-dialog f-stack">
        <span class="m-label">Quick actions</span>
        <button class="button-reset m-menu-item hover:m-bg-hover-surface">Open detail panel</button>
        <button class="button-reset m-menu-item hover:m-bg-hover-surface">Duplicate task</button>
        <div class="m-tooltip">Material tooltip tone for direct guidance</div>
      </div>
      <div class="m-progress" aria-hidden="true"></div>
      <div class="cluster spinner-row">
        <div class="m-spinner" aria-hidden="true"></div>
        <span class="m-label">Refreshing board metrics</span>
      </div>
      <div class="m-skeleton tall-skeleton" aria-hidden="true"></div>
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
          <div class="cluster compact-cluster">
            <span class="m-label">Today</span>
            <span class="m-tag">Shared</span>
          </div>
          <button class="button-reset m-icon-button" aria-label="Open board publish row">+</button>
        </div>
      </div>
    </div>
  </section>
</template>
