// Import Bevy game engine essentials
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
// Import components, resources, and events
use crate::derivables::*;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::LevelSelect), (
				spawn_select,
			))
			.add_systems(Update, (
				navigate_select,
			).run_if(in_state(GameState::LevelSelect)))
		;
	}
}

fn spawn_select(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut selected_level: ResMut<SelectedLevel>,
	asset_server: Res<AssetServer>,
) {
	selected_level.0 = 0;
	
	commands
		.spawn((SpriteBundle {
			texture: asset_server.load("sprites/level_select.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));

	commands
		.spawn((SpriteSheetBundle {
			transform: Transform::from_xyz(0.0, 0.0, 50.0),
			texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/select_highlight.png"), Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT), 3, 2, None, None)).clone(),
			sprite: TextureAtlasSprite{
				index: 0,
				custom_size: Some(Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT)),
				..default()
			},
			..default()
		},
		SelectHighlight,
		DespawnOnExitGameState,
	));
}

fn navigate_select(
	keyboard: Res<Input<KeyCode>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut endless: ResMut<Endless>,
	mut selected_level: ResMut<SelectedLevel>,
	mut highlight_query: Query<(&mut TextureAtlasSprite, With<SelectHighlight>)>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	if keyboard.just_pressed(UP_BUTTON) {
		selected_level.0 = (selected_level.0 + 3) % 6;
	} else if keyboard.just_pressed(DOWN_BUTTON) {
		selected_level.0 = (selected_level.0 + 3) % 6;
	} else if keyboard.just_pressed(LEFT_BUTTON) {
		selected_level.0 = (selected_level.0 + 5) % 6;
	} else if keyboard.just_pressed(RIGHT_BUTTON) {
		selected_level.0 = (selected_level.0 + 1) % 6;
	}

	for (mut sprite, _) in highlight_query.iter_mut() {
		sprite.index = selected_level.0;
	}

	if keyboard.just_pressed(A_BUTTON) {
		if selected_level.0 == 5 {
			endless.0 = true;
		} else {
			endless.0 = false;
		}
		next_game_state.set(GameState::Loading);
	} else if keyboard.just_pressed(B_BUTTON) {
		next_game_state.set(GameState::Menu);
	}

	if keyboard.just_pressed(A_BUTTON)
	|| keyboard.just_pressed(B_BUTTON)
	|| keyboard.just_pressed(UP_BUTTON)
	|| keyboard.just_pressed(DOWN_BUTTON)
	|| keyboard.just_pressed(LEFT_BUTTON)
	|| keyboard.just_pressed(RIGHT_BUTTON)
	{
		audio.play(asset_server.load("sfx/ui_select.ogg")).with_volume(SFX_VOLUME);
	}
}