use ktx2_reader::format::Format;
use ktx2_reader::{Header, Reader, RegionDescription};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let tex_path = get_texture_path();
    let file = tokio::fs::read(tex_path).await.expect("Can't open file");
    let reader = Reader::new(&*file).expect("Can't create reader");
    let header = reader.header();
    println!("Header: {:#?}", header);
    assert_head(header);

    let regions_desc = reader.regions_description();
    println!("Regions: {:#?}", regions_desc);
    assert_eq!(regions_desc.len(), header.level_count.max(1) as usize);

    let data = reader.read_data().expect("Can't read data");
    println!("Data len: {:?}", data.len());
    test_data(data, &regions_desc);
    return;
}

fn test_data(dat: &[u8], info: &[RegionDescription]) {
    for region in info {
        let offset = region.offset_bytes as usize;
        let bytes = &dat[offset..offset + 4];
        println!("Bytes for level {:?}: {:?}", region.level, bytes);
    }
}

fn assert_head(header: Header) {
    assert_eq!(header.format, Format::VK_FORMAT_R8G8B8A8_UINT);
    assert_eq!(header.type_size, 1);
    assert_eq!(header.base_width, 1024);
    assert_eq!(header.base_height, 512);
    assert_eq!(header.base_depth, 0);
    assert_eq!(header.layer_count, 0);
    assert_eq!(header.face_count, 1);
    assert_eq!(header.level_count, 11);
    assert_eq!(header.supercompression_scheme, 0);
}

fn get_texture_path() -> PathBuf {
    let mut current_dir = std::env::current_dir().expect("Can't get current directory");
    current_dir.push("data\\test_tex.ktx2");
    current_dir
}
