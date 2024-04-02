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
```

## Example Code
### Basic usage
Example code is under `examples/basic.rs`.

Run the binary with the following command:
```
cargo run --example basic UniFont
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

### Usage with Arabic
_NOTE: This library handles Arabic scripts, but you need to use the Arabic processors as shown in this example._

Example code is under `examples/arabic.rs`.
Run the binary with the following command:
```
cargo run --example arabic 'أبجد ABC ١٢٣ لا 123'
```

It will produce the following output:
```
                                                                                                                                           ##   
                                                                                                                                          #     
                                                                                                                                           ##   
                                      #           #       #   #  #  #  #                                                                  #     
    #     ####    ####                #           #       #  #   # #  #            ##    #####    ####                                      #   
   ##    #    #  #    #           #   #            #       ##     ####            #  #   #    #  #    #                                     #   
  # #    #    #  #    #            #  #            #       #      #               #  #   #    #  #    #             #                       #   
    #         #       #             # #             #       #      #             #    #  #    #  #                   #              #       #   
    #       ##     ###              ###             #       #      #             #    #  #####   #                    #   ###        #      #   
    #      #          #             # #              #       #      #            ######  #    #  #               #    #      ##      #      #   
    #     #           #            #  ##             #       #      #            #    #  #    #  #               #####################      #   
    #    #       #    #          ##                  #       #      #            #    #  #    #  #    #                                         
    #    #       #    #                                                          #    #  #    #  #    #                    #                    
  #####  ######   ####                                                           #    #  #####    ####                             #            
                                                                                                                                                
                                                                                                                                                
                                                  
```

## How it Works

At compile time, the `build.rs` script parses the `data/unifont-*.hex` file and emits Rust code.
