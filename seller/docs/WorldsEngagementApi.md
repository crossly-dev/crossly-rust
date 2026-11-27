# \WorldsEngagementApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_experience_event**](WorldsEngagementApi.md#create_world_experience_experience_event) | **POST** /v1/worlds/experiences/{experienceId}/experience-events | Schedule an Experience Event shown on the game's card: title, description, start/end (5 min–14 days, ≤ 180 days ahead), thumbnail, place. With liveEventId it follows that live-ops event's schedule.
[**create_world_experience_experience_event_cancel**](WorldsEngagementApi.md#create_world_experience_experience_event_cancel) | **POST** /v1/worlds/experiences/{experienceId}/experience-events/{eventId}/cancel | Cancel an event (it disappears from the game's card; no reminder goes out).
[**create_world_experience_experience_event_rsvp**](WorldsEngagementApi.md#create_world_experience_experience_event_rsvp) | **POST** /v1/worlds/experiences/{experienceId}/experience-events/{eventId}/rsvp | RSVP (going: true) or take it back. remind: true also turns the experience's notifications on, so the 15-minute reminder can reach you.
[**create_world_experience_notification**](WorldsEngagementApi.md#create_world_experience_notification) | **POST** /v1/worlds/experiences/{experienceId}/notifications | Send an approved template to one player (sub) or up to 100 (subs). Only players who turned notifications on, 13+, not banned, at most 2 a day per player and outside their quiet hours receive it; every other send is logged as suppressed with its reason.
[**create_world_experience_notification_template_retire**](WorldsEngagementApi.md#create_world_experience_notification_template_retire) | **POST** /v1/worlds/experiences/{experienceId}/notification-templates/{key}/retire | Retire a template: sends by its key stop.
[**get_world_experience_notification_opt_in**](WorldsEngagementApi.md#get_world_experience_notification_opt_in) | **GET** /v1/worlds/experiences/{experienceId}/notifications/opt-in | Whether you turned this experience's notifications on.
[**get_world_experience_notification_report**](WorldsEngagementApi.md#get_world_experience_notification_report) | **GET** /v1/worlds/experiences/{experienceId}/notifications/report | Sends over the last N days (default 7, max 90): delivered, suppressed by reason, per template, and how many players turned notifications on.
[**list_world_experience_event_upcoming**](WorldsEngagementApi.md#list_world_experience_event_upcoming) | **GET** /v1/worlds/experience-events/upcoming | Events across public experiences in the next 7 days (and live now), soonest first — the Carnival Plaza's \"Coming up\" board.
[**list_world_experience_experience_event_upcoming**](WorldsEngagementApi.md#list_world_experience_experience_event_upcoming) | **GET** /v1/worlds/experiences/{experienceId}/experience-events/upcoming | An experience's upcoming and live events as players see them, with whether you are going.
[**list_world_experience_experience_events**](WorldsEngagementApi.md#list_world_experience_experience_events) | **GET** /v1/worlds/experiences/{experienceId}/experience-events | Your scheduled Experience Events (upcoming, live and the last 30 days) with RSVP counts.
[**list_world_experience_notification_templates**](WorldsEngagementApi.md#list_world_experience_notification_templates) | **GET** /v1/worlds/experiences/{experienceId}/notification-templates | Notification templates: key, title and body with {placeholders}, review status (approved ones can be sent).
[**list_world_me_experience_events**](WorldsEngagementApi.md#list_world_me_experience_events) | **GET** /v1/worlds/me/experience-events | Events you are going to that have not ended.
[**list_world_me_notification_opt_ins**](WorldsEngagementApi.md#list_world_me_notification_opt_ins) | **GET** /v1/worlds/me/notification-opt-ins | Every experience whose notifications you turned on.
[**update_world_experience_experience_event**](WorldsEngagementApi.md#update_world_experience_experience_event) | **PATCH** /v1/worlds/experiences/{experienceId}/experience-events/{eventId} | Change an event. A started event keeps its start; a new start time earns RSVPs a new reminder.
[**update_world_experience_notification_opt_in**](WorldsEngagementApi.md#update_world_experience_notification_opt_in) | **PUT** /v1/worlds/experiences/{experienceId}/notifications/opt-in | Turn this experience's notifications on or off for you. timeZone (IANA) keeps them out of your night.
[**update_world_experience_notification_template**](WorldsEngagementApi.md#update_world_experience_notification_template) | **PUT** /v1/worlds/experiences/{experienceId}/notification-templates/{key} | Create or reword a template. Clean words with no link are approved at once; anything the text filter flags, or any link, waits for Crossly review (24 h). Rewording re-reviews.



## create_world_experience_experience_event

> crate::models::WorldExperienceExperienceEventResponse create_world_experience_experience_event(experience_id)
Schedule an Experience Event shown on the game's card: title, description, start/end (5 min–14 days, ≤ 180 days ahead), thumbnail, place. With liveEventId it follows that live-ops event's schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperienceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_experience_event_cancel

> crate::models::WorldExperienceExperienceEventResponse create_world_experience_experience_event_cancel(experience_id, event_id)
Cancel an event (it disappears from the game's card; no reminder goes out).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**event_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperienceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_experience_event_rsvp

> crate::models::WorldExperienceExperienceEventResponse create_world_experience_experience_event_rsvp(experience_id, event_id)
RSVP (going: true) or take it back. remind: true also turns the experience's notifications on, so the 15-minute reminder can reach you.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**event_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperienceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_notification

> crate::models::CreateWorldExperienceNotificationResponse create_world_experience_notification(experience_id)
Send an approved template to one player (sub) or up to 100 (subs). Only players who turned notifications on, 13+, not banned, at most 2 a day per player and outside their quiet hours receive it; every other send is logged as suppressed with its reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceNotificationResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_notification_template_retire

> crate::models::UpdateWorldExperienceNotificationTemplateResponse create_world_experience_notification_template_retire(experience_id, key)
Retire a template: sends by its key stop.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceNotificationTemplateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_notification_opt_in

> crate::models::GetWorldExperienceNotificationOptInResponse get_world_experience_notification_opt_in(experience_id)
Whether you turned this experience's notifications on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceNotificationOptInResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_notification_report

> crate::models::GetWorldExperienceNotificationReportResponse get_world_experience_notification_report(experience_id)
Sends over the last N days (default 7, max 90): delivered, suppressed by reason, per template, and how many players turned notifications on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceNotificationReportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_event_upcoming

> crate::models::ListWorldExperienceEventUpcomingResponse list_world_experience_event_upcoming()
Events across public experiences in the next 7 days (and live now), soonest first — the Carnival Plaza's \"Coming up\" board.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldExperienceEventUpcomingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_experience_event_upcoming

> crate::models::ListWorldExperienceExperienceEventsResponse list_world_experience_experience_event_upcoming(experience_id)
An experience's upcoming and live events as players see them, with whether you are going.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceExperienceEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_experience_events

> crate::models::ListWorldExperienceExperienceEventsResponse list_world_experience_experience_events(experience_id)
Your scheduled Experience Events (upcoming, live and the last 30 days) with RSVP counts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceExperienceEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_notification_templates

> crate::models::ListWorldExperienceNotificationTemplatesResponse list_world_experience_notification_templates(experience_id)
Notification templates: key, title and body with {placeholders}, review status (approved ones can be sent).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceNotificationTemplatesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_me_experience_events

> crate::models::ListWorldMeExperienceEventsResponse list_world_me_experience_events()
Events you are going to that have not ended.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldMeExperienceEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_me_notification_opt_ins

> crate::models::ListWorldMeNotificationOptInsResponse list_world_me_notification_opt_ins()
Every experience whose notifications you turned on.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldMeNotificationOptInsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_experience_event

> crate::models::WorldExperienceExperienceEventResponse update_world_experience_experience_event(experience_id, event_id)
Change an event. A started event keeps its start; a new start time earns RSVPs a new reminder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**event_id** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceExperienceEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_notification_opt_in

> crate::models::GetWorldExperienceNotificationOptInResponse update_world_experience_notification_opt_in(experience_id)
Turn this experience's notifications on or off for you. timeZone (IANA) keeps them out of your night.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceNotificationOptInResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_notification_template

> crate::models::UpdateWorldExperienceNotificationTemplateResponse update_world_experience_notification_template(experience_id, key)
Create or reword a template. Clean words with no link are approved at once; anything the text filter flags, or any link, waits for Crossly review (24 h). Rewording re-reviews.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceNotificationTemplateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

