# GetConnectionHealthResponseAccounts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**platform_name** | **String** |  | 
**account_id** | Option<**String**> |  | [optional]
**account_slot** | Option<**f32**> |  | [optional]
**label** | Option<**String**> |  | [optional]
**platform_username** | Option<**String**> |  | [optional]
**summary** | **String** | Plain-English \"what is true\" + \"what to do\". Never empty. | 
**action** | **String** |  | 
**liveness** | [**crate::models::GetConnectionHealthResponseLiveness**](GetConnectionHealthResponse_liveness.md) |  | 
**browser** | Option<[**crate::models::GetConnectionHealthResponseBrowser**](GetConnectionHealthResponse_browser.md)> |  | [optional]
**platform** | **String** |  | 
**state** | **String** |  | 
**severity** | **String** |  | 
**audience** | **String** |  | 
**anchors** | Option<[**crate::models::GetConnectionHealthResponseAnchors**](GetConnectionHealthResponse_anchors.md)> |  | [optional]
**notes** | **Vec<String>** | Secondary observations that do not change the verdict but change the debugging. Always safe to show; never the only thing shown. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


