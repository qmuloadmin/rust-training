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
    let client = ApiClient::new("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJkYXRhIjp7ImFjdGl2ZV9ncm91cCI6eyJoYXNfY2hpbGRyZW4iOmZhbHNlLCJpZCI6IjEiLCJuYW1lIjoiT3JkZXJNeUdlYXIiLCJwYXJlbnQiOm51bGx9LCJhc3N1bWVkX3VzZXIiOnsiZW1haWwiOlt7ImFkZHJlc3MiOiJ6YWNoQG9yZGVybXlnZWFyLmNvbSJ9XSwiZmlyc3RfbmFtZSI6IlphY2hhcnkiLCJoYXNNdWx0aXBsZUdyb3VwcyI6ZmFsc2UsImlkIjoiMTIwMDkiLCJpc0NvbnRyYWN0b3IiOmZhbHNlLCJpc09tZyI6dHJ1ZSwibGFzdF9uYW1lIjoiQnVsbG91Z2gifSwiYXV0aGVudGljYXRlZF91c2VyIjp7ImVtYWlsIjpbeyJhZGRyZXNzIjoiemFjaEBvcmRlcm15Z2Vhci5jb20ifV0sImZpcnN0X25hbWUiOiJaYWNoYXJ5IiwiaGFzTXVsdGlwbGVHcm91cHMiOmZhbHNlLCJpZCI6IjEyMDA5IiwiaXNDb250cmFjdG9yIjpmYWxzZSwiaXNPbWciOnRydWUsImxhc3RfbmFtZSI6IkJ1bGxvdWdoIn0sInRva2VuIjoibmNNMXQxRE51b24tYUtCY05UOXpXbnlnckYxZUprWEhFWnhrNFVMMSJ9LCJleHAiOjE2ODY4MDgxMTV9.Zj4Q90QhEMBeAZd9VRlGUKffoIjiZFh2J30GsDa8FQfpTJDv_2iC-96WYy-BtkFdwQJxXY3sI_NZ6Qu5olozBxattghZJYVGFwji_HsjBC8ZtPyu96SfaPtxaefkuvBTdl2uSOI-c0uKPLRrkxHCCS1RtF4216idtDG77I1tkDEHtoFcrwaLmeiDqYVBMmLYq0tE_50DG-JIO1s2rzIP0TpIp6A7KipsqQdq-pmOqZWaNebmA4XoFk0L2gG-nrL5FEHByNiSR-8HqYjXtXklWrF6dVAVMntzlVpDY_rWdpeWhYiLqmZZ9CRhMxeoUN9DRZz9y7nmi7MmuAm0lGeduw".into());
    let resources: Vec<SaleResponse> = client.find_all_resources().await?;
    for res in resources.into_iter() {
        println!("{:?}", res);
    }
    Ok(())
}