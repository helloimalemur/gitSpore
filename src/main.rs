extern crate core;

use config::Config;
use std::collections::HashMap;
use std::env;
use std::time::Duration;

mod get_repos;
use get_repos::get_repos;

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("config/Settings"))
        .build()
        .unwrap();
    let settings_map = config.try_deserialize::<HashMap<String, String>>().unwrap();

    let _args: Vec<String> = env::args().collect();

    let user = "helloimalemur".to_string();
    let auth_key = settings_map
        .get("github_personal_access_token")
        .unwrap()
        .to_string();

    //TODO: params / options
    // if no command line input, fallback on config file input, panic if neither are present
    // statically set username and output folder for now

    let user_repos = get_repos(user, auth_key).await;
    let pb = indicatif::ProgressBar::new(user_repos.len() as u64);
    // print stargazers for each repo, sleeping 2s between repo
    for (int, repo) in user_repos.iter().enumerate() {
        println!("{}", repo.html_url);
        // TODO: download repo to desired location
        pb.println(format!("[+] finished #{}", int));
        pb.inc(1);

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    pb.finish_with_message("done");
}
