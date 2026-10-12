# \EmbedsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_embed_key**](EmbedsApi.md#create_embed_key) | **POST** /v1/embeds/keys | Mint a publishable key for a site.
[**delete_embed_key**](EmbedsApi.md#delete_embed_key) | **DELETE** /v1/embeds/keys/{id} | Revoke a publishable key.
[**list_embed_keys**](EmbedsApi.md#list_embed_keys) | **GET** /v1/embeds/keys | Your publishable keys.



## create_embed_key

> crate::models::CreateEmbedKeyResponse create_embed_key(create_embed_key_request)
Mint a publishable key for a site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_embed_key_request** | **CreateEmbedKeyRequest** |  | [required] |

### Return type

**crate::models::CreateEmbedKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_embed_key

> delete_embed_key(id)
Revoke a publishable key.

Takes effect immediately. Any site still using it will show an empty catalog, so replace the key in the page before revoking the old one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_embed_keys

> crate::models::ListEmbedKeysResponse list_embed_keys()
Your publishable keys.

Publishable keys are safe to put in a public web page — they can read your own active listings and start a checkout, and nothing else. The key is shown in full because it is not a secret.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListEmbedKeysResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

