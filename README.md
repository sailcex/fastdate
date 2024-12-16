[![Rust](https://github.com/helicex-rs/fastdate/actions/workflows/rust.yml/badge.svg)](https://github.com/helicex-rs/fastdate/actions/workflows/rust.yml)

# fastdate

fastdate of Any RFC3339Micro

## why fastdate?
* full test, Code testing coverage >= 99%
* Powerful, easy to use
* based on crate `time`

this date cartes is very fast(<= 50ns) including 
* offset_sec()
* from_str("2022-12-13 11:12:14.123456")
* now()
* utc()
* week_day()
* to_string()
* eq(),==
* add(),sub()
* format("YYYY-MM-DD hh:mm:ss.000000")
* parse("YYYY-MM-DD,hh:mm:ss.000000","2022-12-13,11:12:14.123456")
* set_offset()
* unix_timestamp()
* unix_timestamp_millis()
* unix_timestamp_nano()
* from_timestamp()
* from_timestamp_millis()
* from_timestamp_nano()
* before(&date) -> bool
* after(&date1) -> bool
* from(v: SystemTime)
* from(v: DateTime)
* from(arg: Date)
* from(arg: Time)
* cmp(&self, other: &DateTime)/>/</>=/<= and more....

```bash
# aws c7g.large (2C4G)
$ cargo bench
bench_datetime_from_str time:   [103.66 ns 103.69 ns 103.71 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild

bench_date_from_str_iso_8601
                        time:   [103.66 ns 103.68 ns 103.70 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild

bench_date_from_str_iso_8601_time
                        time:   [110.07 ns 110.09 ns 110.11 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  5 (5.00%) low mild
  1 (1.00%) high mild

bench_date_parse_format time:   [320.43 ns 320.52 ns 320.61 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

bench_date_utc          time:   [75.435 ns 75.479 ns 75.523 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild

bench_date_now_local    time:   [73.585 ns 73.648 ns 73.710 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) low mild

bench_date_display      time:   [38.554 ns 38.566 ns 38.577 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild

bench_add               time:   [25.101 ns 25.102 ns 25.104 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

bench_eq                time:   [3.6993 ns 3.7029 ns 3.7063 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

bench_set_offset        time:   [15.531 ns 15.533 ns 15.535 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe

bench_timestamp         time:   [386.08 ps 386.14 ps 386.21 ps]
Found 17 outliers among 100 measurements (17.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  11 (11.00%) high severe

bench_get_micro         time:   [385.24 ps 385.29 ps 385.35 ps]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

bench_from_timestamp_millis
                        time:   [37.906 ns 37.922 ns 37.940 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  5 (5.00%) high severe

bench_format            time:   [307.83 ns 308.06 ns 308.30 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

* how use?

#### add to Cargo.tom dep
```toml
fastdate = "~0.3"
```

#### use code
```rust
use fastdate::DateTime;
fn main(){
    //now with local time zone
    DateTime::now();
    //utc time now
    DateTime::utc();
    // add
    DateTime::now() + Duration::from_secs(1);
    // sub
    DateTime::now() - Duration::from_secs(1);
    //parse allow token = ["YYYY","MM","DD","hh","mm","ss",".000000","+00:00","Z"]
    fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000Z", "2022-12-13 11:12:14.123456789Z").unwrap();
    fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "2022-12-13 11:12:14.123456789+06:00").unwrap();
    DateTime::parse("hh:mm:ss.000000,YYYY-MM-DD","11:12:14.123456,2022-12-13").unwrap();
    //format allow token = ["YYYY","MM","DD","hh","mm","ss",".000000","+00:00","Z"]
     let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 1233,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    let str:String = dt.format("YYYY-MM-DD/hh/mm/ss");
    //befor,after
    let date1 = DateTime::from_str("2022-12-12 00:00:00").unwrap();
    let date2 = DateTime::from_str("2022-12-12 01:00:00").unwrap();
    assert_eq!(date2.after(&date1), true);
    assert_eq!(date1.before(&date2), true);
    //from str
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456");
    //from str time zone
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456+08:00");
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456Z");
    //to_string()
    let s = datetime.to_string();//1234-12-13 11:12:13.123456
    //unix_timestamp
    let timestamp = DateTime::now().unix_timestamp();
    //from unix_timestamp
    let datetime = DateTime::from_timestamp(timestamp);
    //unix_timestamp_millis
    let timestamp = DateTime::now().unix_timestamp_millis();
    //from unix millis
    let datetime = DateTime::from_timestamp_millis(timestamp);
    //unix_timestamp_nano
    let timestamp = DateTime::now().unix_timestamp_nano();
    //from unix_timestamp_nano
    let datetime = DateTime::from_timestamp_nano(timestamp);
    //sum Greenwich Mean Time (GMT) from datetime
    let time_gmt = DateTime::now().sub(Duration::from_secs(offset_sec() as u64));
}
```
