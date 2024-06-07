use std::fmt::Debug;

pub fn smooth_sort<T: Ord + Clone + Debug>(list: &mut [T]) {
    let len = list.len();
    let mut heap = 0;
    let mut r = 0;

    while r < len {
        if (heap & 7) == 3 {
            sift(list, r - 1, heap >> 3);
            heap = (heap >> 2) + 1;
        } else if (heap & 3) == 1 && r + 1 < len {
            sift(list, r - 1, heap >> 2);
            heap = (heap >> 2) + 1;
        } else {
            trinkle(list, r, heap, false);
            if heap == 3 {
                heap = 1;
            } else {
                let lp = heap.trailing_zeros() as usize;
                if lp < std::mem::size_of_val(&heap) * 8 - 1 {
                    heap = (1 << (lp + 1)) - 1;
                } else {
                    heap = std::usize::MAX;
                }
            }
        }
        r += 1;
    }

    while heap != 1 {
        let lp = heap.trailing_zeros() as usize;
        if lp < std::mem::size_of_val(&heap) * 8 - 1 {
            heap = (1 << (lp + 1)) - 1;
        } else {
            heap = std::usize::MAX;
        }
        trinkle(list, r, heap, false);
        r -= 1;
        if r > 0 {
            trinkle(list, r, heap, false);
            r -= 1;
        }
    }

    if r > 0 {
        trinkle(list, r, heap, false);
    }
}

fn sift<T: Ord + Clone + Debug>(list: &mut [T], r: usize, mut root: usize) {
    println!("Before sift: {:?}", list);
    let mut r = r;
    let mut t = list[r].clone();
    let mut r2;
    if root != std::usize::MAX {
        if root << 1 <= r {
            r2 = r - (root << 1);
        } else {
            r2 = 0;
        }
    } else {
        r2 = r;
    }
    while root > 0 {
        if list[r2] <= t {
            break;
        }
        list[r] = list[r2].clone();
        r = r2;
        root >>= 1;
        if root < std::usize::MAX / 2 && r2 > root << 1 {
            r2 -= root << 1;
        } else {
            r2 = 0;
        }
    }
    list[r] = t;
    println!("After sift: {:?}", list);
    println!("r: {}, root: {}", r, root);
}

fn trinkle<T: Ord + Clone + Debug>(
    list: &mut [T],
    mut r: usize,
    mut root: usize,
    mut trusty: bool,
) {
    println!("Before trinkle: {:?}", list);
    println!("r: {}, root: {}, trusty: {}", r, root, trusty);
    if r >= list.len() {
        return;
    }
    if root > r {
        root = r;
    }
    let t = list[r].clone();
    let mut r3;
    if root < std::usize::MAX - 2 && r > ((root + 3) >> 1) {
        r3 = (r - 1) / 2; // calculate parent index correctly
    } else {
        r3 = r;
    }
    while r3 < list.len() && list[r3] > t {
        // ensure r3 is a valid index
        if !trusty && r - r3 <= root {
            break;
        }
        list[r] = list[r3].clone();
        r = r3;
        if root == 0 {
            break;
        }
        root = (root - 1) >> 1;
        if root < std::usize::MAX - 2 && r > ((root + 3) >> 1) {
            r3 = (r - 1) / 2; // calculate parent index correctly
        } else {
            r3 = r;
        }
    }
    if r != r3 {
        list[r] = list[r3].clone();
        list[r3] = t.clone();
    }
    sift(list, r, root);
    println!("After trinkle: {:?}", list);
    println!("r: {}, root: {}, trusty: {}", r, root, trusty);
}
