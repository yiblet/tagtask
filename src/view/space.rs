use std::ops::Deref;

pub trait Space: PartialEq {
    fn full_width(&self) -> u16;
    fn full_height(&self) -> u16;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

impl<T: Space, D: Deref<Target = T> + PartialEq> Space for D {
    fn full_width(&self) -> u16 {
        self.deref().full_width()
    }

    fn full_height(&self) -> u16 {
        self.deref().full_height()
    }

    fn width(&self) -> u16 {
        self.deref().width()
    }

    fn height(&self) -> u16 {
        self.deref().height()
    }
}
