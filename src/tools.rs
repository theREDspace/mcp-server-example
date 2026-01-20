mod get_actor_info;
mod get_movies_by_actor;

use crate::tools::{get_actor_info::GetActorInfo, get_movies_by_actor::GetMoviesByActor};
use rust_mcp_sdk::tool_box;

tool_box!(TmdbTools, [GetActorInfo, GetMoviesByActor]);
