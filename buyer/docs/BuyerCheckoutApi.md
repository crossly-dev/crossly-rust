# \BuyerCheckoutApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_buyer_checkout**](BuyerCheckoutApi.md#create_buyer_checkout) | **POST** /v1/buyer/checkout | Buy a listing without being present.
[**get_buyer_checkout_control**](BuyerCheckoutApi.md#get_buyer_checkout_control) | **GET** /v1/buyer/checkout/controls | What this key is allowed to spend.
[**update_buyer_checkout_control**](BuyerCheckoutApi.md#update_buyer_checkout_control) | **PUT** /v1/buyer/checkout/controls | Switch this key on for spending, and set its limits.



## create_buyer_checkout

> crate::models::CreateBuyerCheckoutResponse create_buyer_checkout()
Buy a listing without being present.

An Idempotency-Key header is REQUIRED — this endpoint refuses without one, because a retried request would otherwise buy the item twice and a retry is the most likely thing an automated buyer does. Derive the key from what you are buying and reuse it across retries; a fresh random value per attempt satisfies the check and keeps the bug. The item is QUOTED first and the delivered total is checked against both your maxTotalCents and this key's limits before anything is charged. A card that demands 3-D Secure cannot be charged unattended; that answers 402 with `authentication_required` and the purchase must be finished on Crossly.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateBuyerCheckoutResponse**](CreateBuyerCheckoutResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_buyer_checkout_control

> crate::models::GetBuyerCheckoutControlResponse get_buyer_checkout_control()
What this key is allowed to spend.

Reports the controls for the key making the call — not for your account. Every key has its own switch and its own limits.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetBuyerCheckoutControlResponse**](GetBuyerCheckoutControlResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_buyer_checkout_control

> crate::models::UpdateBuyerCheckoutControlResponse update_buyer_checkout_control()
Switch this key on for spending, and set its limits.

A key can only ever raise or lower ITS OWN limits, and only if the token already carries buyer:checkout:write. Turning it off takes effect immediately.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UpdateBuyerCheckoutControlResponse**](UpdateBuyerCheckoutControlResponse.md)

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

