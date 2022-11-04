<h2 align=center>facegrep</h2>

I want to learn Rust and found about [Rustface](https://github.com/atomashpolskiy/rustface). It detects faces in a picture. But what interesting is it doesn't need external dependencies (eg. opencv). So I think it'll be a good practice.

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
```

## Preview

![image](https://user-images.githubusercontent.com/7823011/199857370-6a684e28-5aed-414f-a461-6b1be6a0925a.png)
