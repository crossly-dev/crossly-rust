# \IntegrationsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_notification_integration**](IntegrationsApi.md#create_notification_integration) | **POST** /v1/notification-integrations | Add a Slack/Discord/Webhook destination.
[**create_notification_integration_test**](IntegrationsApi.md#create_notification_integration_test) | **POST** /v1/notification-integrations/{id}/test | Fire a canned test message to a notification destination.
[**delete_notification_integration**](IntegrationsApi.md#delete_notification_integration) | **DELETE** /v1/notification-integrations/{id} | Delete a notification destination.
[**list_notification_integrations**](IntegrationsApi.md#list_notification_integrations) | **GET** /v1/notification-integrations | List Slack/Discord/Webhook destinations for notify.* automation actions.
[**update_notification_integration**](IntegrationsApi.md#update_notification_integration) | **PATCH** /v1/notification-integrations/{id} | Edit a notification destination.



## create_notification_integration

> crate::models::CreateNotificationIntegrationResponse create_notification_integration()
Add a Slack/Discord/Webhook destination.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateNotificationIntegrationResponse**](CreateNotificationIntegrationResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notification_integration_test

> crate::models::CreateNotificationIntegrationTestResponse create_notification_integration_test(id)
Fire a canned test message to a notification destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateNotificationIntegrationTestResponse**](CreateNotificationIntegrationTestResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_integration

> crate::models::DeleteNotificationIntegrationResponse delete_notification_integration(id)
Delete a notification destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteNotificationIntegrationResponse**](DeleteNotificationIntegrationResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_notification_integrations

> crate::models::V1List list_notification_integrations()
List Slack/Discord/Webhook destinations for notify.* automation actions.

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


## update_notification_integration

> crate::models::UpdateNotificationIntegrationResponse update_notification_integration(id)
Edit a notification destination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateNotificationIntegrationResponse**](UpdateNotificationIntegrationResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

