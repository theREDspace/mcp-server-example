use crate::tmdb_client::TmdbClient;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolError, CallToolResult, ContentBlock},
};

#[mcp_tool(
    name = "get_actor_info",
    title="Get Actor Information",
    description = concat!( "Search for detailed information about an actor based on their name.",
       "This tool retrieves data such as actor id, biography, filmography, and other relevant ",
       "information to provide a comprehensive profile of the actor.",
       "Use this tool when you want to learn more about a specific actor or explore their career.",
       "Simply provide the actor's name, and the tool will fetch all available details."),
    icons = [
        (src = "https://raw.githubusercontent.com/theREDspace/mcp-server-example/main/icons/stallone-128.png",
        mime_type = "image/png",
        sizes = ["128x128"])
    ],
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct GetActorInfo {
    /// The name of the actor.
    pub actor_name: String,
}

// Implements the `invoke` function, which is executed whenever the client calls this tool.
impl GetActorInfo {
    // Executes the logic for this tool when it is invoked by the client.
    pub async fn invoke(
        &self,
        tmdb_client: &TmdbClient,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        // make an api call and get actor details from tmdb
        let response = tmdb_client
            .actor_info(&self.actor_name)
            .await
            .map_err(|err| CallToolError::from_message(err.to_string()))?;

        // return an error message if no actor with that name was found
        let Some(actor_details) = response else {
            return Ok(CallToolResult::with_error(CallToolError::from_message(
                format!(
                    "No actors matching the name \"{}\" were found",
                    self.actor_name
                ),
            )));
        };

        // get the actor profile image as base64 encoded image and return it in the result
        let image_data = tmdb_client
            .image_as_base64(&actor_details.profile_path.as_ref().unwrap())
            .await
            .unwrap();

        return Ok(CallToolResult::from_content(vec![
            ContentBlock::text_content(actor_details.to_string()), // actor info as string
            ContentBlock::image_content(image_data, "image/jpeg".into()), // actor profile image as base64 blob
        ]));
    }
}
