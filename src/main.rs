//! Example MCP server showcasing MCP implementation, as presented in a REDspace TechShare session.
mod mcp_handler;
mod tmdb_client;
mod tools;
use crate::{mcp_handler::McpHandler, tmdb_client::TmdbClient};
use rust_mcp_sdk::{
    McpServer, StdioTransport, ToMcpServerHandler, TransportOptions,
    error::SdkResult,
    mcp_icon,
    mcp_server::{McpServerOptions, server_runtime},
    schema::*,
};

#[tokio::main]
async fn main() -> SdkResult<()> {
    //STEP 1: Define server name & capabilities
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Techshare MCP Server".into(),
            version: "0.1.0".into(),
            title: Some("Example MCP Server Demonstrating MCP Tools".into()),
            description: Some("An MCP server that retrieves detailed information about actors and movies from the TMDB database.".into()),
            icons: vec![mcp_icon!(
                src = "https://avatars.githubusercontent.com/u/4128628?s=128",
                mime_type = "image/png",
                sizes = ["128x128"],
            )],
            website_url: Some("https://github.com/theREDspace/mcp-server-example".into()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default() // Using default values for other fields
        },
        meta: None,
        instructions: Some("server instructions...".into()),
        protocol_version: ProtocolVersion::V2025_11_25.into(),
    };

    // use stdio transport
    let transport = StdioTransport::new(TransportOptions::default())?;

    // custom handler for managing various incoming client requests.
    let handler = McpHandler {
        tmdb_client: TmdbClient::new(),
    };

    // create server instance
    let server = server_runtime::create_server(McpServerOptions {
        transport,
        handler: handler.to_mcp_server_handler(),
        server_details,
        task_store: None,
        client_task_store: None,
    });

    // start the server
    server.start().await?;

    Ok(())
}
