/*!
 * Configuration of the client and actions.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiplesApiParams {
    pub configurations: Vec<ApiParams>,
}

/// Accepted parameters to create a client and its respective requests as well as to manage the resolved information.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiParams {
    pub name: Option<String>,
    pub method_request: MethodRequest,
    pub url: String,
    pub authentication: Option<String>,
    pub params_request: Vec<ParamRequest>,
    pub get_value_json: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParamRequest {
    InUri(String),
    InBody(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MethodRequest {
    Get = 0,
    Post = 1,
}
