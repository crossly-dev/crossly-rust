# \CustomersApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_customer_bulk_delete**](CustomersApi.md#create_customer_bulk_delete) | **POST** /v1/customers/bulk-delete | Bulk blocklist customer handles.
[**create_customer_bulk_export**](CustomersApi.md#create_customer_bulk_export) | **POST** /v1/customers/bulk-export | Bulk export aggregated customers as CSV.
[**get_customer**](CustomersApi.md#get_customer) | **GET** /v1/customers/{handle} | Get one customer with their recent 50 orders.
[**list_customers**](CustomersApi.md#list_customers) | **GET** /v1/customers | List aggregated customers (group-by lower(buyer_username)).



## create_customer_bulk_delete

> crate::models::CreateCustomerBulkDeleteResponse create_customer_bulk_delete()
Bulk blocklist customer handles.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateCustomerBulkDeleteResponse**](CreateCustomerBulkDeleteResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_customer_bulk_export

> String create_customer_bulk_export()
Bulk export aggregated customers as CSV.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_customer

> crate::models::GetCustomerResponse get_customer(handle)
Get one customer with their recent 50 orders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

[**crate::models::GetCustomerResponse**](GetCustomerResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_customers

> crate::models::V1List list_customers()
List aggregated customers (group-by lower(buyer_username)).

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

