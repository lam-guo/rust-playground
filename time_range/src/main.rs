use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Weekday};

fn main() {
    let entry1 = ScheduleEntry {
        days: vec![0, 6],
        mode: 2,
        time_range: ("10:00".to_string(), "11:00".to_string()),
    };

    let entry2 = ScheduleEntry {
        days: vec![1, 3, 5],
        mode: 3,
        time_range: ("14:00".to_string(), "16:00".to_string()),
    };

    let entries = vec![entry1, entry2];

    // 测试用例 1：星期一 15:00 (在时间范围内)
    let test_date1 = Local.ymd(2022, 11, 21).and_hms(15, 0, 0);
    println!("{}", is_in_time_range(entries.clone(), test_date1));

    // 测试用例 2：星期二 17:00 (不在时间范围内)
    let test_date2 = Local.ymd(2022, 11, 22).and_hms(17, 0, 0);
    println!("{}", is_in_time_range(entries.clone(), test_date2));

    // 测试用例 3：星期六 10:30 (在时间范围内)
    let test_date3 = Local.ymd(2022, 11, 26).and_hms(10, 30, 0);
    println!("{}", is_in_time_range(entries.clone(), test_date3));

    // 测试用例 4：星期日 11:30 (不在时间范围内)
    let test_date4 = Local.ymd(2022, 11, 27).and_hms(11, 30, 0);
    println!("{}", is_in_time_range(entries.clone(), test_date4));
}

#[derive(Debug, Clone)]
struct ScheduleEntry {
    days: Vec<u8>,                // 0,1,2,3,4,5,6 代表星期日，星期一到星期六
    mode: u8,                     // mode 0每天 1工作日 2周末 3自定义
    time_range: (String, String), //时间范围，格式["10:00","11:00"]
}

fn is_in_time_range(entries: Vec<ScheduleEntry>, date: DateTime<Local>) -> bool {
    let weekday = date.weekday().num_days_from_sunday() as u8;
    println!("weekday:{:?}", weekday);
    let current_time = (date.hour(), date.minute());
    for entry in entries {
        match entry.mode {
            0 => {} // 每天
            1 => {
                // 工作日
                if !matches!(weekday, 1..=5) {
                    continue;
                }
            }
            2 => {
                // 周末
                if !matches!(weekday, 0 | 6) {
                    continue;
                }
            }
            3 => {
                // 自定义
                if !entry.days.contains(&weekday) {
                    continue;
                }
            }
            _ => continue,
        }

        let start_time = parse_time(entry.time_range.0.as_str()).unwrap();
        let end_time = parse_time(entry.time_range.1.as_str()).unwrap();

        if current_time >= start_time && current_time < end_time {
            return true;
        }
    }

    false
}

fn parse_time(time: &str) -> Option<(u32, u32)> {
    let components: Vec<&str> = time.split(':').collect();
    if components.len() == 2 {
        let hour = components[0].parse::<u32>().ok()?;
        let minute = components[1].parse::<u32>().ok()?;
        Some((hour, minute))
    } else {
        None
    }
}
