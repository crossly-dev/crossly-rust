# \SandboxApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sandbox_reset**](SandboxApi.md#create_sandbox_reset) | **POST** /v1/sandbox/reset | Restore the sandbox seller to its seed. Your grant and webhook subscriptions are kept.
[**create_sandbox_simulate_inventory**](SandboxApi.md#create_sandbox_simulate_inventory) | **POST** /v1/sandbox/simulate/inventory | Change stock (set, increase or decrease); the new quantity is pushed to every channel.
[**create_sandbox_simulate_listing_sold_out**](SandboxApi.md#create_sandbox_simulate_listing_sold_out) | **POST** /v1/sandbox/simulate/listing.sold_out | A listing sells out on a marketplace with no order (stock moves, other channels delist).
[**create_sandbox_simulate_order**](SandboxApi.md#create_sandbox_simulate_order) | **POST** /v1/sandbox/simulate/order | Simulate a sale through the real sale pipeline (order.created, sale.detected, stock and cross-channel updates).
[**create_sandbox_simulate_order_address**](SandboxApi.md#create_sandbox_simulate_order_address) | **POST** /v1/sandbox/simulate/order.address | The buyer address of an order simulated with address \"delayed\" arrives now (order.address_available).
[**create_sandbox_simulate_order_cancelled**](SandboxApi.md#create_sandbox_simulate_order_cancelled) | **POST** /v1/sandbox/simulate/order.cancelled | The buyer or the marketplace cancels an order (order.cancelled).
[**create_sandbox_simulate_order_delivered**](SandboxApi.md#create_sandbox_simulate_order_delivered) | **POST** /v1/sandbox/simulate/order.delivered | The carrier delivers a shipped order (order.delivered).
[**create_sandbox_simulate_refund**](SandboxApi.md#create_sandbox_simulate_refund) | **POST** /v1/sandbox/simulate/refund | Refund an order in full or in part (refund.issued).
[**get_sandbox**](SandboxApi.md#get_sandbox) | **GET** /v1/sandbox | The sandbox seller this token acts on, its simulated channels and the failure modes.
[**list_sandbox_events**](SandboxApi.md#list_sandbox_events) | **GET** /v1/sandbox/events | The simulated marketplaces' call log: verb, channel, outcome, latency.



## create_sandbox_reset

> crate::models::CreateSandboxResetResponse create_sandbox_reset()
Restore the sandbox seller to its seed. Your grant and webhook subscriptions are kept.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSandboxResetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sandbox_simulate_inventory

> crate::models::CreateInboxConversationBulkResponse create_sandbox_simulate_inventory()
Change stock (set, increase or decrease); the new quantity is pushed to every channel.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInboxConversationBulkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sandbox_simulate_listing_sold_out

> crate::models::CreateSandboxSimulateListingSoldOutResponse create_sandbox_simulate_listing_sold_out()
A listing sells out on a marketplace with no order (stock moves, other channels delist).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSandboxSimulateListingSoldOutResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sandbox_simulate_order

> crate::models::CreateSandboxSimulateOrderResponse create_sandbox_simulate_order()
Simulate a sale through the real sale pipeline (order.created, sale.detected, stock and cross-channel updates).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSandboxSimulateOrderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sandbox_simulate_order_address

> crate::models::CreateSandboxSimulateOrderAddressResponse create_sandbox_simulate_order_address()
The buyer address of an order simulated with address \"delayed\" arrives now (order.address_available).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSandboxSimulateOrderAddressResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sandbox_simulate_order_cancelled

> crate::models::CreateSandboxSimulateOrderCancelledResponse create_sandbox_simulate_order_cancelled()
The buyer or the marketplace cancels an order (order.cancelled).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSandboxSimulateOrderCancelledResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sandbox_simulate_order_delivered

> crate::models::CreateSandboxSimulateOrderDeliveredResponse create_sandbox_simulate_order_delivered()
The carrier delivers a shipped order (order.delivered).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSandboxSimulateOrderDeliveredResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_sandbox_simulate_refund

> crate::models::CreateSandboxSimulateRefundResponse create_sandbox_simulate_refund()
Refund an order in full or in part (refund.issued).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateSandboxSimulateRefundResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_sandbox

> crate::models::GetSandboxResponse get_sandbox()
The sandbox seller this token acts on, its simulated channels and the failure modes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetSandboxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_sandbox_events

> crate::models::ListSandboxEventsResponse list_sandbox_events()
The simulated marketplaces' call log: verb, channel, outcome, latency.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListSandboxEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

