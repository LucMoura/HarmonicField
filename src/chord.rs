use crate::note::Note;

#[derive(Debug)]
pub struct Chord {
    pub name: String,
    pub notes: Vec<Note>,
}