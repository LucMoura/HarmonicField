mod note;
mod scale;
mod chord;
mod harmonic_field;

use std::io::{stdin,stdout,Write};
use note::Note;
use scale::ScaleType;
use harmonic_field::generate_harmonic_field;

fn read_input(prompt: &str)-> String{
    println!("{}", prompt);
    let _ = stdout().flush();

    leu mut s = String::new();
    stdin().read_line(&mut s).expect("Erro ao ler entrada");

    s.trim().to_string()
}

fn parse_note(input: &str) -> Option<Note> {
    match input.to_uppercase().as_str(){
        "C" => Some(Note::C),
        "CS" | "C#" => Some(Note::Cs),
        "D" => Some(Note::D),
        "DS" | "D#" => Some(Note::Ds),
        "E" => Some(Note::E),
        "F" => Some(Note::F),
        "FS" | "F#" => Some(Note::Fs),
        "G" => Some(Note::G),
        "GS" | "G#" => Some(Note::Gs),
        "A" => Some(Note::A),
        "AS" | "A#" => Some(Note::As),
        "B" => Some(Note::B),
        _ => None,
    }
}

fn parse_scale(input: &str) -> Option<ScaleType> {
    match input.to_lowercase().as_str(){
        "major" | "maior" => Some(ScaleType::Major),
        "minor" | "menor" => Some(ScaleType::MinorNatural),
        "minor_natural" | "natural" => Some(ScaleType::MinorNatural),
        _=> None,
    }
}

fn main(){
    println!("===== Gerador de Campo Harmônico =====");

     let note_input = read_input("Digite a tonalidade (ex: C, D, F#, G#, A): ");
     let note = match parse_note(&note_input){
        Some(n) => n,
        None => {
            println!("Nota invalida: {}", note_input);
            return;
        }
     };

     let scale_input = read_input("Digite o tipo de escala (major / minor): ");
     let scale_type = match parse_scale(&scale_input){
        Some(s) => s,
        None => {
            println!("Tipo de escala inválido: {}", scale_input);
            return;
        }
     };

     println!("\nGerando campo harmônico de {} ({:?})...\n", note_input, scale_type);

     let field = generate_harmonic_field(note, scale_type);

     for (i, chord) in field.iter().enumerate(){
        println!(
            "Grau {:<2}: {:<5} -> {:?}",
            i + 1,
            chord.name,
            chord.notes
        );
     }
}