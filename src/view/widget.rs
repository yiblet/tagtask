extern crate derive_builder;
extern crate ropey;

use view::space::Space;

#[derive(Debug, PartialEq, Eq, Clone, Default, Builder)]
#[builder(default, pattern = "owned")]
pub struct Square {
    top: u16,
    bottom: u16,
    right: u16,
    left: u16,
}

#[derive(Debug, PartialEq, Clone, Builder, Default)]
#[builder(default, pattern = "owned")]
pub struct Widget {
    width: u16,
    height: u16,
    border: bool,
    padding: Square,
    margin: Square,
    text: Option<ropey::Rope>,
}

impl Widget {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            border: false,
            width: 0,
            height: 0,
            margin: SquareBuilder::default().build().unwrap(),
            padding: SquareBuilder::default().build().unwrap(),
            text: Option::default(),
        }
    }
}

impl Space for Widget {
    fn full_width(&self) -> u16 {
        self.width + self.margin.left + self.margin.right
    }

    fn full_height(&self) -> u16 {
        self.height + self.margin.top + self.margin.bottom
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}
