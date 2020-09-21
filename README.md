# KTX v.2 reader
___

Library, that can asynchronously read, validate and parse KTX v.2 texture files. 

## Features
- [x] Async reading
- [x] Parsing
- [x] Validating
- [ ] No-tokio, syncronious realization
- [ ] [Data format description](https://github.khronos.org/KTX-Specification/#_data_format_descriptor)
- [ ] [Key/value data](https://github.khronos.org/KTX-Specification/#_keyvalue_data)
- [ ] [Supercompression](https://github.khronos.org/KTX-Specification/#_scheme_notes_normative)
## Example

```
async fn main() {
    let tex_path = get_texture_path(); /// Returns path ro texture file
    let file = tokio::fs::File::open(tex_path).await.expect("Can't open file");

    // Crate instance of reader.
    // Load, parse and validate header.
    let mut reader = Reader::new(file).await.expect("Can't create reader"); // Crate instance of reader.

    // Get general texture information.
    let header = reader.header();
    
    // Description of texture regions layout e.g. layers and mip-levels.
    let regions_desc = reader.regions_description();

    // Read Vec<u8> with texture data.
    let data = reader.read_data().await.expect("Can't read data");
}
```

Example of usage can be found in `examples` directory.

## Links
* [KTX tools Khronos Group repository (C++)](https://github.com/KhronosGroup/KTX-Software)
* [KTX v.2 specification](https://github.khronos.org/KTX-Specification/)