/*
 * Onshape REST API
 *
 * ## Welcome to the Onshape REST API Explorer  To use this API explorer, sign in to your [Onshape](https://cad.onshape.com) account in another tab, then click the **Try it out** button below (it toggles to a **Cancel** button when selected).  See the **[API Explorer Guide](https://onshape-public.github.io/docs/api-intro/explorer/)** for help navigating this API Explorer, including **[authentication](https://onshape-public.github.io/docs/api-intro/explorer/#authentication)**.  **Tip:** To ensure the current session isn't used when trying other authentication techniques, make sure to [remove the Onshape cookie](https://support.google.com/chrome/answer/95647#zippy=%2Cdelete-cookies-from-a-site) as per the instructions for your browser. Alternatively, you can use a private or incognito window.  ## See Also  * [Onshape API Guide](https://onshape-public.github.io/docs/): Our full suite of developer guides, to be used as an accompaniment to this API Explorer. * [Onshape Developer Portal](https://cad.onshape.com/appstore/dev-portal): The Onshape portal for managing your API keys, OAuth2 credentials, your Onshape applications, and your Onshape App Store entries. * [Authentication Guide](https://onshape-public.github.io/docs/auth/): Our guide to using API keys, request signatures, and OAuth2 in  your Onshape applications.
 *
 * The version of the OpenAPI document: 1.198.56658-8969b956ea73
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GbtNodeChange {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "MOVED")]
    Moved,
    #[serde(rename = "MODIFIED")]
    Modified,
    #[serde(rename = "MOVED_AND_MODIFIED")]
    MovedAndModified,
    #[serde(rename = "ADDED")]
    Added,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "UNKNOWN")]
    Unknown,

}

impl std::fmt::Display for GbtNodeChange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Moved => write!(f, "MOVED"),
            Self::Modified => write!(f, "MODIFIED"),
            Self::MovedAndModified => write!(f, "MOVED_AND_MODIFIED"),
            Self::Added => write!(f, "ADDED"),
            Self::Deleted => write!(f, "DELETED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for GbtNodeChange {
    fn default() -> GbtNodeChange {
        Self::None
    }
}

