# \PayoutApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payout_estimate**](PayoutApi.md#get_payout_estimate) | **GET** /v1/payout/estimate | What one platform nets at a given price, after fees and shipping.
[**get_payout_gross_for_net**](PayoutApi.md#get_payout_gross_for_net) | **GET** /v1/payout/gross-for-net | The gross price needed to clear a target net on one platform.
[**list_payout_compare**](PayoutApi.md#list_payout_compare) | **GET** /v1/payout/compare | Rank platforms by what they net at a given price. Defaults to connected ones.



## get_payout_estimate

> crate::models::GetPayoutEstimateResponse get_payout_estimate()
What one platform nets at a given price, after fees and shipping.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetPayoutEstimateResponse**](GetPayoutEstimateResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payout_gross_for_net

> crate::models::GetPayoutGrossForNetResponse get_payout_gross_for_net()
The gross price needed to clear a target net on one platform.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetPayoutGrossForNetResponse**](GetPayoutGrossForNetResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payout_compare

> crate::models::V1List list_payout_compare()
Rank platforms by what they net at a given price. Defaults to connected ones.

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

