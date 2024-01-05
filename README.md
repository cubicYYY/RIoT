# RIoT: Rust IoT Platform

Simple yet powerful platform to manage IoT devices.
**WIP**

## Usage
1. `git clone`
2. `cd ./RIoT`
3. Config some configuration files:
    +  `src/riot-frontend/src/parser.ts` Data format defs
    +  `src/riot-frontend/config.ts` Backend endpoint
    +  `src/riot-backend/riot_config.toml` Backend config
4.  `docker-compose up -V`

## Endpoints
+ API Docs Online: `/api-doc/`
    + With detailed descriptions
+ Backend API endpoints prefix: `/api`

## Configs

### `src/riot-backend/riot_config.toml`
```toml
#example config file
[riot]
host = "http://your_host:7107" # Port is from docker-compose.yml
# WARN: localhost is NOT an alias of '127.0.0.1': please, make them of both frontend and backend matches

password_salt = "argon2_hash_salt" # For encryption
debug = true # Enabled= disabled CORS
[email] # For sending verification email
addr = "YourRiot@email.com"
smtp_relay_server = "smtp.email.com" # SMTP server
smtp_username = "riotriot@email.com" # SMTP username
smtp_password = "abcdefghijklmnop"   # SMTP password/code
[jwt]
maxage = 86400            # seconds to expire (re-login interval)
secret = "jwt_enc_secret"
[mqtt]
host = "rumqttd" # Service name in docker-compose.yml
port = 1883
[mysql] # DB connection configs, !make sure to match with docker-compose.yml
username = "riot"
password = "Your_password"
host = "mysql" # Service name in docker-compose.yml
port = 3306
database = "riot"

```