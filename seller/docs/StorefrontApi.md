# \StorefrontApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_storefront_shipping**](StorefrontApi.md#get_storefront_shipping) | **GET** /v1/storefront/shipping | Your Crossly shipping policy (domestic and international services, which countries you ship to) and your default ship-from address.
[**update_storefront_shipping**](StorefrontApi.md#update_storefront_shipping) | **PUT** /v1/storefront/shipping | Set your shipping policy (international.enabled, shipsTo worldwide or ISO codes, excludes) and/or your default ship-from address (any country; 422 for a sanctioned origin).



## get_storefront_shipping

> crate::models::GetStorefrontShippingResponse get_storefront_shipping()
Your Crossly shipping policy (domestic and international services, which countries you ship to) and your default ship-from address.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontShippingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_storefront_shipping

> crate::models::GetStorefrontShippingResponse update_storefront_shipping()
Set your shipping policy (international.enabled, shipsTo worldwide or ISO codes, excludes) and/or your default ship-from address (any country; 422 for a sanctioned origin).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetStorefrontShippingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

