//! Loading screen that handles asset loading

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::ScreenState;
use crate::rendering::resources::TextureAssets;
use echos_assets::entities::EntityDefinitions;

/// Loading screen plugin that handles asset loading
pub fn plugin(app: &mut App) {
    // Configure asset loading
    app.add_loading_state(
        LoadingState::new(ScreenState::Loading)
            .continue_to_state(ScreenState::Gameplay)
            .load_collection::<TextureAssets>()
            .load_collection::<EntityDefinitions>()
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("entities.assets.ron"),
    );

    // Add loading screen UI and animation systems
    app.add_systems(OnEnter(ScreenState::Loading), setup_loading_screen)
        .add_systems(Update, animate_loading_screen.run_if(in_state(ScreenState::Loading)))
        .add_systems(OnExit(ScreenState::Loading), cleanup_loading_screen);
}

/// Marker component for loading screen entities
#[derive(Component)]
struct LoadingScreen;

/// Component for the loading text
#[derive(Component)]
struct LoadingText;

/// Component for the progress bar
#[derive(Component)]
struct ProgressBar;

/// Set up the loading screen UI
fn setup_loading_screen(mut commands: Commands) {
    info!("Setting up loading screen...");

    // Spawn camera for loading screen
    commands.spawn((Camera2d, LoadingScreen));

    // Create loading screen UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            LoadingScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Echos in the Dark"),
                TextFont { font_size: 48.0, ..default() },
                TextColor(Color::WHITE),
                Node { margin: UiRect::bottom(Val::Px(50.0)), ..default() },
            ));

            // Loading text with animated dots
            parent.spawn((
                Text::new("Loading..."),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
                Node { margin: UiRect::bottom(Val::Px(20.0)), ..default() },
                LoadingText,
            ));

            // Simple progress indicator (animated bar)
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::WHITE),
                ))
                .with_children(|parent| {
                    // Animated progress bar
                    parent.spawn((
                        Node { width: Val::Percent(0.0), height: Val::Percent(100.0), ..default() },
                        BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                        ProgressBar,
                    ));
                });
        });
}

/// Simple animation for the loading screen
fn animate_loading_screen(
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<LoadingText>>,
    mut bar_query: Query<&mut Node, With<ProgressBar>>,
) {
    let elapsed = time.elapsed_secs();

    // Animate loading text dots
    if let Ok(mut text) = text_query.single_mut() {
        let dots = match ((elapsed * 2.0) as u32) % 4 {
            0 => "",
            1 => ".",
            2 => "..",
            _ => "...",
        };
        **text = format!("Loading{}", dots);
    }

    // Animate progress bar with a sine wave
    if let Ok(mut style) = bar_query.single_mut() {
        let progress = ((elapsed * 0.5).sin() + 1.0) * 50.0; // 0-100%
        style.width = Val::Percent(progress);
    }
}

/// Clean up loading screen entities
fn cleanup_loading_screen(mut commands: Commands, loading_entities: Query<Entity, With<LoadingScreen>>) {
    info!("Cleaning up loading screen...");
    for entity in &loading_entities {
        commands.entity(entity).despawn();
    }
}
