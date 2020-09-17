use ktx2_reader::format::Format;
use ktx2_reader::Reader;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let tex_path = get_current_dir();
    let file = tokio::fs::File::open(tex_path)
        .await
        .expect("Can't open file");
    let reader = Reader::new(file).await.expect("Can't create reader");
    let header = reader.get_header();
    println!("HEADER: {:#?}", header);

    assert_eq!(header.format, Format::VK_FORMAT_BC2_UNORM_BLOCK);
    assert_eq!(header.type_size, 1);
    assert_eq!(header.base_width, 256);
    assert_eq!(header.base_height, 256);
    assert_eq!(header.base_depth, 0);
    assert_eq!(header.layer_count, 0);
    assert_eq!(header.face_count, 1);
    assert_eq!(header.level_count, 9);
    assert_eq!(header.supercompression_scheme, 0);
    return;
}

fn get_current_dir() -> PathBuf {
    let mut current_dir = std::env::current_dir().expect("Can't get current directory");
    current_dir.push("data\\fb.ktx2");
    current_dir
}
