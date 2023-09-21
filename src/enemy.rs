// Import Bevy game engine essentials
use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_kira_audio::{Audio, AudioControl};
// Import components, resources, and events
use crate::derivables::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(Update,(
				enemy_move,
				slash_enemy,
				shoot_enemy,
				beam_enemy,
				enemy_animate,
				advance_corpses,
			).run_if(in_state(GameState::Level))
			.run_if(in_state(PauseState::Unpaused)))
		;
	}
}

fn slash_enemy(
	mut commands: Commands,
	mut enemies_slain: ResMut<EnemiesSlain>,
	mut beam_charge: ResMut<BeamCharge>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	slash_query: Query<(&GlobalTransform, &Direction, &Slash)>,
	enemy_query: Query<(Entity, &Transform, &TruePosition, With<Enemy>)>
) {
	for (slash_transform, direction, slash) in slash_query.iter() {
		let mut hit_sfx = false;
		if slash.active {
			for (entity, transform, pos, _) in enemy_query.iter() {
				let mut hit = false;
				let offset = (slash_transform.translation().xy() - pos.0).abs();
				match direction {
					Direction::Forward | Direction::Backward => if offset.x < 16.0 && offset.y < 12.0 {hit = true},
					Direction::Left | Direction::Right => if offset.x < 12.0 && offset.y < 16.0 {hit = true},
				}
				if hit {
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
		}
		if hit_sfx {audio.play(asset_server.load("sfx/enemy_destroyed.ogg")).with_volume(SFX_VOLUME);}
	}
}

fn shoot_enemy(
	mut commands: Commands,
	mut enemies_slain: ResMut<EnemiesSlain>,
	mut beam_charge: ResMut<BeamCharge>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	bullet_query: Query<(&Transform, With<Bullet>)>,
	enemy_query: Query<(Entity, &TruePosition, &Transform, With<Enemy>)>
) {
	for (transform, _) in bullet_query.iter() {
		let mut hit_sfx = false;
		for (entity, pos, enemy_transform, _) in enemy_query.iter() {
			if (transform.translation.xy() - pos.0).length() < 8.0 {
				enemies_slain.0 += 1;
				beam_charge.0 += 1.0;
				commands.entity(entity).despawn_recursive();
				hit_sfx = true;
				commands
					.spawn((SpriteSheetBundle {
						transform: Transform::from_xyz(enemy_transform.translation.x, enemy_transform.translation.y, 125.0),
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

fn beam_enemy(
	mut commands: Commands,
	mut enemies_slain: ResMut<EnemiesSlain>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	beam_query: Query<(&Transform, &Beam)>,
	enemy_query: Query<(Entity, &TruePosition, &Transform, With<Enemy>)>
) {
	for (transform, beam) in beam_query.iter() {
		let mut hit_sfx = false;
		for (entity, pos, enemy_transform, _) in enemy_query.iter() {
			let offset = match beam.0 {
				Direction::Forward | Direction::Backward => Vec2::new(30.0, 72.0),
				Direction::Left | Direction::Right=> Vec2::new(72.0, 30.0),
			};
			if (transform.translation.x - pos.0.x).abs() < offset.x 
			&& (transform.translation.y - pos.0.y).abs() < offset.y {
				enemies_slain.0 += 1;
				commands.entity(entity).despawn_recursive();
				hit_sfx = true;
				commands
					.spawn((SpriteSheetBundle {
						transform: Transform::from_xyz(enemy_transform.translation.x, enemy_transform.translation.y, 125.0),
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

fn enemy_animate(
	mut enemy_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, With<Enemy>)>,
	time: Res<Time>,
) {
	for (mut sprite, mut timer, _) in enemy_query.iter_mut() {
		timer.0.tick(time.delta());
		if timer.0.just_finished() {
			sprite.index = (sprite.index + 1) % 4;
		}
	}
}

pub fn enemy_move(
	mut enemy_query: Query<(&mut Transform, &mut TruePosition, With<Enemy>)>,
	mut star_query: Query<(&Transform, &mut Star, Without<Enemy>)>,
	mut mech_query: Query<(&TruePosition, &mut Mech, Without<Enemy>)>,
	retaliate: Res<Retaliate>,
	asset_server: Res<AssetServer>,
	time: Res<Time>,
	audio: Res<Audio>,
) {
	for (mech_pos, mut mech, _) in mech_query.iter_mut() {
		for (mut enemy_transform, mut pos, _) in enemy_query.iter_mut() {
			let mut direction = (mech_pos.0 - pos.0).normalize();
			if (mech_pos.0 - pos.0).length() <= 32.0 && mech.stun_cooldown.finished() && retaliate.0 {
				if (mech_pos.0 - pos.0).length() < 8.0 {
					mech.stun_cooldown.reset();
					audio.play(asset_server.load("sfx/unstun.ogg")).with_volume(SFX_VOLUME);
				}
			} else {
				let mut distance = 9999.0;
				for (star_transform, mut star, _) in star_query.iter_mut() {
					let offset = star_transform.translation.xy() - pos.0;
					if offset.length() < 8.0 {
						star.health = (star.health - ENEMY_DPS * time.delta_seconds()).clamp(0.0, 100.0);
						distance = 0.0;
						direction = Vec2::ZERO;
					} else if offset.length() < distance {
						distance = offset.length();
						direction = offset.normalize();
					}
				}
			}
			pos.0 += direction * ENEMY_SPEED * time.delta_seconds();
			enemy_transform.translation.x = pos.0.x.round(); 
			enemy_transform.translation.y = pos.0.y.round();
		}
	}
}

fn advance_corpses(
	time: Res<Time>,
	mut commands: Commands,
	mut corpse_query: Query<(Entity, &mut TextureAtlasSprite, &mut Corpse)>,
) {
	for (entity, mut sprite, mut corpse) in corpse_query.iter_mut() {
		corpse.0.tick(time.delta());
		if corpse.0.just_finished() {
			if sprite.index == 7 {
				commands.entity(entity).despawn_recursive();
			} else {
				sprite.index = (sprite.index + 1).clamp(0, 7);
			}
		}
	}
}