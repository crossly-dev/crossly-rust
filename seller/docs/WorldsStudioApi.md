# \WorldsStudioApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_experience_asset_mesh**](WorldsStudioApi.md#create_world_experience_asset_mesh) | **POST** /v1/worlds/experiences/{experienceId}/assets/meshes | Import a .glb (base64) as a MeshPart: safety parse + budgets, stored on the Crossly CDN, a phone LOD over 5k triangles, collision Box or convex Hull. Returns the asset and the MeshPart node; with placeId it is inserted into that place through its co-editing session.
[**create_world_experience_asset_mesh_check**](WorldsStudioApi.md#create_world_experience_asset_mesh_check) | **POST** /v1/worlds/experiences/{experienceId}/assets/meshes/check | Check a .glb (base64) for import without storing it: the budget report per device, plain-English errors with a fix list, whether a phone LOD will be made, mesh bounds and hull points.
[**create_world_experience_place_assistant**](WorldsStudioApi.md#create_world_experience_place_assistant) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/assistant | Ask the Studio Assistant (mode explain | edit | build): returns an explanation and a proposed change set (scripts, inserts, edits) checked against your role and linted. Nothing is applied.
[**create_world_experience_place_assistant_apply**](WorldsStudioApi.md#create_world_experience_place_assistant_apply) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/assistant/apply | Apply accepted Assistant changes to the place as one co-editing batch (activity via assistant, revertible). Re-checked against the place as it is now.
[**create_world_experience_place_studio_edit**](WorldsStudioApi.md#create_world_experience_place_studio_edit) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/edits | Apply set / insert / move / delete steps by path as ONE batch through Team Create (live in every open Studio, one revertible activity entry). All-or-nothing: any bad step refuses the batch with every problem listed.
[**create_world_experience_place_studio_insert_asset**](WorldsStudioApi.md#create_world_experience_place_studio_insert_asset) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/insert-asset | Insert a Toolbox asset under a path (default Workspace; ServerScripts for scripts). Paid assets need the experience owner's licence.
[**create_world_experience_place_studio_playtest**](WorldsStudioApi.md#create_world_experience_place_studio_playtest) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/playtest | Cloud test: flush the session, freeze a private test version and start a fleet server for it. 201 ready (ticket) or 202 starting; outputCursor reads only this test's Output.
[**create_world_experience_place_studio_playtest_stop**](WorldsStudioApi.md#create_world_experience_place_studio_playtest_stop) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/playtest/stop | Stop this place's cloud-test servers now.
[**create_world_experience_place_studio_publish**](WorldsStudioApi.md#create_world_experience_place_studio_publish) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/publish | Prepare a publish from the live session: checks, details, rating answers, access. Never submits and never acknowledges the money review — the person does (or submit_worlds_publish).
[**create_world_experience_place_studio_script_commit**](WorldsStudioApi.md#create_world_experience_place_studio_script_commit) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/script/commit | Commit your draft with a three-way merge onto the script as it is now (collaborators' edits kept). 409 merge_conflict carries merged text with conflict blocks and theirs.
[**delete_world_experience_place_studio_script**](WorldsStudioApi.md#delete_world_experience_place_studio_script) | **DELETE** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/script | Discard your draft of a script.
[**get_world_assistant_status**](WorldsStudioApi.md#get_world_assistant_status) | **GET** /v1/worlds/assistant/status | Studio Assistant status: whether Crossly's model key is configured, whether you have your own AI key, and your plan allowance (asks left today, budget left this month).
[**get_world_experience_place_studio_find**](WorldsStudioApi.md#get_world_experience_place_studio_find) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/find | Find instances by name (contains), exact class and/or tag.
[**get_world_experience_place_studio_instance**](WorldsStudioApi.md#get_world_experience_place_studio_instance) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/instance | One instance by path (or session id): class, properties, attributes, tags, children. Script source is read with …/studio/script.
[**get_world_experience_place_studio_output**](WorldsStudioApi.md#get_world_experience_place_studio_output) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/output | Server Output (print, warnings, errors) after a cursor: every line of cloud tests, errors of live servers (scope=all). Kept 24 hours.
[**get_world_experience_place_studio_problem**](WorldsStudioApi.md#get_world_experience_place_studio_problem) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/problems | The publish checks on the place as the live session holds it: script lint, budgets per device, settings, shop listings, Toolbox licences. publishable = no errors.
[**get_world_experience_place_studio_script**](WorldsStudioApi.md#get_world_experience_place_studio_script) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/script | A script's current source, your draft of it (if any, with whether someone committed since) and the lint of each.
[**get_world_experience_place_studio_snapshot**](WorldsStudioApi.md#get_world_experience_place_studio_snapshot) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/snapshot | A plan (top) or front drawing of the parts under a path as SVG, with bounds, spawns and unanchored parts that will fall.
[**get_world_experience_place_studio_tree**](WorldsStudioApi.md#get_world_experience_place_studio_tree) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/tree | The live instance tree (what every open Studio shows) by dotted path, depth levels down (default 3, max 12).
[**get_world_studio_class**](WorldsStudioApi.md#get_world_studio_class) | **GET** /v1/worlds/studio/classes | The engine's class table: insertable classes, services, and for ?name= one class with every property, its type, default and the exact JSON a place (and a studio edit) takes.
[**list_world_experience_assets**](WorldsStudioApi.md#list_world_experience_assets) | **GET** /v1/worlds/experiences/{experienceId}/assets | Meshes imported into this experience, newest first: paths, phone LOD, bounds, hull size, budget report, moderation (pending until a reviewed publish uses them) and a URL you can load now.
[**update_world_experience_place_studio_script**](WorldsStudioApi.md#update_world_experience_place_studio_script) | **PUT** /v1/worlds/experiences/{experienceId}/places/{placeId}/studio/script | Save your draft of a script (the place does not change until you commit). Returns lint and the diff from the current source.



## create_world_experience_asset_mesh

> crate::models::CreateWorldExperienceAssetMeshResponse create_world_experience_asset_mesh(experience_id)
Import a .glb (base64) as a MeshPart: safety parse + budgets, stored on the Crossly CDN, a phone LOD over 5k triangles, collision Box or convex Hull. Returns the asset and the MeshPart node; with placeId it is inserted into that place through its co-editing session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceAssetMeshResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_asset_mesh_check

> crate::models::CreateWorldExperienceAssetMeshCheckResponse create_world_experience_asset_mesh_check(experience_id)
Check a .glb (base64) for import without storing it: the budget report per device, plain-English errors with a fix list, whether a phone LOD will be made, mesh bounds and hull points.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceAssetMeshCheckResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_assistant

> crate::models::CreateWorldExperiencePlaceAssistantResponse create_world_experience_place_assistant(experience_id, place_id)
Ask the Studio Assistant (mode explain | edit | build): returns an explanation and a proposed change set (scripts, inserts, edits) checked against your role and linted. Nothing is applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceAssistantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_assistant_apply

> crate::models::CreateWorldExperiencePlaceAssistantApplyResponse create_world_experience_place_assistant_apply(experience_id, place_id)
Apply accepted Assistant changes to the place as one co-editing batch (activity via assistant, revertible). Re-checked against the place as it is now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceAssistantApplyResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_studio_edit

> crate::models::CreateWorldExperiencePlaceStudioEditResponse create_world_experience_place_studio_edit(experience_id, place_id)
Apply set / insert / move / delete steps by path as ONE batch through Team Create (live in every open Studio, one revertible activity entry). All-or-nothing: any bad step refuses the batch with every problem listed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceStudioEditResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_studio_insert_asset

> crate::models::CreateWorldExperiencePlaceStudioInsertAssetResponse create_world_experience_place_studio_insert_asset(experience_id, place_id)
Insert a Toolbox asset under a path (default Workspace; ServerScripts for scripts). Paid assets need the experience owner's licence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceStudioInsertAssetResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_studio_playtest

> crate::models::CreateWorldExperiencePlaceStudioPlaytestResponse create_world_experience_place_studio_playtest(experience_id, place_id)
Cloud test: flush the session, freeze a private test version and start a fleet server for it. 201 ready (ticket) or 202 starting; outputCursor reads only this test's Output.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceStudioPlaytestResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_studio_playtest_stop

> crate::models::CreateWorldExperiencePlaceStudioPlaytestStopResponse create_world_experience_place_studio_playtest_stop(experience_id, place_id)
Stop this place's cloud-test servers now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceStudioPlaytestStopResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_studio_publish

> crate::models::CreateWorldExperiencePlaceStudioPublishResponse create_world_experience_place_studio_publish(experience_id, place_id)
Prepare a publish from the live session: checks, details, rating answers, access. Never submits and never acknowledges the money review — the person does (or submit_worlds_publish).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceStudioPublishResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_studio_script_commit

> crate::models::CreateWorldExperiencePlaceStudioScriptCommitResponse create_world_experience_place_studio_script_commit(experience_id, place_id)
Commit your draft with a three-way merge onto the script as it is now (collaborators' edits kept). 409 merge_conflict carries merged text with conflict blocks and theirs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceStudioScriptCommitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_place_studio_script

> crate::models::DeleteWorldExperiencePlaceCollabScriptResponse delete_world_experience_place_studio_script(experience_id, place_id)
Discard your draft of a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldExperiencePlaceCollabScriptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_assistant_status

> crate::models::GetWorldAssistantStatusResponse get_world_assistant_status()
Studio Assistant status: whether Crossly's model key is configured, whether you have your own AI key, and your plan allowance (asks left today, budget left this month).

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldAssistantStatusResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_studio_find

> crate::models::GetWorldExperiencePlaceStudioFindResponse get_world_experience_place_studio_find(experience_id, place_id)
Find instances by name (contains), exact class and/or tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceStudioFindResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_studio_instance

> crate::models::GetWorldExperiencePlaceStudioInstanceResponse get_world_experience_place_studio_instance(experience_id, place_id)
One instance by path (or session id): class, properties, attributes, tags, children. Script source is read with …/studio/script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceStudioInstanceResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_studio_output

> crate::models::GetWorldExperiencePlaceStudioOutputResponse get_world_experience_place_studio_output(experience_id, place_id)
Server Output (print, warnings, errors) after a cursor: every line of cloud tests, errors of live servers (scope=all). Kept 24 hours.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceStudioOutputResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_studio_problem

> crate::models::GetWorldExperiencePlaceStudioProblemResponse get_world_experience_place_studio_problem(experience_id, place_id)
The publish checks on the place as the live session holds it: script lint, budgets per device, settings, shop listings, Toolbox licences. publishable = no errors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceStudioProblemResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_studio_script

> crate::models::GetWorldExperiencePlaceStudioScriptResponse get_world_experience_place_studio_script(experience_id, place_id)
A script's current source, your draft of it (if any, with whether someone committed since) and the lint of each.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceStudioScriptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_studio_snapshot

> crate::models::GetWorldExperiencePlaceStudioSnapshotResponse get_world_experience_place_studio_snapshot(experience_id, place_id)
A plan (top) or front drawing of the parts under a path as SVG, with bounds, spawns and unanchored parts that will fall.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceStudioSnapshotResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_studio_tree

> crate::models::GetWorldExperiencePlaceStudioTreeResponse get_world_experience_place_studio_tree(experience_id, place_id)
The live instance tree (what every open Studio shows) by dotted path, depth levels down (default 3, max 12).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceStudioTreeResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_studio_class

> crate::models::GetWorldStudioClassResponse get_world_studio_class()
The engine's class table: insertable classes, services, and for ?name= one class with every property, its type, default and the exact JSON a place (and a studio edit) takes.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::GetWorldStudioClassResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_assets

> crate::models::ListWorldExperienceAssetsResponse list_world_experience_assets(experience_id)
Meshes imported into this experience, newest first: paths, phone LOD, bounds, hull size, budget report, moderation (pending until a reviewed publish uses them) and a URL you can load now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceAssetsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_place_studio_script

> crate::models::UpdateWorldExperiencePlaceStudioScriptResponse update_world_experience_place_studio_script(experience_id, place_id)
Save your draft of a script (the place does not change until you commit). Returns lint and the diff from the current source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperiencePlaceStudioScriptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

