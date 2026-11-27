# \BrandingApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_branding_drop**](BrandingApi.md#create_branding_drop) | **POST** /v1/branding/drops | Create a drop as a draft. `config` is the full drop config (what, where: shop|world|live|storefront|featured, format, access, limits, pricing, schedule, promotion, fulfilment). Returns rule problems to fix before publishing.
[**create_branding_drop_control**](BrandingApi.md#create_branding_drop_control) | **POST** /v1/branding/drops/{id}/controls/{control} | Live controls: pause, resume, extend (body endsAt), release-wave, end, draw (raffle).
[**create_branding_drop_publish**](BrandingApi.md#create_branding_drop_publish) | **POST** /v1/branding/drops/{id}/publish | Publish: scheduled, or pending_review for a brand partner without self-publish. Brand partners need a verified business on Stripe (403 business_verification_required).
[**create_branding_event_cancel**](BrandingApi.md#create_branding_event_cancel) | **POST** /v1/branding/events/{id}/cancel | Cancel a published event with the reason attendees see.
[**create_branding_event_check_in**](BrandingApi.md#create_branding_event_check_in) | **POST** /v1/branding/events/{id}/check-in | Check a ticket in at the door by its QR token.
[**create_branding_event_message**](BrandingApi.md#create_branding_event_message) | **POST** /v1/branding/events/{id}/messages | Message the event's attendees: all, going, or ticketed.
[**delete_branding_drop**](BrandingApi.md#delete_branding_drop) | **DELETE** /v1/branding/drops/{id} | Delete a draft, or cancel a scheduled drop. A live drop is ended with the end control.
[**get_branding_brand**](BrandingApi.md#get_branding_brand) | **GET** /v1/branding/brand | Your brand profile (approved brand partners).
[**get_branding_business_verification**](BrandingApi.md#get_branding_business_verification) | **GET** /v1/branding/business-verification | Whether your Stripe account is a verified business (company, EIN/tax id verified, payouts on), with reasons and the page to fix it.
[**get_branding_drop**](BrandingApi.md#get_branding_drop) | **GET** /v1/branding/drops/{id} | One of your drops.
[**get_branding_event**](BrandingApi.md#get_branding_event) | **GET** /v1/branding/events | Your events: upcoming grouped by date (today, this_week, later), or past.
[**get_branding_setting**](BrandingApi.md#get_branding_setting) | **GET** /v1/branding/settings | The store’s Branding defaults new drops start from: timezone, notifyFollowers, botQueue, botCaptcha, cartHoldMinutes, leftovers.
[**list_branding_drops**](BrandingApi.md#list_branding_drops) | **GET** /v1/branding/drops | Your drops, newest first, with config, waves, counts (entries, holding, sold, buyers) and phase. ?scope=upcoming|live|past|drafts|all.
[**update_branding_brand**](BrandingApi.md#update_branding_brand) | **PATCH** /v1/branding/brand | Edit your brand profile: logoUrl, bannerUrl, story, links.
[**update_branding_drop**](BrandingApi.md#update_branding_drop) | **PATCH** /v1/branding/drops/{id} | Edit a drop that is not live yet. A published drop refuses rule problems (400 drop_config_problems).
[**update_branding_event_setting**](BrandingApi.md#update_branding_event_setting) | **PUT** /v1/branding/events/{id}/settings | Save an event's full settings (where, access, reminders, check-in, recording, linked drops) and ticket tiers.
[**update_branding_setting**](BrandingApi.md#update_branding_setting) | **PATCH** /v1/branding/settings | Change the store’s Branding defaults (any subset of the fields).



## create_branding_drop

> crate::models::CreateBrandingDropResponse create_branding_drop()
Create a drop as a draft. `config` is the full drop config (what, where: shop|world|live|storefront|featured, format, access, limits, pricing, schedule, promotion, fulfilment). Returns rule problems to fix before publishing.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBrandingDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_branding_drop_control

> crate::models::GetBrandingDropResponse create_branding_drop_control(id, control)
Live controls: pause, resume, extend (body endsAt), release-wave, end, draw (raffle).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**control** | **String** |  | [required] |

### Return type

**crate::models::GetBrandingDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_branding_drop_publish

> crate::models::CreateBrandingDropPublishResponse create_branding_drop_publish(id)
Publish: scheduled, or pending_review for a brand partner without self-publish. Brand partners need a verified business on Stripe (403 business_verification_required).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBrandingDropPublishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_branding_event_cancel

> crate::models::CreateBrandingEventCancelResponse create_branding_event_cancel(id)
Cancel a published event with the reason attendees see.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBrandingEventCancelResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_branding_event_check_in

> crate::models::CreateBrandingEventCheckInResponse create_branding_event_check_in(id)
Check a ticket in at the door by its QR token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBrandingEventCheckInResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_branding_event_message

> crate::models::CreateBrandingEventMessageResponse create_branding_event_message(id)
Message the event's attendees: all, going, or ticketed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBrandingEventMessageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_branding_drop

> crate::models::DeleteBrandingDropResponse delete_branding_drop(id)
Delete a draft, or cancel a scheduled drop. A live drop is ended with the end control.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteBrandingDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_branding_brand

> crate::models::GetBrandingBrandResponse get_branding_brand()
Your brand profile (approved brand partners).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandingBrandResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_branding_business_verification

> crate::models::GetBrandingBusinessVerificationResponse get_branding_business_verification()
Whether your Stripe account is a verified business (company, EIN/tax id verified, payouts on), with reasons and the page to fix it.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandingBusinessVerificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_branding_drop

> crate::models::GetBrandingDropResponse get_branding_drop(id)
One of your drops.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBrandingDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_branding_event

> crate::models::GetBrandingEventResponse get_branding_event()
Your events: upcoming grouped by date (today, this_week, later), or past.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandingEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_branding_setting

> crate::models::GetBrandingSettingResponse get_branding_setting()
The store’s Branding defaults new drops start from: timezone, notifyFollowers, botQueue, botCaptcha, cartHoldMinutes, leftovers.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandingSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_branding_drops

> crate::models::ListBrandingDropsResponse list_branding_drops()
Your drops, newest first, with config, waves, counts (entries, holding, sold, buyers) and phase. ?scope=upcoming|live|past|drafts|all.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBrandingDropsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_branding_brand

> crate::models::GetBrandingBrandResponse update_branding_brand()
Edit your brand profile: logoUrl, bannerUrl, story, links.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandingBrandResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_branding_drop

> crate::models::CreateBrandingDropResponse update_branding_drop(id)
Edit a drop that is not live yet. A published drop refuses rule problems (400 drop_config_problems).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBrandingDropResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_branding_event_setting

> crate::models::UpdateBrandingEventSettingResponse update_branding_event_setting(id)
Save an event's full settings (where, access, reminders, check-in, recording, linked drops) and ticket tiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::UpdateBrandingEventSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_branding_setting

> crate::models::GetBrandingSettingResponse update_branding_setting()
Change the store’s Branding defaults (any subset of the fields).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetBrandingSettingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

