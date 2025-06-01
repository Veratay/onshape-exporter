# BtMetadataPropertyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation_skipped_filtered_out_values** | Option<**bool**> |  | [optional]
**computed_assembly_property** | Option<**bool**> |  | [optional]
**computed_property** | Option<**bool**> |  | [optional]
**computed_property_error** | Option<**String**> |  | [optional]
**computed_property_eval_info** | Option<**String**> |  | [optional]
**date_format** | Option<**String**> |  | [optional]
**default_value** | Option<[**serde_json::Value**](.md)> |  | [optional]
**dirty** | Option<**bool**> |  | [optional]
**editable** | Option<**bool**> |  | [optional]
**editable_in_ui** | Option<**bool**> |  | [optional]
**enum_values** | Option<[**Vec<models::BtMetadataEnumValueInfo>**](BTMetadataEnumValueInfo.md)> |  | [optional]
**initial_value** | Option<[**serde_json::Value**](.md)> |  | [optional]
**multivalued** | Option<**bool**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**property_id** | Option<**String**> |  | [optional]
**property_override_status** | Option<**i32**> | 0: Unknown | 1: Not computed | 2: Computed without override | 3: Computed with override | 4: Computed with subassembly overrides | 5: Overridden | [optional]
**property_source** | Option<**i32**> |  | [optional]
**required** | Option<**bool**> |  | [optional]
**schema_id** | Option<**String**> |  | [optional]
**ui_hints** | Option<[**models::BtMetadataPropertyUiHintsInfo**](BTMetadataPropertyUiHintsInfo.md)> |  | [optional]
**validator** | Option<[**models::BtMetadataPropertyValidatorInfo**](BTMetadataPropertyValidatorInfo.md)> |  | [optional]
**value** | Option<[**serde_json::Value**](.md)> |  | [optional]
**value_type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


