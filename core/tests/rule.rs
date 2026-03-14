use motif_core::parse::parse_class_name;
use motif_core::rule::resolve_rule;
use motif_core::token::load_registry;

#[test]
fn resolves_whitelisted_fluent_rule() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("hover:f-bg-hover-primary").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert_eq!(rule.raw_class_name, "hover:f-bg-hover-primary");
    assert_eq!(rule.variants, parsed.variants);
    assert_eq!(rule.utility, "bg");
    assert_eq!(rule.value.as_deref(), Some("hover-primary"));
    assert_eq!(rule.declarations[0].property, "background-color");
    assert_eq!(rule.declarations[0].value, "#115ea3");
}

#[test]
fn resolves_material_on_primary_text_rule() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("m-text-on-primary").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert_eq!(rule.declarations[0].property, "color");
    assert_eq!(rule.declarations[0].value, "#ffffff");
}

#[test]
fn resolves_fluent_surface_with_mica_like_traits() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("f-surface").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert!(rule
        .declarations
        .iter()
        .any(|decl| decl.property == "backdrop-filter"));
    assert!(rule
        .declarations
        .iter()
        .any(|decl| decl.property == "border-color"));
    assert!(rule.declarations.iter().any(|decl| {
        decl.property == "background-image"
            && decl.value
                == "linear-gradient(135deg, rgba(255, 255, 255, 0.34), rgba(255, 255, 255, 0.08))"
    }));
}

#[test]
fn resolves_fluent_surface_alt_as_acrylic_panel() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("f-surface-alt").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert!(rule
        .declarations
        .iter()
        .any(|decl| decl.property == "backdrop-filter" && decl.value.contains("28px")));
    assert!(rule
        .declarations
        .iter()
        .any(|decl| decl.property == "box-shadow" && decl.value.contains("34px")));
    assert!(rule.declarations.iter().any(|decl| {
        decl.property == "transition-property"
            && decl.value == "background-color, border-color, box-shadow, backdrop-filter"
    }));
}

#[test]
fn resolves_material_primary_container() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("m-bg-primary-container").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert!(rule
        .declarations
        .iter()
        .any(|decl| decl.property == "background-color" && decl.value == "#d3e3fd"));
    assert!(rule
        .declarations
        .iter()
        .any(|decl| decl.property == "color" && decl.value == "#041e49"));
}

#[test]
fn resolves_workspace_preset_utilities() {
    let tokens = load_registry().unwrap();

    let fluent_panel = resolve_rule(&parse_class_name("f-panel").unwrap(), &tokens).unwrap();
    assert!(fluent_panel.declarations.iter().any(
        |decl| decl.property == "background-color" && decl.value == "rgba(255, 255, 255, 0.74)"
    ));
    assert!(fluent_panel
        .declarations
        .iter()
        .any(|decl| decl.property == "background-image" && decl.value.contains("0.44")));
    assert!(fluent_panel
        .declarations
        .iter()
        .any(|decl| decl.property == "backdrop-filter" && decl.value.contains("24px")));

    let fluent_action =
        resolve_rule(&parse_class_name("f-action-subtle").unwrap(), &tokens).unwrap();
    assert!(fluent_action
        .declarations
        .iter()
        .any(|decl| decl.property == "border" && decl.value.contains("rgba(255, 255, 255, 0.98)")));
    assert!(fluent_action
        .declarations
        .iter()
        .any(|decl| decl.property == "box-shadow" && decl.value.contains("20px")));
    assert!(fluent_action.declarations.iter().any(|decl| {
        decl.property == "transition-property"
            && decl.value == "background-color, border-color, box-shadow, transform"
    }));

    let fluent_press = resolve_rule(&parse_class_name("f-shadow-press").unwrap(), &tokens).unwrap();
    assert!(fluent_press
        .declarations
        .iter()
        .any(|decl| decl.property == "box-shadow" && decl.value.contains("inset 0 1px 1px")));

    let fluent_hover_subtle = resolve_rule(
        &parse_class_name("hover:f-bg-hover-subtle").unwrap(),
        &tokens,
    )
    .unwrap();
    assert!(fluent_hover_subtle
        .declarations
        .iter()
        .any(
            |decl| decl.property == "background-color" && decl.value == "rgba(255, 255, 255, 0.82)"
        ));

    let fluent_border_focus =
        resolve_rule(&parse_class_name("focus:f-border-focus").unwrap(), &tokens).unwrap();
    assert!(fluent_border_focus
        .declarations
        .iter()
        .any(|decl| decl.property == "border" && decl.value == "rgba(15, 108, 189, 0.72)"));

    let material_hover = resolve_rule(
        &parse_class_name("hover:m-bg-hover-primary").unwrap(),
        &tokens,
    )
    .unwrap();
    assert!(material_hover
        .declarations
        .iter()
        .any(|decl| decl.property == "background-color" && decl.value == "#1765cc"));

    let material_press =
        resolve_rule(&parse_class_name("m-shadow-press").unwrap(), &tokens).unwrap();
    assert!(material_press
        .declarations
        .iter()
        .any(|decl| decl.property == "box-shadow" && decl.value.contains("inset 0 1px 0")));

    let material_ring = resolve_rule(&parse_class_name("m-ring").unwrap(), &tokens).unwrap();
    assert!(material_ring
        .declarations
        .iter()
        .any(|decl| decl.property == "box-shadow" && decl.value.contains("0 0 0 4px")));
    assert!(material_ring.declarations.iter().any(|decl| {
        decl.property == "transition-property"
            && decl.value == "background-color, color, box-shadow, border-color"
    }));

    let material_surface =
        resolve_rule(&parse_class_name("m-surface-container").unwrap(), &tokens).unwrap();
    assert!(material_surface
        .declarations
        .iter()
        .any(|decl| decl.property == "background-color" && decl.value == "#f5f7fb"));
    assert!(material_surface
        .declarations
        .iter()
        .any(|decl| decl.property == "background-image" && decl.value.contains("0.72")));
    assert!(material_surface
        .declarations
        .iter()
        .any(|decl| decl.property == "border" && decl.value == "1px solid #dbe3f0"));

    let material_action =
        resolve_rule(&parse_class_name("m-action-outlined").unwrap(), &tokens).unwrap();
    assert!(material_action
        .declarations
        .iter()
        .any(|decl| decl.property == "border" && decl.value == "1px solid #b6c3d6"));
    assert!(material_action
        .declarations
        .iter()
        .any(|decl| decl.property == "min-height" && decl.value == "2.5rem"));
}

#[test]
fn resolves_fluent_field_and_material_action_rules() {
    let tokens = load_registry().unwrap();

    let fluent_field = resolve_rule(&parse_class_name("f-field").unwrap(), &tokens).unwrap();
    assert!(fluent_field
        .declarations
        .iter()
        .any(|decl| decl.property == "border" && decl.value.contains("rgba(255, 255, 255, 0.96)")));
    assert!(fluent_field.declarations.iter().any(
        |decl| decl.property == "background-color" && decl.value == "rgba(255, 255, 255, 0.82)"
    ));
    assert!(fluent_field
        .declarations
        .iter()
        .any(|decl| decl.property == "background-image" && decl.value.contains("0.38")));
    assert!(fluent_field
        .declarations
        .iter()
        .any(|decl| decl.property == "backdrop-filter" && decl.value.contains("14px")));
    assert!(fluent_field
        .declarations
        .iter()
        .any(|decl| decl.property == "caret-color" && decl.value == "#0f6cbd"));

    let material_action =
        resolve_rule(&parse_class_name("m-action-tonal").unwrap(), &tokens).unwrap();
    assert!(material_action
        .declarations
        .iter()
        .any(|decl| decl.property == "background-color" && decl.value == "#d3e3fd"));
    assert!(material_action.declarations.iter().any(
        |decl| decl.property == "box-shadow" && decl.value.contains("rgba(26, 115, 232, 0.16)")
    ));

    let material_field = resolve_rule(&parse_class_name("m-field").unwrap(), &tokens).unwrap();
    assert!(material_field
        .declarations
        .iter()
        .any(|decl| decl.property == "background-image" && decl.value.contains("0.86")));
    assert!(material_field
        .declarations
        .iter()
        .any(|decl| decl.property == "caret-color" && decl.value == "#1a73e8"));
}

#[test]
fn leaves_unknown_rule_unresolved() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("f-bg-secondary").unwrap();
    assert!(resolve_rule(&parsed, &tokens).is_none());
}

#[test]
fn resolves_universal_size_utilities() {
    let tokens = load_registry().unwrap();

    let control = resolve_rule(&parse_class_name("ui-control-lg").unwrap(), &tokens).unwrap();
    assert!(control
        .declarations
        .iter()
        .any(|decl| decl.property == "min-height" && decl.value == "3.25rem"));
    assert!(control
        .declarations
        .iter()
        .any(|decl| decl.property == "padding" && decl.value == "0.95rem 1.2rem"));

    let radius = resolve_rule(&parse_class_name("ui-radius-pill").unwrap(), &tokens).unwrap();
    assert_eq!(radius.declarations[0].property, "border-radius");
    assert_eq!(radius.declarations[0].value, "999px");

    let text = resolve_rule(&parse_class_name("ui-text-sm").unwrap(), &tokens).unwrap();
    assert!(text
        .declarations
        .iter()
        .any(|decl| decl.property == "font-size" && decl.value == "0.875rem"));
}

#[test]
fn resolves_selection_controls_for_both_presets() {
    let tokens = load_registry().unwrap();

    let fluent_checkbox = resolve_rule(&parse_class_name("f-checkbox").unwrap(), &tokens).unwrap();
    assert!(fluent_checkbox
        .declarations
        .iter()
        .any(|decl| decl.property == "appearance" && decl.value == "none"));
    assert!(fluent_checkbox
        .declarations
        .iter()
        .any(|decl| decl.property == "inline-size" && decl.value == "1.125rem"));
    assert!(fluent_checkbox
        .declarations
        .iter()
        .any(|decl| decl.property == "border-radius" && decl.value == "6px"));

    let material_radio = resolve_rule(&parse_class_name("m-radio").unwrap(), &tokens).unwrap();
    assert!(material_radio
        .declarations
        .iter()
        .any(|decl| decl.property == "border-radius" && decl.value == "999px"));

    let material_switch = resolve_rule(&parse_class_name("m-switch").unwrap(), &tokens).unwrap();
    assert!(material_switch
        .declarations
        .iter()
        .any(|decl| decl.property == "inline-size" && decl.value == "2.5rem"));
    assert!(material_switch
        .declarations
        .iter()
        .any(|decl| decl.property == "background-color" && decl.value == "#f5f7fb"));
}

#[test]
fn resolves_second_batch_component_semantics() {
    let tokens = load_registry().unwrap();

    let fluent_textarea = resolve_rule(&parse_class_name("f-textarea").unwrap(), &tokens).unwrap();
    assert!(fluent_textarea
        .declarations
        .iter()
        .any(|decl| decl.property == "resize" && decl.value == "vertical"));

    let material_select = resolve_rule(&parse_class_name("m-select").unwrap(), &tokens).unwrap();
    assert!(material_select
        .declarations
        .iter()
        .any(|decl| decl.property == "appearance" && decl.value == "none"));

    let material_tab = resolve_rule(&parse_class_name("m-tab").unwrap(), &tokens).unwrap();
    assert!(material_tab
        .declarations
        .iter()
        .any(|decl| decl.property == "border-radius" && decl.value == "999px"));

    let fluent_dialog = resolve_rule(&parse_class_name("f-dialog").unwrap(), &tokens).unwrap();
    assert!(fluent_dialog
        .declarations
        .iter()
        .any(|decl| decl.property == "max-width" && decl.value == "32rem"));

    let material_list_item =
        resolve_rule(&parse_class_name("m-list-item").unwrap(), &tokens).unwrap();
    assert!(material_list_item
        .declarations
        .iter()
        .any(|decl| decl.property == "min-height" && decl.value == "3rem"));

    let fluent_menu_item =
        resolve_rule(&parse_class_name("f-menu-item").unwrap(), &tokens).unwrap();
    assert!(fluent_menu_item
        .declarations
        .iter()
        .any(|decl| decl.property == "cursor" && decl.value == "pointer"));
}

#[test]
fn resolves_third_batch_navigation_and_feedback_semantics() {
    let tokens = load_registry().unwrap();

    let fluent_icon = resolve_rule(&parse_class_name("f-icon-button").unwrap(), &tokens).unwrap();
    assert!(fluent_icon
        .declarations
        .iter()
        .any(|decl| decl.property == "inline-size" && decl.value == "2.5rem"));

    let material_nav = resolve_rule(&parse_class_name("m-nav-item").unwrap(), &tokens).unwrap();
    assert!(material_nav
        .declarations
        .iter()
        .any(|decl| decl.property == "background-color" && decl.value == "transparent"));

    let material_badge = resolve_rule(&parse_class_name("m-badge").unwrap(), &tokens).unwrap();
    assert!(material_badge
        .declarations
        .iter()
        .any(|decl| decl.property == "background-color" && decl.value == "#d3e3fd"));

    let fluent_tooltip = resolve_rule(&parse_class_name("f-tooltip").unwrap(), &tokens).unwrap();
    assert!(fluent_tooltip
        .declarations
        .iter()
        .any(|decl| decl.property == "box-shadow"));

    let material_banner = resolve_rule(&parse_class_name("m-banner").unwrap(), &tokens).unwrap();
    assert!(material_banner
        .declarations
        .iter()
        .any(|decl| decl.property == "justify-content" && decl.value == "space-between"));
}

#[test]
fn resolves_fourth_batch_shell_and_identity_semantics() {
    let tokens = load_registry().unwrap();

    let fluent_drawer = resolve_rule(&parse_class_name("f-drawer").unwrap(), &tokens).unwrap();
    assert!(fluent_drawer
        .declarations
        .iter()
        .any(|decl| decl.property == "min-width" && decl.value == "16rem"));

    let material_toast = resolve_rule(&parse_class_name("m-toast").unwrap(), &tokens).unwrap();
    assert!(material_toast
        .declarations
        .iter()
        .any(|decl| decl.property == "box-shadow"));

    let material_segmented =
        resolve_rule(&parse_class_name("m-segmented-button").unwrap(), &tokens).unwrap();
    assert!(material_segmented
        .declarations
        .iter()
        .any(|decl| decl.property == "min-height" && decl.value == "2.5rem"));

    let fluent_search =
        resolve_rule(&parse_class_name("f-search-field").unwrap(), &tokens).unwrap();
    assert!(fluent_search
        .declarations
        .iter()
        .any(|decl| decl.property == "padding-left" && decl.value == "2.5rem"));

    let material_breadcrumb =
        resolve_rule(&parse_class_name("m-breadcrumb-item").unwrap(), &tokens).unwrap();
    assert!(material_breadcrumb
        .declarations
        .iter()
        .any(|decl| decl.property == "display" && decl.value == "inline-flex"));

    let fluent_avatar = resolve_rule(&parse_class_name("f-avatar").unwrap(), &tokens).unwrap();
    assert!(fluent_avatar
        .declarations
        .iter()
        .any(|decl| decl.property == "border-radius" && decl.value == "999px"));
}

#[test]
fn resolves_fifth_batch_loading_and_empty_semantics() {
    let tokens = load_registry().unwrap();

    let fluent_progress = resolve_rule(&parse_class_name("f-progress").unwrap(), &tokens).unwrap();
    assert!(fluent_progress
        .declarations
        .iter()
        .any(|decl| decl.property == "block-size" && decl.value == "0.5rem"));

    let material_spinner = resolve_rule(&parse_class_name("m-spinner").unwrap(), &tokens).unwrap();
    assert!(material_spinner.declarations.iter().any(
        |decl| decl.property == "animation" && decl.value == "motif-spin 0.9s linear infinite"
    ));

    let fluent_skeleton = resolve_rule(&parse_class_name("f-skeleton").unwrap(), &tokens).unwrap();
    assert!(fluent_skeleton
        .declarations
        .iter()
        .any(|decl| decl.property == "background-size" && decl.value == "200% 100%"));

    let material_empty =
        resolve_rule(&parse_class_name("m-empty-state").unwrap(), &tokens).unwrap();
    assert!(material_empty
        .declarations
        .iter()
        .any(|decl| decl.property == "display" && decl.value == "grid"));

    let fluent_sheet = resolve_rule(&parse_class_name("f-sheet").unwrap(), &tokens).unwrap();
    assert!(fluent_sheet
        .declarations
        .iter()
        .any(|decl| decl.property == "max-width" && decl.value == "28rem"));
}

#[test]
fn resolves_sixth_batch_structured_data_semantics() {
    let tokens = load_registry().unwrap();

    let fluent_accordion =
        resolve_rule(&parse_class_name("f-accordion-item").unwrap(), &tokens).unwrap();
    assert!(fluent_accordion
        .declarations
        .iter()
        .any(|decl| decl.property == "display" && decl.value == "grid"));

    let material_table = resolve_rule(&parse_class_name("m-table-row").unwrap(), &tokens).unwrap();
    assert!(material_table
        .declarations
        .iter()
        .any(|decl| decl.property == "grid-template-columns"));
}

#[test]
fn resolves_seventh_batch_identity_and_navigation_groups() {
    let tokens = load_registry().unwrap();

    let fluent_breadcrumb =
        resolve_rule(&parse_class_name("f-breadcrumb").unwrap(), &tokens).unwrap();
    assert!(fluent_breadcrumb
        .declarations
        .iter()
        .any(|decl| decl.property == "flex-wrap" && decl.value == "wrap"));

    let material_persona = resolve_rule(&parse_class_name("m-persona").unwrap(), &tokens).unwrap();
    assert!(material_persona
        .declarations
        .iter()
        .any(|decl| decl.property == "display" && decl.value == "flex"));
}

#[test]
fn resolves_eighth_batch_structured_states() {
    let tokens = load_registry().unwrap();

    let fluent_open =
        resolve_rule(&parse_class_name("f-accordion-item-open").unwrap(), &tokens).unwrap();
    assert!(fluent_open
        .declarations
        .iter()
        .any(|decl| decl.property == "background-image"));

    let material_selected =
        resolve_rule(&parse_class_name("m-table-row-selected").unwrap(), &tokens).unwrap();
    assert!(material_selected
        .declarations
        .iter()
        .any(|decl| decl.property == "border-color" && decl.value == "#1a73e8"));
}

#[test]
fn resolves_ninth_batch_chip_and_sheet_variants() {
    let tokens = load_registry().unwrap();

    let fluent_chip = resolve_rule(&parse_class_name("f-chip").unwrap(), &tokens).unwrap();
    assert!(fluent_chip
        .declarations
        .iter()
        .any(|decl| decl.property == "min-height" && decl.value == "2rem"));

    let material_sheet_side =
        resolve_rule(&parse_class_name("m-sheet-side").unwrap(), &tokens).unwrap();
    assert!(material_sheet_side
        .declarations
        .iter()
        .any(|decl| decl.property == "border-top-right-radius" && decl.value == "0"));

    let fluent_sheet_bottom =
        resolve_rule(&parse_class_name("f-sheet-bottom").unwrap(), &tokens).unwrap();
    assert!(fluent_sheet_bottom
        .declarations
        .iter()
        .any(|decl| decl.property == "max-width" && decl.value == "36rem"));
}

#[test]
fn resolves_tenth_batch_table_container_and_tag_semantics() {
    let tokens = load_registry().unwrap();

    let fluent_table = resolve_rule(&parse_class_name("f-table").unwrap(), &tokens).unwrap();
    assert!(fluent_table
        .declarations
        .iter()
        .any(|decl| decl.property == "display" && decl.value == "grid"));

    let material_tag = resolve_rule(&parse_class_name("m-tag").unwrap(), &tokens).unwrap();
    assert!(material_tag
        .declarations
        .iter()
        .any(|decl| decl.property == "min-height" && decl.value == "1.5rem"));
}

#[test]
fn resolves_eleventh_batch_table_and_accordion_headers() {
    let tokens = load_registry().unwrap();

    let fluent_table_header =
        resolve_rule(&parse_class_name("f-table-header").unwrap(), &tokens).unwrap();
    assert!(fluent_table_header
        .declarations
        .iter()
        .any(|decl| decl.property == "grid-template-columns"));

    let material_accordion_header =
        resolve_rule(&parse_class_name("m-accordion-header").unwrap(), &tokens).unwrap();
    assert!(material_accordion_header
        .declarations
        .iter()
        .any(|decl| decl.property == "justify-content" && decl.value == "space-between"));
}

#[test]
fn resolves_twelfth_batch_table_cells_and_accordion_content() {
    let tokens = load_registry().unwrap();

    let fluent_table_cell =
        resolve_rule(&parse_class_name("f-table-cell").unwrap(), &tokens).unwrap();
    assert!(fluent_table_cell
        .declarations
        .iter()
        .any(|decl| decl.property == "font-size"));

    let material_accordion_content =
        resolve_rule(&parse_class_name("m-accordion-content").unwrap(), &tokens).unwrap();
    assert!(material_accordion_content
        .declarations
        .iter()
        .any(|decl| decl.property == "gap" && decl.value == "0.5rem"));
}
