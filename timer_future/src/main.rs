use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    thread,
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};
// The timer we wrote in the previous section: 引入之前实现的定时器模块
use timer_future::TimerFuture;

/// Task executor that receives tasks off of a channel and runs them.
/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// A future that can reschedule itself to be polled by an `Executor`.
/// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
    /// In-progress future that should be pushed to completion. 进行中的Future，在未来的某个时间点会被完成
    ///
    /// The `Mutex` is not necessary for correctness, since we only have 按理来说`Mutex`在这里是多余的，因为我们只有一个线程来执行任务。但是由于
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread, Rust并不聪明，它无法知道`Future`只会在一个线程内被修改，并不会被跨线程修改。因此
    /// so we need to use the `Mutex` to prove thread-safety. A production 我们需要使用`Mutex`来满足这个笨笨的编译器对线程安全的执着。
    /// executor would not need this, and could use `UnsafeCell` instead.
    ///  如果是生产级的执行器实现，不会使用`Mutex`，因为会带来性能上的开销，取而代之的是使用`UnsafeCell`
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    /// 可以将该任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once. 任务通道允许的最大缓冲数(任务队列的最大长度)
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor. 当前的实现仅仅是为了简单，在实际的执行中，并不会这么使用
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    println!("[{:?}] 生成 Executor 和 Spawner （含发送端、接收端）...", thread::current().id());
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    // 把 Future 包装成任务发送到通道
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        println!("[{:?}] 将 Future 组成 Task，放入 Channel...", thread::current().id());
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
        println!("[{:?}] wake_by_ref...", thread::current().id());
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {
    fn run(&self) {
        println!("[{:?}] Executor running...", thread::current().id());
        while let Ok(task) = self.ready_queue.recv() { // 从通道不断接收任务
            println!("[{:?}] 接收到任务...", thread::current().id());
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it. 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                println!("[{:?}] 从任务中取得 Future...", thread::current().id());
                // Create a `LocalWaker` from the task itself  基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                println!("[{:?}] 获得 waker by ref...", thread::current().id());
                let context = &mut Context::from_waker(&waker);
                // `BoxFuture<T>` is a type alias for  #`BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                println!("[{:?}] 获得 context，准备进行 poll()...", thread::current().id());
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future. Future还没执行完，因此将它放回任务中，等待下次被poll
                    *future_slot = Some(future);
                    println!("[{:?}] Poll::Pending ====", thread::current().id());
                } else {
                    println!("[{:?}] Poll::Ready....", thread::current().id());
                }
            }
        }
        println!("[{:?}] Excutor run 结束", thread::current().id());
    }
}

fn main() {
    // 返回一个执行者和一个任务的生成器
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer. 生成一个任务 async块是一个Future
    spawner.spawn(async {
        println!("[{:?}] howdy!", thread::current().id());
        // Wait for our timer future to complete after two seconds. 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("[{:?}] done!", thread::current().id());
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run. drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    println!("[{:?}] drop Spawner!", thread::current().id());
    drop(spawner);

    // Run the executor until the task queue is empty. 运行执行器直到任务队列为空
    // This will print "howdy!", pause, and then print "done!". 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}
