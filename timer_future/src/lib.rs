use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
/// 在Future和等待的线程间共享状态
struct SharedState {
    /// Whether or not the sleep time has elapsed
    ///  定时(睡眠)是否结束
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("[{:?}] Polling TimerFuture...", thread::current().id());
        // Look at the shared state to see if the timer has already completed.
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            println!("[{:?}] TimerFuture completed...", thread::current().id());
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            // 设置`waker`，这样新线程在睡眠(计时)结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作,
            //
            // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
            // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
            // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            shared_state.waker = Some(cx.waker().clone());
            println!("[{:?}] TimerFuture pending...", thread::current().id());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        println!("[{:?}] 开始创建新的 TimerFuture...", thread::current().id());
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread 创建新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!(
                "[{:?}] TimerFuture 生成新的线程并开始睡眠...",
                thread::current().id()
            );
            thread::sleep(duration); //  睡眠指定时间实现计时功能
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                println!(
                    "[{:?}] TimerFuture 新线程获得 waker，并进行 wake()...",
                    thread::current().id()
                );
                waker.wake()
            } else {
                println!(
                    "[{:?}] TimerFuture 新线程没获得 waker...",
                    thread::current().id()
                );
            }
        });

        println!("[{:?}] 返回新的 TimerFuture...", thread::current().id());
        TimerFuture { shared_state }
    }
}
