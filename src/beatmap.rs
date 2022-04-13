use bevy::{prelude::*, sprite::{SpriteBundle, Sprite}, window::{Windows, self}, math::{Vec3, Size, Rect}, ui::{Style, FlexDirection, JustifyContent, Val, AlignSelf, Overflow, PositionType, UiColor}, text::{Text, TextStyle}, input::Input};
use bevy_kira_audio::Audio;
use bevy_render::options::Backends;
use crate::{BeatmapList, BeatmapInfo, BGMInstance, AppState};
use libosu::{prelude::Event::Background, beatmap};
use crate::consts::*;


#[derive(Component)]
struct BackgroundImg(Handle<Image>);

#[derive(Component)]
struct MainMenu;
#[derive(Component)]
struct BeatmapMenu(usize);

#[derive(Component)]
struct VersionMenu(usize);
#[derive(Component)]
struct VersionItem(usize);

#[derive(Component)]
struct BeatmapItem(usize);

fn add_bg(
    commands: &mut Commands,
    beatmap: &BeatmapInfo,
    asset_server: &Res<AssetServer>
) {
    let bgev = beatmap.beatmaps[beatmap.index].events.iter().find(|item| {
        match item {
            Background(_) => true,
            _ => false
        }
    });
    if let Some(Background(ev)) = bgev {
        let img = &ev.filename;
        let path = crate::load_beatmap_resource(&img, &beatmap.path);
        let texture = asset_server.load(path);
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    ..Default::default()
                },
                texture: texture.clone(),
                ..Default::default()
            })
            .insert(BackgroundImg(texture));
    };
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    beatmaps: Res<BeatmapList>,
    current_map: Res<BeatmapInfo>
) {
    // 设置背景图为当前图的背景
    add_bg(&mut commands, &current_map, &asset_server);
    //let bgev = current_map.beatmaps[current_map.index].events.iter().find(|item| {
    //    match item {
    //        Background(_) => true,
    //        _ => false
    //    }
    //});
    //if let Some(Background(ev)) = bgev {
    //    let img = &ev.filename;
    //    let path = crate::load_beatmap_resource(&img, &current_map.path);
    //    let texture = asset_server.load(path);
    //    commands
    //        .spawn_bundle(SpriteBundle {
    //            sprite: Sprite {
    //                ..Default::default()
    //            },
    //            texture: texture.clone(),
    //            ..Default::default()
    //        })
    //        .insert(BackgroundImg(texture));
    //};
    // 设置选图列表
    commands.spawn_bundle(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            //justify_content: JustifyContent::FlexEnd,
            //size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
            ..Default::default()
        },
        color: Color::NONE.into(),
        //color: Color::rgb(0.15, 0.95, 0.15).into(),
        ..Default::default()
    })
    .insert(MainMenu)
    .with_children(|parent| {
        // 选谱
        parent.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                //align_self: AlignSelf::FlexEnd,
                //size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                overflow: Overflow::Hidden,
                //position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            //color: Color::rgb(0.95, 0.15, 0.15).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // 滚动部分
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    flex_grow: 1.0,
                    max_size: Size::new(Val::Undefined, Val::Undefined),
                    position: Rect {
                        top: Val::Px(300.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                color: Color::NONE.into(),
                //color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            })
            .insert(BeatmapMenu(0))
            .with_children(|parent| {
                // 填充列表
                for (idx, beatmap) in (*beatmaps).beatmaps.iter().enumerate() {
                    let mut title = &beatmap.beatmaps[0].title_unicode;
                    if title.is_empty() {
                        title = &beatmap.beatmaps[0].title;
                    }
                    let mut author = &beatmap.beatmaps[0].artist_unicode;
                    if title.is_empty() {
                        author = &beatmap.beatmaps[0].artist;
                    }
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::ColumnReverse,
                            align_self: AlignSelf::Center,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            //overflow: Overflow::Hidden,
                            ..Default::default()
                        },
                        color: Color::Rgba { red: 0., green: 0., blue: 0., alpha: 0.8 }.into(),
                        ..Default::default()
                    })
                    .insert(BeatmapItem(idx))
                    .with_children(|parent|{
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                flex_shrink: 0.,
                                size: Size::new(Val::Undefined, Val::Px(22.)),
                                margin: Rect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text::with_section(
                                format!("{} - {}", title, author),
                                TextStyle {
                                    font: asset_server
                                        .load(PIX_FONT),
                                    font_size: 22.,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                            
                        });
                    });
                }
            });
        });
        // 选难度
        // TODO: 没时间做，后面补，先用第一个version
        //parent.spawn_bundle(NodeBundle {
        //    style: Style {
        //        flex_direction: FlexDirection::ColumnReverse,
        //        align_self: AlignSelf::Center,
        //        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
        //        overflow: Overflow::Hidden,
        //        ..Default::default()
        //    },
        //    //color: Color::rgb(0.15, 0.15, 0.15).into(),
        //    color: Color::NONE.into(),
        //    ..Default::default()
        //})
        //.insert(VersionMenu(0));
    });
}

fn fit_bg_size(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut bg: Query<(&mut Transform, &BackgroundImg)>,
    assets: Res<Assets<Image>>,
    windows: Res<Windows>
) {
    let (mut transform, bg) = bg.single_mut();
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if *handle == bg.0 {
                    let img = assets.get(bg.0.clone()).unwrap();
                    //let width = img.texture_descriptor.size.width;
                    let height = img.texture_descriptor.size.height;
                    let window_height = windows.get_primary().unwrap().height();
                    //let scale = 100.;
                    let scale = window_height / height as f32;
                    //*transform = Transform::from_scale(Vec3::new(100., 100., 0.));
                    transform.scale = bevy::prelude::Vec3::new(scale, scale, 0.0);
                    info!("change bg scale:{}, window h:{}, img h:{}", scale, window_height, height);
                }
            },
            _ => {

            }
        }
    }
}

fn update_item_bg(
    menu: Query<&BeatmapMenu>,
    mut item: Query<(&mut UiColor, &BeatmapItem)>
) {
    let menu = menu.single();
    for (mut color, item) in item.iter_mut() {
        // 如果idx相等改变背景颜色
        if menu.0 == item.0 {
            *color = Color::rgba(0.0, 0.0, 0.0, 0.95).into();
        } else {
            *color = Color::rgba(0.0, 0.0, 0.0, 0.8).into();
        }
    }
}


fn select_beatmap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu: Query<(&mut Style, &mut BeatmapMenu)>,
    mut input: ResMut<Input<KeyCode>>,
    list: Res<BeatmapList>,
    mut current_map: ResMut<BeatmapInfo>,
    bg: Query<(Entity, &BackgroundImg)>,
    audio: Res<Audio>,
    mut instance: ResMut<BGMInstance>,
    mut app_state: ResMut<State<AppState>>
) {
    let (mut style, mut menu) = menu.single_mut();
    // 判断按键上下选择
    let up = [KeyCode::Up, KeyCode::D, KeyCode::F];
    let down = [KeyCode::Down, KeyCode::J, KeyCode::K];
    let enter = [KeyCode::Return];

    let mut ret = enter.iter().any(|code| input.just_pressed(*code));
    for code in enter.iter(){
        input.reset(*code);
    }
    if ret {
        // TODO: 应该先选version再进游戏
        // 时间不够，先直接用第一个version开始
        //
        // 选择曲谱。进入游戏界面
        app_state.set(AppState::Game).unwrap();
        info!("select idx:{}", menu.0);
    }
    
    // 没有处理的必要
    if list.beatmaps.len() < 2 {
        return;
    }
    let old_idx = menu.0;
    // 上
    ret = up.iter().any(|code| input.just_pressed(*code));
    for code in up.iter(){
        input.reset(*code);
    }
    if ret {
        if menu.0 == 0 {
            menu.0 = list.beatmaps.len() - 1;
        } else {
            menu.0 -= 1;
        }
        info!("up idx:{}", menu.0);
        style.position.top = Val::Px(300. - menu.0 as f32 * 22.);
    }
    // 下
    ret = down.iter().any(|code| input.just_pressed(*code));
    for code in down.iter(){
        input.reset(*code);
    }
    if ret {
        if menu.0 >= list.beatmaps.len() - 1 {
            menu.0 = 0;
        } else {
            menu.0 += 1;
        }
        style.position.top = Val::Px(300. - menu.0 as f32 * 22.);
        info!("down idx:{}", menu.0);
    }
    // 更新当前map信息，更换BGM、更换背景图
    if old_idx != menu.0 {
        *current_map = list.beatmaps[menu.0].clone();
        let (bg, _) = bg.single();
        commands.entity(bg).despawn();
        add_bg(&mut commands, &current_map, &asset_server);
        info!("select idx:{}", menu.0);
        audio.stop();
        let path = crate::load_beatmap_resource(&current_map.beatmaps[current_map.index].audio_filename, &current_map.path);
        let asset_handle = asset_server.load(path);
        instance.instance_handle = audio.play_looped(asset_handle);
    }
}

fn despawn_menu(mut commands: Commands, menu: Query<(Entity, &MainMenu)>,
                bg: Query<(Entity, &BackgroundImg)>
) {
    for (entity, _) in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for (entity, _) in bg.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct BeatmapPlugin;
impl Plugin for BeatmapPlugin {
    fn build(&self, app: &mut App) {
        app//.init_resource::<ButtonBackground>()
            .add_system_set(
                SystemSet::on_update(super::AppState::Beatmap)
                .with_system(fit_bg_size)
                .with_system(update_item_bg)
                .with_system(select_beatmap)
            )
            .add_system_set(
                SystemSet::on_enter(super::AppState::Beatmap)
                .with_system(setup)
            )
            .add_system_set(
                SystemSet::on_exit(super::AppState::Beatmap)
                // 停止播放。清除选图列表
                .with_system(despawn_menu)
            );
    }
}