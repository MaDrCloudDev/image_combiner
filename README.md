# rust_image_combiner

**_NOTE: Images must be sufficiently small, or they'll exceed the buffer size._**

Build the program:

```
cargo build --release
```

Run the program with your images:

```
./target/release/image_combiner images/image1.png images/image2.png images/output.png
```

We're making the images the same size, then combining the images using every-other(ish) column of pixels from each image.

Not really intended to be a useful program (unless you like the every-other-column-of-pixels effect). I wanted to build the same CLI program in Node, GO, and Rust.
