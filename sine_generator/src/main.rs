use sine_generator::{WavSpec, writer::WavWriter, frequency_writer::{SineWavSpec, self}};

const PRIM: (u16, u16) = (1, 1);
const MIN_SEC: (u16, u16) = (16, 15);
const MAJ_SEC: (u16, u16) = (9, 8);
const MIN_THIRD: (u16, u16) = (6, 5);
const MAJ_THIRD: (u16, u16) = (5, 4);
const P_FOURTH: (u16, u16) = (4, 3);
const TRITONE: (u16, u16) = (45, 32);
const P_FIFTH: (u16, u16) = (3, 2);
const MIN_SIXTH: (u16, u16) = (8, 5);
const MAJ_SIXTH: (u16, u16) = (5, 3);
const MIN_SEV: (u16, u16) = (9, 5);
const MAJ_SEV: (u16, u16) = (15, 8);    
const OCTAVE: (u16, u16) = (2, 1);    

struct Scale {
    relations: Vec<(u16, u16)>
}

impl Scale {
    pub fn ionian() -> Self {
        let relations = vec![
            PRIM,
            MAJ_SEC,
            MAJ_THIRD,
            P_FOURTH,
            P_FIFTH,
            MAJ_SIXTH,
            MAJ_SEV,
            OCTAVE,
        ];
        Scale { relations }
    }

    pub fn dorian() -> Self {
        let relations = vec![
            PRIM,
            MAJ_SEC,
            MIN_THIRD,
            P_FOURTH,
            P_FIFTH,
            MAJ_SIXTH,
            MIN_SEV,
            OCTAVE,
        ];
        Scale { relations }
    }

    pub fn phrygian() -> Self {
        let relations = vec![
            PRIM,
            MIN_SEC,
            MIN_THIRD,
            P_FOURTH,
            P_FIFTH,
            MIN_SIXTH,
            MIN_SEV,
            OCTAVE,
        ];
        Scale { relations }
    }

    pub fn lydian() -> Self {
        let relations = vec![
            PRIM,
            MAJ_SEC,
            MAJ_THIRD,
            TRITONE,
            P_FIFTH,
            MAJ_SIXTH,
            MAJ_SEV,
            OCTAVE,
        ];
        Scale { relations }
    }

    pub fn mixolydian() -> Self {
        let relations = vec![
            PRIM,
            MAJ_SEC,
            MAJ_THIRD,
            P_FOURTH,
            P_FIFTH,
            MAJ_SIXTH,
            MIN_SEV,
            OCTAVE,
        ];
        Scale { relations }
    }

    pub fn aeolian() -> Self {
        let relations = vec![
            PRIM,
            MAJ_SEC,
            MIN_THIRD,
            P_FOURTH,
            P_FIFTH,
            MIN_SIXTH,
            MIN_SEV,
            OCTAVE,
        ];
        Scale { relations }
    }

    pub fn locrian() -> Self {
        let relations = vec![
            PRIM,
            MIN_SEC,
            MIN_THIRD,
            P_FOURTH,
            TRITONE,
            MIN_SIXTH,
            MIN_SEV,
            OCTAVE,
        ];
        Scale { relations }
    }

    pub fn write(&self, base_freq: u16) -> Vec<u16> {
        let mut frequencies = vec![]; 
        for (i, j) in &self.relations {
            frequencies.push(base_freq * i / j);
        }

        frequencies
    }
}

fn main() {

    let spec = WavSpec::new(1, 44100, 16).expect("Invalid specification");
    //let sinespec = ...
    let writer = WavWriter::new_with_spec(spec, "ionian.wav").expect("Error creating file");
    //write_wav(sinespec)..
    // let freqs: Vec<u16> = vec![440, 440 * 16 / 15, 440 * 9 / 8, 440 * 6 / 5]; // ein vektor aus frequenzen
    let scale = Scale::ionian();
    let freqs = scale.write(440);
    let sine_spec = SineWavSpec::new(&spec, freqs.to_vec(), 20, 0.).expect("invalid specification");

    frequency_writer::write_wave(sine_spec, writer);
}