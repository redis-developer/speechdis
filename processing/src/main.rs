// use std::io::BufReader;

// fn main() {
//     let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
//     let sink = rodio::Sink::try_new(&handle).unwrap();

//     let file = std::fs::File::open(
//         "/Users/thomas/src/redisai-hackathon/speechdis/processing/src/sample1.flac",
//     )
//     .unwrap();
//     sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

//     sink.sleep_until_end();
// }
// An example of using `sample` to efficiently perform decent quality sample rate conversion on a
// WAV file entirely on the stack.
use audrey::{dasp_sample::conv::f32, read::Reader};
use dasp::interpolate::linear;
use dasp::interpolate::sinc::Sinc;
use dasp::{ring_buffer, signal, Sample, Signal};
// use dasp_interpolate::linear::Linear;
use dasp_signal::{from_iter, interpolate::Converter};

use std::fs::File;
use std::path::Path;
use std::time::Instant;

// The model has been trained on this specific
// sample rate.
const SAMPLE_RATE: u32 = 16_000;

fn main() {
    // Find and load the wav.
    // let audio_file =
    //     File::open("/Users/thomas/src/redisai-hackathon/speechdis/processing/src/sample1.flac")
    //         .unwrap();
    // let mut reader = Reader::new(audio_file).unwrap();
    // let desc = reader.description();

    let audio_file =
        File::open("/Users/thomas/src/redisai-hackathon/speechdis/mlexp/test_input.flac").unwrap();
    let mut reader = Reader::new(audio_file).unwrap();
    let desc = reader.description();
    dbg!(&desc);
    assert_eq!(
        1,
        desc.channel_count(),
        "The channel count is required to be one, at least for now"
    );
    let initialized_time = Instant::now();

    let audio_buf: Vec<f64> = if desc.sample_rate() == SAMPLE_RATE {
        let new_signal = reader
            .samples()
            .filter_map(Result::ok)
            .map(i16::to_sample::<f64>)
            .collect();
        new_signal
    } else {
        let samples = reader
            .samples()
            .filter_map(Result::ok)
            .map(i16::to_sample::<f64>);
        let signal = signal::from_interleaved_samples_iter(samples);
        // Convert the signal's sample rate using `Sinc` interpolation.
        let ring_buffer = ring_buffer::Fixed::from([[0.0]; 100]);
        let sinc = Sinc::new(ring_buffer);
        let new_signal = signal.from_hz_to_hz(sinc, desc.sample_rate() as f64, SAMPLE_RATE as f64);
        new_signal.until_exhausted().map(|v| v[0]).collect()
    };
    dbg!(&audio_buf[..20]);
    // Obtain the buffer of samples
    // let audio_buf: Vec<_> = if desc.sample_rate() == SAMPLE_RATE {
    //     reader.samples().map(|s| s.unwrap()).collect()
    // } else {
    // We need to interpolate to the target sample rate
    // let interpolator = linear::Linear::new([0i16], [0]);
    // let conv = Converter::from_hz_to_hz(
    //     from_iter(reader.samples::<i16>().map(|s| [s.unwrap()])),
    //     interpolator,
    //     desc.sample_rate() as f64,
    //     SAMPLE_RATE as f64,
    // );
    // conv.until_exhausted().map(|v| v[0]).collect()
    // };

    let len_seconds = audio_buf.len() as f64 / SAMPLE_RATE as f64;

    let decoded_time = Instant::now();

    println!(
        "Decoding done in {:?}. Sample length {}s. Running STT.",
        decoded_time - initialized_time,
        len_seconds
    );

    // // Convert the signal's sample rate using `Sinc` interpolation.
    // let ring_buffer = ring_buffer::Fixed::from([[0.0]; 100]);
    // let sinc = Sinc::new(ring_buffer);
    // let new_signal = signal.from_hz_to_hz(sinc, spec.sample_rate as f64, target.sample_rate as f64);

    // // Write the result to a new file.
    // let mut writer = WavWriter::create(assets.join("two_vowels_10k.wav"), target).unwrap();
    // for frame in new_signal.until_exhausted() {
    //     writer.write_sample(frame[0].to_sample::<i16>()).unwrap();
    // }
}
