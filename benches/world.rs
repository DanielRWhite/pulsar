use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pulsar::{
	world::World,
	entity::Entity
};

fn criterion_benchmark(c: &mut Criterion) {
	let mut world = World::new();

	c.bench_function("Create Entity", |benchmark| {
		benchmark.iter(|| {
                        let entity: Entity = Entity::new(&mut world).unwrap();
                        world.create_entity(entity)
                });
	});

        let mut world = World::new();
        c.bench_function("Create & Remove Entity", |benchmark| {
		benchmark.iter(|| {
                        let entity: Entity = Entity::new(&mut world).unwrap();
                        let id = entity.get_id();

                        {
                                world.create_entity(entity);
                        }

                        {
                                world.delete_entity(id, None);
                        }
                });
	});

        let mut world = World::new();
        c.bench_function("Create & Remove Entity (With archetype)", |benchmark| {
		benchmark.iter(|| {
                        let entity: Entity = Entity::new(&mut world).unwrap();
                        let id = entity.get_id();
                        let archetype = entity.get_archetype();

                        {
                                world.create_entity(entity);
                        }
                        
                        {
                                world.delete_entity(id, Some(archetype));
                        }
                });
	});
}
    
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);