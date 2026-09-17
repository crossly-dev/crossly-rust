# \MagicApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_magic_scan**](MagicApi.md#create_magic_scan) | **POST** /v1/magic/scan | Run a Magic List image scan.
[**create_magic_scan_synthesize**](MagicApi.md#create_magic_scan_synthesize) | **POST** /v1/magic/scan/{runId}/synthesize | Synthesize a draft from confirmed matches.
[**get_magic_draft**](MagicApi.md#get_magic_draft) | **GET** /v1/magic/drafts/{draftId} | Get a synthesized Magic List draft.
[**list_magic_recent**](MagicApi.md#list_magic_recent) | **GET** /v1/magic/recent | Recent Magic List scans for this seller.



## create_magic_scan

> crate::models::CreateMagicScanResponse create_magic_scan()
Run a Magic List image scan.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateMagicScanResponse**](CreateMagicScanResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_magic_scan_synthesize

> crate::models::CreateMagicScanSynthesizeResponse create_magic_scan_synthesize(run_id)
Synthesize a draft from confirmed matches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateMagicScanSynthesizeResponse**](CreateMagicScanSynthesizeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_magic_draft

> crate::models::GetMagicDraftResponse get_magic_draft(draft_id)
Get a synthesized Magic List draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**draft_id** | **String** |  | [required] |

### Return type

[**crate::models::GetMagicDraftResponse**](GetMagicDraftResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_magic_recent

> crate::models::V1List list_magic_recent()
Recent Magic List scans for this seller.

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

