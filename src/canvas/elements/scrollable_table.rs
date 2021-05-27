#![allow(dead_code)]
#![allow(unused_variables)]

use tui::widgets::TableState;

use super::element::{Element, ElementBounds};

/// The state for a [`ScrollableTable`].
struct ScrollableTableState {
    tui_state: TableState,
}

/// A [`ScrollableTable`] is a stateful table [`Element`] with scrolling support.
pub struct ScrollableTable {
    bounds: ElementBounds,
    selected: bool,
    state: ScrollableTableState,
}

impl ScrollableTable {}
