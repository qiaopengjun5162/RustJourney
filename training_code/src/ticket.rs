// 博物馆门票
// 疫情期间，博物馆内限流最大容量50人，满了以后，出来一个才能进一个，怎么设计？
// Mutex -> Semaphore
// https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/nightly/std/sync/struct.Semaphore.html
// https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html
use tokio::sync::{Semaphore, SemaphorePermit};

pub struct Museum {
    remaining_tickets: Semaphore,
}

#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("ticket freed")
    }
}

impl Museum {
    pub fn new(total: usize) -> Self {
        Self {
            remaining_tickets: Semaphore::new(total),
        }
    }

    pub fn get_ticket(&self) -> Option<Ticket<'_>> {
        // 从信号量中获取许可
        match self.remaining_tickets.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn tickets(&self) -> usize {
        self.remaining_tickets.available_permits() // 返回当前可用许可的数量
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let museum = Museum::new(50);
        let ticket = museum.get_ticket().unwrap();
        assert_eq!(museum.tickets(), 49);
        let _tickets: Vec<Ticket> = (0..49).map(|_| museum.get_ticket().unwrap()).collect();
        assert_eq!(museum.tickets(), 0);

        assert!(museum.get_ticket().is_none());

        drop(ticket);
        {
            let ticket = museum.get_ticket().unwrap();
            println!("got ticket: {:?}", ticket);
        }
        println!("!!!!!");
        assert!(museum.get_ticket().is_some());
    }
}
