# \CompanyApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user_to_company**](CompanyApi.md#add_user_to_company) | **POST** /companies/{cid}/users | Add a user to a company.
[**find_company**](CompanyApi.md#find_company) | **GET** /companies | Get all companies to which the specified user belongs.
[**get_company**](CompanyApi.md#get_company) | **GET** /companies/{cid} | Get company information by company ID.
[**get_documents_by_name**](CompanyApi.md#get_documents_by_name) | **GET** /companies/{cid}/documentsbyname | Get document by exact document name.
[**remove_user_from_company**](CompanyApi.md#remove_user_from_company) | **DELETE** /companies/{cid}/users/{uid} | Remove a user from a company, company teams, and all the direct shares.
[**update_company_user**](CompanyApi.md#update_company_user) | **POST** /companies/{cid}/users/{uid} | Update the company's information for a user.



## add_user_to_company

> models::BtCompanyUserInfo add_user_to_company(cid, bt_company_user_params)
Add a user to a company.

Returns the company user info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**bt_company_user_params** | [**BtCompanyUserParams**](BtCompanyUserParams.md) |  | [required] |

### Return type

[**models::BtCompanyUserInfo**](BTCompanyUserInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_company

> models::BtListResponseBtCompanyInfo find_company(uid, active_only, include_all)
Get all companies to which the specified user belongs.

If no user is specified, will return all companies associated with the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | Option<**String**> |  |  |
**active_only** | Option<**bool**> |  |  |[default to true]
**include_all** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::BtListResponseBtCompanyInfo**](BTListResponseBTCompanyInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company

> models::BtCompanyInfo get_company(cid)
Get company information by company ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |

### Return type

[**models::BtCompanyInfo**](BTCompanyInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_documents_by_name

> Vec<models::BtDocumentSummaryInfo> get_documents_by_name(cid, name)
Get document by exact document name.

This API can only be called by company admins. Use the `name` field for the exact document name string.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

[**Vec<models::BtDocumentSummaryInfo>**](BTDocumentSummaryInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_company

> serde_json::Value remove_user_from_company(cid, uid, remove_from_teams, remove_direct_shares)
Remove a user from a company, company teams, and all the direct shares.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**uid** | **String** |  | [required] |
**remove_from_teams** | Option<**bool**> |  |  |[default to true]
**remove_direct_shares** | Option<**bool**> |  |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_company_user

> models::BtCompanyUserInfo update_company_user(cid, uid, bt_company_user_params)
Update the company's information for a user.

Returns updated company user info. Global permissions can only be updated by the company admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cid** | **String** |  | [required] |
**uid** | **String** |  | [required] |
**bt_company_user_params** | [**BtCompanyUserParams**](BtCompanyUserParams.md) |  | [required] |

### Return type

[**models::BtCompanyUserInfo**](BTCompanyUserInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

