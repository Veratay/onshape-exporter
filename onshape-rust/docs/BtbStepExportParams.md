# BtbStepExportParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**advanced_params** | Option<[**models::BtbExportAdvancedParams**](BTBExportAdvancedParams.md)> |  | [optional]
**cloud_storage_options** | Option<[**models::BtbCloudStorageOptions**](BTBCloudStorageOptions.md)> |  | [optional]
**destination_name** | Option<**String**> | The name of the exported file. | [optional][default to Untitled]
**email_export_options** | Option<[**models::BtbEmailExportOptions**](BTBEmailExportOptions.md)> |  | [optional]
**exclude_hidden_entities** | Option<**bool**> | Whether or not to exclude hidden parts from export. | [optional][default to false]
**grouping** | Option<**bool**> | Whether parts should be exported as a group or individually in a .zip file. | [optional][default to true]
**include_export_ids** | Option<**bool**> | Whether topology ids should be exported as parasolid attributes. | [optional][default to false]
**is_y_axis_up** | Option<**bool**> | Rotate model from Z-axis-up orientation to Y-axis-up. | [optional][default to false]
**notify_user** | Option<**bool**> | Send notification to the user client. | [optional][default to true]
**step_parasolid_preprocessing_option** | [**models::GbtPreProcessParasolidOption**](GBTPreProcessParasolidOption.md) |  | 
**step_unit** | Option<**String**> | Units for the element: `METER` | `CENTIMETER` | `MILLIMETER` | `INCH` | `FOOT` | `YARD` | [optional][default to METER]
**step_version_string** | **String** | Export STEP in version: `AP242` | `AP203` | `AP214` | [default to AP242]
**store_in_document** | Option<**bool**> | Create a blob with exported file in the source document. | [optional][default to true]
**trigger_auto_download** | Option<**bool**> | Automatically download a translated file. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


