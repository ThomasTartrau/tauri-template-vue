# Tauri app template

This is a tauri template for building desktop and web applications with Tauri v2.

This template using  :

- Tauri-App
- Vue3
- Typescript
- Rust
- Actix-web
- Biscuit-auth

## Requirements

- Read the [Tauri setup guide](https://tauri.studio/en/docs/getting-started/intro/#setting-up-your-environment)
- Latest version of [Node](https://nodejs.org/)
- Lasest version of [Rust](%5Bwww.rust-lang.org%5D\(http://www.rust-lang.org\))
- [PostgreSQL](https://www.postgresql.org) database

## Get started

1. Clone this project

2. Go to /frontend folder and install the requirement dependencies in the frontend part (tauri-app)

   ```shell
   pnpm install
   ```

3. Go to /api folder and create the [run.sh](http://run.sh) file a the root of /api folder. Copy and configure the file with your config.

   ```
   #!/usr/bin/env bash
   
   export BISCUIT_PRIVATE_KEY="" // When you try to start the api, if this field is empty the api will return you an correct and random private key.
   export DATABASE_URL="postgres://username:password@host:port/database" // Only support postgres
   
   export EMAIL_SENDER_ADDRESS="support@template.com"
   export SMTP_CONNECTION_URL="smtp://localhost:1025"
   export APP_URL="http://localhost:8080/"
   export API_URL="http://localhost:8080/"
   
   cargo run -- "$@"
   ```

4. Init the .env at the root of the /api folder. Copy and configure the file with your config.

   ```
   DATABASE_URL="postgres://username:password@host:port/database"
   ```

5. Run the migration to init the database

   ```shell
   sqlx migrate run
   ```

6. Install the requirement dependencies in backend api (Rust api using actix-web) and start the api

   ```shell
   ./run.sh
   ```

7. Run in developpement mode the frontend

   ```shell
    pnpm run tauri dev
   ```

## Features

- Login
- Register
- Logout
- Refresh biscuit token (automatically made in frontend before token is expired)
- Email verification (send email with smtp configuration. After registration, the user need to verify he’s email. Not configurable for now)
- Resend email verification
- Reset password (send email and give a link with token to authentificated the user and give the possibility to reset he’s password if he lost it)
- Change password (if user is logged in)
- Send a profile (stored in static frontend application (/public)
- Change he’s first and last name
- Delete the user account

All this features work (frontend - backend)

## License

This project is licensed under the MIT License - see [License](<https://github.com/ThomasTartrau/tauri-template-vue/blob/main/LICENSE>) for more information.

Copyright (c) 2024 Thomas Tartrau.
