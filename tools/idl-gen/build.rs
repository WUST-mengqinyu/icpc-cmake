fn main() {
    volo_build::ConfigBuilder::default()
        .plugin(pilota_build::plugin::SerdePlugin)
        .write()
        .unwrap();
}
