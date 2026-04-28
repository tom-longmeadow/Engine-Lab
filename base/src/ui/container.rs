 
use std::fmt;

use crate::ui::widget::Widget;

pub trait Container {
    fn children(&self) -> &[Box<dyn Widget>];
    fn push(&mut self, child: Box<dyn Widget>);
    fn remove(&mut self, index: usize) -> Option<Box<dyn Widget>>;
    fn clear(&mut self);
}

pub struct WidgetContainer {
    children: Vec<Box<dyn Widget>>,
    gap: Option<f32>,
    dirty: bool,
}

impl Default for WidgetContainer {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            gap: None,
            dirty: true,
        }
    }
}

impl fmt::Debug for WidgetContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WidgetContainer")
            .field("children_len", &self.children.len())
            .field("gap", &self.gap)
            .field("dirty", &self.dirty)
            .finish()
    }
}

impl WidgetContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gap(&self) -> Option<f32> {
        self.gap
    }

    pub fn set_gap(&mut self, gap: f32) {
        self.gap = Some(gap);
        self.dirty = true;
    }

    pub fn clear_gap(&mut self) {
        self.gap = None;
    }

    pub fn resolved_gap(&self, fallback: f32) -> f32 {
        self.gap.unwrap_or(fallback)
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }

    pub fn push(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
        self.dirty = true;
    }

    pub fn push_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item = Box<dyn Widget>>,
    {
        let mut added = false;
        for child in children {
            self.children.push(child);
            added = true;
        }
        if added {
            self.dirty = true;
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Box<dyn Widget>> {
        if index < self.children.len() {
            self.dirty = true;
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        if !self.children.is_empty() {
            self.children.clear();
            self.dirty = true;
        }
    }

    pub(crate) fn for_each_child_mut(
        &mut self,
        mut f: impl FnMut(&mut Box<dyn Widget>),
    ) {
        for child in &mut self.children {
            f(child);
        }
    }
}