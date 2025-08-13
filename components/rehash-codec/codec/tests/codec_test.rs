use rehashcodec::codec::Mp4Fragmenter;


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn sample_mp4_path() -> String {
        let path = "tests/resources/red.mp4";
        assert!(Path::new(path).exists(), "Sample MP4 file missing: {path}");
        path.to_string()
    }

    #[test]
    fn test_fragmenter_creates_init_and_fragment() {
        let path = sample_mp4_path();

        let mut frag = Mp4Fragmenter::new(&path, 2.0)
            .expect("Failed to create Mp4Fragmenter");

        let init_seg = frag.create_init_segment()
            .expect("Failed to create init segment");
        assert!(!init_seg.is_empty(), "Init segment is empty");
        assert!(
            init_seg.windows(4).any(|w| w == b"ftyp"),
            "Init segment missing 'ftyp' box"
        );
        
        let frag_seg = frag.create_fragment(0.0)
            .expect("Failed to create fragment");
        assert!(!frag_seg.is_empty(), "Fragment is empty");
        assert!(
            frag_seg.windows(4).any(|w| w == b"moof"),
            "Fragment missing 'moof' box"
        );
    }
}