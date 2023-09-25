// Import Bevy game engine essentials
use bevy::prelude::*;
// Import components, resources, and events
use crate::derivables::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Loading), 
				spawn_loading_screen,
			)
			.add_systems(Update, (
				advance_loading_screen,
			).run_if(in_state(GameState::Loading)
			))
		;
	}
}

fn spawn_loading_screen(
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut loading_length: ResMut<LoadingLength>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut hint_text_query: Query<(&mut TextureAtlasSprite, With<HintText>)>,
	mut loading_timer: ResMut<LoadingTimer>,
) {
	commands
		.spawn((SpriteBundle {
			transform: Transform::from_xyz(0.0, 40.0, 400.0),
			texture: asset_server.load("sprites/portrait.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(52.0, 52.0)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));

	commands
		.spawn((SpriteSheetBundle {
			transform: Transform::from_xyz(0.0, -10.0, 200.0),
			texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("text/loading.png"), Vec2::new(160.0, 144.0), 5, 1, None, None)).clone(),
			sprite: TextureAtlasSprite{
				index: 0,
				custom_size: Some(Vec2::new(160.0, 144.0)),
				..default()
			},
			..default()
		},
		LoadingText,
		DespawnOnExitGameState,
	));

	loading_length.current = 0;
	loading_length.total = rand::Rng::gen_range(&mut rand::thread_rng(), 1..TOTAL_LOAD);

	for (mut sprite, _) in hint_text_query.iter_mut() {
		let luck = rand::random::<f32>();
		sprite.index = if luck < 0.45 {
			rand::Rng::gen_range(&mut rand::thread_rng(), 1..5)
		} else if luck < 0.70 {
			rand::Rng::gen_range(&mut rand::thread_rng(), 5..13)
		} else if luck < 0.975 {
			rand::Rng::gen_range(&mut rand::thread_rng(), 13..23)
		} else {
			25
		};
	}

	loading_timer.0.reset();
}

fn advance_loading_screen(
	time: Res<Time>,
	keyboard: Res<Input<KeyCode>>,
	mut loading_length: ResMut<LoadingLength>,
	mut loading_timer: ResMut<LoadingTimer>,
	mut loading_text_query: Query<(&mut TextureAtlasSprite, With<LoadingText>)>,
	mut hint_text_query: Query<(&mut TextureAtlasSprite, (With<HintText>, Without<LoadingText>))>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	loading_timer.0.tick(time.delta());
	if loading_timer.0.just_finished() {
		loading_length.current += 1;
		for (mut sprite, _) in loading_text_query.iter_mut() {
			if loading_length.current < loading_length.total {
				sprite.index = (sprite.index + 1) % 4;
			} else {
				sprite.index = 4;
			}
		}
	}

	if loading_length.current >= loading_length.total && (keyboard.just_pressed(A_BUTTON)
	|| keyboard.just_pressed(ALT_A_BUTTON)
	|| keyboard.just_pressed(START_BUTTON)
	|| keyboard.just_pressed(ALT_START_BUTTON)
	|| keyboard.just_pressed(ALT_ALT_START_BUTTON)) {
		next_game_state.set(GameState::Level);
		for (mut sprite, _) in hint_text_query.iter_mut() {
			sprite.index = 0;
		}
	}
}