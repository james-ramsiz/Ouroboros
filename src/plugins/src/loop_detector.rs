use bevy::prelude::*;
use crate::components::*;

const LOOP_TRIGGER_DIST: f32 = 1.0;
const MIN_SEGS_FOR_LOOP: usize = 10;
const LOOP_COOLDOWN:     f32  = 1.5;

#[derive(Resource)]
pub struct LoopCooldown(pub f32);

pub struct LoopPlugin;

impl Plugin for LoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LoopCount::default())
            .insert_resource(LoopCooldown(0.0))
            .add_event::<LoopCompletedEvent>()
            .add_systems(Update, (
                detect_loop,
                handle_loop_completed,
            ));
    }
}

fn detect_loop(
    time:      Res<Time>,
    positions: Res<SegmentPositions>,
    mut cooldown: ResMut<LoopCooldown>,
    mut events: EventWriter<LoopCompletedEvent>,
) {
    cooldown.0 = (cooldown.0 - time.delta_seconds()).max(0.0);
    if cooldown.0 > 0.0 { return; }

    let chain = &positions.0;
    if chain.len() < MIN_SEGS_FOR_LOOP { return; }

    let head = chain[0];
    let tail = *chain.last().unwrap();

    if head.distance(tail) < LOOP_TRIGGER_DIST {
        cooldown.0 = LOOP_COOLDOWN;
        events.send(LoopCompletedEvent);
    }
}

fn handle_loop_completed(
    mut events:    EventReader<LoopCompletedEvent>,
    mut count:     ResMut<LoopCount>,
    mut positions: ResMut<SegmentPositions>,
) {
    for _ in events.read() {
        count.0 += 1;
        info!("Loop {} completed!", count.0);

        // Grow: duplicate the last two positions
        if let Some(&last) = positions.0.last() {
            positions.0.push(last);
            positions.0.push(last);
        }
    }
}
