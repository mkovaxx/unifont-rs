# unifont-rs

[Unifont](https://en.wikipedia.org/wiki/GNU_Unifont) for Rust

Provides a monochrome bitmap font that covers the entire Unicode Basic Multilingual Plane.
Halfwidth glyphs are 8x16, fullwidth are 16x16 pixels.

## Goals

- easy to use
- small memory footprint

## How it Works

The `build.rs` script parses the `data/unifont-*.hex` file and emits Rust code.
The generated data relies on a `Glyph` type that gives easy access to pixels. 

```
impl Glyph {
    pub fn get_pixel(&self, x: usize, y: usize) -> bool,
    pub fn get_width(&self) -> usize,
}
```
