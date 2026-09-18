use std::{num::{NonZeroU16, NonZeroU32}, thread::sleep, time::{Duration, Instant}};

use rodio::{Source, mixer::mixer, source::{self, Pink, SawtoothWave, SineWave, SquareWave, WhiteUniform, noise::Brownian}};

struct Tune {
    bpm:f32,
    tracks:i32,
    patterns:Vec<Vec<f32>>,
    song:Vec<Vec<i32>>,
    volumes:Vec<Vec<f32>>,
    instruments:Vec<Instruments>,
    types:Vec<Vec<Types>>,
}

#[derive(PartialEq)]
enum Instruments {
    Sine,
    Saw,
    Sqr,
    Puresaw,
    Puresqr,
    Triangle,
    Noise,
}

#[derive(PartialEq)]
enum Types {
    Normal,
    Fadeout,
    Fadein,
}





pub fn play(month: &str) {

    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    //let query = args[1].trim().parse::<f32>().unwrap();
    //
    let mut currentmusic = Tune
    {
        bpm:120.0,
        tracks:1,
        instruments: vec![
            Instruments::Saw
        ],
        patterns: 
        vec![
            vec![69.0,0.0],
            vec![69.0,0.0]
        ],
        volumes:
        vec![
            vec![0.3,0.0],
            vec![0.3,0.0]
        ],
        types:
        vec![
            vec![Types::Normal,Types::Normal],
            vec![Types::Normal,Types::Normal]
        ],
        song:
        vec![
            vec![0]
        ]
    };

    let january = Tune
    {
        bpm:160.0,
        tracks:5,
        instruments: vec![
            Instruments::Saw,
            Instruments::Sqr,
            Instruments::Triangle,
            Instruments::Triangle,
            Instruments::Noise,
        ],
        patterns: 
        vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![48.0, 0.0, 50.0, 0.0, 51.0, 0.0, 52.0, 0.0, 48.0, 0.0, 48.0, 0.0, 45.0, 0.0, 43.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 55.0, 0.0, 55.0, 0.0, 0.0, 0.0, 0.0, 0.0, 55.0, 0.0, 0.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0],
            vec![65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0],
            vec![76.0, 76.0, 76.0, 76.0, 76.0, 76.0, 76.0, 76.0, 74.0, 74.0, 74.0, 74.0, 74.0, 74.0, 74.0, 74.0],
            vec![41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0, 41.0],
            vec![65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0, 65.0, 69.0],
            vec![74.0, 74.0, 74.0, 74.0, 74.0, 74.0, 74.0, 74.0, 72.0, 72.0, 72.0, 72.0, 72.0, 72.0, 72.0, 72.0],
            vec![64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0],
            vec![48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0],
            vec![72.0, 72.0, 72.0, 72.0, 72.0, 72.0, 72.0, 72.0, 71.0, 71.0, 71.0, 71.0, 71.0, 71.0, 71.0, 71.0],
            vec![64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0, 64.0, 67.0],
            vec![48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0,48.0],
            vec![67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0],
            vec![65.0,65.0,69.0,69.0,72.0,72.0,76.0,76.0,74.0,74.0,72.0,72.0,71.0,71.0,72.0,72.0],
            vec![64.0,64.0,67.0,67.0,71.0,71.0,74.0,74.0,72.0,72.0,71.0,71.0,69.0,69.0,64.0,64.0],
            vec![62.0,62.0,65.0,65.0,69.0,69.0,72.0,0.0,72.0,72.0,71.0,71.0,72.0,72.0,74.0,74.0],
            vec![67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0,67.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        ],
        volumes:
        vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            vec![0.3, 0.0, 0.6, 0.0, 1.0, 1.0, 0.6, 0.0, 0.5, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 0.4],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
            vec![0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4],
            vec![0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4],
            vec![0.4, 0.4, 0.4, 0.35, 0.35, 0.35, 0.3, 0.3, 0.3, 0.25, 0.25, 0.25, 0.2, 0.2, 0.1, 0.1],
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3]
        ],
        types:
        vec![
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Fadeout, Types::Normal, Types::Fadeout, Types::Normal, Types::Normal, Types::Fadeout, Types::Fadeout, Types::Normal, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal]
        ],
        song:
        vec![
            vec![2,2,2,2,2,2,2,2,15,16,17,18,15,16,17,18],
            vec![1,1,1,1,1,1,1,1,7,10,7,10,7,10,7,10],
            vec![0,0,0,0,0,0,0,0,5,8,11,13,5,8,11,13],
            vec![0,0,0,0,0,0,0,0,6,9,12,12,6,9,12,12],
            vec![3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3]
        ]
    };

    let february = Tune
    {
        bpm:160.0,
        tracks:5,

        instruments:vec![
            Instruments::Sine,
            Instruments::Triangle,
            Instruments::Saw
        ],

        patterns: vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![79.0, 79.0, 79.0, 79.0, 72.0, 72.0, 72.0, 72.0, 74.0, 74.0, 74.0, 74.0, 84.0, 84.0, 84.0, 84.0],
            vec![55.0, 55.0, 60.0, 60.0, 60.0, 60.0, 62.0, 62.0, 62.0, 62.0, 55.0, 55.0, 55.0, 55.0, 57.0, 57.0],
            vec![48.0, 48.0, 48.0, 48.0, 48.0, 48.0, 48.0, 48.0, 50.0, 50.0, 50.0, 50.0, 53.0, 53.0, 53.0, 53.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        ],

        volumes: vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125, ],
            vec![1.0, 0.5, 1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125, 1.0, 0.5],
            vec![1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125, 1.0, 0.5, 0.25, 0.125],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        ],
        
        types: vec![
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
        ],

        song:vec![
            vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            vec![0,0,0,0,2,2,2,2,2,2,2,2,2,2,2,2],
            vec![0,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3]
        ]
    };



    let mids = Tune {
        bpm:160.0,
        tracks: 10,
        instruments: vec![
            Instruments::Sqr, // beeps
            Instruments::Sqr, // beeps
            Instruments::Triangle, // drone
            Instruments::Puresaw, // melody
            Instruments::Puresaw, // melody
            Instruments::Triangle, // drone 2
            Instruments::Sine, // beep
            Instruments::Noise, // snare
            Instruments::Saw, // bassdrum
        ],
        patterns: vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![24.0, 0.0, 0.0, 0.0, 0.0, 0.0, 24.0, 0.0, 24.0, 0.0, 0.0, 0.0, 0.0, 0.0, 24.0, 0.0], // BASS DRUM              1
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], //snare                       2

            vec![60.0, 0.0, 63.0, 0.0, 64.0, 0.0, 67.0, 0.0, 70.0, 0.0, 72.0, 0.0, 70.0, 0.0, 67.0, 0.0], // PART A BEEPS       3
            vec![0.0, 0.0, 60.0, 0.0, 60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 58.0, 0.0, 58.0, 0.0, 0.0, 0.0], // backbeeps              4
            vec![60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 67.0, 67.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 67.0, 67.0], //melody 1 p1 5
            vec![70.0, 70.0, 72.0, 72.0, 70.0, 70.0, 67.0, 67.0, 65.0, 65.0, 65.0, 65.0, 65.0, 65.0, 67.0, 67.0], //melody 2 p1 6
            vec![63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 67.0, 67.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 65.0, 65.0], //melody 3 p1 7
            vec![60.0, 60.0, 62.0, 62.0, 60.0, 60.0, 70.0, 70.0, 67.0, 67.0, 67.0, 67.0, 67.0, 67.0, 0.0, 0.0], //melody 4 p1   8
            vec![60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0, 60.0], // drone p1   9
            vec![60.0, 60.0, 60.0, 60.0, 64.0, 64.0, 64.0, 64.0, 65.0, 65.0, 65.0, 65.0, 67.0, 67.0, 67.0, 67.0], // segue      10


            vec![68.0, 0.0, 63.0, 0.0, 63.0, 0.0, 60.0, 0.0, 68.0, 0.0, 63.0, 0.0, 63.0, 0.0, 60.0, 0.0], // beep 1 p2          11
            vec![63.0, 0.0, 60.0, 0.0, 60.0, 0.0, 56.0, 0.0, 63.0, 0.0, 60.0, 0.0, 60.0, 0.0, 56.0, 0.0], // Bbeep 1 p2         12
            vec![63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0, 63.0], // drone 1 p2 13

            vec![68.0, 0.0, 62.0, 0.0, 62.0, 0.0, 58.0, 0.0, 68.0, 0.0, 62.0, 0.0, 62.0, 0.0, 58.0, 0.0], // beep 2 p2          14
            vec![62.0, 0.0, 58.0, 0.0, 58.0, 0.0, 53.0, 0.0, 62.0, 0.0, 58.0, 0.0, 58.0, 0.0, 53.0, 0.0], // Bbeep 2 p2         15 
            vec![62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0], // drn 2 p2   16

            vec![67.0, 0.0, 63.0, 0.0, 63.0, 0.0, 58.0, 0.0, 67.0, 0.0, 63.0, 0.0, 63.0, 0.0, 58.0, 0.0], // beep 3 p2          17 
            vec![63.0, 0.0, 58.0, 0.0, 58.0, 0.0, 55.0, 0.0, 63.0, 0.0, 58.0, 0.0, 58.0, 0.0, 55.0, 0.0], // Bbeep 3 p2         18 
            vec![62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0, 62.0], // drn 3 p2   19

            vec![67.0, 0.0, 61.0, 0.0, 61.0, 0.0, 58.0, 0.0, 67.0, 0.0, 61.0, 0.0, 61.0, 0.0, 58.0, 0.0], // beep 4 p2          20
            vec![61.0, 0.0, 58.0, 0.0, 55.0, 0.0, 52.0, 0.0, 61.0, 0.0, 58.0, 0.0, 55.0, 0.0, 52.0, 0.0], // Bbeep 4 p2         21
            vec![61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0, 61.0], // drn 4 p2    22 

            vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], //snare 2                     23
        ],


        song: vec![
            vec![3, 3, 3, 3, 3, 3, 3, 3, 11,14,17,20], // beeps
            vec![4, 4, 4, 4, 4, 4, 4, 4, 12,15,18,21], // beeps
            vec![9, 9, 9, 9, 9, 9, 9, 9, 13,16,19,22], // drone
            vec![5, 6, 7, 8, 5, 6, 7, 10,0, 0, 0, 0,], // melody
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,], // melody
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,], // drone 2
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,], // beep
            vec![2, 23,2, 23,2, 23,2, 23,2, 23,2, 23], // snare
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,], // bassdrum
        ],

        volumes: vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0], // BASS DRUM
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0], //snare
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0], // PART A BEEPS
            vec![0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0], // backbeeps
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 0.2, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.2, 1.0, 1.0], //melody 1 p1
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.2, 1.0, 1.0], //melody 2 p1
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 0.2, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.2, 1.0, 1.0], //melody 1 p1
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.2, 0.0, 0.0], //melody 2 p1
            vec![0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3], // drone p1
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0], // segue

            
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],

            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],

            vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], //snare 2                     23
        ],

        types: vec![
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],
            vec![Types::Fadeout, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Fadeout, Types::Normal, Types::Fadeout, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Fadeout, Types::Normal], // BASS DRUM
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout], //snare
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout], // PART A BEEPS
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout], // backbeeps
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal], //melody 1 p1
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal], //melody 2 p1
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal], //melody 1 p1
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal], //melody 2 p1
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal], // drone p1
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal], // segue
            
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],

            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],

            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],

            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout],
            vec![Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal, Types::Normal],

            vec![Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout, Types::Fadeout], // PART A BEEPS
            
        ]

    };
//
    if month=="January"{
        currentmusic=january;
    } else if month=="February"{
        currentmusic=february;
    }  else if month=="Mids"{
        currentmusic=mids;
    } 

    // _stream must live as long as the sink
    let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    let sample_rate = NonZeroU32::new(48000).unwrap();

    for asdfasdf in 0..3 {
    let mut songpos = 0;
    // Add a dummy source of the sake of the example.
    use std::time::Instant;

    let start = Instant::now();

    
    
    println!("{},{},{}", currentmusic.bpm, currentmusic.bpm/60.0/16.0,  currentmusic.bpm/60.0/16.0);

    
    for song in 0..(currentmusic.song[0].len()*currentmusic.patterns[0].len()) {
        let sawmix = mixer(NonZeroU16::new(1).unwrap(), sample_rate);
        let pattpos=songpos%currentmusic.patterns[0].len();
        let fullpos: usize= ((songpos/currentmusic.patterns[0].len()) as f32).floor() as usize;
        for channel in currentmusic.instruments.iter().enumerate(){
            if currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] != 0.0 || currentmusic.song[channel.0][fullpos] != 0{
                if currentmusic.instruments[channel.0] == Instruments::Saw{
                for i in 1..9{
                    if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Normal {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/(2.0*i as f32));
                        sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadein {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/(2.0*i as f32)).fade_in(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                        sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadeout {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/(2.0*i as f32)).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                        sawmix.0.add(source);
                    }
                    
                }
                } else if currentmusic.instruments[channel.0] == Instruments::Sine {

                    if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Normal {
                    let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]);
                    sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadein {
                    let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_in(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);                    
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadeout {
                    let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                    }
                    
                } else if currentmusic.instruments[channel.0] == Instruments::Puresaw {
                    if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Normal {
                    let source = SawtoothWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]);
                    sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadein {
                    let source = SawtoothWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_in(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadeout {
                    let source = SawtoothWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                    }
                } else if currentmusic.instruments[channel.0] == Instruments::Puresqr {
                    if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Normal {
                    let source = SquareWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]);
                    sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadein {
                    let source = SquareWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_in(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadeout {
                    let source = SquareWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                    }
                } else if currentmusic.instruments[channel.0] == Instruments::Sqr {
                    for i in 0..5{
                        if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Normal {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(1.0+2.0*i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/((2.0 as f32).powf(1.0+i as f32)));
                        sawmix.0.add(source);
                        } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadein {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(1.0+2.0*i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/((2.0 as f32).powf(1.0+i as f32))).fade_in(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                        sawmix.0.add(source);
                        } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadeout {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(1.0+2.0*i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/((2.0 as f32).powf(1.0+i as f32))).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                        sawmix.0.add(source);
                        }
                    }
                } else if currentmusic.instruments[channel.0] == Instruments::Triangle {
                    for i in 0..8{
                        if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Normal {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(1.0+2.0*i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/(2.0*(1.0+i as f32)));
                        sawmix.0.add(source);
                        } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadein {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(1.0+2.0*i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/(2.0*(1.0+i as f32))).fade_in(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                        sawmix.0.add(source);
                        } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadeout {
                        let source = SineWave::new((440.0*(2.0 as f32).powf( (currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] - 69.0)/12.0 ) )*(1.0+2.0*i as f32)).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]/(2.0*(1.0+i as f32))).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                        sawmix.0.add(source);
                        }
                    }
                } else if currentmusic.instruments[channel.0] == Instruments::Noise {
                    if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Normal {
                    let source = WhiteUniform::new(sample_rate).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]);
                    sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadein {
                    let source = WhiteUniform::new(sample_rate).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_in(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                    } else if currentmusic.types[currentmusic.song[channel.0][fullpos] as usize][pattpos] == Types::Fadeout {
                    let source = WhiteUniform::new(sample_rate).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (1.0 / (currentmusic.instruments.len() as f32)) * 0.70*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]*currentmusic.volumes[currentmusic.song[channel.0][fullpos] as usize][pattpos]).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                    }
                    
                }

                if currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] < 30.0 && currentmusic.patterns[currentmusic.song[channel.0][fullpos] as usize][pattpos] > 5.0 {
                    let source = Brownian::new(sample_rate).take_duration(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) ).amplify( (2.0 / (currentmusic.instruments.len() as f32))).fade_out(Duration::from_secs_f32(( currentmusic.bpm/60.0/16.0) as f32) );
                    sawmix.0.add(source);
                }
            }
        }

        player.append(sawmix.1);
        
        songpos+=1;
        if songpos>=currentmusic.song[0].len()*16{
            songpos=0;
        }
        //if songpos<10{
        //    println!("{:?}", rodio::wav_to_file(sawmix.1, "awawa0".to_owned()+&songpos.to_string()+".wav"));
        //} else {
        //    println!("{:?}", rodio::wav_to_file(sawmix.1, "awawa".to_owned()+&songpos.to_string()+".wav"));
        //}
    }
    
    let elapsed = start.elapsed();

    player.play();

    let end = Instant::now() + Duration::from_millis((((currentmusic.song[0].len()*16) as f32 * (currentmusic.bpm/60.0/16.0)) as u64)*1000 -5);
    println!("loop");
    sleep(end - Instant::now());
    println!("deloop");
    }

}




pub fn beep() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    let sawmix = mixer(NonZeroU16::new(1).unwrap(), NonZeroU32::new(48000).unwrap());

    for i in 0..5{
        let source = SineWave::new(770.0 * (i*2+1) as f32).take_duration(Duration::from_secs_f32(0.1)).amplify(0.2/((i+1) as f32)).fade_out(Duration::from_secs_f32(0.1));
        sawmix.0.add(source);
    }
    player.append(sawmix.1);
    player.sleep_until_end();
}

pub fn pakala() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());

    let source= Brownian::new(NonZeroU32::new(48000).unwrap()).take_duration(Duration::from_secs(1)).amplify(0.4).fade_out(Duration::from_secs_f32(0.3));
    player.append(source);
    player.sleep_until_end();
}

pub fn bark() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    let sawmix = mixer(NonZeroU16::new(1).unwrap(), NonZeroU32::new(48000).unwrap());

    for i in 0..9{
        let source = SineWave::new(80.0 * (i*2+1) as f32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.04);
        sawmix.0.add(source);
    }
    player.append(sawmix.1);
    player.sleep_until_end();
}


pub fn mynothing() {

}


pub fn bells() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    let tune = [69.0, 73.0, 71.0, 64.0, 0.0, 69.0, 71.0, 73.0, 69.0, 0.0, 73.0, 71.0, 69.0, 64.0, 0.0, 69.0, 71.0, 73.0, 69.0, 0.0];
    for note in tune{
        let sawmix = mixer(NonZeroU16::new(1).unwrap(), NonZeroU32::new(48000).unwrap());
        for i in 0..5{
            let source = SineWave::new(440.0*(2.0 as f32).powf( (note - 69.0)/12.0 ) * (i*2+1) as f32).take_duration(Duration::from_secs_f32(1.0)).amplify(0.03).fade_out(Duration::from_secs_f32(1.0));
            sawmix.0.add(source);
        }
        player.append(sawmix.1);
    }
    player.sleep_until_end();
    //sleep(Duration::from_secs((0.5*19.0) as u64));
}


pub fn tada() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    let sawmix = mixer(NonZeroU16::new(1).unwrap(), NonZeroU32::new(48000).unwrap());

    for i in 0..5{
        let source = SineWave::new(130.81 * (i*2+1) as f32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.05/((i+1) as f32)).fade_out(Duration::from_secs_f32(0.2));
        sawmix.0.add(source);
    }
    for i in 0..5{
        let source = SineWave::new(329.63 * (i*2+1) as f32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.05/((i+1) as f32)).fade_out(Duration::from_secs_f32(0.2));
        sawmix.0.add(source);
    }
    for i in 0..5{
        let source = SineWave::new(392.0 * (i*2+1) as f32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.05/((i+1) as f32)).fade_out(Duration::from_secs_f32(0.2));
        sawmix.0.add(source);
    }
    for i in 0..5{
        let source = SineWave::new(493.88 * (i*2+1) as f32).take_duration(Duration::from_secs_f32(0.2)).amplify(0.05/((i+1) as f32)).fade_out(Duration::from_secs_f32(0.2));
        sawmix.0.add(source);
    }
    player.append(sawmix.1);

    player.sleep_until_end();
}