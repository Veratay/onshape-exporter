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
pub struct BtpExpressionFunction1325 {
    /// Type of JSON object.
    #[serde(rename = "btType", skip_serializing_if = "Option::is_none")]
    pub bt_type: Option<String>,
    #[serde(rename = "atomic", skip_serializing_if = "Option::is_none")]
    pub atomic: Option<bool>,
    #[serde(rename = "documentationType", skip_serializing_if = "Option::is_none")]
    pub documentation_type: Option<models::GbtpDefinitionType>,
    #[serde(rename = "endSourceLocation", skip_serializing_if = "Option::is_none")]
    pub end_source_location: Option<i32>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "shortDescriptor", skip_serializing_if = "Option::is_none")]
    pub short_descriptor: Option<String>,
    #[serde(rename = "spaceAfter", skip_serializing_if = "Option::is_none")]
    pub space_after: Option<Box<models::BtpSpace10>>,
    #[serde(rename = "spaceBefore", skip_serializing_if = "Option::is_none")]
    pub space_before: Option<Box<models::BtpSpace10>>,
    #[serde(rename = "spaceDefault", skip_serializing_if = "Option::is_none")]
    pub space_default: Option<bool>,
    #[serde(rename = "startSourceLocation", skip_serializing_if = "Option::is_none")]
    pub start_source_location: Option<i32>,
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<models::BtpArgumentDeclaration232>>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<models::BtpStatementBlock271>>,
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<Box<models::BtpExpression9>>,
    #[serde(rename = "isLambda", skip_serializing_if = "Option::is_none")]
    pub is_lambda: Option<bool>,
    #[serde(rename = "isLambdaWithNoParens", skip_serializing_if = "Option::is_none")]
    pub is_lambda_with_no_parens: Option<bool>,
    #[serde(rename = "precondition", skip_serializing_if = "Option::is_none")]
    pub precondition: Option<Box<models::BtpStatement269>>,
    #[serde(rename = "returnType", skip_serializing_if = "Option::is_none")]
    pub return_type: Option<Box<models::BtpTypeName290>>,
    #[serde(rename = "spaceAfterArglist", skip_serializing_if = "Option::is_none")]
    pub space_after_arglist: Option<Box<models::BtpSpace10>>,
    #[serde(rename = "spaceAfterFunction", skip_serializing_if = "Option::is_none")]
    pub space_after_function: Option<Box<models::BtpSpace10>>,
    #[serde(rename = "spaceInEmptyList", skip_serializing_if = "Option::is_none")]
    pub space_in_empty_list: Option<Box<models::BtpSpace10>>,
}

impl BtpExpressionFunction1325 {
    pub fn new() -> BtpExpressionFunction1325 {
        BtpExpressionFunction1325 {
            bt_type: None,
            atomic: None,
            documentation_type: None,
            end_source_location: None,
            node_id: None,
            short_descriptor: None,
            space_after: None,
            space_before: None,
            space_default: None,
            start_source_location: None,
            arguments: None,
            body: None,
            expression: None,
            is_lambda: None,
            is_lambda_with_no_parens: None,
            precondition: None,
            return_type: None,
            space_after_arglist: None,
            space_after_function: None,
            space_in_empty_list: None,
        }
    }
}

