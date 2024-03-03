```
           _|                                      
   _|_|_|        _|_|_|    _|_|_|  _|_|_|  _|_|    
 _|    _|  _|  _|        _|    _|  _|    _|    _|  
 _|    _|  _|  _|        _|    _|  _|    _|    _|  
   _|_|_|  _|    _|_|_|    _|_|_|  _|    _|    _|
```

## aicam

### `AI`-based face detection app for video surveillance and notification

Based on `opencv` and `libfacedetection` libs

<hr/>

- [aicam](#aicam)
  - [`AI`-based face detection app for video surveillance and notification](#ai-based-face-detection-app-for-video-surveillance-and-notification)
  - [1. General](#1-general)
  - [2. OpenCV lib](#2-opencv-lib)
    - [2.1 Getting sources](#21-getting-sources)
    - [2.2. Building @ Linux (x86\_64)](#22-building--linux-x86_64)
    - [2.3. Cross-building @ Odroid U3 (arm-linux-gnueabihf)](#23-cross-building--odroid-u3-arm-linux-gnueabihf)
  - [3. aicam](#3-aicam)
    - [3.1. Building and running @ Linux (x86\_64)](#31-building-and-running--linux-x86_64)
    - [3.2. Cross-building @ Odroid U3 (arm-linux-gnueabihf)](#32-cross-building--odroid-u3-arm-linux-gnueabihf)
    - [3.3. Deployment, configuring and running @ Odroid U3 (arm-linux-gnueabihf)](#33-deployment-configuring-and-running--odroid-u3-arm-linux-gnueabihf)


### 1. General

This application performs face recognition (usually in motion detection) followed by notification by uploading captured images to your `Telegram` group.

**IMPORTANT:** please do not forget to configure both `token` and `channel` params under `telegram` section of your `config/aicam.toml` config file.

### 2. OpenCV lib

#### 2.1 Getting sources
```
➜  ✗ git clone --recursive https://github.com/opencv/opencv.git && cd ./opencv
```


#### 2.2. Building @ Linux (x86_64)
```
➜  opencv git:(4.x) mkdir -p build/x86_64 && cd build/x86_64
➜  x86_64 git:(4.x) cmake -DCMAKE_INSTALL_PREFIX=../../dist/x86_64 -DCMAKE_TOOLCHAIN_FILE=../../platforms/linux/gnu.toolchain.cmake ../..
➜  x86_64 git:(4.x) make -j $(nproc)
➜  x86_64 git:(4.x) make install
```


#### 2.3. Cross-building @ Odroid U3 (arm-linux-gnueabihf)

TBD


### 3. aicam

#### 3.1. Building and running @ Linux (x86_64)
```
➜  ✗ git clone https://github.com/vitali2y/aicam_libfacedetection.git && cd aicam_libfacedetection
➜  aicam_libfacedetection git:(master) ✗ git clone --recursive https://github.com/vitali2y/libfacedetection-rs.git
Cloning into 'libfacedetection-rs'...
~...~
➜  aicam_libfacedetection git:(master) ✗ export LD_LIBRARY_PATH=./dist/x86_64:$LD_LIBRARY_PATH
➜  aicam_libfacedetection git:(master) ✗ RUST_LOG=debug mold -run cargo r  # or, just: RUST_LOG=debug cargo run
    Updating crates.io index
~...~
    Finished dev [unoptimized + debuginfo] target(s) in 25.70s
     Running `target/debug/aicam_libfacedetection`
aicam 0.1.0
AI face detection & notification app (opencv + libfacedetection)
[2024-03-03T13:50:53Z DEBUG aicam] camera /dev/video0 (640x480) is running...
[2024-03-03T13:50:56Z DEBUG aicam] face: Face { confidence: 94, x: 220, y: 240, width: 262, height: 223, landmarks: [(271, 398), (378, 410), (296, 480), (269, 524), (360, 534)] }
[2024-03-03T13:50:56Z DEBUG aicam] saving to ./output/aicam_20240303155056.jpg
[2024-03-03T13:50:56Z DEBUG aicam::upload] uploading ./output/aicam_20240303155056.jpg
[2024-03-03T13:50:56Z DEBUG reqwest::connect] starting new connection: https://api.telegram.org/
[2024-03-03T13:50:57Z DEBUG aicam] saving to ./output/aicam_20240303155056.jpg
[2024-03-03T13:50:57Z DEBUG aicam::upload] uploading ./output/aicam_20240303155056.jpg
[2024-03-03T13:50:57Z DEBUG reqwest::connect] starting new connection: https://api.telegram.org/
[2024-03-03T13:50:58Z DEBUG aicam] saving to ./output/aicam_20240303155057.jpg
[2024-03-03T13:50:58Z DEBUG aicam::upload] uploading ./output/aicam_20240303155057.jpg
[2024-03-03T13:50:58Z DEBUG reqwest::connect] starting new connection: https://api.telegram.org/
[2024-03-03T13:51:00Z DEBUG aicam] saving to ./output/aicam_20240303155059.jpg
[2024-03-03T13:51:00Z DEBUG aicam::upload] uploading ./output/aicam_20240303155059.jpg
[2024-03-03T13:51:00Z DEBUG reqwest::connect] starting new connection: https://api.telegram.org/
~...~
```


#### 3.2. Cross-building @ Odroid U3 (arm-linux-gnueabihf)

TBD


#### 3.3. Deployment, configuring and running @ Odroid U3 (arm-linux-gnueabihf)

TBD
