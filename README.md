# Telegram Ngrok Bot

A simple bot to fetch the running tunnels from ngrok api.

## Prerequisits

- Ngrok agent should be running.
  
## Configuration

### Ngrok config

```yaml
version: "2"
authtoken: xxxxxx_xxxx
api_key: xxxxxx_xxxx
# other configurations
# full example -> https://ngrok.com/docs/agent/config/#full-example
```

### Bot Config
```yaml
token: xxxx:xxxx_x
chat_id: -xxxx # to get the chat ids -> https://api.telegram.org/bot<replace-with-bot-id>/getUpdates
ngrok_api_url: https://api.ngrok.com/tunnels
```


## Development
```sh
cargo run -- --config path/to/bot-config-file.yml  --ngrok-config path/to/ngrok-config.yml
```

## Production
```sh
telegram_ngrok_bot --config path/to/bot-config-file.yml  --ngrok-config path/to/ngrok-config.yml
```

## Available Commands

```sh
Usage: telegram_ngrok_bot --config <FILE> --ngrok-config <FILE>

Options:
  -c, --config <FILE>        Sets a config file
  -n, --ngrok-config <FILE>  Sets a ngrok config file
  -h, --help                 Print help
  -V, --version              Print version
```
