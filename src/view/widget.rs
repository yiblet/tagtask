extern crate derive_builder;
extern crate ropey;

use view::space::Space;

#[derive(Debug, PartialEq, Eq, Clone, Default, Builder, Hash)]
#[builder(default, pattern = "owned")]
pub struct Square {
    pub top: u16,
    pub bottom: u16,
    pub right: u16,
    pub left: u16,
}

impl Square {
    pub fn horizontal(&self) -> u16 {
        self.right + self.left
    }

    pub fn vertical(&self) -> u16 {
        self.top + self.bottom
    }
}

#[derive(Debug, PartialEq, Clone, Builder, Default)]
#[builder(default, pattern = "owned")]
pub struct Widget {
    width: u16,
    height: Option<u16>,
    border: bool,
    padding: Square,
    margin: Square,
    text: Option<ropey::Rope>,
    id: u64,
}

impl Widget {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            id: 0,
            border: false,
            width: 0,
            height: Some(0),
            margin: SquareBuilder::default().build().unwrap(),
            padding: SquareBuilder::default().build().unwrap(),
            text: Option::default(),
        }
    }
}

impl Space for Widget {
    fn full_width(&self) -> u16 {
        self.width() + self.margin.horizontal()
    }

    fn full_height(&self) -> u16 {
        self.height() + self.margin.vertical()
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height.unwrap_or_else(|| {
            self.text
                .as_ref()
                .map(|rope: &ropey::Rope| rope.len_lines() as u16)
                .unwrap_or(0)
        })
    }
}
