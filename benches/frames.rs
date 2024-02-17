use nablo_shape::prelude::Vec2;
use shapoist_client::Shapoist;
use nablo::Manager;

fn frames(frames: usize) {
	let _ = Manager::new_with_settings(Shapoist::init(), nablo::Settings {
		title: "Shapoist".into(),
		size: Some(Vec2::new(1920.0, 1080.0)),
		..Default::default()
	}).run_limited_frames(frames);
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("frames", |b| b.iter(|| frames(black_box(20))));
}

criterion_group!(name = benches; config = Criterion::default().measurement_time(std::time::Duration::from_secs(1000));targets = criterion_benchmark);
criterion_main!(benches);