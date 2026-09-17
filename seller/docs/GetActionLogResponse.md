# GetActionLogResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**created_at** | **String** |  | 
**user_id** | Option<**String**> |  | [optional]
**status** | **String** |  | 
**source** | **String** |  | 
**platform** | Option<**String**> |  | [optional]
**action** | **String** |  | 
**latency_ms** | Option<**f32**> |  | [optional]
**error_class** | Option<**String**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**ip_address** | Option<**String**> |  | [optional]
**user_agent** | Option<**String**> |  | [optional]
**oauth_app_id** | Option<**String**> |  | [optional]
**category** | **String** |  | 
**actor_user_id** | Option<**String**> |  | [optional]
**finished_at** | Option<**String**> |  | [optional]
**correlation_id** | **String** |  | 
**track** | Option<**String**> |  | [optional]
**target_type** | Option<**String**> |  | [optional]
**target_id** | Option<**String**> |  | [optional]
**http_status** | Option<**f32**> |  | [optional]
**actor_email** | Option<**String**> | Resolved from actorUserId so the UI can say \"Jane relisted this\" rather than printing a UUID. Null for worker/system actions, which genuinely had no human actor. | [optional]
**actor_display_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


