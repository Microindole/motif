use motif_core::parse::parse_class_name;
use motif_core::rule::resolve_rule;
use motif_core::token::load_registry;

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
