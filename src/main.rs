extern crate rand;
extern crate gnuplot;
use rand::{thread_rng, Rng};
use gnuplot::*;
use std::fs::File;
use std::io::prelude::*;
mod lily;
mod notetree;
use lily::*;
use notetree::*;
#[allow(dead_code)]
fn main() {

    //We need to generate some files for csound and lilypond rn because I'm not using the direct apis
    let mut file = match File::create("data.txt"){
        Err(why) => panic!("Couldn't create/erase data: {}", why),
        Ok(file) => file,
    };

    let mut graph = match File::create("plot.txt"){
        Err(why) => panic!("Couldn't create/erase data: {}", why),
        Ok(file) => file,
    };

    let mut lily = match File::create("lily-plot.ly"){
        Err(why) => panic!("Couldn't create/erase data: {}", why),
        Ok(file) => file,
    };

    //Strings for making files. @MoveStart
    let fore = "<CsoundSynthesizer>\n<CsOptions>\n</CsOptions>
<CsInstruments>

sr = 44100
ksmps = 32
nchnls = 2
0dbfs  = 1

instr 1
  ; Attack time.
  iattack = 0.1
  ; Decay time.
  idecay = 0.01
  ; Sustain level.
  isustain = 0.001
  ; Release time.
  irelease = 0.01
  idelay = p5
  aenv madsr iattack, idecay, isustain, irelease

  aSource vco2 p4/2.0, p5
  as2 vco2 p4/2.0, p5*3/2.0
  as3 vco2 p4/2.0, (p5*6/2.0)
  afin = ((aSource + as2+as3) * aenv)
  aL, aR freeverb afin, afin, 0.5, 0.9, sr, 0
  adl1 vdelay aL, idelay+(p3*100), 5000
  adl2 vdelay adl1, idelay+(p3*100), 5000
  outs aL+adl1+adl2, aR+adl1+adl2
endin


instr 2
  aLFO vco2 1, 1000
  aenv madsr p3/10.0, 0.01, 0.010, 1.0
  kfo = k(aLFO)
  aSource poscil 1,(kfo+1)*100 + p5
  as2     poscil 1, (kfo+1)*100 + (p5*4)
  as3     poscil 1, (kfo+1)*10 + (p5*8/3)
  as4     poscil 1, (kfo+1)*10 + (p5*8/5)
  as = as2+as3+as4
  afilter butterbp aSource, p2*2.0, 100
  af2     butterbp as, p2*2.0, p3
  asig = (af2+afilter) * aenv
  aL, aR freeverb asig, asig, 0.8, 0.95, sr, 0
  a1, a2 pan2 aL, 0.0
  outs a1*2.0, a2*2.0
endin


gifn	ftgen	0,0, 257, 9, .5,1,270,1.5,.33,90,2.5,.2,270,3.5,.143,90,4.5,.111,270

instr 3

  kpcs = 440+p5
  kcar = 1
  kmod = p3 * 1001
  kndx line 0, 0.1, 1.0
  aLFO vco2 1, 10
  aFM vco2 1, p5
  afm2 vco2 1, p5* 3.0/2.0
  as vco2 1+k(aLFO), k(aFM*1000)+p5
  af foscil p4/5.0, kpcs, kcar, kmod, kndx
  asource distort af+aFM+afm2, 0.99, gifn
  knum init 1.00
  knum = knum +0.1
  k2 = (aLFO + 2.00)/2.0
  aenv madsr p3, 0.1, 0.00, 2.0
  afin = aenv*asource
  aL, aR freeverb afin, afin, 0.91, 0.1, sr, 0
  a1, a2 pan2 aL, 1.0
  outs a1/5.0, a2/5.0
endin

instr 4

  krd rand 1, 10
  krand abs krd
  ; Attack time.
  iattack = 0.8 -  i(krand) 
  ; Decay time.
  idecay = 0.02 + i(krand)
  ; Sustain level.
  isustain = 0.0001
  ; Release time.
  irelease = 0.001
  ; Delay line control
  idelay = p5 +i(krand)

  aenv madsr iattack, idecay, isustain, irelease

  aSource vco2 p4/2.0, p5
  as2 vco2 p4/2.0, p5*3/2.0
  as3 vco2 p4/2.0, (p5*krand/2.0)
  as4 vco2 p4/2.0, (p5*3/(2.0+krand))
  afin = ((aSource+as2+as3+as4) * aenv)
  aL, aR freeverb afin, afin, 0.5, 0.9, sr, 0
  adl1 vdelay aL, idelay+(p3*100), 5000
  adl2 vdelay adl1, idelay+(p3*100), 5000
  adl3 vdelay adl2, idelay+(p3*100), 5000
  adl4 vdelay adl3, idelay+(p3*100), 5000
  adl5 vdelay adl4, idelay+(p3*100), 5000
  outs aL+adl1+adl2, aR+adl3+adl4
endin

instr 5

  aLFO vco2 1, 4
  aenv madsr 2.0, 0.1, 0.00, 0.5
  kfo = k(aLFO)
  aSource poscil 1,(kfo+1)*100 + p5
  as2     poscil 1, (kfo+1)*100 + (p5*4)
  as3     poscil 1, (kfo+1)*10 + (p5*8/3)
  as4     poscil 1, (kfo+1)*10 + (p5*8/5)
  as = as2+as3+as4
  afilter butterbp aSource, p2*2.0, 100
  af2     butterbp as, p2*2.0, p3
  asig = (as - afilter +af2) * aenv * aLFO
  aL, aR freeverb asig, asig, 0.8, 0.95, sr, 0
  a1, a2 pan2 aL, 0.0
  outs a1*2.0, a2*2.0
endin

</CsInstruments>
<CsScore>

f1 0 32768 10

";

    let aft = "</CsScore>\n</CsoundSynthesizer>";

    let lilyopen = "\\version \"2.18.2\" \n \\score { \n \\new Staff << \n \\new Voice  { \n";
    let lilyclose = "} \n>> \n \\layout {\n \\context{\n \\Staff\n \\remove \"Time_signature_engraver\" \\remove \"Bar_engraver\" \n }\n}\n}";

    //@MoveEnd

    file.write_all(fore.as_bytes());
    lily.write(lilyopen.as_bytes());

    // make the vector-based plots
    let ikedas = iterate_fn(&ikeda, 0.0, 1.0);
    let henons = iterate_fn(&hen, 0.0, 0.0);
    let ikeda_iter = ikedas.0.iter().zip(ikedas.1.iter());
    let henon_iter = henons.0.iter().zip(henons.1.iter());
    let h_and_i_iter = henon_iter.zip(ikeda_iter);

    // csound calculates when to play a note based on the note's start time and duration
    // if we want a melodic texture for our score, we need to advance some amount of time after each note is played
    let mut current_time: f64 = 0.0;
    let mut advance: f64 = 0.0;
    
    //iterate over the data to make our various files
    for col in h_and_i_iter
    {
        // Lets make some musical data!
        let ((henon_x, henon_y,), (ikeda_x, ikeda_y)) = col;
        let amplitude =  ((*henon_y*100.0) + 120.0)/100.0;
        let mut freq = (*ikeda_x * 100.0) as i64 + 30 ;
        if freq < 1 {
            freq = 30
        }
        let instrument = (freq % 5)+1;

        let mut rand = true;
        if thread_rng().gen_range(0, 100) as i32 > 10{
            rand = false;
        }

        /*
        let note = Note::new(henon_x.abs()*100.0, henon_y.abs()*100.0, ikeda_y.abs()*100.0, rand);
        let note_n = note.note_name();
        let note_r = note.note_rhythm();
        */
        //Lets write that data to our files
        let phrase = notetree::make_phrase();
        lily.write(phrase.as_bytes());
        write!(file, "i {} {} {} {} {} \n", instrument,
               current_time + henon_x*2.0, ikeda_y.abs()*7.0,
               amplitude/10.0-1.0, freq);
        write!(graph, "{} {} {} {}\n", freq, ikeda_y, henon_x, henon_y);

        // Advance time
        current_time +=ikeda_y.abs()*3.0;
        advance += 1.0;
    }
    
    file.write_all(aft.as_bytes());
    lily.write_all(lilyclose.as_bytes());


    // @Debugging
    /*
    let mut  v: Vec<(&Fn(f64,f64) -> (f64,f64))> = Vec::new();
    let ik = &ikeda;
    v.push(&ik);*/
}


/*
fn plot_all(fs: Vec<&Fn(f64, f64) -> (f64, f64)>){
   // let mut final_iter;
    let mut i = false;
    let mut final_vec: Vec<Vec<(&f64, &f64)>> = Vec::new();
    let mut vecs: (Vec<f64>, Vec<f64>);
    let mut xs: Vec<f64>;
    let mut ys: Vec<f64>;
    for f in fs {
        vecs = iterate_fn(&f, 0.0, 0.0).clone();
        // println!("{:?}", vecs);
        xs = vecs.0.clone();
        ys = vecs.1.clone();
        final_vec.push(xs.iter().zip(ys.iter()).collect());
        println!("{:?}", final_vec);
        i = true;
    }
}*/

fn find_devision(bpm: f64, seconds: f64)-> u64{
   return (((60.0/bpm) * seconds) * 100.0).ceil() as u64;
}
fn plot(vecs: (Vec<f64>, Vec<f64>)){
    let mut fg = Figure::new();
    fg.axes2d().points(vecs.0, vecs.1, &[Caption("Iterative function: "), Color("#cc00ff"), PointSize(0.02)]);
    fg.show();
    //return vecs;
}

fn ikeda_code(){
    let mut rng = rand::thread_rng();
    //let mut x;
    //let mut y;

    for i in 1..10 {

        //vecs = iterate_fn(&ikeda, rng.gen::<f64>(), rng.gen::<f64>());
        //x = vecs.0;
        //y = vecs.1;
        //fg.axes2d()
        //    .points(&x, &y, &[Caption("Ikeda iterations "), Color("#cc00ff"), PointSize(0.02)])
        //    .set_title("Ikeda Iterations", &[Font("Go Mono Regular", 24.0)]);
    }
}

#[allow(dead_code)]
fn duffing(x: f64, y: f64 ) -> (f64, f64){
    let b: f64 = 0.15;
    let a: f64 = 2.75;
    let new_x: f64 = y;
    let new_y: f64 = -(b*x) + (a*y) + (y*y*y);
    return (new_x, new_y);
}

#[allow(dead_code)]
fn ginger(x: f64, y: f64 ) -> (f64, f64){

    let new_x: f64 = 1.0 - y + x.abs();
    let new_y: f64 = x;
    return (new_x, new_y);
}

#[allow(dead_code)]
fn hen (x: f64, y: f64 ) -> (f64, f64){
    // The typical a and b values
    /*
    let a: f64 = 1.4;
    let b: f64 = 0.3;
     */
    let a: f64 = 0.2;
    let b: f64 = -0.9999;
    let new_x: f64 = y + 1.0 - (a * (x * x));
    let new_y: f64 = b*x;
    return (new_x, new_y);
}

#[allow(dead_code)]
fn tinker (x: f64, y: f64 ) -> (f64, f64){
    let a: f64 = 0.9;
    let b: f64 = -0.6013;
    let c: f64 = 2.0;
    let d: f64 = 0.5;
    let new_x: f64 = (x*x) - (y*y) + a*x + b*y;
    let new_y: f64 = 2.0*(x*y) + c*x + d*y;
    return (new_x, new_y);
}

#[allow(dead_code)]
fn ikeda (x: f64, y: f64) -> (f64, f64){
    let u: f64 = 0.9;
    let t: f64 = 0.4 - (6.0 / (1.0 + (x * x) + (y * y)));
    let new_x: f64 = 1.0 + (u * ((x * t.cos()) - (y * t.sin())));
    let new_y: f64 = u * ((x * t.sin())+ (y * t.cos()));
    return (new_x, new_y);
}


fn iterate_fn (f: &Fn(f64, f64) -> (f64,f64),
                x: f64,
                y: f64,) -> (Vec<f64>, Vec<f64>){
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut xa;
    let mut ya;

    xa = x;
    ya = y;

    for i in 0..600 {
        let new_tuple: (f64, f64) = f(xa,ya);
        if i > 20 {
        xs.push(new_tuple.0);
        ys.push(new_tuple.1);
        }
        xa = new_tuple.0;
        ya = new_tuple.1;
    }

    return (xs, ys)

}
