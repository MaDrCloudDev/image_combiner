# rust_image_combiner

Build the program:
```
cargo build --release
```
Run the program with your images:
```
./target/release/image_combiner images/image1.png images/image2.png images/output.png
```
We're making the images the same size, then combining the images using every-other column of pixels from each image.
