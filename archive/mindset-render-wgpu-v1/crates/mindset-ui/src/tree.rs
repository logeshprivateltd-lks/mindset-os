//! Retained UI component tree, granular invalidation, and declarative builder.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::{ComponentNode, ElementId, WidgetState};
use crate::layout::{FlexContainer, LayoutCache, LayoutRect};
use std::collections::{HashMap, HashSet};

/// Retained-mode UI Component Tree.
#[derive(Debug, Clone, Default)]
pub struct UiTree {
    pub nodes: HashMap<ElementId, ComponentNode>,
    pub root_id: Option<ElementId>,
    pub hovered: Option<ElementId>,
    pub focused: Option<ElementId>,
    pub captured_pointer: Option<ElementId>,
    pub layout_cache: LayoutCache,
    pub dirty_nodes: HashSet<ElementId>,
}

impl UiTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_root(&mut self, node: ComponentNode) -> ElementId {
        let id = node.id;
        self.nodes.insert(id, node);
        self.root_id = Some(id);
        self.dirty_nodes.insert(id);
        id
    }

    pub fn add_child(&mut self, parent_id: ElementId, mut child: ComponentNode) -> ElementId {
        let child_id = child.id;
        child.parent = Some(parent_id);
        self.nodes.insert(child_id, child);
        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.add_child(child_id);
        }
        self.dirty_nodes.insert(child_id);
        self.dirty_nodes.insert(parent_id);
        child_id
    }

    pub fn get(&self, id: ElementId) -> Option<&ComponentNode> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: ElementId) -> Option<&mut ComponentNode> {
        self.nodes.get_mut(&id)
    }

    pub fn mark_dirty(&mut self, id: ElementId) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.dirty.mark_layout();
            self.dirty_nodes.insert(id);
            self.layout_cache.invalidate(id);
        }
    }

    pub fn mark_dirty_subtree(&mut self, id: ElementId) {
        let mut queue = vec![id];
        while let Some(curr_id) = queue.pop() {
            if let Some(node) = self.nodes.get_mut(&curr_id) {
                node.dirty.mark_subtree();
                self.dirty_nodes.insert(curr_id);
                self.layout_cache.invalidate(curr_id);
                queue.extend(node.children.iter().copied());
            }
        }
    }

    /// Recursively solves the layout starting from root, utilizing `layout_cache`.
    pub fn solve_layout(&mut self, screen_bounds: LayoutRect) {
        let root = match self.root_id {
            Some(r) => r,
            None => return,
        };
        self.solve_node_layout(root, screen_bounds);
    }

    fn solve_node_layout(&mut self, id: ElementId, bounds: LayoutRect) {
        let (children, is_dirty) = match self.nodes.get(&id) {
            Some(n) => (n.children.clone(), n.dirty.layout),
            None => return,
        };

        // If cached and not dirty, use cached bounds
        let node_rect = if !is_dirty {
            if let Some(cached) = self.layout_cache.get(id) {
                cached
            } else {
                bounds
            }
        } else {
            bounds
        };

        if let Some(n) = self.nodes.get_mut(&id) {
            n.rect = node_rect;
            n.dirty.layout = false;
        }
        self.layout_cache.insert(id, node_rect);
        self.dirty_nodes.remove(&id);

        if children.is_empty() {
            return;
        }

        // Layout children in a column or row
        let _n_children = children.len();
        let child_sizes: Vec<(f32, f32)> = children
            .iter()
            .map(|&c_id| {
                let r = self.nodes.get(&c_id).map_or(LayoutRect::zero(), |n| n.rect);
                (
                    if r.w > 0.0 { r.w } else { node_rect.w },
                    if r.h > 0.0 { r.h } else { 32.0 },
                )
            })
            .collect();

        let flex = FlexContainer::column(4.0);
        let child_rects = flex.solve(node_rect, &child_sizes);

        for (i, &child_id) in children.iter().enumerate() {
            let cb = child_rects.get(i).copied().unwrap_or(node_rect);
            self.solve_node_layout(child_id, cb);
        }
    }

    /// Performs a top-down hit test for point (px, py).
    pub fn hit_test(&self, px: f32, py: f32) -> Option<ElementId> {
        let root = self.root_id?;
        self.hit_test_recursive(root, px, py)
    }

    fn hit_test_recursive(&self, id: ElementId, px: f32, py: f32) -> Option<ElementId> {
        let node = self.nodes.get(&id)?;
        if !node.is_visible || !node.rect.contains(px, py) {
            return None;
        }

        // Check children from top-most to bottom-most
        for &child_id in node.children.iter().rev() {
            if let Some(hit) = self.hit_test_recursive(child_id, px, py) {
                return Some(hit);
            }
        }

        Some(id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

/// Declarative Builder API for constructing component trees ergonomically.
#[derive(Debug)]
pub struct UiBuilder<'a> {
    pub tree: &'a mut UiTree,
    pub current_parent: ElementId,
    counter: usize,
}

impl<'a> UiBuilder<'a> {
    pub fn new(tree: &'a mut UiTree, root: ElementId) -> Self {
        Self {
            tree,
            current_parent: root,
            counter: 0,
        }
    }

    fn next_id(&mut self, prefix: &str) -> ElementId {
        self.counter += 1;
        ElementId::from_name_and_index(prefix, self.counter)
    }

    pub fn column<F>(&mut self, name: &str, f: F) -> ElementId
    where
        F: FnOnce(&mut UiBuilder),
    {
        let id = self.next_id(name);
        let node = ComponentNode::new(id, name);
        self.tree.add_child(self.current_parent, node);

        let old_parent = self.current_parent;
        self.current_parent = id;
        f(self);
        self.current_parent = old_parent;
        id
    }

    pub fn row<F>(&mut self, name: &str, f: F) -> ElementId
    where
        F: FnOnce(&mut UiBuilder),
    {
        let id = self.next_id(name);
        let node = ComponentNode::new(id, name);
        self.tree.add_child(self.current_parent, node);

        let old_parent = self.current_parent;
        self.current_parent = id;
        f(self);
        self.current_parent = old_parent;
        id
    }

    pub fn card<F>(&mut self, name: &str, f: F) -> ElementId
    where
        F: FnOnce(&mut UiBuilder),
    {
        let id = self.next_id(name);
        let node = ComponentNode::new(id, name);
        self.tree.add_child(self.current_parent, node);

        let old_parent = self.current_parent;
        self.current_parent = id;
        f(self);
        self.current_parent = old_parent;
        id
    }

    pub fn button(&mut self, label: &str) -> ElementId {
        let id = self.next_id("button");
        let mut node = ComponentNode::new(id, label);
        node.accessibility_label = label.to_string();
        self.tree.add_child(self.current_parent, node)
    }

    pub fn slider(&mut self, label: &str, _val: f32) -> ElementId {
        let id = self.next_id("slider");
        let mut node = ComponentNode::new(id, label);
        node.accessibility_label = label.to_string();
        self.tree.add_child(self.current_parent, node)
    }

    pub fn toggle(&mut self, label: &str, is_on: bool) -> ElementId {
        let id = self.next_id("toggle");
        let mut node = ComponentNode::new(id, label);
        node.state = if is_on {
            WidgetState::Selected
        } else {
            WidgetState::Normal
        };
        node.accessibility_label = label.to_string();
        self.tree.add_child(self.current_parent, node)
    }

    pub fn text_input(&mut self, placeholder: &str) -> ElementId {
        let id = self.next_id("text_input");
        let mut node = ComponentNode::new(id, placeholder);
        node.accessibility_label = placeholder.to_string();
        self.tree.add_child(self.current_parent, node)
    }

    pub fn label(&mut self, text: &str) -> ElementId {
        let id = self.next_id("label");
        let mut node = ComponentNode::new(id, text);
        node.accessibility_label = text.to_string();
        self.tree.add_child(self.current_parent, node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_tree_1000_elements_stress_and_performance() {
        let mut tree = UiTree::new();
        let root_id = tree.insert_root(
            ComponentNode::new(ElementId::ROOT, "root")
                .with_rect(LayoutRect::new(0.0, 0.0, 1920.0, 1080.0)),
        );

        let start = std::time::Instant::now();
        {
            let mut builder = UiBuilder::new(&mut tree, root_id);
            for col_idx in 0..10 {
                builder.column(&format!("col_{}", col_idx), |b| {
                    for row_idx in 0..10 {
                        b.card(&format!("card_{}_{}", col_idx, row_idx), |cb| {
                            for item_idx in 0..9 {
                                cb.button(&format!("btn_{}_{}_{}", col_idx, row_idx, item_idx));
                            }
                        });
                    }
                });
            }
        }

        // Root (1) + 10 cols + 10*10 cards (100) + 100*9 buttons (900) = 1011 elements!
        assert!(tree.node_count() > 1000);

        // Solve layout
        let layout_start = std::time::Instant::now();
        tree.solve_layout(LayoutRect::new(0.0, 0.0, 1920.0, 1080.0));
        let layout_elapsed = layout_start.elapsed();

        #[cfg(debug_assertions)]
        let max_allowed_ms = 16;
        #[cfg(not(debug_assertions))]
        let max_allowed_ms = 4;
        assert!(
            layout_elapsed.as_millis() <= max_allowed_ms,
            "1,000 node layout took too long: {:?}",
            layout_elapsed
        );

        // Hit test
        let hit = tree.hit_test(100.0, 100.0);
        assert!(hit.is_some());

        // Invalidate single node
        tree.mark_dirty(hit.unwrap());
        assert_eq!(tree.dirty_nodes.len(), 1);

        let total_elapsed = start.elapsed();
        log::info!(
            "1,000 elements tree creation + layout took: {:?}",
            total_elapsed
        );
    }
}
