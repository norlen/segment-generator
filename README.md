# Segment generator

Tiny helper tool that converts a list of segments to a `Segment Control Character` for the LCD display on a AVR Butterfly. Where each segment letter correspond
to these locations on the LCD.

```
    AAAAAAA
  F H  J  K B
  F  H J K  B
    GGG LLL 
  E  P N M  C
  E P  N  M C
    DDDDDDD
```

## Usage

By default it will look for the file `input.txt` and perform the conversion and print the result to stdout. For other files just pass the path as the first argument.

```
cargo run -- <path>
```
