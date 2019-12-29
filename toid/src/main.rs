extern crate portaudio;

use std::boxed::Box;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

use toid::data::sf2;
use toid::music_state_manager::{MusicStateEvent, MusicStateManager};
use toid::portaudio_outputter::PortAudioOutputter;
use toid::state_management::store::Store;
use toid::states::music_state::MusicState;
use toid::stores::default_store::DefaultStore;

fn main() {
    let store: Box<dyn Store<MusicState>> = Box::new(DefaultStore::new(MusicState::new()));
    let store = Arc::new(RwLock::new(store));

    let sound_state_manager = MusicStateManager::new(Arc::clone(&store));
    let sound_state_manager = Arc::new(RwLock::new(sound_state_manager));

    let mut portaudio_outputter = PortAudioOutputter::new(Arc::clone(&sound_state_manager));
    {
        let reducer = sound_state_manager.read().unwrap().get_reducer();

        let mut f = File::open("../florestan-subset.sf2").unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let buffer = buffer.as_slice();
        let sf2 = sf2::own::SF2::parse(buffer);
        let sf2 = Arc::new(sf2);

        reducer.reduce(MusicStateEvent::SetSF2(sf2));

        portaudio_outputter.run();

        reducer.reduce(MusicStateEvent::AddNewNoteOn(60.0, 0 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(62.0, 1 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(64.0, 2 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(65.0, 3 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(67.0, 4 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(69.0, 6 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(65.0, 7 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(64.0, 8 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOff(9 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(62.0, 10 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOff(11 * (44100 / 4)));
        reducer.reduce(MusicStateEvent::AddNewNoteOn(60.0, 12 * (44100 / 4)));
    }

    portaudio_outputter.sleep(4000);
    portaudio_outputter.stop();
}
