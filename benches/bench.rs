#![feature(test)]

extern crate test;

use test::Bencher;

use std::thread;
use std::sync::Arc;
use std::task::Waker;
use std::cell::UnsafeCell;
use std::collections::vec_deque::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime};

use rand::{Rng, thread_rng};
use fastrand;
use quanta::{Clock, Upkeep};
use st3::{fifo, lifo};
use crossbeam_queue::{ArrayQueue, SegQueue};
use crossbeam_channel::{bounded, unbounded};
use crossbeam_utils::atomic::AtomicCell;

use wrr::IWRRSelector;

#[bench]
fn bench_1_0(b: &mut Bencher) {
    const COUNT: usize = 10000000;
    let mut selector = IWRRSelector::new([1, 0]);

    thread::sleep(Duration::from_secs(1));

    b.iter(|| {
        let mut x = 0;
        let mut y = 0;

        for _ in 0..COUNT {
            match selector.select() {
                0 => x += 1,
                1 => y += 1,
                _ => (),
            }
        }
        assert_eq!(COUNT, x + y);
    });
}

#[bench]
fn bench_2_1(b: &mut Bencher) {
    const COUNT: usize = 10000000;
    let mut selector = IWRRSelector::new([2, 1]);

    thread::sleep(Duration::from_secs(1));

    b.iter(|| {
        let mut x = 0;
        let mut y = 0;

        for _ in 0..COUNT {
            match selector.select() {
                0 => x += 1,
                1 => y += 1,
                _ => (),
            }
        }
        assert_eq!(COUNT, x + y);
    });
}

#[bench]
fn bench_30_1(b: &mut Bencher) {
    const COUNT: usize = 10000000;
    let mut selector = IWRRSelector::new([30, 1]);

    thread::sleep(Duration::from_secs(1));

    b.iter(|| {
        let mut x = 0;
        let mut y = 0;

        for _ in 0..COUNT {
            match selector.select() {
                0 => x += 1,
                1 => y += 1,
                _ => (),
            }
        }
        assert_eq!(COUNT, x + y);
    });
}

#[bench]
fn bench_rand(b: &mut Bencher) {
    const COUNT: usize = 10000000;
    let mut rand = thread_rng();

    thread::sleep(Duration::from_secs(1));

    b.iter(|| {
        let mut x = 0;
        let mut y = 0;

        for _ in 0..COUNT {
            match rand.gen_range(0..30) {
                0 => y += 1,
                _ => x += 1,
            }
        }
    });
}

#[bench]
fn bench_fastrand(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    b.iter(|| {
        let mut x = 0;
        let mut y = 0;

        for _ in 0..COUNT {
            match fastrand::u8(0..30) {
                0 => y += 1,
                _ => x += 1,
            }
        }
    });
}

#[bench]
fn bench_2_1_by_unsafecell(b: &mut Bencher) {
    const COUNT: usize = 10000000;
    let mut selector = UnsafeCell::new(IWRRSelector::new([2, 1]));

    thread::sleep(Duration::from_secs(1));

    b.iter(|| {
        let mut x = 0;
        let mut y = 0;

        for _ in 0..COUNT {
            match unsafe { (&mut *selector.get()).select() } {
                0 => x += 1,
                1 => y += 1,
                _ => (),
            }
        }
        assert_eq!(COUNT, x + y);
    });
}

#[bench]
fn bench_instant(b: &mut Bencher) {
    const COUNT: usize = 1000000;
    let now = Instant::now();

    thread::sleep(Duration::from_secs(1));

    let mut time = now.elapsed();
    b.iter(|| {
        for _ in 0..COUNT {
            time = now.elapsed();
        }
    });
    println!("time: {:?}", time);
}

#[bench]
fn bench_system_time(b: &mut Bencher) {
    const COUNT: usize = 1000000;
    let now = SystemTime::now();

    thread::sleep(Duration::from_secs(1));

    let mut time = now.elapsed();
    b.iter(|| {
        for _ in 0..COUNT {
            time = now.elapsed();
        }
    });
    println!("time: {:?}", time);
}

#[bench]
fn bench_quanta_instant(b: &mut Bencher) {
    const COUNT: usize = 10000000;
    let clock = Clock::new();
    let timer = Upkeep::new_with_clock(Duration::from_millis(1), clock);
    let _handle = timer.start().unwrap();
    let clock = Clock::new();
    let _now = clock.recent();

    thread::sleep(Duration::from_secs(1));

    let n = Instant::now();
    let now = clock.recent();
    b.iter(|| {
        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join0 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join1 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join2 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join3 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join4 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join5 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join6 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        let clock_copy = clock.clone();
        let now_copy = clock.recent();
        let join7 = thread::spawn(move || {
            let mut time = clock_copy.recent().duration_since(now);
            for _ in 0..COUNT / 8 {
                time = clock_copy
                    .recent()
                    .duration_since(now_copy);
            }
        });

        join0.join();
        join1.join();
        join2.join();
        join3.join();
        join4.join();
        join5.join();
        join6.join();
        join7.join();
    });
    println!("finish time: {:?}, {:?}", n.elapsed(), clock.recent().duration_since(now));
}

#[bench]
fn bench_msb(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut count = 0;
    b.iter(|| {
        for n in 0..COUNT  {
            count += get_msb(n);
        }
    });
    println!("count: {}", count);
}

const fn get_msb(n: usize) -> usize {
    usize::BITS as usize - n.leading_zeros() as usize
}

#[bench]
fn bench_st3_fifo(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = None;
    let worker = fifo::Worker::new(10000000);
    b.iter(|| {
        for n in 0..COUNT {
            worker.push(n);
        }
        for _ in 0..COUNT {
            r = worker.pop();
        }
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn bench_st3_lifo(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = None;
    let worker = fifo::Worker::new(10000000);
    b.iter(|| {
        for n in 0..COUNT {
            worker.push(n);
        }
        for _ in 0..COUNT {
            r = worker.pop();
        }
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn bench_deque(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = None;
    let mut worker = UnsafeCell::new(VecDeque::new());
    b.iter(|| {
        unsafe {
            for n in 0..COUNT {
                (&mut *worker.get()).push_back(n);
                r = (&mut *worker.get()).pop_front();
            }
        }
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn bench_deque_by_front(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = None;
    let mut worker = UnsafeCell::new(VecDeque::new());
    b.iter(|| {
        unsafe {
            for n in 0..COUNT {
                (&mut *worker.get()).push_front(n);
                r = (&mut *worker.get()).pop_front();
            }
        }
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn bench_vec(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = None;
    let mut worker = UnsafeCell::new(Vec::new());
    b.iter(|| {
        unsafe {
            for n in 0..COUNT {
                (&mut *worker.get()).push(n);
                r = (&mut *worker.get()).pop();
            }
        }
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn bench_vec_drain(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = 0;
    let mut worker = UnsafeCell::new(Vec::new());
    b.iter(|| {
        unsafe {
            for n in 0..COUNT {
                (&mut *worker.get()).push(n);
            }
            for n in (&mut *worker.get()).drain(..) {
                r = n;
            }
        }
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn bench_array_queue(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = None;
    let mut worker = Arc::new(ArrayQueue::new(10000000));
    b.iter(|| {
        let worker_copy = worker.clone();
        let join0 = thread::spawn(move || {
            let mut count = 0;
            loop {
                let n = worker_copy.pop();
                if n.is_some() {
                    r = n;
                    count += 1;
                }

                if count >= COUNT {
                    break;
                }
            }
        });

        let worker_copy = worker.clone();
        let join1 = thread::spawn(move || {
            for n in 0..2500000 {
                worker_copy.push(n);
            }
        });

        let worker_copy = worker.clone();
        let join2 = thread::spawn(move || {
            for n in 2500000..5000000 {
                worker_copy.push(n);
            }
        });

        let worker_copy = worker.clone();
        let join3 = thread::spawn(move || {
            for n in 5000000..7500000 {
                worker_copy.push(n);
            }
        });

        let worker_copy = worker.clone();
        let join4 = thread::spawn(move || {
            for n in 7500000..10000000 {
                worker_copy.push(n);
            }
        });

        join0.join();
        join1.join();
        join2.join();
        join3.join();
        join4.join();
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn bench_seq_queue(b: &mut Bencher) {
    const COUNT: usize = 10000000;

    thread::sleep(Duration::from_secs(1));

    let mut r = None;
    let mut worker = Arc::new(SegQueue::new());
    b.iter(|| {
        let worker_copy = worker.clone();
        let join0 = thread::spawn(move || {
            let mut count = 0;
            loop {
                let n = worker_copy.pop();
                if n.is_some() {
                    r = n;
                    count += 1;
                }

                if count >= COUNT {
                    break;
                }
            }
        });

        let worker_copy = worker.clone();
        let join1 = thread::spawn(move || {
            for n in 0..2500000 {
                worker_copy.push(n);
            }
        });

        let worker_copy = worker.clone();
        let join2 = thread::spawn(move || {
            for n in 2500000..5000000 {
                worker_copy.push(n);
            }
        });

        let worker_copy = worker.clone();
        let join3 = thread::spawn(move || {
            for n in 5000000..7500000 {
                worker_copy.push(n);
            }
        });

        let worker_copy = worker.clone();
        let join4 = thread::spawn(move || {
            for n in 7500000..10000000 {
                worker_copy.push(n);
            }
        });

        join0.join();
        join1.join();
        join2.join();
        join3.join();
        join4.join();
    });
    println!("!!!!!!r: {:?}", r);
}

#[bench]
fn test_new_atomic_usize(b: &mut Bencher) {
    let mut atomic = Arc::new(AtomicUsize::new(0));
    b.iter(|| {
        for index in 0..1000000 {
            atomic = Arc::new(AtomicUsize::new(index));
        }
    });
    println!("{}", atomic.load(Ordering::Relaxed));
}

#[bench]
fn test_load_atomic_usize(b: &mut Bencher) {
    let atomic = Arc::new(AtomicUsize::new(6679138));

    b.iter(|| {
        for _ in 0..100000000 {
            let n = atomic.load(Ordering::Acquire);
            if (n & 0xffff) != 60002 || (n >> 16) & 0xffff != 101 {
                panic!("invalid number {}", n);
            }
        }
    });
}

#[bench]
fn test_fetch_add_atomic_usize(b: &mut Bencher) {
    let atomic = Arc::new(AtomicUsize::new(6679138));

    let mut n = 0;
    b.iter(|| {
        for _ in 0..10000000 {
            n = atomic.fetch_add(1, Ordering::Relaxed);
        }
    });
    println!("{}", n);
}

#[bench]
fn test_new_atomic_u128(b: &mut Bencher) {
    let mut atomic = AtomicCell::new(0);
    b.iter(|| {
        for index in 0..1000000u128 {
            let ptr = Box::into_raw(Box::new(Some(index))) as u128;
            atomic = AtomicCell::new(ptr << 64 | (6679138 & 0xffffffffffffffff));
        }
    });
    println!("{}, {}", atomic.load(), atomic.load() as u64);
}

#[bench]
fn test_load_atomic_u128(b: &mut Bencher) {
    let atomic = AtomicCell::new(32516125253190580227719993616994u128);

    b.iter(|| {
        for _ in 0..100000000 {
            let n = atomic.load();
            if (n & 0xffff) != 60002 || (n >> 16) & 0xffff != 101 {
                panic!("invalid number {}", n);
            }
        }
    });
}

pub struct TaskId(UnsafeCell<u128>);

pub struct TaskHandle<R: 'static>(Box<(
    AtomicCell<Option<Waker>>,
    AtomicCell<Option<R>>,
)>);

impl<R: 'static> Default for TaskHandle<R> {
    fn default() -> Self {
        TaskHandle(Box::new((AtomicCell::new(None), AtomicCell::new(None))))
    }
}

impl<R: 'static> TaskHandle<R> {
    pub unsafe fn from_raw(raw: *const ()) -> TaskHandle<R> {
        let inner
            = Box::from_raw(raw as *const (AtomicCell<Option<Waker>>, AtomicCell<Option<R>>) as *mut (AtomicCell<Option<Waker>>, AtomicCell<Option<R>>));
        TaskHandle(inner)
    }

    pub fn into_raw(self) -> *const () {
        Box::into_raw(self.0)
            as *mut (AtomicCell<Option<Waker>>, AtomicCell<Option<R>>)
            as *const (AtomicCell<Option<Waker>>, AtomicCell<Option<R>>)
            as *const ()
    }
}

#[bench]
fn test_new_task_id(b: &mut Bencher) {
    let mut task_id = TaskId(UnsafeCell::new(0));
    b.iter(|| {
        for index in 0..1000000u128 {
            task_id = TaskId(UnsafeCell::new((TaskHandle::<String>::default().into_raw() as u128) << 64 | (index as u128) << 32 | u32::MAX as u128 & 0xffffffff));
        }
    });
}






