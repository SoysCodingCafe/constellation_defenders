// Import Bevy game engine essentials
use bevy::prelude::*;

// CONSTANTS
// Controls
pub const START_BUTTON: KeyCode = KeyCode::A;
pub const SELECT_BUTTON: KeyCode = KeyCode::S;
pub const A_BUTTON: KeyCode = KeyCode::Z;
pub const B_BUTTON: KeyCode = KeyCode::X;
pub const UP_BUTTON: KeyCode = KeyCode::Up;
pub const DOWN_BUTTON: KeyCode = KeyCode::Down;
pub const LEFT_BUTTON: KeyCode = KeyCode::Left;
pub const RIGHT_BUTTON: KeyCode = KeyCode::Right;

// Audio
pub const BGM_VOLUME: f64 = 1.0;
pub const SFX_VOLUME: f64 = 1.0;

// Window Resolution
pub const ORTHO_WIDTH: f32 = 160.0;
pub const ORTHO_HEIGHT: f32 = 144.0;

// Color indexes for the palette
//pub const COLOR_A: Color = Color::rgb(0.0, 0.0, 0.0);
//pub const COLOR_B: Color = Color::rgb(0.3, 0.0, 0.0);
//pub const COLOR_C: Color = Color::rgb(0.6, 0.0, 0.0);
//pub const COLOR_D: Color = Color::rgb(1.0, 0.0, 0.0);

// General Parameters
pub const BOOT_DURATION: f32 = 1.0;
pub const START_DURATION: f32 = 0.5;
pub const LOADING_DURATION: f32 = 0.5;
pub const NO_ENEMIES_DURATION: f32 = 2.0;
pub const WIN_DURATION: f32 = 1.0;
pub const CORPSE_DURATION: f32 = 0.05;
pub const RETALIATE_DURATION: f32 = 2.0;

pub const DUST_COOLDOWN: f32 = 0.5;
pub const DUST_SPEED: f32 = 10.0;
pub const DUST_DURATION: f32 = 0.25;

pub const TOTAL_LOAD: usize = 9;

// Mech Stats
pub const MECH_SPEED: f32 = 60.0;
pub const MAX_MECH_SPEED: f32 = 30.0;
pub const MECH_ACCELERATION: f32 = 10.0;
pub const MECH_DAMPING: f32 = 1.0;

pub const STUN_COOLDOWN: f32 = 1.0;
pub const SLASH_COOLDOWN: f32 = 0.5;
pub const SHOOT_COOLDOWN: f32 = 0.25;
pub const BEAM_COOLDOWN: f32 = 0.8;

pub const BEAM_CHARGE_REQUIREMENT: f32 = 30.0;
pub const BEAM_SPEED: f32 = 0.125;

pub const SLASH_SPEED: f32 = 0.05;
pub const BULLET_SPEED: f32 = 80.0;

pub const MECH_ANIMATION_SPEED: f32 = 0.35;

// Enemy Stats
pub const ENEMY_ANIMATION_SPEED: f32 = 0.35;
pub const ENEMY_SPAWN_DELAY: f32 = 0.6;

pub const ENEMY_MAX_SPEED: f32 = 35.0;
pub const ENEMY_ACCELERATION: f32 = 5.0;

pub const ENEMY_0_DPS: f32 = 25.0;
pub const ENEMY_1_DPS: f32 = 5.0;

// Star Stats
pub const STAR_HEALTH: f32 = 80.0;

// STATES
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	Boot,
	Menu,
	LevelSelect,
	Loading,
	Level,
	Win,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
	#[default]
	Unpaused,
	Paused,
}

// COMPONENTS
#[derive(Component)]
pub struct DespawnOnExitGameState;

#[derive(Component)]
pub struct DespawnOnExitPauseState;

#[derive(Component)]
pub struct StartText;

#[derive(Component)]
pub struct LoadingText;

#[derive(Component)]
pub struct HintText;

#[derive(Component)]
pub struct WinText;

#[derive(Component)]
pub struct RetaliateText(pub Timer);

#[derive(Component)]
pub struct Mech{
	pub stun_cooldown: Timer,
	pub slash_cooldown: Timer,
	pub shoot_cooldown: Timer,
	pub beam_cooldown: Timer,
}

#[derive(Component)]
pub struct Slash{
	pub active: bool,
}

#[derive(Component)]
pub struct Star{
	pub health: f32,
}

#[derive(Component)]
pub struct Beam(pub Direction);

#[derive(Component)]
pub struct BeamBar;

#[derive(Component)]
pub struct Enemy {
	pub spec: usize,
	pub rotation: f32,
	pub dps: f32,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Corpse(pub Timer);

#[derive(Component)]
pub struct Dust(pub Timer);

#[derive(Component)]
pub struct Bullet {
	pub velocity: Vec2,
}

#[derive(Component)]
pub struct PauseHighlight;

#[derive(Component)]
pub struct SelectHighlight;

#[derive(Component)]
pub struct TruePosition(pub Vec2);

#[derive(Component, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
	Forward,
	Backward,
	Left,
	Right,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

// RESOURCES
#[derive(Resource)]
pub struct SelectedLevel(pub usize);

#[derive(Resource)]
pub struct Endless(pub bool);

#[derive(Resource)]
pub struct Retaliate(pub bool);

#[derive(Resource)]
pub struct Milky(pub bool);

#[derive(Resource)]
pub struct LoadTimes(pub bool);

#[derive(Resource)]
pub struct SecretCode(pub usize);

#[derive(Resource)]
pub struct EnemiesSlain(pub usize);

#[derive(Resource)]
pub struct BeamCharge(pub f32);

#[derive(Resource)]
pub struct WinState(pub usize);

#[derive(Resource)]
pub struct WinSpawned(pub usize);

#[derive(Resource)]
pub struct DustTimer(pub Timer);

#[derive(Resource)]
pub struct BootTimer(pub Timer);

#[derive(Resource)]
pub struct StartTimer(pub Timer);

#[derive(Resource)]
pub struct LoadingTimer(pub Timer);

#[derive(Resource)]
pub struct LoadingLength {
	pub total: usize,
	pub current: usize,
}

#[derive(Resource)]
pub struct NoEnemies(pub Timer);

#[derive(Resource)]
pub struct WinTimer(pub Timer);

#[derive(Resource)]
pub struct LevelInfo{
	pub round_timer: Timer,
	pub round: usize,
}

#[derive(Resource)]
pub struct LevelLayout{
	pub constellations: Vec<Vec<Vec2>>,
}

//#[derive(Resource)]
//pub struct BgmHandle(pub Handle<AudioInstance>);