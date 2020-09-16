# Hyper2ch

## What is this?

[WARNING!] This project is working in progress!

Hyper2ch is unofficial 2channel(Japanese traditional BBS) clone, implemented in Rust.

It is free!

## How to host

Run on PostgreSQL.

```sql
CREATE USER hyper2ch WITH PASSWORD 'example' CREATEDB; # Create an user for Hyper2ch
```

Run on your shell.

```sh
hyper2ch db create # Create a database for Hyper2ch
hyper2ch db init # Initialize a database
hyper2ch run # Run Hyper2ch on your machine!
```

Done!
