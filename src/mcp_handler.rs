use crate::{tmdb_client::TmdbClient, tools::TmdbTools};
use async_trait::async_trait;
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler, schema::*};
use std::sync::Arc;

// Define a custom handler for mcp messages
pub struct McpHandler {
    pub tmdb_client: TmdbClient,
}

/// MCP server handler implementation.
///
/// Handlers for processing incoming client messages and events can be
/// implemented here. Only the handlers required by this server need to be provided.
///
/// For a complete list of all MCP client messages and events that may be
/// implemented, see:
/// https://github.com/rust-mcp-stack/rust-mcp-sdk/blob/main/crates/rust-mcp-sdk/src/mcp_handlers/mcp_server_handler.rs#L20
#[async_trait]
impl ServerHandler for McpHandler {
    /// returns list of available tools. (STEP 2 from slide)
    async fn handle_list_tools_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: TmdbTools::tools(),
            meta: None,
            next_cursor: None,
        })
    }

    /// Handles client requests to invoke a specific tool (Step 3 from the slide).
    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        // Create a tool instance from the request, or return an error if the request is invalid.
        let requested_tool: TmdbTools = TmdbTools::try_from(params).map_err(CallToolError::new)?;

        // invoke the tool
        match requested_tool {
            TmdbTools::GetActorInfo(get_actor_info) => {
                get_actor_info.invoke(&self.tmdb_client).await
            }
            TmdbTools::GetMoviesByActor(get_movie_info) => {
                get_movie_info.invoke(&self.tmdb_client).await
            }
        }
    }
}
