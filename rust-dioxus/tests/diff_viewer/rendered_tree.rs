//! A test-only renderer: applies the VirtualDom's mutations to an in-memory
//! element tree the tests can read and click through.
//!
//! Server-side rendering yields markup but no element ids, and clicks are
//! dispatched to element ids, so the tests render through this instead. It
//! follows the mutation semantics of Dioxus's own native renderer
//! (`packages/native-dom/src/mutation_writer.rs`): a node stack, an
//! element-id-to-node mapping, and templates cloned from prototypes.

use std::collections::HashMap;

use dioxus::dioxus_core::{
    AttributeValue, ElementId, Template, TemplateAttribute, TemplateNode, WriteMutations,
};

type NodeIndex = usize;

enum NodeKind {
    Element {
        tag: &'static str,
        attributes: Vec<(&'static str, String)>,
        listeners: Vec<&'static str>,
    },
    Text(String),
    Placeholder,
}

struct Node {
    kind: NodeKind,
    parent: Option<NodeIndex>,
    children: Vec<NodeIndex>,
    element_id: Option<ElementId>,
}

/// The rendered element tree.
pub struct RenderedTree {
    nodes: Vec<Node>,
    root: NodeIndex,
    templates: HashMap<Template, Vec<NodeIndex>>,
    stack: Vec<NodeIndex>,
    element_nodes: Vec<Option<NodeIndex>>,
    applied_mutations: usize,
}

/// One element in the rendered tree.
#[derive(Clone, Copy)]
pub struct RenderedElement<'a> {
    tree: &'a RenderedTree,
    index: NodeIndex,
}

impl RenderedTree {
    pub fn new() -> Self {
        let mut tree = Self {
            nodes: Vec::new(),
            root: 0,
            templates: HashMap::new(),
            stack: Vec::new(),
            element_nodes: Vec::new(),
            applied_mutations: 0,
        };
        tree.root = tree.new_node(NodeKind::Element {
            tag: "root",
            attributes: Vec::new(),
            listeners: Vec::new(),
        });
        tree.map_element(ElementId(0), tree.root);
        tree.stack.push(tree.root);
        tree
    }

    /// How many mutations have been applied, so a caller can tell when a
    /// rendering pass changed nothing.
    pub fn applied_mutations(&self) -> usize {
        self.applied_mutations
    }

    /// Every element under the root carrying `class`, in document order.
    pub fn elements_with_class(&self, class: &str) -> Vec<RenderedElement<'_>> {
        self.element(self.root).descendants_with_class(class)
    }

    /// Every element under the root with this tag, in document order.
    pub fn elements_with_tag(&self, tag: &str) -> Vec<RenderedElement<'_>> {
        self.element(self.root).descendants_with_tag(tag)
    }

    fn element(&self, index: NodeIndex) -> RenderedElement<'_> {
        RenderedElement { tree: self, index }
    }

    fn new_node(&mut self, kind: NodeKind) -> NodeIndex {
        self.nodes.push(Node {
            kind,
            parent: None,
            children: Vec::new(),
            element_id: None,
        });
        self.nodes.len() - 1
    }

    fn map_element(&mut self, id: ElementId, index: NodeIndex) {
        if self.element_nodes.len() <= id.0 {
            self.element_nodes.resize(id.0 + 1, None);
        }
        self.element_nodes[id.0] = Some(index);
        self.nodes[index].element_id = Some(id);
    }

    fn node_of(&self, id: ElementId) -> NodeIndex {
        self.element_nodes[id.0].expect("the element id is mapped")
    }

    fn node_at_path(&self, path: &[u8]) -> NodeIndex {
        let top = *self.stack.last().expect("the stack holds a node");
        path.iter().fold(top, |node, &child| {
            self.nodes[node].children[child as usize]
        })
    }

    fn take_from_stack(&mut self, count: usize) -> Vec<NodeIndex> {
        self.stack.split_off(self.stack.len() - count)
    }

    fn detach(&mut self, index: NodeIndex) {
        if let Some(parent) = self.nodes[index].parent.take() {
            self.nodes[parent].children.retain(|&child| child != index);
        }
    }

    /// Places `new_nodes` among `anchor`'s siblings, `offset` after the anchor.
    fn insert_beside(&mut self, anchor: NodeIndex, new_nodes: Vec<NodeIndex>, offset: usize) {
        let parent = self.nodes[anchor].parent.expect("the anchor has a parent");
        for &node in &new_nodes {
            self.detach(node);
            self.nodes[node].parent = Some(parent);
        }
        let at = self.nodes[parent]
            .children
            .iter()
            .position(|&child| child == anchor)
            .expect("the anchor is its parent's child")
            + offset;
        self.nodes[parent].children.splice(at..at, new_nodes);
    }

    fn replace(&mut self, anchor: NodeIndex, new_nodes: Vec<NodeIndex>) {
        self.insert_beside(anchor, new_nodes, 0);
        self.detach(anchor);
    }

    fn prototype(&mut self, node: &TemplateNode) -> NodeIndex {
        match node {
            TemplateNode::Element {
                tag,
                attrs,
                children,
                ..
            } => {
                let attributes = attrs
                    .iter()
                    .filter_map(|attribute| match attribute {
                        TemplateAttribute::Static { name, value, .. } => {
                            Some((*name, (*value).to_owned()))
                        }
                        TemplateAttribute::Dynamic { .. } => None,
                    })
                    .collect();
                let index = self.new_node(NodeKind::Element {
                    tag,
                    attributes,
                    listeners: Vec::new(),
                });
                for child in children.iter() {
                    let child = self.prototype(child);
                    self.nodes[child].parent = Some(index);
                    self.nodes[index].children.push(child);
                }
                index
            }
            TemplateNode::Text { text } => self.new_node(NodeKind::Text((*text).to_owned())),
            TemplateNode::Dynamic { .. } => self.new_node(NodeKind::Placeholder),
        }
    }

    fn deep_clone(&mut self, index: NodeIndex) -> NodeIndex {
        let kind = match &self.nodes[index].kind {
            NodeKind::Element {
                tag, attributes, ..
            } => NodeKind::Element {
                tag,
                attributes: attributes.clone(),
                listeners: Vec::new(),
            },
            NodeKind::Text(text) => NodeKind::Text(text.clone()),
            NodeKind::Placeholder => NodeKind::Placeholder,
        };
        let clone = self.new_node(kind);
        for child in self.nodes[index].children.clone() {
            let child = self.deep_clone(child);
            self.nodes[child].parent = Some(clone);
            self.nodes[clone].children.push(child);
        }
        clone
    }
}

impl WriteMutations for RenderedTree {
    fn append_children(&mut self, id: ElementId, m: usize) {
        self.applied_mutations += 1;
        let parent = self.node_of(id);
        for child in self.take_from_stack(m) {
            self.detach(child);
            self.nodes[child].parent = Some(parent);
            self.nodes[parent].children.push(child);
        }
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: ElementId) {
        self.applied_mutations += 1;
        let node = self.node_at_path(path);
        self.map_element(id, node);
    }

    fn create_placeholder(&mut self, id: ElementId) {
        self.applied_mutations += 1;
        let node = self.new_node(NodeKind::Placeholder);
        self.map_element(id, node);
        self.stack.push(node);
    }

    fn create_text_node(&mut self, value: &str, id: ElementId) {
        self.applied_mutations += 1;
        let node = self.new_node(NodeKind::Text(value.to_owned()));
        self.map_element(id, node);
        self.stack.push(node);
    }

    fn load_template(&mut self, template: Template, index: usize, id: ElementId) {
        self.applied_mutations += 1;
        if !self.templates.contains_key(&template) {
            let roots = template
                .roots
                .iter()
                .map(|root| self.prototype(root))
                .collect();
            self.templates.insert(template, roots);
        }
        let prototype = self.templates[&template][index];
        let node = self.deep_clone(prototype);
        self.map_element(id, node);
        self.stack.push(node);
    }

    fn replace_node_with(&mut self, id: ElementId, m: usize) {
        self.applied_mutations += 1;
        let anchor = self.node_of(id);
        let new_nodes = self.take_from_stack(m);
        self.replace(anchor, new_nodes);
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        self.applied_mutations += 1;
        // The new nodes leave the stack before the path is read from its top.
        let new_nodes = self.take_from_stack(m);
        let anchor = self.node_at_path(path);
        self.replace(anchor, new_nodes);
    }

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        self.applied_mutations += 1;
        let anchor = self.node_of(id);
        let new_nodes = self.take_from_stack(m);
        self.insert_beside(anchor, new_nodes, 1);
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        self.applied_mutations += 1;
        let anchor = self.node_of(id);
        let new_nodes = self.take_from_stack(m);
        self.insert_beside(anchor, new_nodes, 0);
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        _namespace: Option<&'static str>,
        value: &AttributeValue,
        id: ElementId,
    ) {
        self.applied_mutations += 1;
        let node = self.node_of(id);
        let NodeKind::Element { attributes, .. } = &mut self.nodes[node].kind else {
            panic!("attribute {name} set on a node that is not an element");
        };
        attributes.retain(|(existing, _)| *existing != name);
        let value = match value {
            AttributeValue::Text(text) => text.clone(),
            AttributeValue::Int(number) => number.to_string(),
            AttributeValue::Float(number) => number.to_string(),
            AttributeValue::Bool(flag) => flag.to_string(),
            _ => return,
        };
        attributes.push((name, value));
    }

    fn set_node_text(&mut self, value: &str, id: ElementId) {
        self.applied_mutations += 1;
        let node = self.node_of(id);
        self.nodes[node].kind = NodeKind::Text(value.to_owned());
    }

    fn create_event_listener(&mut self, name: &'static str, id: ElementId) {
        self.applied_mutations += 1;
        let node = self.node_of(id);
        if let NodeKind::Element { listeners, .. } = &mut self.nodes[node].kind {
            listeners.push(name);
        }
    }

    fn remove_event_listener(&mut self, name: &'static str, id: ElementId) {
        self.applied_mutations += 1;
        let node = self.node_of(id);
        if let NodeKind::Element { listeners, .. } = &mut self.nodes[node].kind {
            listeners.retain(|listener| *listener != name);
        }
    }

    fn remove_node(&mut self, id: ElementId) {
        self.applied_mutations += 1;
        let node = self.node_of(id);
        self.detach(node);
    }

    fn push_root(&mut self, id: ElementId) {
        self.applied_mutations += 1;
        let node = self.node_of(id);
        self.stack.push(node);
    }
}

impl<'a> RenderedElement<'a> {
    fn node(&self) -> &'a Node {
        &self.tree.nodes[self.index]
    }

    pub fn tag(&self) -> &'a str {
        match &self.node().kind {
            NodeKind::Element { tag, .. } => tag,
            _ => "",
        }
    }

    pub fn attribute(&self, name: &str) -> Option<&'a str> {
        match &self.node().kind {
            NodeKind::Element { attributes, .. } => attributes
                .iter()
                .find(|(existing, _)| *existing == name)
                .map(|(_, value)| value.as_str()),
            _ => None,
        }
    }

    pub fn classes(&self) -> Vec<&'a str> {
        self.attribute("class")
            .map(|classes| classes.split_whitespace().collect())
            .unwrap_or_default()
    }

    pub fn has_class(&self, class: &str) -> bool {
        self.classes().contains(&class)
    }

    /// The concatenated text of every text node under this element.
    pub fn text(&self) -> String {
        let mut text = String::new();
        self.collect_text(self.index, &mut text);
        text
    }

    fn collect_text(&self, index: NodeIndex, text: &mut String) {
        let node = &self.tree.nodes[index];
        if let NodeKind::Text(value) = &node.kind {
            text.push_str(value);
        }
        for &child in &node.children {
            self.collect_text(child, text);
        }
    }

    /// The element children, in order.
    pub fn child_elements(&self) -> Vec<RenderedElement<'a>> {
        self.node()
            .children
            .iter()
            .filter(|&&child| matches!(self.tree.nodes[child].kind, NodeKind::Element { .. }))
            .map(|&child| self.tree.element(child))
            .collect()
    }

    pub fn descendants_with_class(&self, class: &str) -> Vec<RenderedElement<'a>> {
        self.descendants(&|element| element.has_class(class))
    }

    pub fn descendants_with_tag(&self, tag: &str) -> Vec<RenderedElement<'a>> {
        self.descendants(&|element| element.tag() == tag)
    }

    fn descendants(
        &self,
        matches: &dyn Fn(&RenderedElement<'a>) -> bool,
    ) -> Vec<RenderedElement<'a>> {
        let mut found = Vec::new();
        let mut pending: Vec<NodeIndex> = self.node().children.iter().rev().copied().collect();
        while let Some(index) = pending.pop() {
            let element = self.tree.element(index);
            if matches!(element.node().kind, NodeKind::Element { .. }) && matches(&element) {
                found.push(element);
            }
            pending.extend(self.tree.nodes[index].children.iter().rev().copied());
        }
        found
    }

    /// The element id a listener for `event` is attached under, when it has one.
    pub fn listener_element_id(&self, event: &str) -> Option<ElementId> {
        match &self.node().kind {
            NodeKind::Element { listeners, .. } if listeners.contains(&event) => {
                self.node().element_id
            }
            _ => None,
        }
    }
}
