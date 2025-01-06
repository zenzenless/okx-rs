# \FundingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v5_asset_asset_valuation_get**](FundingsApi.md#api_v5_asset_asset_valuation_get) | **GET** /api/v5/asset/asset-valuation | 获取账户资产估值
[**api_v5_asset_balances_get**](FundingsApi.md#api_v5_asset_balances_get) | **GET** /api/v5/asset/balances | 获取资金账户余额
[**api_v5_asset_bills_get**](FundingsApi.md#api_v5_asset_bills_get) | **GET** /api/v5/asset/bills | 获取资金流水
[**api_v5_asset_currencies_get**](FundingsApi.md#api_v5_asset_currencies_get) | **GET** /api/v5/asset/currencies | 获取币种列表
[**api_v5_asset_transfer_post**](FundingsApi.md#api_v5_asset_transfer_post) | **POST** /api/v5/asset/transfer | 资金划转
[**api_v5_asset_transfer_state_get**](FundingsApi.md#api_v5_asset_transfer_state_get) | **GET** /api/v5/asset/transfer-state | 获取资金划转状态



## api_v5_asset_asset_valuation_get

> serde_json::Value api_v5_asset_asset_valuation_get(ccy)
获取账户资产估值

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 资产估值对应的单位<br>BTC 、USDT、USD 、CNY 、JPY、KRW、RUB、EUR、VND 、IDR 、INR、PHP、THB、TRY、AUD 、SGD 、ARS、SAR、AED、IQD<br>默认以`BTC`为单位的估值 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_asset_balances_get

> serde_json::Value api_v5_asset_balances_get(ccy)
获取资金账户余额

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 币种,支持多币种查询（不超过20个），币种之间半角逗号分隔 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_asset_bills_get

> serde_json::Value api_v5_asset_bills_get(ccy, r#type, after, before, limit)
获取资金流水

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ccy** | Option<**String**> | 币种，如 `BTC` |  |
**r#type** | Option<**String**> | 账单类型 |  |
**after** | Option<**String**> | 查询在此之前的内容，值为时间戳，Unix 时间戳为毫秒数格式，如 `1597026383085` |  |
**before** | Option<**String**> | 查询在此之后的内容，值为时间戳，Unix 时间戳为毫秒数格式，如 `1597026383085` |  |
**limit** | Option<**String**> | 分页返回的结果集数量，最大为100，不填默认返回100条 |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_asset_currencies_get

> serde_json::Value api_v5_asset_currencies_get()
获取币种列表

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_asset_transfer_post

> std::collections::HashMap<String, serde_json::Value> api_v5_asset_transfer_post(api_v5_asset_transfer_post_request)
资金划转

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_v5_asset_transfer_post_request** | [**ApiV5AssetTransferPostRequest**](ApiV5AssetTransferPostRequest.md) |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v5_asset_transfer_state_get

> serde_json::Value api_v5_asset_transfer_state_get(trans_id, r#type)
获取资金划转状态

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trans_id** | **String** | 划转ID | [required] |
**r#type** | Option<**String**> | 划转类型<br>`0`: 账户内划转<br>`1`: 母账户转子账户<br>`2`: 子账户转母账户<br>默认为`0` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

