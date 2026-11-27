# \BillingApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_billing_upgrade**](BillingApi.md#create_billing_upgrade) | **POST** /v1/billing/upgrade | Start an upgrade to a higher plan.
[**list_billing_app_allowances**](BillingApi.md#list_billing_app_allowances) | **GET** /v1/billing/app-allowances | Your listing allowances through OAuth apps (used of limit, per app, with its platform scope).
[**list_billing_plans**](BillingApi.md#list_billing_plans) | **GET** /v1/billing/plans | List the plans with their prices, limits and features.



## create_billing_upgrade

> crate::models::CreateBillingUpgradeResponse create_billing_upgrade(create_billing_upgrade_request)
Start an upgrade to a higher plan.

Returns a Stripe Checkout URL. Nothing is charged by this call — a person completes the payment. Downgrades and cancellation are not available to a token at all; they stay with the account owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_billing_upgrade_request** | **CreateBillingUpgradeRequest** |  | [required] |

### Return type

**crate::models::CreateBillingUpgradeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_billing_app_allowances

> crate::models::ListBillingAppAllowancesResponse list_billing_app_allowances()
Your listing allowances through OAuth apps (used of limit, per app, with its platform scope).

Staff can let an OAuth app list above a seller's plan, counted apart from the plan's itemsPerMonth and only on the allowance's platforms. `label` is the sentence to show (12 of 10,000 managed by Infinity; ... this month via Infinity (resets Dec 1); or counts toward your plan for limitMode shared).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBillingAppAllowancesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_billing_plans

> crate::models::ListBillingPlansResponse list_billing_plans()
List the plans with their prices, limits and features.

Prices are the Stripe Prices checkout charges (whole USD). A null limit means no cap.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBillingPlansResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

