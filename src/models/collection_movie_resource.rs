/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: b08981dee068e1ed23e4f45a0d8fe70ef7bf7703
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionMovieResource {
    #[serde(rename = "tmdbId", skip_serializing_if = "Option::is_none")]
    pub tmdb_id: Option<i32>,
    #[serde(rename = "imdbId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<Option<String>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "cleanTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub clean_title: Option<Option<String>>,
    #[serde(rename = "sortTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_title: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::MovieStatusType>,
    #[serde(rename = "overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "images", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub images: Option<Option<Vec<models::MediaCover>>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "ratings", skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Box<models::Ratings>>,
    #[serde(rename = "genres", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Option<Vec<String>>>,
    #[serde(rename = "folder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder: Option<Option<String>>,
}

impl CollectionMovieResource {
    pub fn new() -> CollectionMovieResource {
        CollectionMovieResource {
            tmdb_id: None,
            imdb_id: None,
            title: None,
            clean_title: None,
            sort_title: None,
            status: None,
            overview: None,
            runtime: None,
            images: None,
            year: None,
            ratings: None,
            genres: None,
            folder: None,
        }
    }
}

