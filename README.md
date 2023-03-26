# Real app
Real app is a real-time chat application built with rust.

# Dependencies
actix-web – Provides an asynchronous and actor-based web framework for building high-performance, scalable, and modular web applications in Rust.


actix-cors – Provides a convenient way to configure CORS policies in Actix-Web applications.


serde_json – For serializing and deserializing data using the JSON format.


serde – Provides a convenient and flexible solution for serializing and deserializing data structures 
in various formats including JSON, YAML, BSON, and more using a macro-based framework in Rust.


chrono – Date and time library for Rust.


env_logger – Enables dynamic control of logging behaviour in Rust applications using environment variables to set log level and format.


dotenv – Loads the content of a .env file into environment variables, which can then be read by the application using standard Rust methods.


uuid – A library to generate and parse UUIDs in Rust.


sqlx – Provides an efficient and user-friendly interface for accessing and manipulating SQL databases in Rust, with a high-level, asynchronous API for executing SQL statements, retrieving data, and managing database transactions.


sqlx-cli – The sqlx-cli binary provides a command-line interface for managing and applying changes to a SQL database schema in a version-controlled and repeatable way, making it easier to maintain the database structure as the application evolves over time.


jsonwebtoken – For encoding and decoding JSON Web Tokens in Rust.

argon2 – Provides functions to securely hash passwords for storage and verify the authenticity of passwords during authentication.


rand_core – This crate provides low-level, core traits for random number generators (RNGs) and algorithms that use RNGs.


cargo-watch – Automatically rebuilds a Rust project whenever a change is made to its source files.


To add dependencies run
```bash 
cargo add <name-of-dependency>
```

In the event that the current versions of the crates cause any issues with your application, you can revert to the specified versions in the Cargo.toml file below.