//
// aicam
// AI-based face detection app for video surveillance and notification
//

use chrono::Local;
use libfacedetection::facedetect_cnn;
use log::debug;
use opencv::{prelude::MatTraitConst, prelude::*, videoio::VideoCapture};
use std::{error::Error, process::exit, thread, time::Duration};

use aicam::{upload::upload_image, CliArgs, SETTINGS};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
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
        if let Err(err) = run_app().await {
            println!("{err:?}");
            exit(1);
        }
    }
    Ok(())
}

async fn run_app() -> Result<(), Box<dyn Error>> {
    fn get_path() -> String {
        format!(
            "{}/{}_{}.jpg",
            SETTINGS.general.output_dir,
            env!("CARGO_PKG_NAME"),
            Local::now().format("%Y%m%d%H%M%S"),
        )
    }

    let mut dev = VideoCapture::from_file(&SETTINGS.general.device, opencv::videoio::CAP_ANY)
        .expect("unable to open camera");

    let width = dev
        .get(opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32)
        .expect("unable to get camera width") as i32;
    let height = dev
        .get(opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32)
        .expect("unable to get camera height") as i32;
    debug!(
        "camera {} ({width}x{height}) is running...",
        SETTINGS.general.device
    );

    let post_frames_amount = SETTINGS.general.post_frames_amount;
    let mut post_frames_counter = post_frames_amount;
    let mut is_post_frames_activated = false;
    loop {
        let is_ready = dev.grab().expect("unable to get camera status");
        if !is_ready {
            is_post_frames_activated = false;
            continue;
        }

        let mut img = Mat::default();
        dev.retrieve(&mut img, 0)
            .expect("unable to get frame from camera");

        let mut path = get_path();
        if is_post_frames_activated {
            post_frames_counter -= 1;
            let one_sec = Duration::from_millis(1000);
            thread::sleep(one_sec);

            debug!("saving to {}", path);
            match opencv::imgcodecs::imwrite(&path, &img, &opencv::core::Vector::default()) {
                Ok(rslt) => {
                    if rslt {
                        match upload_image(
                            format!(
                                "{} ({})",
                                &SETTINGS.general.device_name,
                                Local::now().format("%Y-%m-%d %H:%M:%S")
                            ),
                            path,
                        )
                        .await
                        {
                            Ok(_) => {}
                            Err(err) => {
                                debug!("{}", err);
                                return Err(err);
                            }
                        }
                    } else {
                        debug!(
                            "oops, error happened (how about \"{}\" dir?)",
                            SETTINGS.general.output_dir,
                        );
                        return Err(Box::new(std::fmt::Error));
                    }
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
            if post_frames_counter == 0 {
                is_post_frames_activated = false;
                post_frames_counter = post_frames_amount;
            }
        } else {
            let faces = facedetect_cnn(
                img.ptr(0).unwrap(),
                img.cols(),
                img.rows(),
                img.mat_step().get(0) as u32,
            )
            .expect("failed to detect faces");
            for face in faces.faces {
                if face.confidence > SETTINGS.general.confidence {
                    debug!("face: {:?}", face);
                    path = get_path();
                    debug!("saving to {}", path);
                    match opencv::imgcodecs::imwrite(&path, &img, &opencv::core::Vector::default())
                    {
                        Ok(rslt) => {
                            if rslt {
                                is_post_frames_activated = true;
                                match upload_image(
                                    format!(
                                        "{} ({})",
                                        &SETTINGS.general.device_name,
                                        Local::now().format("%Y-%m-%d %H:%M:%S")
                                    ),
                                    path,
                                )
                                .await
                                {
                                    Ok(_) => {}
                                    Err(err) => {
                                        debug!("{}", err);
                                        return Err(err);
                                    }
                                }
                            } else {
                                debug!(
                                    "oops, error happened (how about \"{}\" dir?)",
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
}
