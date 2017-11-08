pub struct Note {
    gesture: Gesture,
    range: Range,
    rhythm: Rhythm,
    rest: bool,
}

impl Note {
    pub fn new(g: Gesture, r:f64, rh:f64, rs: bool) -> Note {
        Note{
            gesture: g,
            range: determine_range(r),
            rhythm: determine_rhythm(rh),
            rest: rs,
        }
    }

    pub fn note_name(&self) -> &'static str {
        let g = &self.gesture;
        let r = &self.range;
        if self.rest == true {
            "r"
        } else {
            match (g, r) {
                (&Gesture::Arco, &Range::High) => "wbh",
                (&Gesture::Arco, &Range::Middle) => "wbl",
                (&Gesture::Arco, &Range::Low) => "bd",
                (&Gesture::Pizzicato, &Range::High) => "hh",
                (&Gesture::Pizzicato, &Range::Middle) => "cb",
                (&Gesture::Pizzicato, &Range::Low) => "hhp",
                (&Gesture::Percussion, &Range::High) => "tomh",
                (&Gesture::Percussion, &Range::Middle) => "rb",
                (&Gesture::Percussion, &Range::Low) => "toml",
                (_, _) => "wbh"
            }
        }
    }
    pub fn note_rhythm (&self) -> &'static str {
        let r = &self.rhythm;
        match r {
            &Rhythm::Whole => "1",
            &Rhythm::Half => "2",
            &Rhythm::HalfDot => "2.",
            &Rhythm::Quarter => "4",
            &Rhythm::QuarterDot => "4.",
            &Rhythm::Eigth => "8",
            &Rhythm::EigthDot => "8.",
            &Rhythm::Sixteenth => "16",
            &Rhythm::SixteenthDot => "16.",
            &Rhythm::ThirtySecond => "32",
            &Rhythm::ThirtySecondDot => "32.",
            &Rhythm::SixtyFourth => "64",
        }
    }

    pub fn gen_note(&self) -> (&'static str, &'static str){
        (self.note_name(), self.note_rhythm())
    }
}

pub enum Gesture{
    Arco,
    Pizzicato,
    Percussion,
}
pub enum Range {
    High,
    Middle,
    Low,
}

pub enum Rhythm {
    Whole,
    Half,
    HalfDot,
    Quarter,
    QuarterDot,
    Eigth,
    EigthDot,
    Sixteenth,
    SixteenthDot,
    ThirtySecond,
    ThirtySecondDot,
    SixtyFourth,
}

pub enum R {
    Whole,
    Half,
    HalfDot,
    HalfTri,
    HalfQuint,
    Quarter,
    QuarterDot,
    QuarterTri,
    QuarterQuint,
    Eigth,
    EigthDot,
    EigthTri,
    EigthQuint,
    Sixteenth,
    SixteenthDot,
    SixteenthTri,
    SixteenthQuint,
    ThirtySecond,
    ThirtySecondDot,
    ThirtySecondTri,
    ThirtySecondQuint,
    SixtyFourth,
}

pub struct Phrase {
    notes: Vec<Note>,
    rhythms: Vec<Rhythm>,
    pattern: u8,
    pulses: u8,
    beat: u8,
}

impl Phrase {
    pub fn new(pattern: u8, pulses: u8, beat: u8)->Phrase{
        let x: u8 = 0b10000000;
        let mut phrase = Phrase {
            notes: vec!{},
            rhythms: vec!{},
            pattern: 0,
            pulses: 0,
            beat: 0,
        };
        for position in 0..pulses{
            if pattern | x >> position !=pattern{
                phrase.notes.push(Note::new(gesture(pattern, position),
                                            10.0, 10.0, true));
            } else {
                phrase.notes.push(Note::new(Gesture::Pizzicato,
                                            10.0, 10.0, false));
            }
        }
        phrase
    }
}

pub fn gesture(g: u8, i: u8) -> Gesture{
    let arco = g |0b00 << i;
    let pizz = g |0b01 << i;
    let perc = g |0b11 << i;

        match g {
            arco => Gesture::Arco,
            pizz => Gesture::Pizzicato,
            perc => Gesture::Percussion,
        }
}

pub fn rhythm(pulses: u8, beat: u8) -> R{
    match (pulses, beat) {
        (1,1) => R::Whole,
        (2,2) => R::Half,
        (3,2) => R::HalfTri,
        (2,4) => R::Eigth,
        (3,4) => R::EigthTri,
        (4,4) => R::Sixteenth,
        (2,8) => R::Sixteenth,
        (3,8) => R::SixteenthTri,
        (4,8) => R::ThirtySecond,
        (2,16) => R::ThirtySecond,
        (3,16) => R::ThirtySecondTri,
        (_,_) => R::Whole,
    }
}
/*
pub fn determine_gesture(element: f64) -> Gesture{
    let constrained_elem = (element as i64) % 3;

    match constrained_elem {
        0 => Gesture::Arco,
        1 => Gesture::Pizzicato,
        2 => Gesture::Percussion,
        _ => Gesture::Arco,
    }
}
 */

pub fn determine_range(element: f64) -> Range {
    let constrained_elem = (element as i64) % 3;

    match constrained_elem {
        0 => Range::High,
        1 => Range::Middle,
        2 => Range::Low,
        _ => Range::High,
    }
}

pub fn determine_rhythm(element: f64) -> Rhythm{
    let rhythm = (element as i64) % 13;

    match rhythm {
        0 => Rhythm::Whole,
        2 => Rhythm::Half,
        3 => Rhythm::HalfDot,
        4 => Rhythm::Quarter,
        5 => Rhythm::QuarterDot,
        6 => Rhythm::Eigth,
        7 => Rhythm::EigthDot,
        8 => Rhythm::Sixteenth,
        9 => Rhythm::SixteenthDot,
        10 => Rhythm::ThirtySecond,
        11 => Rhythm::ThirtySecondDot,
        13 => Rhythm::SixtyFourth,
        _ => Rhythm::Quarter,
    }
}

