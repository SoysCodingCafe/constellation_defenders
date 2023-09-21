// Import Bevy game engine essentials
use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_kira_audio::{Audio, AudioControl};
// Import components, resources, and events
use crate::{derivables::*, enemy::enemy_move};

pub struct MechPlugin;

impl Plugin for MechPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(OnEnter(GameState::Level), (
				spawn_mech,
			))
			.add_systems(Update,(
				mech_move,
				mech_slash,
				mech_shoot,
				mech_beam,
				mech_stun.after(enemy_move),
				mech_animate,
				slash_animate,
				beam_animate,
			).run_if(in_state(GameState::Level))
			.run_if(in_state(PauseState::Unpaused)))
			.add_systems( Update, (
				disable_retaliate,
				hide_retaliation_text
			))
		;
	}
}

fn spawn_mech(
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn((SpriteSheetBundle {
			transform: Transform::from_xyz(0.0, 0.0, 200.0),
			texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/mech.png"), Vec2::new(16.0, 16.0), 4, 4, None, None)).clone(),
			sprite: TextureAtlasSprite{
				index: 0,
				custom_size: Some(Vec2::new(16.0, 16.0)),
				..default()
			},
			..default()
		},
		Mech{
			stun_cooldown: Timer::from_seconds(STUN_COOLDOWN, TimerMode::Once),
			slash_cooldown: Timer::from_seconds(SLASH_COOLDOWN, TimerMode::Once),
			shoot_cooldown: Timer::from_seconds(SHOOT_COOLDOWN, TimerMode::Once),
			beam_cooldown: Timer::from_seconds(BEAM_COOLDOWN, TimerMode::Once),
		},
		TruePosition(Vec2::new(0.0, 0.0)),
		Direction::Forward,
		AnimationTimer(Timer::from_seconds(MECH_ANIMATION_SPEED, TimerMode::Repeating)),
		DespawnOnExitGameState,
	)).with_children(|parent| {
		parent
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(0.0, -16.0, 10.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/forward_slash.png"), Vec2::new(24.0, 16.0), 4, 2, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(24.0, 16.0)),
					..default()
				},
				..default()
			},
			Slash{
				active: false,
			},
			Direction::Forward,
			AnimationTimer(Timer::from_seconds(SLASH_SPEED, TimerMode::Repeating)),
		));
	}).with_children(|parent| {
		parent
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(0.0, 16.0, 10.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/backward_slash.png"), Vec2::new(24.0, 16.0), 4, 2, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(24.0, 16.0)),
					..default()
				},
				..default()
			},
			Slash{
				active: false,
			},
			Direction::Backward,
			AnimationTimer(Timer::from_seconds(SLASH_SPEED, TimerMode::Repeating)),
		));
	}).with_children(|parent| {
		parent
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(-16.0, 0.0, 10.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/left_slash.png"), Vec2::new(16.0, 24.0), 4, 2, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(16.0, 24.0)),
					..default()
				},
				..default()
			},
			Slash{
				active: false,
			},
			Direction::Left,
			AnimationTimer(Timer::from_seconds(SLASH_SPEED, TimerMode::Repeating)),
		));
	}).with_children(|parent| {
		parent
			.spawn((SpriteSheetBundle {
				transform: Transform::from_xyz(16.0, 0.0, 10.0),
				texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/right_slash.png"), Vec2::new(16.0, 24.0), 4, 2, None, None)).clone(),
				sprite: TextureAtlasSprite{
					index: 0,
					custom_size: Some(Vec2::new(16.0, 24.0)),
					..default()
				},
				..default()
			},
			Slash{
				active: false,
			},
			Direction::Right,
			AnimationTimer(Timer::from_seconds(SLASH_SPEED, TimerMode::Repeating)),
		));
	})
	;
}

fn mech_animate(
	mut mech_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &Direction, With<Mech>)>,
	time: Res<Time>,
) {
	for (mut sprite, mut timer, direction, _) in mech_query.iter_mut() {
		timer.0.tick(time.delta());
		if timer.0.just_finished() {
			sprite.index = sprite.index + 1;
		}
		match direction {
			Direction::Forward => sprite.index = sprite.index % 4,
			Direction::Backward => sprite.index = sprite.index % 4 + 4,
			Direction::Left => sprite.index = sprite.index % 4 + 8,
			Direction::Right => sprite.index = sprite.index % 4 + 12,
		}
	}
}

fn slash_animate(
	mut slash_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Slash)>,
	time: Res<Time>,
) {
	for (mut sprite, mut timer, mut slash) in slash_query.iter_mut() {
		if slash.active {
			timer.0.tick(time.delta());
			if timer.0.just_finished() {
				if sprite.index == 7 {
					sprite.index = 0;
					slash.active = false;
				} else {
					sprite.index = sprite.index + 1;
				}
			}
		}
	}
}

fn beam_animate(
	mut commands: Commands,
	mut beam_query: Query<(Entity, &mut TextureAtlasSprite, &mut AnimationTimer, With<Beam>)>,
	time: Res<Time>,
) {
	for (entity, mut sprite, mut timer, _) in beam_query.iter_mut() {
		timer.0.tick(time.delta());
		if timer.0.just_finished() {
			if sprite.index == 7 {
				commands.entity(entity).despawn_recursive();
			} else {
				sprite.index = sprite.index + 1;
			}
		}
	}
}

fn mech_move(
	time: Res<Time>,
	keyboard: Res<Input<KeyCode>>,
	mut mech_query: Query<(&mut Transform, &mut TruePosition, &mut Direction, &Mech)>,
) {
	for (mut transform, mut pos, mut direction, mech) in mech_query.iter_mut() {
		if keyboard.pressed(UP_BUTTON) {
			*direction = Direction::Backward;
		} else if keyboard.pressed(DOWN_BUTTON) {
			*direction = Direction::Forward;
		} else if keyboard.pressed(LEFT_BUTTON) {
			*direction = Direction::Left;
		} else if keyboard.pressed(RIGHT_BUTTON) {
			*direction = Direction::Right;
		}
		if mech.stun_cooldown.finished() && mech.beam_cooldown.finished() {
			let slowdown = if !mech.slash_cooldown.finished() || !mech.shoot_cooldown.finished() {0.4} else {1.0};
			if keyboard.pressed(UP_BUTTON) {
				pos.0.y += MECH_SPEED * slowdown * time.delta_seconds();
			} else if keyboard.pressed(DOWN_BUTTON) {
				pos.0.y -= MECH_SPEED * slowdown * time.delta_seconds();
			} else if keyboard.pressed(LEFT_BUTTON) {
				pos.0.x -= MECH_SPEED * slowdown * time.delta_seconds();
			} else if keyboard.pressed(RIGHT_BUTTON) {
				pos.0.x += MECH_SPEED * slowdown * time.delta_seconds();
			}
			pos.0 = Vec2::new(pos.0.x.clamp(-72.0, 72.0), pos.0.y.clamp(-64.0, 64.0));
			transform.translation.x = pos.0.x.round(); 
			transform.translation.y = pos.0.y.round();
		}
	}
}

fn mech_stun(
	mut commands: Commands,
	mut mech_query: Query<(&TruePosition, &mut Mech)>,
	mut slash_query: Query<(&mut Slash, Without<Mech>)>,
	mut enemies_slain: ResMut<EnemiesSlain>,
	mut beam_charge: ResMut<BeamCharge>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	retaliate: Res<Retaliate>,
	enemy_query: Query<(Entity, &Transform, &TruePosition, With<Enemy>)>,
	time: Res<Time>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
) {
	for (mech_pos, mut mech) in mech_query.iter_mut() {
		mech.stun_cooldown.tick(time.delta());
		mech.beam_cooldown.tick(time.delta());
		if mech.stun_cooldown.just_finished() && retaliate.0 {
			audio.play(asset_server.load("sfx/slash.ogg")).with_volume(SFX_VOLUME);
			let mut hit_sfx = false;
			for (mut slash, _) in slash_query.iter_mut() {
				slash.active = true;
			}
			for (entity, transform, enemy_pos, _) in enemy_query.iter() {
				if (mech_pos.0 - enemy_pos.0).length() < 16.0 {
					enemies_slain.0 += 1;
					beam_charge.0 += 1.0;
					commands.entity(entity).despawn_recursive();
					hit_sfx = true;
					commands
						.spawn((SpriteSheetBundle {
							transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 125.0),
							texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/corpse.png"), Vec2::new(8.0, 8.0), 4, 2, None, None)).clone(),
							sprite: TextureAtlasSprite{
								index: 0,
								custom_size: Some(Vec2::new(8.0, 8.0)),
								..default()
							},
							..default()
						},
						Corpse(Timer::from_seconds(CORPSE_DURATION, TimerMode::Repeating)),
						DespawnOnExitGameState,
					));
				}
			}
			if hit_sfx {audio.play(asset_server.load("sfx/enemy_destroyed.ogg")).with_volume(SFX_VOLUME);}
		}
	}
}

fn mech_slash(
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut mech_query: Query<(&Direction, &mut Mech)>,
	mut slash_query: Query<(&Direction, &mut Slash)>,
) {
	for (mech_direction, mut mech) in mech_query.iter_mut() {
		mech.slash_cooldown.tick(time.delta());
		if keyboard.just_pressed(A_BUTTON) 
		&& mech.slash_cooldown.finished() 
		&& mech.stun_cooldown.finished() 
		&& mech.beam_cooldown.finished(){
			mech.slash_cooldown.reset();
			audio.play(asset_server.load("sfx/slash.ogg")).with_volume(SFX_VOLUME);
			for (slash_direction, mut slash) in slash_query.iter_mut() {
				if *mech_direction == *slash_direction {
					slash.active = true;
				}
			}
		}
	}
}

fn mech_shoot(
	keyboard: Res<Input<KeyCode>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	mut commands: Commands,
	mut mech_query: Query<(&Transform, &Direction, &mut Mech)>,
) {
	for (transform, direction, mut mech) in mech_query.iter_mut() {
		mech.shoot_cooldown.tick(time.delta());
		if keyboard.pressed(B_BUTTON) 
		&& mech.shoot_cooldown.finished() 
		&& mech.stun_cooldown.finished() 
		&& mech.beam_cooldown.finished() {
			mech.shoot_cooldown.reset();
			audio.play(asset_server.load("sfx/pew.ogg")).with_volume(SFX_VOLUME);
			commands
				.spawn((SpriteBundle {
					transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 150.0),
					..default()
				},
				Bullet{
					velocity: Vec2::new(
						if *direction == Direction::Left {-BULLET_SPEED} else if *direction == Direction::Right {BULLET_SPEED} else {0.0},
						if *direction == Direction::Forward {-BULLET_SPEED} else if *direction == Direction::Backward {BULLET_SPEED} else {0.0},
					),
				},
				TruePosition(transform.translation.xy()),
				DespawnOnExitGameState,
			));
		}
	}
}

fn mech_beam(
	mut beam_charge: ResMut<BeamCharge>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	keyboard: Res<Input<KeyCode>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut mech_query: Query<(&Transform, &Direction, &mut Mech)>,
) {
	if beam_charge.0 >= BEAM_CHARGE_REQUIREMENT {
		if keyboard.pressed(A_BUTTON) && keyboard.pressed(B_BUTTON) {
			beam_charge.0 = 0.0;
			for (transform, direction, mut mech) in mech_query.iter_mut() {
				mech.beam_cooldown.reset();
				audio.play(asset_server.load("sfx/beam.ogg")).with_volume(SFX_VOLUME);
				let offset = match direction {
					Direction::Forward => Vec2::new(0.0, -80.0),
					Direction::Backward => Vec2::new(0.0, 80.0),
					Direction::Left => Vec2::new(-80.0, 0.0),
					Direction::Right => Vec2::new(80.0, 0.0),
				};
				commands
					.spawn((SpriteSheetBundle {
						transform: Transform::from_xyz(transform.translation.x + offset.x, transform.translation.y + offset.y, 190.0),
						texture_atlas: match direction {
							Direction::Forward => texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/beam_down.png"), Vec2::new(60.0, 144.0), 4, 2, None, None)).clone(),
							Direction::Backward => texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/beam_up.png"), Vec2::new(60.0, 144.0), 4, 2, None, None)).clone(),
							Direction::Left => texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/beam_left.png"), Vec2::new(144.0, 60.0), 2, 4, None, None)).clone(),
							Direction::Right => texture_atlases.add(TextureAtlas::from_grid(asset_server.load("sprites/beam_right.png"), Vec2::new(144.0, 60.0), 2, 4, None, None)).clone(),
						},
						sprite: TextureAtlasSprite{
							index: 0,
							custom_size: match direction {
								Direction::Forward => Some(Vec2::new(60.0, 144.0)),
								Direction::Backward => Some(Vec2::new(60.0, 144.0)),
								Direction::Left => Some(Vec2::new(144.0, 60.0)),
								Direction::Right => Some(Vec2::new(144.0, 60.0)),
							},
							..default()
						},
						..default()
					},
					Beam(*direction),
					AnimationTimer(Timer::from_seconds(BEAM_SPEED, TimerMode::Repeating)),
					DespawnOnExitGameState,
				));
			}
		}
	}
}

fn disable_retaliate(
	keyboard: Res<Input<KeyCode>>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut commands: Commands,
	mut retaliate: ResMut<Retaliate>,
	mut secret_code: ResMut<SecretCode>,
) {
	if keyboard.just_pressed(UP_BUTTON) {
		if secret_code.0 == 0 
		|| secret_code.0 == 1 
		|| secret_code.0 == 6 
		|| secret_code.0 == 8 {
			secret_code.0 += 1
		} else {
			secret_code.0 = 0;
		}
	} else if keyboard.just_pressed(DOWN_BUTTON) {
		if secret_code.0 == 3 
		|| secret_code.0 == 5 {
			secret_code.0 += 1
		} else {
			secret_code.0 = 0;
		}
	} else if keyboard.just_pressed(LEFT_BUTTON) {
		if secret_code.0 == 2 
		|| secret_code.0 == 4
		|| secret_code.0 == 7 {
			secret_code.0 += 1
		} else {
			secret_code.0 = 0;
		}
	} else if keyboard.just_pressed(RIGHT_BUTTON) {
		if secret_code.0 == 9 {
			secret_code.0 += 1
		} else {
			secret_code.0 = 0;
		}
	} else if keyboard.just_pressed(SELECT_BUTTON) {
		if secret_code.0 == 10 {
			secret_code.0 += 1
		} else {
			secret_code.0 = 0;
		}
	} else if keyboard.just_pressed(START_BUTTON) {
		if secret_code.0 == 11 {
			audio.play(asset_server.load("sfx/secret.ogg")).with_volume(SFX_VOLUME);
			secret_code.0 = 0;
			retaliate.0 = !retaliate.0;
			if retaliate.0 {
				//println!("Override Accepted. Retaliation enabled.");
				commands
					.spawn((SpriteSheetBundle {
						transform: Transform::from_xyz(0.0, 0.0, 800.0),
						texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("text/retaliation.png"), Vec2::new(92.0, 12.0), 1, 2, None, None)).clone(),
						sprite: TextureAtlasSprite{
							index: 1,
							custom_size: Some(Vec2::new(92.0, 12.0)),
							..default()
						},
						..default()
					},
					RetaliateText(Timer::from_seconds(RETALIATE_DURATION, TimerMode::Once)),
				));
			} else {
				//println!("Override Accepted. Retaliation disabled.");
				commands
					.spawn((SpriteSheetBundle {
						transform: Transform::from_xyz(0.0, 0.0, 800.0),
						texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("text/retaliation.png"), Vec2::new(92.0, 12.0), 1, 2, None, None)).clone(),
						sprite: TextureAtlasSprite{
							index: 0,
							custom_size: Some(Vec2::new(92.0, 12.0)),
							..default()
						},
						..default()
					},
					RetaliateText(Timer::from_seconds(RETALIATE_DURATION, TimerMode::Once)),
				));
			}
		} else {
			secret_code.0 = 0;
		}
	} 
}

fn hide_retaliation_text(
	mut commands: Commands,
	mut retaliation_query: Query<(Entity, &mut RetaliateText)>,
	time: Res<Time>,
) {
	for (entity, mut retaliate_text) in retaliation_query.iter_mut() {
		retaliate_text.0.tick(time.delta());
		if retaliate_text.0.finished() {
			commands.entity(entity).despawn_recursive();
		}
	}
}