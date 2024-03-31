# unifont-rs

[Unifont](https://en.wikipedia.org/wiki/GNU_Unifont) for Rust

Provides a monochrome bitmap font that covers the entire Unicode Basic Multilingual Plane. Halfwidth glyphs are 8x16, fullwidth are 16x16 pixels.

## Features

- easy to use
- access to raw binary data
- `#[no_std]` for embedded use
- small memory footprint

## API

```rust
pub fn get_glyph(c: char) -> Option<&'static Glyph> { /* ... */ }
pub fn enumerate_glyphs() -> impl Iterator<Item = (char, &'static Glyph)> { /* ... */ }

pub enum Glyph {
    HalfWidth([u8; 16]),
    FullWidth([u16; 16]),
}

impl Glyph {
    pub fn get_pixel(&self, x: usize, y: usize) -> bool,
    pub fn get_width(&self) -> usize,
}
```

## Example Code

Example code is under `examples/banner.rs`.

Run the binary with the following command:
```sh
cargo run --example banner UniFont
```

It will produce the following output:
```
                                                        
                                                        
                                                        
                    #                                   
 #    #             #    ######                    #    
 #    #                  #                         #    
 #    #  # ###     ##    #        ####   # ###     #    
 #    #  ##   #     #    #       #    #  ##   #  #####  
 #    #  #    #     #    #####   #    #  #    #    #    
 #    #  #    #     #    #       #    #  #    #    #    
 #    #  #    #     #    #       #    #  #    #    #    
 #    #  #    #     #    #       #    #  #    #    #    
 #    #  #    #     #    #       #    #  #    #    #    
  ####   #    #   #####  #        ####   #    #     ##  
                                                        
                                                        
```

## How it Works

At compile time, the `build.rs` script parses the `data/unifont-*.hex` file and emits Rust code.
