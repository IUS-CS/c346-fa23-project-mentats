use chrono::{prelude::*, NaiveDateTime};

pub fn naive_date_time_to_string(
    date: NaiveDateTime,
) -> String {
    let year_str = date.date().year().to_string();
    let month_str = date.date().month().to_string();
    let day_str = date.date().day().to_string();
    let hour_str = date.time().hour().to_string();
    let minute_str = date.time().minute().to_string();
    let second_str = date.time().second().to_string();
    let mut date_vec: Vec<String> = Vec::new();
    date_vec.push(year_str);
    date_vec.push(month_str);
    date_vec.push(day_str);
    date_vec.push(hour_str);
    date_vec.push(minute_str);
    date_vec.push(second_str);
    let date_string = date_vec.join(",");
    return date_string;
}

pub fn string_to_NaiveDateTime(
     date: String
) -> NaiveDateTime {
    let string_vec: Vec<&str> = date.split(',').collect();
        //create NaiveDateTime object out of string
        let year = string_vec[0].parse::<i32>();
        let month = string_vec[1].parse::<u32>();
        let day = string_vec[2].parse::<u32>();
        let hour = string_vec[3].parse::<u32>();
        let minute = string_vec[4].parse::<u32>();
        let second = string_vec[5].parse::<u32>();
        let date = NaiveDate::from_ymd_opt(year.unwrap(), month.unwrap(), day.unwrap()).unwrap();
        let time =
            NaiveTime::from_hms_opt(hour.unwrap(), minute.unwrap(), second.unwrap()).unwrap();
        let session_start_datetime = NaiveDateTime::new(date, time);
        return session_start_datetime;
}