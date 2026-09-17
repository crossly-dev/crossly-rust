# \CatalogApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_catalog_lookup**](CatalogApi.md#get_catalog_lookup) | **GET** /v1/catalog/lookup | Live Crossly offers for a product identifier (barcode, style code, LEGO set…).



## get_catalog_lookup

> crate::models::GetCatalogLookupResponse get_catalog_lookup(namespace, value)
Live Crossly offers for a product identifier (barcode, style code, LEGO set…).

Identifier-first: a GTIN is validated against its GS1 check digit and every length is normalised to 14 digits before lookup, so a UPC-A and its EAN-13 twin resolve to the same product. There is no fuzzy fallback — an identifier we cannot validate returns nothing rather than a guess.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**value** | **String** |  | [required] |

### Return type

[**crate::models::GetCatalogLookupResponse**](GetCatalogLookupResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

