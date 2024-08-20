# kindle2cbz

### Purpose

This is a very simple tool which extracts all images from an **unencrypted** eBook in MOBI format (.mobi or .azw3) and 
places them in a CBZ archive. Images in CBZ file will appear in the order they exist as PDB records in the MOBI file, 
except that the cover will be moved to the start and the thumbnail will be omitted.

I created this so that I would have a simpler alternative to using
[KindleUnpack](https://github.com/kevinhendricks/KindleUnpack) to convert my comic books in Kindle format into CBZ so 
that I can read them in my preferred software. 

### Basic Setup and Usage

An x64 Windows binary is provided in the [releases](https://github.com/shloop/kindle2cbz/releases) section. 
Alternatively, if you have a [Rust development environment](https://www.rust-lang.org/tools/install) set up, you can 
install with Cargo by simply running:

```
cargo install kindle2cbz
``` 

To convert a Kindle eBook to CBZ with no options, run:

```
kindle2cbz <PATH_TO_EBOOK>
```

where <PATH_TO_EBOOK> is the path to a valid unencrypted eBook in MOBI format. If successful, this will output a CBZ 
file with the same base name in the same directory as the input file. For instance, if the input file was `comic.azw3`, 
there would be a new file named `comic.cbz` in the same directory containing the extracted images.

### Note on CBZ Format

A CBZ file is actually just a standard .zip file containing images with a different extension. If you wish, you can 
extract the images from a generated CBZ file by renaming it to .zip and opening it in a file explorer. When using this
program you can also specify an output filename with .zip as the extension if you ust want the images.

### All Options

```
Usage: kindle2cbz [OPTIONS] <IN_FILE> [OUT_FILE]

Arguments:
  <IN_FILE>   Path to mobi file to convert
  [OUT_FILE]  Path to save CBZ to. If omitted, original path and base filename will be used

Options:
  -f, --force    If set, will overwrite any existing file with same path as output
  -h, --help     Print help
  -V, --version  Print version
```