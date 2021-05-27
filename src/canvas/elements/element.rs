#![allow(dead_code)]

use tui::{backend::Backend, layout::Rect, Frame};

use crate::canvas::canvas_colours::CanvasColours;

use super::{Layout, Node};

/// A single point.
#[derive(Copy, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

/// The top-left and bottom-right corners of a [`Element`].
#[derive(Copy, Clone)]
pub enum ElementBounds {
    Unset,
    Points {
        top_left_corner: Point,
        bottom_right_corner: Point,
    },
}

/// A basic [`Element`] trait, all drawn components must implement this.
pub trait Element {
    /// The type of data that is expected for the [`Element`].

    /// The main drawing function.
    fn draw<B: Backend>(
        &mut self, f: &mut Frame<'_, B>, layout: Layout<'_>, style: &CanvasColours,
    ) -> anyhow::Result<()>;

    /// Determines the layout.
    fn layout(&mut self, bounds: Rect) -> Node;

    /// Returns the name of the [`Element`], if it exists.  Default implementation returns `None`.
    fn name(&self) -> Option<&str> {
        None
    }

    /// Returns whether an [`Element`] is selected.  Default implementation
    fn is_selected(&self) -> bool {
        false
    }

    /// Marks an [`Element`] as selected.  Default implementation does nothing.
    fn select(&mut self) {}

    /// Marks an [`Element`] as unselected.  Default implementation does nothing.
    fn unselect(&mut self) {}
}
