# CreateMagicScanSynthesizeResponsePayloadGrading

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grader_slug** | **String** |  | 
**grade** | **String** | As the grader writes it: '10', '9.8', 'MS-65'. | 
**qualifier** | Option<**String**> |  | [optional]
**seal_grade** | Option<**String**> | Second axis on a `dual` scale — WATA's seal grade. | [optional]
**cert_number** | Option<**String**> |  | [optional]
**grade_key** | **String** | Derived. Never assign by hand — call `gradeKey()`. | 
**verified_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


