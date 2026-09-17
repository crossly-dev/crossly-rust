# \InboxApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inbox_ai_suggest**](InboxApi.md#create_inbox_ai_suggest) | **POST** /v1/inbox/ai-suggest | AI reply suggestion for a conversation.
[**create_inbox_canned_respons**](InboxApi.md#create_inbox_canned_respons) | **POST** /v1/inbox/canned-responses | Create a canned response.
[**create_inbox_conversation_bulk**](InboxApi.md#create_inbox_conversation_bulk) | **POST** /v1/inbox/conversations/bulk | Bulk mark read / mark unread / soft-delete conversations.
[**create_inbox_conversation_bulk_ai_respond**](InboxApi.md#create_inbox_conversation_bulk_ai_respond) | **POST** /v1/inbox/conversations/bulk-ai-respond | AI reply suggestion for multiple conversations — draft or send.
[**create_inbox_conversation_offer_action**](InboxApi.md#create_inbox_conversation_offer_action) | **POST** /v1/inbox/conversations/{id}/offer-action | Accept / counter / decline an active offer on a conversation.
[**create_inbox_message_triage**](InboxApi.md#create_inbox_message_triage) | **POST** /v1/inbox/messages/{id}/triage | Manually re-triage a buyer message.
[**create_inbox_offer**](InboxApi.md#create_inbox_offer) | **POST** /v1/inbox/{id}/offer | Accept, counter, or decline an offer on a conversation.
[**create_inbox_reply**](InboxApi.md#create_inbox_reply) | **POST** /v1/inbox/{id}/reply | Send a reply to a conversation thread.
[**delete_inbox_canned_respons**](InboxApi.md#delete_inbox_canned_respons) | **DELETE** /v1/inbox/canned-responses/{id} | Delete a canned response.
[**get_inbox**](InboxApi.md#get_inbox) | **GET** /v1/inbox/{id} | Get one conversation with its messages.
[**get_inbox_canned_respons**](InboxApi.md#get_inbox_canned_respons) | **GET** /v1/inbox/canned-responses | List canned responses.
[**get_inbox_conversation_message**](InboxApi.md#get_inbox_conversation_message) | **GET** /v1/inbox/conversations/{id}/messages | Paginated messages for a conversation.
[**get_inbox_conversation_unread_count**](InboxApi.md#get_inbox_conversation_unread_count) | **GET** /v1/inbox/conversations/unread-count | Sidebar badge: unread conversation count.
[**list_inbox**](InboxApi.md#list_inbox) | **GET** /v1/inbox | List conversations.
[**update_inbox_canned_respons**](InboxApi.md#update_inbox_canned_respons) | **PUT** /v1/inbox/canned-responses/{id} | Update a canned response.
[**update_inbox_conversation**](InboxApi.md#update_inbox_conversation) | **PATCH** /v1/inbox/conversations/{id} | Mark read / change status / close conversation.



## create_inbox_ai_suggest

> crate::models::CreateInboxAiSuggestResponse create_inbox_ai_suggest()
AI reply suggestion for a conversation.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInboxAiSuggestResponse**](CreateInboxAiSuggestResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_canned_respons

> crate::models::CreateInboxCannedResponsResponse create_inbox_canned_respons()
Create a canned response.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInboxCannedResponsResponse**](CreateInboxCannedResponsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_conversation_bulk

> crate::models::CreateInboxConversationBulkResponse create_inbox_conversation_bulk()
Bulk mark read / mark unread / soft-delete conversations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInboxConversationBulkResponse**](CreateInboxConversationBulkResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_conversation_bulk_ai_respond

> crate::models::CreateInboxConversationBulkAiRespondResponse create_inbox_conversation_bulk_ai_respond()
AI reply suggestion for multiple conversations — draft or send.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CreateInboxConversationBulkAiRespondResponse**](CreateInboxConversationBulkAiRespondResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_conversation_offer_action

> crate::models::CreateInboxConversationOfferActionResponse create_inbox_conversation_offer_action(id)
Accept / counter / decline an active offer on a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateInboxConversationOfferActionResponse**](CreateInboxConversationOfferActionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_message_triage

> crate::models::CreateInboxMessageTriageResponse create_inbox_message_triage(id)
Manually re-triage a buyer message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateInboxMessageTriageResponse**](CreateInboxMessageTriageResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_offer

> crate::models::CreateInboxOfferResponse create_inbox_offer(id)
Accept, counter, or decline an offer on a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateInboxOfferResponse**](CreateInboxOfferResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_reply

> crate::models::CreateInboxReplyResponse create_inbox_reply(id)
Send a reply to a conversation thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::CreateInboxReplyResponse**](CreateInboxReplyResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_inbox_canned_respons

> crate::models::DeleteInboxCannedResponsResponse delete_inbox_canned_respons(id)
Delete a canned response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DeleteInboxCannedResponsResponse**](DeleteInboxCannedResponsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbox

> crate::models::GetInboxResponse get_inbox(id)
Get one conversation with its messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetInboxResponse**](GetInboxResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbox_canned_respons

> crate::models::GetInboxCannedResponsResponse get_inbox_canned_respons()
List canned responses.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetInboxCannedResponsResponse**](GetInboxCannedResponsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbox_conversation_message

> crate::models::GetInboxConversationMessageResponse get_inbox_conversation_message(id)
Paginated messages for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::GetInboxConversationMessageResponse**](GetInboxConversationMessageResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbox_conversation_unread_count

> crate::models::GetInboxConversationUnreadCountResponse get_inbox_conversation_unread_count()
Sidebar badge: unread conversation count.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetInboxConversationUnreadCountResponse**](GetInboxConversationUnreadCountResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_inbox

> crate::models::V1List list_inbox(page, limit)
List conversations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]

### Return type

[**crate::models::V1List**](V1List.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_inbox_canned_respons

> crate::models::UpdateInboxCannedResponsResponse update_inbox_canned_respons(id)
Update a canned response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateInboxCannedResponsResponse**](UpdateInboxCannedResponsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_inbox_conversation

> crate::models::UpdateInboxConversationResponse update_inbox_conversation(id)
Mark read / change status / close conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::UpdateInboxConversationResponse**](UpdateInboxConversationResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

