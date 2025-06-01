# \ApiApplicationApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_app_settings**](ApiApplicationApi.md#delete_app_settings) | **DELETE** /applications/clients/{cid}/settings/users/{uid} | Delete a user's application preference settings.
[**delete_company_app_settings**](ApiApplicationApi.md#delete_company_app_settings) | **DELETE** /applications/clients/{cid}/settings/companies/{cpid} | Delete a company's application preference settings.
[**get_applicable_extensions_for_client**](ApiApplicationApi.md#get_applicable_extensions_for_client) | **GET** /applications/extensions/user/{uid}/client/{cid} | Get a list of the client extensions the specified user has granted/accepted terms for.
[**get_company_app_settings**](ApiApplicationApi.md#get_company_app_settings) | **GET** /applications/clients/{cid}/settings/companies/{cpid} | Get company-level preference settings for an application.
[**get_user_app_settings**](ApiApplicationApi.md#get_user_app_settings) | **GET** /applications/clients/{cid}/settings/users/{uid} | Get user-level preference settings for an application.
[**update_app_company_settings**](ApiApplicationApi.md#update_app_company_settings) | **POST** /applications/clients/{cid}/settings/companies/{cpid} | Update company preference settings for an application.
[**update_app_settings**](ApiApplicationApi.md#update_app_settings) | **POST** /applications/clients/{cid}/settings/users/{uid} | Update a user's application preference settings.



## delete_app_settings

> delete_app_settings(uid, cid, key)
Delete a user's application preference settings.

This API is only usable with an OAuth token and only by the current user or admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**key** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_company_app_settings

> serde_json::Value delete_company_app_settings(cpid, cid, key)
Delete a company's application preference settings.

This API is only usable with an OAuth token and only by the current user or admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cpid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**key** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_applicable_extensions_for_client

> Vec<models::BtapiApplicationExtensionInfo> get_applicable_extensions_for_client(uid, cid, valid_purchases)
Get a list of the client extensions the specified user has granted/accepted terms for.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**valid_purchases** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::BtapiApplicationExtensionInfo>**](BTAPIApplicationExtensionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_app_settings

> models::BtUserAppSettingsInfo get_company_app_settings(cpid, cid, document_id, key)
Get company-level preference settings for an application.

This API is only usable with an OAuth token and only by the current user or admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cpid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**document_id** | Option<**String**> | A document owned by the company. Read access to this document allows read access to its owning company's settings. |  |
**key** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::BtUserAppSettingsInfo**](BTUserAppSettingsInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_app_settings

> models::BtUserAppSettingsInfo get_user_app_settings(uid, cid, key)
Get user-level preference settings for an application.

This API is only usable with an OAuth token and only by the current user or admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**key** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::BtUserAppSettingsInfo**](BTUserAppSettingsInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app_company_settings

> serde_json::Value update_app_company_settings(cpid, cid, bt_user_app_settings_params)
Update company preference settings for an application.

This API is only usable with an OAuth token and only by the current user or admin.  * Add or update a setting identified by key with value.  * Operation and field may optionally be specified when updating Map type settings.  * Field specifies the key of the setting Map to update.  * Operation may be one of:      * `ADD`: Add or update an existing field of the settings Map.      * `UPDATE`: Update an existing field of the settings Map and return an error if the field does not exist.      * `REMOVE`: Remove the field from the settings Map.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cpid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**bt_user_app_settings_params** | [**BtUserAppSettingsParams**](BtUserAppSettingsParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app_settings

> serde_json::Value update_app_settings(uid, cid, bt_user_app_settings_params)
Update a user's application preference settings.

This API is only usable with an OAuth token and only by the current user or admin.  * Add or update a setting identified by key with value.  * Operation and field may optionally be specified when updating Map type settings.  * Field specifies the key of the setting Map to update.  * Operation may be one of:      * `ADD`: Add or update an existing field of the settings Map.      * `UPDATE`: Update an existing field of the settings Map and return an error if the field does not exist.      * `REMOVE`: Remove the field from the settings Map.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**bt_user_app_settings_params** | [**BtUserAppSettingsParams**](BtUserAppSettingsParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

