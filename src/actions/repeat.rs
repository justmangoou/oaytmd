use openaction::{Action, Instance, OpenActionResult, async_trait};
use serde::{Deserialize, Serialize};
use ytmd_companion_rs::models::{RepeatMode, request::CommandRequest};

use crate::actions::send_command;

async fn set_new_state(instance: &Instance, mode: &RepeatMode) -> OpenActionResult<()> {
	let new_state = match mode {
		RepeatMode::Unknown | RepeatMode::None => 0,
		RepeatMode::All => 1,
		RepeatMode::One => 2,
	};

	instance.set_state(new_state).await
}

#[derive(Serialize, Deserialize, Default)]
pub struct RepeatActionSettings {
	pub mode: RepeatMode,
}

pub struct RepeatAction;

#[async_trait]
impl Action for RepeatAction {
	const UUID: &'static str = "justmangoou.oaytmd.repeat";
	type Settings = RepeatActionSettings;

	async fn did_receive_settings(
		&self,
		instance: &Instance,
		settings: &Self::Settings,
	) -> OpenActionResult<()> {
		set_new_state(instance, &settings.mode).await
	}

	async fn will_appear(
		&self,
		_instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		Ok(())
	}

	async fn will_disappear(
		&self,
		_instance: &Instance,
		_settings: &Self::Settings,
	) -> OpenActionResult<()> {
		Ok(())
	}

	async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
		let new_mode = match settings.mode {
			RepeatMode::Unknown => RepeatMode::None,
			RepeatMode::None => RepeatMode::All,
			RepeatMode::All => RepeatMode::One,
			RepeatMode::One => RepeatMode::None,
		};

		match send_command(instance, &CommandRequest::RepeatMode(new_mode.clone())).await {
			Ok(_) => {
				set_new_state(instance, &new_mode).await?;
				instance
					.set_settings(&RepeatActionSettings { mode: new_mode })
					.await?;
			}
			Err(error) => {
				log::error!("Failed to send repeat mode command: {}", error);
				instance.show_alert().await?;
			}
		}

		Ok(())
	}
}
