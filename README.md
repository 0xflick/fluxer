# Fluxer #
Generate images like those at http://koaning.io/fluctuating-repetition.html

## Installation ##
Install via `cargo install fluxer` or download and run `cargo build`.

## Usage ##
```
Usage: fluxer OUTPUT [options]

Options:
        --height HEIGHT height of the output image in pixels. Defaults to 1080
        --width WIDTH   width of the output image in pixels. Defaults to 1920
        --density DENSITY
                        density of points in final image. Defaults to 0.1
    -i, --invert        inverts colors of the final image. Defaults to false
    -s, --smooth        smooth final image by super-sampling. Defaults to
                        false
    -h, --help          print this help menu
```
