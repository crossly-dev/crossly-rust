# \PolicyPresetsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy_preset**](PolicyPresetsApi.md#create_policy_preset) | **POST** /v1/policy-presets | Create a return / shipping / payment policy preset.
[**delete_policy_preset**](PolicyPresetsApi.md#delete_policy_preset) | **DELETE** /v1/policy-presets/{id} | Delete a policy preset.
[**list_policy_presets**](PolicyPresetsApi.md#list_policy_presets) | **GET** /v1/policy-presets | List the seller's return / shipping / payment policy presets.
[**update_policy_preset**](PolicyPresetsApi.md#update_policy_preset) | **PATCH** /v1/policy-presets/{id} | Update a policy preset.



## create_policy_preset

> crate::models::CreatePolicyPresetResponse create_policy_preset()
Create a return / shipping / payment policy preset.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreatePolicyPresetResponse**](CreatePolicyPresetResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_policy_preset

> crate::models::DeletePolicyPresetResponse delete_policy_preset(id)
Delete a policy preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeletePolicyPresetResponse**](DeletePolicyPresetResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_policy_presets

> crate::models::V1List list_policy_presets(kind)
List the seller's return / shipping / payment policy presets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kind** | Option<**String**> |  |  |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_policy_preset

> crate::models::UpdatePolicyPresetResponse update_policy_preset(id)
Update a policy preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdatePolicyPresetResponse**](UpdatePolicyPresetResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

