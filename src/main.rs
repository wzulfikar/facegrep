use image::{DynamicImage, Rgb};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use serde_json::json;

use facegrep::{create_detector, detect_faces};

fn main() {
    // Parse args
    let args: Vec<String> = std::env::args().into_iter().collect();
    let cmd_name: &str = args[0].as_ref();
    if args.len() < 2 || args[1] == "-h" {
        println!("Usage: {} <image-path> [--json]", cmd_name);
        println!("Example: {} ~/Downloads/my-image.jpg", cmd_name);
        std::process::exit(0);
    }

    let image_path = args[1].clone();
    let model_path: Option<String> = match std::env::var("RUSTFACE_MODEL") {
        Ok(v) => Some(v),
        Err(_e) => None,
    };
    if model_path.is_none() {
        println!("[ERROR] Missing environment variable: RUSTFACE_MODEL");
        std::process::exit(1);
    }
    let json_output = args.len() == 3 && args[2] == "--json";

    // Init detector with model
    let mut detector = create_detector(model_path.as_deref().unwrap());

    // Load image
    let image: DynamicImage = match image::open(&image_path) {
        Ok(image) => image,
        Err(message) => {
            println!("Failed to read image: {}", message);
            std::process::exit(1)
        }
    };

    // Detect the faces
    let mut rgb = image.to_rgb8();
    let (faces, detector_msg) = detect_faces(&mut *detector, &image.to_luma8());
    let mut faces_json: Vec<String> = Vec::new();

    // Draw bounding boxes
    for face in faces {
        let bbox = face.bbox();
        let rect = Rect::at(bbox.x(), bbox.y()).of_size(bbox.width(), bbox.height());
        draw_hollow_rect_mut(&mut rgb, rect, Rgb([255, 0, 0]));

        let face_json = json!({
            "score": face.score(),
            "bbox": json!({"x": bbox.x(), "y": bbox.y() })
        });
        faces_json.push(face_json.to_string());
    }

    // Save output file
    let output_file = format!("{}-rustface.png", &image_path);
    let result = match rgb.save(output_file.clone()) {
        Ok(_) => output_file,
        Err(message) => message.to_string(),
    };

    if json_output {
        println!("[{}]", faces_json.join(","));
    } else {
        println!("{}", detector_msg);
        println!("{}", result)
    }
}
