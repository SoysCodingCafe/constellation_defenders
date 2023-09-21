// Import Bevy game engine essentials
use bevy::{prelude::*, window::WindowResolution};

// MODULES
mod audio;
mod enemy;
mod level;
mod loading;
mod mech;
mod menu;
mod post_processing;
mod select;
mod setup;
mod win;

mod derivables;

// Only include in debug builds
#[cfg(debug_assertions)]
mod debug;

// Can't forget main!
fn main() {
	// Create app to hold all our plugins, resources, events, and systems
	let mut app = App::new();
	let mut resolution = WindowResolution::new(160.0, 144.0);
	resolution.set_physical_resolution(800, 720);
	app.insert_resource(Msaa::Off);
	app
		// Default plugins provided by Bevy handles all essentials for a game
		// such as the game window, asset management, input handling, and time
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					resolution: resolution,
					resizable: false,
					decorations: false,
					position: WindowPosition::Centered(MonitorSelection::Primary),
					// Stops the game from stopping keyboard shortcuts e.g. F12
					//prevent_default_event_handling: false,
					// Default to Borderless Fullscreen
					//mode: bevy::window::WindowMode::BorderlessFullscreen,
					// Set custom window title
					title: "Constellation Defenders".to_string(),
					..default()
				}),
				..default()
			})
			// Prevents pixel art sprites from becoming blurry
			.set(ImagePlugin::default_nearest())
		)

		// Plugins
		.add_plugins((
			audio::AudioPlugin,
			// Kira audio plugin for Bevy for playing sound files
			bevy_kira_audio::AudioPlugin,
			enemy::EnemyPlugin,
			level::LevelPlugin,
			loading::LoadingPlugin,
			mech::MechPlugin,
			menu::MenuPlugin,
			post_processing::PostProcessingPlugin,
			select::SelectPlugin,
			setup::SetupPlugin,
			win::WinPlugin,
		))
		;

	{
		// Only include in debug builds
		#[cfg(debug_assertions)]
		app
			// Debug module for dev tools
			.add_plugins(debug::DebugPlugin)
		;
	}

	app.run();
}