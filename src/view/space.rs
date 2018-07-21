pub trait Space {
    fn full_width(&self) -> u16;
    fn full_height(&self) -> u16;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}
