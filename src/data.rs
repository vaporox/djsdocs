use serde::Deserialize;

#[derive(Deserialize)]
pub struct ParameterData {
	pub name: String,
	pub description: String,
	pub r#type: String,
}

#[derive(Deserialize)]
pub struct ReturnData {
	pub r#type: String,
}

#[derive(Deserialize)]
pub struct APIData {
	pub name: String,
	pub description: String,
	pub internal_type: String,
	pub parent: Option<String>,
	pub props: Option<Vec<String>>,
	pub methods: Option<Vec<String>>,
	pub events: Option<Vec<String>>,
	pub r#type: Option<String>,
	pub params: Option<Vec<ParameterData>>,
	pub returns: Option<ReturnData>,
}

#[derive(Deserialize)]
pub struct APIError {
	pub message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum APIResponse {
	Data(APIData),
	Error(APIError),
}

impl TryFrom<APIResponse> for APIData {
	type Error = String;

	fn try_from(data: APIResponse) -> Result<Self, Self::Error> {
		match data {
			APIResponse::Data(data) => Ok(data),
			APIResponse::Error(error) => Err(error.message),
		}
	}
}
