pub struct DirectionFlags;

#[rustfmt::skip]
impl DirectionFlags {
    pub const NORTH:    u8 = 1 << 0;
    pub const EAST:     u8 = 1 << 1;
    pub const SOUTH:    u8 = 1 << 2;
    pub const WEST:     u8 = 1 << 3;
    pub const UP:       u8 = 1 << 4;
    pub const DOWN:     u8 = 1 << 5;
}
