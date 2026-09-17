# \SourcingApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sourcing_receipt**](SourcingApi.md#create_sourcing_receipt) | **POST** /v1/sourcing/receipts | Append a parsed receipt to the sourcing ledger.
[**get_sourcing_receipt**](SourcingApi.md#get_sourcing_receipt) | **GET** /v1/sourcing/receipts | List parsed sourcing receipts in this user's ledger.
[**list_sourcing_demand**](SourcingApi.md#list_sourcing_demand) | **GET** /v1/sourcing/demand | Items buyers looked for on other sites that Crossly did not have.
[**list_sourcing_demand_mine**](SourcingApi.md#list_sourcing_demand_mine) | **GET** /v1/sourcing/demand/mine | Unmet buyer demand for items you hold or have sold before.



## create_sourcing_receipt

> crate::models::CreateSourcingReceiptResponse create_sourcing_receipt()
Append a parsed receipt to the sourcing ledger.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateSourcingReceiptResponse**](CreateSourcingReceiptResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sourcing_receipt

> crate::models::GetSourcingReceiptResponse get_sourcing_receipt()
List parsed sourcing receipts in this user's ledger.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetSourcingReceiptResponse**](GetSourcingReceiptResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sourcing_demand

> crate::models::V1List list_sourcing_demand(days, min_looks, limit)
Items buyers looked for on other sites that Crossly did not have.

Aggregate demand observed by the Scout extension, ranked by MISSES — the times somebody asked and we had nothing. `medianPageCents` is what the retailers were charging, which is the number to source against. Anonymous in every case; there is no per-buyer view of this.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> |  |  |[default to 30]
**min_looks** | Option<**i32**> |  |  |[default to 3]
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sourcing_demand_mine

> crate::models::V1List list_sourcing_demand_mine(days, min_lookers, limit)
Unmet buyer demand for items you hold or have sold before.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> |  |  |[default to 60]
**min_lookers** | Option<**i32**> |  |  |[default to 2]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

