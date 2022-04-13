use crate::{consts::*, AppState};
use bevy::prelude::*;
use bevy::app::AppExit;

struct ButtonBackground {
    none: Color,
    normal: Color,
    hovered: Color,
    pressed: Color,
    font: Handle<Font>,
}

impl FromWorld for ButtonBackground {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        ButtonBackground {
            none: Color::NONE,
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
            pressed: Color::rgb(0.35, 0.75, 0.35),
            font: asset_server.load(PIX_FONT),
        }
    }
}

#[derive(Component)]
struct MenuUI {
    // 当前选中的项
    index: usize,
    // 所有选项个数
    size: usize,
}

#[derive(Component)]
struct MenuButton {
    // 当前按钮的索引值
    index: usize,
    // 按钮事件
    //action: fn(ResMut<State<AppState>>, ) -> ResMut<State<AppState>>,
}
#[derive(Component)]
struct StartButton;
#[derive(Component)]
struct ExitButton;

fn setup_menu(mut commands: Commands, button_materials: Res<ButtonBackground>) {
    commands
        // 根节点
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: button_materials.none.into(),
            ..Default::default()
        })
        .insert(MenuUI{index:0, size: 2})
        .with_children(|parent| {
            // 标题
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                color: button_materials.none.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                // 标题
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Percent(50.0),
                                //left: Val::Percent(30.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "AVA DASH",
                            TextStyle {
                                font: button_materials.font.clone(),
                                font_size: 110.0,
                                color: Color::rgb_u8(201, 212, 253),
                            },
                            // Note: You can use `Default::default()` in place of the `TextAlignment`
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                ..Default::default()
                            },
                        ),
                        ..Default::default()
                    });
            });
            // 选单
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                color: button_materials.none.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                // 开始
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(30.0), Val::Percent(20.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: button_materials.none.into(),
                        ..Default::default()
                    })
                    .insert(MenuButton{
                        index:0,
                    })
                    .insert(StartButton)
                    .with_children(|parent| {
                        let mut sections = Vec::<TextSection>::new();
                        sections.push(TextSection{
                            value: "开始".to_string(),
                            style: TextStyle { 
                                font: button_materials.font.clone(),
                                font_size: 24.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..Default::default()
                            }
                        });

                        parent.spawn_bundle(TextBundle {
                            text: Text { 
                                sections,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
                })
            .with_children(|parent| {
                // 退出
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(30.0), Val::Percent(20.0)),
                            //size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: button_materials.none.into(),
                        ..Default::default()
                    })
                    .insert(MenuButton{
                        index:1,
                    })
                    .insert(ExitButton)
                    .with_children(|parent| {
                        let mut sections = Vec::<TextSection>::new();
                        sections.push(TextSection{
                            value: "退出".to_string(),
                            style: TextStyle { 
                                font: button_materials.font.clone(),
                                font_size: 24.0,
                                color: Color::rgb(0.0, 0.0, 0.0),
                                ..Default::default()
                            }
                        });

                        parent.spawn_bundle(TextBundle {
                            text: Text { 
                                sections,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
                
            });
        });
}

fn despawn_menu(mut commands: Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// 按下按键时查找按钮、更新选择状态更新按钮样式
fn select_button(
    mut ui: Query<&mut MenuUI>,
    mut input: ResMut<Input<KeyCode>>
) {
    let mut ui = ui.single_mut();
    // 判断按键上下选择
    let up = [KeyCode::Up, KeyCode::D, KeyCode::F];
    let down = [KeyCode::Down, KeyCode::J, KeyCode::K];

    // 上
    let mut ret = up.iter().any(|code| input.just_pressed(*code));
    for code in up.iter(){
        input.reset(*code);
    }
    if ret {
        if ui.index == 0 {
            ui.index = ui.size - 1;
        } else {
            ui.index -= 1;
        }
        info!("up idx:{}", ui.index)
    }
    // 下
    ret = down.iter().any(|code| input.just_pressed(*code));
    for code in up.iter(){
        input.reset(*code);
    }
    if ret {
        if ui.index >= ui.size -1 {
            ui.index = 0;
        } else {
            ui.index += 1;
        }
    }
}

fn start_button(
    button_materials: Res<ButtonBackground>,
    ui: Query<&MenuUI>,
    mut button: Query<(&StartButton, &MenuButton, &mut UiColor)>,
    mut input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>
) {
    let ui = ui.single();
    let (_, button, mut color) = button.single_mut();
    if ui.index == button.index {
        *color = button_materials.hovered.into();
        let keys = [KeyCode::Return];
        let pressed = keys.iter().any(|code| input.just_pressed(*code));
        for code in keys.iter(){
            input.reset(*code);
        }
        if pressed {
            *color = button_materials.pressed.into();
            app_state.set(AppState::Beatmap).unwrap();
            //app_state.set(AppState::Game).unwrap();
        }
    } else {
        *color = button_materials.normal.into();
    }
}

fn exit_button(
    button_materials: Res<ButtonBackground>,
    ui: Query<&MenuUI>,
    mut button: Query<(&ExitButton, &MenuButton, &mut UiColor)>,
    mut input: ResMut<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    let ui = ui.single();
    let (_, button, mut color) = button.single_mut();
    if ui.index == button.index {
        *color = button_materials.hovered.into();
        let keys = [KeyCode::Return];
        let pressed = keys.iter().any(|code| input.just_pressed(*code));
        for code in keys.iter(){
            input.reset(*code);
        }
        if pressed {
            *color = button_materials.pressed.into();
            exit.send(AppExit);
        }
    } else {
        *color = button_materials.normal.into();
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonBackground>()
            .add_system_set(
                SystemSet::on_update(super::AppState::Menu)
                .with_system(select_button)
                .with_system(start_button)
                .with_system(exit_button)
            )
            .add_system_set(
                SystemSet::on_enter(super::AppState::Menu)
                .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_exit(super::AppState::Menu)
                .with_system(despawn_menu)
            );
    }
}