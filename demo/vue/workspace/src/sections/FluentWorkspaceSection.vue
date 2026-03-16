<template>
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
      <div class="cluster">
        <button class="button-reset f-segmented-button">Focused</button>
        <button class="button-reset f-segmented-button">Planned</button>
      </div>
      <div class="f-toast">
        <span>Draft synced to desktop shell</span>
        <button class="button-reset f-icon-button" aria-label="Dismiss sync toast">x</button>
      </div>
    </aside>
    <div class="f-banner">
      <span class="f-label">Windows workspace</span>
      <div class="cluster compact-cluster">
        <span class="f-badge">Synced</span>
        <span class="f-chip">Desktop</span>
      </div>
    </div>
    <div class="cluster">
      <button class="button-reset f-nav-item">Inbox</button>
      <button class="button-reset f-nav-item">Archive</button>
      <button class="button-reset f-icon-button" aria-label="Open quick actions">+</button>
    </div>
    <span class="f-label">Windows workspace</span>
    <h2 class="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
    <p class="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
    <div class="f-divider"></div>
    <div class="cluster">
      <button class="button-reset f-tab">Mail</button>
      <button class="button-reset f-tab">Review</button>
    </div>
    <div class="f-panel f-stack">
      <label class="f-label row">
        Subject
        <input class="entry f-field focus:f-border-focus focus:f-ring" value="Quarterly sync notes" />
      </label>
      <label class="f-label row">
        Summary
        <textarea class="entry f-textarea focus:f-border-focus focus:f-ring" rows="4">Tighten the new preset rules and review the AI output guardrails.</textarea>
      </label>
      <label class="f-label row">
        Folder
        <select class="entry f-select focus:f-border-focus focus:f-ring">
          <option>Inbox</option>
          <option>Follow up</option>
        </select>
      </label>
      <div class="f-list-item">
        <span>Share with review group</span>
        <input type="checkbox" class="f-checkbox" aria-label="Share with review group" />
      </div>
      <div class="cluster">
        <button class="button-reset f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Preview panel</button>
        <button class="button-reset f-action-outlined focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Open inspector</button>
        <button class="button-reset f-action-primary hover:f-bg-hover-primary active:f-shadow-press">Send update</button>
      </div>
      <div class="f-dialog f-stack">
        <span class="f-label">Quick menu</span>
        <button class="button-reset f-menu-item hover:f-bg-hover-subtle">Move to archive</button>
        <button class="button-reset f-menu-item hover:f-bg-hover-subtle">Assign owner</button>
        <div class="f-tooltip">Fluent tooltip tone for calm guidance</div>
      </div>
      <div class="f-progress" aria-hidden="true"></div>
      <div class="cluster spinner-row">
        <div class="f-spinner" aria-hidden="true"></div>
        <span class="f-label">Syncing shell surfaces</span>
      </div>
      <div class="f-skeleton tall-skeleton" aria-hidden="true"></div>
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
          <div class="cluster compact-cluster">
            <span class="f-label">Pending</span>
            <span class="f-tag">Desktop only</span>
          </div>
          <button class="button-reset f-icon-button" aria-label="Open inbox sync row">+</button>
        </div>
      </div>
    </div>
  </section>
</template>


