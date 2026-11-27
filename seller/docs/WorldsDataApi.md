# \WorldsDataApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_datastore_entry_increment**](WorldsDataApi.md#create_world_experience_datastore_entry_increment) | **POST** /v1/worlds/experiences/{experienceId}/datastores/{store}/entries/{key}/increment | Atomically add `by` to a numeric key (absent counts as 0).
[**create_world_experience_memory_queue_item**](WorldsDataApi.md#create_world_experience_memory_queue_item) | **POST** /v1/worlds/experiences/{experienceId}/memory/queues/{queue}/items | Add an item to a MemoryStore queue (ttlSeconds ≤ 45 days, priority 0-1000).
[**create_world_experience_memory_queue_read**](WorldsDataApi.md#create_world_experience_memory_queue_read) | **POST** /v1/worlds/experiences/{experienceId}/memory/queues/{queue}/read | Read and hide up to `count` queue items for invisibilitySeconds; remove them with the readId.
[**create_world_experience_messaging**](WorldsDataApi.md#create_world_experience_messaging) | **POST** /v1/worlds/experiences/{experienceId}/messaging/{topic} | Publish a message (≤ 1 KB) to every live server of the experience subscribed to the topic.
[**create_world_experience_ordered_entry_increment**](WorldsDataApi.md#create_world_experience_ordered_entry_increment) | **POST** /v1/worlds/experiences/{experienceId}/ordered/{store}/entries/{key}/increment | Atomically add `by` to an OrderedStore entry.
[**delete_world_experience_datastore_entry**](WorldsDataApi.md#delete_world_experience_datastore_entry) | **DELETE** /v1/worlds/experiences/{experienceId}/datastores/{store}/entries/{key} | Remove a key (a tombstone version; history stays readable for 30 days). Honours If-Match.
[**delete_world_experience_memory_map**](WorldsDataApi.md#delete_world_experience_memory_map) | **DELETE** /v1/worlds/experiences/{experienceId}/memory/maps/{map}/{key} | Remove a hash map item.
[**delete_world_experience_memory_queue_read**](WorldsDataApi.md#delete_world_experience_memory_queue_read) | **DELETE** /v1/worlds/experiences/{experienceId}/memory/queues/{queue}/reads/{readId} | Remove the items of one read (409 if the read lapsed).
[**delete_world_experience_memory_sorted**](WorldsDataApi.md#delete_world_experience_memory_sorted) | **DELETE** /v1/worlds/experiences/{experienceId}/memory/sorted/{map}/{key} | Remove a sorted map item.
[**delete_world_experience_ordered_entry**](WorldsDataApi.md#delete_world_experience_ordered_entry) | **DELETE** /v1/worlds/experiences/{experienceId}/ordered/{store}/entries/{key} | Remove an OrderedStore entry.
[**get_world_experience_data_usage**](WorldsDataApi.md#get_world_experience_data_usage) | **GET** /v1/worlds/experiences/{experienceId}/data/usage | Live quotas and usage: CCU, request budgets per minute, DataStore storage, MemoryStore memory.
[**get_world_experience_datastore_entry**](WorldsDataApi.md#get_world_experience_datastore_entry) | **GET** /v1/worlds/experiences/{experienceId}/datastores/{store}/entries/{key} | The current value of a key: version (ETag \"v<n>\"), userIds and attributes.
[**get_world_experience_datastore_entry_version**](WorldsDataApi.md#get_world_experience_datastore_entry_version) | **GET** /v1/worlds/experiences/{experienceId}/datastores/{store}/entries/{key}/versions/{version} | One version of a key, including a removed one.
[**get_world_experience_memory_map**](WorldsDataApi.md#get_world_experience_memory_map) | **GET** /v1/worlds/experiences/{experienceId}/memory/maps/{map} | Items of a MemoryStore hash map (cursor-paged, unordered).
[**get_world_experience_memory_map_by_key**](WorldsDataApi.md#get_world_experience_memory_map_by_key) | **GET** /v1/worlds/experiences/{experienceId}/memory/maps/{map}/{key} | One MemoryStore hash map item with its version and expiry.
[**get_world_experience_memory_queue**](WorldsDataApi.md#get_world_experience_memory_queue) | **GET** /v1/worlds/experiences/{experienceId}/memory/queues/{queue} | Queue length (visible, in flight) and the head, without hiding anything.
[**get_world_experience_memory_sorted**](WorldsDataApi.md#get_world_experience_memory_sorted) | **GET** /v1/worlds/experiences/{experienceId}/memory/sorted/{map} | A MemoryStore sorted map in order (sortKey, then key); ?order=&cursor=&limit=.
[**get_world_experience_memory_sorted_by_key**](WorldsDataApi.md#get_world_experience_memory_sorted_by_key) | **GET** /v1/worlds/experiences/{experienceId}/memory/sorted/{map}/{key} | One sorted map item.
[**get_world_experience_ordered**](WorldsDataApi.md#get_world_experience_ordered) | **GET** /v1/worlds/experiences/{experienceId}/ordered/{store} | An OrderedStore page ranked by value; ?order=desc|asc&min=&max=&cursor=&limit=.
[**get_world_experience_ordered_entry**](WorldsDataApi.md#get_world_experience_ordered_entry) | **GET** /v1/worlds/experiences/{experienceId}/ordered/{store}/entries/{key} | One OrderedStore entry.
[**list_world_experience_datastore_entries**](WorldsDataApi.md#list_world_experience_datastore_entries) | **GET** /v1/worlds/experiences/{experienceId}/datastores/{store}/entries | Keys of a DataStore in key order; ?prefix=&scope=&cursor=&limit=.
[**list_world_experience_datastore_entry_versions**](WorldsDataApi.md#list_world_experience_datastore_entry_versions) | **GET** /v1/worlds/experiences/{experienceId}/datastores/{store}/entries/{key}/versions | Versions of a key, newest first (30-day history; the current version is kept).
[**list_world_experience_datastores**](WorldsDataApi.md#list_world_experience_datastores) | **GET** /v1/worlds/experiences/{experienceId}/datastores | The DataStores of an experience, with key counts and bytes.
[**update_world_experience_datastore_entry**](WorldsDataApi.md#update_world_experience_datastore_entry) | **PUT** /v1/worlds/experiences/{experienceId}/datastores/{store}/entries/{key} | Write a new version. If-Match \"v<n>\" (or ifVersion) writes only at that version; If-None-Match: * only if absent. 412 on a mismatch.
[**update_world_experience_memory_map**](WorldsDataApi.md#update_world_experience_memory_map) | **PUT** /v1/worlds/experiences/{experienceId}/memory/maps/{map}/{key} | Set a hash map item (ttlSeconds required; ifVersion for compare-and-set).
[**update_world_experience_memory_sorted**](WorldsDataApi.md#update_world_experience_memory_sorted) | **PUT** /v1/worlds/experiences/{experienceId}/memory/sorted/{map}/{key} | Set a sorted map item (ttlSeconds, optional sortKey and ifVersion).
[**update_world_experience_ordered_entry**](WorldsDataApi.md#update_world_experience_ordered_entry) | **PUT** /v1/worlds/experiences/{experienceId}/ordered/{store}/entries/{key} | Set an OrderedStore entry to a whole number.



## create_world_experience_datastore_entry_increment

> crate::models::CreateWorldExperienceDatastoreEntryIncrementResponse create_world_experience_datastore_entry_increment(experience_id, store, key)
Atomically add `by` to a numeric key (absent counts as 0).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceDatastoreEntryIncrementResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_memory_queue_item

> crate::models::CreateWorldPartyInviteResponse create_world_experience_memory_queue_item(experience_id, queue)
Add an item to a MemoryStore queue (ttlSeconds ≤ 45 days, priority 0-1000).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldPartyInviteResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_memory_queue_read

> crate::models::CreateWorldExperienceMemoryQueueReadResponse create_world_experience_memory_queue_read(experience_id, queue)
Read and hide up to `count` queue items for invisibilitySeconds; remove them with the readId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceMemoryQueueReadResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_messaging

> crate::models::CreateWorldExperienceMessagingResponse create_world_experience_messaging(experience_id, topic)
Publish a message (≤ 1 KB) to every live server of the experience subscribed to the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**topic** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceMessagingResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_ordered_entry_increment

> crate::models::WorldExperienceOrderedEntryResponse create_world_experience_ordered_entry_increment(experience_id, store, key)
Atomically add `by` to an OrderedStore entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceOrderedEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_datastore_entry

> crate::models::DeleteWorldExperienceDatastoreEntryResponse delete_world_experience_datastore_entry(experience_id, store, key)
Remove a key (a tombstone version; history stays readable for 30 days). Honours If-Match.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldExperienceDatastoreEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_memory_map

> crate::models::DeleteSpatialSceneGrantResponse delete_world_experience_memory_map(experience_id, map, key)
Remove a hash map item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_memory_queue_read

> crate::models::DeleteSpatialSceneNodeResponse delete_world_experience_memory_queue_read(experience_id, queue, read_id)
Remove the items of one read (409 if the read lapsed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**queue** | **String** |  | [required] |
**read_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneNodeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_memory_sorted

> crate::models::DeleteSpatialSceneGrantResponse delete_world_experience_memory_sorted(experience_id, map, key)
Remove a sorted map item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_ordered_entry

> crate::models::DeleteWorldExperienceOrderedEntryResponse delete_world_experience_ordered_entry(experience_id, store, key)
Remove an OrderedStore entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldExperienceOrderedEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_data_usage

> crate::models::GetWorldExperienceDataUsageResponse get_world_experience_data_usage(experience_id)
Live quotas and usage: CCU, request budgets per minute, DataStore storage, MemoryStore memory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceDataUsageResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_datastore_entry

> crate::models::GetWorldExperienceDatastoreEntryResponse get_world_experience_datastore_entry(experience_id, store, key)
The current value of a key: version (ETag \"v<n>\"), userIds and attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceDatastoreEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_datastore_entry_version

> crate::models::GetWorldExperienceDatastoreEntryResponse get_world_experience_datastore_entry_version(experience_id, store, key, version)
One version of a key, including a removed one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |
**version** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceDatastoreEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_memory_map

> crate::models::GetWorldExperienceMemoryMapResponse get_world_experience_memory_map(experience_id, map)
Items of a MemoryStore hash map (cursor-paged, unordered).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceMemoryMapResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_memory_map_by_key

> crate::models::GetWorldExperienceMemoryMapByKeyResponse get_world_experience_memory_map_by_key(experience_id, map, key)
One MemoryStore hash map item with its version and expiry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceMemoryMapByKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_memory_queue

> crate::models::GetWorldExperienceMemoryQueueResponse get_world_experience_memory_queue(experience_id, queue)
Queue length (visible, in flight) and the head, without hiding anything.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceMemoryQueueResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_memory_sorted

> crate::models::GetWorldExperienceMemoryMapResponse get_world_experience_memory_sorted(experience_id, map)
A MemoryStore sorted map in order (sortKey, then key); ?order=&cursor=&limit=.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceMemoryMapResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_memory_sorted_by_key

> crate::models::GetWorldExperienceMemoryMapByKeyResponse get_world_experience_memory_sorted_by_key(experience_id, map, key)
One sorted map item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceMemoryMapByKeyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_ordered

> crate::models::GetWorldExperienceOrderedResponse get_world_experience_ordered(experience_id, store)
An OrderedStore page ranked by value; ?order=desc|asc&min=&max=&cursor=&limit=.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceOrderedResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_ordered_entry

> crate::models::WorldExperienceOrderedEntryResponse get_world_experience_ordered_entry(experience_id, store, key)
One OrderedStore entry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceOrderedEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_datastore_entries

> crate::models::ListWorldExperienceDatastoreEntriesResponse list_world_experience_datastore_entries(experience_id, store)
Keys of a DataStore in key order; ?prefix=&scope=&cursor=&limit=.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceDatastoreEntriesResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_datastore_entry_versions

> crate::models::ListWorldExperienceDatastoreEntryVersionsResponse list_world_experience_datastore_entry_versions(experience_id, store, key)
Versions of a key, newest first (30-day history; the current version is kept).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceDatastoreEntryVersionsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_datastores

> crate::models::ListWorldExperienceDatastoresResponse list_world_experience_datastores(experience_id)
The DataStores of an experience, with key counts and bytes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceDatastoresResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_datastore_entry

> crate::models::UpdateWorldExperienceDatastoreEntryResponse update_world_experience_datastore_entry(experience_id, store, key)
Write a new version. If-Match \"v<n>\" (or ifVersion) writes only at that version; If-None-Match: * only if absent. 412 on a mismatch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceDatastoreEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_memory_map

> crate::models::UpdateWorldExperienceMemoryMapResponse update_world_experience_memory_map(experience_id, map, key)
Set a hash map item (ttlSeconds required; ifVersion for compare-and-set).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceMemoryMapResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_memory_sorted

> crate::models::UpdateWorldExperienceMemoryMapResponse update_world_experience_memory_sorted(experience_id, map, key)
Set a sorted map item (ttlSeconds, optional sortKey and ifVersion).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**map** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperienceMemoryMapResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_ordered_entry

> crate::models::WorldExperienceOrderedEntryResponse update_world_experience_ordered_entry(experience_id, store, key)
Set an OrderedStore entry to a whole number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**store** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

**crate::models::WorldExperienceOrderedEntryResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

