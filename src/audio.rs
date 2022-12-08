use std::f32::consts::PI;

use cpal::Stream;
use cpal::traits::*;
use ringbuf::*;
use crate::kmath::*;
use crate::sound_instance::*;

pub const SOUND_PLAY: u32 = 1 << (31);
pub const SOUND_UNIQUE: u32 = 1 << (30);

fn adshr(t: u64, a: u64, d: u64, s: f32, h: u64, r: u64) -> f32 {
    if t < a {
        return t as f32 / a as f32;
    }
    let t = t - a;
    if t < d {
        return 1.0 - (1.0 - s) * t as f32 / d as f32;
    }
    let t = t - d;
    if t < h {
        return s;
    }
    let t = t - h;
    
    if t > r {
        return 0.0;
    }
    let t = t - r;
    return s - s * t as f32 / r as f32;
}

#[derive(Default)]
pub struct AudioState {
    sound_manager: SoundManager,
}

impl AudioState {
    pub fn handle_event(&mut self, e: u32) {
        let play = e & SOUND_PLAY != 0;
        let unique = e & SOUND_UNIQUE != 0;
        let sound = e & (SOUND_UNIQUE - 1);

        if play {
            if unique {
                if !self.sound_manager.sounds.iter().any(|x| x.id == sound) {
                    self.sound_manager.play_sound(sound);
                }
            } else {
                self.sound_manager.play_sound(sound);
            }
        } else {
            self.sound_manager.sounds.retain(|x| x.id != sound);
        }
    }

    // pub fn tick(&mut self) -> f32 {
    //     // (440.0 * self.t as f32 / 44100.0).sin()
    //     const denom: f32 = 44100.0 / (2.0 * PI);
        
        
    //     let duration = [44100, 10000, 44100*3, 20000, 10000, 20000];
    //     let mut scurr = [false; NUM_SOUNDS];
    //     for i in 0..self.sounds.len() {
    //         // let scurr = self.sounds[i].t

    //         scurr[i] = self.sounds[i] != SOUND_NONE && self.t - self.sound_start[i] < duration[i]
    //     }
    //     let mut sd = [0; NUM_SOUNDS];
    //     for i in 0..NUM_SOUNDS {
    //         if scurr[i] {
    //             sd[i] = self.t - self.sound_start[i]
    //         }
    //     }
    //     let mut st = [0.0; NUM_SOUNDS];
    //     for i in 0..NUM_SOUNDS {
    //         if scurr[i] {
    //             st[i] = sd[i] as f32 / duration[i] as f32;
    //         }
    //     }

    //     let amp = [0.2, 0.4, 0.2, 0.2, 0.1, 0.3];

    //     let samp = [
    //         (220.0 * self.t as f32 / denom).sin(),  // laser

    //         // 0.1 * (440.0 * self.t as f32 / denom).sin(), // enemy shoot
    //         adshr(sd[1], 1000, 1000, 0.3, 5000, 5000) * (440.0 * self.t as f32 / denom).sin(), // enemy shoot

    //         (880.0 * self.t as f32 / denom).sin(),
    //         krand(self.t as u32) as f32,
    //     ];
            
    //     let mut acc = 0.0;
            
    //     for i in 0..NUM_SOUNDS {
    //         if self.sound_start[i] != SOUND_NONE && self.t - self.sound_start[i] < duration[i] {
    //             acc += samp[i] * amp[i];
    //         }
    //     }
        
    //     self.t += 1;
    //     acc
    // }

    pub fn tick(&mut self) -> f32 {
        self.sound_manager.tick()

        // let mut acc = 0.0;
        // for sound in self.sounds.iter() {
        //     acc +=
        // }  
        // self.sounds.retain(|s| s.age(self.t) < s.duration());
        // self.t += 1;
        // acc

    }
}


pub fn sample_next(o: &mut SampleRequestOptions) -> f32 {
    o.audio_state.tick()
}

pub struct SampleRequestOptions {
    pub sample_rate: f32,
    pub nchannels: usize,

    pub audio_state: AudioState,
    pub channel: Consumer<u32>,
}

pub fn stream_setup_for<F>(on_sample: F, channel: Consumer<u32>) -> Result<cpal::Stream, anyhow::Error>
where
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static + Copy,
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => stream_make::<f32, _>(&device, &config.into(), on_sample, channel),
        cpal::SampleFormat::I16 => stream_make::<i16, _>(&device, &config.into(), on_sample, channel),
        cpal::SampleFormat::U16 => stream_make::<u16, _>(&device, &config.into(), on_sample, channel),
    }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {:?}", config);

    Ok((host, device, config))
}

pub fn stream_make<T, F>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    on_sample: F,
    channel: Consumer<u32>,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: cpal::Sample,
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static + Copy,
{
    let sample_rate = config.sample_rate.0 as f32;
    let nchannels = config.channels as usize;
    let mut request = SampleRequestOptions {
        sample_rate,
        nchannels,
        audio_state: AudioState::default(),
        channel,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            on_window(output, &mut request, on_sample)
        },
        err_fn,
    )?;

    Ok(stream)
}

fn on_window<T, F>(output: &mut [T], request: &mut SampleRequestOptions, mut on_sample: F)
where
    T: cpal::Sample,
    F: FnMut(&mut SampleRequestOptions) -> f32 + std::marker::Send + 'static,
{
    if let Some(sc) = request.channel.pop() {
        request.audio_state.handle_event(sc);
    }
    for frame in output.chunks_mut(request.nchannels) {
        let value: T = cpal::Sample::from::<f32>(&on_sample(request));
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}