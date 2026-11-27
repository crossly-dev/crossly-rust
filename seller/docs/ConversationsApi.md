# \ConversationsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_conversation_message**](ConversationsApi.md#create_conversation_message) | **POST** /v1/conversations/{id}/messages | Reply to the buyer, on the marketplace the conversation is on (Crossly marketplace: straight into the thread). The message is in the thread at once with deliveryStatus 'sending'; it settles to 'sent' or 'failed'. reply_not_supported when the thread cannot take a reply (a no-reply sender, a platform with no reply wire).
[**list_conversation_messages**](ConversationsApi.md#list_conversation_messages) | **GET** /v1/conversations/{id}/messages | A conversation's messages, oldest first. sender is buyer | seller; a reply you sent shows deliveryStatus sending → sent | failed. since (ISO 8601): only messages sent at or after it.
[**list_conversations**](ConversationsApi.md#list_conversations) | **GET** /v1/conversations | List buyer conversations, newest activity first. orderId: the conversation(s) about that order (same platform, its listing, its buyer). listingId: about that listing. updatedSince (ISO 8601): only threads with activity at or after it, for polling. search: contains-match on the buyer username or the subject (case-insensitive). Pair with the message.received webhook.



## create_conversation_message

> crate::models::CreateConversationMessageResponse create_conversation_message(id, create_conversation_message_request)
Reply to the buyer, on the marketplace the conversation is on (Crossly marketplace: straight into the thread). The message is in the thread at once with deliveryStatus 'sending'; it settles to 'sent' or 'failed'. reply_not_supported when the thread cannot take a reply (a no-reply sender, a platform with no reply wire).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_conversation_message_request** | **CreateConversationMessageRequest** |  | [required] |

### Return type

**crate::models::CreateConversationMessageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_conversation_messages

> crate::models::ListConversationMessagesResponse list_conversation_messages(id, page, limit, since)
A conversation's messages, oldest first. sender is buyer | seller; a reply you sent shows deliveryStatus sending → sent | failed. since (ISO 8601): only messages sent at or after it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 50]
**since** | Option<**String**> |  |  |

### Return type

**crate::models::ListConversationMessagesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_conversations

> crate::models::ListConversationsResponse list_conversations(page, limit, order_id, listing_id, platform, updated_since, search)
List buyer conversations, newest activity first. orderId: the conversation(s) about that order (same platform, its listing, its buyer). listingId: about that listing. updatedSince (ISO 8601): only threads with activity at or after it, for polling. search: contains-match on the buyer username or the subject (case-insensitive). Pair with the message.received webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**order_id** | Option<**String**> |  |  |
**listing_id** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |
**updated_since** | Option<**String**> |  |  |
**search** | Option<**String**> | Contains-match (case-insensitive) on the buyer username or the thread subject. |  |

### Return type

**crate::models::ListConversationsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

