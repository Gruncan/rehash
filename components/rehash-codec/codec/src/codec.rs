use ffmpeg::{
    encoder,
    format,
    media,
    util::rational::Rational,
};
use ffmpeg_next as ffmpeg;
use std::{fs, path::PathBuf, sync::Once};

static INIT: Once = Once::new();

fn ensure_ffmpeg_init() {
    INIT.call_once(|| {
        ffmpeg::init().expect("failed to init ffmpeg");
    });
}

pub struct Mp4Fragmenter {
    input_path: String,
    fragment_duration: f64, // seconds
    video_time_base: Rational,
}

impl Mp4Fragmenter {
    pub fn new(path: &str, fragment_duration_secs: f64) -> Result<Self, Box<dyn std::error::Error>> {
        ensure_ffmpeg_init();

        let ictx = format::input(path)?;
        let vstream = ictx
            .streams()
            .best(media::Type::Video)
            .ok_or("no video stream found")?;
        let video_time_base = vstream.time_base();

        Ok(Self {
            input_path: path.to_string(),
            fragment_duration: fragment_duration_secs,
            video_time_base,
        })
    }

    pub fn create_init_segment(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ictx = format::input(&self.input_path)?;
        let vstream = ictx
            .streams()
            .best(media::Type::Video)
            .ok_or("no video stream found")?;

        let tmp = tempfile::Builder::new().prefix("init_").suffix(".mp4").tempfile()?;
        let path: PathBuf = tmp.path().to_path_buf();

        let mut octx = format::output(&path)?;

        {
            let mut ost = octx.add_stream(encoder::find(ffmpeg::codec::Id::None))?;
            ost.set_parameters(vstream.parameters());
            unsafe {
                (*ost.parameters().as_mut_ptr()).codec_tag = 0;
            }
        }

        let mut opts = ffmpeg::Dictionary::new();
        opts.set(
            "movflags",
            "empty_moov+separate_moof+default_base_moof+omit_tfhd_offset",
        );

        octx.write_header_with(opts)?;
        drop(octx);

        let bytes = fs::read(&path)?;
        drop(tmp);
        Ok(bytes)
    }

    /// fMP4 fragment (moof+mdat) for [start_time, start_time+fragment_duration).
    pub fn create_fragment(&self, start_time: f64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut ictx = format::input(&self.input_path)?;
        let vstream = ictx
            .streams()
            .best(media::Type::Video)
            .ok_or("no video stream found")?;
        let video_stream_index = vstream.index();
        let in_tb = vstream.time_base();

        // Prepare output
        let tmp = tempfile::Builder::new().prefix("frag_").suffix(".mp4").tempfile()?;
        let path: PathBuf = tmp.path().to_path_buf();
        let mut octx = format::output(&path)?;

        {
            let mut ost = octx.add_stream(encoder::find(ffmpeg::codec::Id::None))?;
            ost.set_parameters(vstream.parameters());
            unsafe {
                (*ost.parameters().as_mut_ptr()).codec_tag = 0;
            }
        }

        let mut opts = ffmpeg::Dictionary::new();
        opts.set(
            "movflags",
            "frag_keyframe+separate_moof+default_base_moof+omit_tfhd_offset",
        );
        octx.write_header_with(opts)?;

        let out_tb = octx.stream(0).unwrap().time_base();

        let pos_us = (start_time * ffmpeg::ffi::AV_TIME_BASE as f64).round() as i64;
        let _ = ictx.seek(pos_us, ..pos_us);

        let start_pts = (start_time * in_tb.1 as f64 / in_tb.0 as f64).round() as i64;
        let end_pts = ((start_time + self.fragment_duration) * in_tb.1 as f64 / in_tb.0 as f64).round() as i64;

        let mut wrote_any = false;
        for (stream, mut packet) in ictx.packets() {
            if stream.index() != video_stream_index {
                continue;
            }
            let Some(pkt_pts) = packet.pts() else { continue };
            if pkt_pts < start_pts {
                continue;
            }
            if pkt_pts >= end_pts {
                break;
            }

            packet.rescale_ts(in_tb, out_tb);
            packet.set_position(-1);
            packet.set_stream(0);
            packet.write_interleaved(&mut octx)?;
            wrote_any = true;
        }

        if !wrote_any {
            drop(octx);
            drop(tmp);
            return Err("fragment window contained no packets (try a different start_time or larger fragment_duration)".into());
        }

        octx.write_trailer()?;
        drop(octx);

        let bytes = fs::read(&path)?;
        drop(tmp);
        Ok(bytes)
    }
}
