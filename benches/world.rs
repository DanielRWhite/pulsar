use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::sync::{ Arc, Mutex };
use pulsar::{
	world::World,
	archetypes::Archetypes,
	entity::{ Entity, EntityBuilder }
};

fn criterion_benchmark(c: &mut Criterion) {
	let mut archetypes = Archetypes::new();
	let entity = Entity::new(0);
	let components = entity.get_archetype();

	let entity_arc = Arc::new(Mutex::new(Box::new(entity)));

	c.bench_function("Create Entity", |benchmark| {
		benchmark.iter(|| {
			archetypes.add_entity(entity_arc.clone(), components.clone());
		});
	});

	c.bench_function("Remove Entity", |benchmark| {
		benchmark.iter(|| {
			archetypes.remove_entity(entity_arc.clone(), components.clone());
		});
	});

	//c.bench_function("", |b| b.iter(|| fibonacci(black_box(20))));
}
    
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);