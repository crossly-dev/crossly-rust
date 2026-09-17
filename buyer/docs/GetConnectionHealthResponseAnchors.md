# GetConnectionHealthResponseAnchors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expected** | **Vec<String>** | What we were looking for. Empty ⇒ this platform is unmonitored. | 
**observations** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**present** | **Vec<String>** | Name found carrying a non-empty value — the only honest \"logged in\". | 
**empty** | **Vec<String>** | Name found, value is the empty string. The Whatnot class. | 
**missing** | **Vec<String>** | Name not in the jar at all. | 
**cookie_count** | **f32** |  | 
**observed_cookie_names** | **Vec<String>** | Cookie names actually in the jar, truncated. This is the payload that turns \"anchors missing\" into a diagnosis: if the jar holds 30 cookies and none are ours, a rename is the likely story; if it holds three device cookies, the browser is signed out. NAMES ONLY — never values. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


