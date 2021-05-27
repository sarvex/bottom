use tui::layout::Rect;

pub struct Node {
    bounds: Rect,
    children: Vec<Node>,
}

impl Node {
    pub fn new(bounds: Rect, children: Vec<Node>) -> Node {
        Self { bounds, children }
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

pub struct Layout<'a> {
    node: &'a Node,
}

impl<'a> Layout<'a> {
    pub fn children(&self) -> impl Iterator<Item = Layout<'a>> {
        self.node.children().iter().map(move |node| Layout { node })
    }

    pub fn bounds(&self) -> Rect {
        self.node.bounds()
    }
}
