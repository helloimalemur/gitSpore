# gitSpore
Github mirroring and backup tool that concurrently downloads or updates all of the repo's of the specified user.
If the repo already exists in the destination folder, that repo will be pulled on.

Note: If no token is provided only the public repo will be downloaded.


### Install from Crates.io
    cargo install gitspore

## Running gitSpore from cli
    Usage: gitspore [OPTIONS] -u <USER>
    
    Options:
    -u <USER>               Github username
    -o <OUTPUT_FOLDER>      folder to save github repo [default: ./]
    -t <TOKEN>              Github token - If no token is provided only the public repos will be downloaded [default: None]
    -h, --help              Print help

## Example CLI commands
    ./gitSpore config-file # use config file
    ./gitSpore -u helloimalemur # Download public repo only
    ./gitSpore -u helloimalemur -t mytoken # Download public and private repo
    ./gitSpore -u helloimalemur -t mytoken -o /tmp/output_dir # Download public and private repo to specified directory

## Running gitSpore from config file
    Usage: gitspore config-file
#### config/Settings.toml
```toml
github_username = "helloimalemur"
output_folder = "/tmp/"
github_personal_access_token = "github_xxxxx"
```

## Development and Collaboration
#### Feel free to open a pull gitsporest, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check
####
