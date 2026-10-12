# \MobileApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mobile_push_test**](MobileApi.md#create_mobile_push_test) | **POST** /v1/mobile/push-test | Fire a no-op test push to this user's devices.
[**create_mobile_push_token**](MobileApi.md#create_mobile_push_token) | **POST** /v1/mobile/push-token | Register an Expo push token for this user.
[**delete_mobile_push_token**](MobileApi.md#delete_mobile_push_token) | **DELETE** /v1/mobile/push-tokens | Clear ALL registered push tokens for this user.
[**list_mobile_push_tokens**](MobileApi.md#list_mobile_push_tokens) | **GET** /v1/mobile/push-tokens | List registered Expo push tokens (masked).



## create_mobile_push_test

> crate::models::OkResponse create_mobile_push_test()
Fire a no-op test push to this user's devices.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_mobile_push_token

> crate::models::OkResponse create_mobile_push_token()
Register an Expo push token for this user.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::OkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_mobile_push_token

> crate::models::DeleteMobilePushTokenResponse delete_mobile_push_token()
Clear ALL registered push tokens for this user.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::DeleteMobilePushTokenResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_mobile_push_tokens

> crate::models::ListMobilePushTokensResponse list_mobile_push_tokens()
List registered Expo push tokens (masked).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListMobilePushTokensResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

