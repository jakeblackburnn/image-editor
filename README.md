# Image Editor

By J. Blackburn

Last Updated Mar 30 2025 (**WIP**)

Simple CLI based low level image editor.

<div>
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/in/iris.jpg?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/out1.png?raw=true" width="205">
<img src="https://github.com/jakeblackburnn/image-editor/blob/main/out/out2.png?raw=true" width="205">
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
This mode is barely implemented. 
All it does right now is show the provided image.
In future will be added to allow user to make and view changes in real time before committing to saving the new image.

---

## Technologies Used

- Rust, Regex

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Contribution Guidelines

This project is not intended for collaboration, However, If you would like to contribute:

1. **Issues** - Use the [Issues tab](https://github.com/jakeblackburnn/image-editor/issues) to report issues or make suggestions. 
2. **Changes** - Fork the repo, create, commmit, and push a new branch (**feature/feature-name**), then open a pull request. 
3. **Contact Me** - jackblackburnn@icloud.com

