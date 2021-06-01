<p align="center">
    <img src="./readme/logo.svg" width="256" height="256" alt="logo">
</p>

# GTM sync
This client is meant to be run wherever it can access your git repository to fetch time data 
from notes and sync it up to backend ([gtm-api](https://github.com/DEVELOPEST/gtm-api))  

## Installation
You can install it to debian based system via `.deb` package which you can find in Gihub Releases.

## Usage
Add repo with
```bash
curl -H "Content-Type: application/json" \
  --request POST \
  --data '{"url": "<ssh_clone_url>"}' \
  http://localhost:8090/repositories
```
or by manually editing config file.  
Example config can be found in `config/config.toml`
  
Sync repo with  
```bash
curl http://localhost:8090/repositories/<provider>/<user>/<repo>/sync
```
For more endpoints see `src/server/controller.rs`

## Contributing

We are happy to welcome all contributions!
See [CONTRIBUTING.md](./CONTRIBUTING.md) for more details.