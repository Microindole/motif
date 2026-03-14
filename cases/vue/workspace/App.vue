<template>
  <div class="f-drawer f-stack"><div class="f-persona"><div class="f-avatar">A</div><div><div class="f-label">Alex</div><nav class="f-breadcrumb"><div class="f-breadcrumb-item">Workspace</div><div class="f-breadcrumb-item">Inbox</div></nav></div></div><input class="f-search-field" value="Search drafts" /><button class="f-segmented-button">Focused</button><div class="f-toast">Synced</div></div>
  <div class="f-banner"><span class="f-badge">Synced</span><button class="f-icon-button">+</button></div>
  <button class="f-nav-item">Inbox</button>
  <div class="f-surface f-stack">
    <span class="f-label">Windows workspace</span>
    <h2 class="f-title f-text-primary">Inbox draft with quiet acrylic controls</h2>
    <p class="f-body f-text-muted">The shell stays soft, the divider stays subtle, and the primary action stays restrained until you need it.</p>
    <div class="f-divider"></div>
    <button class="f-tab">Mail</button>
    <div class="f-panel f-stack">
      <label class="f-label">
        Subject
        <input class="f-field focus:f-border-focus focus:f-ring" value="Quarterly sync notes" />
      </label>
      <textarea class="f-textarea focus:f-border-focus focus:f-ring">Draft body</textarea>
      <select class="f-select focus:f-border-focus focus:f-ring"><option>Inbox</option></select>
      <div class="f-list-item"><span>Share</span><input type="checkbox" class="f-checkbox" /></div>
      <button class="f-action-subtle focus:f-ring hover:f-bg-hover-subtle active:f-shadow-press">Preview panel</button>
      <button class="f-action-primary hover:f-bg-hover-primary active:f-shadow-press">Send update</button>
      <div class="f-dialog f-stack"><button class="f-menu-item">Move</button></div>
      <div class="f-tooltip">Tip</div>
      <div class="f-progress"></div>
      <div class="f-spinner"></div>
      <div class="f-skeleton"></div>
      <div class="f-empty-state">No pinned layouts</div>
      <aside class="f-sheet f-stack">Inspector</aside>
      <div class="f-accordion-item">Rules</div>
      <div class="f-table-row"><span>Inbox sync</span><span>Pending</span><button class="f-icon-button">+</button></div>
    </div>
  </div>
  <div class="m-banner"><span class="m-badge">Ready</span><button class="m-icon-button">+</button></div>
  <div class="m-drawer f-stack"><div class="m-persona"><div class="m-avatar">R</div><div><div class="m-label">Riley</div><nav class="m-breadcrumb"><div class="m-breadcrumb-item">Workspace</div><div class="m-breadcrumb-item">Board</div></nav></div></div><input class="m-search-field" value="Search tasks" /><button class="m-segmented-button">Board</button><div class="m-toast">Ready</div></div>
  <button class="m-nav-item">Board</button>
  <div class="m-surface f-stack dark:m-elevation-1">
    <span class="m-label">Google workspace</span>
    <h2 class="m-title m-text-primary">Task board with explicit action hierarchy</h2>
    <p class="m-body m-text-muted">The container system is clearer, the label scale is tighter, and the actions feel more direct.</p>
    <div class="m-divider"></div>
    <button class="m-tab">Board</button>
    <div class="m-surface-container f-stack">
      <label class="m-label">
        Search
        <input class="m-field focus:m-border-focus focus:m-ring" value="Ship the preset charter and workspace demo" />
      </label>
      <textarea class="m-textarea focus:m-border-focus focus:m-ring">Board note</textarea>
      <select class="m-select focus:m-border-focus focus:m-ring"><option>Ready</option></select>
      <div class="m-list-item"><span>Sync</span><button class="m-switch">Toggle</button></div>
      <button class="m-action-outlined hover:m-bg-hover-surface active:m-shadow-press">Open filter</button>
      <button class="m-action-tonal hover:m-bg-hover-container active:m-shadow-press">Save draft</button>
      <button class="m-action-primary focus:m-ring hover:m-bg-hover-primary active:m-shadow-press">Create task</button>
      <div class="m-dialog f-stack"><button class="m-menu-item">Duplicate</button></div>
      <div class="m-tooltip">Tip</div>
      <div class="m-progress"></div>
      <div class="m-spinner"></div>
      <div class="m-skeleton"></div>
      <div class="m-empty-state">No suggested tasks</div>
      <aside class="m-sheet f-stack">Details</aside>
      <div class="m-accordion-item">Constraints</div>
      <div class="m-table-row"><span>Board publish</span><span>Today</span><button class="m-icon-button">+</button></div>
    </div>
  </div>
</template>





