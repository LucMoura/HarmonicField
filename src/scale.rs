use crate::note::Note;

pub enum ScaleType {
    Major,
    MinorNatural,
}

impl ScaleType {
    pub fn intervals(&self) -> &'static [i32; 7] {
        match self{
            ScaleType::Major => &[0, 2, 4, 5, 7, 9, 11],
            ScaleType::MinorNatural => &[0, 2, 3, 5, 7, 8, 10],
        }
    }
}

pub struct Scale {
    pub root: Note,
    pub notes: Vec<Note>
}

impl Scale {
    pub fn new(root: Note, stype: ScaleType) -> Self {
        let notes = stype
        .intervals()
        .iter()
        .map(|i| Note::from_value(root.value() + i))
        .collect();

        Scale { root, notes }
    }
}