mod get_actor_info;
mod get_movies_by_actor;

use crate::tools::{get_actor_info::GetActorInfo, get_movies_by_actor::GetMoviesByActor};
use rust_mcp_sdk::tool_box;

// List of tools provided by this server
// To add a new tool, create it in the `/tools/` folder and include it in the list below.
tool_box!(TmdbTools, [GetActorInfo, GetMoviesByActor]);
