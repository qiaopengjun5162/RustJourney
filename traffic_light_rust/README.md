# traffic_light_rust

为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同

## Explain code

这段代码定义了一个 trait  `TrafficLightDuration` ，其中包含一个返回时间的方法。然后定义了一个枚举  `TrafficLight`  包括红灯、黄灯和绿灯。接着为每种交通信号灯实现了  `TrafficLightDuration`  trait，使得每种信号灯可以返回相应的持续时间。在  `main`  函数中创建了红灯、黄灯和绿灯的实例，并打印出它们各自持续的时间。

代码步骤解释：

1. 定义了一个 trait  `TrafficLightDuration` ，其中包含一个返回时间的方法  `duration` 。
2. 定义了一个枚举  `TrafficLight` ，包括了红灯、黄灯和绿灯。
3. 为  `TrafficLight`  枚举实现了  `TrafficLightDuration`  trait，根据不同的信号灯返回相应的持续时间。
4. 在  `main`  函数中创建了红灯、黄灯和绿灯的实例。
5. 分别打印出红灯、黄灯和绿灯的持续时间。
