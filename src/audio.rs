use std::time::Duration;

// Import Bevy game engine essentials
use bevy::prelude::*;
// Import Kira audio for Bevy to handle loading sound files
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};
// Import components, resources, and events
use crate::derivables::*;

// Plugin for background music and sound effects
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
		//.add_systems(Startup, (
		//	play_bgm,
		//))
		.add_systems(OnEnter(GameState::Menu), (
			switch_song,
		))
		.add_systems(OnEnter(GameState::Loading), (
			switch_song,
		))
		.add_systems(OnEnter(GameState::Level), (
			switch_song,
		))
		.add_systems(OnEnter(GameState::Win), (
			switch_song,
		))
		;
	}
}

fn play_bgm(
	mut commands: Commands,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
) {
	let bgm_handle = audio
		.play(asset_server.load("bgm/menu_theme.ogg"))
		.looped()
		.with_volume(BGM_VOLUME)
		.handle();

	//commands.insert_resource(BgmHandle(bgm_handle));
}

fn switch_song(
	current_state: Res<State<GameState>>,
	current_level: Res<SelectedLevel>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	win_state: Res<WinState>,
	//bgm_handle: Res<BgmHandle>,
	//mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	//if let Some(instance) = audio_instances.get_mut(&bgm_handle.0) {
		audio.stop().fade_out(AudioTween::default());
		match current_state.get() {
			GameState::Menu => {
				audio
					.play(asset_server.load("bgm/menu_theme.ogg"))
					.looped()
					.with_volume(BGM_VOLUME);
			},
			GameState::Loading => {
				audio
					.play(asset_server.load("bgm/loading_theme.ogg"))
					.loop_from(19.2)
					.with_volume(BGM_VOLUME);
			},
			GameState::Level => {
				match current_level.0 {
					0 => {audio
						.play(asset_server.load("bgm/cassiopeia.ogg"))
						.looped()
						.with_volume(BGM_VOLUME);},
					1 => {audio
						.play(asset_server.load("bgm/cepheus.ogg"))
						.looped()
						.with_volume(BGM_VOLUME);},
					2 => {audio
						.play(asset_server.load("bgm/ursa_minor.ogg"))
						.looped()
						.with_volume(BGM_VOLUME);},
					3 => {audio
						.play(asset_server.load("bgm/orion.ogg"))
						.looped()
						.with_volume(BGM_VOLUME);},
					4 => {audio
						.play(asset_server.load("bgm/random.ogg"))
						.looped()
						.with_volume(BGM_VOLUME);},
					5 => {audio
						.play(asset_server.load("bgm/endless.ogg"))
						.looped()
						.with_volume(BGM_VOLUME);},
					_ => (),
				}
			},
			GameState::Win => {
				if win_state.0 == 0 || win_state.0 == 1 {
					audio
						.play(asset_server.load("bgm/lose_theme.ogg"))
						.with_volume(BGM_VOLUME);
				} else {
					audio
						.play(asset_server.load("bgm/win_theme.ogg"))
						.with_volume(BGM_VOLUME);
				}
			},
			_ => (),
		}
	//}
}