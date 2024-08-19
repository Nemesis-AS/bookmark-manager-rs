# Bookmark Manager

Bookmark Manager is a web application for managing bookmarks. It is built using Rust with the Actix web framework and Diesel ORM for database interactions.

## Features

-   Add, view, and delete bookmarks
-   Filter bookmarks by tags
-   RESTful API

## Project Architecture

### Server (`server/`)

The server is the core of the app and contains the actual functionality of managing the bookmark database.

#### Server Stack

-   Rust
-   Actix-web
-   Sqlite3

### Browser Extension (`extension/`)

\[WIP\] The extensions will allow the users to export/import their bookmarks and also add and access the bookmarks easily.\

### Frontend
\[Planned\] A web Interface is also planned to be bundled with the **server** to allow the users to manage their bookmarks without the need of an extension.

## Getting Started

### Prerequisites

-   Rust (latest stable version)

### Running the Server Locally

1. Clone the repository:

    ```sh
    git clone https://github.com/Nemesis-AS/bookmark-manager-rs.git
    cd bookmark-manager-rs/server
    ```

2. Set up the environment variables:

    Create a `.env` file in the root directory and add the following:

    ```env
    DATABASE_URL=db.sqlite3
    ```

    A `.env.example` file has been provided. Just fill in the data and rename it to `.env`

3. Run the database migrations:

    ```sh
    diesel migration run
    ```
    **Note:** This step currently requires the [Diesel CLI](https://diesel.rs/guides/getting-started), it will be moved to embedded migrations at some point in the near future.

4. Build and run the server:

    ```sh
    cargo run
    ```

5. The server will be running at [`http://127.0.0.1:8080`](http://127.0.0.1:8080).


## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
