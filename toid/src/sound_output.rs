//! MVP的にmonophonicなメロディをライブコーディングするモジュールです。
//! sound_outputモジュールでは、stateモジュールのユースケースとなります。
use std::f64::consts::PI;
use std::sync::Arc;
use std::sync::RwLock;
use std::vec::Vec;

use super::state_management::reducer::Reduce;
use super::state_management::reducer::Reducer;
use super::state_management::store::Store;
use super::states::flex_state::FlexState;
use super::states::flex_state::ManualState;

/// state::Storeで使う用のStateです。
/// SoundStateから、audioのAPIのコールバックで使う用の波形が取得できます。
pub struct SoundState {
    phase: f32,
    pitch: i32,
    sound_on: bool,
    wave_length: usize,
}

impl SoundState {
    pub fn new(wave_length: usize) -> Self {
        SoundState {
            phase: 0.0,
            pitch: 60,
            sound_on: true,
            wave_length,
        }
    }
}

impl ManualState for SoundState {
    fn get_by_address(&self, address: String) -> Result<FlexState, String> {
        match &*address {
            "phase" => Ok(FlexState::F32(self.phase)),
            "pitch" => Ok(FlexState::I32(self.pitch)),
            "sound_on" => Ok(FlexState::Bool(self.sound_on)),
            "wave_length" => Ok(FlexState::Usize(self.wave_length)),
            _ => Err(String::from("invalid address")),
        }
    }

    fn update(&self, address: String, value: FlexState) -> Result<FlexState, String> {
        match &*address {
            "phase" => Ok(FlexState::ManualState(Arc::new(SoundState {
                phase: value.unwrap_f32(),
                pitch: self.pitch,
                sound_on: self.sound_on,
                wave_length: self.wave_length,
            }))),
            "pitch" => Ok(FlexState::ManualState(Arc::new(SoundState {
                phase: self.phase,
                pitch: value.unwrap_i32(),
                sound_on: self.sound_on,
                wave_length: self.wave_length,
            }))),
            "sound_on" => Ok(FlexState::ManualState(Arc::new(SoundState {
                phase: self.phase,
                pitch: self.pitch,
                sound_on: value.unwrap_bool(),
                wave_length: self.wave_length,
            }))),
            "wave_length" => Ok(FlexState::ManualState(Arc::new(SoundState {
                phase: self.phase,
                pitch: self.pitch,
                sound_on: self.sound_on,
                wave_length: value.unwrap_usize(),
            }))),
            _ => Err(String::from("invalid address")),
        }
    }

    fn contains_address(&self, address: String) -> bool {
        match &*address {
            "phase" => true,
            "pitch" => true,
            "sound_on" => true,
            "wave_length" => true,
            _ => false,
        }
    }
}

pub struct SoundStateManager {
    store: Arc<RwLock<Store<FlexState>>>,
    reducer: Reducer<FlexState, SoundStateEvent>,
}

impl SoundStateManager {
    pub fn new(
        store: Arc<RwLock<Store<FlexState>>>,
        reducer: Reducer<FlexState, SoundStateEvent>,
    ) -> Self {
        SoundStateManager { store, reducer }
    }
    pub fn get_wave(&self) -> Vec<f32> {
        let mut ret = Vec::new();
        let state = self.store.read().unwrap().get_state();

        let pitch = state
            .unwrap_manual_state()
            .get_by_address(String::from("pitch"))
            .unwrap()
            .unwrap_i32();
        let phase = state
            .unwrap_manual_state()
            .get_by_address(String::from("phase"))
            .unwrap()
            .unwrap_f32();
        let sound_on = state
            .unwrap_manual_state()
            .get_by_address(String::from("sound_on"))
            .unwrap()
            .unwrap_bool();
        let wave_length = state
            .unwrap_manual_state()
            .get_by_address(String::from("wave_length"))
            .unwrap()
            .unwrap_usize();
        let hertz = self.get_hertz(pitch);

        if sound_on {
            for wave_idx in 0..wave_length {
                let ret_ = phase + (wave_idx as f32) * hertz / (44100 as f32);
                let ret_ = ret_ * 2.0 * (PI as f32);
                let ret_ = ret_.sin();
                ret.push(ret_);
            }
            let next_phase = phase + (wave_length as f32) * hertz / (44100 as f32) % 1.0;
            self.reducer
                .reduce(SoundStateEvent::ChangePhase(next_phase));
        } else {
            for _ in 0..wave_length {
                ret.push(0.0);
            }
        }
        ret
    }

    fn get_hertz(&self, pitch: i32) -> f32 {
        // A4 -> 69 440hz
        (440 as f32) * (2.0 as f32).powf(((pitch - 69) as f32) / 12 as f32)
    }
}

pub enum SoundStateEvent {
    ChangePitch(i32),
    SoundOn,
    SoundOff,
    ChangePhase(f32),
}

pub struct SoundStateReduce {}

impl Reduce<FlexState, SoundStateEvent> for SoundStateReduce {
    fn reduce(&self, state: FlexState, event: SoundStateEvent) -> FlexState {
        match event {
            SoundStateEvent::ChangePitch(pitch) => state
                .unwrap_manual_state()
                .update(String::from("pitch"), FlexState::I32(pitch))
                .unwrap(),
            SoundStateEvent::SoundOn => state
                .unwrap_manual_state()
                .update(String::from("sound_on"), FlexState::Bool(true))
                .unwrap(),
            SoundStateEvent::SoundOff => state
                .unwrap_manual_state()
                .update(String::from("sound_on"), FlexState::Bool(false))
                .unwrap(),
            SoundStateEvent::ChangePhase(phase) => state
                .unwrap_manual_state()
                .update(String::from("phase"), FlexState::F32(phase))
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is_close(a: f32, b: f32, delta: f32) {
        if (a - b).abs() > delta {
            panic!("is not close: {} {}", a, b)
        }
    }

    #[test]
    fn state_works() {
        let initial_state = FlexState::ManualState(Arc::new(SoundState::new(512)));
        let store = Arc::new(RwLock::new(Store::new(initial_state)));

        let manual_state = store.read().unwrap().get_state().unwrap_manual_state();
        assert_eq!(
            manual_state
                .get_by_address(String::from("phase"))
                .unwrap()
                .unwrap_f32(),
            0.0
        );
        assert_eq!(
            manual_state
                .get_by_address(String::from("pitch"))
                .unwrap()
                .unwrap_i32(),
            60
        );
        assert_eq!(
            manual_state
                .get_by_address(String::from("sound_on"))
                .unwrap()
                .unwrap_bool(),
            true
        );
        assert_eq!(
            manual_state
                .get_by_address(String::from("wave_length"))
                .unwrap()
                .unwrap_usize(),
            512
        );

        let reduce = Box::new(SoundStateReduce {});
        let reducer = Reducer::new(Arc::clone(&store), reduce);
        let manager = SoundStateManager::new(Arc::clone(&store), reducer);

        let reduce = Box::new(SoundStateReduce {});
        let reducer = Reducer::new(Arc::clone(&store), reduce);
        reducer.reduce(SoundStateEvent::ChangePitch(69));
        let manual_state = store.read().unwrap().get_state().unwrap_manual_state();
        assert_eq!(
            manual_state
                .get_by_address(String::from("pitch"))
                .unwrap()
                .unwrap_i32(),
            69
        );

        let wave = manager.get_wave();

        let true_wave = [
            0., 0.06268834, 0.12537667, 0.188065, 0.25075334, 0.3134417, 0.37613, 0.43881837,
            0.5015067, 0.56419504,
        ];

        for i in 0..10 {
            assert_is_close(wave[i], true_wave[i], 0.03);
        }

        let manual_state = store.read().unwrap().get_state().unwrap_manual_state();
        let now_phase = manual_state
            .get_by_address(String::from("phase"))
            .unwrap()
            .unwrap_f32();
        assert_is_close(now_phase, 0.108390026, 0.01);
    }
}
