use ini::Ini;
use std::path::PathBuf;

const DEFAULT_TAGS: &str = "Ready, Blocked, Completed";
const DEFAULT_CONTEXT: &str = "Inbox, Work, Life";

pub fn create_default_init(db_path: &PathBuf) -> Ini {
    let mut conf = Ini::new();
    conf.with_section(None::<String>)
        .set("db_path", db_path.to_str().unwrap())
        .set("tag", DEFAULT_TAGS)
        .set("context", DEFAULT_CONTEXT);

    //conf.with_section(Some("User"))
    //.set("given_name", "Tommy")
    //.set("family_name", "Green")
    //.set("unicode", "Raspberry树莓");
    //conf.with_section(Some("Book")).set("name", "Rust cool");
    return conf;
}
