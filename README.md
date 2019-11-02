# Prerequisites

## Install Rust using rustup

https://www.rust-lang.org/tools/install

# Database setup

Put **DATABASE_URL** string in *.env* file on project root. Example content:

`DATABASE_URL=postgres://username:password@localhost:5432/event_service`

Install PostgreSQL on your system and ensure the credentials above can access 'event_service' database.

If you're starting from scratch you can follow these steps using *psql* with *postgres* user:

- $ `sudo -i -u postgres`
- $ `psql`
- postgres=# `CREATE USER username WITH PASSWORD 'password';`
- postgres=# `CREATE DATABASE event_service OWNER username;`
- postgres=# `\q`

Database tables will be created by event-service on startup if missing.

# Running the service

`$ cargo run`

From another terminal you can try:

`$ curl -i -X POST -H 'Content-Type: application/json' -d '{"event_id": 3, "user": "admin", "text": "A comment", "timestamp": 123}' http://localhost:3000/comments`

Open the database (for instance with *pgAdmin*) and check that *comments* table contains the row.
