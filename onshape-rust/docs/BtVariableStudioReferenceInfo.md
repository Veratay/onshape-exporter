# BtVariableStudioReferenceInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**configuration_id_to_value** | Option<[**std::collections::HashMap<String, models::BtOptionallyConfiguredValue>**](BTOptionallyConfiguredValue.md)> | Optional map of configuration parameter id to value | [optional]
**entire_variable_studio** | Option<**bool**> | Whether all variables in the referenced variable studio are included | [optional]
**reference_document_id** | Option<**String**> | DocumentId of referenced variable studio, blank for intra-workspace references | [optional]
**reference_element_id** | **String** | ElementId of referenced variable studio | 
**reference_version_id** | Option<**String**> | VersionId of referenced variable studio, blank for intra-workspace references | [optional]
**variable_names** | Option<**Vec<String>**> | Optional list of selected variables | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


