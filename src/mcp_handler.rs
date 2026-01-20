use crate::{tmdb_client::TmdbClient, tools::TmdbTools};
use async_trait::async_trait;
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler, schema::*};
use std::sync::Arc;

// Define a custom handler for mcp messages
pub struct McpHandler {
    pub tmdb_client: TmdbClient,
}

#[async_trait]
impl ServerHandler for McpHandler {
    /// returns list of available tools.
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

    /// Handles requests to call a specific tool.
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
