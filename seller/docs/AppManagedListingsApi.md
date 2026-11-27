# \AppManagedListingsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_listing_command_ack**](AppManagedListingsApi.md#create_listing_command_ack) | **POST** /v1/listing-commands/{id}/ack | Acknowledge a command (you will act on it). Extends its deadline: 24 h for listing commands; tracking 2 h, cancel 4 h, refund 24 h, message 1 h.
[**create_listing_command_complete**](AppManagedListingsApi.md#create_listing_command_complete) | **POST** /v1/listing-commands/{id}/complete | Report a command done on the marketplace. relist: newExternalId when a new id was issued; refund_order: platformRefundId + amount (required); submit_tracking: trackingNumber/carrier as the marketplace holds them; message_buyer: platformMessageId.
[**create_listing_command_fail**](AppManagedListingsApi.md#create_listing_command_fail) | **POST** /v1/listing-commands/{id}/fail | Report a command failed, with a reason the seller will see.
[**create_listing_event**](AppManagedListingsApi.md#create_listing_event) | **POST** /v1/listings/{id}/events | Report a marketplace event (item.updated / ended / sold / relisted) for a listing your app manages.
[**create_listing_event2**](AppManagedListingsApi.md#create_listing_event2) | **POST** /v1/listings/events | Report up to 100 marketplace events keyed by platform + externalId. One bad event never fails the rest.
[**create_listing_link**](AppManagedListingsApi.md#create_listing_link) | **POST** /v1/listings/link | Link a live marketplace listing your app already manages into Crossly (no re-listing). Idempotent.
[**create_listing_link_bulk**](AppManagedListingsApi.md#create_listing_link_bulk) | **POST** /v1/listings/link/bulk | Link up to 100 live marketplace listings. Each item succeeds or fails on its own; re-sending is safe.
[**create_listing_platform_reclone**](AppManagedListingsApi.md#create_listing_platform_reclone) | **POST** /v1/listings/{id}/platforms/{platform}/reclone | End this marketplace item and relist it under Crossly. Requires consent: true.
[**create_listing_platform_reclone_cancel**](AppManagedListingsApi.md#create_listing_platform_reclone_cancel) | **POST** /v1/listings/{id}/platforms/{platform}/reclone/cancel | Cancel a relist still waiting for you to end the old item.
[**create_listing_platform_reclone_seller_ended**](AppManagedListingsApi.md#create_listing_platform_reclone_seller_ended) | **POST** /v1/listings/{id}/platforms/{platform}/reclone/seller-ended | You ended the old item on the marketplace yourself (Done).
[**create_listing_platform_take_over**](AppManagedListingsApi.md#create_listing_platform_take_over) | **POST** /v1/listings/{id}/platforms/{platform}/take-over | Take over an app-managed item: Crossly acts on the marketplace itself (needs your own connection).
[**create_listing_unlink**](AppManagedListingsApi.md#create_listing_unlink) | **POST** /v1/listings/{id}/unlink | Stop the app managing this listing. It stays live everywhere; the app's Crossly Shop fee ends for future sales.
[**create_order_event**](AppManagedListingsApi.md#create_order_event) | **POST** /v1/orders/events | Report up to 100 order events (shipped, cancelled, refunded, delivered) keyed by platform + platformOrderId. Each completes the matching open order command. One bad event never fails the rest.
[**create_order_take_over**](AppManagedListingsApi.md#create_order_take_over) | **POST** /v1/orders/{id}/take-over | Take over the app-managed item behind an order; its actions then go through your own connection.
[**get_listing_management**](AppManagedListingsApi.md#get_listing_management) | **GET** /v1/listings/{id}/management | Which app manages this listing (null when none). Its fee is charged only on Crossly Shop sales, never on eBay or other marketplaces.
[**get_listing_platform_reclone**](AppManagedListingsApi.md#get_listing_platform_reclone) | **GET** /v1/listings/{id}/platforms/{platform}/reclone | The latest relist-under-Crossly run for this listing on a platform (null when none).
[**list_listing_commands**](AppManagedListingsApi.md#list_listing_commands) | **GET** /v1/listing-commands | Your app's listing (delist/update/relist) and order (submit_tracking, remove_tracking, cancel_order, refund_order, message_buyer) commands for this seller (default: open ones), oldest first.



## create_listing_command_ack

> crate::models::ListListingCommandsItem create_listing_command_ack(id)
Acknowledge a command (you will act on it). Extends its deadline: 24 h for listing commands; tracking 2 h, cancel 4 h, refund 24 h, message 1 h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListListingCommandsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_command_complete

> crate::models::ListListingCommandsItem create_listing_command_complete(id)
Report a command done on the marketplace. relist: newExternalId when a new id was issued; refund_order: platformRefundId + amount (required); submit_tracking: trackingNumber/carrier as the marketplace holds them; message_buyer: platformMessageId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListListingCommandsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_command_fail

> crate::models::ListListingCommandsItem create_listing_command_fail(id)
Report a command failed, with a reason the seller will see.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListListingCommandsItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_event

> crate::models::CreateListingEventResponse create_listing_event(id)
Report a marketplace event (item.updated / ended / sold / relisted) for a listing your app manages.

Idempotent on `eventId`. State events apply only when `sequence` is higher than the last applied one (else `stale`, ignored). `item.sold` runs Crossly's sale cascade (other channels delist) and is never dropped as stale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_event2

> crate::models::CreateListingEvent2Response create_listing_event2()
Report up to 100 marketplace events keyed by platform + externalId. One bad event never fails the rest.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingEvent2Response**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_link

> crate::models::CreateListingLinkResponse create_listing_link()
Link a live marketplace listing your app already manages into Crossly (no re-listing). Idempotent.

Direct when Crossly holds the seller's own connection (verified by item id: exists, active, theirs); relayed otherwise, on your attestation plus `snapshot`. The listing shows \"Managed by <your app>\". Your revenue share is charged only on sales made on Crossly Shop; sales on eBay or any other marketplace never carry an app fee.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingLinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_link_bulk

> crate::models::CreateListingLinkBulkResponse create_listing_link_bulk()
Link up to 100 live marketplace listings. Each item succeeds or fails on its own; re-sending is safe.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateListingLinkBulkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_platform_reclone

> crate::models::CreateListingPlatformRecloneResponse create_listing_platform_reclone(id, platform)
End this marketplace item and relist it under Crossly. Requires consent: true.

Loses the item's watchers, sold count, sales history, Promoted Listings and open offers; the item number changes. Refused while an order awaits fulfilment or an auction has bids. The new listing is published only after the old one is confirmed ended.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform** | **String** |  | [required] |

### Return type

**crate::models::CreateListingPlatformRecloneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_platform_reclone_cancel

> crate::models::CreateListingPlatformRecloneResponse create_listing_platform_reclone_cancel(id, platform)
Cancel a relist still waiting for you to end the old item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform** | **String** |  | [required] |

### Return type

**crate::models::CreateListingPlatformRecloneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_platform_reclone_seller_ended

> crate::models::CreateListingPlatformRecloneResponse create_listing_platform_reclone_seller_ended(id, platform)
You ended the old item on the marketplace yourself (Done).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform** | **String** |  | [required] |

### Return type

**crate::models::CreateListingPlatformRecloneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_platform_take_over

> crate::models::CreateListingPlatformTakeOverResponse create_listing_platform_take_over(id, platform)
Take over an app-managed item: Crossly acts on the marketplace itself (needs your own connection).

Verifies the item is on your connected account first. mode 'direct' (default) keeps the app as manager; 'full' also unlinks it (listing.unlinked reason 'taken_over'; its Crossly Shop fee stops). An app command already acknowledged answers 409 app_command_in_flight until you send confirm: true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform** | **String** |  | [required] |

### Return type

**crate::models::CreateListingPlatformTakeOverResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_listing_unlink

> crate::models::CreateListingUnlinkResponse create_listing_unlink(id)
Stop the app managing this listing. It stays live everywhere; the app's Crossly Shop fee ends for future sales.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateListingUnlinkResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_event

> crate::models::CreateOrderEventResponse create_order_event()
Report up to 100 order events (shipped, cancelled, refunded, delivered) keyed by platform + platformOrderId. Each completes the matching open order command. One bad event never fails the rest.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateOrderEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_order_take_over

> crate::models::CreateOrderTakeOverResponse create_order_take_over(id)
Take over the app-managed item behind an order; its actions then go through your own connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateOrderTakeOverResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing_management

> crate::models::GetListingManagementResponse get_listing_management(id)
Which app manages this listing (null when none). Its fee is charged only on Crossly Shop sales, never on eBay or other marketplaces.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetListingManagementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_listing_platform_reclone

> crate::models::GetListingPlatformRecloneResponse get_listing_platform_reclone(id, platform)
The latest relist-under-Crossly run for this listing on a platform (null when none).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**platform** | **String** |  | [required] |

### Return type

**crate::models::GetListingPlatformRecloneResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_listing_commands

> crate::models::ListListingCommandsResponse list_listing_commands(state, _type, limit, page)
Your app's listing (delist/update/relist) and order (submit_tracking, remove_tracking, cancel_order, refund_order, message_buyer) commands for this seller (default: open ones), oldest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state** | Option<**String**> | Comma-separated: pending, acked, completed, failed, expired, cancelled |  |
**_type** | Option<**String**> | Comma-separated: delist, update, relist, submit_tracking, remove_tracking, cancel_order, refund_order, message_buyer |  |
**limit** | Option<**i32**> |  |  |
**page** | Option<**i32**> |  |  |

### Return type

**crate::models::ListListingCommandsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

