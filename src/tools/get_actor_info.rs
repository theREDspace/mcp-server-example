use crate::tmdb_client::TmdbClient;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolError, CallToolResult, ContentBlock},
};
use serde_json::{Map, Value};

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

impl GetActorInfo {
    pub async fn invoke(
        &self,
        tmdb_client: &TmdbClient,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        // get actor details from tmdb
        let details = tmdb_client
            .actor_info(&self.actor_name)
            .await
            .map_err(|err| CallToolError::from_message(err.to_string()))?;

        // return an error message if no actor with that name was found
        let Some(actor_details) = details else {
            return Ok(CallToolResult::with_error(CallToolError::from_message(
                format!(
                    "No actors matching the name \"{}\" were found",
                    self.actor_name
                ),
            )));
        };

        let info = format!(
            r#"ID: {}
Name: {}
Date of Birth: {}
Place of Birth: {}
Biography: {}"#,
            actor_details.id,
            actor_details.name,
            actor_details.birthday.unwrap_or_default(),
            actor_details.place_of_birth.unwrap_or_default(),
            actor_details.biography,
        );

        let image_data = tmdb_client
            .image_as_base64(&actor_details.profile_path.unwrap())
            .await
            .unwrap();

        let meta = Some(
            [("actor_id".to_string(), Value::from(actor_details.id))]
                .into_iter()
                .collect::<Map<String, Value>>(),
        );

        return Ok(CallToolResult {
            content: vec![
                ContentBlock::text_content(info),
                ContentBlock::image_content(image_data, "image/jpeg".into()),
            ],
            is_error: None,
            meta,
            structured_content: None,
        });
    }
}
