use crate::kmath::*;

// todo: why window isnt working + how to do it better

pub const LASER: u32 = 0;
pub const ENEMY_SHOOT: u32 = 1;
pub const PLAYER_DIE: u32 = 2;
pub const ENEMY_DIE: u32 = 3;
pub const ENEMY_SPAWN: u32 = 4;
pub const LASER_POP: u32 = 5;

const PHASE_UNIT: f32 = 2.0 * PI / 44100.0; // phase unit
                                //   laser      eshoot  pdie    edie    espawn  lpop
const duration: [u64; 9] =          [u64::MAX,  2500,   15000,  10000,  2500,  5000,  100, 100, 100];
const freq_base: [f32; 9] =         [110.0,     150.0,  110.0,  880.0,  200.0,  666.0,  100.0, 100.0, 100.0];
const freq_mult_range: [f32; 9] =   [0.0,       2.0,    0.0,    0.0,    0.0,    0.0,    0.0, 0.0, 0.0];
const amp: [f32; 9] =               [0.1,       0.05,    0.6,    0.03,   0.0,    0.1,    0.1, 0.1, 0.1, ];
const amp_start: [f32; 9] =         [1.0,       1.0,    1.0,    1.0,    0.0,    1.0,    1.0, 1.0, 1.0, ];
const amp_end: [f32; 9] =           [1.0,       0.8,    0.0,    0.0,    1.0,    0.0,    1.0, 1.0, 1.0, ];

pub struct SoundInstance {
    pub birth: u64,
    pub age: u64,
    pub id: u32,
    pub seed: u32,
}

impl SoundInstance {
    pub fn duration(&self) -> u64 {
        duration[self.id as usize]
    }

    pub fn freq(&self) -> f32 {
        freq_base[self.id as usize] * (1.0 + krand(self.seed) * freq_mult_range[self.id as usize])
    }

    pub fn tick(&mut self) -> f32 {
        self.age += 1;
        let a = amp[self.id as usize] * (self.age as f32 * PHASE_UNIT * self.freq()).sin() * self.sin_window(100);
        let t = self.age as f32 / self.duration() as f32;
        lerp(amp_start[self.id as usize], amp_end[self.id as usize], t) * a
    }
    pub fn t_window(&self, transition_length: u64) -> f32 {
        if self.age < transition_length {
            self.age as f32 / transition_length as f32
        } else if self.age > (self.duration() - transition_length) {
            1.0 - (self.age - (self.duration() - transition_length)) as f32 / transition_length as f32
        } else {
            1.0
        }
    }
    pub fn sin_window(&self, transition_length: u64) -> f32 {
        let t = self.t_window(transition_length);
        (t * PI / 2.0).sin()
    }
}

pub struct SoundManager {
    pub age: u64,
    pub seed: u32,
    pub sounds: Vec<SoundInstance>,
}
impl Default for SoundManager {
    fn default() -> Self {
        SoundManager { age: 0, sounds: Vec::new(), seed: 134157}
    }
}
impl SoundManager {
    pub fn tick(&mut self) -> f32 {
        self.age += 1;
        if self.age % 44100 == 0 {
            self.sitrep()
        }
        let mut acc = 0.0;
        for sound in self.sounds.iter_mut() {
            acc += sound.tick();
        }
        self.sounds.retain(|s| s.age < s.duration());
        acc

        // (self.age as f32 * PHASE_UNIT * 440.0).sin()
    }

    pub fn play_sound(&mut self, id: u32) {
        self.seed = khash(self.seed.wrapping_mul(1241417).wrapping_add(124114));
        self.sounds.push(SoundInstance {
            age: 0,
            birth: self.age,
            id,
            seed: self.seed,
        });
    }

    pub fn sitrep(&self) {
        println!("{}: n sounds {}", self.age / 44100, self.sounds.len())
    }
}