# \PayoutApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payout_connect_onboard**](PayoutApi.md#create_payout_connect_onboard) | **POST** /v1/payout/connect/onboard | Start or resume payout-account onboarding; returns a Stripe-hosted URL.
[**get_payout_connect**](PayoutApi.md#get_payout_connect) | **GET** /v1/payout/connect | Your Stripe payout account: status, country, agreement and payout currency.
[**get_payout_estimate**](PayoutApi.md#get_payout_estimate) | **GET** /v1/payout/estimate | What one platform nets at a given price, after fees and shipping.
[**get_payout_gross_for_net**](PayoutApi.md#get_payout_gross_for_net) | **GET** /v1/payout/gross-for-net | The gross price needed to clear a target net on one platform.
[**list_payout_compare**](PayoutApi.md#list_payout_compare) | **GET** /v1/payout/compare | Rank platforms by what they net at a given price. Defaults to connected ones.



## create_payout_connect_onboard

> crate::models::CreatePayoutConnectOnboardResponse create_payout_connect_onboard(create_payout_connect_onboard_request)
Start or resume payout-account onboarding; returns a Stripe-hosted URL.

country (ISO 3166-1 alpha-2) defaults to the storefront ship-from country, then US. Fixed once the account exists. Nothing is set up until a person completes the link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payout_connect_onboard_request** | **CreatePayoutConnectOnboardRequest** |  | [required] |

### Return type

**crate::models::CreatePayoutConnectOnboardResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_connect

> crate::models::GetPayoutConnectResponse get_payout_connect()
Your Stripe payout account: status, country, agreement and payout currency.

status is not_started (with defaultCountry), pending, active, restricted or payouts_unavailable. Only active can be paid; payouts_unavailable means Stripe offers no payout path to the account's country yet.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPayoutConnectResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_estimate

> crate::models::GetPayoutEstimateResponse get_payout_estimate()
What one platform nets at a given price, after fees and shipping.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPayoutEstimateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_payout_gross_for_net

> crate::models::GetPayoutGrossForNetResponse get_payout_gross_for_net()
The gross price needed to clear a target net on one platform.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetPayoutGrossForNetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_payout_compare

> crate::models::ListPayoutCompareResponse list_payout_compare()
Rank platforms by what they net at a given price. Defaults to connected ones.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListPayoutCompareResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

