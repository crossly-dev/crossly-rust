# \BillingApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_billing_upgrade**](BillingApi.md#create_billing_upgrade) | **POST** /v1/billing/upgrade | Start an upgrade to a higher plan.



## create_billing_upgrade

> crate::models::CreateBillingUpgradeResponse create_billing_upgrade(inline_object)
Start an upgrade to a higher plan.

Returns a Stripe Checkout URL. Nothing is charged by this call — a person completes the payment. Downgrades and cancellation are not available to a token at all; they stay with the account owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object** | [**InlineObject**](InlineObject.md) |  | [required] |

### Return type

[**crate::models::CreateBillingUpgradeResponse**](CreateBillingUpgradeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

