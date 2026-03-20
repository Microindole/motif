pub(crate) fn assert_case_selectors(css: &str) {
    assert!(css.contains(".f-stack {"));
    assert!(css.contains(".focus\\:f-ring:focus {") || css.contains(".focus\\:m-ring:focus {"));
    assert!(
        css.contains(".hover\\:f-bg-hover-primary:hover {")
            || css.contains(".hover\\:m-bg-hover-primary:hover {")
    );
    assert!(css.contains(".m-surface {"));
}

pub(crate) fn assert_demo_selectors(css: &str) {
    assert_case_selectors(css);
    assert!(css.contains(".f-text-primary {"));
}

pub(crate) fn assert_variant_selectors(css: &str) {
    assert!(css.contains(".hover\\:m-bg-hover-primary:hover {"));
    assert!(css.contains("background-color: #1765cc;"));
    assert!(css.contains(".active\\:m-shadow-press:active {"));
    assert!(css.contains(
        "box-shadow: 0 1px 2px rgba(60, 64, 67, 0.24), inset 0 1px 0 rgba(255, 255, 255, 0.18);"
    ));
    assert!(css.contains("@media (prefers-color-scheme: dark) {"));
    assert!(css.contains(".dark\\:m-elevation-1 {"));
    assert!(css.contains("box-shadow: 0 1px 2px rgba(60, 64, 67, 0.24);"));
    assert!(css.contains(".focus\\:m-ring:focus {"));
    assert!(css.contains(".ui-control-lg {"));
    assert!(css.contains(".ui-pad-xl {"));
    assert!(css.contains("padding: 1.5rem;"));
    assert!(css.contains(".ui-gap-xl {"));
    assert!(css.contains("gap: 1.25rem;"));
    assert!(css.contains(".ui-text-xs {"));
    assert!(css.contains("font-size: 0.75rem;"));
    assert!(css.contains(".ui-density-compact {"));
    assert!(css.contains("padding: 0.625rem;"));
    assert!(css.contains(".ui-density-comfortable {"));
    assert!(css.contains(".ui-stack-inline {"));
    assert!(css.contains("flex-wrap: wrap;"));
    assert!(css.contains(".ui-stack-center {"));
    assert!(css.contains("min-height: 3.25rem;"));
    assert!(css.contains(".ui-radius-pill {"));
    assert!(css.contains("border-radius: 999px;"));
    assert!(css.contains(".f-checkbox {"));
    assert!(css.contains(".m-radio {"));
    assert!(css.contains(".m-switch {"));
    assert!(css.contains("inline-size: 2.5rem;"));
}

pub(crate) fn assert_theme_selectors(css: &str) {
    assert!(css.contains(".f-surface {"));
    assert!(css.contains("background-image: linear-gradient(135deg, rgba(255, 255, 255, 0.34), rgba(255, 255, 255, 0.08));"));
    assert!(css.contains("backdrop-filter: blur(18px) saturate(1.15);"));
    assert!(css.contains(".f-surface-alt {"));
    assert!(css.contains("backdrop-filter: blur(28px) saturate(1.25);"));
    assert!(css.contains(".m-surface-variant {"));
    assert!(css.contains("background-image: linear-gradient(180deg, rgba(255, 255, 255, 0.42), rgba(211, 227, 253, 0.24));"));
    assert!(css.contains("background-color: #e8eef9;"));
    assert!(css.contains(".m-bg-primary-container {"));
    assert!(css.contains("background-color: #d3e3fd;"));
    assert!(css.contains("color: #041e49;"));
}

pub(crate) fn assert_workspace_selectors(css: &str) {
    assert_workspace_base_selectors(css);
    assert_workspace_component_matrix(css);
    assert_workspace_layout_metrics(css);
    assert_workspace_material_states(css);
}

// Keep the workspace selector checks grouped by page concern so the test still
// reads like a fixture contract instead of scattering hundreds of assertions
// across the call sites.
fn assert_workspace_base_selectors(css: &str) {
    assert!(css.contains(".f-label {"));
    assert!(css.contains(".f-divider {"));
    assert!(css.contains(".f-field {"));
    assert!(css.contains("caret-color: #0f6cbd;"));
    assert!(css.contains(".f-panel {"));
    assert!(css.contains(".f-action-primary {"));
    assert!(
        css.contains("transition-property: background-color, border-color, box-shadow, transform;")
    );
    assert!(css.contains(".f-action-subtle {"));
    assert!(css.contains(".f-action-outlined {"));
    assert!(css.contains(".hover\\:f-bg-hover-subtle:hover {"));
    assert!(css.contains(".focus\\:f-border-focus:focus {"));
    assert!(css.contains(".m-label {"));
    assert!(css.contains(".m-divider {"));
    assert!(css.contains(".m-field {"));
    assert!(css.contains("caret-color: #1a73e8;"));
    assert!(css.contains(".m-surface-container {"));
    assert!(css.contains("background-image: linear-gradient(180deg, rgba(255, 255, 255, 0.72), rgba(232, 240, 254, 0.24));"));
    assert!(css.contains(".m-action-primary {"));
    assert!(css.contains(".m-action-tonal {"));
    assert!(css.contains(".m-action-outlined {"));
}

fn assert_workspace_component_matrix(css: &str) {
    assert_workspace_form_and_overlay_selectors(css);
    assert_workspace_navigation_and_identity_selectors(css);
    assert_workspace_feedback_and_data_selectors(css);
}

fn assert_workspace_form_and_overlay_selectors(css: &str) {
    for selector in [
        ".f-textarea {",
        ".m-textarea {",
        ".f-select {",
        ".m-select {",
        ".f-tab {",
        ".m-tab {",
        ".f-dialog {",
        ".m-dialog {",
        ".f-list-item {",
        ".m-list-item {",
        ".f-menu-item {",
        ".m-menu-item {",
    ] {
        assert!(css.contains(selector));
    }
}

fn assert_workspace_navigation_and_identity_selectors(css: &str) {
    for selector in [
        ".f-icon-button {",
        ".m-icon-button {",
        ".f-nav-item {",
        ".m-nav-item {",
        ".f-badge {",
        ".m-badge {",
        ".f-chip {",
        ".m-chip {",
        ".f-tag {",
        ".m-tag {",
        ".f-tooltip {",
        ".m-tooltip {",
        ".f-banner {",
        ".m-banner {",
        ".f-drawer {",
        ".m-drawer {",
        ".f-toast {",
        ".m-toast {",
        ".f-segmented-button {",
        ".m-segmented-button {",
        ".f-search-field {",
        ".m-search-field {",
        ".f-breadcrumb-item {",
        ".m-breadcrumb-item {",
        ".f-breadcrumb {",
        ".m-breadcrumb {",
        ".f-avatar {",
        ".m-avatar {",
        ".f-persona {",
        ".m-persona {",
    ] {
        assert!(css.contains(selector));
    }
}

fn assert_workspace_feedback_and_data_selectors(css: &str) {
    for selector in [
        ".f-progress {",
        ".m-progress {",
        ".f-spinner {",
        ".m-spinner {",
        ".f-skeleton {",
        ".m-skeleton {",
        ".f-empty-state {",
        ".m-empty-state {",
        ".f-sheet-side {",
        ".m-sheet-side {",
        ".f-sheet-bottom {",
        ".m-sheet-bottom {",
        ".f-accordion-item-open {",
        ".m-accordion-item-open {",
        ".f-table-row-selected {",
        ".m-table-row-selected {",
        ".f-table {",
        ".m-table {",
        ".f-table-header {",
        ".m-table-header {",
        ".f-table-cell {",
        ".m-table-cell {",
        ".f-accordion-header {",
        ".m-accordion-header {",
        ".f-accordion-content {",
        ".m-accordion-content {",
    ] {
        assert!(css.contains(selector));
    }
}

fn assert_workspace_layout_metrics(css: &str) {
    for value in [
        "min-width: 16rem;",
        "padding-left: 2.5rem;",
        "max-width: 32rem;",
        "max-width: 28rem;",
        "max-width: 36rem;",
        "resize: vertical;",
        "appearance: none;",
        "animation: motif-spin 0.9s linear infinite;",
        "animation: motif-shimmer 1.4s ease-in-out infinite;",
        "grid-template-columns: minmax(0, 2fr) minmax(0, 1fr) auto;",
        "flex-wrap: wrap;",
        "border-color: #1a73e8;",
        "border-top-right-radius: 0;",
        "min-height: 1.5rem;",
        "justify-content: space-between;",
    ] {
        assert!(css.contains(value));
    }
}

fn assert_workspace_material_states(css: &str) {
    assert!(css.contains(".hover\\:m-bg-hover-primary:hover {"));
    assert!(css.contains(".hover\\:m-bg-hover-container:hover {"));
    assert!(css.contains(".hover\\:m-bg-hover-surface:hover {"));
    assert!(css.contains(".active\\:m-shadow-press:active {"));
    assert!(css.contains(".focus\\:m-border-focus:focus {"));
    assert!(css.contains(".focus\\:m-ring:focus {"));
}
