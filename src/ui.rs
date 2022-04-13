use bevy::prelude::*;

#[derive(Component)]
struct GameUI;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load(crate::consts::PIX_FONT);
    let material = color_materials.add(Color::NONE.into());

    commands
        // 时间文本节点
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(12.),
                    top: Val::Px(12.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameUI)
        .with_children(|parent| {
            let mut sections = Vec::<TextSection>::new();
            sections.push(TextSection{
                value: "时间：0.0".to_string(),
                style: TextStyle { font: font.clone(), font_size: 12.0, color: Color::rgb(0.0, 0.0, 0.0) }
            });

            parent
                .spawn_bundle(TextBundle {
                    text: Text { 
                        sections,
                        ..Default::default()
                    },
                    ..Default::default()
                    }
                )
                .insert(TimeText);
            });
        commands.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(12.),
                    bottom: Val::Px(12.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameUI)
        .with_children(|parent| {
            let mut sections = Vec::<TextSection>::new();
            sections.push(TextSection{
                value: "score:".to_string(),
                style: TextStyle { font: font.clone(), font_size: 12.0, color: Color::rgb(0.0, 0.0, 0.0) }
            });

            parent
                .spawn_bundle(TextBundle {
                    text: Text { 
                        sections,
                        ..Default::default()
                    },
                    ..Default::default()
                    }
                )
                .insert(ScoreText);
        });
}

#[derive(Component)]
struct TimeText;

fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    // 歌曲在实时启动 3 秒后开始
    let secs = time.seconds_since_startup() - 3.;

    // 在歌曲开始播放前不做任何处理
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

use crate::ScoreResource;

#[derive(Component)]
struct ScoreText;

fn update_score_text(score: Res<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {} Hp: {}. Perfect: {}. Good: {}. Bad: {}. Miss: {}. Acc: {}. Combo: {}. Max Combo: {}",
            score.score,
            score.hp,
            score.perfect,
            score.good,
            score.bad,
            score.miss,
            score.acc,
            score.combo,
            score.max_combo
        );
    }
}

fn cleanup_ui(
    mut commands: Commands,
    ui: Query<(Entity, &GameUI)>
) {
    for (ui, _) in ui.iter() {
        commands.entity(ui).despawn_recursive();
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app//.add_startup_system(setup_ui.system())
            .add_system_set(
                SystemSet::on_update(crate::AppState::Game)
                .with_system(update_time_text)
                .with_system(update_score_text)
            )
            .add_system_set(
                SystemSet::on_exit(crate::AppState::Game)
                .with_system(cleanup_ui)
            )
            .add_system_set(
                SystemSet::on_enter(crate::AppState::Game)
                .with_system(setup_ui)
            );
    }
}