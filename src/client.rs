use arc_swap::ArcSwapOption;
use openaction::set_global_settings;
use std::sync::{Arc, OnceLock};
use ytmd_companion_rs::{Client, ClientSettings};

use crate::current_settings;

pub fn ytmd_client() -> &'static ArcSwapOption<Client> {
	static CLIENT: OnceLock<ArcSwapOption<Client>> = OnceLock::new();
	CLIENT.get_or_init(|| ArcSwapOption::from(None))
}

pub(crate) async fn update_error(error: Option<&str>) {
	let current_guard = current_settings().load();

	if current_guard.error.as_deref() == error {
		return;
	}

	let mut updated_settings = (**current_guard).clone();
	updated_settings.error = error.map(|s| s.to_owned());

	if let Err(e) = set_global_settings(&updated_settings).await {
		log::error!("Failed to save error to global settings: {}", e);
	}

	current_settings().store(Arc::new(updated_settings));
}

pub(crate) async fn reinitialize() {
	let current_app_settings = current_settings().load();

	let client_settings = ClientSettings {
		app_id: current_app_settings.client_settings.app_id.clone(),
		app_name: current_app_settings.client_settings.app_name.clone(),
		app_version: current_app_settings.client_settings.app_version.clone(),
		host: current_app_settings.client_settings.host.clone(),
		port: current_app_settings.client_settings.port,
		token: current_app_settings.client_settings.token.clone(),
	};

	drop(current_app_settings);

	let client = Client::new(client_settings);

	if client.settings.load().token.is_some() {
		ytmd_client().store(Some(Arc::new(client)));
		return;
	}

	let code_response = match client.auth_request_code().await {
		Ok(res) => res,
		Err(e) => {
			log::error!("Failed to request auth code from YTMD: {}", e);
			update_error(Some(&format!(
				"Authentication initialization failed: {}",
				e
			)))
			.await;
			return;
		}
	};

	match client.auth_request(code_response.code).await {
		Ok(_) => {
			log::info!("YTMD Authentication successful!");

			let client_arc = Arc::new(client);
			ytmd_client().store(Some(client_arc.clone()));

			let current_guard = current_settings().load();
			let mut updated_settings = (**current_guard).clone();

			updated_settings.client_settings.token = client_arc.settings.load().token.clone();

			if let Err(e) = openaction::set_global_settings(&updated_settings).await {
				log::error!("Failed to persist token to OpenAction store: {}", e);
			}

			current_settings().store(Arc::new(updated_settings));
			update_error(None).await;
		}
		Err(e) => {
			log::error!("Failed to exchange authorization pin with YTMD: {}", e);
			update_error(Some(&format!("Authentication exchange failed: {}", e))).await;
		}
	}
}
