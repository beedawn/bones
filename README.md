# Bones
This is a Rust and Svelte web application that utilizes a PostgreSQL database to manage bills.

## Installing the database
For instructions on how to install PostgreSQL please see this link:
https://www.postgresql.org/download/

Once the database is configured, load it into your terminal and run the commands in
```bash
sql/build_db.sql
```

In my installation I created a service user and needed to run the commands in the following directory to grant proper permissions to the service account
```bash
sql/insert.sql
```

## Building the application

### API
This application requires a .env file to be configured in the root directory with two variables
```
JWT_KEY="this is the key rust uses to sign the JWT within the login cookie"
POSTGRESS_CONN_STRING=postgres://postgre_username:postgres_password:5432/postgres
```

Please make sure you have rust installed, rust installation instructions are available here:
https://rust-lang.github.io/rustup/

To build the backend navigate to the bones directory and run.
```bash
cargo build
cargo run
```
### Install and Launch Svelte UI
Once that is running you can then navigate to 
```
svelte/frontend
```
And run
```
npm install
npm run dev
```
