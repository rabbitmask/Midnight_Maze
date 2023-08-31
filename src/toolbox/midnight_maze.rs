use chrono::prelude::*;


fn east8() {
    let utc_offset = FixedOffset::east(8 * 3600); // 东八区偏移量为 8 小时
    let beijing_time = Utc::now().with_timezone(&utc_offset);
    println!("当前东八区时间：{}", beijing_time);
}


pub(crate) fn midnight_maze(target_hour:u32,target_minute:u32){
    let utc_offset = FixedOffset::east(8 * 3600); // 东八区偏移量为 8 小时
    let beijing_time = Utc::now().with_timezone(&utc_offset);

    // println!("当前东八区时间：{}", beijing_time);

    // 获取当前时间的小时和分钟
    let current_hour = beijing_time.hour();
    let current_minute = beijing_time.minute();

    // 定义特定时间执行程序的条件
    // let target_hour = 9;
    // let target_minute = 30;

    // 检查当前时间是否满足特定时间条件
    if current_hour != target_hour || current_minute != target_minute {
        std::process::exit(0);
    } else {
    }
}
