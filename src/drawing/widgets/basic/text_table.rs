use std::borrow::Cow;

use tui::{
    backend::Backend,
    layout::Constraint,
    style::Style as TuiStyle,
    widgets::{Block, Cell, Row, Table, TableState},
};

use crate::drawing::{VerticalScrollDirection, Widget};

const DOWN_ARROW: char = '▼';
const UP_ARROW: char = '▲';

/// Style for a [`TextTable`].
#[derive(Default)]
pub struct Style {
    highlight: TuiStyle,
    text: TuiStyle,
}

/// State for a [`TextTable`].
pub struct State {
    /// The previously selected index.
    previously_selected: usize,

    /// The current scroll direction.
    vertical_scroll_direction: VerticalScrollDirection,

    /// tui-rs' own internal table state representation.
    /// Contains the currently selected index.
    tui_table_state: TableState,

    /// The index to sort by.  Note that the data is not actually sorted here, all this does
    /// is indicate which column we're sorting by!
    sort_column_index: usize,

    /// Whether to sort by ascending or descending.
    sort_ascending: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            previously_selected: 0,
            vertical_scroll_direction: VerticalScrollDirection::Down,
            tui_table_state: TableState::default(),
            sort_column_index: 0,
            sort_ascending: false,
        }
    }
}

/// [`TextTable`] is a scrollable, sortable text table.
pub struct TextTable<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// The state of the [`TextTable`].
    state: &'a mut State,

    /// The headers to use for the [`TextTable`].
    headers: Cow<'a, [T]>,

    /// The data to display for the [`TextTable`].
    data: Cow<'a, [Cow<'a, [T]>]>,

    width: Constraint,
    height: Constraint,

    style: Style,

    table_gap: u16,
}

impl<'a, T> TextTable<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// Creates a new [`TextTable`].
    ///
    /// Note that there is no check that the data and headers are the same length.  This is something that
    /// the caller must ensure (if they care for it).  A mismatch in header and data size may result in drawing
    /// data without headers or headers without data, which is a valid state, but may not be desired!
    pub fn new(
        state: &'a mut State, headers: impl Into<Cow<'a, [T]>>,
        data: impl Into<Cow<'a, [Cow<'a, [T]>]>>,
    ) -> Self {
        Self {
            state,
            headers: headers.into(),
            data: data.into(),
            width: Constraint::Min(0),
            height: Constraint::Min(0),
            style: Style::default(),
            table_gap: 1,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn table_gap(mut self, table_gap: u16) -> Self {
        self.table_gap = table_gap;
        self
    }
}

impl<'a, B, T> Widget<B> for TextTable<'a, T>
where
    B: Backend,
    [T]: ToOwned<Owned = Vec<T>>,
    T: ToString,
{
    fn draw(&mut self, ctx: &mut tui::Frame<'_, B>, node: &'_ crate::drawing::Node) {
        let bounds = node.bounds();

        let num_rows = (bounds.height - 1 - self.table_gap) as usize;
        let row_iter = self
            .data
            .iter()
            .take(num_rows)
            .map(|row| Row::new(row.iter().map(ToString::to_string)));
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style();

        let header = Row::new(self.headers.iter().enumerate().map(|(itx, value)| {
            if itx == self.state.sort_column_index {
                value.to_string()
            } else {
                format!(
                    "{} {}",
                    value.to_string(),
                    if self.state.sort_ascending {
                        UP_ARROW
                    } else {
                        DOWN_ARROW
                    }
                )
            }
        }));
        let widths = vec![]; // TODO: Widths

        ctx.render_stateful_widget(
            Table::new(row_iter)
                .block(block)
                .header(header)
                .highlight_style(self.style.highlight)
                .style(self.style.text)
                .widths(&widths),
            bounds,
            &mut self.state.tui_table_state,
        )
    }

    fn layout(&self, bounds: tui::layout::Rect) -> crate::drawing::Node {
        todo!()
    }

    fn width(&self) -> tui::layout::Constraint {
        self.width
    }

    fn height(&self) -> tui::layout::Constraint {
        self.height
    }

    fn on_event(&self, event: crate::drawing::Event) -> crate::drawing::EventStatus {
        crate::drawing::EventStatus::Ignored
    }
}
