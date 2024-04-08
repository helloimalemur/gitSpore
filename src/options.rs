use clap::Parser;
use config::Config;
use std::collections::HashMap;

#[derive(Parser)]
pub struct Arguments {
    /// Github username
    #[clap(short)]
    pub user: String,
    /// folder to save github repo
    #[clap(short, default_value="./")]
    pub output_folder: String,
    /// Github api token
    #[clap(short, default_value="None")]
    pub token: Option<String>,
}

pub fn load_from_clap() -> (String, String, String) {
    let options = Arguments::parse();
    let user = options.user.to_string();
    let output = options.output_folder.to_string();
    let mut token = String::new();
    match options.token {
        None => {token = "".to_string()}
        Some(tk) => {token = tk}
    }

    (user, output, token)
}

pub fn load_from_config_file(settings_map: &mut HashMap<String, String>) {
    let config = Config::builder()
        .add_source(config::File::with_name("config/Settings"))
        .build()
        .unwrap();
    let config_map = config.try_deserialize::<HashMap<String, String>>().unwrap();

    settings_map.insert(
        "user".to_string(),
        config_map.get("github_username").unwrap().to_string(),
    );

    settings_map.insert(
        "output".to_string(),
        config_map.get("output_folder").unwrap().to_string(),
    );

    settings_map.insert(
        "token".to_string(),
        config_map
            .get("github_personal_access_token")
            .unwrap()
            .to_string(),
    );
}
