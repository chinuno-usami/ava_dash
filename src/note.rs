use std::default;
use std::f32::consts::PI;
use super::consts::*;
use bevy::utils::tracing::span::EnteredSpan;
use bevy_kira_audio::{Audio, PlaybackState, InstanceHandle, AudioSource};
use libosu::timing::Millis;

use bevy::input::{keyboard::KeyCode, Input};

use bevy::prelude::*;
use libosu::prelude::{HitObjectKind, SampleSet, Additions};

use crate::{BeatmapInfo, BGMInstance};
use crate::score::ScoreResource;
use crate::time::ControlledTime;


struct BeatSound {
    sound: Handle<AudioSource>
}

fn play_beat_sound(
    audio: &Res<Audio>,
    //sound: &mut ResMut<BeatSound>,
    asset_server: &Res<AssetServer>,
    note_type: &NoteType
) {
    //sound.sound = asset_server.load(BEAT_SOUND);
    //audio.play(sound.sound.clone());
    audio.play(asset_server.load(
            match note_type {
                NoteType::Up => BEAT_SOUND_UP,
                NoteType::Down => BEAT_SOUND_DOWN,
            }));
}

struct NoteResource {
    up_texture: Handle<Image>,
    down_texture: Handle<Image>,
    ava_texture: Handle<Image>,
}
impl FromWorld for NoteResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        NoteResource {
            up_texture: asset_server.load("images/dr1.png"),
            down_texture: asset_server.load("images/dr2.png"),
            ava_texture: asset_server.load("images/avarun.png"),
        }
    }
}

struct NoteCount {
    count: usize,
}

#[derive(Component)]
struct AvA;

fn setup_ava(
    mut commands: Commands,
    texture: Res<NoteResource>
) {
    let mut transform = Transform::from_translation(Vec3::new(TARGET_POSITION - 50., -50., 998.));
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(128., 128.)),
                    ..Default::default()
                },
                transform,
                texture: texture.ava_texture.clone(),
                ..Default::default()
            })
            .insert(AvA);
}

#[derive(Component)]
struct Note {
    click_time: Millis,
    note_type: NoteType,
    note_hit_type: NoteHitType,
}
struct SpawnTimer(Timer);

fn spawn_note(
    mut commands: Commands,
    textures: Res<NoteResource>,
    time: Res<crate::time::ControlledTime>,
    mut count: ResMut<NoteCount>, // 当前生成到的note位置
    beatmap: Res<BeatmapInfo> // 当前的beatmap
) {
    //let secs = time.seconds_since_startup();
    let secs = time.seconds_since_startup() - 3.;
    let secs_last = secs - time.delta_seconds_f64();
    
    let objs = &beatmap.beatmaps[beatmap.index].hit_objects;
    
    //info!("SPAWN count:{}, secs:{}, last:{}", count.count, secs, s、cs_last);
    let objs = &objs[count.count..];
    for obj in objs {
        let click_time = obj.start_time;
        let spawn_time = click_time.as_seconds() - (((SPAWN_POSITION-50.-TARGET_POSITION)/NOTE_SPEED) as f64);
        //info!("SPAWN clicktime:{},spawntime:{},sec_last:{},secs:{}", click_time.as_seconds(), spawn_time, secs_last, secs);
        if secs_last < spawn_time && spawn_time < secs {
            // TODO:处理滑条长按类型
            // Notice: The slider's length can be used to determine the time it takes to complete the slider. length / (SliderMultiplier*100)*beatLength tells how many milliseconds it takes to complete one slide of the slider (assuming beatLength has been adjusted for inherited timing points).
            // TODO: 处理连打
            let note_hit_type = match obj.kind {
                HitObjectKind::Circle  => NoteHitType::Tap,
                HitObjectKind::Slider(_)  => NoteHitType::Hold,
                HitObjectKind::Spinner(_) => NoteHitType::Combo
            };
            //info!("SPAWN title:{}({}), obj:{:?}", beatmap.beatmaps[0].title, beatmap.beatmaps[0].difficulty_name ,obj);
            //info!("SPAWN title:{}({}), sample_info:{:?}", beatmap.beatmaps[0].title, beatmap.beatmaps[0].difficulty_name ,obj.sample_info);
            let note_type = match &obj.kind {
                HitObjectKind::Circle => {
                    // 取sample_info.addition_set
                    match obj.additions {
                        Additions::WHISTLE => NoteType::Up,
                        Additions::CLAP => NoteType::Up,
                        _ => NoteType::Down,
                    }
                },
                HitObjectKind::Slider(slider) => {
                    // 取开始addition
                    match slider.edge_additions[0] {
                        Additions::WHISTLE => NoteType::Up,
                        Additions::CLAP => NoteType::Up,
                        _ => NoteType::Down,
                    }
                },
                HitObjectKind::Spinner(_) => {
                    NoteType::Down
                }
            };
            let transform = Transform::from_translation(Vec3::new(SPAWN_POSITION, note_type.y(), 0.001 * count.count as f32));
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100., 100.)),
                        ..Default::default()
                    },
                    transform,
                    texture: match note_type {
                        NoteType::Up => textures.up_texture.clone(),
                        NoteType::Down => textures.down_texture.clone(),
                    },
                    ..Default::default()
                })
                .insert(Note { click_time, note_type, note_hit_type });
            count.count += 1;
        } else {
            break;
        }
    }

}

enum HitResultType {
    Perfect,
    Good,
    Bad,
    Miss,
}

fn check_hit_result(click_time: f64, current_time: f64) -> HitResultType {
    // TODO: 改为基于时间判断
    let list = [
        (HitResultType::Perfect, 0.001 * PERFECT_TIME as f64),
        (HitResultType::Good, 0.001 * GOOD_TIME as f64),
        (HitResultType::Bad, 0.001 * BAD_TIME as f64)
        ];

    info!("current:{}, click:{}", current_time, click_time);
    for (hit_type, time) in list {
        let hit = (current_time - time..=current_time + time).contains(&click_time);
        if hit {
            return hit_type;
        }
    }
    HitResultType::Miss
}

fn despawn_notes(
    mut commands: Commands,
    query: Query<(Entity, &Note)>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
    bgm: Res<BGMInstance>,
    audio: Res<Audio>,
    //mut beat_sound: ResMut<BeatSound>,
    asset_server: Res<AssetServer>
) {
    // 排序，从最近的开始判断
    let current_time = if let PlaybackState::Playing{position:pos} = audio.state(bgm.instance_handle.clone()) {
        pos
    } else {
        0.0
    };
    let mut q2 = query.iter().collect::<Vec<_>>();
    q2.sort_by(|a, b| {
        let a = (a.1.click_time.0 as f64 * 0.001 - current_time).abs();
        let b = (b.1.click_time.0 as f64 * 0.001 - current_time).abs();
        a.partial_cmp(&b).unwrap_or(core::cmp::Ordering::Equal)
    }
    );
    for (entity, note) in q2.iter() {
        let click_time = 0.001 * note.click_time.0 as f64;

        // 检查按下按键时，是否是在特定的阈值内
        if note.note_type.pressed(&mut keyboard_input) {
            match check_hit_result(click_time, current_time) {
                HitResultType::Perfect => {
                    // 播放音效
                    play_beat_sound(&audio, &asset_server, &note.note_type);
                    //play_beat_sound(&audio, &mut beat_sound, &asset_server);
                    score.increase_perfect();
                    commands.entity(*entity).remove::<Note>().insert(NoteHit);
                    //commands.entity(*entity).despawn();
                },
                HitResultType::Good => {
                    play_beat_sound(&audio, &asset_server, &note.note_type);
                    score.increase_good();
                    commands.entity(*entity).remove::<Note>().insert(NoteHit);
                    //commands.entity(*entity).despawn();
                },
                HitResultType::Bad => {
                    play_beat_sound(&audio, &asset_server, &note.note_type);
                    score.increase_bad();
                    commands.entity(*entity).remove::<Note>().insert(NoteHit);
                    //commands.entity(*entity).despawn();
                }
                _ => ()
            }
        }

        // 当离开屏幕时，消失
        if current_time - click_time > 0.001 * BAD_TIME as f64 {
            score.increase_miss();
            commands.entity(*entity).despawn();
        }
    }
}

#[derive(Component)]
struct NoteHit;

// 被击中的对象飞走
fn note_hit(
    mut commands: Commands,
    mut note: Query<(Entity, &mut Transform, &NoteHit)>,
    time: Res<ControlledTime>
) {
    for (entity, mut transform, _) in note.iter_mut() {
        let distance = transform.translation.x - -350.;
        if distance <= 0. {
            // 出屏幕后删除
            commands.entity(entity).despawn();
            continue;
        }
        // 位移
        // TODO: 用抛物线计算
        let rotation_change = Quat::from_rotation_z(2.0 * PI * 0.2 * time.delta_seconds());
        transform.rotate(rotation_change);
        transform.translation.x -= time.delta_seconds() * NOTE_SPEED * 4.;
        transform.translation.y += time.delta_seconds() * NOTE_SPEED * 4. * distance / (TARGET_POSITION + 350.);
    }
}

fn move_notes(time: Res<crate::time::ControlledTime>, mut query: Query<(&mut Transform, &Note)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.x -= time.delta_seconds() * NOTE_SPEED;
    }
}

fn cleanup(
    mut commands: Commands,
    ava: Query<(Entity, &AvA)>,
    notes: Query<(Entity, &Note)>,
    mut note_count: ResMut<NoteCount>
) {
    note_count.count = 0;
    for (ava, _) in ava.iter() {
        commands.entity(ava).despawn();
    }
    for (note, _) in notes.iter() {
        commands.entity(note).despawn();
    }
}

pub struct NotePlugin;
impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app
            // 初始化资源
            .init_resource::<NoteResource>()
            .insert_resource(NoteCount{count:0})
            //.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            // 增加 system
            .add_system_set(
                SystemSet::on_update(super::AppState::Game)
                .with_system(spawn_note)
                .with_system(move_notes)
                .with_system(despawn_notes)
                .with_system(note_hit)
            )
            .add_system_set(
                SystemSet::on_exit(super::AppState::Game)
                .with_system(cleanup)
            )
            .add_system_set(
                SystemSet::on_enter(super::AppState::Game)
                .with_system(setup_ava)
            );
    }
}

enum NoteType {
    Up,
    Down,
}

impl NoteType {
    fn pressed(&self, input: &mut Input<KeyCode>) -> bool {
        let keys = match self {
            NoteType::Up => [KeyCode::Up, KeyCode::D, KeyCode::F],
            NoteType::Down => [KeyCode::Down, KeyCode::J, KeyCode::K],
        };
        let ret = keys.iter().any(|code| input.just_pressed(*code));
        for code in keys.iter(){
            input.reset(*code);
        }
        ret
    }
    
    fn y(&self) -> f32 {
        match self {
            NoteType::Up => 50.,
            NoteType::Down => -50.,
        }
    }
}

enum NoteHitType {
    Tap,
    Hold,
    Combo,
}