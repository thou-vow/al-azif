#![feature(linked_list_cursors)]
#![allow(clippy::all)]

use al_azif_core::_prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn effects_vec_bench(c: &mut Criterion) {
    let mut vector = Vec::new();

    for _ in 0 .. 25 {
        vector.push(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 2 }));
        vector.push(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 1 }));
        vector.push(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 0 }));
        vector.push(Effect::Block(BlockEffect));
    }

    c.bench_function("vec 100", |b| b.iter(|| effects_vec(vector.clone())));
}

fn effects_vec(mut vector: Vec<Effect>) {
    let mut acting_effects = Vec::new();

    let mut i = 0;
    while i < vector.len() {
        let mut remove_effect = false;

        match &mut vector[i] {
            Effect::Bleed(bleed) => {
                acting_effects.push(Effect::Bleed(bleed.clone()));

                if bleed.turn_duration <= 0 {
                    remove_effect = true;
                } else {
                    bleed.turn_duration -= 1;
                }
            },
            _ => (),
        }

        if remove_effect {
            vector.remove(i);
        } else {
            i += 1;
        }
    }
}

fn effects_list_bench(c: &mut Criterion) {
    let mut list = LinkedList::new();

    for _ in 0 .. 25 {
        list.push_back(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 2 }));
        list.push_back(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 1 }));
        list.push_back(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 0 }));
        list.push_back(Effect::Block(BlockEffect));
    }

    c.bench_function("list 100", |b| b.iter(|| effects_list(list.clone())));
}

fn effects_list(mut list: LinkedList<Effect>) {
    let mut cursor = list.cursor_front_mut();

    let mut acting_effects = Vec::new();

    while let Some(effect) = cursor.current() {
        let mut remove_effect = false;

        match effect {
            Effect::Bleed(bleed) => {
                acting_effects.push(Effect::Bleed(bleed.clone()));

                if bleed.turn_duration <= 0 {
                    remove_effect = true;
                } else {
                    bleed.turn_duration -= 1;
                }
            },
            _ => (),
        }

        if remove_effect {
            cursor.remove_current();
        } else {
            cursor.move_next();
        }
    }
}

fn effects_deque_bench(c: &mut Criterion) {
    let mut deque = VecDeque::new();

    for _ in 0 .. 25 {
        deque.push_back(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 2 }));
        deque.push_back(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 1 }));
        deque.push_back(Effect::Bleed(BleedEffect { damage_over_turn: 10, turn_duration: 0 }));
        deque.push_back(Effect::Block(BlockEffect));
    }

    c.bench_function("deque 100", |b| b.iter(|| effects_deque(deque.clone())));
}

fn effects_deque(mut deque: VecDeque<Effect>) {
    let mut acting_effects = Vec::new();

    let mut i = 0;
    while i < deque.len() {
        let mut remove_effect = false;

        let Some(effect) = deque.get_mut(i) else {
            break;
        };

        match effect {
            Effect::Bleed(bleed) => {
                acting_effects.push(Effect::Bleed(bleed.clone()));

                if bleed.turn_duration <= 0 {
                    remove_effect = true;
                } else {
                    bleed.turn_duration -= 1;
                }
            },
            _ => (),
        }

        if remove_effect {
            deque.remove(i);
        } else {
            i += 1;
        }
    }
}

criterion_group!(benches, effects_vec_bench, effects_list_bench, effects_deque_bench);
criterion_main!(benches);
