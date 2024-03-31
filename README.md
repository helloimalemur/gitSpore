# gitSpore
Github backup tool that concurrently downloads all public and private github repo's of the specified user.

### Install from Crates.io
    cargo install gitspore

## Running gitSpore from cli
    Usage: gitspore -u <USER> -o <OUTPUT_FOLDER> -t <TOKEN>

    Options:
    -u <USER>               Github username
    -o <OUTPUT_FOLDER>      folder to save github repo
    -t <TOKEN>              Github api token
    -h, --help              Print help


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
