# ApiV5TradeClosePositionPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_id** | **String** | 必填<br>产品ID，如：`BTC-USDT-SWAP` | 
**mgn_mode** | **String** | 必填<br>保证金模式。全仓：`cross`；逐仓：`isolated` | 
**pos_side** | Option<**String**> | 可选<br>持仓方向<br>单向持仓模式下：可不填写此参数，默认值`net`，如果填写，仅可以填写`net`<br>双向持仓模式下： 必须填写此参数，且仅可以填写 `long`：平多 ，`short`：平空 | [optional]
**ccy** | Option<**String**> | 可选<br>保证金币种，如：`USDT`。单币种保证金模式的全仓币币杠杆平仓必填 | [optional]
**auto_cxl** | Option<**bool**> | 否<br>当市价全平时，平仓单是否需要自动撤销,默认为false.`false`：不自动撤单 `true`：自动撤单 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


