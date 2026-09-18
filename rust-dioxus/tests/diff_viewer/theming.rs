//! Themes: the palettes the sheet declares, the properties its rules read,
//! the layer they sit in, and the theme a viewer reports.
//!
//! Two paths reach this material. The stylesheet never appears in the rendered
//! tree, because `document::Style` inserts it through the document and returns
//! an empty node, so what the sheet declares is asserted over its own text.
//! What the viewer reports, and which gutters answer a click, are properties of
//! the markup and are read from the rendered tree.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::document::{Document, Eval, NoOpDocument, StyleProps};
use dioxus::prelude::*;
use dioxus_diff_viewer::styling_hooks::{DXDIFF__GUTTER, DXDIFF__GUTTER_CLICKABLE};
use dioxus_diff_viewer::{DiffTheme, DiffViewer, LineNumberClick};

use crate::mounted_app::MountedApp;

/// The stylesheet the crate embeds, read from the same file the component
/// includes, so the two cannot drift apart.
const STYLESHEET: &str = include_str!("../../src/dxdiff.css");

/// The reference implementation, whose two palettes the themes carry.
const REFERENCE_STYLES: &str = include_str!("../../../src/styles.ts");

/// Each reference palette name against the `--dxdiff-` property carrying its
/// color. This table is the claim REQT-78dg0g6a2h makes: the light and dark
/// sets carry the package's colors for the same elements.
const CARRIED_COLORS: [(&str, &str); 24] = [
    ("diffViewerBackground", "--dxdiff-background"),
    ("diffViewerColor", "--dxdiff-text-color"),
    ("addedBackground", "--dxdiff-added-background"),
    ("addedColor", "--dxdiff-added-text-color"),
    ("removedBackground", "--dxdiff-removed-background"),
    ("removedColor", "--dxdiff-removed-text-color"),
    ("wordAddedBackground", "--dxdiff-added-token-background"),
    ("wordRemovedBackground", "--dxdiff-removed-token-background"),
    ("addedGutterBackground", "--dxdiff-added-gutter-background"),
    (
        "removedGutterBackground",
        "--dxdiff-removed-gutter-background",
    ),
    ("gutterBackground", "--dxdiff-gutter-background"),
    ("gutterBackgroundDark", "--dxdiff-gutter-hover-background"),
    ("highlightBackground", "--dxdiff-highlighted-background"),
    (
        "highlightGutterBackground",
        "--dxdiff-highlighted-gutter-background",
    ),
    (
        "codeFoldGutterBackground",
        "--dxdiff-fold-gutter-background",
    ),
    ("codeFoldBackground", "--dxdiff-fold-background"),
    ("emptyLineBackground", "--dxdiff-empty-background"),
    ("gutterColor", "--dxdiff-gutter-text-color"),
    ("addedGutterColor", "--dxdiff-added-gutter-text-color"),
    ("removedGutterColor", "--dxdiff-removed-gutter-text-color"),
    ("codeFoldContentColor", "--dxdiff-fold-text-color"),
    ("diffViewerTitleBackground", "--dxdiff-title-background"),
    ("diffViewerTitleColor", "--dxdiff-title-text-color"),
    ("diffViewerTitleBorderColor", "--dxdiff-title-border-color"),
];

/// The colors each theme names for the elements the package does not render.
/// REQT-8y5fp2aarz (Chip and arrow color).
const OWN_COLORS: [&str; 4] = [
    "--dxdiff-chip-background",
    "--dxdiff-chip-text-color",
    "--dxdiff-chip-border-color",
    "--dxdiff-arrow-color",
];

/// A color as both files can be compared: lowercase, and the one CSS color
/// keyword the reference spells out written as its hex form.
fn as_comparable_color(value: &str) -> String {
    let value = value.trim().to_lowercase();
    match value.as_str() {
        "white" => "#fff".to_owned(),
        _ => {
            assert!(
                value.starts_with('#'),
                "{value} is neither a hex color nor a keyword this comparison knows"
            );
            value
        }
    }
}

/// The reference's palette for one theme, as name-to-color pairs read from the
/// `light:` or `dark:` block of `src/styles.ts`.
fn reference_palette(theme: &str) -> Vec<(String, String)> {
    let block_start = REFERENCE_STYLES
        .find(&format!("\n\t\t{theme}: {{"))
        .unwrap_or_else(|| panic!("the reference declares a {theme} palette"));
    let block = &REFERENCE_STYLES[block_start..];
    let block_end = block
        .find("...(overrideVariables")
        .expect("the palette block ends with its override spread");
    block[..block_end]
        .lines()
        .filter_map(|line| {
            let line = line.trim().trim_end_matches(',');
            let (name, value) = line.split_once(':')?;
            let value = value.trim();
            let color = value.strip_prefix('\'')?.strip_suffix('\'')?;
            Some((name.trim().to_owned(), as_comparable_color(color)))
        })
        .collect()
}

/// The sheet with its comments removed, so a scan reads declarations alone.
fn stylesheet_without_comments() -> String {
    let mut remaining = STYLESHEET;
    let mut kept = String::with_capacity(STYLESHEET.len());
    while let Some(start) = remaining.find("/*") {
        kept.push_str(&remaining[..start]);
        let after = &remaining[start + 2..];
        let end = after.find("*/").expect("every comment is closed");
        remaining = &after[end + 2..];
    }
    kept.push_str(remaining);
    kept
}

/// The run of `--dxdiff-` declarations following a marker, as name-to-value
/// pairs. The marker names the palette block rather than reproducing its
/// selector line, so the sheet stays free to lay its blocks out as it likes.
fn declared_palette(marker: &str) -> Vec<(String, String)> {
    let sheet = stylesheet_without_comments();
    let block_start = sheet
        .find(marker)
        .unwrap_or_else(|| panic!("the sheet holds a palette block opened by {marker}"));
    let mut declared = Vec::new();
    for line in sheet[block_start + marker.len()..].lines() {
        let line = line.trim().trim_end_matches(';');
        match line.split_once(':') {
            Some((name, value)) if name.trim().starts_with("--dxdiff-") => {
                declared.push((name.trim().to_owned(), as_comparable_color(value)));
            }
            // Anything else closes the run, once the run has started.
            _ if declared.is_empty() => continue,
            _ => break,
        }
    }
    assert!(!declared.is_empty(), "{marker} opens a palette block");
    declared
}

/// The palette blocks, named by what opens each.
const PALETTE_BLOCKS: [(&str, &str); 3] = [
    ("light, at the document root", ":root,"),
    (
        "dark, under the reader's preference",
        "@media (prefers-color-scheme: dark) {",
    ),
    (
        "dark, on a viewer asked for it",
        ".dxdiff-viewer[data-dxdiff-theme=\"dark\"] {",
    ),
];

/// REQT-78dg0g6a2h (Theme palettes): the light and dark sets carry the colors
/// the package uses for the same elements.
#[test]
fn each_theme_carries_the_package_s_color_for_every_element_it_names() {
    let sheet = stylesheet_without_comments();
    let light_block = sheet
        .find(PALETTE_BLOCKS[0].1)
        .expect("the root declares the light palette");
    let light_selectors = &sheet[light_block..][..sheet[light_block..]
        .find('{')
        .expect("the light palette block opens")];
    assert!(
        light_selectors.contains(r#".dxdiff-viewer[data-dxdiff-theme="light"]"#),
        "a viewer asked for light reads the same set the root defaults to"
    );

    for (theme, selectors) in [
        ("light", vec![PALETTE_BLOCKS[0].1]),
        ("dark", vec![PALETTE_BLOCKS[1].1, PALETTE_BLOCKS[2].1]),
    ] {
        let reference = reference_palette(theme);
        assert_eq!(
            reference.len(),
            CARRIED_COLORS.len(),
            "the {theme} palette of the reference holds a name this test does not carry"
        );

        for selector in selectors {
            let declared = declared_palette(selector);
            for (reference_name, property) in CARRIED_COLORS {
                let expected = reference
                    .iter()
                    .find(|(name, _)| name == reference_name)
                    .map(|(_, color)| color)
                    .unwrap_or_else(|| {
                        panic!("the reference's {theme} palette names {reference_name}")
                    });
                let carried = declared
                    .iter()
                    .find(|(name, _)| name == property)
                    .map(|(_, color)| color)
                    .unwrap_or_else(|| panic!("the {theme} palette declares {property}"));
                assert_eq!(
                    carried, expected,
                    "{property} should carry the {theme} {reference_name} of the package"
                );
            }
        }
    }
}

/// REQT-8y5fp2aarz (Chip and arrow color): the package renders no chips and no
/// arrow, so each theme names their colors rather than carrying them.
#[test]
fn each_theme_names_the_colors_of_the_elements_the_package_does_not_render() {
    for (block, selector) in PALETTE_BLOCKS {
        let declared = declared_palette(selector);
        for property in OWN_COLORS {
            assert!(
                declared.iter().any(|(name, _)| name == property),
                "the palette for {block} should name {property}"
            );
        }
    }

    let sheet = stylesheet_without_comments();
    for (element, property) in [
        (".dxdiff-line-ending-chip", "--dxdiff-chip-text-color"),
        (".dxdiff-whitespace-chip", "--dxdiff-chip-text-color"),
        (".dxdiff-line-ending-arrow", "--dxdiff-arrow-color"),
    ] {
        let rule_start = sheet
            .find(element)
            .unwrap_or_else(|| panic!("a rule targets {element}"));
        let rule = &sheet[rule_start..];
        let rule_end = rule.find('}').expect("the rule is closed");
        assert!(
            rule[..rule_end].contains(&format!("var({property})")),
            "{element} should read its color from {property}"
        );
    }
}

/// The properties coloring a line's background, which is what a chip or the
/// arrow ends up sitting on.
const LINE_BACKGROUNDS: [&str; 6] = [
    "--dxdiff-background",
    "--dxdiff-added-background",
    "--dxdiff-removed-background",
    "--dxdiff-empty-background",
    "--dxdiff-highlighted-background",
    "--dxdiff-fold-background",
];

/// A hex color's relative luminance, per the WCAG definition.
fn relative_luminance(color: &str) -> f64 {
    let digits = color.trim_start_matches('#');
    let digits: String = if digits.len() == 3 {
        digits.chars().flat_map(|digit| [digit, digit]).collect()
    } else {
        digits.to_owned()
    };
    let channel = |at: usize| {
        let raw = u8::from_str_radix(&digits[at..at + 2], 16)
            .unwrap_or_else(|_| panic!("{color} is a hex color")) as f64
            / 255.0;
        if raw <= 0.03928 {
            raw / 12.92
        } else {
            ((raw + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(0) + 0.7152 * channel(2) + 0.0722 * channel(4)
}

/// The contrast ratio between two hex colors, from 1.0 to 21.0.
fn contrast_ratio(one: &str, other: &str) -> f64 {
    let (one, other) = (relative_luminance(one), relative_luminance(other));
    (one.max(other) + 0.05) / (one.min(other) + 0.05)
}

/// REQT-8y5fp2aarz (Chip and arrow color): the colors each theme names for
/// chips and the arrow contrast with every line background they appear on.
///
/// A chip is a filled box with a border, so what separates it from the line is
/// its border, and what makes it readable is its text against its own fill. The
/// arrow is a bare glyph on the line itself. The floors are the WCAG ones for
/// each of those jobs: 3 for a graphical boundary, 4.5 for text.
#[test]
fn every_theme_s_chips_and_arrow_contrast_with_the_lines_they_sit_on() {
    for (theme, selector) in PALETTE_BLOCKS {
        let declared = declared_palette(selector);
        let color = |property: &str| {
            declared
                .iter()
                .find(|(name, _)| name == property)
                .map(|(_, color)| color.clone())
                .unwrap_or_else(|| panic!("the palette for {theme} names {property}"))
        };

        let chip_fill = color("--dxdiff-chip-background");
        let chip_text = color("--dxdiff-chip-text-color");
        let chip_border = color("--dxdiff-chip-border-color");
        let arrow = color("--dxdiff-arrow-color");

        let text_on_chip = contrast_ratio(&chip_text, &chip_fill);
        assert!(
            text_on_chip >= 4.5,
            "in {theme}, a chip's text reads at {text_on_chip:.2} against its own fill"
        );

        for line in LINE_BACKGROUNDS {
            let background = color(line);
            let edge = contrast_ratio(&chip_border, &background);
            assert!(
                edge >= 3.0,
                "in {theme}, a chip's edge reads at {edge:.2} against {line}"
            );
            let glyph = contrast_ratio(&arrow, &background);
            assert!(
                glyph >= 3.0,
                "in {theme}, the arrow reads at {glyph:.2} against {line}"
            );
        }
    }
}

/// The selector opening each block that declares `--dxdiff-` properties, in
/// the order the sheet states them.
fn palette_selectors() -> Vec<String> {
    let sheet = stylesheet_without_comments();
    let mut open_selectors: Vec<String> = Vec::new();
    let mut pending = String::new();
    let mut found: Vec<String> = Vec::new();

    for line in sheet.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(selector) = line.strip_suffix('{') {
            pending.push_str(selector.trim());
            open_selectors.push(pending.trim().to_owned());
            pending.clear();
        } else if line == "}" {
            open_selectors.pop();
        } else if line.starts_with("--dxdiff-") {
            let selector = open_selectors
                .last()
                .expect("a declaration sits inside a block")
                .clone();
            if !found.contains(&selector) {
                found.push(selector);
            }
        } else if line.ends_with(',') {
            // A selector list continued onto the next line.
            pending.push_str(line);
            pending.push(' ');
        }
    }
    found
}

/// REQT-kcm5ba45sp (Palette override): an app restyles the viewer, wholly or
/// one mounted viewer at a time, using only its own CSS.
///
/// The app-wide path needs the viewer to inherit its palette rather than
/// declare one, since a declaration on the element would beat an app's
/// assignment on any ancestor. So a viewer following the reader must match no
/// palette block, and only a viewer given an explicit theme may carry values of
/// its own, where an app's rules still reach it.
#[test]
fn an_app_reaches_every_viewer_it_did_not_give_an_explicit_theme() {
    let selectors = palette_selectors();
    assert!(
        !selectors.is_empty(),
        "the sheet declares its palettes somewhere"
    );

    for selector in &selectors {
        assert!(
            !selector.contains("auto"),
            "{selector} would put palette values on a viewer following the reader, \
             where they would beat an app's assignment on an ancestor"
        );
        let declares_at_the_root = selector.starts_with(":root");
        let declares_on_a_themed_viewer = selector
            .contains(r#".dxdiff-viewer[data-dxdiff-theme="light"]"#)
            || selector.contains(r#".dxdiff-viewer[data-dxdiff-theme="dark"]"#);
        assert!(
            declares_at_the_root || declares_on_a_themed_viewer,
            "{selector} declares palette values somewhere an app cannot predict"
        );
    }

    let themed_viewer_blocks = selectors
        .iter()
        .filter(|selector| selector.contains("data-dxdiff-theme"))
        .count();
    assert_eq!(
        themed_viewer_blocks, 2,
        "one block per explicit theme carries its values onto the viewer element, \
         which is what lets two mounted viewers differ"
    );
}

/// REQT-1accrb4jhp (Named colors): every theme color a rule reads comes from a
/// `dxdiff`-prefixed custom property, and every such property is defaulted at
/// the document root.
#[test]
fn every_color_a_rule_reads_is_a_property_defaulted_at_the_root() {
    let sheet = stylesheet_without_comments();
    let root_defaults: Vec<String> = declared_palette(PALETTE_BLOCKS[0].1)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    for line in sheet.lines() {
        let line = line.trim();
        if line.starts_with("--dxdiff-") {
            continue;
        }
        assert!(
            !line.contains('#'),
            "a rule states a color of its own rather than reading a property: {line}"
        );
    }

    let mut read_properties = Vec::new();
    let mut remaining = sheet.as_str();
    while let Some(start) = remaining.find("var(--dxdiff-") {
        let after = &remaining[start + 4..];
        let end = after.find(')').expect("every var() is closed");
        read_properties.push(after[..end].to_owned());
        remaining = &after[end..];
    }
    assert!(
        !read_properties.is_empty(),
        "the sheet's rules read their colors from properties"
    );
    for property in read_properties {
        assert!(
            root_defaults.contains(&property),
            "{property} is read by a rule but has no default at the document root"
        );
    }
}

/// REQT-ey9f1s27r1 (Yielding rules): the viewer's rules yield to an app's for
/// the same element whatever the specificity, so an app never needs
/// `!important`.
#[test]
fn every_rule_sits_in_the_crate_s_cascade_layer() {
    let sheet = stylesheet_without_comments();
    let layer_open = sheet
        .find("@layer dxdiff {")
        .expect("the sheet opens one cascade layer");
    assert!(
        sheet[..layer_open].trim().is_empty(),
        "nothing precedes the layer, so no rule sits outside it"
    );
    assert!(
        sheet[layer_open..].trim_end().ends_with('}'),
        "the layer closes at the end of the sheet"
    );
    assert_eq!(
        sheet.matches("@layer").count(),
        1,
        "one layer holds every rule, so no rule sits outside it"
    );
    assert!(
        !sheet.contains("!important"),
        "an important declaration would defeat the layer an app's rules win against"
    );
}

/// REQT-aaxrz33x1n (Line numbers under the pointer): the numbers of the row
/// under the pointer read at full strength, in every viewer, while the click
/// affordance stays on the gutters that answer a click (REQT-3928hx46s3).
#[test]
fn the_row_under_the_pointer_shows_its_line_numbers_at_full_strength() {
    let sheet = stylesheet_without_comments();

    let brightening = sheet
        .find(".dxdiff-row:hover .dxdiff-gutter pre")
        .expect("a rule brightens the numbers of the row under the pointer");
    let rule = &sheet[brightening..];
    let rule = &rule[..rule.find('}').expect("the rule is closed")];
    assert!(
        rule.contains("opacity: 1"),
        "the row-hover rule returns the numbers to full strength"
    );
    assert!(
        !rule.contains("cursor"),
        "hovering a row offers no click affordance of its own"
    );

    let affordance = sheet
        .find("var(--dxdiff-gutter-hover-background)")
        .expect("some rule reads the gutter hover background");
    let selector = &sheet[..sheet[..affordance].rfind('{').expect("the rule opens")];
    assert!(
        selector.trim_end().ends_with(".dxdiff-gutter-clickable:hover"),
        "the hover background reaches only the gutters that answer a click"
    );
}

/// Records what the component hands the document, so a test can read what was
/// delivered rather than what was rendered.
struct RecordingDocument;

thread_local! {
    static DELIVERED_STYLES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

impl Document for RecordingDocument {
    fn eval(&self, js: String) -> Eval {
        NoOpDocument.eval(js)
    }

    fn create_style(&self, props: StyleProps) {
        let Ok(contents) = props.style_contents() else {
            panic!("the style carries its contents inline");
        };
        DELIVERED_STYLES.with_borrow_mut(|delivered| delivered.push(contents));
    }
}

/// Puts a recording document in front of the viewer, so the styles it renders
/// into the document are captured instead of discarded.
fn record_delivered_styles() {
    use_hook(|| {
        DELIVERED_STYLES.with_borrow_mut(Vec::clear);
        provide_context(Rc::new(RecordingDocument) as Rc<dyn Document>);
    });
}

/// REQT-q356bvvv15 (Style delivery): the viewer renders its own styles into
/// the document, so an app mounting it adds no stylesheet of its own.
#[test]
fn a_mounted_viewer_delivers_the_crate_s_stylesheet_to_the_document() {
    fn app() -> Element {
        record_delivered_styles();
        rsx! { DiffViewer { old_text: "a", new_text: "b" } }
    }

    let viewer = MountedApp::new(app);

    assert_eq!(
        DELIVERED_STYLES.with_borrow(|delivered| delivered.clone()),
        vec![STYLESHEET.to_owned()],
        "the app adds no stylesheet, so the viewer delivers the crate's own"
    );
    assert!(
        viewer.tree().elements_with_tag("style").is_empty(),
        "the stylesheet reaches the document's head, not the viewer's own tree"
    );
}

/// REQT-q356bvvv15 (Style delivery) with more than one viewer mounted: each
/// delivers the crate's sheet, and the copies are identical, so an app that
/// mounts several still adds none of its own.
#[test]
fn every_mounted_viewer_delivers_the_same_stylesheet() {
    fn app() -> Element {
        record_delivered_styles();
        rsx! {
            DiffViewer { old_text: "a", new_text: "b" }
            DiffViewer { old_text: "a", new_text: "b", theme: DiffTheme::Dark }
        }
    }

    let _viewer = MountedApp::new(app);

    assert_eq!(
        DELIVERED_STYLES.with_borrow(|delivered| delivered.clone()),
        vec![STYLESHEET.to_owned(); 2]
    );
}

/// REQT-sc8expw3q8 (Theme selection): a viewer follows the reader's preference
/// unless the consumer selects light or dark.
#[test]
fn a_viewer_follows_the_reader_unless_the_app_chooses() {
    fn reader_app() -> Element {
        rsx! { DiffViewer { old_text: "a", new_text: "b" } }
    }
    fn chosen_app() -> Element {
        rsx! { DiffViewer { old_text: "a", new_text: "b", theme: DiffTheme::Dark } }
    }

    assert_eq!(viewer_themes(&MountedApp::new(reader_app)), vec!["auto"]);
    assert_eq!(viewer_themes(&MountedApp::new(chosen_app)), vec!["dark"]);
}

/// REQT-sc8expw3q8 (Theme selection) with ARCH-7kwnstr5rt (DiffTheme): the
/// choice rides on the viewer element, so two mounted viewers may differ.
#[test]
fn two_mounted_viewers_may_read_different_themes() {
    fn app() -> Element {
        rsx! {
            DiffViewer { old_text: "a", new_text: "b", theme: DiffTheme::Light }
            DiffViewer { old_text: "a", new_text: "b", theme: DiffTheme::Dark }
        }
    }

    assert_eq!(viewer_themes(&MountedApp::new(app)), vec!["light", "dark"]);
}

/// The theme each mounted viewer reports, in document order.
fn viewer_themes(viewer: &MountedApp) -> Vec<String> {
    viewer
        .tree()
        .elements_with_tag("table")
        .iter()
        .map(|table| {
            table
                .attribute("data-dxdiff-theme")
                .expect("the viewer reports its theme")
                .to_owned()
        })
        .collect()
}

/// REQT-3928hx46s3 (Styling hooks): a gutter that reports its clicks carries a
/// class naming that, so a hover affordance reaches only those gutters.
#[test]
fn only_a_gutter_that_answers_a_click_is_marked_clickable() {
    fn listening_app() -> Element {
        rsx! {
            DiffViewer {
                old_text: "a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\nchanged",
                new_text: "a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\ndifferent",
                on_line_number_click: move |_: LineNumberClick| {},
            }
        }
    }

    let viewer = MountedApp::new(listening_app);
    let gutters = viewer.tree().elements_with_class(DXDIFF__GUTTER);
    let marked: Vec<bool> = gutters
        .iter()
        .map(|gutter| gutter.has_class(DXDIFF__GUTTER_CLICKABLE))
        .collect();

    assert!(
        marked.iter().any(|clickable| *clickable),
        "a viewer given a click handler marks the gutters that answer a click"
    );
    for (gutter, clickable) in gutters.iter().zip(&marked) {
        assert_eq!(
            gutter.listener_element_id("click").is_some(),
            *clickable,
            "the class and the click listener reach exactly the same gutters"
        );
    }
    assert!(
        viewer
            .tree()
            .elements_with_class("dxdiff-fold-row")
            .iter()
            .flat_map(|fold_row| fold_row.descendants_with_class(DXDIFF__GUTTER))
            .all(|gutter| !gutter.has_class(DXDIFF__GUTTER_CLICKABLE)),
        "a fold row's gutters answer no click, so they go unmarked"
    );
}

/// REQT-3928hx46s3 (Styling hooks): a viewer that reports no clicks marks no
/// gutter, so nothing offers an affordance that answers nothing.
#[test]
fn a_viewer_reporting_no_clicks_marks_no_gutter_clickable() {
    fn app() -> Element {
        rsx! { DiffViewer { old_text: "a\nb", new_text: "a\nc" } }
    }

    let viewer = MountedApp::new(app);

    assert!(
        viewer
            .tree()
            .elements_with_class(DXDIFF__GUTTER_CLICKABLE)
            .is_empty()
    );
}
