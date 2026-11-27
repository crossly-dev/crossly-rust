# \AccountApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account_cancel_deletion**](AccountApi.md#create_account_cancel_deletion) | **POST** /v1/account/cancel-deletion | Cancel a pending account deletion.
[**create_account_identity_verification_start**](AccountApi.md#create_account_identity_verification_start) | **POST** /v1/account/identity-verification/start | Start the identity check (photo ID + selfie). Returns a Stripe-hosted URL the person completes.
[**create_account_logout_all**](AccountApi.md#create_account_logout_all) | **POST** /v1/account/logout-all | Revoke every browser auth session for this user.
[**create_account_reconfirm_deletion**](AccountApi.md#create_account_reconfirm_deletion) | **POST** /v1/account/reconfirm-deletion | Confirm again a deletion paused because the account was in use.
[**create_account_request_deletion**](AccountApi.md#create_account_request_deletion) | **POST** /v1/account/request-deletion | Schedule account deletion after a grace period. Requires confirmEmail.
[**delete_auth_session**](AccountApi.md#delete_auth_session) | **DELETE** /v1/auth/sessions | Revoke all active browser sessions.
[**delete_auth_session_by_session_id**](AccountApi.md#delete_auth_session_by_session_id) | **DELETE** /v1/auth/sessions/{sessionId} | Revoke a single browser session by id.
[**delete_connected_app**](AccountApi.md#delete_connected_app) | **DELETE** /v1/connected-apps/{grantId} | Disconnect a third-party app. Its tokens stop working immediately.
[**get_account_closure_checklist**](AccountApi.md#get_account_closure_checklist) | **GET** /v1/account/closure-checklist | What closing the account will and will not do, before you request it.
[**get_account_deletion_status**](AccountApi.md#get_account_deletion_status) | **GET** /v1/account/deletion-status | Get the currently-pending deletion request, if any.
[**get_account_identity_verification**](AccountApi.md#get_account_identity_verification) | **GET** /v1/account/identity-verification | Your identity verification: one status for the whole account, read by every feature that needs ID.
[**get_me**](AccountApi.md#get_me) | **GET** /v1/me | Identity check — authenticated user + PAT scopes + account state.
[**get_membership**](AccountApi.md#get_membership) | **GET** /v1/membership | The caller's Crossly One membership status and benefits.
[**list_auth_sessions**](AccountApi.md#list_auth_sessions) | **GET** /v1/auth/sessions | List active browser auth sessions.
[**list_connected_apps**](AccountApi.md#list_connected_apps) | **GET** /v1/connected-apps | List third-party OAuth apps with access to this account.



## create_account_cancel_deletion

> crate::models::CreateAccountCancelDeletionResponse create_account_cancel_deletion()
Cancel a pending account deletion.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAccountCancelDeletionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_account_identity_verification_start

> crate::models::CreateWorldMatureVerifyResponse create_account_identity_verification_start(create_account_identity_verification_start_request)
Start the identity check (photo ID + selfie). Returns a Stripe-hosted URL the person completes.

alreadyVerified:true when there is nothing to do. Nothing is verified until a person completes the link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_account_identity_verification_start_request** | **CreateAccountIdentityVerificationStartRequest** |  | [required] |

### Return type

**crate::models::CreateWorldMatureVerifyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
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


## create_account_reconfirm_deletion

> crate::models::CreateAccountReconfirmDeletionResponse create_account_reconfirm_deletion()
Confirm again a deletion paused because the account was in use.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateAccountReconfirmDeletionResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_account_request_deletion

> crate::models::CreateAccountRequestDeletionResponse create_account_request_deletion()
Schedule account deletion after a grace period. Requires confirmEmail.

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


## get_account_closure_checklist

> crate::models::GetAccountClosureChecklistResponse get_account_closure_checklist()
What closing the account will and will not do, before you request it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAccountClosureChecklistResponse**

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


## get_account_identity_verification

> crate::models::GetAccountIdentityVerificationResponse get_account_identity_verification()
Your identity verification: one status for the whole account, read by every feature that needs ID.

status is unverified, requires_input, processing, verified or canceled. requires_input with a lastError { code, reason } is a failed try the person can redo on the same link (start returns it); processing is being reviewed; neither is a decline, only canceled is. attempts counts the failed tries. ageVerified passes the age-restricted checkout gate. message is the next step in words (null once verified).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetAccountIdentityVerificationResponse**

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

