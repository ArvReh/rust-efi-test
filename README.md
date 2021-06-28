## Building and running 
`# cargo build`
This will build the rust project, optional is the `--release` flag that will output an optimized build.

`# ./qemu {configuration}`
This shell script copies the efi executable to the `iso/` directory and starts qemu in text mode. 
Replace `{configuration}` with whatever configuration you built for in the last step (`release` if you built with the `--release flag`, `debug` otherwise)
_Both commands need to be run in the root directory_
