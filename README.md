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