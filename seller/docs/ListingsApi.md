# \ListingsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_listing**](ListingsApi.md#create_listing) | **POST** /v1/listings | Create a listing and fan out crosspost jobs across platforms.
[**create_listing_bulk_check_status**](ListingsApi.md#create_listing_bulk_check_status) | **POST** /v1/listings/bulk-check-status | Check listing status on platforms
[**create_listing_bulk_crosspost**](ListingsApi.md#create_listing_bulk_crosspost) | **POST** /v1/listings/bulk-crosspost | Bulk crosspost (no delist phase)
[**create_listing_bulk_delete**](ListingsApi.md#create_listing_bulk_delete) | **POST** /v1/listings/bulk-delete | Bulk archive + delist
[**create_listing_bulk_delist**](ListingsApi.md#create_listing_bulk_delist) | **POST** /v1/listings/bulk-delist | Bulk delist from platforms
[**create_listing_bulk_delist_preview**](ListingsApi.md#create_listing_bulk_delist_preview) | **POST** /v1/listings/bulk-delist-preview | Preview which marketplaces a delist would touch
[**create_listing_bulk_hard_delete**](ListingsApi.md#create_listing_bulk_hard_delete) | **POST** /v1/listings/bulk-hard-delete | Permanently delete archived listings
[**create_listing_bulk_relist**](ListingsApi.md#create_listing_bulk_relist) | **POST** /v1/listings/bulk-relist | Bulk relist across platforms
[**create_listing_bulk_update**](ListingsApi.md#create_listing_bulk_update) | **POST** /v1/listings/bulk-update | Bulk update listing fields
[**create_listing_by_id**](ListingsApi.md#create_listing_by_id) | **POST** /v1/listings/by-ids | Fetch hydrated listings by ID
[**create_listing_check_duplicate**](ListingsApi.md#create_listing_check_duplicate) | **POST** /v1/listings/check-duplicates | Check whether the seller already owns something matching this title/photo, and what to do about it.
[**create_listing_combine**](ListingsApi.md#create_listing_combine) | **POST** /v1/listings/combine | Combine duplicate listings into one: sums their stock, delists and archives the rest.
[**create_listing_discrepancy_resolve**](ListingsApi.md#create_listing_discrepancy_resolve) | **POST** /v1/listings/{id}/discrepancies/{discrepancyId}/resolve | Resolve a detected marketplace-drift discrepancy: accept the platform value, push ours back, relist to apply it, or dismiss.
[**create_listing_import_by_url**](ListingsApi.md#create_listing_import_by_url) | **POST** /v1/listings/{id}/import-by-url | Attach a real platform listing to this listing by pasting its live URL.
[**create_listing_magic_fill**](ListingsApi.md#create_listing_magic_fill) | **POST** /v1/listings/{id}/magic-fill | Auto-fill empty fields on one platform tab from the master listing + AI/deterministic taxonomy resolution.
[**delete_listing**](ListingsApi.md#delete_listing) | **DELETE** /v1/listings/{id} | Delist a listing (optionally narrowed to specific platforms via ?platforms=).
[**get_listing**](ListingsApi.md#get_listing) | **GET** /v1/listings/{id} | Get one listing with its platform rows.
[**get_listing_facet**](ListingsApi.md#get_listing_facet) | **GET** /v1/listings/facets | Distinct brands + categories across listings + inventory.
[**get_listing_sku_exist**](ListingsApi.md#get_listing_sku_exist) | **GET** /v1/listings/sku-exists | Check whether a SKU is already used by one of this user's items.
[**list_listing_discrepancies**](ListingsApi.md#list_listing_discrepancies) | **GET** /v1/listings/{id}/discrepancies | List detected marketplace-drift discrepancies for a listing.
[**list_listing_ids**](ListingsApi.md#list_listing_ids) | **GET** /v1/listings/ids | Filter listings → return matching id list (no pagination).
[**list_listings**](ListingsApi.md#list_listings) | **GET** /v1/listings | List active platform listings.
[**update_listing**](ListingsApi.md#update_listing) | **PATCH** /v1/listings/{id} | Edit a listing and fan out update jobs to existing platform listings.



## create_listing

> crate::models::CreateListingResponse create_listing()
Create a listing and fan out crosspost jobs across platforms.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingResponse**](CreateListingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_check_status

> crate::models::CreateListingBulkCheckStatusResponse create_listing_bulk_check_status()
Check listing status on platforms

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkCheckStatusResponse**](CreateListingBulkCheckStatusResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_crosspost

> crate::models::CreateListingBulkCrosspostResponse create_listing_bulk_crosspost()
Bulk crosspost (no delist phase)

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkCrosspostResponse**](CreateListingBulkCrosspostResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_delete

> crate::models::CreateListingBulkDeleteResponse create_listing_bulk_delete()
Bulk archive + delist

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkDeleteResponse**](CreateListingBulkDeleteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_delist

> crate::models::CreateListingBulkDelistResponse create_listing_bulk_delist()
Bulk delist from platforms

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkDelistResponse**](CreateListingBulkDelistResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_delist_preview

> crate::models::CreateListingBulkDelistPreviewResponse create_listing_bulk_delist_preview()
Preview which marketplaces a delist would touch

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkDelistPreviewResponse**](CreateListingBulkDelistPreviewResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_hard_delete

> crate::models::CreateListingBulkHardDeleteResponse create_listing_bulk_hard_delete()
Permanently delete archived listings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkHardDeleteResponse**](CreateListingBulkHardDeleteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_relist

> crate::models::CreateListingBulkRelistResponse create_listing_bulk_relist()
Bulk relist across platforms

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkRelistResponse**](CreateListingBulkRelistResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_bulk_update

> crate::models::CreateListingBulkUpdateResponse create_listing_bulk_update()
Bulk update listing fields

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingBulkUpdateResponse**](CreateListingBulkUpdateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_by_id

> crate::models::CreateListingByIdResponse create_listing_by_id()
Fetch hydrated listings by ID

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingByIdResponse**](CreateListingByIdResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_check_duplicate

> crate::models::CreateListingCheckDuplicateResponse create_listing_check_duplicate()
Check whether the seller already owns something matching this title/photo, and what to do about it.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingCheckDuplicateResponse**](CreateListingCheckDuplicateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_combine

> crate::models::CreateListingCombineResponse create_listing_combine()
Combine duplicate listings into one: sums their stock, delists and archives the rest.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateListingCombineResponse**](CreateListingCombineResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_discrepancy_resolve

> crate::models::CreateListingDiscrepancyResolveResponse create_listing_discrepancy_resolve(id, discrepancy_id)
Resolve a detected marketplace-drift discrepancy: accept the platform value, push ours back, relist to apply it, or dismiss.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**discrepancy_id** | **String** |  | [required] |

### Return type

[**crate::models::CreateListingDiscrepancyResolveResponse**](CreateListingDiscrepancyResolveResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_import_by_url

> crate::models::CreateListingImportByUrlResponse create_listing_import_by_url(id)
Attach a real platform listing to this listing by pasting its live URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateListingImportByUrlResponse**](CreateListingImportByUrlResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_listing_magic_fill

> crate::models::CreateListingMagicFillResponse create_listing_magic_fill(id)
Auto-fill empty fields on one platform tab from the master listing + AI/deterministic taxonomy resolution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateListingMagicFillResponse**](CreateListingMagicFillResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_listing

> crate::models::DeleteListingResponse delete_listing(id, platforms)
Delist a listing (optionally narrowed to specific platforms via ?platforms=).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platforms** | Option<**String**> | Comma-separated platform slugs to limit the delist fan-out. |  |

### Return type

[**crate::models::DeleteListingResponse**](DeleteListingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listing

> crate::models::GetListingResponse get_listing(id)
Get one listing with its platform rows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetListingResponse**](GetListingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listing_facet

> crate::models::GetListingFacetResponse get_listing_facet()
Distinct brands + categories across listings + inventory.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetListingFacetResponse**](GetListingFacetResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listing_sku_exist

> crate::models::GetListingSkuExistResponse get_listing_sku_exist(sku)
Check whether a SKU is already used by one of this user's items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sku** | **String** |  | [required] |

### Return type

[**crate::models::GetListingSkuExistResponse**](GetListingSkuExistResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_listing_discrepancies

> crate::models::V1List list_listing_discrepancies(id)
List detected marketplace-drift discrepancies for a listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_listing_ids

> crate::models::V1List list_listing_ids()
Filter listings → return matching id list (no pagination).

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


## list_listings

> crate::models::V1List list_listings(page, limit, platform, status)
List active platform listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**platform** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_listing

> crate::models::UpdateListingResponse update_listing(id)
Edit a listing and fan out update jobs to existing platform listings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateListingResponse**](UpdateListingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

