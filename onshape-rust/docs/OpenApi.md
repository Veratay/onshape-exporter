# OpenApi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**components** | Option<[**models::Components**](Components.md)> |  | [optional]
**extensions** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**external_docs** | Option<[**models::ExternalDocumentation**](ExternalDocumentation.md)> |  | [optional]
**info** | Option<[**models::Info**](Info.md)> |  | [optional]
**json_schema_dialect** | Option<**String**> |  | [optional]
**openapi** | Option<**String**> |  | [optional]
**paths** | Option<[**models::OpenApiPaths**](OpenAPI_paths.md)> |  | [optional]
**security** | Option<[**Vec<models::SecurityRequirement>**](SecurityRequirement.md)> |  | [optional]
**servers** | Option<[**Vec<models::Server>**](Server.md)> |  | [optional]
**tags** | Option<[**Vec<models::Tag>**](Tag.md)> |  | [optional]
**webhooks** | Option<[**std::collections::HashMap<String, models::PathItem>**](PathItem.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


