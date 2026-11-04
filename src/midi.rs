use itertools::Itertools;
use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;
use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::fs::File;
use std::sync::Arc;
use tinyaudio::prelude::*;

pub fn play (month: &str){
    // Setup the audio output.
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    // Buffer for the audio output.
    let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
    let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Load the SoundFont.
    let mut msf = "assets/midi.sf2";
    if month=="March"{
        msf="assets/mar.sf2";
    }
    let mut sf2 = File::open(msf).unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let mut song = "mids";
    if month=="January"{
        song = "girlp";
    }else if month=="February"{
        song = "february";
    }else if month=="March"{
        song = "march";
    }else if month=="April"{
        song = "feuill";
    }else if month=="May"{
        song = "may";
    }else if month=="Mids"{
        song = "mids";
    }else if month=="June"{
        song = "hellishfire";
    }else if month=="September"{
        song = "sept";
    }else if month=="November"{
        song = "eterno";
    }else if month=="December"{
        song = "thankgoodnessitsnotfestivehere";
    }
    
    let mut mid = File::open("assets/".to_owned() + song + ".mid").unwrap();
    let midi_file = Arc::new(MidiFile::new(&mut mid).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // Play the MIDI file.
    sequencer.play(&midi_file, true);

    // Start the audio output.
    let _device = run_output_device(params, {
        move |data| {
            sequencer.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();
    
    // Wait for 10 seconds.
    std::thread::sleep(std::time::Duration::from_millis((midi_file.get_length() *4000.0) as u64 ));
}