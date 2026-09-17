# GetAutomationRuleExportByIdResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**action** | [**crate::models::GetAutomationRuleExportResponseAction**](GetAutomationRuleExportResponse_action.md) |  | 
**trigger** | [**crate::models::GetAutomationRuleExportResponseAction**](GetAutomationRuleExportResponse_action.md) |  | 
**schema_version** | **f32** |  | 
**condition** | Option<[**crate::models::GetAutomationRuleExportResponseCondition**](GetAutomationRuleExportResponse_condition.md)> |  | [optional]
**metadata** | Option<[**crate::models::GetAutomationRuleExportResponseMetadata**](GetAutomationRuleExportResponse_metadata.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**platforms** | Option<**Vec<String>**> |  | [optional]
**schema** | Option<**String**> | Optional reference to the public schema URL — purely cosmetic. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


