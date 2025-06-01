# \AliasApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alias**](AliasApi.md#create_alias) | **POST** /aliases | Create an alias in your enterprise.
[**delete_alias**](AliasApi.md#delete_alias) | **DELETE** /aliases/{aid} | Delete an alias from your enterprise.
[**get_alias**](AliasApi.md#get_alias) | **GET** /aliases/{aid} | Get an alias by ID.
[**get_alias_members**](AliasApi.md#get_alias_members) | **GET** /aliases/{aid}/members | Get all users and teams assigned to an alias.
[**get_aliases_in_company**](AliasApi.md#get_aliases_in_company) | **GET** /aliases | Get a list of all aliases that exist for your enterprise.
[**update_alias**](AliasApi.md#update_alias) | **POST** /aliases/{aid} | Add, remove, replace, or rename entries in an alias list.



## create_alias

> models::BtAliasInfo create_alias(bt_alias_params)
Create an alias in your enterprise.

`Manage users and teams` global permission is required to call this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_alias_params** | [**BtAliasParams**](BtAliasParams.md) |  | [required] |

### Return type

[**models::BtAliasInfo**](BTAliasInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alias

> serde_json::Value delete_alias(aid)
Delete an alias from your enterprise.

`Manage users and teams` global permission is required to call this API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias

> models::BtAliasInfo get_alias(aid)
Get an alias by ID.

Get the information for an alias ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |

### Return type

[**models::BtAliasInfo**](BTAliasInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias_members

> models::BtListResponseBtAliasEntryInfo get_alias_members(aid, prefix, sort_column, sort_order, offset, limit)
Get all users and teams assigned to an alias.

This is a search-like endpoint that returns a subset of the member list. Use `getAlias` to return all members every time it's called.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |
**prefix** | Option<**String**> |  |  |[default to ]
**sort_column** | Option<**String**> |  |  |[default to name]
**sort_order** | Option<**String**> |  |  |[default to asc]
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::BtListResponseBtAliasEntryInfo**](BTListResponseBTAliasEntryInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aliases_in_company

> models::BtListResponseBtAliasInfo get_aliases_in_company(prefix, sort_column, sort_order, offset, limit)
Get a list of all aliases that exist for your enterprise.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | Option<**String**> |  |  |[default to ]
**sort_column** | Option<**String**> |  |  |[default to name]
**sort_order** | Option<**String**> |  |  |[default to asc]
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**models::BtListResponseBtAliasInfo**](BTListResponseBTAliasInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alias

> models::BtAliasInfo update_alias(aid, bt_alias_params)
Add, remove, replace, or rename entries in an alias list.

`Manage users and teams` global permission is required to call this API.  * Add new users in the `additions` array.  * Remove existing users in the `removals` array. Attempts to remove a user that does not exist in the Alias list will have no effect. * Replace the entire Alias list with the `entries` array.  * You can also update the alias' `name` and `description`.  For example, given an Alias with members userA and userB:  * `additions: [userC]` results in [userA, userB, userC]  * `removals: [userB]` results in [userA]  * `entries: [userC, user D]` results in [userC, userD]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |
**bt_alias_params** | [**BtAliasParams**](BtAliasParams.md) |  | [required] |

### Return type

[**models::BtAliasInfo**](BTAliasInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

