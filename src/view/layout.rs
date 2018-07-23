use view::space::Space;
use view::widget::{Square, SquareBuilder};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Layout {
    Vertical,
    Horizontal,
}

impl Default for Layout {
    fn default() -> Layout {
        Layout::Vertical
    }
}

#[derive(Debug, PartialEq, Clone, Default, Builder)]
#[builder(pattern = "owned")]
pub struct Flex<T: Space> {
    border: bool,
    center: bool,
    items: Vec<T>,
    layout: Layout,
    margin: Square,
}

impl<T: Space> Space for Flex<T> {
    fn full_width(&self) -> u16 {
        self.items.iter().map(|a| a.full_width()).max().unwrap_or(0) + self.margin.horizontal()
    }

    fn full_height(&self) -> u16 {
        self.items
            .iter()
            .map(|a| a.full_height())
            .max()
            .unwrap_or(0) + self.margin.vertical()
    }

    fn width(&self) -> u16 {
        self.items.iter().map(|a| a.full_width()).max().unwrap_or(0)
    }

    fn height(&self) -> u16 {
        self.items
            .iter()
            .map(|a| a.full_height())
            .max()
            .unwrap_or(0)
    }
}
