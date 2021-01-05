
use std::io;
use num::Integer;
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


fn combination(k: usize, n: usize) -> BigUint {
    let one = BigUint::from(1u32);
    let mut ret = one.clone();
    assert!(k >= n, "k={}, n={}", k, n);
    let t = k-n;
    if(t < n) {
        ret = combination(k, t);
    } else {
        let mut vec_n = Vec::new();
        let mut vec_r = Vec::new();
        let mut i = k;
        let mut j = n;
        let m = k - n;
        while i > m {
            vec_n.push(BigUint::from(i));
            vec_r.push(BigUint::from(j));
            i -= 1;
            j -= 1;
        }
        for i in 0..n {
            if vec_n[i].gt(&one) {
                for  j in 0..n {
                    if vec_r[j].gt(&one) {
                        let gcd = vec_n[i].gcd(&vec_r[j]);
                        if gcd.gt(&one) {
                            vec_n[i].div_assign(&gcd);
                            vec_r[j].div_assign(&gcd);
                        }
                    }
                }
            }
        }
        for i in 0..n {
            assert!(vec_r[i].eq(&one));
            if vec_n[i].gt(&one) {
                ret = ret.mul(&vec_n[i]);
            }
        }
    }
    ret
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


