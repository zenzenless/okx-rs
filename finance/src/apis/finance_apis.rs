use crate::apis::configuration;
use crate::apis::ResponseContent;
use crate::models::savings_balance::SavingsBalanceResponse;
use crate::models::purchase_redempt::{PurchaseRedemptRequest, PurchaseRedemptResponse};

pub async fn purchase_redempt(
    configuration: &configuration::Configuration,
    purchase_redempt_request: PurchaseRedemptRequest,
) -> Result<ResponseContent<PurchaseRedemptResponse>, anyhow::Error> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v5/finance/savings/purchase-redempt",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&purchase_redempt_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: PurchaseRedemptResponse =
            serde_json::from_str(&local_var_content)?;
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: Some(local_var_entity),
        };
        Ok(local_var_result)
    } else {
        Err(anyhow::anyhow!(local_var_content))
    }
}

pub async fn get_savings_balance(
    configuration: &configuration::Configuration,
    ccy: Option<String>,
) -> Result<ResponseContent<SavingsBalanceResponse>, anyhow::Error> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v5/finance/savings/balance",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ccy {
        local_var_req_builder =
            local_var_req_builder.query(&[("ccy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: SavingsBalanceResponse =
            serde_json::from_str(&local_var_content)?;
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: Some(local_var_entity),
        };
        Ok(local_var_result)
    } else {
        Err(anyhow::anyhow!(local_var_content))
    }
}


#[derive(Debug)]
pub struct SavingsLendingHistoryError;

impl std::error::Error for SavingsLendingHistoryError {}

impl std::fmt::Display for SavingsLendingHistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred.")
    }
}