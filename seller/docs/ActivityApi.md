# \ActivityApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_action_log_export**](ActivityApi.md#create_action_log_export) | **POST** /v1/action-log/exports | Queue an export of the Activity Log: every event (csv, ndjson, pdf) or a summary report (pdf, ndjson). Poll GET /api/v1/action-log/exports/{exportId}, then download it. Exports never contain IP addresses.
[**create_action_log_search_translate**](ActivityApi.md#create_action_log_search_translate) | **POST** /v1/action-log/search/translate | Turn a question in words (\"failed delists on Mercari last week\") into an Activity Log search: pass the returned `q` to GET /api/v1/action-log (or the histogram and csv/ndjson exports). Nothing is run. A 404 with details.code `not_enabled` means natural-language search is off for this account.
[**create_detected_event_event_confirm**](ActivityApi.md#create_detected_event_event_confirm) | **POST** /v1/detected-events/events/{id}/confirm | Answer an open ask (canConfirm true): optionally which item it is (itemId, one of the match candidates) and which actions to run instead of the ask's preselected ones. Returns the event. 409 ask_not_open when the event is not waiting.
[**create_detected_event_event_undo**](ActivityApi.md#create_detected_event_event_undo) | **POST** /v1/detected-events/events/{id}/undo | Undo what Crossly did for an event (canUndo true): every executed action's stored inverse runs, newest first (restore stock, relist, unlink…). Returns the event. 409 nothing_to_undo when nothing is reversible.
[**create_detected_event_rule**](ActivityApi.md#create_detected_event_rule) | **POST** /v1/detected-events/rules | Create a detection rule: which detected events it applies to (filter + optional condition over any event axis) and what Crossly does then (and else). Without order it is placed most-specific-first, above the default set. Returns the stored rule with its id.
[**create_detected_event_rule_place**](ActivityApi.md#create_detected_event_rule_place) | **POST** /v1/detected-events/rules/place | Save one rule and the whole evaluation order in one transaction: rule.id is an existing id (replace) or any new client id (create); ids is every rule id top-down, naming the rule by that id. Returns the stored rule and the list.
[**create_detected_event_rule_reorder**](ActivityApi.md#create_detected_event_rule_reorder) | **POST** /v1/detected-events/rules/reorder | Set the evaluation order: ids top-down, defaults included (ids from the rules list). Rules are re-numbered 10, 20, 30…; a default moved above one of your rules is stored as your edit. Returns the list.
[**create_detected_event_rule_reset_default**](ActivityApi.md#create_detected_event_rule_reset_default) | **POST** /v1/detected-events/rules/reset-defaults | Drop your edits of the shipped default rules (keys, or all when omitted). Returns the list.
[**create_detected_event_simulate**](ActivityApi.md#create_detected_event_simulate) | **POST** /v1/detected-events/simulate | Run one event through the rules without acting: the winning rule, rules it shadowed, the trace, the guards that held actions back and the plan per target with how it would run (direct / relay / ask / skip). event needs only kind; every axis left out takes its default. rules = an unsaved draft list.
[**create_detected_event_simulate_impact**](ActivityApi.md#create_detected_event_simulate_impact) | **POST** /v1/detected-events/simulate/impact | What a rule edit would change: your detected events of the last days (default 30, at most 500) re-resolved against the draft rules and against the stored ones. Per event: winning rule and actions before and after, and whether it changed. Nothing runs.
[**create_email_event_confirm**](ActivityApi.md#create_email_event_confirm) | **POST** /v1/email-events/{id}/confirm | Confirm an email-detected event: re-run it as certain (a possible sale records the sale and delists the item elsewhere). When several listings could be meant, pass platformListingDbId to choose one.
[**create_email_event_dismiss**](ActivityApi.md#create_email_event_dismiss) | **POST** /v1/email-events/{id}/dismiss | Dismiss an email-detected event (\"not a sale\"): closes the card without acting.
[**create_event_policy_decision_confirm**](ActivityApi.md#create_event_policy_decision_confirm) | **POST** /v1/event-policies/decisions/{id}/confirm | Confirm a review decision from a webhook, order poller or app relay (canConfirm true): the event runs again with the seller confirmed, so a review rule runs the strongest allowed non-review action for that cell. Returns the new decision. 409 when the decision was already confirmed or is not confirmable (email review decisions are confirmed on their review card).
[**delete_detected_event_rule**](ActivityApi.md#delete_detected_event_rule) | **DELETE** /v1/detected-events/rules/{id} | Delete a detection rule. Deleting your edit of a default (its id, or default:<key>) brings the shipped default back; deleting a translated event-policy rule deletes that policy rule too.
[**delete_event_policy_rule**](ActivityApi.md#delete_event_policy_rule) | **DELETE** /v1/event-policies/rules/{id} | Delete a rule: its cell falls back to the next scope (platform → global → the built-in default).
[**get_action_log**](ActivityApi.md#get_action_log) | **GET** /v1/action-log/{id} | One Activity Log event by id, with `view` including its steps and, for bulk jobs, its children.
[**get_action_log_export**](ActivityApi.md#get_action_log_export) | **GET** /v1/action-log/exports/{exportId} | The state of one Activity Log export.
[**get_action_log_export_download**](ActivityApi.md#get_action_log_export_download) | **GET** /v1/action-log/exports/{exportId}/download | Download a finished Activity Log export (for 24 hours after it completes).
[**get_action_log_facet**](ActivityApi.md#get_action_log_facet) | **GET** /v1/action-log/facets | Filter options of the caller's Activity Log (last 90 days): areas with their actions and counts, the team members who acted, platforms and outcomes.
[**get_detected_event_event**](ActivityApi.md#get_detected_event_event) | **GET** /v1/detected-events/events/{id} | One detected event with every report of it (first + corroborations, with correlation and contradiction), the guards that held actions back, the plan and each executed action's outcome and reversibility.
[**get_event_policy**](ActivityApi.md#get_event_policy) | **GET** /v1/event-policies | What Crossly does with each detected marketplace event (sale, listing ended, offer, cancellation, label, shipped, delivered, refund, return, payout, message), per who owns the item (crossly, imported, app_managed, other_tool, untracked). Each matrix cell lists the allowed actions and the effective one, with the scope it comes from (app > platform > global > default). Pass platform (or appId) to see the matrix as that platform's (or app's) items see it. Also returns the rules, platforms and connected apps.
[**list_action_log**](ActivityApi.md#list_action_log) | **GET** /v1/action-log | List Activity Log events, newest first. Each event carries `view` (title, sentence, fields, failure advice, who did it) and typed `detail`. Page with `cursor`; `pagination.total` counts matches.
[**list_action_log_calls**](ActivityApi.md#list_action_log_calls) | **GET** /v1/action-log/{id}/calls | The outbound platform HTTP calls under an event (oldest first) — platform, operation, method, host, status, latency, error class/message, proxy + recipe id. Answers \"what was sent / what went wrong\". The URL path, request/response bodies and persisted-query hash are never returned.
[**list_action_log_histogram**](ActivityApi.md#list_action_log_histogram) | **GET** /v1/action-log/histogram | Events per UTC day for the list filters (default: the last 30 days; at most 366).
[**list_detected_event_events**](ActivityApi.md#list_detected_event_events) | **GET** /v1/detected-events/events | Every event Crossly detected (sales, order changes, listing changes, offers, messages, account notices), newest first: all its axes, the rule that decided it, its corroborating reports, the plan per target and how each action went, with canConfirm (an open ask) and canUndo. Cursor-paged; filter by kinds, platforms statuses and accounts (comma-separated) and since (ISO time).
[**list_detected_event_rules**](ActivityApi.md#list_detected_event_rules) | **GET** /v1/detected-events/rules | The seller's detection rules, top-down in the order they are evaluated (first match wins unless a rule says continue): the seller's own rules, translations of their event-policy rules (source legacy), then the shipped default set (source default, ids default:<key>; a stored edit of a default replaces it). Each rule has a filter (event kinds, platforms, accounts, apps, tags, items), an optional condition over any event axis, the actions it runs (then / else) and its guard loosenings.
[**list_email_events**](ActivityApi.md#list_email_events) | **GET** /v1/email-events | List email-detected events waiting for the seller (status=needs_review): a possible sale, ended listing, cancellation or other event an email reported that was not certain enough to act on automatically.
[**list_event_policy_decisions**](ActivityApi.md#list_event_policy_decisions) | **GET** /v1/event-policies/decisions | The decision log, newest first: for each detected event, what the rule or default asked for, what was done (after capability fallback and the email confidence ceiling), the rule that decided it, and the outcome (done / skipped / failed / review). Cursor-paged; filter by event, ownership, platform or outcome (outcome=review: still waiting for the seller). Kept 90 days.
[**update_detected_event_rule**](ActivityApi.md#update_detected_event_rule) | **PUT** /v1/detected-events/rules/{id} | Replace a detection rule. An id default:<key> stores your edit of that shipped default (delete it, or reset-defaults, to go back to the shipped one).
[**update_event_policy_rule**](ActivityApi.md#update_event_policy_rule) | **PUT** /v1/event-policies/rules | Set what Crossly does with one kind of detected event for one kind of item: scope global (scopeValue null), platform (scopeValue = platform id) or app (scopeValue = OAuth app id, app_managed items only). Replaces the rule for the same cell and scope. The action must be one the matrix allows for that cell; import actions exist only for untracked items.



## create_action_log_export

> crate::models::GetActionLogExportResponse create_action_log_export(create_action_log_export_request)
Queue an export of the Activity Log: every event (csv, ndjson, pdf) or a summary report (pdf, ndjson). Poll GET /api/v1/action-log/exports/{exportId}, then download it. Exports never contain IP addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_action_log_export_request** | **CreateActionLogExportRequest** |  | [required] |

### Return type

**crate::models::GetActionLogExportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_action_log_search_translate

> crate::models::CreateActionLogSearchTranslateResponse create_action_log_search_translate(create_action_log_search_translate_request)
Turn a question in words (\"failed delists on Mercari last week\") into an Activity Log search: pass the returned `q` to GET /api/v1/action-log (or the histogram and csv/ndjson exports). Nothing is run. A 404 with details.code `not_enabled` means natural-language search is off for this account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_action_log_search_translate_request** | **CreateActionLogSearchTranslateRequest** |  | [required] |

### Return type

**crate::models::CreateActionLogSearchTranslateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_event_confirm

> crate::models::CreateDetectedEventEventUndoResponse create_detected_event_event_confirm(id, create_detected_event_event_confirm_request)
Answer an open ask (canConfirm true): optionally which item it is (itemId, one of the match candidates) and which actions to run instead of the ask's preselected ones. Returns the event. 409 ask_not_open when the event is not waiting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_detected_event_event_confirm_request** | **CreateDetectedEventEventConfirmRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventEventUndoResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_event_undo

> crate::models::CreateDetectedEventEventUndoResponse create_detected_event_event_undo(id)
Undo what Crossly did for an event (canUndo true): every executed action's stored inverse runs, newest first (restore stock, relist, unlink…). Returns the event. 409 nothing_to_undo when nothing is reversible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateDetectedEventEventUndoResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_rule

> crate::models::CreateDetectedEventRuleResponse create_detected_event_rule(create_detected_event_rule_request)
Create a detection rule: which detected events it applies to (filter + optional condition over any event axis) and what Crossly does then (and else). Without order it is placed most-specific-first, above the default set. Returns the stored rule with its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_detected_event_rule_request** | **CreateDetectedEventRuleRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventRuleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_rule_place

> crate::models::CreateDetectedEventRulePlaceResponse create_detected_event_rule_place(create_detected_event_rule_place_request)
Save one rule and the whole evaluation order in one transaction: rule.id is an existing id (replace) or any new client id (create); ids is every rule id top-down, naming the rule by that id. Returns the stored rule and the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_detected_event_rule_place_request** | **CreateDetectedEventRulePlaceRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventRulePlaceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_rule_reorder

> crate::models::CreateDetectedEventRuleReorderResponse create_detected_event_rule_reorder(create_detected_event_rule_reorder_request)
Set the evaluation order: ids top-down, defaults included (ids from the rules list). Rules are re-numbered 10, 20, 30…; a default moved above one of your rules is stored as your edit. Returns the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_detected_event_rule_reorder_request** | **CreateDetectedEventRuleReorderRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventRuleReorderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_rule_reset_default

> crate::models::CreateDetectedEventRuleReorderResponse create_detected_event_rule_reset_default(create_detected_event_rule_reset_default_request)
Drop your edits of the shipped default rules (keys, or all when omitted). Returns the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_detected_event_rule_reset_default_request** | **CreateDetectedEventRuleResetDefaultRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventRuleReorderResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_simulate

> crate::models::CreateDetectedEventSimulateResponse create_detected_event_simulate(create_detected_event_simulate_request)
Run one event through the rules without acting: the winning rule, rules it shadowed, the trace, the guards that held actions back and the plan per target with how it would run (direct / relay / ask / skip). event needs only kind; every axis left out takes its default. rules = an unsaved draft list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_detected_event_simulate_request** | **CreateDetectedEventSimulateRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventSimulateResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_detected_event_simulate_impact

> crate::models::CreateDetectedEventSimulateImpactResponse create_detected_event_simulate_impact(create_detected_event_simulate_impact_request)
What a rule edit would change: your detected events of the last days (default 30, at most 500) re-resolved against the draft rules and against the stored ones. Per event: winning rule and actions before and after, and whether it changed. Nothing runs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_detected_event_simulate_impact_request** | **CreateDetectedEventSimulateImpactRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventSimulateImpactResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_email_event_confirm

> crate::models::CreateEmailEventConfirmResponse create_email_event_confirm(id, create_email_event_confirm_request)
Confirm an email-detected event: re-run it as certain (a possible sale records the sale and delists the item elsewhere). When several listings could be meant, pass platformListingDbId to choose one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_email_event_confirm_request** | **CreateEmailEventConfirmRequest** |  | [required] |

### Return type

**crate::models::CreateEmailEventConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_email_event_dismiss

> crate::models::IdStatusResponse create_email_event_dismiss(id)
Dismiss an email-detected event (\"not a sale\"): closes the card without acting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::IdStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_event_policy_decision_confirm

> crate::models::CreateEventPolicyDecisionConfirmResponse create_event_policy_decision_confirm(id)
Confirm a review decision from a webhook, order poller or app relay (canConfirm true): the event runs again with the seller confirmed, so a review rule runs the strongest allowed non-review action for that cell. Returns the new decision. 409 when the decision was already confirmed or is not confirmable (email review decisions are confirmed on their review card).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateEventPolicyDecisionConfirmResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_detected_event_rule

> crate::models::DeleteStoryResponse delete_detected_event_rule(id)
Delete a detection rule. Deleting your edit of a default (its id, or default:<key>) brings the shipped default back; deleting a translated event-policy rule deletes that policy rule too.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_event_policy_rule

> crate::models::DeleteStoryResponse delete_event_policy_rule(id)
Delete a rule: its cell falls back to the next scope (platform → global → the built-in default).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteStoryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_action_log

> crate::models::ListActionLogItem get_action_log(id)
One Activity Log event by id, with `view` including its steps and, for bulk jobs, its children.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListActionLogItem**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_action_log_export

> crate::models::GetActionLogExportResponse get_action_log_export(export_id)
The state of one Activity Log export.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_id** | **String** |  | [required] |

### Return type

**crate::models::GetActionLogExportResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_action_log_export_download

> crate::models::GetActionLogExportDownloadResponse get_action_log_export_download(export_id)
Download a finished Activity Log export (for 24 hours after it completes).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_id** | **String** |  | [required] |

### Return type

**crate::models::GetActionLogExportDownloadResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_action_log_facet

> crate::models::GetActionLogFacetResponse get_action_log_facet()
Filter options of the caller's Activity Log (last 90 days): areas with their actions and counts, the team members who acted, platforms and outcomes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetActionLogFacetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_detected_event_event

> crate::models::GetDetectedEventEventResponse get_detected_event_event(id)
One detected event with every report of it (first + corroborations, with correlation and contradiction), the guards that held actions back, the plan and each executed action's outcome and reversibility.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetDetectedEventEventResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_event_policy

> crate::models::GetEventPolicyResponse get_event_policy(platform, app_id)
What Crossly does with each detected marketplace event (sale, listing ended, offer, cancellation, label, shipped, delivered, refund, return, payout, message), per who owns the item (crossly, imported, app_managed, other_tool, untracked). Each matrix cell lists the allowed actions and the effective one, with the scope it comes from (app > platform > global > default). Pass platform (or appId) to see the matrix as that platform's (or app's) items see it. Also returns the rules, platforms and connected apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> |  |  |
**app_id** | Option<**String**> |  |  |

### Return type

**crate::models::GetEventPolicyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_action_log

> crate::models::ListActionLogResponse list_action_log(platform, action, category, area, status, source, target_type, target_id, actor_user_id, q, since, until, at, cursor, limit, offset)
List Activity Log events, newest first. Each event carries `view` (title, sentence, fields, failure advice, who did it) and typed `detail`. Page with `cursor`; `pagination.total` counts matches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> | Platform id (`ebay`, `poshmark`, ...). |  |
**action** | Option<**String**> | A canonical action key (`listing.delist`, see the public action catalog) or a legacy action word. |  |
**category** | Option<**String**> |  |  |
**area** | Option<**String**> | A seller area: every action of that area (the grouped filter of the dashboard). |  |
**status** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**target_type** | Option<**String**> | With targetId: events about that object (any of their targets). |  |
**target_id** | Option<**String**> |  |  |
**actor_user_id** | Option<**String**> | Only what one team member did. |  |
**q** | Option<**String**> | A search in the activity query language (`action:listing.* -outcome:failure \"free text\"`), ANDed with the other filters. POST /api/v1/action-log/search/translate turns a question in words into one. |  |
**since** | Option<**String**> | Inclusive lower bound on when the action happened. |  |
**until** | Option<**String**> | Exclusive upper bound. |  |
**at** | Option<**String**> | Jump to a time: the page starts at the newest event at or before it. |  |
**cursor** | Option<**String**> | `pagination.nextCursor` of the previous page. |  |
**limit** | Option<**i32**> |  |  |[default to 50]
**offset** | Option<**i32**> | Deprecated: use cursor. offset + limit may reach at most 1000 events. |  |

### Return type

**crate::models::ListActionLogResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_action_log_calls

> crate::models::ListActionLogCallsResponse list_action_log_calls(id)
The outbound platform HTTP calls under an event (oldest first) — platform, operation, method, host, status, latency, error class/message, proxy + recipe id. Answers \"what was sent / what went wrong\". The URL path, request/response bodies and persisted-query hash are never returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::ListActionLogCallsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_action_log_histogram

> crate::models::ListActionLogHistogramResponse list_action_log_histogram(platform, action, category, area, status, source, target_type, target_id, actor_user_id, q, since, until)
Events per UTC day for the list filters (default: the last 30 days; at most 366).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | Option<**String**> | Platform id (`ebay`, `poshmark`, ...). |  |
**action** | Option<**String**> | A canonical action key (`listing.delist`, see the public action catalog) or a legacy action word. |  |
**category** | Option<**String**> |  |  |
**area** | Option<**String**> | A seller area: every action of that area (the grouped filter of the dashboard). |  |
**status** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**target_type** | Option<**String**> | With targetId: events about that object (any of their targets). |  |
**target_id** | Option<**String**> |  |  |
**actor_user_id** | Option<**String**> | Only what one team member did. |  |
**q** | Option<**String**> | A search in the activity query language (`action:listing.* -outcome:failure \"free text\"`), ANDed with the other filters. POST /api/v1/action-log/search/translate turns a question in words into one. |  |
**since** | Option<**String**> | Inclusive lower bound on when the action happened. |  |
**until** | Option<**String**> | Exclusive upper bound. |  |

### Return type

**crate::models::ListActionLogHistogramResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_detected_event_events

> crate::models::ListDetectedEventEventsResponse list_detected_event_events(kinds, platforms, accounts, statuses, since, cursor, limit)
Every event Crossly detected (sales, order changes, listing changes, offers, messages, account notices), newest first: all its axes, the rule that decided it, its corroborating reports, the plan per target and how each action went, with canConfirm (an open ask) and canUndo. Cursor-paged; filter by kinds, platforms statuses and accounts (comma-separated) and since (ISO time).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kinds** | Option<**String**> |  |  |
**platforms** | Option<**String**> |  |  |
**accounts** | Option<**String**> | Comma-separated platform account ids (scope.account). |  |
**statuses** | Option<**String**> | Comma-separated: acted, held, asked, confirmed, ignored, undone, failed, received, corrected |  |
**since** | Option<**String**> |  |  |
**cursor** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

**crate::models::ListDetectedEventEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_detected_event_rules

> crate::models::ListDetectedEventRulesResponse list_detected_event_rules()
The seller's detection rules, top-down in the order they are evaluated (first match wins unless a rule says continue): the seller's own rules, translations of their event-policy rules (source legacy), then the shipped default set (source default, ids default:<key>; a stored edit of a default replaces it). Each rule has a filter (event kinds, platforms, accounts, apps, tags, items), an optional condition over any event axis, the actions it runs (then / else) and its guard loosenings.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListDetectedEventRulesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_email_events

> crate::models::ListEmailEventsResponse list_email_events(status)
List email-detected events waiting for the seller (status=needs_review): a possible sale, ended listing, cancellation or other event an email reported that was not certain enough to act on automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |[default to needs_review]

### Return type

**crate::models::ListEmailEventsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_event_policy_decisions

> crate::models::ListEventPolicyDecisionsResponse list_event_policy_decisions(cursor, limit, event, ownership, platform, outcome)
The decision log, newest first: for each detected event, what the rule or default asked for, what was done (after capability fallback and the email confidence ceiling), the rule that decided it, and the outcome (done / skipped / failed / review). Cursor-paged; filter by event, ownership, platform or outcome (outcome=review: still waiting for the seller). Kept 90 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**event** | Option<**String**> |  |  |
**ownership** | Option<**String**> |  |  |
**platform** | Option<**String**> |  |  |
**outcome** | Option<**String**> |  |  |

### Return type

**crate::models::ListEventPolicyDecisionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_detected_event_rule

> crate::models::CreateDetectedEventRuleResponse update_detected_event_rule(id, create_detected_event_rule_request)
Replace a detection rule. An id default:<key> stores your edit of that shipped default (delete it, or reset-defaults, to go back to the shipped one).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_detected_event_rule_request** | **CreateDetectedEventRuleRequest** |  | [required] |

### Return type

**crate::models::CreateDetectedEventRuleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_event_policy_rule

> crate::models::UpdateEventPolicyRuleResponse update_event_policy_rule(update_event_policy_rule_request)
Set what Crossly does with one kind of detected event for one kind of item: scope global (scopeValue null), platform (scopeValue = platform id) or app (scopeValue = OAuth app id, app_managed items only). Replaces the rule for the same cell and scope. The action must be one the matrix allows for that cell; import actions exist only for untracked items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_event_policy_rule_request** | **UpdateEventPolicyRuleRequest** |  | [required] |

### Return type

**crate::models::UpdateEventPolicyRuleResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

