# BtAppElementReferenceResolveInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_id** | Option<**String**> |  | [optional]
**error_code** | Option<**i32**> | `0: OK (healthy) | 1: INFO | 2: WARNING | 3: ERROR (dangling or view generation call failed) | 4: UNKNOWN` | [optional]
**error_description** | Option<**String**> | A human-readable value for the error that occurred, if one occurred. | [optional]
**error_value** | Option<[**models::BtAppElementErrorCode**](BTAppElementErrorCode.md)> |  | [optional]
**id_tag** | Option<**String**> |  | [optional]
**id_tag_is_valid** | Option<**bool**> |  | [optional]
**is_configurable** | Option<**bool**> |  | [optional]
**is_flattened_part** | Option<**bool**> |  | [optional]
**is_locked** | Option<**bool**> |  | [optional]
**is_sketch_only** | Option<**bool**> |  | [optional]
**is_surface** | Option<**bool**> |  | [optional]
**latest_element_microversion_id** | Option<**String**> |  | [optional]
**part_identity** | Option<**String**> |  | [optional]
**part_number** | Option<**String**> |  | [optional]
**reference_id** | Option<**String**> |  | [optional]
**reference_type** | Option<**i32**> |  | [optional]
**resolved_document_microversion_id** | Option<**String**> |  | [optional]
**resolved_element_microversion_id** | Option<**String**> |  | [optional]
**revision** | Option<**String**> |  | [optional]
**sketch_ids** | Option<**Vec<String>**> |  | [optional]
**source_element_id** | Option<**String**> |  | [optional]
**target_configuration** | Option<**String**> |  | [optional]
**target_document_id** | Option<**String**> |  | [optional]
**target_document_microversion_id** | Option<**String**> |  | [optional]
**target_element_id** | Option<**String**> |  | [optional]
**target_element_microversion_id** | Option<**String**> |  | [optional]
**target_version_id** | Option<**String**> | Reference to a part or assembly in a version; `NULL` when reference is in a workspace. | [optional]
**track_new_versions** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


