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
pub enum GbtuiSelectionType {
    #[serde(rename = "ENTITY")]
    Entity,
    #[serde(rename = "FEATURE")]
    Feature,
    #[serde(rename = "BODY")]
    Body,
    #[serde(rename = "OCCURRENCE")]
    Occurrence,
    #[serde(rename = "USERCODE")]
    Usercode,
    #[serde(rename = "ROLLBACKBAR")]
    Rollbackbar,
    #[serde(rename = "ELEMENT")]
    Element,
    #[serde(rename = "MATE")]
    Mate,
    #[serde(rename = "MATE_CONNECTOR")]
    MateConnector,
    #[serde(rename = "EDGE_POINT")]
    EdgePoint,
    #[serde(rename = "MESH_POINT")]
    MeshPoint,
    #[serde(rename = "TABLE_ITEM")]
    TableItem,
    #[serde(rename = "SKETCH_GROUP")]
    SketchGroup,
    #[serde(rename = "FOLDER")]
    Folder,
    #[serde(rename = "NON_GEOMETRIC_ITEM")]
    NonGeometricItem,
    #[serde(rename = "TEMPORARY_GEOMETRY")]
    TemporaryGeometry,
    #[serde(rename = "PROPERTY")]
    Property,
    #[serde(rename = "SIMULATION_LOAD")]
    SimulationLoad,
    #[serde(rename = "PERSISTENT_QUERY_STRING")]
    PersistentQueryString,
    #[serde(rename = "GENERATIVE_DESIGN_ITEM")]
    GenerativeDesignItem,
    #[serde(rename = "ANNOTATION")]
    Annotation,
    #[serde(rename = "DIMENSION")]
    Dimension,
    #[serde(rename = "UNKNOWN")]
    Unknown,

}

impl std::fmt::Display for GbtuiSelectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Entity => write!(f, "ENTITY"),
            Self::Feature => write!(f, "FEATURE"),
            Self::Body => write!(f, "BODY"),
            Self::Occurrence => write!(f, "OCCURRENCE"),
            Self::Usercode => write!(f, "USERCODE"),
            Self::Rollbackbar => write!(f, "ROLLBACKBAR"),
            Self::Element => write!(f, "ELEMENT"),
            Self::Mate => write!(f, "MATE"),
            Self::MateConnector => write!(f, "MATE_CONNECTOR"),
            Self::EdgePoint => write!(f, "EDGE_POINT"),
            Self::MeshPoint => write!(f, "MESH_POINT"),
            Self::TableItem => write!(f, "TABLE_ITEM"),
            Self::SketchGroup => write!(f, "SKETCH_GROUP"),
            Self::Folder => write!(f, "FOLDER"),
            Self::NonGeometricItem => write!(f, "NON_GEOMETRIC_ITEM"),
            Self::TemporaryGeometry => write!(f, "TEMPORARY_GEOMETRY"),
            Self::Property => write!(f, "PROPERTY"),
            Self::SimulationLoad => write!(f, "SIMULATION_LOAD"),
            Self::PersistentQueryString => write!(f, "PERSISTENT_QUERY_STRING"),
            Self::GenerativeDesignItem => write!(f, "GENERATIVE_DESIGN_ITEM"),
            Self::Annotation => write!(f, "ANNOTATION"),
            Self::Dimension => write!(f, "DIMENSION"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for GbtuiSelectionType {
    fn default() -> GbtuiSelectionType {
        Self::Entity
    }
}

