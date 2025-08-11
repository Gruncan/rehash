use ffmpeg_next::codec::traits::Encoder;
use ffmpeg_next::codec::Context;
use ffmpeg_next::format::context::Output;
use ffmpeg_next::format::Pixel;
use ffmpeg_next::media::Type;
use ffmpeg_next::{codec, format, frame};
use rehash_codec_proc_macro::rehash_codec_ffi;
use std::path::Path;

#[rehash_codec_ffi]
fn print_codec_version() {
    println!("Codec version: {}", env!("CARGO_PKG_VERSION"));
}


#[cfg(target_os = "windows")]
const RED_VIDEO: &'static str = "C:\\Development\\Rust\\rehash\\components\\rehash-codec\\codec\\tests\\resources\\red.mp4";
#[cfg(target_os = "windows")]
const GREEN_OUT_VIDEO: &'static str = "C:\\Development\\Rust\\rehash\\components\\rehash-codec\\codec\\tests\\resources\\green_out_test.mp4";

#[cfg(target_os = "linux")]
const RED_VIDEO: &'static str = "/home/duncan/Development/Rust/rehash/components/rehash-codec/codec/tests/resources/red.mp4";
#[cfg(target_os = "linux")]
const GREEN_OUT_VIDEO: &'static str = "/home/duncan/Development/Rust/rehash/components/rehash-codec/codec/tests/resources/green_out_test.mp4";


fn write_packet_stream(encoder: &mut ffmpeg_next::encoder::video::Video, outx: &mut Output, stream_index: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut encoded = ffmpeg_next::Packet::empty();
    while encoder.receive_packet(&mut encoded).is_ok() {
        encoded.set_stream(stream_index);
        let out = outx.stream(stream_index).ok_or("Failed to get stream index")?;
        encoded.rescale_ts(encoder.time_base(), out.time_base());
        encoded.write_interleaved(outx)?;
    }
    Ok(())
}


fn change_frame_colour(decoder: &mut ffmpeg_next::codec::decoder::video::Video, encoder: &mut ffmpeg_next::encoder::Encoder) -> Result<(), ffmpeg_next::Error> {
    let mut decoded_frame = frame::Video::empty();
    let height = decoder.height();
    let width = decoder.width();

    while decoder.receive_frame(&mut decoded_frame).is_ok() {
        println!("Decoded frame is {:?}", decoded_frame.format());
        if decoded_frame.format() == Pixel::RGB24 {
            let stride = decoded_frame.stride(0);
            let data = decoded_frame.data_mut(0);
            for y in 0..height {
                for x in 0..width {
                    let offset = (y as usize * stride) + (x as usize * 3);
                    data[offset] = 0;
                    data[offset + 1] = 255; // Green channel
                    data[offset + 2] = 0;
                }
            }
            encoder.send_frame(&decoded_frame)?;
        }
    }
    Ok(())
}


fn codec_test() -> Result<(), Box<dyn std::error::Error>> {
    let _ = ffmpeg_next::init()?;

    let mut input = format::input(&Path::new(RED_VIDEO))?;

    let stream = input.streams().best(Type::Video).ok_or("Failed to get stream")?;
    println!("Stream: {:?}", stream);
    let stream_index = stream.index();

    let context_decoder = Context::from_parameters(stream.parameters())?;

    let mut decoder = context_decoder.decoder().video()?;


    let codec = ffmpeg_next::encoder::find(codec::Id::H264).ok_or("Failed to find h264 codec")?;
    let encoder = codec.encoder().ok_or("Failed to get encoder")?;
    let mut enc = ffmpeg_next::codec::context::Context::new_with_codec(encoder).encoder().video()?;
    enc.set_width(decoder.width());
    enc.set_height(decoder.height());
    enc.set_format(Pixel::YUV420P);
    enc.set_time_base(ffmpeg_next::Rational(1, 30));

    let mut encoder = enc.open_as(codec).expect("Failed to create encoder");

    let mut outx = format::output(GREEN_OUT_VIDEO).expect("Failed to create out context");
    let index = {
        let mut out_stream = outx.add_stream(codec).expect("Failed to create output stream");
        out_stream.set_time_base(ffmpeg_next::Rational(1, 30));
        out_stream.set_parameters(&encoder);
        out_stream.index()
    };
    outx.write_header()?;

    for (stream, packet) in input.packets() {
        if stream.index() == stream_index {
            decoder.send_packet(&packet)?;
            change_frame_colour(&mut decoder, &mut encoder).expect("Failed to change frame colour");
            write_packet_stream(&mut encoder, &mut outx, index).expect("Failed to write packet to stream");
        }
    }
    decoder.send_eof()?;
    change_frame_colour(&mut decoder, &mut encoder)?;

    encoder.send_eof()?;
    write_packet_stream(&mut encoder, &mut outx, index)?;
    outx.write_trailer()?;

    Ok(())
}

#[rehash_codec_ffi]
fn run_codec_test() {
    match codec_test() {
        Ok(_) => println!("Successfully completed"),
        Err(e) => println!("Error: {:?}", e),
    }
}