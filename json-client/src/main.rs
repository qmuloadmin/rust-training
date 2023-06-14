#![allow(dead_code)]
use serde::de::DeserializeOwned;
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct SaleResponse {
    id: String,
    name: String,
    sale_code: String,
    is_hidden: bool,
    is_template: bool,
    status: Status,
    experience_type: ExperienceType,
    channel_type: ChannelType,
    #[serde(rename="type")]
    typ: Type
}

impl ResourceNamer for SaleResponse {
    fn resource_name() -> &'static str {
        "sales"
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Status {
    Archived,
    Closed,
    Fulfilled,
    Open,
    Ordered,
    Pending,
    Scheduled,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum ExperienceType {
    Legacy,
    Modern,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum ChannelType {
    AlwaysOpen,
    PopUp,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Type {
    Catalog,
    Group,
}

// { "sales": [{}] }
#[derive(Deserialize)]
struct ApiResponse {
    #[serde(flatten)]
    data: Value,
}

impl From<serde_json::Error> for ClientError {
    fn from(value: serde_json::Error) -> Self {
        ClientError {
            reason: ClientErrorReason::ParseError,
            message: "failed to parse json body".into(),
            cause: Some(Box::new(value)),
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(value: reqwest::Error) -> Self {
        ClientError {
            reason: ClientErrorReason::RequestError,
            message: "error".into(),
            cause: Some(Box::new(value)),
        }
    }
}

#[derive(Debug)]
struct ClientError {
    reason: ClientErrorReason,
    message: String,
    cause: Option<Box<dyn std::error::Error>>,
}

#[derive(Debug)]
enum ClientErrorReason {
    ParseError,
    ResponseCodeError,
    RequestError
}

trait FromApiResponse
where
    Self: Sized,
{
    fn from_response(res: ApiResponse) -> Result<Self, ClientError>;
}

trait ResourceNamer {
    fn resource_name() -> &'static str;
}

impl<Resource> FromApiResponse for Vec<Resource>
where
    Resource: ResourceNamer + DeserializeOwned,
    Self: Sized,
{
    fn from_response(res: ApiResponse) -> Result<Self, ClientError> {
        let resource = Resource::resource_name();
        match res.data[resource].to_owned() {
            Value::Array(values) => {
                let mut result: Vec<Resource> = Vec::with_capacity(values.len());
                for res in values.into_iter() {
                    result.push(serde_json::from_value(res)?);
                }
                Ok(result)
            }
            _ => Err(ClientError {
                reason: ClientErrorReason::ParseError,
                message: "unexpected json response structure".into(),
                cause: None,
            }),
        }
    }
}

struct ApiClient {
    client: reqwest::Client,
    token: String,
}

impl ApiClient {
    fn new(token: String) -> Self {
        ApiClient {
            client: reqwest::Client::new(),
            token,
        }
    }

    async fn find_all_resources<Resource: ResourceNamer + DeserializeOwned>(
        &self,
    ) -> Result<Vec<Resource>, ClientError> {
        let resource_name = Resource::resource_name();
        let res = self
            .client
            .get(format!(
                "https://api.ordermygear.com/tdo/{}/",
                resource_name
            ))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        if res.status() == 200 {
            let api_response: ApiResponse = res.json().await?;
            Ok(FromApiResponse::from_response(api_response)?)
        } else {
            Err(ClientError {
                message: format!("failure status code {}", res.status()),
                reason: ClientErrorReason::ResponseCodeError,
                cause: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let client = ApiClient::new("".into());
    let resources: Vec<SaleResponse> = client.find_all_resources().await?;
    for res in resources.into_iter() {
        println!("{:?}", res);
    }
    Ok(())
}