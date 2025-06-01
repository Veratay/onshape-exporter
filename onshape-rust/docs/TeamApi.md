# \TeamApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**find**](TeamApi.md#find) | **GET** /teams | Get a list of all teams the current user belongs to.
[**get_members**](TeamApi.md#get_members) | **GET** /teams/{tid}/members | Get a list of a team's members.
[**get_team**](TeamApi.md#get_team) | **GET** /teams/{tid} | Get team information by team ID.



## find

> models::BtGlobalTreeNodeListResponseBtTeamInfo find(query, filter, uid, company_id, offset, limit, sort_column, sort_order, include_company_owned_teams)
Get a list of all teams the current user belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |[default to ]
**filter** | Option<**i32**> |  |  |[default to 0]
**uid** | Option<**String**> |  |  |
**company_id** | Option<**String**> |  |  |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 100]
**sort_column** | Option<**String**> |  |  |
**sort_order** | Option<**String**> |  |  |[default to asc]
**include_company_owned_teams** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::BtGlobalTreeNodeListResponseBtTeamInfo**](BTGlobalTreeNodeListResponseBTTeamInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_members

> models::BtListResponseBtTeamMemberInfo get_members(tid, sort_column, sort_order, offset, limit, q)
Get a list of a team's members.

Returns a maximum of 20 per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |
**sort_column** | Option<**String**> |  |  |
**sort_order** | Option<**String**> |  |  |[default to asc]
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]
**q** | Option<**String**> |  |  |[default to ]

### Return type

[**models::BtListResponseBtTeamMemberInfo**](BTListResponseBTTeamMemberInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team

> models::BtTeamInfo get_team(tid)
Get team information by team ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |

### Return type

[**models::BtTeamInfo**](BTTeamInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

