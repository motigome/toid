use std::sync::Arc;

use toid::data::music_info::{Beat, SampleNote, SamplePhrase, SampleTrack};
use toid::high_layer_trial::music_language::num_lang::send_num_lang;
use toid::music_state::states::{
    MusicState, MusicStateEvent, SchedulingStateEvent, SectionStateEvent,
};
use toid::music_state::wave_reader::{WaveReader, WaveReaderEvent};
use toid::outputters::portaudio_outputter::PortAudioOutputter;
use toid::players::local_player::LocalPlayer;
use toid::players::player::Player;

fn main() {
    let player = LocalPlayer::new();
    let player = Arc::new(player);

    player
        .get_resource_manager()
        .register(String::from("./toid-sample-resource/sf2/sf2.toml"))
        .unwrap();
    player
        .get_resource_manager()
        .register(String::from("./toid-sample-resource/drums/drums.toml"))
        .unwrap();

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&player)
        as Arc<
            dyn Player<
                MusicState,
                MusicStateEvent,
                WaveReader,
                (Vec<i16>, Vec<i16>),
                WaveReaderEvent,
            >,
        >)
    .unwrap();
    portaudio_outputter.set_volume(0.3);

    player
        .send_event(MusicStateEvent::SchedulingStateEvent(
            SchedulingStateEvent::ChangeBPM(Beat::from(0), 120.0),
        ))
        .unwrap();
    player
        .send_event(MusicStateEvent::SchedulingStateEvent(
            SchedulingStateEvent::ChangeBPM(Beat::from(8), 180.0),
        ))
        .unwrap();
    player
        .send_event(MusicStateEvent::SchedulingStateEvent(
            SchedulingStateEvent::ChangeBPM(Beat::from(16), 120.0),
        ))
        .unwrap();

    player
        .send_event(MusicStateEvent::NewSection(Beat::from(8.0)))
        .unwrap();

    send_num_lang(
        "12345 643 2 1   ".to_string(),
        0.0,
        0.0,
        Beat::from(0),
        "main".to_string(),
        Some(String::from("example_sf2")),
        1.0,
        -0.5,
        Arc::clone(&player)
            as Arc<
                dyn Player<
                    MusicState,
                    MusicStateEvent,
                    WaveReader,
                    (Vec<i16>, Vec<i16>),
                    WaveReaderEvent,
                >,
            >,
    )
    .unwrap();

    send_num_lang(
        "1   4   5   1   ".to_string(),
        -2.0,
        0.0,
        Beat::from(0),
        "sub".to_string(),
        Some(String::from("example_sf2")),
        0.7,
        0.5,
        Arc::clone(&player)
            as Arc<
                dyn Player<
                    MusicState,
                    MusicStateEvent,
                    WaveReader,
                    (Vec<i16>, Vec<i16>),
                    WaveReaderEvent,
                >,
            >,
    )
    .unwrap();

    let mut sample_phrase = SamplePhrase::new();
    sample_phrase = sample_phrase.add_note(SampleNote {
        sound: "x".to_string(),
        start: Beat::from(0),
    });
    sample_phrase = sample_phrase.set_length(Beat::from(1));
    let sample_track = SampleTrack {
        phrase: sample_phrase,
        sample_name: "example_drums".to_string(),
        vol: 1.0,
        pan: 0.0,
    };
    player
        .send_event(MusicStateEvent::SectionStateEvent(
            Beat::from(0),
            SectionStateEvent::NewSampleTrack("drums".to_string(), sample_track),
        ))
        .unwrap();

    send_num_lang(
        "5 3 4 65        ".to_string(),
        0.0,
        0.0,
        Beat::from(8),
        "main".to_string(),
        Some(String::from("example_sf2")),
        1.0,
        -0.5,
        Arc::clone(&player)
            as Arc<
                dyn Player<
                    MusicState,
                    MusicStateEvent,
                    WaveReader,
                    (Vec<i16>, Vec<i16>),
                    WaveReaderEvent,
                >,
            >,
    )
    .unwrap();

    send_num_lang(
        "3   5   4   1   ".to_string(),
        -2.0,
        0.0,
        Beat::from(8),
        "sub".to_string(),
        Some(String::from("example_sf2")),
        0.7,
        0.5,
        Arc::clone(&player)
            as Arc<
                dyn Player<
                    MusicState,
                    MusicStateEvent,
                    WaveReader,
                    (Vec<i16>, Vec<i16>),
                    WaveReaderEvent,
                >,
            >,
    )
    .unwrap();

    let mut sample_phrase = SamplePhrase::new();
    sample_phrase = sample_phrase.add_note(SampleNote {
        sound: "x".to_string(),
        start: Beat::from(0),
    });
    sample_phrase = sample_phrase.add_note(SampleNote {
        sound: "o".to_string(),
        start: Beat::from(0),
    });
    sample_phrase = sample_phrase.set_length(Beat::from(1));
    let sample_track = SampleTrack {
        phrase: sample_phrase,
        sample_name: "example_drums".to_string(),
        vol: 1.0,
        pan: 0.0,
    };
    player
        .send_event(MusicStateEvent::SectionStateEvent(
            Beat::from(8),
            SectionStateEvent::NewSampleTrack("drums".to_string(), sample_track),
        ))
        .unwrap();

    portaudio_outputter.run().unwrap();
    portaudio_outputter.sleep(2250);
    player
        .send_reader_event(WaveReaderEvent::MoveStart)
        .unwrap();
    portaudio_outputter.sleep(12000);
    portaudio_outputter.stop().unwrap();
}