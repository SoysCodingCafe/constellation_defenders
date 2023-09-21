// Import Bevy game engine essentials
use bevy::{prelude::*, app::AppExit};

use crate::derivables::*;

// Plugin for devtools only available in the
// debug version of the game
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems( Startup, (
				filler,
			//	spawn_palette,
			))
			.add_systems( Update, (
				quit_game,
			))
		;
	}
}

fn filler(

) {

}

fn spawn_palette(
	mut commands: Commands,
) {
	//let colors = [COLOR_A, COLOR_B, COLOR_C, COLOR_D];
	for i in 0..4 {
		commands.spawn((
			SpriteBundle{
				transform: Transform::from_xyz(-20.0 + 20.0 * i as f32, -20.0, i as f32 + 1.0),
				sprite: Sprite {
	//				color: colors[i],
					custom_size: Some(Vec2::new(30.0, 30.0)),
					..default()},
				..default()
			},
		));
	}
}

fn quit_game(
	keyboard: Res<Input<KeyCode>>,
	mut ev_w_exit: EventWriter<AppExit>,
) {
	if keyboard.just_pressed(KeyCode::Escape) {
		ev_w_exit.send(AppExit);
	}
}