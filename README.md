# Image Editor

By J. Blackburn

Last Updated Apr 8 2025 (**WIP**)

Simple CLI based low level image editor.

<div>
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/iris/02.png?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/iris/13.png?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/iris/16.png?raw=true" width="205">
</div>

<div>
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/orchid/01.png?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/orchid/02.png?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/orchid/04.png?raw=true" width="205">
</div>

<div>
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/rose/03.png?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/rose/04.png?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/rose/05.png?raw=true" width="205">
</div>

***



## Installation

0. Ensure Rust is installed: [official rust installation guide](https://www.rust-lang.org/tools/install)

1. Clone the repository:
   ```bash
   git clone https://github.com/jakeblackburnn/image-editor.git
   ```
2. Navigate to the project directory:
   ```bash
   cd image-editor
   ```

---

## Usage


- single run:

```bash
cargo run -- add <input> <filter> <output>
```

- batch input:

```bash
cargo run -- add -b <input_directory> <filter> <output_directory>
```

- batch output:

```bash
cargo run -- add <input> -s <filter_set> <output_directory>
```

- view mode: 

```bash
cargo run -- view <input>
```

---

## Features

1. Modes:

- **Add mode** - 
This mode adds a filter or filters to an image or images. 
three sub modes: single run, batch input run, batch output run.
batch input is not implemented yet.

- **View Mode** - 
This mode allows a user to add view changes to an image before saving.
Will be updated in future for better UX and with Reset option

2. Filters:

pass a filter identifier to add / view to apply the filter.
identifiers are of the form \<filter name>-\<key string>


- **Invert** -
this filter inverts all the pixel values in the image, does not require a key string. 

- **Swap** -
this filter swaps around the rgb values for each pixel based on the pattern provided in the key string.
Examples: swap-brg, swap-ggg.

- **Plus** -
this filter adds the provided value to each pixel component, or to only one pixel component if provided. 
Examples: plus-50, plus-r100

- **Minus** -
this filter subtracts the provided value from each pixel component, or from only one pixel component if provided. 
Examples: minus-50, minus-r100

- **Mult** -
this filter multiplies each pixel component by the provided value, or individual components as with plus / minus. multiplier must include a decimal point.
Examples: mult-1.1, mult-r2.0

- **Black/White** -
this filter colors the image black and white, based on a provided "split" value. all pixels with average brightness above the split will become white, all below become black.
Example: bw-124

- **Colorize** -
this filter replaces all pixels of a given color with a different color. specify current and substitute colors in the form R.G.B, separated by a comma. Examples: colorize-0.0.0,255.255.255 (turns all black pixels white), colorize-255.255.255,255.0.0 ( turns all white pixels red )

---

## Technologies Used

- Rust, Regex, egui

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Contribution Guidelines

This project is not intended for collaboration, However, If you would like to contribute:

1. **Issues** - Use the [Issues tab](https://github.com/jakeblackburnn/image-editor/issues) to report issues or make suggestions. 
2. **Changes** - Fork the repo, create, commmit, and push a new branch (**feature/feature-name**), then open a pull request. 
3. **Contact Me** - jackblackburnn@icloud.com

