// Import Bevy game engine essentials
use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, render::camera::ScalingMode};
// Import components, resources, and events
use crate::{derivables::*, post_processing::PostProcessSettings};

// Plugin for handling all initial one time setup 
// such as camera spawning, loading save data and 
// initializing resources
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
			// States
			.add_state::<GameState>()
			.add_state::<PauseState>()
			// Events

			// Resources
			.insert_resource(Milky(false))
			.insert_resource(Endless(false))
			.insert_resource(LoadTimes(true))
			.insert_resource(SecretCode(0))
			.insert_resource(Retaliate(true))
			.insert_resource(BeamCharge(0.0))
			.insert_resource(SelectedLevel(0))
			.insert_resource(EnemiesSlain(0))
			.insert_resource(WinSpawned(0))
			.insert_resource(WinState(0))
			.insert_resource(LoadingLength{
				total: 4,
				current: 0,
			})
			.insert_resource(DustTimer(Timer::from_seconds(DUST_COOLDOWN, TimerMode::Repeating)))
			.insert_resource(BootTimer(Timer::from_seconds(BOOT_DURATION, TimerMode::Once)))
			.insert_resource(StartTimer(Timer::from_seconds(START_DURATION, TimerMode::Repeating)))
			.insert_resource(LoadingTimer(Timer::from_seconds(LOADING_DURATION, TimerMode::Repeating)))
			.insert_resource(NoEnemies(Timer::from_seconds(NO_ENEMIES_DURATION, TimerMode::Repeating)))
			.insert_resource(WinTimer(Timer::from_seconds(WIN_DURATION, TimerMode::Repeating)))
			.insert_resource(LevelInfo{
				round_timer: Timer::from_seconds(ENEMY_SPAWN_DELAY, TimerMode::Repeating),
				round: 0,
			})
			// Systems
			.add_systems( Startup,(
				spawn_camera,
				spawn_splash_screen,
				spawn_hints,
				init_constellations,
			))
			.add_systems( Update,(
				advance_splash_screen,
			).run_if(in_state(GameState::Boot)))
			.add_systems(OnExit(PauseState::Paused), (
				despawn_entities_with::<DespawnOnExitPauseState>,
			))
			.add_systems(OnExit(GameState::Boot), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Menu), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::LevelSelect), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Loading), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Level), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
			.add_systems(OnExit(GameState::Win), (
				despawn_entities_with::<DespawnOnExitGameState>,
			))
		;
	}
}

fn spawn_camera(
	mut commands: Commands,
) {
	// Main camera
	commands.spawn((
		Camera2dBundle{
			camera_2d: Camera2d{
				clear_color: ClearColorConfig::Custom(Color::BLACK),
				..default()
			},
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
			projection: OrthographicProjection {
				//scale: 1.0,
				scaling_mode: ScalingMode::FixedVertical(ORTHO_HEIGHT),
				//scaling_mode: ScalingMode::Fixed{width: ORTHO_WIDTH, height: ORTHO_HEIGHT},
				..Default::default()
			},
			..default()
		},
		PostProcessSettings {
            intensity: 0.0,
            ..default()
        },
	));
}

fn spawn_splash_screen(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SpriteBundle{
			transform: Transform::from_xyz(0.0, 0.0, 0.0),
			texture: asset_server.load("sprites/splash.png"),
			sprite: Sprite {
				custom_size: Some(Vec2::new(ORTHO_WIDTH, ORTHO_HEIGHT)),
				..default()
			},
			..default()
		},
		DespawnOnExitGameState,
	));
}

// Fade transitions into menu after a certain amount 
// of time or when the user presses a button
fn advance_splash_screen(
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut boot_timer: ResMut<BootTimer>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	boot_timer.0.tick(time.delta());
	if keyboard.just_pressed(START_BUTTON) 
	|| keyboard.just_pressed(A_BUTTON) 
	|| boot_timer.0.just_finished() {
		next_game_state.set(GameState::Menu);
	}
}

fn init_constellations(
	mut commands: Commands,
	mut milky: ResMut<Milky>,
) {
	let mut constellations = Vec::new(); 
	let mut cassiopeia = Vec::new();
	cassiopeia.push(Vec2::new(-44.5, 18.5));
	cassiopeia.push(Vec2::new(-24.5, -16.5));
	cassiopeia.push(Vec2::new(8.5, 0.5));
	cassiopeia.push(Vec2::new(26.5, -34.5));
	cassiopeia.push(Vec2::new(46.5, 2.5));

	let mut cepheus = Vec::new();
	cepheus.push(Vec2::new(-10.5, 40.5));
	cepheus.push(Vec2::new(-38.5, 8.5));
	cepheus.push(Vec2::new(-40.5, -50.5));
	cepheus.push(Vec2::new(14.5, -12.5));
	cepheus.push(Vec2::new(30.5, 26.5));

	let mut ursa_minor = Vec::new();
	ursa_minor.push(Vec2::new(18.5, 40.5));
	ursa_minor.push(Vec2::new(40.5, 28.5));
	ursa_minor.push(Vec2::new(30.5, -2.5));
	ursa_minor.push(Vec2::new(4.5, 4.5));
	ursa_minor.push(Vec2::new(-16.5, -8.5));
	ursa_minor.push(Vec2::new(-26.5, -26.5));
	ursa_minor.push(Vec2::new(-24.5, -52.5));

	let mut orion = Vec::new();
	orion.push(Vec2::new(-40.5, 40.5));
	orion.push(Vec2::new(-6.5, 34.5));
	orion.push(Vec2::new(-48.5, 6.5));
	orion.push(Vec2::new(6.5, 2.5));
	orion.push(Vec2::new(-2.5, -10.5));
	orion.push(Vec2::new(-12.5, -22.5));
	orion.push(Vec2::new(46.5, -14.5));
	orion.push(Vec2::new(16.5, -58.5));

	let mut random = Vec::new();
	if rand::random::<f32>() < 0.95 { 
		let total_stars = rand::Rng::gen_range(&mut rand::thread_rng(), 2..9);
		for _ in 0..total_stars {
			random.push(Vec2::new(((rand::random::<f32>() - 0.5) * 120.0).round() + 0.5, ((rand::random::<f32>() - 0.5) * 80.0).round() + 0.5));
		}
		milky.0 = false;
	} else {
		// The Sun
		random.push(Vec2::new(0.5, 0.5));
		milky.0 = true;
	}

	let endless = Vec::new();

	constellations.push(cassiopeia);
	constellations.push(cepheus);
	constellations.push(ursa_minor);
	constellations.push(orion);
	constellations.push(random);
	constellations.push(endless);

	let mut level_layout = LevelLayout{
		constellations: Vec::new(),
	};

	for constellation in constellations {
		level_layout.constellations.push(constellation);
	}

	commands.insert_resource(level_layout);
}

fn spawn_hints(
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	commands
		.spawn((SpriteSheetBundle {
			transform: Transform::from_xyz(0.0, 0.0, 400.0),
			texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("text/hints.png"), Vec2::new(140.0, 120.0), 4, 7, None, None)).clone(),
			sprite: TextureAtlasSprite{
				index: 0,
				custom_size: Some(Vec2::new(140.0, 120.0)),
				..default()
			},
			..default()
		},
		HintText,
	));
}

// Generic function used for despawning all entities with a specific component,
// mainly used for cleanup on state transitions
pub fn despawn_entities_with<T: Component>(
	mut commands: Commands,
	to_despawn: Query<Entity, With<T>>, 
) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}