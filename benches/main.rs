use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pulsar::game::PulsarGame;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn create_random_string(length: usize) -> String {
        thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
}

fn create_remove_entity_state(game: &mut PulsarGame, state: &str) {
        game.create_entity_state(state);
        game.remove_entity_state(state);
}

fn create_remove_entity(game: &mut PulsarGame, state: &str) {
        let id = game.create_entity(state).unwrap();
        
        #[allow(dead_code)]
        game.remove_entity(id, Some(state));
}

fn create_remove_entity_no_state(game: &mut PulsarGame, state: &str) {
        let id = game.create_entity(state).unwrap();
        
        #[allow(dead_code)]
        game.remove_entity(id, None);
}

fn swap_entity_state(game: &mut PulsarGame, initial_state: &str, new_state: &str) {
        let id = game.create_entity(initial_state).unwrap();
        
        #[allow(dead_code)]
        game.set_entity_state(id, new_state, Some(initial_state));
}

fn swap_entity_state_no_state(game: &mut PulsarGame, initial_state: &str, new_state: &str) {
        let id = game.create_entity(initial_state).unwrap();
        
        #[allow(dead_code)]
        game.set_entity_state(id, new_state, None);
}

fn get_entity_states(game: &PulsarGame) {
        #[allow(dead_code)]
        game.get_entity_states();
}

fn get_entities_in_state(game: &PulsarGame, state: &str) {
        #[allow(dead_code)]
        game.get_entities_in_state(state);
}

fn criterion_benchmark(c: &mut Criterion) {
        let mut game = PulsarGame::new();
        c.bench_function("Create & Remove Entity State", |benchmark| {
                benchmark.iter(|| create_remove_entity_state(black_box(&mut game), black_box("default")))
        });

        game.clear_all_entity_states();
        game.create_entity_state("default").unwrap();
        c.bench_function("Create & Remove Entity (With entity state passed)", |benchmark| {
                benchmark.iter(|| create_remove_entity(black_box(&mut game), black_box("default")))
        });

        game.clear_all_entity_states();
        game.create_entity_state("default").unwrap();
        c.bench_function("Create & Remove Entity (Without entity state passed)", |benchmark| {
                benchmark.iter(|| create_remove_entity_no_state(black_box(&mut game), black_box("default")))
        });

        game.clear_all_entity_states();
        game.create_entity_state("default").unwrap();
        game.create_entity_state("dead").unwrap();
        c.bench_function("Create & Swap Entity State (With entity state passed)", |benchmark| {
                benchmark.iter(|| swap_entity_state(black_box(&mut game), black_box("default"), black_box("dead")))
        });

        game.clear_all_entity_states();
        game.create_entity_state("default").unwrap();
        game.create_entity_state("dead").unwrap();
        c.bench_function("Create & Swap Entity State (Without entity state passed)", |benchmark| {
                benchmark.iter(|| swap_entity_state_no_state(black_box(&mut game), black_box("default"), black_box("dead")))
        });

        game.clear_all_entity_states();
        game.create_entity_state("default").unwrap();
        c.bench_function("Get Entity States", |benchmark| {
                benchmark.iter(|| get_entity_states(black_box(&game)))
        });

        game.clear_all_entity_states();
        game.create_entity_state("default").unwrap();
        game.create_entity("default").unwrap();
        c.bench_function("Get Entities in State", |benchmark| {
                benchmark.iter(|| get_entities_in_state(black_box(&game), black_box("default")))
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);