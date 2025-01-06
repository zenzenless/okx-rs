# ApiV5RfqMakerInstrumentSettingsPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_type** | **String** | 产品类别，枚举值包括 FUTURES, OPTION, SWAP 和 SPOT。 | 
**include_all** | Option<**bool**> | 是否接收该instType下所有产品。有效值为true或false。默认false。 | [optional]
**data** | [**models::ArrayOfObjects**](array of objects.md) | instType的元素 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


