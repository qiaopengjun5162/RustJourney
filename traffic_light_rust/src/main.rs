// 定义一个 trait，它有一个返回时间的方法
trait TrafficLightDuration {
    fn duration(&self) -> u32; // 假设返回的是一个无符号32位整数表示的时间（秒）
}

// 定义交通信号灯的枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 为每种交通信号灯实现 TrafficLightDuration trait
impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match *self {
            TrafficLight::Red => 60,   // 红灯持续60秒
            TrafficLight::Yellow => 5, // 黄灯持续5秒
            TrafficLight::Green => 45, // 绿灯持续45秒
        }
    }
}

fn main() {
    // 创建一个 TrafficLight 枚举的实例
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red light duration: {} seconds", red.duration());
    println!("Yellow light duration: {} seconds", yellow.duration());
    println!("Green light duration: {} seconds", green.duration());
}
