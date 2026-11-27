# \StallsApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stall_book**](StallsApi.md#create_stall_book) | **POST** /v1/stalls/book | Book and pay for a pop-up stall: { street, spotIndex, rail, unit: \"day\", startDate, days } or { ..., unit: \"event\", eventIds }, optional booth { canopy, pattern, lights, counter, piece } (premium values need holding; checked before paying). Paid up front; full-access token and Idempotency-Key required.
[**create_stall_booking_cancel**](StallsApi.md#create_stall_booking_cancel) | **POST** /v1/stalls/bookings/{id}/cancel | Cancel a stall before it opens: a full refund on the rail it was paid on. Final once it has opened.
[**get_stall_booking_counter**](StallsApi.md#get_stall_booking_counter) | **GET** /v1/stalls/bookings/{id}/counter | A stall counter: the listings you picked (in order), its booth kind and capacity.
[**get_stall_quote**](StallsApi.md#get_stall_quote) | **GET** /v1/stalls/quote | Quote a stall: street, spot, and either unit=day with start (YYYY-MM-DD) and days, or unit=event with events (comma-separated event ids). Returns the line items in cents, the rails the spot accepts and whether it is bookable.
[**get_stall_street_aerial**](StallsApi.md#get_stall_street_aerial) | **GET** /v1/stalls/streets/{slug}/aerial | A street from above for booking: every spot with its owner, booth size, counter capacity and price, the street events, and who is booked on each date.
[**list_stall_listing_search**](StallsApi.md#list_stall_listing_search) | **GET** /v1/stalls/listing-search | Search ANY of your live Crossly listings for a stall counter or convention booth: q, category, min, max (cents), street (marks what its terms allow), limit, offset.
[**list_stall_mine**](StallsApi.md#list_stall_mine) | **GET** /v1/stalls/mine | Your stall bookings, newest first, with visits, offers and sales while the stall was up.
[**list_street_events**](StallsApi.md#list_street_events) | **GET** /v1/streets/{slug}/events | Current and upcoming street events (Night Market, Swap Meet) on a street, soonest first.
[**list_street_stalls**](StallsApi.md#list_street_stalls) | **GET** /v1/streets/{slug}/stalls | Today's pop-up stalls on a street (each with its live listings, awning, sign and seller), plus the spots still free today. meta carries the street, the phase (day / night_market / swap_meet) and the free spots.
[**update_stall_booking**](StallsApi.md#update_stall_booking) | **PATCH** /v1/stalls/bookings/{id} | Change a booked stall before or while it is up: listingIds (up to 6 of your live listings), awning { colour, pattern }, booth { canopy, pattern, lights, counter, piece } (premium values need holding), sign, attendant.



## create_stall_book

> crate::models::CreateStallBookResponse create_stall_book()
Book and pay for a pop-up stall: { street, spotIndex, rail, unit: \"day\", startDate, days } or { ..., unit: \"event\", eventIds }, optional booth { canopy, pattern, lights, counter, piece } (premium values need holding; checked before paying). Paid up front; full-access token and Idempotency-Key required.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateStallBookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_stall_booking_cancel

> crate::models::CreateStallBookingCancelResponse create_stall_booking_cancel(id)
Cancel a stall before it opens: a full refund on the rail it was paid on. Final once it has opened.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateStallBookingCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_stall_booking_counter

> crate::models::GetStallBookingCounterResponse get_stall_booking_counter(id)
A stall counter: the listings you picked (in order), its booth kind and capacity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetStallBookingCounterResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_stall_quote

> crate::models::GetStallQuoteResponse get_stall_quote(street, spot, unit, start, days, events)
Quote a stall: street, spot, and either unit=day with start (YYYY-MM-DD) and days, or unit=event with events (comma-separated event ids). Returns the line items in cents, the rails the spot accepts and whether it is bookable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**street** | **String** |  | [required] |
**spot** | **i32** |  | [required] |
**unit** | Option<**String**> |  |  |
**start** | Option<**String**> | YYYY-MM-DD (unit=day). |  |
**days** | Option<**i32**> | How many days (unit=day). |  |
**events** | Option<**String**> | Comma-separated street event ids (unit=event). |  |

### Return type

**crate::models::GetStallQuoteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_stall_street_aerial

> crate::models::GetStallStreetAerialResponse get_stall_street_aerial(slug)
A street from above for booking: every spot with its owner, booth size, counter capacity and price, the street events, and who is booked on each date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::GetStallStreetAerialResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_stall_listing_search

> crate::models::ListStallListingSearchResponse list_stall_listing_search()
Search ANY of your live Crossly listings for a stall counter or convention booth: q, category, min, max (cents), street (marks what its terms allow), limit, offset.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStallListingSearchResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_stall_mine

> crate::models::ListStallMineResponse list_stall_mine()
Your stall bookings, newest first, with visits, offers and sales while the stall was up.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListStallMineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_street_events

> crate::models::ListStreetEventsResponse list_street_events(slug)
Current and upcoming street events (Night Market, Swap Meet) on a street, soonest first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListStreetEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_street_stalls

> crate::models::ListStreetStallsResponse list_street_stalls(slug)
Today's pop-up stalls on a street (each with its live listings, awning, sign and seller), plus the spots still free today. meta carries the street, the phase (day / night_market / swap_meet) and the free spots.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

**crate::models::ListStreetStallsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_stall_booking

> crate::models::CreateStallBookResponse update_stall_booking(id)
Change a booked stall before or while it is up: listingIds (up to 6 of your live listings), awning { colour, pattern }, booth { canopy, pattern, lights, counter, piece } (premium values need holding), sign, attendant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateStallBookResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

