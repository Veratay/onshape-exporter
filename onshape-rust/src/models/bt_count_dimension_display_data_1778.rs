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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BtCountDimensionDisplayData1778 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    #[serde(rename = "coordinateSystem", skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<Box<models::BtMatrix3x3340>>,
    #[serde(rename = "featureId", skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<String>,
    #[serde(rename = "fitClass", skip_serializing_if = "Option::is_none")]
    pub fit_class: Option<String>,
    #[serde(rename = "hasMaximumLimit", skip_serializing_if = "Option::is_none")]
    pub has_maximum_limit: Option<bool>,
    #[serde(rename = "hasMinimumLimit", skip_serializing_if = "Option::is_none")]
    pub has_minimum_limit: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isAnnotationDimension", skip_serializing_if = "Option::is_none")]
    pub is_annotation_dimension: Option<bool>,
    #[serde(rename = "isAssociatedWithFlat", skip_serializing_if = "Option::is_none")]
    pub is_associated_with_flat: Option<bool>,
    #[serde(rename = "isDriven", skip_serializing_if = "Option::is_none")]
    pub is_driven: Option<bool>,
    #[serde(rename = "isOverDefined", skip_serializing_if = "Option::is_none")]
    pub is_over_defined: Option<bool>,
    #[serde(rename = "lowerTolerance", skip_serializing_if = "Option::is_none")]
    pub lower_tolerance: Option<f64>,
    #[serde(rename = "maximumLimit", skip_serializing_if = "Option::is_none")]
    pub maximum_limit: Option<f64>,
    #[serde(rename = "minimumLimit", skip_serializing_if = "Option::is_none")]
    pub minimum_limit: Option<f64>,
    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,
    #[serde(rename = "partId", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "planeMatrix", skip_serializing_if = "Option::is_none")]
    pub plane_matrix: Option<Box<models::BtbsMatrix386>>,
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<models::GbtTolerancePrecision>,
    #[serde(rename = "toleranceType", skip_serializing_if = "Option::is_none")]
    pub tolerance_type: Option<models::GbtToleranceType>,
    #[serde(rename = "upperTolerance", skip_serializing_if = "Option::is_none")]
    pub upper_tolerance: Option<f64>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "positionX", skip_serializing_if = "Option::is_none")]
    pub position_x: Option<f64>,
    #[serde(rename = "positionY", skip_serializing_if = "Option::is_none")]
    pub position_y: Option<f64>,
}

impl BtCountDimensionDisplayData1778 {
    pub fn new() -> BtCountDimensionDisplayData1778 {
        BtCountDimensionDisplayData1778 {
            bt_type: None,
            coordinate_system: None,
            feature_id: None,
            fit_class: None,
            has_maximum_limit: None,
            has_minimum_limit: None,
            id: None,
            is_annotation_dimension: None,
            is_associated_with_flat: None,
            is_driven: None,
            is_over_defined: None,
            lower_tolerance: None,
            maximum_limit: None,
            minimum_limit: None,
            parameter_id: None,
            part_id: None,
            plane_matrix: None,
            precision: None,
            tolerance_type: None,
            upper_tolerance: None,
            value: None,
            position_x: None,
            position_y: None,
        }
    }
}

