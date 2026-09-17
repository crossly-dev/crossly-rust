# GetConnectionHealthResponseLiveness

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_browser_push_at** | Option<**String**> | The clean \"the browser pushed cookies\" signal. | [optional]
**last_browser_push_ago** | **String** |  | 
**last_synced_at** | Option<**String**> | Also stamped by the executor on any successful server-side call, so it is NOT evidence the extension is alive. Exposed for debugging only. | [optional]
**last_used_at** | Option<**String**> | Last server-exec attempt, success or fail. | [optional]
**last_used_ago** | **String** |  | 
**auth_failure_streak** | **f32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


