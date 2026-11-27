# \InboxApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inbox_ai_suggest**](InboxApi.md#create_inbox_ai_suggest) | **POST** /v1/inbox/ai-suggest | AI reply suggestion for a conversation.
[**create_inbox_canned_response**](InboxApi.md#create_inbox_canned_response) | **POST** /v1/inbox/canned-responses | Create a canned response (added to the end of the list).
[**create_inbox_conversation_bulk**](InboxApi.md#create_inbox_conversation_bulk) | **POST** /v1/inbox/conversations/bulk | Bulk mark read / mark unread / soft-delete conversations.
[**create_inbox_conversation_bulk_ai_respond**](InboxApi.md#create_inbox_conversation_bulk_ai_respond) | **POST** /v1/inbox/conversations/bulk-ai-respond | AI reply suggestion for multiple conversations — draft or send.
[**create_inbox_conversation_offer_action**](InboxApi.md#create_inbox_conversation_offer_action) | **POST** /v1/inbox/conversations/{id}/offer-action | Accept / counter / decline an active offer on a conversation.
[**create_inbox_message_triage**](InboxApi.md#create_inbox_message_triage) | **POST** /v1/inbox/messages/{id}/triage | Manually re-triage a buyer message.
[**create_inbox_offer**](InboxApi.md#create_inbox_offer) | **POST** /v1/inbox/{id}/offer | Accept, counter, or decline an offer on a conversation.
[**create_inbox_reply**](InboxApi.md#create_inbox_reply) | **POST** /v1/inbox/{id}/reply | Send a reply to a conversation thread.
[**create_inbox_send_offer**](InboxApi.md#create_inbox_send_offer) | **POST** /v1/inbox/{id}/send-offer | Make the buyer an offer inside a conversation (seller → buyer). Only where the conversation's canSendOffer is true (Grailed today). `amount` is in the listing's currency, above 0 and below the listing price (whole dollars on Grailed). The offer is in the thread at once as a message with offerAmount + offerStatus, deliveryStatus 'sending' until the platform confirms.
[**create_video_request_answer**](InboxApi.md#create_video_request_answer) | **POST** /v1/video-requests/{id}/answer | Answer a video request with a clip you uploaded (POST /v1/video-requests/upload-url, then PUT the file to uploadUrl). The buyer is notified and the clip shows on the listing.
[**create_video_request_decline**](InboxApi.md#create_video_request_decline) | **POST** /v1/video-requests/{id}/decline | Decline a video request, with an optional note to the buyer.
[**create_video_request_upload_url**](InboxApi.md#create_video_request_upload_url) | **POST** /v1/video-requests/upload-url | A presigned upload for a video answer: PUT the file to uploadUrl with the same Content-Type and exact Content-Length, then answer with publicUrl.
[**delete_inbox_canned_response**](InboxApi.md#delete_inbox_canned_response) | **DELETE** /v1/inbox/canned-responses/{id} | Delete a canned response.
[**get_inbox**](InboxApi.md#get_inbox) | **GET** /v1/inbox/{id} | Get one conversation with its messages and the item it is about (itemTitle, itemThumbnail, itemPrice, itemCurrency, itemUrl on the platform, listingId in Crossly, crosslyListingSlug, canSendOffer). An offer made in the thread is a message with offerAmount + offerStatus.
[**get_inbox_canned_response**](InboxApi.md#get_inbox_canned_response) | **GET** /v1/inbox/canned-responses | List canned responses in the seller's order.
[**get_inbox_conversation_message**](InboxApi.md#get_inbox_conversation_message) | **GET** /v1/inbox/conversations/{id}/messages | Paginated messages for a conversation.
[**get_inbox_conversation_unread_count**](InboxApi.md#get_inbox_conversation_unread_count) | **GET** /v1/inbox/conversations/unread-count | Unread conversation count (threads only; /v1/inbox/counts adds offers).
[**get_inbox_count**](InboxApi.md#get_inbox_count) | **GET** /v1/inbox/counts | Inbox badges: unread and needs-reply, split into conversations and offers.
[**list_inbox**](InboxApi.md#list_inbox) | **GET** /v1/inbox | List conversations. `filter` narrows to the rows GET /v1/inbox/counts counts.
[**list_video_requests**](InboxApi.md#list_video_requests) | **GET** /v1/video-requests | Buyers asking for a video of your items. status: open (default), answered, declined or all.
[**update_inbox_canned_response**](InboxApi.md#update_inbox_canned_response) | **PUT** /v1/inbox/canned-responses/{id} | Update a canned response.
[**update_inbox_canned_response_order**](InboxApi.md#update_inbox_canned_response_order) | **PUT** /v1/inbox/canned-responses/order | Reorder canned responses.
[**update_inbox_conversation**](InboxApi.md#update_inbox_conversation) | **PATCH** /v1/inbox/conversations/{id} | Mark read / change status / close conversation.



## create_inbox_ai_suggest

> crate::models::CreateInboxAiSuggestResponse create_inbox_ai_suggest()
AI reply suggestion for a conversation.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInboxAiSuggestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inbox_canned_response

> crate::models::CreateInboxCannedResponseResponse create_inbox_canned_response()
Create a canned response (added to the end of the list).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInboxCannedResponseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inbox_conversation_bulk

> crate::models::CreateInboxConversationBulkResponse create_inbox_conversation_bulk()
Bulk mark read / mark unread / soft-delete conversations.

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


## create_inbox_conversation_bulk_ai_respond

> crate::models::CreateInboxConversationBulkAiRespondResponse create_inbox_conversation_bulk_ai_respond()
AI reply suggestion for multiple conversations — draft or send.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateInboxConversationBulkAiRespondResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inbox_conversation_offer_action

> crate::models::UpdateInboxConversationResponse create_inbox_conversation_offer_action(id)
Accept / counter / decline an active offer on a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateInboxConversationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inbox_message_triage

> crate::models::CreateInboxMessageTriageResponse create_inbox_message_triage(id)
Manually re-triage a buyer message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInboxMessageTriageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inbox_offer

> crate::models::CreateInboxOfferResponse create_inbox_offer(id)
Accept, counter, or decline an offer on a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInboxOfferResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inbox_reply

> crate::models::CreateInboxReplyResponse create_inbox_reply(id)
Send a reply to a conversation thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInboxReplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_inbox_send_offer

> crate::models::CreateInboxSendOfferResponse create_inbox_send_offer(id, create_inbox_send_offer_request)
Make the buyer an offer inside a conversation (seller → buyer). Only where the conversation's canSendOffer is true (Grailed today). `amount` is in the listing's currency, above 0 and below the listing price (whole dollars on Grailed). The offer is in the thread at once as a message with offerAmount + offerStatus, deliveryStatus 'sending' until the platform confirms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_inbox_send_offer_request** | **CreateInboxSendOfferRequest** |  | [required] |

### Return type

**crate::models::CreateInboxSendOfferResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_video_request_answer

> crate::models::CreateVideoRequestAnswerResponse create_video_request_answer(id)
Answer a video request with a clip you uploaded (POST /v1/video-requests/upload-url, then PUT the file to uploadUrl). The buyer is notified and the clip shows on the listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVideoRequestAnswerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_video_request_decline

> crate::models::CreateVideoRequestAnswerResponse create_video_request_decline(id)
Decline a video request, with an optional note to the buyer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateVideoRequestAnswerResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_video_request_upload_url

> crate::models::CreateVideoRequestUploadUrlResponse create_video_request_upload_url()
A presigned upload for a video answer: PUT the file to uploadUrl with the same Content-Type and exact Content-Length, then answer with publicUrl.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateVideoRequestUploadUrlResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_inbox_canned_response

> delete_inbox_canned_response(id)
Delete a canned response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inbox

> crate::models::GetInboxResponse get_inbox(id)
Get one conversation with its messages and the item it is about (itemTitle, itemThumbnail, itemPrice, itemCurrency, itemUrl on the platform, listingId in Crossly, crosslyListingSlug, canSendOffer). An offer made in the thread is a message with offerAmount + offerStatus.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetInboxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inbox_canned_response

> crate::models::GetInboxCannedResponseResponse get_inbox_canned_response()
List canned responses in the seller's order.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetInboxCannedResponseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inbox_conversation_message

> crate::models::GetInboxConversationMessageResponse get_inbox_conversation_message(id)
Paginated messages for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetInboxConversationMessageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inbox_conversation_unread_count

> crate::models::CreateWeb3LaunchArtResponse get_inbox_conversation_unread_count()
Unread conversation count (threads only; /v1/inbox/counts adds offers).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateWeb3LaunchArtResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_inbox_count

> crate::models::GetInboxCountResponse get_inbox_count()
Inbox badges: unread and needs-reply, split into conversations and offers.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetInboxCountResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_inbox

> crate::models::ListInboxResponse list_inbox(page, limit, filter)
List conversations. `filter` narrows to the rows GET /v1/inbox/counts counts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**limit** | Option<**i32**> |  |  |[default to 25]
**filter** | Option<**String**> | unread | needs_reply (services/inbox/attention.ts). Omit for all. |  |

### Return type

**crate::models::ListInboxResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_video_requests

> crate::models::ListVideoRequestsResponse list_video_requests()
Buyers asking for a video of your items. status: open (default), answered, declined or all.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListVideoRequestsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_inbox_canned_response

> crate::models::CreateInboxCannedResponseResponse update_inbox_canned_response(id)
Update a canned response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateInboxCannedResponseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_inbox_canned_response_order

> crate::models::GetInboxCannedResponseResponse update_inbox_canned_response_order(update_inbox_canned_response_order_request)
Reorder canned responses.

Pass EVERY canned response id in the new order (first = top of the picker). A list with an unknown, missing or repeated id is refused with a validation error and nothing changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_inbox_canned_response_order_request** | **UpdateInboxCannedResponseOrderRequest** |  | [required] |

### Return type

**crate::models::GetInboxCannedResponseResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_inbox_conversation

> crate::models::UpdateInboxConversationResponse update_inbox_conversation(id)
Mark read / change status / close conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateInboxConversationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

