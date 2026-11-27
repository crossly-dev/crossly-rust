# \CreatorApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_creator_experience_key**](CreatorApi.md#create_creator_experience_key) | **POST** /v1/creator/experience-keys | Issue an experience API key for a back-end or CI: name, creator scopes (worlds:*, items:write — only ones your token holds), an IP allowlist (CIDRs; 0.0.0.0/0 + ::/0 for anywhere), optional experienceIds and expiresInDays. The token is in the response ONCE.
[**create_creator_experience_key_rotate**](CreatorApi.md#create_creator_experience_key_rotate) | **POST** /v1/creator/experience-keys/{id}/rotate | Rotate a key: a replacement with the same settings (its token shown ONCE); the old key keeps working for overlapHours (default 24, 0 = revoke now, max 168).
[**delete_creator_experience_key**](CreatorApi.md#delete_creator_experience_key) | **DELETE** /v1/creator/experience-keys/{id} | Revoke an experience key now. Never needs a plan: a credential must always be revocable.
[**get_creator_entitlement**](CreatorApi.md#get_creator_entitlement) | **GET** /v1/creator/entitlements | What you can do in Creator Studio: your plan, whether you can author, grandfathering, and every limit with used and remaining. Read by web, apps, SDK and MCP alike.
[**list_creator_experience_keys**](CreatorApi.md#list_creator_experience_keys) | **GET** /v1/creator/experience-keys | Your experience API keys, newest first (never the secret). ?includeRevoked=true for the history; ?experienceId= for the keys that reach one game (a game's own keys are narrowed to it alone). Cursor-paged: pass pagination.nextCursor back as ?cursor=.



## create_creator_experience_key

> crate::models::CreateCreatorExperienceKeyResponse create_creator_experience_key()
Issue an experience API key for a back-end or CI: name, creator scopes (worlds:*, items:write — only ones your token holds), an IP allowlist (CIDRs; 0.0.0.0/0 + ::/0 for anywhere), optional experienceIds and expiresInDays. The token is in the response ONCE.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateCreatorExperienceKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_creator_experience_key_rotate

> crate::models::CreateCreatorExperienceKeyRotateResponse create_creator_experience_key_rotate(id)
Rotate a key: a replacement with the same settings (its token shown ONCE); the old key keeps working for overlapHours (default 24, 0 = revoke now, max 168).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateCreatorExperienceKeyRotateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_creator_experience_key

> crate::models::DeleteCreatorExperienceKeyResponse delete_creator_experience_key(id)
Revoke an experience key now. Never needs a plan: a credential must always be revocable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteCreatorExperienceKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_creator_entitlement

> crate::models::GetCreatorEntitlementResponse get_creator_entitlement()
What you can do in Creator Studio: your plan, whether you can author, grandfathering, and every limit with used and remaining. Read by web, apps, SDK and MCP alike.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetCreatorEntitlementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_creator_experience_keys

> crate::models::ListCreatorExperienceKeysResponse list_creator_experience_keys()
Your experience API keys, newest first (never the secret). ?includeRevoked=true for the history; ?experienceId= for the keys that reach one game (a game's own keys are narrowed to it alone). Cursor-paged: pass pagination.nextCursor back as ?cursor=.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListCreatorExperienceKeysResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

