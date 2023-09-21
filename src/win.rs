// Import Bevy game engine essentials
use bevy::prelude::*;
// Import components, resources, and events
use crate::derivables::*;

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Win), 
				spawn_win_screen,
			)
			.add_systems(Update, (
				update_win_timer,
			).run_if(in_state(GameState::Win)
			))
		;
	}
}

fn spawn_win_screen(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SpriteBundle{
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			texture: asset_server.load("sprites/win_screen.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(160.0, 144.0)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));
}

fn update_win_timer(
	mut commands: Commands,
	mut win_spawned: ResMut<WinSpawned>,
	mut win_timer: ResMut<WinTimer>,
	mut clock_divider: Local<bool>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut win_text_query: Query<(Entity, &mut Visibility, With<WinText>)>,
	mut next_game_state: ResMut<NextState<GameState>>,
	keyboard: Res<Input<KeyCode>>,
	asset_server: Res<AssetServer>,
	win_state: Res<WinState>,
	time: Res<Time>,
) {
	win_timer.0.tick(time.delta());
	let manual = keyboard.just_pressed(A_BUTTON);
	let mut auto = win_timer.0.just_finished();
	
	if auto {*clock_divider = !*clock_divider};
	if manual {
		auto = false;
		win_timer.0.reset();
		*clock_divider = false;
	}

	if manual || (auto && *clock_divider) {
		if win_spawned.0 == 0 {
			commands
				.spawn((SpriteSheetBundle {
					transform: Transform::from_xyz(0.0, 0.0, 200.0),
					texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("text/lost.png"), Vec2::new(160.0, 144.0), 3, 1, None, None)).clone(),
					sprite: TextureAtlasSprite{
						index: win_state.0,
						custom_size: Some(Vec2::new(160.0, 144.0)),
						..default()
					},
					..default()
				},
				WinText,
				DespawnOnExitGameState,
			));
			win_spawned.0 += 1;
		} else if win_spawned.0 == 1 {
			commands
				.spawn((SpriteSheetBundle {
					transform: Transform::from_xyz(0.0, 0.0, 200.0),
					texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("text/constellation.png"), Vec2::new(160.0, 144.0), 3, 1, None, None)).clone(),
					sprite: TextureAtlasSprite{
						index: win_state.0,
						custom_size: Some(Vec2::new(160.0, 144.0)),
						..default()
					},
					..default()
				},
				WinText,
				DespawnOnExitGameState,
			));
			win_spawned.0 += 1;
		} else if win_spawned.0 == 2 {
			for (entity, _, _) in win_text_query.iter() {
				commands.entity(entity).despawn_recursive();
			}
			commands
				.spawn((SpriteSheetBundle {
					transform: Transform::from_xyz(0.0, 0.0, 200.0),
					texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("text/win.png"), Vec2::new(160.0, 144.0), 3, 1, None, None)).clone(),
					sprite: TextureAtlasSprite{
						index: win_state.0,
						custom_size: Some(Vec2::new(160.0, 144.0)),
						..default()
					},
					..default()
				},
				WinText,
				DespawnOnExitGameState,
			));
			win_spawned.0 += 1;
		} else if win_spawned.0 == 3 && manual {
			next_game_state.set(GameState::Menu);
		}
	}

	if win_spawned.0 == 3 && win_state.0 != 2 {
		if win_timer.0.just_finished() {
			for (_, mut visibility, _) in win_text_query.iter_mut() {
				if *visibility == Visibility::Visible || *visibility == Visibility::Inherited {
					*visibility = Visibility::Hidden;
				} else {
					*visibility = Visibility::Visible;
				}
			}
		}
	}
}