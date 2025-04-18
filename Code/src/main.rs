struct Time {
    hour: u8, //24 hour
    minute: u8,
}

enum Alarm{
    SunRise {light_start: Time, light_full: Time, light_off: Time, noise: AlarmNoise},
    SunSet {light_on: Time, begin_fade: Time, light_off: Time},
    AlarmOnly {noise: AlarmNoise, alarm_time: Time},
}

struct AlarmNoise{
    snoozable: u8, //number of times alarm is snoozable
}

impl Alarm{
    fn get_start_time(&self) -> &Time{ //to discuss the consequence of returning a borrowed
        match self {
            Alarm::SunRise{light_start, ..} => light_start,
            Alarm::SunSet{light_on, ..} => light_on,
            Alarm::AlarmOnly{alarm_time, ..} => alarm_time,
        }
    }

    fn new_alarm(&self) -> Alarm{
        todo!("use the some/none functionality to create new alarms with defaults");
    }
}

fn get_start_time(alarm: Alarm) -> Time{ //need to think through the ownership of this a bit more
    match alarm {
        Alarm::SunRise{light_start, ..} => light_start,
        Alarm::SunSet{light_on, ..} => light_on,
        Alarm::AlarmOnly{alarm_time, ..} => alarm_time,
    }
}

fn main() {
    let sun_rise = Alarm::SunRise{
        light_start: Time{hour: 1, minute: 1}, 
        light_full: Time{hour: 1, minute: 1}, 
        light_off: Time{hour: 1, minute: 1}, 
        noise: AlarmNoise{snoozable: 3}
    };
}
