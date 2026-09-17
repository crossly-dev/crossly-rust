# \AccountsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](AccountsApi.md#create_account) | **POST** /v1/accounts | Connect a new platform account (kicks off OAuth or extension handshake).
[**create_connection_email_imap**](AccountsApi.md#create_connection_email_imap) | **POST** /v1/connections/email/imap | Add an IMAP mailbox connection.
[**create_connection_email_imap_test**](AccountsApi.md#create_connection_email_imap_test) | **POST** /v1/connections/email/imap/test | Validate IMAP credentials without persisting.
[**create_connection_request**](AccountsApi.md#create_connection_request) | **POST** /v1/connections/{platform}/request | Express interest in a request_only platform.
[**create_platform_account_connect**](AccountsApi.md#create_platform_account_connect) | **POST** /v1/platform-accounts/{platform}/connect | Revive or initiate connection for a cookie platform.
[**create_platform_account_disconnect**](AccountsApi.md#create_platform_account_disconnect) | **POST** /v1/platform-accounts/{platform}/disconnect | Archive every active account row for a platform.
[**create_platform_account_history_import**](AccountsApi.md#create_platform_account_history_import) | **POST** /v1/platform-accounts/{platform}/history-import | Set how far back to backfill order history + active listings for a platform, and run it now.
[**create_platform_account_refresh_status**](AccountsApi.md#create_platform_account_refresh_status) | **POST** /v1/platform-accounts/refresh-status | Run on-demand healthchecks across cookie accounts.
[**delete_account**](AccountsApi.md#delete_account) | **DELETE** /v1/accounts/{id} | Disconnect a platform account.
[**delete_connection_by_id**](AccountsApi.md#delete_connection_by_id) | **DELETE** /v1/connections/by-id/{id} | Disconnect a specific OAuth connection by id.
[**delete_connection_email_imap**](AccountsApi.md#delete_connection_email_imap) | **DELETE** /v1/connections/email/imap/{id} | Remove an IMAP mailbox connection.
[**get_connection_email**](AccountsApi.md#get_connection_email) | **GET** /v1/connections/email | List IMAP and email-OAuth connections.
[**get_connection_extension_online**](AccountsApi.md#get_connection_extension_online) | **GET** /v1/connections/extension-online | Check if the browser extension is online.
[**get_oauth_init**](AccountsApi.md#get_oauth_init) | **GET** /v1/oauth/{platform}/init | Return the OAuth authorize URL for an API-track platform.
[**get_platform_limit**](AccountsApi.md#get_platform_limit) | **GET** /v1/platforms/limits | eBay free-tier + Etsy fees aggregate.
[**list_accounts**](AccountsApi.md#list_accounts) | **GET** /v1/accounts | List your connected platform accounts.
[**list_connections**](AccountsApi.md#list_connections) | **GET** /v1/connections | List OAuth-connected API platforms.
[**update_connection_email_imap**](AccountsApi.md#update_connection_email_imap) | **PATCH** /v1/connections/email/imap/{id} | Edit an IMAP mailbox connection.
[**update_platform_preference**](AccountsApi.md#update_platform_preference) | **PATCH** /v1/platforms/{platform}/preferences | Update per-platform connection preferences.



## create_account

> crate::models::CreateAccountResponse create_account()
Connect a new platform account (kicks off OAuth or extension handshake).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateAccountResponse**](CreateAccountResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connection_email_imap

> crate::models::CreateConnectionEmailImapResponse create_connection_email_imap()
Add an IMAP mailbox connection.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateConnectionEmailImapResponse**](CreateConnectionEmailImapResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connection_email_imap_test

> crate::models::CreateConnectionEmailImapTestResponse create_connection_email_imap_test()
Validate IMAP credentials without persisting.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateConnectionEmailImapTestResponse**](CreateConnectionEmailImapTestResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_connection_request

> crate::models::CreateConnectionRequestResponse create_connection_request(platform)
Express interest in a request_only platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

[**crate::models::CreateConnectionRequestResponse**](CreateConnectionRequestResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_platform_account_connect

> crate::models::CreatePlatformAccountConnectResponse create_platform_account_connect(platform)
Revive or initiate connection for a cookie platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

[**crate::models::CreatePlatformAccountConnectResponse**](CreatePlatformAccountConnectResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_platform_account_disconnect

> crate::models::CreatePlatformAccountDisconnectResponse create_platform_account_disconnect(platform)
Archive every active account row for a platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |

### Return type

[**crate::models::CreatePlatformAccountDisconnectResponse**](CreatePlatformAccountDisconnectResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_platform_account_history_import

> crate::models::CreatePlatformAccountHistoryImportResponse create_platform_account_history_import(platform, inline_object1)
Set how far back to backfill order history + active listings for a platform, and run it now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**inline_object1** | [**InlineObject1**](InlineObject1.md) |  | [required] |

### Return type

[**crate::models::CreatePlatformAccountHistoryImportResponse**](CreatePlatformAccountHistoryImportResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_platform_account_refresh_status

> crate::models::CreatePlatformAccountRefreshStatusResponse create_platform_account_refresh_status()
Run on-demand healthchecks across cookie accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreatePlatformAccountRefreshStatusResponse**](CreatePlatformAccountRefreshStatusResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account

> crate::models::DeleteAccountResponse delete_account(id)
Disconnect a platform account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteAccountResponse**](DeleteAccountResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connection_by_id

> crate::models::DeleteConnectionByIdResponse delete_connection_by_id(id)
Disconnect a specific OAuth connection by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteConnectionByIdResponse**](DeleteConnectionByIdResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connection_email_imap

> crate::models::DeleteConnectionEmailImapResponse delete_connection_email_imap(id)
Remove an IMAP mailbox connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteConnectionEmailImapResponse**](DeleteConnectionEmailImapResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_email

> crate::models::GetConnectionEmailResponse get_connection_email()
List IMAP and email-OAuth connections.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetConnectionEmailResponse**](GetConnectionEmailResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_extension_online

> crate::models::GetConnectionExtensionOnlineResponse get_connection_extension_online()
Check if the browser extension is online.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetConnectionExtensionOnlineResponse**](GetConnectionExtensionOnlineResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_oauth_init

> crate::models::GetOauthInitResponse get_oauth_init(platform, shop, region, site_url)
Return the OAuth authorize URL for an API-track platform.

Most platforms return `{ url }`. Etsy adds `{ correlationId }` (PKCE verifier stashed in Redis). Bonanza adds `{ oneShot: true }`. WooCommerce requires `?siteUrl=...`. Walmart returns `{ status: \"request_only\", requestUrl, message }` instead of a URL because per-seller OAuth needs Solution Provider approval.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**shop** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**site_url** | Option<**String**> |  |  |

### Return type

[**crate::models::GetOauthInitResponse**](GetOauthInitResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_platform_limit

> crate::models::GetPlatformLimitResponse get_platform_limit()
eBay free-tier + Etsy fees aggregate.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetPlatformLimitResponse**](GetPlatformLimitResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accounts

> crate::models::V1List list_accounts()
List your connected platform accounts.

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


## list_connections

> crate::models::V1List list_connections()
List OAuth-connected API platforms.

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


## update_connection_email_imap

> crate::models::UpdateConnectionEmailImapResponse update_connection_email_imap(id)
Edit an IMAP mailbox connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateConnectionEmailImapResponse**](UpdateConnectionEmailImapResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_platform_preference

> crate::models::UpdatePlatformPreferenceResponse update_platform_preference(platform, connection_id)
Update per-platform connection preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | **String** |  | [required] |
**connection_id** | Option<**String**> |  |  |

### Return type

[**crate::models::UpdatePlatformPreferenceResponse**](UpdatePlatformPreferenceResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

