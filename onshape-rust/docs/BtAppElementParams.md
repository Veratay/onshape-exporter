# BtAppElementParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The label that will appear in the document's edit history for this operation. If blank, a value will be auto-generated. | [optional]
**format_id** | **String** | The data type of the application. This string allows an application to distinguish their elements from elements of another application. | 
**json_tree** | Option<[**serde_json::Value**](.md)> | Initialization data for the new element's json tree. | [optional]
**location** | Option<[**models::BtElementLocationParams**](BTElementLocationParams.md)> |  | [optional]
**name** | Option<**String**> | The name of the element being created. If blank, a name will be auto-generated. | [optional]
**subelements** | Option<[**Vec<models::BtAppElementChangeParams>**](BTAppElementChangeParams.md)> | Initialization data for the new element's subelements. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


