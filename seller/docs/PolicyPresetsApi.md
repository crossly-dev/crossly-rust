# \PolicyPresetsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_geo_location_check**](PolicyPresetsApi.md#create_geo_location_check) | **POST** /v1/geo/location-check | Check whether a shipping policy ships to an address.
[**create_policy_preset**](PolicyPresetsApi.md#create_policy_preset) | **POST** /v1/policy-presets | Create a return / shipping / payment policy preset.
[**delete_policy_preset**](PolicyPresetsApi.md#delete_policy_preset) | **DELETE** /v1/policy-presets/{id} | Delete a policy preset. Listings that used it switch to the default preset of its kind.
[**get_policy**](PolicyPresetsApi.md#get_policy) | **GET** /v1/policies | The seller's shipping / return / payment policy presets by kind, with each kind's default.
[**list_policy_presets**](PolicyPresetsApi.md#list_policy_presets) | **GET** /v1/policy-presets | List the seller's return / shipping / payment policy presets. meta.migrationNotices lists defaults the policy migration demoted (kind still has no default).
[**update_policy_preset**](PolicyPresetsApi.md#update_policy_preset) | **PATCH** /v1/policy-presets/{id} | Update a policy preset.



## create_geo_location_check

> crate::models::CreateGeoLocationCheckResponse create_geo_location_check()
Check whether a shipping policy ships to an address.

Pass an unsaved `policy` draft or a `presetId` (neither = your default shipping policy), an `address`, and optionally `consolidateVia` (a ship-together hub). Returns `ships`, and when it does not: `reason`, `stage` (buyer | hub), `message` and the deciding `rule`. `partial` means a fuller address (state, ZIP) could change the answer. A draft that is not a valid shipping policy is a 422; a presetId that is not yours is a 404.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateGeoLocationCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_policy_preset

> crate::models::CreatePolicyPresetResponse create_policy_preset()
Create a return / shipping / payment policy preset.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreatePolicyPresetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_policy_preset

> crate::models::DeletePolicyPresetResponse delete_policy_preset(id)
Delete a policy preset. Listings that used it switch to the default preset of its kind.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeletePolicyPresetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_policy

> crate::models::GetPolicyResponse get_policy()
The seller's shipping / return / payment policy presets by kind, with each kind's default.

Pass a preset `id` as `policies.shippingPresetId` / `returnPresetId` / `paymentPresetId` on POST or PATCH /v1/listings (null = the marketplace's own default). A kind you omit uses `defaultPresetId`.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPolicyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_policy_presets

> crate::models::ListPolicyPresetsResponse list_policy_presets(kind)
List the seller's return / shipping / payment policy presets. meta.migrationNotices lists defaults the policy migration demoted (kind still has no default).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kind** | Option<**String**> |  |  |

### Return type

**crate::models::ListPolicyPresetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_policy_preset

> crate::models::CreatePolicyPresetResponse update_policy_preset(id)
Update a policy preset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreatePolicyPresetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

