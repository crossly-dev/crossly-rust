# \BuyerAgentApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_buyer_agent_session**](BuyerAgentApi.md#create_buyer_agent_session) | **POST** /v1/buyer/agent/sessions | Start a shopping agent conversation.
[**create_buyer_agent_session_act**](BuyerAgentApi.md#create_buyer_agent_session_act) | **POST** /v1/buyer/agent/sessions/{id}/act | Act on an option: save, cart, alert, registry, price (real-sales check), offer (prepare / confirm), or buy (quote — complete with POST /api/v1/buyer/checkout). Needs the action’s own scope too.
[**create_buyer_agent_session_react**](BuyerAgentApi.md#create_buyer_agent_session_react) | **POST** /v1/buyer/agent/sessions/{id}/react | Like or dislike an option; the next search leans toward likes and drops dislikes.
[**create_buyer_agent_session_turn**](BuyerAgentApi.md#create_buyer_agent_session_turn) | **POST** /v1/buyer/agent/sessions/{id}/turns | Say something to the agent: what you want, a critique of the options (\"cheaper\", \"more like #2\"), or send a photo (imageBase64) to find things like it. Returns its reply and the new numbered options.
[**delete_buyer_agent_session**](BuyerAgentApi.md#delete_buyer_agent_session) | **DELETE** /v1/buyer/agent/sessions/{id} | Delete a conversation.
[**get_buyer_agent_session**](BuyerAgentApi.md#get_buyer_agent_session) | **GET** /v1/buyer/agent/sessions/{id} | A conversation: its brief and turns, with each turn’s options re-read live.
[**list_buyer_agent_sessions**](BuyerAgentApi.md#list_buyer_agent_sessions) | **GET** /v1/buyer/agent/sessions | Your shopping agent conversations, newest first.



## create_buyer_agent_session

> crate::models::CreateBuyerAgentSessionResponse create_buyer_agent_session()
Start a shopping agent conversation.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::CreateBuyerAgentSessionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_agent_session_act

> crate::models::CreateBuyerAgentSessionActResponse create_buyer_agent_session_act(id)
Act on an option: save, cart, alert, registry, price (real-sales check), offer (prepare / confirm), or buy (quote — complete with POST /api/v1/buyer/checkout). Needs the action’s own scope too.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerAgentSessionActResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_agent_session_react

> crate::models::DeleteBuyerAgentSessionResponse create_buyer_agent_session_react(id)
Like or dislike an option; the next search leans toward likes and drops dislikes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteBuyerAgentSessionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_buyer_agent_session_turn

> crate::models::CreateBuyerAgentSessionTurnResponse create_buyer_agent_session_turn(id)
Say something to the agent: what you want, a critique of the options (\"cheaper\", \"more like #2\"), or send a photo (imageBase64) to find things like it. Returns its reply and the new numbered options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::CreateBuyerAgentSessionTurnResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_buyer_agent_session

> crate::models::DeleteBuyerAgentSessionResponse delete_buyer_agent_session(id)
Delete a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::DeleteBuyerAgentSessionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_buyer_agent_session

> crate::models::GetBuyerAgentSessionResponse get_buyer_agent_session(id)
A conversation: its brief and turns, with each turn’s options re-read live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**crate::models::GetBuyerAgentSessionResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_buyer_agent_sessions

> crate::models::ListBuyerAgentSessionsResponse list_buyer_agent_sessions()
Your shopping agent conversations, newest first.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListBuyerAgentSessionsResponse**

### Authorization

[BuyerOAuth](../README.md#BuyerOAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

