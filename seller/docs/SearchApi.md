# \SearchApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search**](SearchApi.md#get_search) | **GET** /v1/search | Search everything in your business at once: inventory, listings, orders, sales, customers, conversations, offers, shipments and returns. Exact identifiers (SKU, barcode, order number, tracking number, buyer email, marketplace listing id or a pasted listing URL) rank first, then prefixes, then fuzzy title / brand / buyer matches. Operators: sku:, order:, buyer:, tracking:, in:<type>, status:, platform:. Groups your token has no read scope for are left out.



## get_search

> crate::models::GetSearchResponse get_search(q, types, limit)
Search everything in your business at once: inventory, listings, orders, sales, customers, conversations, offers, shipments and returns. Exact identifiers (SKU, barcode, order number, tracking number, buyer email, marketplace listing id or a pasted listing URL) rank first, then prefixes, then fuzzy title / brand / buyer matches. Operators: sku:, order:, buyer:, tracking:, in:<type>, status:, platform:. Groups your token has no read scope for are left out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> | What to search for (operators allowed). |  |
**types** | Option<**String**> | Comma-separated groups to search: inventory, listings, orders, sales, customers, conversations, offers, shipments, returns. |  |
**limit** | Option<**i32**> | Hits per group. |  |[default to 5]

### Return type

**crate::models::GetSearchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

