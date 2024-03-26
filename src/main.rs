extern crate core;

use config::Config;
use std::collections::HashMap;
use std::{env, process, thread};
use std::path::Path;
use std::process::Stdio;
use std::thread::JoinHandle;
use std::time::Duration;

mod get_repos;
use crate::get_repos::download_repo;
use get_repos::*;

#[tokio::main]
async fn main() {
    let mut settings_map = HashMap::<String, String>::new();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        4 => {
            let user_arg = String::from(args.get(1).unwrap());
            settings_map.insert("user".to_string(), user_arg.clone());
            let output_path_arg = String::from(args.get(2).unwrap());
            settings_map.insert("output".to_string(), output_path_arg.clone());
            let github_token_arg = String::from(args.get(3).unwrap());
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

    let user_repos = get_repos(user, token).await;

    // let pb = indicatif::ProgressBar::new(user_repos.len() as u64);

    let mut handles: Vec<JoinHandle<()>> = vec![];

    // each repo, sleeping 1s between repo
    for (_int, repo) in user_repos.iter().enumerate() {
        // println!("{}", repo.clone().html_url);
        let repo_name = repo.html_url.as_str().split('/').last().unwrap();
        let final_output_path = format!("{}{}/", output, repo_name);

        if Path::new(final_output_path.as_str()).exists() {
            let handle = update_repo(final_output_path);
            handles.push(handle);
        } else {
            let handle = download_repo(
                String::from(repo.html_url.as_str()),
                String::from(repo_name),
                String::from(final_output_path),
                String::from(token),
            );
            handles.push(handle);
            // pb.println(format!("[+] #{}/{}", int, user_repos.len()));
            // pb.inc(1);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // clean up handles
    for handle in handles {
        handle.join().unwrap()
    }
}
