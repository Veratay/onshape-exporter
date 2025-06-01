# \AccountApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_purchase_new**](AccountApi.md#cancel_purchase_new) | **DELETE** /accounts/{aid}/purchases/{pid} | Cancel a recurring subscription for the specified account ID and purchase ID.
[**consume_purchase**](AccountApi.md#consume_purchase) | **POST** /accounts/purchases/{pid}/consume | Mark a purchase as consumed by the current user.
[**get_plan_purchases**](AccountApi.md#get_plan_purchases) | **GET** /accounts/plans/{planId}/purchases | Get a list of all app purchases made for the specified plan.
[**get_purchases**](AccountApi.md#get_purchases) | **GET** /accounts/purchases | Get a list of all app purchases made by the current user.



## cancel_purchase_new

> serde_json::Value cancel_purchase_new(aid, pid, cancel_immediately)
Cancel a recurring subscription for the specified account ID and purchase ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**cancel_immediately** | Option<**bool**> |  |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## consume_purchase

> models::BtPurchaseInfo consume_purchase(pid, bt_purchase_identity_params)
Mark a purchase as consumed by the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** |  | [required] |
**bt_purchase_identity_params** | Option<[**BtPurchaseIdentityParams**](BtPurchaseIdentityParams.md)> |  |  |

### Return type

[**models::BtPurchaseInfo**](BTPurchaseInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plan_purchases

> models::BtListResponseBtPurchaseInfo get_plan_purchases(plan_id, offset, limit)
Get a list of all app purchases made for the specified plan.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** |  | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::BtListResponseBtPurchaseInfo**](BTListResponseBTPurchaseInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purchases

> Vec<models::BtPurchaseInfo> get_purchases(all, own_purchase_only)
Get a list of all app purchases made by the current user.

This API should be used within the context of an OAuth-enabled application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> |  |  |[default to false]
**own_purchase_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::BtPurchaseInfo>**](BTPurchaseInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

