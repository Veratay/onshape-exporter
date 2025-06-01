# BtAppElementUpdateParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**changes** | Option<[**Vec<models::BtAppElementChangeParams>**](BTAppElementChangeParams.md)> | Edits to be applied to the element's subelement data. | [optional]
**description** | Option<**String**> | The label that will appear in the document's edit history for this operation. If blank, a value will be auto-generated. | [optional]
**json_patch** | Option<**String**> | A json patch that will be applied to the application element's json data. | [optional]
**json_tree_edit** | Option<[**models::BtjEdit3734**](BTJEdit-3734.md)> |  | [optional]
**parent_change_id** | Option<**String**> | The id of the last change made to this application element. This can be retrieved from the response for any app element modification endpoint. | [optional]
**property_updates** | Option<[**Vec<models::BtMetadataPropertyUpdateParams>**](BTMetadataPropertyUpdateParams.md)> | Edits to be applied to the element's metadata. | [optional]
**return_error** | Option<**bool**> | If true, errors in request processing will be returned in a 200 response with a meaningful description. Otherwise, errors will result in a relevant HTTP error response. | [optional]
**return_json_difference_format** | Option<**String**> | If specified, and jsonTreeEdit is non-empty, the json difference will be returned in this format, otherwise no json difference will be returned. | [optional]
**transaction_id** | Option<**String**> | The id of the transaction in which this operation should take place. Transaction ids can be generated through the AppElement startTransaction API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


