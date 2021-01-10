use ini::Ini;
use std::path::PathBuf;

pub fn create_default_init(db_path: &PathBuf) -> Ini {
    let mut conf = Ini::new();
    conf.with_section(None::<String>)
        .set("db_path", db_path.to_str().unwrap());
    return conf;
}
