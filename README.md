# unifont-rs

[Unifont](https://en.wikipedia.org/wiki/GNU_Unifont) for Rust

Provides a monochrome bitmap font that covers the entire Unicode Basic Multilingual Plane. Halfwidth glyphs are 8x16, fullwidth are 16x16 pixels.

## Features

- easy to use
- access to raw binary data
- `#[no_std]` for embedded use
- small memory footprint
- i18n support

## API

```rust
fn get_glyph(c: char) -> Option<&'static Glyph>;
fn enumerate_glyphs() -> impl Iterator<Item = (char, &'static Glyph)>;

enum Glyph {
    Halfwidth([u8; 16]),
    Fullwidth([u16; 16]),
}

impl Glyph {
    fn get_pixel(&self, x: usize, y: usize) -> bool;
    fn get_width(&self) -> usize;
    fn is_fullwidth(&self) -> bool;
}

/// Preprocess a sequence of characters so that it may be rendered via Unifont.
/// Currently supported scripts: Arabic.
/// Works in place to avoid the need for allocation.
fn preprocess_text(chars: &mut [char]);
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

### i18n (Internationalization)

Basic preprocessing is provided for displaying text in non-Latin scripts, such as Arabic.

Try running the example as follows:
```sh
cargo run --example banner "أبجد"
```

It will produce the following output:
```
                           ##   
                          #     
                           ##   
                          #     
                            #   
                            #   
    #                       #   
     #              #       #   
      #   ###        #      #   
 #    #      ##      #      #   
 #####################      #   
                                
           #                    
                   #            
                                
                                
```

## How it Works

At compile time, the `build.rs` script parses the `data/unifont-*.hex` file and emits Rust code.
