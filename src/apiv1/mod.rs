//! osu! API v1
//! ---
//!
//! Documentation: <https://github.com/ppy/osu-api/wiki>

mod errors;
mod models;

use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

use crate::data::Mode;

pub use self::errors::{Error, Result};
pub use self::models::*;

const API_BASE: &str = "https://osu.ppy.sh/api";

/// A struct used for interfacing with the osu! API.
pub struct API {
  client: Client,
  api_key: String,
}

impl API {
  /// Creates a new API object with the specified key.
  /// Obtain a key from the [osu! website](https://osu.ppy.sh/p/api).
  pub async fn new(api_key: impl AsRef<str>) -> Result<Self> {
    Ok(API {
      client: Client::builder().build()?,
      api_key: api_key.as_ref().to_owned(),
    })
  }

  /// Make a GET request to an arbitrary endpoint of the OSU API
  pub async fn get<T>(&self, url: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let full_url = Url::from_str(&format!("{}{}", API_BASE, url.as_ref()))?;
    let resp = self.client.get(full_url).send().await?;
    let result = resp.json().await?;
    Ok(result)
  }

  /// Gets all recent
  pub async fn get_user_recent(
    &self,
    user: impl Into<UserLookup>,
    mode: Option<Mode>,
    limit: Option<u32>,
  ) -> Result<Vec<UserScore>> {
    let user = user.into();
    let mode = match mode {
      Some(n) => n as u32,
      None => 0,
    };

    // TODO: construct URL using a library
    let uri = format!(
      "/get_user_recent?k={}&u={}&m={}&limit={}",
      self.api_key,
      user,
      mode,
      limit.unwrap_or(10),
    );
    self.get(uri).await
  }
}

/// A method of looking up a user in the API.
pub enum UserLookup {
  /// Look up by ID
  Id(u32),

  /// Look up by username
  Name(String),
}

impl fmt::Display for UserLookup {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UserLookup::Id(n) => write!(f, "{}", n),
      UserLookup::Name(s) => write!(f, "{}", s),
    }
  }
}

impl From<&str> for UserLookup {
  fn from(s: &str) -> Self {
    UserLookup::Name(s.to_owned())
  }
}

impl From<u32> for UserLookup {
  fn from(n: u32) -> Self {
    UserLookup::Id(n)
  }
}
