# \UserApi

All URIs are relative to *https://cad.onshape.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_settings**](UserApi.md#get_user_settings) | **GET** /users/{uid}/settings | Get the user settings for any user in your organization (admins only).
[**get_user_settings_current_logged_in_user**](UserApi.md#get_user_settings_current_logged_in_user) | **GET** /users/settings | Get the user settings for the signed-in user (i.e., you) for the current session.
[**session**](UserApi.md#session) | **POST** /users/session | Authenticate a user's Onshape credentials, and create a session.
[**session_info**](UserApi.md#session_info) | **GET** /users/sessioninfo | Get the session information for an authenticated (signed-in) user.



## get_user_settings

> models::BtUserSettingsInfo get_user_settings(uid, includematerials)
Get the user settings for any user in your organization (admins only).

* Mouse button settings are contained in `reverseScrollWheelZoomDirection` and `viewManipulationMouseKeyMapping`.  * For each action in `viewManipulationMouseKeyMapping`, an array of modifier key/mouse combos is provided that performs that action.  * Possible modifier keys include `SHIFT` and `CTRL`.  * Possible mouse buttons include `MMB` (middle mouse button), `RMB` (right mouse button), and `SCROLLWHEEL`.  * Scrolling forward zooms in, unless `reverseScrollWheelZoomDirection` is set to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uid** | **String** |  | [required] |
**includematerials** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::BtUserSettingsInfo**](BTUserSettingsInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings_current_logged_in_user

> models::BtUserSettingsInfo get_user_settings_current_logged_in_user(includematerials)
Get the user settings for the signed-in user (i.e., you) for the current session.

* Non-admins can call this API for their own user ID.  * Mouse button settings are contained in `reverseScrollWheelZoomDirection` and `viewManipulationMouseKeyMapping`.  * For each action in `viewManipulationMouseKeyMapping`, an array of modifier key/mouse combos is provided that performs that action.  * Possible modifier keys include `SHIFT` and `CTRL`.  * Possible mouse buttons include `MMB` (middle mouse button), `RMB` (right mouse button), and `SCROLLWHEEL`.  * Scrolling forward zooms in, unless `reverseScrollWheelZoomDirection` is set to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**includematerials** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::BtUserSettingsInfo**](BTUserSettingsInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session

> serde_json::Value session(bt_login_params)
Authenticate a user's Onshape credentials, and create a session.

Returned information depends on caller's `OAuth2ReadPll` scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_login_params** | [**BtLoginParams**](BtLoginParams.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_info

> models::BtUserOAuth2SummaryInfo session_info()
Get the session information for an authenticated (signed-in) user.

Returned information depends on caller's `OAuth2ReadPll` scope.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::BtUserOAuth2SummaryInfo**](BTUserOAuth2SummaryInfo.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

