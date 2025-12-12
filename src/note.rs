#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Note{
    C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B,
}

impl Note {
    pub fn value(self) -> i32{
        match self {
            Note::C => 0,
            Note::Cs => 1,
            Note::D => 2,
            Note::Ds => 3,
            Note::E => 4,
            Note::F => 5,
            Note::Fs => 6,
            Note::G => 7,
            Note::Gs => 8,
            Note::A => 9,
            Note::As => 10,
            Note::B => 11,
        }
    }

    pub fn from_value(v: i32) -> Self {
        match (v % 12 + 12) % 12 {
           0 => Note::C,
           1 => Note::Cs,
           2 => Note::D,
           3 => Note::Ds,
           4 => Note::E, 
           5 => Note::F,
           6 => Note::Fs,
           7 => Note::G,
           8 => Note::Gs,
           9 => Note::A,
           10 => Note::As,
           11 => Note::B,
           _ => unreachable!(),
        }
    }
}