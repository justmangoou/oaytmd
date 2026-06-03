use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
	Result,
	models::{
		request::{AuthCodeRequest, AuthRequest, CommandRequest},
		response::{AuthCodeResponse, AuthResponse, StateResponse},
	},
};

mod rest;
mod socket;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ClientSettings {
	pub app_id: String,
	pub app_name: String,
	pub app_version: String,
	pub host: String,
	pub port: u16,
	pub token: Option<String>, // Token added back
}

impl ClientSettings {
	pub fn base_url(&self) -> String {
		format!("http://{}:{}/api/v1", self.host, self.port)
	}
}

pub struct Client {
	pub settings: ArcSwap<ClientSettings>,
	rest: ArcSwap<rest::RestClient>,
}

impl Client {
	pub fn new(settings: ClientSettings) -> Self {
		let rest_client = rest::RestClient::new(settings.base_url());

		Self {
			rest: ArcSwap::from_pointee(rest_client),
			settings: ArcSwap::from_pointee(settings),
		}
	}

	pub fn set_settings(&self, new_settings: ClientSettings) {
		let new_rest_client = rest::RestClient::new(new_settings.base_url());

		self.settings.store(Arc::new(new_settings));
		self.rest.store(Arc::new(new_rest_client));
	}

	pub async fn get_state(&self) -> Result<StateResponse> {
		let settings = self.settings.load();
		let rest = self.rest.load();

		rest.get("/state", settings.token.as_deref()).await
	}

	pub async fn auth_request(&self, code: String) -> Result<AuthResponse> {
		let current_settings = self.settings.load();
		let rest = self.rest.load();

		let response = rest
			.post::<AuthRequest, AuthResponse>(
				"/auth/request",
				&AuthRequest {
					app_id: current_settings.app_id.clone(),
					code,
				},
				current_settings.token.as_deref(),
			)
			.await?
			.ok_or_else(|| crate::Error::UnexpectedResponse("No auth token received".into()))?;

		let mut new_settings = (**current_settings).clone();
		new_settings.token = Some(response.token.clone());

		self.settings.store(Arc::new(new_settings));

		Ok(response)
	}

	pub async fn auth_request_code(&self) -> Result<AuthCodeResponse> {
		let settings = self.settings.load();
		let rest = self.rest.load();

		rest.post::<AuthCodeRequest, AuthCodeResponse>(
			"/auth/requestcode",
			&AuthCodeRequest {
				app_id: settings.app_id.clone(),
				app_name: settings.app_name.clone(),
				app_version: settings.app_version.clone(),
			},
			settings.token.as_deref(),
		)
		.await?
		.ok_or_else(|| crate::Error::UnexpectedResponse("No auth code received".into()))
	}

	pub async fn send_command(&self, command: &CommandRequest) -> Result<()> {
		let settings = self.settings.load();
		let rest = self.rest.load();

		rest.post::<_, serde_json::Value>("/command", command, settings.token.as_deref())
			.await?;

		Ok(())
	}
}
