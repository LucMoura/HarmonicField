use crate::{note::Note, scale::{Scale, ScaleType}, chord::Chord};

pub fn generate_harmonic_field(root: Note, scale_type: ScaleType) -> Vec<Chord> {
    let scale = Scale::new(root, scale_type);
    let chord_labels = ["Maj7", "m7", "m7", "Maj7", "7", "m7", "m7b5"];

    let mut chords = Vec::new();

    for grau in 0..7{
        let fundamental = scale.notes[grau];

        let notes = vec![
            fundamental, 
            scale.notes[(grau + 2) % 7],
            scale.notes[(grau + 4) % 7],
            scale.notes[(grau + 6) % 7],
        ];

        let chord = Chord {
            name: format!("{:?}{}", fundamental, chord_labels[grau]),
            notes,
        };

        chords.push(chord);
    }

    chords
}