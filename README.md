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

  - [1. General](#1-general)
  - [2. OpenCV lib](#2-opencv-lib)
    - [2.1 Getting sources](#21-getting-sources)
    - [2.2. Building @ Linux (x86\_64)](#22-building--linux-x86_64)
    - [2.3. Cross-building @ Odroid U3 (arm-linux-gnueabihf)](#23-cross-building--odroid-u3-arm-linux-gnueabihf)
  - [3. aicam](#3-aicam)
    - [3.1. Bilding and running @ Linux (x86\_64)](#31-bilding-and-running--linux-x86_64)
    - [3.2. Cross-building @ Odroid U3 (arm-linux-gnueabihf)](#32-cross-building--odroid-u3-arm-linux-gnueabihf)
    - [3.3. Deployment, configuring and running @ Odroid U3 (arm-linux-gnueabihf)](#33-deployment-configuring-and-running--odroid-u3-arm-linux-gnueabihf)

### 1. General

TBD

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
#### 3.1. Bilding and running @ Linux (x86_64)
```
➜  aicam_libfacedetection git:(master) ✗ git clone --recursive https://github.com/vitali2y/libfacedetection-rs.git
Cloning into 'libfacedetection-rs'...
~...~
➜  aicam_libfacedetection git:(master) ✗ export LD_LIBRARY_PATH=./dist/x86_64:$LD_LIBRARY_PATH
➜  aicam_libfacedetection git:(master) ✗ mold -run cargo r  # or, just: cargo run
    Updating crates.io index
~...~
    Finished dev [unoptimized + debuginfo] target(s) in 25.70s
     Running `target/debug/aicam_libfacedetection`
Available camera:
    Width: 640
    Height: 480
Face: Face { confidence: 63, x: 390, y: 4, width: 231, height: 193, landmarks: [(472, 45), (555, 20), (534, 79), (511, 132), (579, 110)] }
Saving to ./aicam_20240225094529.jpg
~...~
```

#### 3.2. Cross-building @ Odroid U3 (arm-linux-gnueabihf)

TBD

#### 3.3. Deployment, configuring and running @ Odroid U3 (arm-linux-gnueabihf)

TBD
