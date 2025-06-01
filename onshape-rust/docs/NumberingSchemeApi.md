# \NumberingSchemeApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**next_numbers**](NumberingSchemeApi.md#next_numbers) | **POST** /numberingscheme/nextnumbers | Send the items to generate numbers for, and return the next valid available part numbers.



## next_numbers

> std::collections::HashMap<String, Vec<models::BtNextPartNumber>> next_numbers(bt_next_part_numbers_param, cid, did)
Send the items to generate numbers for, and return the next valid available part numbers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_next_part_numbers_param** | [**BtNextPartNumbersParam**](BtNextPartNumbersParam.md) |  | [required] |
**cid** | Option<**String**> |  |  |
**did** | Option<**String**> |  |  |

### Return type

[**std::collections::HashMap<String, Vec<models::BtNextPartNumber>>**](Vec.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

