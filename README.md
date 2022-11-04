<h1 align=center>facegrep</h2>

I want to learn Rust and found about [Rustface](https://github.com/atomashpolskiy/rustface) (which is a port of [C++ SeetaFace Engine](https://github.com/seetaface/SeetaFaceEngine)). It detects faces in a picture. What interesting is it doesn't need external dependencies (eg. opencv).

I think it'll be a good exercise.

## Goal
- [x] Use the [example code](https://github.com/atomashpolskiy/rustface/blob/master/examples/image_demo.rs) from rustface as base
- [x] Get model path from environment variable so users don't have to put it as argument
- [x] Show output file (image file with bounding boxes)
- [x] Add json support

## Usage

```sh
# Clone repo
git clone --depth 1 https://github.com/wzulfikar/facegrep

# Download model from Rustface repo
curl -L -O --output-dir /tmp https://github.com/atomashpolskiy/rustface/raw/master/model/seeta_fd_frontal_v1.0.bin

# Expose the model as environment variable
export RUSTFACE_MODEL=/tmp/seeta_fd_frontal_v1.0.bin

# Run the binary with the test image
cargo run assets/test/scientists.jpg

# Use --json flag to show json output
cargo run assets/test/scientists.jpg --json
```

If you don't want to install Rust cargo:

- for M1, try use the binary at `bin/facegrep-m1`
- for Windows (Rust target x86_64-pc-windows-gnu), try use `bin/facegrep-win`

You'll still need to download the model and expose its path as environment variable.

## Preview

<p align=center><img width=900 src=https://user-images.githubusercontent.com/7823011/199857370-6a684e28-5aed-414f-a461-6b1be6a0925a.png /></p>

## Notes
- JSON should be properly handled instead of just string-concatenated ([main.rs#L57](https://github.com/wzulfikar/facegrep/blob/5d02d6c9a9eac9c78327e71e76fd9f116ccaf461/src/main.rs#L57), [L69](https://github.com/wzulfikar/facegrep/blob/5d02d6c9a9eac9c78327e71e76fd9f116ccaf461/src/main.rs#L69))
- If we want to reuse the code in web, how difficult is it to port to wasm? Would there be performance penalty?
