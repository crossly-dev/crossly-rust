# \SalesApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sale_bulk_delete**](SalesApi.md#create_sale_bulk_delete) | **POST** /v1/sales/bulk-delete | Bulk soft-delete sales rows.
[**list_sales**](SalesApi.md#list_sales) | **GET** /v1/sales | List sales (each unique sale event).



## create_sale_bulk_delete

> crate::models::CreateSaleBulkDeleteResponse create_sale_bulk_delete()
Bulk soft-delete sales rows.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSaleBulkDeleteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_sales

> crate::models::ListSalesResponse list_sales()
List sales (each unique sale event).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSalesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

