
use std::io;
use num::integer::gcd;
use num::bigint::BigInt;
use num::bigint::BigUint;
use std::ops::{Div, Mul, AddAssign, DivAssign, Sub, MulAssign};

use bigdecimal::BigDecimal;
use std::str::FromStr;

pub fn matrix_init(n: usize, mut k: usize, r: usize) -> Vec<Vec<BigUint>> {
    assert!(n > 0 && k > 0 && r > 0);
    if k > n {
        k = n;
    }
    let mut matrix = vec![vec![BigUint::from(0u32); k+1]; n+1];
    for i_n in 1..=n {
        for i_k in 1..=k {
            if i_k > 1 && i_n == r*i_k {
                matrix[i_n][i_k] = BigUint::from(1u32);
                for j in 2..=i_k {
                    matrix[i_n][i_k].mul_assign(combination(j*r, r));
                }
            } else if i_k == 1 && i_n <= r {
                matrix[i_n][i_k] = BigUint::from(1u32);
            } else if i_n == 1 {
                matrix[i_n][i_k] = BigUint::from(i_k);
            }
        }
    }
    matrix
}

pub fn partition_matrix(n: usize, k: usize, r: usize, matrix: &mut Vec<Vec<BigUint>>) -> BigUint {
    assert!(n > 0 && k > 0 && r > 0, "n = {}, k = {}, r = {}", n, k, r);
    let mut ret = BigUint::from(0u32);
    if k > n {
        ret = combination(k, n).mul(partition_matrix(n, n, r, matrix));
    } else if matrix[n][k].ne(&ret) {  // assert!(ret.eq(BigUint::from(0u32)));
        ret = matrix[n][k].clone();
    } else if n < k*r {
        let volume_next = (k-1)*r;
        for i in 0..=r {
            if n > i && (n-i) <= volume_next {
                ret.add_assign(combination(n, i).mul(partition_matrix(n-i, k-1, r, matrix)));
            }
        }
        matrix[n][k] = ret.clone();
        println!("matrix[{}][{}] == {}", n, k, matrix[n][k]);
    }
    ret
}

// C(k, n) = k! / n!(k-n)!
// C(100, 20) = (100!) / ((20!) * (80!)) = 100*99*...81 / 20*19*...*1
fn combination(k: usize, mut n: usize) -> BigUint {
    assert!(k >= n, "k={}, n={}", k, n);
    if n > k-n {
        n = k-n; // e.g. C(100, 80) = C(100, 20)
    }
    let mut vec_up = Vec::new();
    let mut vec_down = Vec::new();
    let mut i = k;
    let mut j = n;
    while i > k-n {
        vec_up.push(i);
        vec_down.push(j);
        i -= 1;
        j -= 1;
    }
    for i in 0..n {
        for j in 0..n {
            if vec_down[j] > 1 {
                let gcd_ret = gcd(vec_up[i], vec_down[j]);
                if gcd_ret > 1 {
                    vec_up[i] /= gcd_ret;
                    vec_down[j] /= gcd_ret;
                }
            }
            if vec_up[i] == 1 {
                break;
            }
        }
    }
    let mut ret = BigUint::from(1u32);
    for i in 0..n {
        assert!(vec_down[i] == 1);
        if vec_up[i] > 1 {
            ret = ret.mul(vec_up[i]);
        }
    }
    ret
}

#[test]
fn test_gcd() {
    use num::integer::gcd;
    let x = gcd(10, 5);
    println!("{}", x);
}

#[test]
fn test_combination() {
    for i in 0..=50 {
        let ret = combination(100, i);
        println!("{:?}, {}", ret, i);
    }

    for i in 51..=100 {
        let ret = combination(100, i);
        println!("{:?}, {}", ret, i);
    }
}

#[test]
fn test_partition() {
    let n = 50;
    let k = 100;
    let r = 5;
    //let g = 10;
    let mut matrix = matrix_init(n, k, r);
    let ret0 = partition_matrix(n, k, r, &mut matrix);

    let r = 10;
    let mut matrix = matrix_init(n, k, r);
    let ret1 = partition_matrix(n, k, r, &mut matrix);

    println!("ret0 == {}", ret0);
    println!("ret1 == {}", ret1);

    let input0 = format!("{}.0", ret0);
    let dec0 = BigDecimal::from_str(&input0).unwrap();
    println!("dec0 == {}", dec0);
    let float0 = f64::from_str(&input0).unwrap();
    println!("float0 == {}", float0);

    let input1 = format!("{}.0", ret1);
    let dec1 = BigDecimal::from_str(&input1).unwrap();
    println!("dec0 == {}", dec1);
    let float1 = f64::from_str(&input1).unwrap();
    println!("float1 == {}", float1);

    let p = dec0.div(&dec1);
    println!("float0 / float1 == {}", float0.div(&float1));
    println!("dec0 / dec1 == {}", p);

    let one = BigDecimal::from_str("1.0").unwrap();
    println!("dec0 / dec1 == {}", one.sub(&p));
}

fn main() {
    let n = 100;
    let k = 100;
    let r = 5;
    let mut matrix = matrix_init(n, k, r);
    println!("Count is  {:?}", matrix);
    //let ret = partition_matrix(n, k, r, &mut matrix);
    //println!("Count is  {}", ret);
}


