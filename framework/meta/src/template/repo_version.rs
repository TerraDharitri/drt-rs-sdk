pub enum RepoVersion {
    Main,
    Tag(String),
}

impl RepoVersion {
    pub fn url(&self) -> String {
        match self {
            RepoVersion::Main => {
                "https://github.com/TerraDharitri/drt-rs-sdk/archive/refs/heads/main.zip".to_string()
            },
            RepoVersion::Tag(tag) => {
                format!("https://github.com/TerraDharitri/drt-rs-sdk/archive/refs/tags/v{tag}.zip")
            },
        }
    }

    pub fn temp_dir_name(&self) -> String {
        match self {
            RepoVersion::Main => "drt-rs-sdk-main".to_string(),
            RepoVersion::Tag(tag) => {
                format!("drt-rs-sdk-{tag}")
            },
        }
    }
}
