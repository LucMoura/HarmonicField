mod note;
mod scale;
mod chord;
mod harmonic_field;

use note::Note;
use scale::ScaleType;
use harmonic_field::generate_harmonic_field;

fn main(){
    let field = generate_harmonic_field(Note::C, ScaleType::Major);

    for (i, chord) in field.iter().enumerate(){
        println!("Grau {}: {} -> {:?}", i + 1, chord.name, chord.notes);
    }
}