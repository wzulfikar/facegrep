# facegrep

TODO:

- [x] default output: path to image file with bounding boxes
- [ ] json output: json of bounding boxes

# Usage

```sh
# Download model from Rustface repo
curl -L -O --output-dir /tmp https://github.com/atomashpolskiy/rustface/raw/master/model/seeta_fd_frontal_v1.0.bin

# Expose the model as environment variable
export RUSTFACE_MODEL=/tmp/seeta_fd_frontal_v1.0.bin

# Run the binary with the test image
cargo run assets/test/scientist.jpg
```
