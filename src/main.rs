extern crate core;

use config::Config;
use std::collections::HashMap;
use std::env;
use std::time::Duration;

mod get_repos;
use get_repos::get_repos;
use crate::get_repos::download_repo;

#[tokio::main]
async fn main() {
    let mut settings_map = HashMap::<String, String>::new();
    let mut user_arg = String::new();
    let mut output_path_arg = String::new();
    let mut github_token_arg = String::new();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        4 => {
            user_arg = String::from(args.get(1).unwrap());
            settings_map.insert("user".to_string(), user_arg.clone());
            output_path_arg = String::from(args.get(2).unwrap());
            settings_map.insert("output".to_string(), output_path_arg.clone());
            github_token_arg = String::from(args.get(3).unwrap());
            settings_map.insert("token".to_string(), github_token_arg.clone());
        }
        _ => load_from_config_file(&mut settings_map),
    }

    fn load_from_config_file(settings_map: &mut HashMap<String, String>) {
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

    let user = settings_map.get("user").expect("invalid user argument");
    let output = settings_map.get("output").expect("invalid output argument");
    let token = settings_map.get("token").expect("invalid token argument");

    println!("User: {}\nOutput Path: {}\n", user, output);

    let user_repos = get_repos(
        user,
        token
    ).await;

    // let pb = indicatif::ProgressBar::new(user_repos.len() as u64);

    // each repo, sleeping 1s between repo
    for (int, repo) in user_repos.iter().enumerate() {
        println!("{}", repo.clone().html_url);
        download_repo(String::from(repo.html_url.as_str()), String::from(output), String::from(token));
        // pb.println(format!("[+] #{}/{}", int, user_repos.len()));
        // pb.inc(1);
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
    // pb.finish_with_message("done");
}
