# AGENTS.md

This document provides guidance for agents working with the techshare-mcp codebase.

## Quick Commands

```bash
# Build the project (debug)
cargo build

# Build release binary (required for MCP Inspector)
cargo build --release

# Binary location: target/release/techshare-mcp

# Test with MCP Inspector
npx -y @modelcontextprotocol/inspector@latest
```

**Required Environment Variable**: `TMDB_TOKEN` must be set before running the server or tests.

## Project Overview

- **Language**: Rust Edition 2024
- **Framework**: rust-mcp-sdk 0.8
- **Purpose**: MCP server that provides tools for fetching actor/movie data from TMDB API
- **Transport**: STDIO (via `StdioTransport`)

## Architecture

```
main.rs
  └─> McpHandler (struct with TmdbClient)
        └─> mcp_handler.rs
              ├─> handle_list_tools_request() -> returns TmdbTools::tools()
              └─> handle_call_tool_request()
                    └─> TmdbTools::try_from(params)
                          └─> match dispatch to tool.invoke(&self.tmdb_client)
                                └─> tools/*.rs (GetActorInfo, GetMoviesByActor)
                                      └─> tmdb_client.rs (API calls)
```

## Tool Management

### Adding a New Tool

**Step 1: Create the tool file** (`src/tools/get_movie_info.rs`)

```rust
use crate::tmdb_client::TmdbClient;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolError, CallToolResult, ContentBlock},
};

#[mcp_tool(
    name = "get_movie_info",
    title = "Get Movie Information",
    description = concat!(
        "Search for detailed information about a movie by title.",
        "Returns plot, release date, cast, ratings, and more.",
    ),
    icons = [
        (src = "https://raw.githubusercontent.com/theREDspace/mcp-server-example/main/icons/filename.png",
         mime_type = "image/png",
         sizes = ["128x128"])
    ],
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct GetMovieInfo {
    /// The title of the movie to search for
    pub movie_title: String,
}

impl GetMovieInfo {
    pub async fn invoke(
        &self,
        tmdb_client: &TmdbClient,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        // Tool implementation here
        Ok(CallToolResult::text_content(vec!["result".into()]))
    }
}
```

**Step 2: Register in `src/tools.rs`**

```rust
mod get_actor_info;
mod get_movies_by_actor;
mod get_movie_info; // ADD THIS

use crate::tools::{
    get_actor_info::GetActorInfo,
    get_movies_by_actor::GetMoviesByActor,
    get_movie_info::GetMovieInfo, // ADD THIS
};
use rust_mcp_sdk::tool_box;

tool_box!(TmdbTools, [
    GetActorInfo,
    GetMoviesByActor,
    GetMovieInfo, // ADD THIS
]);
```

**Step 3: Add dispatch in `src/mcp_handler.rs`**

```rust
match requested_tool {
    TmdbTools::GetActorInfo(get_actor_info) => {
        get_actor_info.invoke(&self.tmdb_client).await
    }
    TmdbTools::GetMoviesByActor(get_movie_info) => {
        get_movie_info.invoke(&self.tmdb_client).await
    }
    TmdbTools::GetMovieInfo(movie_info) => { // ADD THIS
        movie_info.invoke(&self.tmdb_client).await
    }
}
```

### Modifying an Existing Tool

1. **Change input fields**: Add/remove/modify struct fields - the `#[derive(JsonSchema)]` handles validation automatically
2. **Update field documentation**: The `///` doc comments on struct fields become the `description` in the JSON Schema sent to clients. These are critical for LLM tool calling - update them when field behavior changes
3. **Update description**: Modify the `description` in `#[mcp_tool(...)]` (overall tool description)
4. **Change icon**: Update the `icons` array in `#[mcp_tool(...)]`
5. **Change return format**: Modify the `invoke()` method implementation

### Field Documentation (IMPORTANT)

**Doc comments on struct fields are used in the tool's JSON Schema:**

```rust
#[derive(JsonSchema)]
pub struct GetActorInfo {
    /// The name of the actor to search for
    pub actor_name: String,
}
```

The `/// The name of the actor to search for` comment becomes the `description` for the `actor_name` parameter in the JSON Schema that MCP clients receive. This is critical because:
- LLMs use these descriptions to understand what to pass as arguments
- Poor or missing field documentation causes incorrect tool calls
- Always document what the field expects, format, and any constraints

**When modifying tools:**
- If you change a field's meaning, update its doc comment
- If you add a new field, always add a doc comment
- If you remove a field, the JSON Schema updates automatically

### Removing a Tool

1. Delete `src/tools/tool_name.rs`
2. Remove `mod tool_name;` from `src/tools.rs`
3. Remove the import from `src/tools.rs`
4. Remove from `tool_box!(TmdbTools, [...])`
5. Remove the match arm in `src/mcp_handler.rs`

## TMDB Client API

Available methods on `TmdbClient` for use in tools:

```rust
// Get movies by actor ID
pub async fn movies_by_actor(&self, actor_id: i64) -> Result<Vec<MovieDetail>, reqwest::Error>

// Get actor details by name
pub async fn actor_info(&self, actor_name: &str) -> Result<Option<PersonDetails>, reqwest::Error>

// Get full image URL from path
pub fn resolve_image_url(image_path: &str) -> String

// Get image as base64 string
pub async fn image_as_base64(&self, image_path: &str) -> Result<String, reqwest::Error>
```

### Shared Types

```rust
// MovieDetail - has Display impl for formatted output
pub struct MovieDetail {
    pub id: i64,
    pub title: String,
    pub release_date: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    // ... other fields
}

// PersonDetails - has Display impl for formatted output
pub struct PersonDetails {
    pub id: u32,
    pub name: String,
    pub biography: String,
    pub birthday: Option<String>,
    pub place_of_birth: Option<String>,
    pub profile_path: Option<String>,
    // ... other fields
}
```

## Return Types

```rust
// Simple text response
CallToolResult::text_content(vec!["text content".into()])

// Multiple content blocks (text + images)
CallToolResult::from_content(vec![
    ContentBlock::text_content("text".into()),
    ContentBlock::image_content(base64_data, "image/jpeg".into()),
])

// Error response
CallToolResult::with_error(CallToolError::from_message("error message"))
```

## Error Handling Pattern

```rust
// Convert reqwest errors to CallToolError
tmdb_client.movies_by_actor(self.actor_id)
    .await
    .map_err(|err| CallToolError::from_message(err.to_string()))?;

// Return "not found" errors
let Some(result) = response else {
    return Ok(CallToolResult::with_error(CallToolError::from_message(
        "No results found",
    )));
};
```

## Code Style

- **Files**: snake_case (`tmdb_client.rs`, `get_actor_info.rs`)
- **Types**: PascalCase (`TmdbClient`, `GetActorInfo`, `MovieDetail`)
- **Functions/Methods**: snake_case (`actor_info()`, `movies_by_actor()`)
- **Variables**: snake_case (`actor_name`, `movie_title`)
- **Constants**: SCREAMING_SNAKE_CASE (`BASE_URL`)
- **Modules**: snake_case (`mcp_handler`, `tools`)
- **Comments**: Use `///` for doc comments on structs, fields, and functions
- **Derives**: Always include `Debug, Clone, Serialize, Deserialize, JsonSchema` on data structs
- **Imports**: Group by crate, use full paths for clarity

## Testing with MCP Inspector

```bash
# Set environment variable
export TMDB_TOKEN="your_token_here"

# Run inspector
npx -y @modelcontextprotocol/inspector@latest

# In the browser UI:
# 1. Select "STDIO" transport
# 2. Enter path: /path/to/target/release/techshare-mcp
# 3. Add env var: TMDB_TOKEN=your_token
# 4. Click Connect
# 5. View and invoke available tools
```

## Dependencies

Key crates used:
- `rust-mcp-sdk = "0.8"` - MCP protocol implementation
- `reqwest = "0.13"` - HTTP client for TMDB API
- `tokio = "1.49"` - Async runtime
- `serde = "1.0"` / `serde_json = "1.0"` - JSON serialization
- `async-trait = "0.1"` - For async trait implementations
- `base64 = "0.22.1"` - Image encoding

## References

- [rust-mcp-sdk Macro Documentation](https://github.com/rust-mcp-stack/rust-mcp-sdk/tree/main/crates/rust-mcp-macros#%EF%B8%8F-mcp_tool-macro)
- [MCP Protocol Specification](https://github.com/modelcontextprotocol/specification)
- [TMDB API Documentation](https://developer.themoviedb.org/docs)
