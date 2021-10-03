use ktx2::{Format, Header, Reader};

fn main() {
    let file = include_bytes!("../data/test_tex.ktx2");
    let reader = Reader::new(file).expect("Can't create reader");
    let header = reader.header();
    println!("Header: {:#?}", header);
    assert_head(header);

    let levels = reader.levels().collect::<Vec<_>>();
    assert_eq!(levels.len(), header.level_count.max(1) as usize);

    let data = reader.data();
    println!("Data len: {:?}", data.len());
    test_data(&levels);
}

fn test_data(info: &[&[u8]]) {
    for (i, region) in info.iter().enumerate() {
        println!("Bytes for level {:?}: {:?}", i, &region[..4]);
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
