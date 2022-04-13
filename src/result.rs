use bevy::prelude::*;
use crate::AppState;
use crate::consts::*;

use crate::BeatmapInfo;
use crate::score::ScoreResource;


#[derive(Component)]
struct ResultUI;

fn setup_result_ui(
    mut commands: Commands,
    beatmap: Res<BeatmapInfo>,
    asset_server: Res<AssetServer>,
    score: Res<ScoreResource>
) {
    // 顶部曲名
    // 下左统计信息
    // 下右分数、评级
    commands
        // 根节点
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                //align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            //color: Color::Rgba { red: 0., green: 1., blue: 0., alpha: 1. }.into(),
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(ResultUI)
        .with_children(|parent| {
            // 曲名
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Px(70.)),
                    //size: Size::new(Val::Px(800.), Val::Px(66.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    //margin: Rect {
                    //    left: Val::Px(0.),
                    //    right: Val::Px(0.),
                    //    ..Default::default()
                    //},
                    ..Default::default()
                },
                color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0.95 }.into(),
                ..Default::default()
            })
            .with_children(|parent|{
                let title = beatmap.beatmaps[beatmap.index].title_unicode.clone();
                let author = beatmap.beatmaps[beatmap.index].artist_unicode.clone();
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        flex_shrink: 0.,
                        size: Size::new(Val::Percent(100.), Val::Px(33.)),
                        margin: Rect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: title,
                                style: TextStyle {
                                    font: asset_server
                                        .load(PIX_FONT),
                                    font_size: 33.,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                });
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        flex_shrink: 0.,
                        size: Size::new(Val::Percent(100.), Val::Px(33.)),
                        margin: Rect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: author,
                                style: TextStyle {
                                    font: asset_server
                                        .load(PIX_FONT),
                                    font_size: 22.,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                });
            });
            // 统计信息
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    //size: Size::new(Val::Percent(100.), Val::Percent(88.)),
                    size: Size::new(Val::Percent(100.), Val::Px(530.)),
                    //size: Size::new(Val::Px(800.), Val::Px(66.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    //margin: Rect::all(Val::Percent(10.)),
                    //margin: Rect {
                    //    left: Val::Percent(10.),
                    //    //right: Val::Px(0.),
                    //    bottom: Val::Percent(5.),
                    //    ..Default::default()
                    //},
                    ..Default::default()
                },
                //color: Color::Rgba { red: 1., green: 0., blue: 0., alpha: 0.95 }.into(),
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                // 各类型
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        flex_shrink: 0.,
                        //size: Size::new(Val::Percent(100.), Val::Px(530.)),
                        //size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        //margin: Rect::all(Val::Percent(10.)),
                        margin: Rect { right: Val::Px(50.), ..Default::default() },
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            // perfect
                            TextSection {
                                value: format!("完美:{}", score.perfect),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 55.,
                                    color: Color::GOLD,
                                },
                            },
                            //// good
                            TextSection {
                                value: format!("\n\n還行:{}", score.good),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 55.,
                                    color: Color::PINK,
                                },
                            },
                            // bad
                            TextSection {
                                value: format!("\n\n不好:{}", score.bad),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 55.,
                                    color: Color::PURPLE,
                                },
                            },
                            // miss
                            TextSection {
                                value: format!("\n\n沒中:{}", score.miss),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 55.,
                                    color: Color::INDIGO,
                                },
                            },
                        ],
                        alignment: TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center }
                    },
                    ..Default::default()
                });
                let rank = if score.hp <= 0. {
                    "F"
                } else if score.acc > 95. {
                    "S"
                } else if score.acc > 90. {
                    "A"
                } else if score.acc > 80. {
                    "B"
                } else if score.acc > 70. {
                    "C"
                } else {
                    "D"
                };
                // 分数、准确率、评级
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        flex_shrink: 0.,
                        //size: Size::new(Val::Percent(100.), Val::Px(530.)),
                        //size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        //margin: Rect::all(Val::Percent(10.)),
                        margin: Rect { left: Val::Px(50.), ..Default::default() },
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            // perfect
                            TextSection {
                                value: format!("評級:{}", rank),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 66.,
                                    color: Color::SALMON,
                                },
                            },
                            //// good
                            TextSection {
                                value: format!("\n\n分數:{}", score.score),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 55.,
                                    color: Color::AZURE,
                                },
                            },
                            // bad
                            TextSection {
                                value: format!("\n\n準確率:{:.2}", score.acc*100.),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 55.,
                                    color: Color::ORANGE,
                                },
                            },
                            // miss
                            TextSection {
                                value: format!("\n\n最大連擊:{}", score.max_combo),
                                style: TextStyle {
                                    font: asset_server.load(PIX_FONT),
                                    font_size: 55.,
                                    color: Color::BISQUE,
                                },
                            },
                        ],
                        alignment: TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center }
                    },
                    ..Default::default()
                });
            })
            ;
            
        });
}

fn cleanup_result_ui(
    mut commands: Commands,
    ui: Query<(Entity, &ResultUI)>,
    mut score: ResMut<ScoreResource>
) {
    // 重置分数状态
    *score = ScoreResource::default();
    for (ui, _) in ui.iter() {
        commands.entity(ui).despawn_recursive();
    }
}

fn go_back(
    mut input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    
    let enter = [KeyCode::Return];

    let ret = enter.iter().any(|code| input.just_pressed(*code));
    for code in enter.iter(){
        input.reset(*code);
    }
    if ret {
        // 返回选曲界面
        app_state.set(AppState::Beatmap).unwrap();
    }
}

pub struct ResultUIPlugin;
impl Plugin for ResultUIPlugin {
    fn build(&self, app: &mut App) {
        app//.add_startup_system(setup_ui.system())
            .add_system_set(
                SystemSet::on_update(crate::AppState::Result)
                .with_system(go_back)
            )
            .add_system_set(
                SystemSet::on_exit(crate::AppState::Result)
                .with_system(cleanup_result_ui)
            )
            .add_system_set(
                SystemSet::on_enter(crate::AppState::Result)
                .with_system(setup_result_ui)
            );
    }
}