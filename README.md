# env-manager

Manage dotenv files via cli

## Installation

```
    cargo install env-manager
```

Homebrew (soon)

```
    brew tap rawnly/tap
    brew install env-manager
```

## Usage

```sh
Usage: dotenv [OPTIONS] <COMMAND>

Commands:
  list        Print all .env variables
  list-files  List all env files
  set         Set environment
  get         Get environment
  help        Print this message or the help of the given subcommand(s)

Options:
  -s, --stage <STAGE>  Set stage
  -h, --help           Print help
  -V, --version        Print version
```

## Example

```sh
http POST https://my-service-auth.com/auth/sign-in username=root password=toor \
    | jq -r .accessToken \
    | xargs dotenv set ACCESS_TOKEN
```

> Set the response accessToken to the `.env` file
