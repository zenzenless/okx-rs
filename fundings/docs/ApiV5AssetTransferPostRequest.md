# ApiV5AssetTransferPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ccy** | **String** | 必填<br>币种，如 `USDT` | 
**amt** | **String** | 必填<br>划转数量 | 
**from** | **String** | 必填<br>转出账户<br>`6`: 资金账户, `18`: 交易账户 | 
**to** | **String** | 必填<br>转入账户<br>`6`: 资金账户, `18`: 交易账户 | 
**sub_acct** | Option<**String**> | 可选<br>子账户名称，`type`为1或2：`subAcct` 为必填项 | [optional]
**r#type** | Option<**String**> | 划转类型<br>`0`: 账户内划转<br>`1`: 母账户转子账户(仅适用于母账户APIKey)<br>`2`: 子账户转母账户(仅适用于母账户APIKey)<br>默认为`0` | [optional]
**loan_trans** | Option<**bool**> | 是否支持跨币种保证金模式或组合保证金模式下的借币转入/转出<br>`true`或`false`，默认`false` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


