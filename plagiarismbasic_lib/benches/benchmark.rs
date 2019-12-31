use criterion::{black_box, criterion_group, criterion_main, Criterion};
use plagiarismbasic_lib::{run_plagiarism_checks, AppSettings, Metric};
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("app-group");
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(120));
    group.sample_size(20);

    let settings = AppSettings {
        n: 10,
        s: 0,
        metric: Metric::Equal,
        tdir: "testfiles/cs-corpus/t".to_string(),
        udir: "testfiles/cs-corpus/ut".to_string(),
        output_cli: false,
        output_html: true,
        open_html_after: false,
    };
    // Long running function
    group.bench_function("run_plag nocli + html + no-open-html", |b| {
        b.iter(|| run_plagiarism_checks(black_box(&settings)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
