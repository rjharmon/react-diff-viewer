//! Mounts an app component and reads its rendered rows, so each test reads as
//! arrange-act-assert over rows rather than over framework calls.

use std::any::Any;
use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_html::geometry::{ClientPoint, Coordinates, ElementPoint, PagePoint, ScreenPoint};
use dioxus_html::input_data::MouseButtonSet;
use dioxus_html::{
    Modifiers, PlatformEventData, SerializedHtmlEventConverter, SerializedMouseData,
};

use crate::rendered_tree::{RenderedElement, RenderedTree};

/// An app component mounted in its own VirtualDom and rendered to a tree.
pub struct MountedApp {
    vdom: VirtualDom,
    tree: RenderedTree,
}

impl MountedApp {
    pub fn new(app: fn() -> Element) -> Self {
        let mut mounted = Self {
            vdom: VirtualDom::new(app),
            tree: RenderedTree::new(),
        };
        mounted.vdom.rebuild(&mut mounted.tree);
        mounted.settle();
        mounted
    }

    /// Runs queued work until the rendering settles.
    ///
    /// A prop change can mark memos dirty while the parent renders, which
    /// queues further rendering that a single pass leaves undone. The
    /// VirtualDom exposes no pending-work query, so rounds run until two in a
    /// row apply no mutations.
    fn settle(&mut self) {
        let mut quiet_rounds = 0;
        for _ in 0..32 {
            let applied_before = self.tree.applied_mutations();
            self.vdom.process_events();
            self.vdom.render_immediate(&mut self.tree);
            if self.tree.applied_mutations() == applied_before {
                quiet_rounds += 1;
                if quiet_rounds == 2 {
                    return;
                }
            } else {
                quiet_rounds = 0;
            }
        }
        panic!("the rendering did not settle within 32 rounds");
    }

    pub fn tree(&self) -> &RenderedTree {
        &self.tree
    }

    /// Each rendered table row read as its cells' texts joined by ` | `, in
    /// document order: title, line, and fold rows alike.
    pub fn row_readings(&self) -> Vec<String> {
        self.tree
            .elements_with_tag("tr")
            .iter()
            .map(row_reading)
            .collect()
    }

    /// Each rendered table row's width in columns: its cells, each counted by
    /// its `colspan` or as one column, in document order.
    pub fn row_column_counts(&self) -> Vec<usize> {
        self.tree
            .elements_with_tag("tr")
            .iter()
            .map(|row| {
                row.child_elements()
                    .iter()
                    .map(|cell| {
                        cell.attribute("colspan")
                            .map_or(1, |span| span.parse().unwrap())
                    })
                    .sum()
            })
            .collect()
    }

    /// The texts of every element carrying `class`, in document order.
    pub fn texts_with_class(&self, class: &str) -> Vec<String> {
        self.tree
            .elements_with_class(class)
            .iter()
            .map(RenderedElement::text)
            .collect()
    }

    /// Clicks the `button` whose text is `label`.
    pub fn click_button(&mut self, label: &str) {
        self.click_matching(|element| element.tag() == "button" && element.text() == label);
    }

    /// Clicks the first fold row's button.
    pub fn expand_first_fold(&mut self) {
        let fold_row = *self
            .tree
            .elements_with_class("dxdiff-fold-row")
            .first()
            .expect("a fold row is shown");
        let button = *fold_row
            .descendants_with_tag("button")
            .first()
            .expect("the fold row holds a button");
        let element_id = button
            .listener_element_id("click")
            .expect("the fold row's button listens for clicks");
        self.dispatch_click(element_id, Modifiers::empty());
    }

    /// Clicks the `column`-th gutter of the `row`-th line row (column 0 is a
    /// split view's old side or an inline view's old column).
    pub fn click_line_number(&mut self, row: usize, column: usize, modifiers: Modifiers) {
        let rows = self.tree.elements_with_class("dxdiff-row");
        let gutter = *rows[row]
            .descendants_with_class("dxdiff-gutter")
            .get(column)
            .expect("the row has that gutter");
        let element_id = gutter
            .listener_element_id("click")
            .expect("the gutter listens for clicks");
        self.dispatch_click(element_id, modifiers);
    }

    fn click_matching(&mut self, matches: impl Fn(&RenderedElement<'_>) -> bool) {
        let element_id = self
            .tree
            .elements_with_tag("button")
            .into_iter()
            .find(|element| matches(element))
            .and_then(|element| element.listener_element_id("click"))
            .expect("a matching clickable element is rendered");
        self.dispatch_click(element_id, Modifiers::empty());
    }

    fn dispatch_click(&mut self, element_id: dioxus::dioxus_core::ElementId, modifiers: Modifiers) {
        set_event_converter(Box::new(SerializedHtmlEventConverter));
        let mouse = SerializedMouseData::new(
            None,
            MouseButtonSet::empty(),
            Coordinates::new(
                ScreenPoint::zero(),
                ClientPoint::zero(),
                ElementPoint::zero(),
                PagePoint::zero(),
            ),
            modifiers,
        );
        let event = Event::new(
            Rc::new(PlatformEventData::new(Box::new(mouse))) as Rc<dyn Any>,
            true,
        );
        self.vdom.runtime().handle_event("click", event, element_id);
        self.settle();
    }
}

fn row_reading(row: &RenderedElement<'_>) -> String {
    row.child_elements()
        .iter()
        .map(RenderedElement::text)
        .collect::<Vec<_>>()
        .join(" | ")
}
