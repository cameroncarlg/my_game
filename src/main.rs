use bevy::window::PrimaryWindow;
use bevy::{color::palettes::basic::PURPLE, prelude::*};

const PLAYER_SPEED: f32 = 900.;
const PLAYER_SIZE: f32 = 64.;

//#[derive(Component)]
//enum Direction {
//    Left,
//    Right,
//}

#[derive(Component)]
struct Player;

//fn setup_for_circle(
//    mut commands: Commands,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<ColorMaterial>>,
//) {
//    commands.spawn(Camera2d);
//    commands.spawn((
//        Mesh2d(meshes.add(Circle::default())),
//        MeshMaterial2d(materials.add(Color::from(PURPLE))),
//        Transform::default().with_scale(Vec3::splat(128.)),
//    ));
//}

fn spawn_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("samurai.png")),
        Transform::from_xyz(0., 0., 0.),
        //Direction::Right,
    ));
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new("Welcome to Samurai VERSUS Knight!\n...FIGHT!"),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
}

// input works!!!!!!
fn move_sprite(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    // This handles the input but does not UPDATE with respect to time
    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the players y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_sprite, setup_instructions))
        .add_systems(Update, (move_sprite, confine_player_movement).chain())
        .run();
}
