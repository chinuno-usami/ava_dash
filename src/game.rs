use bevy::prelude::{Plugin, App, SystemSet, Res, ResMut, AssetServer, State, Local, ConfigurableSystem, info};
use bevy_kira_audio::{Audio, PlaybackState};

use crate::AppState;
use crate::time::ControlledTime;
use crate::score::ScoreResource;

use super::BGMInstance;
fn play_music(
    beatmap: Res<crate::BeatmapInfo>,
    asset_server:Res<AssetServer>,
    mut instance: ResMut<BGMInstance>,
    audio: Res<Audio>,
    time: Res<ControlledTime>
) {
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= 3. && 3. <= secs {
        let path = crate::load_beatmap_resource(&beatmap.beatmaps[beatmap.index].audio_filename, &beatmap.path);
        let asset_handle = asset_server.load(path);
        instance.instance_handle = audio.play(asset_handle);
    }
}

fn stop_music(audio: Res<Audio>) {
    audio.stop();
}

#[derive(Default)]
struct MusicPlayed(bool);

fn gameover(
    score: Res<ScoreResource>,
    bgm: Res<BGMInstance>,
    audio: Res<Audio>,
    mut app_state: ResMut<State<AppState>>,
    mut played: Local<MusicPlayed>,
    time: Res<ControlledTime>
) {
    if score.hp <= 0. {
        played.0 = false;
        info!("hp < 0");
        app_state.set(AppState::Result).unwrap();
        return;
    }
    
    //info!("bgm played:{}", played.0);
    match audio.state(bgm.instance_handle.clone())  {
        PlaybackState::Playing { position:_ } => {
            let secs = time.seconds_since_startup();
            if secs > 3. {
                //info!("bgm playing");
                played.0 = true;
            }
        },
        PlaybackState::Stopped => {
            if played.0 {
                // 转到结算画面
                played.0 = false;
                info!("bgm end,played:{}", played.0);
                app_state.set(AppState::Result).unwrap();
            }
        },
        _ => ()
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(super::AppState::Game)
                .with_system(play_music)
                .with_system(gameover.config(|param| {
                    param.4 = Some(MusicPlayed(false));
                }) )
            )
            .add_system_set(
                SystemSet::on_enter(super::AppState::Game)
                .with_system(stop_music)
            );
            //.add_system(spawn_note.system())
            //.add_system(move_notes.system());
    }
}