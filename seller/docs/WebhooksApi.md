# \WebhooksApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhooksApi.md#create_webhook) | **POST** /v1/webhooks | Register a webhook endpoint (returns the signing secret once).
[**create_webhook_test**](WebhooksApi.md#create_webhook_test) | **POST** /v1/webhooks/{id}/test | Fire a synthetic test.ping delivery to one webhook.
[**delete_webhook**](WebhooksApi.md#delete_webhook) | **DELETE** /v1/webhooks/{id} | Delete a webhook endpoint.
[**get_webhook_stream**](WebhooksApi.md#get_webhook_stream) | **GET** /v1/webhooks/stream | Stream this account's webhook events as they happen (SSE).
[**list_webhooks**](WebhooksApi.md#list_webhooks) | **GET** /v1/webhooks | List your registered webhook endpoints.



## create_webhook

> crate::models::CreateWebhookResponse create_webhook()
Register a webhook endpoint (returns the signing secret once).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateWebhookResponse**](CreateWebhookResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhook_test

> crate::models::CreateWebhookTestResponse create_webhook_test(id)
Fire a synthetic test.ping delivery to one webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateWebhookTestResponse**](CreateWebhookTestResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> crate::models::DeleteWebhookResponse delete_webhook(id)
Delete a webhook endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteWebhookResponse**](DeleteWebhookResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_stream

> String get_webhook_stream()
Stream this account's webhook events as they happen (SSE).

A long-lived text/event-stream. Each event is one `data:` line of JSON: {eventName, eventId, createdAt, payload}. Used by `crossly dev` to forward live events to a local URL without exposing the machine to the internet. Does not replay history — you see what happens from the moment you connect.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/event-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_webhooks

> crate::models::V1List list_webhooks()
List your registered webhook endpoints.

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

