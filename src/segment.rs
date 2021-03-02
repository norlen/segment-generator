/// Letter representing each segment.
/// See the documentation for exactly where they should be put.
pub(crate) enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    N,
    P,
}

impl Segment {
    /// Converts a letter char to a `Segment`. Will panic on invalid segment letters.
    pub fn from_ch(ch: char) -> Self {
        match ch {
            'A' => Segment::A,
            'B' => Segment::B,
            'C' => Segment::C,
            'D' => Segment::D,
            'E' => Segment::E,
            'F' => Segment::F,
            'G' => Segment::G,
            'H' => Segment::H,
            'J' => Segment::J,
            'K' => Segment::K,
            'L' => Segment::L,
            'M' => Segment::M,
            'N' => Segment::N,
            'P' => Segment::P,
            _ => panic!(format!("invalid char: {}", ch)),
        }
    }

    /// Retreives the bit offset for this segment.
    pub fn offset(&self) -> usize {
        match *self {
            Segment::D => 0,
            Segment::N => 1,
            Segment::P => 2,
            Segment::M => 3,

            Segment::C => 4,
            Segment::G => 5,
            Segment::E => 6,
            Segment::L => 7,

            Segment::B => 8,
            Segment::H => 9,
            Segment::F => 10,
            Segment::J => 11,

            Segment::A => 12,
            Segment::K => 15,
        }
    }

    /// Retreives the mask for this segment.
    pub fn mask(&self) -> u16 {
        1 << self.offset()
    }
}
