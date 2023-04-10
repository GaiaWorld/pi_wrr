use std::cell::{RefCell, UnsafeCell};
use std::marker::PhantomData;

use wrr::IWRRSelector;

#[test]
fn test() {
    const COUNT: usize = 100000;

    let mut selector = IWRRSelector::new([1, 0]);
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
    println!("total: {}, x: {}, y: {}", COUNT, x, y);

    let mut selector = IWRRSelector::new([1, 1]);
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
    println!("total: {}, x: {}, y: {}", COUNT, x, y);

    let mut selector = IWRRSelector::new([2, 1]);
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
    println!("total: {}, x: {}, y: {}", COUNT, x, y);

    let mut selector = IWRRSelector::new([254, 1]);
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
    println!("total: {}, x: {}, y: {}", COUNT, x, y);

    let mut selector = IWRRSelector::new([6, 3, 1]);
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for _ in 0..COUNT {
        match selector.select() {
            0 => x += 1,
            1 => y += 1,
            2 => z += 1,
            _ => (),
        }
    }
    assert_eq!(COUNT, x + y + z);
    println!("total: {}, x: {}, y: {}, z: {}", COUNT, x, y, z);

    let mut selector = IWRRSelector::new([18, 16, 12, 14, 8, 10, 4, 6, 2, 1]);
    let mut q0 = 0;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let mut q5 = 0;
    let mut q6 = 0;
    let mut q7 = 0;
    let mut q8 = 0;
    let mut q9 = 0;
    for _ in 0..COUNT {
        match selector.select() {
            0 => q0 += 1,
            1 => q1 += 1,
            2 => q2 += 1,
            3 => q3 += 1,
            4 => q4 += 1,
            5 => q5 += 1,
            6 => q6 += 1,
            7 => q7 += 1,
            8 => q8 += 1,
            9 => q9 += 1,
            _ => (),
        }
    }
    assert_eq!(COUNT, q0 + q1 + q2 + q3 + q4 + q5 + q6 + q7 + q8 + q9);
    println!("total: {}\nq0: {}\nq1: {}\nq2: {}\nq3: {}\nq4: {}\nq5: {}\nq6: {}\nq7: {}\nq8: {}\nq9: {}",
             COUNT, q0, q1, q2, q3, q4, q5, q6, q7, q8, q9);
}

#[test]
fn test_msb() {
    println!("{}, {}", get_msb(0), 0 >> get_msb(0).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(1), 1 >> get_msb(1).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(2), 2 >> get_msb(2).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(65), 65 >> get_msb(65).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(127), 127 >> get_msb(127).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(128), 128 >> get_msb(128).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(255), 255 >> get_msb(255).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(256), 256 >> get_msb(256).checked_sub(2).unwrap_or(0));
    println!("{}, {}", get_msb(usize::MAX), usize::MAX >> get_msb(usize::MAX).checked_sub(2).unwrap_or(0));
}

const fn get_msb(n: usize) -> usize {
    usize::BITS as usize - n.leading_zeros() as usize
}


