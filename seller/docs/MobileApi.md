# \MobileApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_mobile_push_test**](MobileApi.md#create_mobile_push_test) | **POST** /v1/mobile/push-test | Fire a no-op test push to this user's devices.
[**create_mobile_push_token**](MobileApi.md#create_mobile_push_token) | **POST** /v1/mobile/push-token | Register an Expo push token for this user.
[**delete_mobile_push_token**](MobileApi.md#delete_mobile_push_token) | **DELETE** /v1/mobile/push-tokens | Clear ALL registered push tokens for this user.
[**list_mobile_push_tokens**](MobileApi.md#list_mobile_push_tokens) | **GET** /v1/mobile/push-tokens | List registered Expo push tokens (masked).



## create_mobile_push_test

> crate::models::CreateMobilePushTestResponse create_mobile_push_test()
Fire a no-op test push to this user's devices.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateMobilePushTestResponse**](CreateMobilePushTestResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_mobile_push_token

> crate::models::CreateMobilePushTokenResponse create_mobile_push_token()
Register an Expo push token for this user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateMobilePushTokenResponse**](CreateMobilePushTokenResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_mobile_push_token

> crate::models::DeleteMobilePushTokenResponse delete_mobile_push_token()
Clear ALL registered push tokens for this user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeleteMobilePushTokenResponse**](DeleteMobilePushTokenResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_mobile_push_tokens

> crate::models::V1List list_mobile_push_tokens()
List registered Expo push tokens (masked).

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

