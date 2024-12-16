use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fastdate::DateTime;
use std::str::FromStr;
use std::time::Duration;

//test bench_datetime_from_str     ... bench:          11 ns/iter (+/- 0)
fn bench_datetime_from_str(c: &mut Criterion) {
    c.bench_function("bench_datetime_from_str", |b| {
        b.iter(|| {
            black_box(DateTime::from_str("1997-12-13T11:12:13.123456Z").expect("TODO: panic message"));
        });
    });
}

//test bench_date_from_str_iso_8601   ... bench:          41 ns/iter (+/- 2)
fn bench_date_from_str_iso_8601(c: &mut Criterion) {
    c.bench_function("bench_date_from_str_iso_8601", |b| {
        b.iter(|| {
            black_box(DateTime::from_str("1997-12-13T11:12:13.123456Z").expect("TODO: panic message"));
        });
    });
}

//test bench_date_from_str_iso_8601_time   ... bench:          41 ns/iter (+/- 2)
fn bench_date_from_str_iso_8601_time(c: &mut Criterion) {
    c.bench_function("bench_date_from_str_iso_8601_time", |b| {
        b.iter(|| {
            black_box(DateTime::from_str("1997-12-13T11:12:13.123456+09:00").expect("TODO: panic message"));
        });
    });
}

//test bench_date_parse_format ... bench:          58 ns/iter (+/- 1)
fn bench_date_parse_format(c: &mut Criterion) {
    c.bench_function("bench_date_parse_format", |b| {
        b.iter(|| {
            black_box(DateTime::parse("YYYY-MM-DD hh:mm:ss.000000Z", "2022-12-13 11:12:14.123456Z")
                .expect("TODO: panic message"));
        });
    });
}

//test bench_date_now   ... bench:          40 ns/iter (+/- 1)
fn bench_date_utc(c: &mut Criterion) {
    c.bench_function("bench_date_utc", |b| {
        b.iter(|| {
            black_box(DateTime::utc());
        });
    });
}

//test bench_date_now_local ... bench:          40 ns/iter (+/- 1)
fn bench_date_now_local(c: &mut Criterion) {
    c.bench_function("bench_date_now_local", |b| {
        b.iter(|| {
            black_box(DateTime::now());
        });
    });
}

//test bench_date_display    ... bench:          40 ns/iter (+/- 1)
fn bench_date_display(c: &mut Criterion) {
    let now = DateTime::now();
    c.bench_function("bench_date_display", |b| {
        b.iter(|| {
            black_box(now.to_string());
        });
    });
}

fn bench_add(c: &mut Criterion) {
    let now = DateTime::now();
    c.bench_function("bench_add", |b| {
        b.iter(|| {
            black_box(now.clone() + Duration::from_secs(24 * 3600));
        });
    });
}

fn bench_eq(c: &mut Criterion) {
    let now = DateTime::now();
    let now2 = DateTime::now();
    c.bench_function("bench_eq", |b| {
        b.iter(|| {
            black_box(now.eq(&now2));
        });
    });
}

fn bench_set_offset(c: &mut Criterion) {
    let now = DateTime::utc();
    c.bench_function("bench_set_offset", |b| {
        b.iter(|| {
            black_box( now.clone().set_offset(8 * 3600));
        });
    });
}

fn bench_timestamp(c: &mut Criterion) {
    let now = DateTime::utc();
    c.bench_function("bench_timestamp", |b| {
        b.iter(|| {
            black_box(now.unix_timestamp());
        });
    });
}

fn bench_get_micro(c: &mut Criterion) {
    let now = DateTime::utc();
    c.bench_function("bench_get_micro", |b| {
        b.iter(|| {
            black_box(now.micro());
        });
    });
}

//27 ns/iter (+/- 1)
fn bench_from_timestamp_millis(c: &mut Criterion) {
    c.bench_function("bench_from_timestamp_millis", |b| {
        b.iter(|| {
            black_box(DateTime::from_timestamp_millis(1692948547638));
        });
    });
}

//181 ns/iter (+/- 2)
fn bench_format(c: &mut Criterion) {
    let dt = DateTime::from_str("1997-12-13T11:12:13.123456+09:00").unwrap();
    c.bench_function("bench_format", |b| {
        b.iter(|| {
            black_box(dt.format("YYYY-MM-DD/hh/mm/ss.000000"));
        });
    });
}

criterion_group!(benches,
    bench_datetime_from_str,
    bench_date_from_str_iso_8601,
    bench_date_from_str_iso_8601_time,
    bench_date_parse_format,
    bench_date_utc,
    bench_date_now_local,
    bench_date_display,
    bench_add,
    bench_eq,
    bench_set_offset,
    bench_timestamp,
    bench_get_micro,
    bench_from_timestamp_millis,
    bench_format
);
criterion_main!(benches);