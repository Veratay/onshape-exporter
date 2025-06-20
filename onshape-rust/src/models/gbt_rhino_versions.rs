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
pub enum GbtRhinoVersions {
    #[serde(rename = "V2")]
    V2,
    #[serde(rename = "V3")]
    V3,
    #[serde(rename = "V4")]
    V4,
    #[serde(rename = "V5")]
    V5,
    #[serde(rename = "V6")]
    V6,
    #[serde(rename = "V7")]
    V7,
    #[serde(rename = "V8")]
    V8,
    #[serde(rename = "UNKNOWN")]
    Unknown,

}

impl std::fmt::Display for GbtRhinoVersions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::V2 => write!(f, "V2"),
            Self::V3 => write!(f, "V3"),
            Self::V4 => write!(f, "V4"),
            Self::V5 => write!(f, "V5"),
            Self::V6 => write!(f, "V6"),
            Self::V7 => write!(f, "V7"),
            Self::V8 => write!(f, "V8"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for GbtRhinoVersions {
    fn default() -> GbtRhinoVersions {
        Self::V2
    }
}

