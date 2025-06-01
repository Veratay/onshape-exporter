# BtWebhookInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company_id** | Option<**String**> | Company admins can register webhooks to listen to all company events. | [optional]
**created_by** | Option<[**models::BtUserSummaryInfo**](BTUserSummaryInfo.md)> |  | [optional]
**data** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**dropped_event_count** | Option<**i32**> |  | [optional]
**events** | Option<**Vec<String>**> | List of events for which webhook callback is invoked. | [optional]
**external_session_id** | Option<**String**> | Applications can pass this parameter as X-Session-ID with every REST call to distinguish webhooks triggered by self. | [optional]
**filter** | Option<**String**> |  | [optional]
**folder_id** | Option<**String**> |  | [optional]
**href** | Option<**String**> | URI to fetch complete information of the resource. | [optional]
**id** | Option<**String**> | Id of the resource. | [optional]
**is_transient** | Option<**bool**> | Transient webhooks are automatically cleaned up after a period of inactivity. | [optional][default to true]
**name** | Option<**String**> | Name of the resource. | [optional]
**options** | Option<[**models::BtWebhookOptions**](BTWebhookOptions.md)> |  | [optional]
**project_id** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**view_ref** | Option<**String**> | URI to visualize the resource in a webclient if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


