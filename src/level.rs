use std::time::Duration;

// Import Bevy game engine essentials
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioTween};
// Import components, resources, and events
use crate::derivables::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Level), (
				reset_level_timer,
				spawn_level,
			))
			.add_systems(Update,(
				update_level_timer,
				bullet_move,
				star_animate,
				constellation_lost,
				constellation_defended,
				pause_game,
				beam_bar_animate,
			).run_if(in_state(GameState::Level))
			.run_if(in_state(PauseState::Unpaused)))
			.add_systems(Update,(
				navigate_pause.after(pause_game),
			).run_if(in_state(GameState::Level))
			.run_if(in_state(PauseState::Paused)))
		;
	}
}

fn reset_level_timer(
	mut no_enemies_timer: ResMut<NoEnemies>,
	mut round_timer: ResMut<LevelInfo>,
	mut enemies_slain: ResMut<EnemiesSlain>,
	mut beam_charge: ResMut<BeamCharge>,
	mut win_spawned: ResMut<WinSpawned>,
	mut win_state: ResMut<WinState>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
) {
	round_timer.round = 0;
	round_timer.round_timer.reset();
	no_enemies_timer.0.reset();
	enemies_slain.0 = 0;
	beam_charge.0 = 0.0;
	win_spawned.0 = 0;
	win_state.0 = 0;
	audio.play(asset_server.load("sfx/unstun.ogg")).with_volume(SFX_VOLUME);
}

fn update_level_timer(
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	mech_query: Query<&TruePosition, With<Mech>>,
	endless: Res<Endless>,
	selected_level: Res<SelectedLevel>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut round_timer: ResMut<LevelInfo>,
	mut commands: Commands,
) {
	let max_rounds = match selected_level.0 {
		0 => 30,
		1 => 45,
		2 => 60,
		3 => 75,
		4 => 88,
		_ => 999,
	};
	if round_timer.round < max_rounds || endless.0 {
		round_timer.round_timer.tick(time.delta());
		if round_timer.round_timer.just_finished() {
			round_timer.round += 1;
			for pos in mech_query.iter() {
				let j = (round_timer.round / 10).clamp(1, 10);
				for _ in 0..j {
					let r = rand::random::<f32>();
					let direction = if r >= 0.5 {90.0} else if r >= 0.75 {180.0} else {360.0};
					let offset = if pos.0.length() > 1.0 {(-pos.0.normalize().rotate(Vec2::from_angle(((rand::random::<f32>() - 0.5) * direction).to_radians())) * 120.0).clamp_length(120.0, 128.0)}
					else {(-Vec2::Y.rotate(Vec2::from_angle((rand::random::<f32>() * 360.0).to_radians())) * 120.0).clamp_length(120.0, 128.0)};
					let spec = if rand::random::<f32>() > 0.8 {0} else {1};
					commands
						.spawn((SpriteSheetBundle {
							transform: Transform::from_xyz(offset.x, offset.y, 150.0),
							texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load(if spec == 0 {"sprites/enemy_0.png"} else {"sprites/enemy_1.png"}), Vec2::new(16.0, 16.0), 2, 2, None, None)).clone(),
							sprite: TextureAtlasSprite{
								index: 0,
								custom_size: Some(Vec2::new(16.0, 16.0)),
								..default()
							},
							..default()
						},
						Enemy{
							spec: spec, 
							rotation: if rand::random::<f32>() > 0.5 {1.0} else {-1.0},
							dps: if spec == 0 {ENEMY_0_DPS} else {ENEMY_1_DPS},
						},
						//Velocity(Vec2::new((rand::random::<f32>() - 0.5) * ENEMY_SPEED, (rand::random::<f32>() - 0.5) * ENEMY_SPEED)),
						Velocity(Vec2::ZERO),//(pos.0 - offset).normalize().rotate(Vec2::from_angle(80.0_f32.to_radians())) * INITIAL_TANGENTIAL_SPEED),
						TruePosition(Vec2::new(offset.x, offset.y)),
						AnimationTimer(Timer::from_seconds(ENEMY_ANIMATION_SPEED, TimerMode::Repeating)),
						DespawnOnExitGameState,
					));
				}
			}
		};
	} else {
		round_timer.round = max_rounds;
	}
}

fn spawn_level(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	milky: Res<Milky>,
	retaliate: Res<Retaliate>,
	asset_server: Res<AssetServer>,
	level_layout: Res<LevelLayout>,
	selected_level: Res<SelectedLevel>,
) {
	let background = if milky.0 {0} else {rand::Rng::gen_range(&mut rand::thread_rng(), 1..5)};
	commands
		.spawn((SpriteSheetBundle {
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/background.png"), Vec2::new(160.0, 144.0), 3, 2, None, None)).clone(),
			sprite: TextureAtlasSprite{
				index: background,
				custom_size: Some(Vec2::new(160.0, 144.0)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));
	if retaliate.0 {
		commands
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(0.0, 60.0, 300.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/beam_bar.png"), Vec2::new(80.0, 20.0), 2, 4, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(80.0, 20.0)),
					..default()
				},
				..default()
			},
			BeamBar,
			DespawnOnExitGameState,
		));
	}
	for star_loc in &level_layout.constellations[selected_level.0] {
		commands
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(star_loc.x, star_loc.y, 100.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/star.png"), Vec2::new(15.0, 15.0), 4, 2, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(15.0, 15.0)),
					..default()
				},
				..default()
			},
			Star{
				health: STAR_HEALTH,
			},
			DespawnOnExitGameState,
		));
	}
}

fn star_animate(
	mut commands: Commands,
	mut star_query: Query<(Entity, &mut TextureAtlasSprite, &Star)>,
) {
	for (entity, mut sprite, star) in star_query.iter_mut() {
		if star.health == 0.0 {
			commands.entity(entity).despawn_recursive();
		} else {
			sprite.index = (8.0 - (star.health/10.0)).clamp(0.0, 7.0) as usize;
		}
	}
}

fn beam_bar_animate(
	beam_charge: Res<BeamCharge>,
	mut star_query: Query<(&mut Visibility, &mut TextureAtlasSprite, With<BeamBar>)>,
	mech_query: Query<(&TruePosition, With<Mech>)>,
) {
	for (mut visibility, mut sprite, _) in star_query.iter_mut() {
		for (pos, _) in mech_query.iter() {
			if pos.0.x.abs() < 48.0 && pos.0.y > 42.0 {
				*visibility = Visibility::Hidden;
			} else {
				*visibility = Visibility::Visible;
			}
		}
		sprite.index = ((beam_charge.0 / (BEAM_CHARGE_REQUIREMENT/8.0)).clamp(0.0, 7.0)) as usize;
	}
}

fn bullet_move(
	time: Res<Time>,
	mut commands: Commands,
	mut bullet_query: Query<(Entity, &mut Transform, &mut TruePosition, &Bullet)>,
) {
	for (entity, mut transform, mut pos, bullet) in bullet_query.iter_mut() {
		pos.0 += bullet.velocity * time.delta_seconds();
		if pos.0.x.abs() > 81.0 || pos.0.y.abs() > 73.0 {
			commands.entity(entity).despawn_recursive();
		} else {
			transform.translation.x = pos.0.x.round(); 
			transform.translation.y = pos.0.y.round();
		}
	}
}

fn constellation_lost(
	star_query: Query<With<Star>>,
	enemies_slain: Res<EnemiesSlain>,
	endless: Res<Endless>,
	mut win_state: ResMut<WinState>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	let mut remaining_stars = 0;
	for _ in star_query.iter() {
		remaining_stars += 1;
	}
	if remaining_stars == 0 && !endless.0 {
		if enemies_slain.0 > 0 {win_state.0 = 0} else {win_state.0 = 2};
		next_game_state.set(GameState::Win);
	}
}

fn constellation_defended(
	round_timer: Res<LevelInfo>,
	enemy_query: Query<With<Enemy>>,
	endless: Res<Endless>,
	audio: Res<Audio>,
	time: Res<Time>,
	selected_level: Res<SelectedLevel>,
	mut no_enemies_timer: ResMut<NoEnemies>,
	mut win_state: ResMut<WinState>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	let max_rounds = match selected_level.0 {
		0 => 30,
		1 => 45,
		2 => 60,
		3 => 75,
		4 => 88,
		_ => 999,
	};
	if round_timer.round == max_rounds && !endless.0 {
		if enemy_query.is_empty() {
			if no_enemies_timer.0.percent() == 0.0 {
				audio.stop().fade_out(AudioTween::linear(Duration::new(2, 0)));
			}
			no_enemies_timer.0.tick(time.delta());
			if no_enemies_timer.0.just_finished() {
				win_state.0 = 1;
				next_game_state.set(GameState::Win);
			}
		}
	}
}

fn pause_game(
	keyboard: Res<Input<KeyCode>>,
	pause_state: Res<State<PauseState>>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut hint_text_query: Query<(&mut TextureAtlasSprite, With<HintText>)>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut commands: Commands,
	mut next_pause_state: ResMut<NextState<PauseState>>,
) {
	if keyboard.just_pressed(START_BUTTON) {
		audio.play(asset_server.load("sfx/ui_select.ogg")).with_volume(SFX_VOLUME);
		if *pause_state.get() == PauseState::Unpaused {
			next_pause_state.set(PauseState::Paused);
			commands
				.spawn((SpriteBundle {
					transform: Transform::from_xyz(0.0, 0.0, 350.0),
					texture: asset_server.load("sprites/pause_screen.png"),
					sprite: Sprite {
						custom_size: Some(Vec2::new(140.0, 120.0)),
						..default()
					},
					..default()
				},
				DespawnOnExitPauseState,
			));
			commands
				.spawn((SpriteSheetBundle {
					transform: Transform::from_xyz(0.0, 0.0, 400.0),
					texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/pause_highlight.png"), Vec2::new(140.0, 120.0), 2, 1, None, None)).clone(),
					sprite: TextureAtlasSprite{
						index: 0,
						custom_size: Some(Vec2::new(140.0, 120.0)),
						..default()
					},
					..default()
				},
				PauseHighlight,
				DespawnOnExitPauseState,
			));
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
		}
	}
}

fn navigate_pause(
	keyboard: Res<Input<KeyCode>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut hint_text_query: Query<(&mut TextureAtlasSprite, With<HintText>)>,
	mut pause_highlight_query: Query<(&mut TextureAtlasSprite, (With<PauseHighlight>, Without<HintText>))>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	if keyboard.just_pressed(UP_BUTTON)
	|| keyboard.just_pressed(DOWN_BUTTON) 
	|| keyboard.just_pressed(A_BUTTON) 
	|| keyboard.just_pressed(B_BUTTON) 
	|| keyboard.just_pressed(START_BUTTON) {
		audio.play(asset_server.load("sfx/ui_select.ogg")).with_volume(SFX_VOLUME);
	}

	if keyboard.just_pressed(UP_BUTTON)
	|| keyboard.just_pressed(DOWN_BUTTON) {
		for (mut sprite, _) in pause_highlight_query.iter_mut() {
			sprite.index = (sprite.index + 1) % 2;
		}
	} else if keyboard.just_pressed(A_BUTTON) {
		for (mut sprite, _) in hint_text_query.iter_mut() {
			sprite.index = 0;
		}
		for (sprite, _) in pause_highlight_query.iter() {
			if sprite.index == 1 {
				next_pause_state.set(PauseState::Unpaused);
				next_game_state.set(GameState::Menu);
			} else {
				next_pause_state.set(PauseState::Unpaused);
			}
		}
	} else if keyboard.just_pressed(B_BUTTON) {
		for (mut sprite, _) in hint_text_query.iter_mut() {
			sprite.index = 0;
		}
		next_pause_state.set(PauseState::Unpaused);
	} else if keyboard.just_pressed(START_BUTTON) {
		next_pause_state.set(PauseState::Unpaused);
		for (mut sprite, _) in hint_text_query.iter_mut() {
			sprite.index = 0;
		}
	}
}