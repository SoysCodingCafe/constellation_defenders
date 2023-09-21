// Import Bevy game engine essentials
use bevy::{prelude::*, app::AppExit};
use bevy_kira_audio::{Audio, AudioControl};
// Import components, resources, and events
use crate::derivables::*;

// Plugin for generating the main menu
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Menu), (
				spawn_menu,
				generate_random_constellation,
			))
			.add_systems(Update,(
				flash_start_text,
				advance_menu,
			).run_if(in_state(GameState::Menu)))
		;
	}
}

fn spawn_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SpriteBundle{
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			texture: asset_server.load("sprites/title_screen.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));

	commands.spawn((
		SpriteBundle{
			transform: Transform::from_xyz(36.0, 18.0, 5.0),
			texture: asset_server.load("text/press_start.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(56.0, 8.0)),
				..default()
			},
			..default()
		},
		StartText,
		DespawnOnExitGameState,
	));
}

fn generate_random_constellation(
	mut level_layout: ResMut<LevelLayout>,
) {
	let mut random = Vec::new();
	if rand::random::<f32>() < 0.95 {
		let total_stars = rand::Rng::gen_range(&mut rand::thread_rng(), 1..9);
		for _ in 0..total_stars {
			random.push(Vec2::new(((rand::random::<f32>() - 0.5) * 120.0).round() + 0.5, ((rand::random::<f32>() - 0.5) * 80.0).round() + 0.5));
		}
	} else {
		// The Sun
		random.push(Vec2::new(0.5, 0.5));
	}

	level_layout.constellations[4] = random;
}

fn flash_start_text(
	time: Res<Time>,
	mut start_timer: ResMut<StartTimer>,
	mut start_text_query: Query<(&mut Visibility, With<StartText>)>, 
) {
	start_timer.0.tick(time.delta());
	if start_timer.0.just_finished() {
		for (mut visibility, _) in start_text_query.iter_mut() {
			if *visibility == Visibility::Visible {
				*visibility = Visibility::Hidden;
			} else {
				*visibility = Visibility::Visible;
			}
		}
	}
}

fn advance_menu(
	keyboard: Res<Input<KeyCode>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut ev_w_exit: EventWriter<AppExit>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	if keyboard.just_pressed(START_BUTTON) 
	|| keyboard.just_pressed(A_BUTTON) {
		audio.play(asset_server.load("sfx/ui_select.ogg")).with_volume(SFX_VOLUME);
		next_game_state.set(GameState::LevelSelect);
	} else if keyboard.just_pressed(B_BUTTON) {
		ev_w_exit.send(AppExit);
	}
}