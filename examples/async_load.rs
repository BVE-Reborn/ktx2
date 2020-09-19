use ktx2_reader::format::Format;
use ktx2_reader::{Header, Reader};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let tex_path = get_current_dir();
    let file = tokio::fs::File::open(tex_path)
        .await
        .expect("Can't open file");
    let reader = Reader::new(file).await.expect("Can't create reader");
    let header = reader.header();
    println!("HEADER: {:#?}", header);
    assert_head(header);

    let frames_info = reader.levels_index();
    println!("FRAMES INFO: {:#?}", frames_info);
    assert_eq!(frames_info.len(), header.level_count.max(1) as usize);

    let regions_desc = reader.regions_description();
    println!("REGIONS: {:#?}", regions_desc);
    assert_eq!(regions_desc.len(), header.level_count.max(1) as usize);

    return;
}

fn assert_head(header: &Header) {
    assert_eq!(header.format, Format::VK_FORMAT_BC2_UNORM_BLOCK);
    assert_eq!(header.type_size, 1);
    assert_eq!(header.base_width, 256);
    assert_eq!(header.base_height, 256);
    assert_eq!(header.base_depth, 0);
    assert_eq!(header.layer_count, 0);
    assert_eq!(header.face_count, 1);
    assert_eq!(header.level_count, 9);
    assert_eq!(header.supercompression_scheme, 0);
}

fn get_current_dir() -> PathBuf {
    let mut current_dir = std::env::current_dir().expect("Can't get current directory");
    current_dir.push("data\\fb.ktx2");
    current_dir
}
