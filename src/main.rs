//
// aicam
// AI-based face detection app for video surveillance and notification
//

use chrono::Local;
use libfacedetection::facedetect_cnn;
use opencv::{prelude::MatTraitConst, prelude::*, videoio::VideoCapture};
use std::{error::Error, process::exit};

use aicam::{CliArgs, SETTINGS};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "{} {}\n{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    let args: CliArgs = argh::from_env();
    if args.version {
        println!(env!("CARGO_PKG_VERSION"));
    } else {
        if let Err(err) = run_app() {
            println!("{err:?}");
            exit(1);
        }
    }
    Ok(())
}

fn run_app() -> Result<(), Box<dyn Error>> {
    let mut dev = VideoCapture::from_file(&SETTINGS.general.device, opencv::videoio::CAP_ANY)
        .expect("Unable to open camera");

    let width = dev
        .get(opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32)
        .expect("Unable to get camera width") as i32;
    let height = dev
        .get(opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32)
        .expect("Unable to get camera height") as i32;
    println!(
        "Camera {} ({width}x{height}) is running...",
        SETTINGS.general.device
    );

    loop {
        let is_ready = dev.grab().expect("Unable to get camera status");
        if !is_ready {
            continue;
        }

        let mut img = Mat::default();
        dev.retrieve(&mut img, 0)
            .expect("Unable to get frame from camera");

        let faces = facedetect_cnn(
            img.ptr(0).unwrap(),
            img.cols(),
            img.rows(),
            img.mat_step().get(0) as u32,
        )
        .expect("Failed to detect faces");
        for face in faces.faces {
            if face.confidence > SETTINGS.general.confidence {
                if SETTINGS.general.debug {
                    println!("Face: {:?}", face);
                }
                let ts = Local::now().format("%Y%m%d%H%M%S");
                let path = format!(
                    "{}/{}_{}.jpg",
                    SETTINGS.general.output_dir,
                    env!("CARGO_PKG_NAME"),
                    ts,
                );
                if SETTINGS.general.debug {
                    println!("Saving to {}", path);
                };
                match opencv::imgcodecs::imwrite(&path, &img, &opencv::core::Vector::default()) {
                    Ok(rslt) => {
                        if !rslt {
                            println!(
                                "Oops, error happened (how about \"{}\" dir?)",
                                SETTINGS.general.output_dir,
                            );
                            return Err(Box::new(std::fmt::Error));
                        }
                    }
                    Err(err) => {
                        return Err(Box::new(err));
                    }
                }
            }
        }
    }
}
