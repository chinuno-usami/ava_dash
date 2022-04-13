mod ui;
mod note;
mod consts;
mod score;
mod menu;
mod game;
mod time;
mod result;
mod beatmap;
use bevy_kira_audio::{Audio, AudioPlugin, InstanceHandle, AudioSource};
use bevy::{input::system::exit_on_esc_system, prelude::*, };
use bevy_loading:: prelude ::*;
use libosu::prelude::Beatmap;
use note::NotePlugin;
use score::ScoreResource;
use std::{vec::Vec, env, fs, path::PathBuf};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub enum AppState {
    Loading,
    Menu,
    Game,
    Pause,
    Beatmap,
    Result,
    Setting,
}

#[derive(Debug, Clone)]
struct BeatmapInfo {
    beatmaps: Vec<Beatmap>, // libosu的结果.一张图多个难度
    index: usize,   // 当前难度
    path: String, // 曲谱目录
}
struct BeatmapList {
    beatmaps: Vec<BeatmapInfo>,
}

struct BGMInstance {
    instance_handle: InstanceHandle,
}

struct BackgroundResource {
    background: Handle<Image>,
}

fn load_beatmaps(mut commands: Commands, mut app_state: ResMut<State<AppState>>) { //-> Progress{
    // 读取beatmaps目录加载曲谱
    info!("开始加载曲谱");
    let mut list = Vec::new();
    let cur_path = std::env::current_exe().unwrap();
    let cur_dir = cur_path.parent().unwrap();
    let beatmaps_dir = fs::read_dir(cur_dir.join("beatmaps"));
    if let Ok(dirs) = beatmaps_dir {
        for dir in dirs {
            let dir = dir.unwrap().path();
            let mut info = BeatmapInfo{ beatmaps: Vec::new(), index: 0, path: dir.to_str().unwrap().to_string()};
            info!("发现osu文件：{}", info.path);
            let osu_files = fs::read_dir(dir).unwrap();
            for osu in osu_files {
                let osu = osu.unwrap();
                info!("file:{:?}, ext:{:?}", osu.path(), osu.path().extension());
                if let Some(ext) = osu.path().extension() {
                    if ext != "osu" {
                        continue;
                    }
                } else {
                    continue;
                }
                let osu = fs::File::open(osu.path()).unwrap();
                let beatmap = Beatmap::parse(osu);
                if let Ok(beatmap) = beatmap {
                    info!("发现osu文件 标题：{}, 难度: {}({})",  beatmap.title, beatmap.difficulty_name, beatmap.difficulty.overall_difficulty);
                    info.beatmaps.push(beatmap);
                }
            }
            // 如果解析出数据，加入到曲谱列表
            if !info.beatmaps.is_empty() {
                list.push(info);
            } 
        }
        if !list.is_empty() {
            let cur_beatmap = list[0].clone();
            commands.insert_resource(cur_beatmap);
        }
        commands.insert_resource(BeatmapList{beatmaps:list});
    }
    //app_state.set(AppState::Game).unwrap();
    app_state.set(AppState::Menu).unwrap();
    //true.into()
}

fn main() {
    //init_log();
    App::new()
        // 抗锯齿设置 samples 为 4
        //.insert_resource(Msaa { samples: 4 })
        // 设置 WindowDescriptor 资源修改标题和窗口大小
        .insert_resource(WindowDescriptor {
            title: "AvA Dash".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_state(AppState::Loading)
        //.add_plugin(LoadingPlugin {
        //    loading_state: AppState::Loading,
        //    next_state: AppState::Menu,
        //})
        //.insert_resource(State::new(state::AppState::Menu))
//        .add_stage_after( // <--- 新代码
//            stage::UPDATE,
//            APP_STATE_STAGE,
//            StateStage::<AppState>::default(),
//        )
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(NotePlugin)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(load_beatmaps)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Menu)
            .with_system(play_bgm)
        )
        .add_plugin(ui::UIPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(time::TimePlugin)
        .add_plugin(beatmap::BeatmapPlugin)
        .add_plugin(result::ResultUIPlugin)
        .insert_resource(ScoreResource::default())
        .run();
}


fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .spawn_bundle(UiCameraBundle::default());
    //load_beatmaps(commands);
}

fn load_beatmap_resource(file: &String, path: &String) -> std::path::PathBuf{
    let path = std::path::Path::new(path);
    let path = path.join(file);
    path
}

fn play_bgm(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>, beatmap: Res<BeatmapInfo>) {
//    let file = &beatmap.beatmaps[0].audio_filename;
//    let path = std::path::Path::new(&beatmap.path);
//    let path = path.join(&file);
    let path = load_beatmap_resource(&beatmap.beatmaps[beatmap.index].audio_filename, &beatmap.path);
    let asset_handle = asset_server.load(path);
    let instance_handle = audio.play_looped(asset_handle);
    commands.insert_resource(BGMInstance { instance_handle });
}