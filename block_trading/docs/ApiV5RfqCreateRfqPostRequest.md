# ApiV5RfqCreateRfqPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**counterparties** | **Vec<String>** | 报价方列表。 | 
**anonymous** | Option<**bool**> | 是否匿名询价，true表示匿名询价，false表示公开询价，默认值为 false，为true时，即使在交易执行之后，身份也不会透露给报价方。 | [optional]
**cl_rfq_id** | Option<**String**> | 询价单自定义ID，字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。 | [optional]
**allow_partial_execution** | Option<**bool**> | RFQ是否可以被部分执行，如果腿的比例和原RFQ一致。有效值为true或false。默认为false。 | [optional]
**legs** | **Vec<String>** | 组合交易，每次最多可以提交15组交易信息 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


