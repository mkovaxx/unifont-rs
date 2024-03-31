# unifont-rs

[Unifont](https://en.wikipedia.org/wiki/GNU_Unifont) for Rust

Provides a monochrome bitmap font that covers the entire Unicode Basic Multilingual Plane. Halfwidth glyphs are 8x16, fullwidth are 16x16 pixels.

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
                                                                                                                                           $$   
                                                                                                                                          $     
                                                                                                                                           $$   
                                      $           $       $   $  $  $  $                                                                  $     
    $     $$$$    $$$$                $           $       $  $   $ $  $            $$    $$$$$    $$$$                                      $   
   $$    $    $  $    $           $   $            $       $$     $$$$            $  $   $    $  $    $                                     $   
  $ $    $    $  $    $            $  $            $       $      $               $  $   $    $  $    $             $                       $   
    $         $       $             $ $             $       $      $             $    $  $    $  $                   $              $       $   
    $       $$     $$$              $$$             $       $      $             $    $  $$$$$   $                    $   $$$        $      $   
    $      $          $             $ $              $       $      $            $$$$$$  $    $  $               $    $      $$      $      $   
    $     $           $            $  $$             $       $      $            $    $  $    $  $               $$$$$$$$$$$$$$$$$$$$$      $   
    $    $       $    $          $$                  $       $      $            $    $  $    $  $    $                                         
    $    $       $    $                                                          $    $  $    $  $    $                    $                    
  $$$$$  $$$$$$   $$$$                                                           $    $  $$$$$    $$$$                             $            
                                                                                                                                                
                                                                                                                                                
                                                   
```
