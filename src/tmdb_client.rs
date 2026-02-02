use base64::{Engine, engine::general_purpose};
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use std::{env, fmt::Formatter};

const BASE_URL: &str = "https://api.themoviedb.org/3";

/// A simple client for interacting with The Movie Database (TMDB) API.
pub struct TmdbClient {
    client: Client,
}

impl TmdbClient {
    /// Creates a new TMDB client using the API token from the environment variable `TMDB_TOKEN`.
    ///
    /// # Panics
    /// Panics if the `TMDB_TOKEN` environment variable is not set.
    pub fn new() -> Self {
        let auth_token = env::var("TMDB_TOKEN").expect("TMDB_TOKEN must be set in environment");
        // Build the client with default headers
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", auth_token)).unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Self { client }
    }

    /// Retrieves a list of movies featuring the specified actor by TMDB actor ID.
    ///
    /// # Arguments
    /// * `actor_id` - The TMDB ID of the actor.
    ///
    /// # Returns
    /// * `Ok(Vec<MovieDetail>)` - List of movies the actor appeared in.
    /// * `Err(reqwest::Error)` - If the request or parsing fails.
    pub async fn movies_by_actor(&self, actor_id: i64) -> Result<Vec<MovieDetail>, reqwest::Error> {
        // https://api.themoviedb.org/3/discover/movie?with_cast=
        let url = format!("{BASE_URL}/discover/movie");

        let response = self
            .client
            .get(url)
            .query(&[("with_cast", actor_id.to_string())])
            .send()
            .await?;

        let result: MovieResponse = response.json().await?;

        Ok(result.results)
    }

    /// Searches for an actor by name and returns their TMDB ID if found.
    /// this is used internally to find actor id by name, other details will be retrieved by other endpoints
    ///
    /// # Arguments
    /// * `actor_name` - The name of the actor to search for.
    ///
    /// # Returns
    /// * `Ok(Some(id))` - The TMDB ID of the actor if found.
    /// * `Ok(None)` - If no actor is found.
    /// * `Err(reqwest::Error)` - If the request or parsing fails.
    async fn actor_id(&self, actor_name: &str) -> Result<Option<i64>, reqwest::Error> {
        // https://api.themoviedb.org/3/search/person?query=
        let url = format!("{BASE_URL}/search/person");
        let response = self
            .client
            .get(url)
            .query(&[("query", actor_name), ("language", "en-US")])
            .send()
            .await?;

        let json_value: Value = response.json().await?;

        // extract the .results.id from the response json and return it
        Ok(json_value
            .get("results")
            .and_then(|r| r.as_array())
            .and_then(|arr| {
                arr.iter()
                    .find_map(|item| item.get("id").and_then(|id| id.as_i64()))
            }))
    }

    /// Retrieves detailed information about an actor by name.
    ///
    /// # Arguments
    /// * `actor_name` - The name of the actor.
    ///
    /// # Returns
    /// * `Ok(Some(PersonDetails))` - Detailed info if the actor is found.
    /// * `Ok(None)` - If no actor is found.
    /// * `Err(reqwest::Error)` - If the request or parsing fails.
    pub async fn actor_info(
        &self,
        actor_name: &str,
    ) -> Result<Option<PersonDetails>, reqwest::Error> {
        let Some(person_id) = self.actor_id(actor_name).await? else {
            return Ok(None);
        };

        // https://api.themoviedb.org/3/search/person/{id}
        let response = self
            .client
            .get(format!("{BASE_URL}/person/{person_id}"))
            .send()
            .await?;

        Ok(Some(response.json::<PersonDetails>().await?))
    }

    /// Resolves a TMDB image path to a full image URL.
    ///
    /// # Arguments
    /// * `image_path` - The relative path to the image from TMDB.
    ///
    /// # Returns
    /// * `String` - The full URL to the image.
    pub fn resolve_image_url(image_path: &str) -> String {
        let image_size = "w92";
        format!("https://image.tmdb.org/t/p/{image_size}{image_path}")
    }

    /// Downloads an image from a URL and encodes it as a base64 string.
    ///
    /// # Arguments
    /// * `image_url` - The full URL to the image.
    ///
    /// # Returns
    /// * `Ok(String)` - The base64-encoded image data.
    /// * `Err(reqwest::Error)` - If the request or encoding fails.
    async fn image_url_to_base64(&self, image_url: &str) -> Result<String, reqwest::Error> {
        let response = self
            .client
            .get(image_url)
            .send()
            .await?
            .error_for_status()?;

        let bytes = response.bytes().await?;

        let base64_string = general_purpose::STANDARD.encode(&bytes);

        Ok(base64_string)
    }

    /// Retrieves an image from TMDB by its path and returns it as a base64 string.
    ///
    /// # Arguments
    /// * `image_path` - The relative path to the image from TMDB.
    ///
    /// # Returns
    /// * `Ok(String)` - The base64-encoded image data.
    /// * `Err(reqwest::Error)` - If the request or encoding fails.
    pub async fn image_as_base64(&self, image_path: &str) -> Result<String, reqwest::Error> {
        self.image_url_to_base64(Self::resolve_image_url(image_path).as_str())
            .await
    }
}

// TMDB Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieDetail {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<u32>,
    pub id: i64,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u32,
}

/// Implements Display for MovieDetail to show the movie title and release year (if available).
impl Display for MovieDetail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let release_year = if self.release_date.len() > 4 {
            Some(self.release_date[0..4].to_string())
        } else {
            None
        };
        write!(
            f,
            "{} {}",
            self.title,
            release_year
                .map(|year| format!("({year})"))
                .unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieResponse {
    results: Vec<MovieDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonDetails {
    /// Whether the person is marked as adult content
    pub adult: bool,

    /// Alternative names in different languages/scripts
    #[serde(rename = "also_known_as")]
    pub also_known_as: Vec<String>,

    /// Biography text (often sourced from Wikipedia or TMDB editors)
    pub biography: String,

    /// Birth date in YYYY-MM-DD format
    pub birthday: Option<String>,

    /// Death date in YYYY-MM-DD format (null if still alive)
    pub deathday: Option<String>,

    /// Gender code (0 = unknown, 1 = female, 2 = male, 3 = non-binary)
    pub gender: u8,

    /// Official personal or agency homepage URL (often null)
    pub homepage: Option<String>,

    /// TMDB person ID
    pub id: u32,

    /// Corresponding IMDb ID (with "nm" prefix)
    #[serde(rename = "imdb_id")]
    pub imdb_id: Option<String>,

    /// Primary department this person is known for
    #[serde(rename = "known_for_department")]
    pub known_for_department: String,

    /// Primary name used for display
    pub name: String,

    /// Place of birth (city, country, etc.)
    #[serde(rename = "place_of_birth")]
    pub place_of_birth: Option<String>,

    /// Popularity score (higher = more popular)
    pub popularity: f64,

    /// Relative path to profile image
    #[serde(rename = "profile_path")]
    pub profile_path: Option<String>,
}

impl Display for PersonDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"ID: {}
Name: {}
Date of Birth: {}
Place of Birth: {}
Biography: {}"#,
            self.id,
            self.name,
            self.birthday.as_deref().unwrap_or_default(),
            self.place_of_birth.as_deref().unwrap_or_default(),
            self.biography
        )
    }
}
