pub fn get_store_path() -> String {
    let proj_dirs = xdg::BaseDirectories::with_prefix("rtd");

    String::from(
        proj_dirs
            .place_config_file("store.json")
            .unwrap()
            .to_str()
            .unwrap(),
    )
}
