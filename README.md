# chipboi
Chip8 interpreter in rust

Works on most of the games I tested

No sound :(

# Usage
Games can be loaded by passing a path to the ROM as a command line argument. For example:
```sh
cargo run --release "ROMs/BRIX"
```
Or if you are running the binary directly
```sh
chipboi "ROMs/BRIX"
```

Other than that:
```
Usage: chipboi <path to file> [Options]
Options:
  -s <scale>   set framebuffer scale to passed integer
  -l           use legacy instruction implementations
  -f           lock to 60 FPS
  -h           show help menu
```

Epic
