# BtWebhookParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> |  | [optional]
**company_id** | Option<**String**> | Company admins can register webhooks to listen to all company events. | [optional]
**data** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**document_id** | Option<**String**> |  | [optional]
**element_id** | Option<**String**> |  | [optional]
**events** | Option<**Vec<String>**> | List of events for which webhook callback is invoked. | [optional]
**external_session_id** | Option<**String**> | Applications can pass this parameter as X-Session-ID with every REST call to distinguish webhooks triggered by self. | [optional]
**filter** | Option<**String**> |  | [optional]
**folder_id** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**is_transient** | Option<**bool**> | Transient webhooks are automatically cleaned up after a period of inactivity. | [optional][default to true]
**link_document_id** | Option<**String**> |  | [optional]
**options** | Option<[**models::BtWebhookOptions**](BTWebhookOptions.md)> |  | [optional]
**part_id** | Option<**String**> |  | [optional]
**project_id** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**version_id** | Option<**String**> |  | [optional]
**workspace_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


