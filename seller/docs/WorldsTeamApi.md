# \WorldsTeamApi

All URIs are relative to *https://crossly.net/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_world_collaboration_accept**](WorldsTeamApi.md#create_world_collaboration_accept) | **POST** /v1/worlds/collaborations/{experienceId}/accept | Accept an invitation.
[**create_world_collaboration_decline**](WorldsTeamApi.md#create_world_collaboration_decline) | **POST** /v1/worlds/collaborations/{experienceId}/decline | Decline an invitation.
[**create_world_experience_activity_revert**](WorldsTeamApi.md#create_world_experience_activity_revert) | **POST** /v1/worlds/experiences/{experienceId}/activity/{activityId}/revert | Revert an instance change: its undo is applied as a new edit by you; anything changed since by others is left alone.
[**create_world_experience_collaborator**](WorldsTeamApi.md#create_world_experience_collaborator) | **POST** /v1/worlds/experiences/{experienceId}/collaborators | Invite a Crossly handle with a role. Owner or admin; only the owner makes admins. Counts against the owner's plan cap.
[**create_world_experience_place_collab_op**](WorldsTeamApi.md#create_world_experience_place_collab_op) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab/ops | Apply a batch of ops (insert, delete, move, prop, attr, name, tags, world) in server order. Each op is checked against your role; a retried batch id is answered, not re-applied.
[**create_world_experience_place_collab_save**](WorldsTeamApi.md#create_world_experience_place_collab_save) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab/save | Write the session into the place draft now (publishing and cloud tests do this themselves).
[**create_world_experience_place_collab_script_commit**](WorldsTeamApi.md#create_world_experience_place_collab_script_commit) | **POST** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab/scripts/{nodeId}/commit | Commit your draft with a three-way merge. 409 merge_conflict carries the merged text with conflict blocks and the current source.
[**delete_world_experience_collaborator**](WorldsTeamApi.md#delete_world_experience_collaborator) | **DELETE** /v1/worlds/experiences/{experienceId}/collaborators/{userId} | Remove a collaborator, or leave (your own user id). Their revenue share returns to the owner from next month.
[**delete_world_experience_place_collab_script**](WorldsTeamApi.md#delete_world_experience_place_collab_script) | **DELETE** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab/scripts/{nodeId} | Discard your draft of a script.
[**get_world_experience_place_collab**](WorldsTeamApi.md#get_world_experience_place_collab) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab | The live co-editing document of a place (node ids every editor shares), its op sequence and your role.
[**get_world_experience_place_collab_op**](WorldsTeamApi.md#get_world_experience_place_collab_op) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab/ops | Ops after a sequence number, grouped by batch. 409 resync when they are no longer kept: reload the session.
[**get_world_experience_split**](WorldsTeamApi.md#get_world_experience_split) | **GET** /v1/worlds/experiences/{experienceId}/splits | Revenue split of the creator share: current, scheduled, history.
[**get_world_experience_team**](WorldsTeamApi.md#get_world_experience_team) | **GET** /v1/worlds/experiences/{experienceId}/team | The team: owner, collaborators with roles (owner, admin, editor, scripter, builder, tester, analyst), your role and permissions, the plan cap.
[**list_world_collaborations**](WorldsTeamApi.md#list_world_collaborations) | **GET** /v1/worlds/collaborations | Experiences you are invited to or work on, with your role.
[**list_world_experience_activity**](WorldsTeamApi.md#list_world_experience_activity) | **GET** /v1/worlds/experiences/{experienceId}/activity | Activity history, newest first: who changed what, when, from where (studio, api, mcp, cli); instance changes are revertible.
[**list_world_experience_place_collab_scripts**](WorldsTeamApi.md#list_world_experience_place_collab_scripts) | **GET** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab/scripts | Your script drafts in this place, with whether someone committed since you started.
[**update_world_experience_collaborator**](WorldsTeamApi.md#update_world_experience_collaborator) | **PATCH** /v1/worlds/experiences/{experienceId}/collaborators/{userId} | Change a collaborator's role.
[**update_world_experience_place_collab_script**](WorldsTeamApi.md#update_world_experience_place_collab_script) | **PUT** /v1/worlds/experiences/{experienceId}/places/{placeId}/collab/scripts/{nodeId} | Save your own draft of a script (needs edit scripts).
[**update_world_experience_split**](WorldsTeamApi.md#update_world_experience_split) | **PUT** /v1/worlds/experiences/{experienceId}/splits | Owner only: split the creator share (70% of net) in basis points summing to 10000. Takes effect the 1st of next month; every party is notified. The 70% total never changes.



## create_world_collaboration_accept

> crate::models::CreateWorldCollaborationAcceptResponse create_world_collaboration_accept(experience_id)
Accept an invitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldCollaborationAcceptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_collaboration_decline

> crate::models::CreateWorldCollaborationDeclineResponse create_world_collaboration_decline(experience_id)
Decline an invitation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldCollaborationDeclineResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_activity_revert

> crate::models::CreateWorldExperienceActivityRevertResponse create_world_experience_activity_revert(experience_id, activity_id)
Revert an instance change: its undo is applied as a new edit by you; anything changed since by others is left alone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**activity_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceActivityRevertResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_collaborator

> crate::models::CreateWorldExperienceCollaboratorResponse create_world_experience_collaborator(experience_id)
Invite a Crossly handle with a role. Owner or admin; only the owner makes admins. Counts against the owner's plan cap.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceCollaboratorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_collab_op

> crate::models::CreateWorldExperiencePlaceCollabOpResponse create_world_experience_place_collab_op(experience_id, place_id)
Apply a batch of ops (insert, delete, move, prop, attr, name, tags, world) in server order. Each op is checked against your role; a retried batch id is answered, not re-applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceCollabOpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_collab_save

> crate::models::CreateWorldExperiencePlaceCollabSaveResponse create_world_experience_place_collab_save(experience_id, place_id)
Write the session into the place draft now (publishing and cloud tests do this themselves).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceCollabSaveResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## create_world_experience_place_collab_script_commit

> crate::models::CreateWorldExperiencePlaceCollabScriptCommitResponse create_world_experience_place_collab_script_commit(experience_id, place_id, node_id)
Commit your draft with a three-way merge. 409 merge_conflict carries the merged text with conflict blocks and the current source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperiencePlaceCollabScriptCommitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_collaborator

> crate::models::DeleteSpatialSceneGrantResponse delete_world_experience_collaborator(experience_id, user_id)
Remove a collaborator, or leave (your own user id). Their revenue share returns to the owner from next month.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteSpatialSceneGrantResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## delete_world_experience_place_collab_script

> crate::models::DeleteWorldExperiencePlaceCollabScriptResponse delete_world_experience_place_collab_script(experience_id, place_id, node_id)
Discard your draft of a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

**crate::models::DeleteWorldExperiencePlaceCollabScriptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_collab

> crate::models::GetWorldExperiencePlaceCollabResponse get_world_experience_place_collab(experience_id, place_id)
The live co-editing document of a place (node ids every editor shares), its op sequence and your role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceCollabResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_place_collab_op

> crate::models::GetWorldExperiencePlaceCollabOpResponse get_world_experience_place_collab_op(experience_id, place_id)
Ops after a sequence number, grouped by batch. 409 resync when they are no longer kept: reload the session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperiencePlaceCollabOpResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_split

> crate::models::GetWorldExperienceSplitResponse get_world_experience_split(experience_id)
Revenue split of the creator share: current, scheduled, history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceSplitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## get_world_experience_team

> crate::models::GetWorldExperienceTeamResponse get_world_experience_team(experience_id)
The team: owner, collaborators with roles (owner, admin, editor, scripter, builder, tester, analyst), your role and permissions, the plan cap.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceTeamResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_collaborations

> crate::models::ListWorldCollaborationsResponse list_world_collaborations()
Experiences you are invited to or work on, with your role.

### Parameters

This endpoint does not need any parameter.

### Return type

**crate::models::ListWorldCollaborationsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_activity

> crate::models::ListWorldExperienceActivityResponse list_world_experience_activity(experience_id)
Activity history, newest first: who changed what, when, from where (studio, api, mcp, cli); instance changes are revertible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperienceActivityResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## list_world_experience_place_collab_scripts

> crate::models::ListWorldExperiencePlaceCollabScriptsResponse list_world_experience_place_collab_scripts(experience_id, place_id)
Your script drafts in this place, with whether someone committed since you started.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |

### Return type

**crate::models::ListWorldExperiencePlaceCollabScriptsResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_collaborator

> crate::models::CreateWorldExperienceCollaboratorResponse update_world_experience_collaborator(experience_id, user_id)
Change a collaborator's role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

**crate::models::CreateWorldExperienceCollaboratorResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_place_collab_script

> crate::models::UpdateWorldExperiencePlaceCollabScriptResponse update_world_experience_place_collab_script(experience_id, place_id, node_id)
Save your own draft of a script (needs edit scripts).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |
**place_id** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

**crate::models::UpdateWorldExperiencePlaceCollabScriptResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


## update_world_experience_split

> crate::models::GetWorldExperienceSplitResponse update_world_experience_split(experience_id)
Owner only: split the creator share (70% of net) in basis points summing to 10000. Takes effect the 1st of next month; every party is notified. The 70% total never changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experience_id** | **String** |  | [required] |

### Return type

**crate::models::GetWorldExperienceSplitResponse**

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

