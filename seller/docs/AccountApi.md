# \AccountApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account_cancel_deletion**](AccountApi.md#create_account_cancel_deletion) | **POST** /v1/account/cancel-deletion | Cancel a pending account deletion.
[**create_account_logout_all**](AccountApi.md#create_account_logout_all) | **POST** /v1/account/logout-all | Revoke every browser auth session for this user.
[**create_account_request_deletion**](AccountApi.md#create_account_request_deletion) | **POST** /v1/account/request-deletion | Schedule account deletion after a grace period.
[**delete_auth_session**](AccountApi.md#delete_auth_session) | **DELETE** /v1/auth/sessions | Revoke all active browser sessions.
[**delete_auth_session_by_session_id**](AccountApi.md#delete_auth_session_by_session_id) | **DELETE** /v1/auth/sessions/{sessionId} | Revoke a single browser session by id.
[**delete_connected_app**](AccountApi.md#delete_connected_app) | **DELETE** /v1/connected-apps/{grantId} | Disconnect a third-party app. Its tokens stop working immediately.
[**get_account_deletion_status**](AccountApi.md#get_account_deletion_status) | **GET** /v1/account/deletion-status | Get the currently-pending deletion request, if any.
[**get_me**](AccountApi.md#get_me) | **GET** /v1/me | Identity check — authenticated user + PAT scopes + account state.
[**get_membership**](AccountApi.md#get_membership) | **GET** /v1/membership | The caller's Crossly One membership status and benefits.
[**list_auth_sessions**](AccountApi.md#list_auth_sessions) | **GET** /v1/auth/sessions | List active browser auth sessions.
[**list_connected_apps**](AccountApi.md#list_connected_apps) | **GET** /v1/connected-apps | List third-party OAuth apps with access to this account.



## create_account_cancel_deletion

> crate::models::DeleteDropResponse create_account_cancel_deletion()
Cancel a pending account deletion.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::DeleteDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_account_logout_all

> crate::models::CreateAccountLogoutAllResponse create_account_logout_all()
Revoke every browser auth session for this user.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAccountLogoutAllResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_account_request_deletion

> crate::models::CreateAccountRequestDeletionResponse create_account_request_deletion()
Schedule account deletion after a grace period.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAccountRequestDeletionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_auth_session

> crate::models::DeleteAuthSessionResponse delete_auth_session()
Revoke all active browser sessions.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::DeleteAuthSessionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_auth_session_by_session_id

> crate::models::SuccessResponse delete_auth_session_by_session_id(session_id)
Revoke a single browser session by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

**crate::models::SuccessResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_connected_app

> crate::models::OkResponse delete_connected_app(grant_id)
Disconnect a third-party app. Its tokens stop working immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_id** | **String** |  | [required] |

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_account_deletion_status

> crate::models::GetAccountDeletionStatusResponse get_account_deletion_status()
Get the currently-pending deletion request, if any.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAccountDeletionStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_me

> crate::models::GetMeResponse get_me()
Identity check — authenticated user + PAT scopes + account state.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetMeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_membership

> crate::models::GetMembershipResponse get_membership()
The caller's Crossly One membership status and benefits.

Returns the active tier (or null), the period end, and the concrete benefit values for that tier. Benefits are the single source of truth shared with the web and mobile clients.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetMembershipResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_auth_sessions

> crate::models::ListAuthSessionsResponse list_auth_sessions()
List active browser auth sessions.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListAuthSessionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_connected_apps

> crate::models::ListConnectedAppsResponse list_connected_apps()
List third-party OAuth apps with access to this account.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListConnectedAppsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

