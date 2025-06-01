# \ExportRuleApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_valid_rule_options**](ExportRuleApi.md#get_valid_rule_options) | **GET** /exportrules/options/{cu}/{cuid} |  Get a list of valid export rule options for the user or company.



## get_valid_rule_options

> models::BtExportRuleValidOptionsInfo get_valid_rule_options(cu, cuid)
 Get a list of valid export rule options for the user or company.

Does NOT get the rules themselves; it gets the information used to create them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cu** | **String** | Indicates which of company (c) or user (u) id is specified below. | [required] |
**cuid** | **String** | The id of the company or user in which the operation should be performed. | [required] |

### Return type

[**models::BtExportRuleValidOptionsInfo**](BTExportRuleValidOptionsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

