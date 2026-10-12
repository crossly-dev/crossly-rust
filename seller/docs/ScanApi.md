# \ScanApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_scan_captures**](ScanApi.md#list_scan_captures) | **GET** /v1/scan/captures | Your capture sessions and what they produced.
[**list_scan_models**](ScanApi.md#list_scan_models) | **GET** /v1/scan/models | Which items already have a 3D model.



## list_scan_captures

> crate::models::ListScanCapturesResponse list_scan_captures(status, limit)
Your capture sessions and what they produced.

One row per scan, newest first. `status` is capturing | uploaded | processing | ready | failed | rejected; `rejectReason` says why in the same words the seller was shown. A capture that reports `ready` with no mesh is normal — most items are textured onto a known shape rather than having geometry generated, which is cheaper and looks the same. When an item that WOULD have been generated was not, `generationSkipped` says why: out of monthly allowance, too few distinct angles, or you had already had a model built from this object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Filter to one status. |  |
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

**crate::models::ListScanCapturesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_scan_models

> crate::models::ListScanModelsResponse list_scan_models(variant_ids, inventory_item_ids)
Which items already have a 3D model.

Ask by CATALOGUE variant (`variantIds`) or by your own inventory rows (`inventoryItemIds`), comma separated, up to 200 of each. At least one is required. Returns only the ones that have a model. The two are different KINDS of answer. A catalogue model belongs to the product and is shared: a hit means your listing gets it with nothing to capture, because somebody else already paid to generate it. An inventory model belongs to one of your rows, because an object with no catalogue identity — a lamp, a one-off — is not a product anyone else can claim. `longestEdgeMm` is present only when a capture MEASURED the item against something of known size; null means the mesh has correct proportions and no real-world scale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variant_ids** | Option<**String**> | Comma-separated catalogue variant ids, max 200. |  |
**inventory_item_ids** | Option<**String**> | Comma-separated inventory item ids, max 200. |  |

### Return type

**crate::models::ListScanModelsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

