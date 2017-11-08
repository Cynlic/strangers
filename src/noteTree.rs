/*
Context-free grammar for tuplets?

start => (prob pulse or tuplet or rest)
tuplet => (prob of all tuplets)
pulse => (prob of all pulses)
rest => (prob of all rests)

 */

use rand;

pub enum NonTerminal {
    Start,
    Tuplet,
    Pulse,
    Rest,
    Pitch,
    Rhythm,
    Technique,
    PRest,
}

fn g_start (sentence: &mut Vec<NonTerminal>) {
    let mut a: i32 = rand::random();
    let a = a % 100;

    if a < 90 {
        sentence.push(NonTerminal::Pulse);
        sentence.push(NonTerminal::Rest);
    } else {
        sentence.push(NonTerminal::Tuplet);
        sentence.push(NonTerminal::Start);
    }
}

fn g_pulse(sentence: &mut Vec<NonTerminal>) {
    let mut a: i32 = rand::random();
    let a = a % 100;

    if a > 90 {
        sentence.push(NonTerminal::Pitch);
        sentence.push(NonTerminal::Rhythm);
        sentence.push(NonTerminal::Pulse);
    } else {
        sentence.push(NonTerminal::Pitch);
        sentence.push(NonTerminal::Rhythm);
    }
}

fn g_tuplet (sentence: &mut Vec<NonTerminal>) {
    sentence.push(NonTerminal::Tuplet)
}

fn g_rest(sentence: &mut Vec<NonTerminal>) {
    let mut a: i32 = rand::random();
    let a = a % 100;

    if a > 97 {
        sentence.push(NonTerminal::Start);
    } else {
        sentence.push(NonTerminal::PRest);
        sentence.push(NonTerminal::Rhythm)
    }
}

fn eval_nonterms (sentence:& Vec<NonTerminal>) -> Vec<NonTerminal>{
    let mut new_s: Vec<NonTerminal> = vec!{};
    let s_iter = sentence.iter();

    for word in s_iter {
        match word {
            &NonTerminal::Start => g_start(&mut new_s),
            &NonTerminal::Pulse => g_pulse(&mut new_s),
            &NonTerminal::Tuplet => g_tuplet(&mut new_s),
            &NonTerminal::Rest => g_rest(&mut new_s),
            &NonTerminal::PRest => new_s.push(NonTerminal::PRest),
            &NonTerminal::Pitch => new_s.push(NonTerminal::Pitch),
            &NonTerminal::Rhythm => new_s.push(NonTerminal::Rhythm),
            &NonTerminal::Technique => new_s.push(NonTerminal::Technique),
        }
    }
    new_s
}

fn make_sentence() -> Vec<NonTerminal> {
    let mut sentence: Vec<NonTerminal> = vec!{NonTerminal::Start};
    for i in 0..10 {
        sentence = eval_nonterms(&sentence);
    }
    sentence
}

fn gen_pitch() -> String {
    let mut a: i32 = rand::random();
    let a = a.abs() % 9;

    match a {
        0=> "wbh".to_string(),
        1 => "wbl".to_string(),
        2 => "bd".to_string(),
        3 => "hh".to_string(),
        4 => "cb".to_string(),
        5 => "hhp".to_string(),
        6 => "tomh".to_string(),
        7 => "rb".to_string(),
        8 => "toml".to_string(),
        _ => "wbh".to_string(),
    }
}

fn gen_rhythm() -> String {

    let mut a: i32 = rand::random();
    let a = a.abs() % 9;

    match a {
        0 => "1".to_string(),
        1 => "2".to_string(),
        2 => "4".to_string(),
        3 => "8".to_string(),
        4 => "16".to_string(),
        5 => "32".to_string(),
        6 => "64".to_string(),
        7 => "2.".to_string(),
        8 => "4.".to_string(),
        _ => "8".to_string(),
    }
}

fn gen_subdivision(num: i32)-> &'static str{
    match num {
        1 => "2 ",
        2 => "4 ",
        4 => "8 ",
        8 => "16 ",
        16 => "32 ",
        32 => "64 ",
        _ => "8 ",
    }
}

fn gen_beat() -> i32 {
    let mut a: i32 = rand::random();
    a = a.abs() % 7;
    match a {
        0 => 1,
        1 => 2,
        2 => 4,
        3 => 8,
        4 => 16,
        5 => 32,
        6 => 64,
        _ => 4,
    }
}

fn is_beamable(rhythm: &str) -> bool {
    match rhythm {
        "1" => false,
        "2" => false,
        "2." => false,
        "4" => false,
        "4." => false,
        "1." => false,
        _ => true,
    }
}

fn gen_tuplet() -> String {
    let mut times: i32 = rand::random();
    times = (times.abs() % 9) + 1;
    if times % 2 == 0 {
        times = times + 1;
    }
    let beat = times - 1;
    if times == 1 {
        gen_pitch() + &gen_rhythm()[..]
    }
    else {
        let mut tuplet: String = format!(" \\autoBeamOff \\tuplet {}/{}", times, beat).to_string();
        tuplet = tuplet + "{";
        let mut r = &gen_rhythm()[..];
        if r == "1" {
            r = "2";
        }
        for i in 1..(times+1) {
            tuplet = tuplet + &gen_pitch()[..] + r + " ";
            if i == 1 && is_beamable(r) {
                tuplet = tuplet + "[";
            }
        }
        if is_beamable(r){
            tuplet = tuplet + "]"
        }
        tuplet = tuplet + "} \\autoBeamOn ";
        tuplet
    }
}

fn to_terminals(sentence: Vec<NonTerminal>) -> String {
    let s_iter = sentence.iter();
    let mut phrase: String = "".to_string();
    for nt in s_iter {
        match nt {
            &NonTerminal::PRest =>{
                phrase = phrase + " r";
            },
            &NonTerminal::Pitch =>{
                phrase = phrase + " " + &gen_pitch()[..];
            },
            &NonTerminal::Rhythm => {
                phrase = phrase + &gen_rhythm()[..];
            },
            &NonTerminal::Technique => {
                phrase = phrase + "";
            },
            &NonTerminal::Tuplet => {
                phrase = phrase + &gen_tuplet()[..];
            },
            _ => {
                phrase = phrase + "";
            },
        }
    }
    phrase
}

pub fn make_phrase() -> String {
    let mut sentence:Vec<NonTerminal> = make_sentence();
    to_terminals(sentence)
}

/*
pub struct NoteTree {
    children: Vec<NoteTree>,
    rhythm: RhythmType,
    beat: f32,
    pulse: f32,
}

impl NoteTree {

    pub fn rhythm_to_str(&self) -> &'static str {
        let r = &self.rhythm;
        match r {
            &RhythmType::Pulse => "4",
            &RhythmType::TupletOpen => "3/2 {",
            &RhythmType::TupletClose => "}",
            &RhythmType::Rest => "r",
        }
    }

    pub fn pulses_to_rhythm(&self) -> &'static str {
        let b = &self.beat;
        let p = &self.pulse;

        match *b / *p {
            1.0 => "w",
            0.75 => "2.",
            0.5 => "2",
            0.375 => "4.",
            0.25 => "4",
            0.2875 => "8.",
            0.125 => "8",
            0.09375 => "16.",
            0.0625 => "16",
            0.046875 => "32.",
            0.03125 => "32",
            0.015625 => "64",
            0.0078125 => "64",
            _ => "4",
        }
    }

    /*

    pub fn new(pulses: f32, beat: f32, rhythm: RhythmType) -> NoteTree {
        let mut children Vector<NoteTree> = vec!{};
        if rhythm = RhythmType::TupletOpen {
            // Spawn tuplets

        } else {
            // Spawn note with no children
        }
    }*/
}
*/
