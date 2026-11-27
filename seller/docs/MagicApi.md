# \MagicApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_magic_auto_decision_undo**](MagicApi.md#create_magic_auto_decision_undo) | **POST** /v1/magic/auto/decisions/{decisionId}/undo | Undo an automated Magic List decision, deleting its unpublished draft.
[**create_magic_auto_item**](MagicApi.md#create_magic_auto_item) | **POST** /v1/magic/auto/items | Scan one item and match it automatically (drafts only, never publishes).
[**create_magic_auto_run**](MagicApi.md#create_magic_auto_run) | **POST** /v1/magic/auto/runs/{runId} | Score a Magic List run automatically; a confident match in auto mode becomes a draft.
[**create_magic_auto_run_swipe**](MagicApi.md#create_magic_auto_run_swipe) | **POST** /v1/magic/auto/runs/{runId}/swipes | Record a seller swipe session (kept and rejected match keys) for a Magic List run.
[**create_magic_scan**](MagicApi.md#create_magic_scan) | **POST** /v1/magic/scan | Run a Magic List image scan.
[**create_magic_scan_synthesize**](MagicApi.md#create_magic_scan_synthesize) | **POST** /v1/magic/scan/{runId}/synthesize | Synthesize a draft from confirmed matches.
[**get_magic_auto_setting**](MagicApi.md#get_magic_auto_setting) | **GET** /v1/magic/auto/settings | The seller’s Magic List automated-matching mode.
[**get_magic_draft**](MagicApi.md#get_magic_draft) | **GET** /v1/magic/drafts/{draftId} | Get a synthesized Magic List draft.
[**list_magic_auto_run_decisions**](MagicApi.md#list_magic_auto_run_decisions) | **GET** /v1/magic/auto/runs/{runId}/decisions | Every match decision round (automated and seller) for a Magic List run.
[**list_magic_recent**](MagicApi.md#list_magic_recent) | **GET** /v1/magic/recent | Recent Magic List scans for this seller.
[**update_magic_auto_setting**](MagicApi.md#update_magic_auto_setting) | **PUT** /v1/magic/auto/settings | Set the Magic List automated-matching mode (off | auto | review_all).



## create_magic_auto_decision_undo

> crate::models::CreateMagicAutoDecisionUndoResponse create_magic_auto_decision_undo(decision_id)
Undo an automated Magic List decision, deleting its unpublished draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**decision_id** | **String** |  | [required] |

### Return type

**crate::models::CreateMagicAutoDecisionUndoResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_magic_auto_item

> crate::models::CreateMagicAutoItemResponse create_magic_auto_item()
Scan one item and match it automatically (drafts only, never publishes).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateMagicAutoItemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_magic_auto_run

> crate::models::CreateMagicAutoRunResponse create_magic_auto_run(run_id)
Score a Magic List run automatically; a confident match in auto mode becomes a draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

**crate::models::CreateMagicAutoRunResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_magic_auto_run_swipe

> crate::models::CreateMagicAutoRunSwipeResponse create_magic_auto_run_swipe(run_id)
Record a seller swipe session (kept and rejected match keys) for a Magic List run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

**crate::models::CreateMagicAutoRunSwipeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_magic_scan

> crate::models::CreateMagicScanResponse create_magic_scan()
Run a Magic List image scan.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateMagicScanResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_magic_scan_synthesize

> crate::models::CreateMagicScanSynthesizeResponse create_magic_scan_synthesize(run_id)
Synthesize a draft from confirmed matches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

**crate::models::CreateMagicScanSynthesizeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_magic_auto_setting

> crate::models::GetMagicAutoSettingResponse get_magic_auto_setting()
The seller’s Magic List automated-matching mode.

mode: off (every match is swiped by hand), auto (confident matches become DRAFTS on their own, never published, always undoable) or review_all (Crossly pre-marks its picks; the seller confirms). Default review_all.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetMagicAutoSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_magic_draft

> crate::models::GetMagicDraftResponse get_magic_draft(draft_id)
Get a synthesized Magic List draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**draft_id** | **String** |  | [required] |

### Return type

**crate::models::GetMagicDraftResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_magic_auto_run_decisions

> crate::models::ListMagicAutoRunDecisionsResponse list_magic_auto_run_decisions(run_id)
Every match decision round (automated and seller) for a Magic List run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

**crate::models::ListMagicAutoRunDecisionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_magic_recent

> crate::models::ListMagicRecentResponse list_magic_recent()
Recent Magic List scans for this seller.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListMagicRecentResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_magic_auto_setting

> crate::models::GetMagicAutoSettingResponse update_magic_auto_setting(update_magic_auto_setting_request)
Set the Magic List automated-matching mode (off | auto | review_all).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_magic_auto_setting_request** | **UpdateMagicAutoSettingRequest** |  | [required] |

### Return type

**crate::models::GetMagicAutoSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

