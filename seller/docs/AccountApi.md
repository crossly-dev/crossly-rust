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
[**list_auth_sessions**](AccountApi.md#list_auth_sessions) | **GET** /v1/auth/sessions | List active browser auth sessions.
[**list_connected_apps**](AccountApi.md#list_connected_apps) | **GET** /v1/connected-apps | List third-party OAuth apps with access to this account.



## create_account_cancel_deletion

> crate::models::CreateAccountCancelDeletionResponse create_account_cancel_deletion()
Cancel a pending account deletion.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAccountCancelDeletionResponse**](CreateAccountCancelDeletionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account_logout_all

> crate::models::CreateAccountLogoutAllResponse create_account_logout_all()
Revoke every browser auth session for this user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAccountLogoutAllResponse**](CreateAccountLogoutAllResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_account_request_deletion

> crate::models::CreateAccountRequestDeletionResponse create_account_request_deletion()
Schedule account deletion after a grace period.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAccountRequestDeletionResponse**](CreateAccountRequestDeletionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_auth_session

> crate::models::DeleteAuthSessionResponse delete_auth_session()
Revoke all active browser sessions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeleteAuthSessionResponse**](DeleteAuthSessionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_auth_session_by_session_id

> crate::models::DeleteAuthSessionBySessionIdResponse delete_auth_session_by_session_id(session_id)
Revoke a single browser session by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteAuthSessionBySessionIdResponse**](DeleteAuthSessionBySessionIdResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connected_app

> crate::models::DeleteConnectedAppResponse delete_connected_app(grant_id)
Disconnect a third-party app. Its tokens stop working immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteConnectedAppResponse**](DeleteConnectedAppResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_deletion_status

> crate::models::GetAccountDeletionStatusResponse get_account_deletion_status()
Get the currently-pending deletion request, if any.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAccountDeletionStatusResponse**](GetAccountDeletionStatusResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_me

> crate::models::InlineResponse200 get_me()
Identity check — authenticated user + PAT scopes + account state.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_auth_sessions

> crate::models::V1List list_auth_sessions()
List active browser auth sessions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_connected_apps

> crate::models::V1List list_connected_apps()
List third-party OAuth apps with access to this account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

