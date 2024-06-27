use hound::{WavReader, WavWriter, SampleFormat::Int, WavSpec};
use pitch_shift::PitchShifter;

fn main() {
    let (in_b, sample_rate) = read_wav("in.wav");
    let mut wav = Vec::new();
    let mut shifter = PitchShifter::new(50, sample_rate);
    let mut out_b = vec![0.0; in_b.len()];
    shifter.shift_pitch(8, 2., &in_b, &mut out_b);
    wav.extend_from_slice(&out_b);
    save_wav("out.wav", &wav, sample_rate);
}

fn read_wav(path: &str) -> (Vec<f32>, usize) {
    let mut reader = WavReader::open(path).unwrap();
    let spec = reader.spec();
    assert!(spec.sample_format == Int);
    assert!(spec.bits_per_sample == 16);
    let samples_orig = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32)
        .collect::<Vec<f32>>();
    let mut s = Vec::with_capacity(samples_orig.len() / (spec.channels as usize));
    let mut i = 0;
    for sample in samples_orig {
        if i == 0 {
            s.push(sample);
        }
        i += 1;
        if i == spec.channels {
            i = 0;
        }
    }
    (s, spec.sample_rate as usize)
}

fn save_wav(path: &str, samples: &[f32], sample_rate: usize) {
    let spec = WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: Int,
    };
    let mut writer = WavWriter::create(path, spec).unwrap();
    for s in samples {
        writer.write_sample(*s as i16).unwrap();
    }
}