/*
 * REST API
 *
 * # 使用说明   <b>该功能接口用户需先登陆，接口只会请求模拟环境</b><br><br>*Parameters* 面板中点击`Try it out`按钮，编辑请求参数，点击`Execute`按钮发送请求。*Responses* 面板中查看请求结果。<br>
 *
 * The version of the OpenAPI document: v5
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV5AssetTransferPostRequest {
    /// 必填<br>币种，如 `USDT`
    #[serde(rename = "ccy")]
    pub ccy: String,
    /// 必填<br>划转数量
    #[serde(rename = "amt")]
    pub amt: String,
    /// 必填<br>转出账户<br>`6`: 资金账户, `18`: 交易账户
    #[serde(rename = "from")]
    pub from: String,
    /// 必填<br>转入账户<br>`6`: 资金账户, `18`: 交易账户
    #[serde(rename = "to")]
    pub to: String,
    /// 可选<br>子账户名称，`type`为1或2：`subAcct` 为必填项
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// 划转类型<br>`0`: 账户内划转<br>`1`: 母账户转子账户(仅适用于母账户APIKey)<br>`2`: 子账户转母账户(仅适用于母账户APIKey)<br>默认为`0`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 是否支持跨币种保证金模式或组合保证金模式下的借币转入/转出<br>`true`或`false`，默认`false`
    #[serde(rename = "loanTrans", skip_serializing_if = "Option::is_none")]
    pub loan_trans: Option<bool>,
}

impl ApiV5AssetTransferPostRequest {
    pub fn new(ccy: String, amt: String, from: String, to: String) -> ApiV5AssetTransferPostRequest {
        ApiV5AssetTransferPostRequest {
            ccy,
            amt,
            from,
            to,
            sub_acct: None,
            r#type: None,
            loan_trans: None,
        }
    }
}

