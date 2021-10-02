use ktx2_reader::Format;
use ktx2_reader::{Header, Level, Reader};
use std::path::PathBuf;

fn main() {
    let tex_path = get_texture_path();
    let file = std::fs::read(tex_path).expect("Can't open file");
    let reader = Reader::new(&*file).expect("Can't create reader");
    let header = reader.header();
    println!("Header: {:#?}", header);
    assert_head(header);

    let levels = reader.levels().collect::<Vec<_>>();
    println!("levels: {:#?}", levels);
    assert_eq!(levels.len(), header.level_count.max(1) as usize);

    let data = reader.data();
    println!("Data len: {:?}", data.len());
    test_data(data, &levels);
}

fn test_data(dat: &[u8], info: &[Level]) {
    for (i, region) in info.iter().enumerate() {
        let offset = region.offset_bytes as usize;
        let bytes = &dat[offset..offset + 4];
        println!("Bytes for level {:?}: {:?}", i, bytes);
    }
}

fn assert_head(header: Header) {
    assert_eq!(header.format, Some(Format::R8G8B8A8_UINT));
    assert_eq!(header.type_size, 1);
    assert_eq!(header.pixel_width, 1024);
    assert_eq!(header.pixel_height, 512);
    assert_eq!(header.pixel_depth, 0);
    assert_eq!(header.layer_count, 0);
    assert_eq!(header.face_count, 1);
    assert_eq!(header.level_count, 11);
    assert_eq!(header.supercompression_scheme, None);
}

fn get_texture_path() -> PathBuf {
    let mut current_dir = std::env::current_dir().expect("Can't get current directory");
    current_dir.push("data/test_tex.ktx2");
    current_dir
}
