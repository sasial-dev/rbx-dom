mod util;

use crate::util::bench;
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};

pub fn folders_100(c: &mut Criterion) {
    bench(
        &mut c.benchmark_group("100 Folders"),
        include_bytes!("../files/folders-100.rbxm"),
    )
}

pub fn deep_folders_100(c: &mut Criterion) {
    bench(
        &mut c.benchmark_group("100 Deep Folders"),
        include_bytes!("../files/deep-folders-100.rbxm"),
    )
}

pub fn modulescripts_100_lines_100(c: &mut Criterion) {
    bench(
        &mut c.benchmark_group("100 100-line ModuleScripts"),
        include_bytes!("../files/modulescripts-100-lines-100.rbxm"),
    )
}

pub fn parts_1000(c: &mut Criterion) {
    bench(
        c.benchmark_group("1,000 Parts")
            .sampling_mode(SamplingMode::Flat),
        include_bytes!("../files/parts-1000.rbxm"),
    )
}

pub fn alsbury(c: &mut Criterion) {
    bench(
        c.benchmark_group("Alsbury")
            .sampling_mode(SamplingMode::Flat),
        include_bytes!("../../../../New-Division/Game2/edit.rbxl"),
    )
}

criterion_group!(
    bench_suite,
    folders_100,
    deep_folders_100,
    modulescripts_100_lines_100,
    parts_1000,
    alsbury,
);

criterion_main!(bench_suite);
