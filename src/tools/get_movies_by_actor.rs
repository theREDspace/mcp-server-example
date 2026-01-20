use crate::tmdb_client::TmdbClient;
use rust_mcp_sdk::{
    macros::{JsonSchema, mcp_tool},
    schema::{CallToolError, CallToolResult},
};

#[mcp_tool(
    name = "get_movies_by_actor",
        title = "Get Movies by Actor ID",
        description = concat!(
            "Retrieve a list of movies featuring a specific actor. ",
            "Specify `actor_id` to search for movies that the actor appeared in. ",
        ),
    icons = [
        (src = "https://raw.githubusercontent.com/theREDspace/mcp-server-example/main/icons/movies-128.png",
        mime_type = "image/png",
        sizes = ["128x128"])
    ],
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct GetMoviesByActor {
    /// Required filter: return movies for this actor ID
    pub actor_id: i64,
}

impl GetMoviesByActor {
    pub async fn invoke(
        &self,
        tmdb_client: &TmdbClient,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        // retrieve list of movies the actor appeared in
        let movies = tmdb_client
            .movies_by_actor(self.actor_id)
            .await
            .map_err(|err| CallToolError::from_message(err.to_string()))?;

        if movies.is_empty() {
            return Ok(CallToolResult::with_error(CallToolError::from_message(
                "No movies were found!",
            )));
        }

        let result = movies
            .iter()
            .enumerate()
            .map(|(index, movie)| format!("{}. {}", index, movie))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(CallToolResult::text_content(vec![result.into()]))
    }
}
