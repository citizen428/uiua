use std::cmp::Ordering;
type CmpFn<T> = fn(&T, &T) -> Ordering;

pub fn force_length<T: Clone>(data: &mut Vec<T>, len: usize) {
    match data.len().cmp(&len) {
        Ordering::Less => {
            let mut i = 0;
            while data.len() < len {
                data.push(data[i].clone());
                i += 1;
            }
        }
        Ordering::Greater => data.truncate(len),
        Ordering::Equal => {}
    }
}

pub fn sort_array<T: Clone>(shape: &[usize], data: &mut [T], cmp: CmpFn<T>) {
    if shape.is_empty() {
        return;
    }
    let chunk_size: usize = shape.iter().skip(1).product();
    merge_sort_chunks(chunk_size, data, cmp);
}

fn merge_sort_chunks<T: Clone>(chunk_size: usize, data: &mut [T], cmp: CmpFn<T>) {
    let cells = data.len() / chunk_size;
    assert_ne!(cells, 0);
    if cells == 1 {
        return;
    }
    let mid = cells / 2;
    let mut tmp = Vec::with_capacity(data.len());
    let (left, right) = data.split_at_mut(mid * chunk_size);
    merge_sort_chunks(chunk_size, left, cmp);
    merge_sort_chunks(chunk_size, right, cmp);
    let mut left = left.chunks_exact(chunk_size);
    let mut right = right.chunks_exact(chunk_size);
    let mut left_next = left.next();
    let mut right_next = right.next();
    loop {
        match (left_next, right_next) {
            (Some(l), Some(r)) => {
                let mut ordering = Ordering::Equal;
                for (l, r) in l.iter().zip(r) {
                    ordering = cmp(l, r);
                    if ordering != Ordering::Equal {
                        break;
                    }
                }
                if ordering == Ordering::Less {
                    tmp.extend_from_slice(l);
                    left_next = left.next();
                } else {
                    tmp.extend_from_slice(r);
                    right_next = right.next();
                }
            }
            (Some(l), None) => {
                tmp.extend_from_slice(l);
                left_next = left.next();
            }
            (None, Some(r)) => {
                tmp.extend_from_slice(r);
                right_next = right.next();
            }
            (None, None) => {
                break;
            }
        }
    }
    data.clone_from_slice(&tmp);
}

pub fn pervade_operator<A, B, C: Default>(
    a_shape: &[usize],
    a: &[A],
    b_shape: &[usize],
    b: &[B],
    f: fn(&A, &B) -> C,
) -> (Vec<usize>, Vec<C>) {
    let c_shape = a_shape.max(b_shape);
    let c_len: usize = c_shape.iter().product();
    let mut c: Vec<C> = Vec::with_capacity(c_len);
    for _ in 0..c_len {
        c.push(C::default());
    }
    pervade_operator_recursive(a_shape, a, b_shape, b, &mut c, f);
    (c_shape.to_vec(), c)
}

fn pervade_operator_recursive<A, B, C>(
    a_shape: &[usize],
    a: &[A],
    b_shape: &[usize],
    b: &[B],
    c: &mut [C],
    f: fn(&A, &B) -> C,
) {
    match (a_shape.is_empty(), b_shape.is_empty()) {
        (true, true) => c[0] = f(&a[0], &b[0]),
        (false, true) => {
            for (a, c) in a.iter().zip(c) {
                *c = f(a, &b[0]);
            }
        }
        (true, false) => {
            for (b, c) in b.iter().zip(c) {
                *c = f(&a[0], b);
            }
        }
        (false, false) => {
            let a_cells = a_shape[0];
            let b_cells = b_shape[0];
            if a_cells != b_cells {
                panic!("Shapes do not match");
            }
            let chunk_size = a.len() / a_cells;
            if b.len() / b_cells != chunk_size {
                panic!("Shapes do not match");
            }
            for ((a, b), c) in a
                .chunks_exact(chunk_size)
                .zip(b.chunks_exact(chunk_size))
                .zip(c.chunks_exact_mut(chunk_size))
            {
                pervade_operator_recursive(&a_shape[1..], a, &b_shape[1..], b, c, f);
            }
        }
    }
}