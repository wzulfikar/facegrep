extern crate rustface;

use image::GrayImage;
use rustface::{Detector, FaceInfo, ImageData};
use std::time::{Duration, Instant};

pub fn create_detector(model_path: &str) -> Box<dyn Detector> {
    let mut detector = match rustface::create_detector(model_path) {
        Ok(detector) => detector,
        Err(error) => {
            println!("[ERROR] Failed to create detector: {}", error.to_string());
            std::process::exit(1)
        }
    };

    // Configure params for the detector
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    return detector;
}

pub fn detect_faces(detector: &mut dyn Detector, gray: &GrayImage) -> (Vec<FaceInfo>, String) {
    let (width, height) = gray.dimensions();
    let mut image = ImageData::new(gray, width, height);
    let now = Instant::now();
    let faces = detector.detect(&mut image);

    let msg: String = format!(
        "Found {} faces in {} ms",
        faces.len(),
        get_millis(now.elapsed())
    );

    (faces, msg)
}

fn get_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000u64 + u64::from(duration.subsec_nanos() / 1_000_000)
}
