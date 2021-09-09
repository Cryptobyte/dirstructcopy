## Directory Structure Copy (`dirstructcopy`)
This is a simple program that copies the directory structure of a directory to another. I created this as a quick tool for myself as I found myself wanting to mirror the structure of a directory without copying the files and I wanted to make something using the Rust language to continue learning with hands-on excercises. This is likely completely useless to most people but it was fun to write and explore the language! In the future I would like to make this use less libraries as I learn more about the programming language because 4 libraries for a simple console utility is too much imo

### Usage
```
dirstructcopy --help

USAGE:
    dirstructcopy [FLAGS] [OPTIONS] --input <INPUT> --output <OUTPUT>

FLAGS:
    -f, --follow-links    
    -h, --help            Print help information
    -r, --recursive       
    -v, --verbose         
    -V, --version         Print version information
    -y, --yes             

OPTIONS:
    -d, --depth <DEPTH>      [default: 4096]
    -i, --input <INPUT>      
    -o, --output <OUTPUT>
```