# Image Editor

By J. Blackburn

Last Updated Jan 20 2025 (**WIP**)

Simple CLI based low level image editor.

***

## Installation

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

1. hard-code desired filter key and output path into src/add/mod.rs

2. run the program in add mode using cargo, specifying the path to your desired image:

```bash
cargo run -- add <path-to-your-image>
```


---

## Features

1. Modes:
- **Add mode** - 
This mode adds and saves a filter to the image. 
In future will be extended for adding multiple filters and saving batches of filtered images.

- **View Mode** - 
This mode is not implemented. 
In future will be added to allow user to make and view changes in real time before committing to saving the new image.

2. Filters: 
- **Swap Filter** -
This filter takes each rgb pixel and swaps them around according to a pattern called the "key". 
For example, the key "grb" swaps the red and green pixel values for every pixel in the image.


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

