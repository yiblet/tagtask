
pub trait Space : PartialEq {
    fn full_width(&self) -> u16;
    fn full_height(&self) -> u16;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

impl <T: Space> Space for Box<T> {
    fn full_width(&self) -> u16 {
        self.as_ref().full_width()
    }
    
    fn full_height(&self) -> u16 {
        self.as_ref().full_height()
    }

    fn width(&self) -> u16 {
        self.as_ref().width()
    }
    
    fn height(&self) -> u16 {
        self.as_ref().height()
    }
}
