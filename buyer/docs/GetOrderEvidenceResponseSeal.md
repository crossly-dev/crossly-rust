# GetOrderEvidenceResponseSeal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assessment** | [**crate::models::GetOrderEvidenceResponseSealAssessment**](GetOrderEvidenceResponse_seal_assessment.md) |  | 
**dispatch_searched_frames** | Option<**f32**> | How many packing frames were searched to find the dispatch frame shot closest to the arrival angle. Null when the dispatch reading was simply the frame the recorder ended on.  Surfaced rather than kept internal: \"the best of eighteen frames matched\" is a weaker claim than \"the frame we took matched\", and a reviewer has to be able to tell them apart. See frame-match.ts. | [optional]
**has_dispatch** | **bool** |  | 
**has_arrival** | **bool** |  | 
**has_courier_photo** | **bool** | A courier photo exists, even if the seal was not legible in it. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


