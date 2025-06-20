# Schema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_items** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**additional_properties** | Option<[**serde_json::Value**](.md)> |  | [optional]
**all_of** | Option<[**Vec<models::Schema>**](Schema.md)> |  | [optional]
**any_of** | Option<[**Vec<models::Schema>**](Schema.md)> |  | [optional]
**boolean_schema_value** | Option<**bool**> |  | [optional]
**r#const** | Option<[**serde_json::Value**](.md)> |  | [optional]
**contains** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**content_encoding** | Option<**String**> |  | [optional]
**content_media_type** | Option<**String**> |  | [optional]
**content_schema** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**default** | Option<[**serde_json::Value**](.md)> |  | [optional]
**dependent_required** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> |  | [optional]
**dependent_schemas** | Option<[**std::collections::HashMap<String, models::Schema>**](Schema.md)> |  | [optional]
**deprecated** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**discriminator** | Option<[**models::Discriminator**](Discriminator.md)> |  | [optional]
**r#else** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**r#enum** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**example** | Option<[**serde_json::Value**](.md)> |  | [optional]
**example_set_flag** | Option<**bool**> |  | [optional]
**examples** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**exclusive_maximum** | Option<**bool**> |  | [optional]
**exclusive_maximum_value** | Option<**f64**> |  | [optional]
**exclusive_minimum** | Option<**bool**> |  | [optional]
**exclusive_minimum_value** | Option<**f64**> |  | [optional]
**extensions** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**external_docs** | Option<[**models::ExternalDocumentation**](ExternalDocumentation.md)> |  | [optional]
**format** | Option<**String**> |  | [optional]
**get_dollar_anchor** | Option<**String**> |  | [optional]
**get_dollar_comment** | Option<**String**> |  | [optional]
**get_dollar_id** | Option<**String**> |  | [optional]
**get_dollar_ref** | Option<**String**> |  | [optional]
**get_dollar_schema** | Option<**String**> |  | [optional]
**r#if** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**items** | Option<[**models::SchemaObject**](SchemaObject.md)> |  | [optional]
**json_schema** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**json_schema_impl** | Option<[**serde_json::Value**](.md)> |  | [optional]
**max_contains** | Option<**i32**> |  | [optional]
**max_items** | Option<**i32**> |  | [optional]
**max_length** | Option<**i32**> |  | [optional]
**max_properties** | Option<**i32**> |  | [optional]
**maximum** | Option<**f64**> |  | [optional]
**min_contains** | Option<**i32**> |  | [optional]
**min_items** | Option<**i32**> |  | [optional]
**min_length** | Option<**i32**> |  | [optional]
**min_properties** | Option<**i32**> |  | [optional]
**minimum** | Option<**f64**> |  | [optional]
**multiple_of** | Option<**f64**> |  | [optional]
**not** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**nullable** | Option<**bool**> |  | [optional]
**one_of** | Option<[**Vec<models::Schema>**](Schema.md)> |  | [optional]
**pattern** | Option<**String**> |  | [optional]
**pattern_properties** | Option<[**std::collections::HashMap<String, models::Schema>**](Schema.md)> |  | [optional]
**prefix_items** | Option<[**Vec<models::Schema>**](Schema.md)> |  | [optional]
**properties** | Option<[**std::collections::HashMap<String, models::Schema>**](Schema.md)> |  | [optional]
**property_names** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**read_only** | Option<**bool**> |  | [optional]
**required** | Option<**Vec<String>**> |  | [optional]
**then** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**types** | Option<**Vec<String>**> |  | [optional]
**unevaluated_items** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**unevaluated_properties** | Option<[**models::Schema**](Schema.md)> |  | [optional]
**unique_items** | Option<**bool**> |  | [optional]
**write_only** | Option<**bool**> |  | [optional]
**xml** | Option<[**models::Xml**](XML.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


