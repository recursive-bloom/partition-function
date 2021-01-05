
use std::io;
use num::integer::gcd;
use num::bigint::BigInt;
use num::bigint::BigUint;
use std::ops::{Div, Mul, AddAssign, DivAssign};

use bigdecimal::BigDecimal;
use std::str::FromStr;

pub fn pause(n: usize, k: usize, r: usize) {
    println!("Please Input Something: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("n = {}, k = {}, r = {}", n, k, r);
}

pub fn partion(n: usize, k: usize, mut r: usize) -> BigUint {
    let mut ret = BigUint::from(0u32);
    if n >= 1 && k >= 1 && r >= 1 {
        if n < r {
            r = n;
        }
        let tmp = k*r;
        if tmp == n {
            ret = BigUint::from(1u32);;
        } else if tmp > n {
            if n == 1 {
                ret = BigUint::from(k);
            } else {
                for i in  0..= r {
                    ret.add_assign(partion(n-i, k-1, r));
                }
            }
        }
    }
    ret
}

pub fn partion_with_matrix(n: usize, k: usize, mut r: usize, matrix: &mut Vec<Vec<Vec<BigUint>>>) -> BigUint {
    let mut ret = BigUint::from(0u32);
    if n >= 1 && k >= 1 && r >= 1 {
        if n < r {
            r = n;
        }
        // println!("{} {} {}", n, k, r);
        if matrix[n-1][k-1][r-1].ne(&BigUint::from(0u32)) {
            ret = matrix[n-1][k-1][r-1].clone();
        } else if k*r > n {
            if n == 1 {
                ret = BigUint::from(k);
            } else if k > n {
                ret = combination(k, k-n).mul(partion_with_matrix(n, n, r, matrix));
            } else {
                for i in  0..= r {
                    ret.add_assign(partion_with_matrix(n-i, k-1, r, matrix));
                }
            }
            matrix[n-1][k-1][r-1] = ret.clone();
        }
    }
    ret
}


pub fn matrix_init(n: usize, k: usize, r: usize) -> Vec<Vec<Vec<BigUint>>> {
    let mut matrix = vec![vec![vec![BigUint::from(0u32); r]; k]; n];
    for i_n in 0..n {
        for i_k in 0..k {
            for i_r in 0..r {
                if (i_k+1) * (i_r+1) == (i_n+1) {
                    matrix[i_n][i_k][i_r] = BigUint::from(1u32);
                    // println!("===={}, {}, {}, {}", i_n, i_k, i_r, mt[i_n][i_k][i_r]);
                }
            }
        }
    }
    matrix
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
    let one = BigUint::from(1u32);
    let mut ret = one.clone();
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
fn test_partion() {
    let n = 100;
    let k = 100;
    let r = 5;
    //let g = 10;
    let mut matrix = matrix_init(n, k, r);
    let ret0 = partion_with_matrix(n, k, r, &mut matrix);
    println!("r0 == {}", ret0);

    let n = 100;
    let k = 100;
    let r = 10;
    let mut matrix = matrix_init(n, k, r);
    let ret1 = partion_with_matrix(n, k, r, &mut matrix);
    println!("r1 == {}", ret1);

    //println!("div == {}", ret1.div(&ret0));

    let input0 = format!("{}", ret0);
    let dec = BigDecimal::from_str(&input0).unwrap();
    let float0 = f64::from_str(&input0).unwrap();
    println!("float == {}", float0);

    let input1 = format!("{}", ret1);
    let dec = BigDecimal::from_str(&input1).unwrap();
    let float1 = f64::from_str(&input1).unwrap();
    println!("float == {}", float1);
    println!("float0/float1 == {}", float0.div(&float1));
}


fn main() {
    let n = 500;
    let k = 100;
    let r = 10;
    let mut matrix = matrix_init(n, k, r);
    let ret = partion_with_matrix(n, k, r, &mut matrix);
    println!("Count is  {}", ret);
}


