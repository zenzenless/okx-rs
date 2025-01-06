# ApiV5AccountSimulatedMarginPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**inst_type** | Option<**String**> | 产品类型<br>`SWAP`：永续合约，`FUTURES`：交割合约，`OPTION`：期权 | [optional]
**incl_real_pos** | Option<**String**> | 是否代入已有仓位<br>`true`：调整被代入的已有仓位信息<br>`false`：不代入已有仓位，仅使用simPos里新增的模拟仓位进行计算,默认为True | [optional]
**sim_pos** | Option<**String**> | 调整持仓列表 | [optional]
**greater_than__inst_id** | Option<**String**> | 交易产品ID | [optional]
**greater_than__pos** | Option<**String**> | 持仓量 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


