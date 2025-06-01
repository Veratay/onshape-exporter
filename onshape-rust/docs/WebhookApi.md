# \WebhookApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhookApi.md#create_webhook) | **POST** /webhooks | Create a new webhook.
[**get_webhook**](WebhookApi.md#get_webhook) | **GET** /webhooks/{webhookid} | Get webhook info by webhook ID.
[**get_webhooks**](WebhookApi.md#get_webhooks) | **GET** /webhooks | Get a list of all webhooks registered by a user or company.
[**ping_webhook**](WebhookApi.md#ping_webhook) | **POST** /webhooks/{webhookid}/ping | Ping a webhook.
[**unregister_webhook**](WebhookApi.md#unregister_webhook) | **DELETE** /webhooks/{webhookid} | Unregister a webhook.
[**update_webhook**](WebhookApi.md#update_webhook) | **POST** /webhooks/{webhookid} | Update a webhook.



## create_webhook

> models::BtWebhookInfo create_webhook(bt_webhook_params)
Create a new webhook.

Click **Callbacks** below for a list of events your app can subscribe to. See [API Guide: Webhooks](https://onshape-public.github.io/docs/app-dev/webhook/) for implementation details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_webhook_params** | Option<[**BtWebhookParams**](BtWebhookParams.md)> |  |  |

### Return type

[**models::BtWebhookInfo**](BTWebhookInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> models::BtWebhookInfo get_webhook(webhookid)
Get webhook info by webhook ID.

See [API Guide: Webhooks](https://onshape-public.github.io/docs/app-dev/webhook/) for implementation details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhookid** | **String** |  | [required] |

### Return type

[**models::BtWebhookInfo**](BTWebhookInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> models::BtListResponseBtWebhookInfo get_webhooks(company, user, offset, limit)
Get a list of all webhooks registered by a user or company.

See [API Guide: Webhooks](https://onshape-public.github.io/docs/app-dev/webhook/) for implementation details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**company** | Option<**String**> |  |  |[default to ]
**user** | Option<**String**> |  |  |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::BtListResponseBtWebhookInfo**](BTListResponseBTWebhookInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_webhook

> serde_json::Value ping_webhook(webhookid)
Ping a webhook.

See [API Guide: Webhooks](https://onshape-public.github.io/docs/app-dev/webhook/) for implementation details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhookid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unregister_webhook

> serde_json::Value unregister_webhook(webhookid, block_notification)
Unregister a webhook.

See [API Guide: Webhooks](https://onshape-public.github.io/docs/app-dev/webhook/) for implementation details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhookid** | **String** |  | [required] |
**block_notification** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> models::BtWebhookInfo update_webhook(webhookid, bt_webhook_params)
Update a webhook.

See [API Guide: Webhooks](https://onshape-public.github.io/docs/app-dev/webhook/) for implementation details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhookid** | **String** |  | [required] |
**bt_webhook_params** | Option<[**BtWebhookParams**](BtWebhookParams.md)> |  |  |

### Return type

[**models::BtWebhookInfo**](BTWebhookInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

