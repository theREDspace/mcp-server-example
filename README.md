# TMDB MCP Server Example

This example MCP server demonstrates how to create a custom MCP Server and add your own tools using the MCP protocol.

To use this example, you'll need a TMDB API access token. The server connects to [The Movie Database (TMDB)](https://www.themoviedb.org/) to fetch detailed actor and movie information. Make sure you have a valid TMDB API token ready, as it is required for the server to function properly.



## Features

This MCP server provides two simple tools for interacting with The Movie Database (TMDB):

- **get_actor_info:**  
  Allows you to search for an actor by name and retrieve detailed information such as their biography, date and place of birth and more.

- **get_movies_by_actor:**  
Allows you to retrieve a list of movies associated with a particular actor by providing their TMDB ID.

---



## Prerequisites

This project is written in Rust and uses the [rust-mcp-sdk](https://github.com/rust-mcp-stack/rust-mcp-sdk) library.  
Cargo, Rust’s package manager and build tool, is required to build and run the project.

### 1. Install Rust and Cargo

- **Windows/Mac/Linux:**  
  Visit [https://rustup.rs/](https://rustup.rs/) and follow the instructions to download and run the installer.

- After installation, restart your terminal and check that Rust and Cargo are installed:
  ```
  rustc --version
  cargo --version
  ```

### 2. Get a TMDB API Token

- Sign up for a free account at [TMDB](https://www.themoviedb.org/).
- Go to your account settings → API → Request an API key.
- Once you have your API key (token), keep it handy for the next step.

---

## Building the Project

1. **Clone or Download the Repository**

   If you have Git:
   ```
   git clone https://github.com/theREDspace/mcp-server-example.git
   cd mcp-server-example/techshare-mcp
   ```
   Or, download the ZIP from GitHub and extract it, then open a terminal in the `techshare-mcp` folder.

2. **Build the Project**

   In the `techshare-mcp` directory, run:
   ```
   cargo build --release
   ```

   This will download dependencies and compile the project.  
   The compiled binary will be in `target/release/techshare-mcp`.


---

## Using with MCP Inspector or Cloud Desktop

### MCP Inspector

1. Install and launch [MCP Inspector](https://github.com/theREDspace/mcp-inspector) by running:
```
  npx -y @modelcontextprotocol/inspector@latest
```
2. MCP Inspector will be opened in the browser.
3. Select `STDIO` as "Transport Type" and enter the compiled binary path in the "Command" text box.
4. Set the `TMDB_TOKEN` environment variable in the "Environment Variables" section.
5. Click "Connect".
6. Once connected, you will see the available tools and can invoke them as needed.

### Cloud Desktop



---

## Troubleshooting

- **Missing TMDB Token:**  
  If you see an error about `TMDB_TOKEN must be set in environment`, make sure you set the environment variable before running the server.

- **Build Errors:**  
  Ensure Rust and Cargo are installed and up to date. Run `rustup update` if needed.

- **Cannot Connect from MCP Inspector/Cloud Desktop:**  
  Double-check the path to the binary.

---

## Project Structure

- `src/main.rs` — Entry point, sets up the server.
- `src/mcp_handler.rs` — Handles incoming MCP requests.
- `src/tools/` — Contains the tool definitions, and uses the `tmdb_client`.
- `src/tmdb_client.rs` — Communicates with TMDB API.

---

## More Information

- [MCP Protocol Documentation](https://github.com/theREDspace/mcp)
- [TMDB API Documentation](https://developer.themoviedb.org/docs)

---

## License

This project is for educational and demonstration purposes.
