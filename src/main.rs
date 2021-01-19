
use std::io;
use num::integer::gcd;
use num::bigint::BigInt;
use num::bigint::BigUint;
use std::ops::{Div, Mul, AddAssign, DivAssign, Sub, MulAssign};

use bigdecimal::BigDecimal;
use std::str::FromStr;

pub fn matrix_init(n: usize, m: usize, k: usize, t: usize, r: usize) -> Vec<Vec<Vec<BigUint>>> {
    // assert!(n > 0 && m > 0 && k > 0);
    let mut matrix = vec![vec![vec![BigUint::from(0u32); k+1]; m+1]; n+1];
    for i_k in 1..=k {
        let volume = i_k*t;
        for i_n in 0..=n {
            for i_m in 0..=m {
                if i_n+i_m == volume && i_n <= r {
                    let mut ret = BigUint::from(1u32);
                    for j in 2..=i_k {
                        ret.mul_assign(combination(j*t, t));
                    }
                    matrix[i_n][i_m][i_k] = ret.clone();
                    // println!("matrix[{}][{}][{}] is {}", i_n, i_m, i_k, matrix[i_n][i_m][i_k]);
                }
            }
        }
    }
    // println!("matrix is {:?}", matrix);
    matrix
}

pub fn partition(
    n: usize, m: usize, k: usize, t: usize, r: usize,
    matrix: &mut Vec<Vec<Vec<BigUint>>>, sum: &mut BigUint, hited: &mut BigUint
) -> BigUint {
    // println!("#### Begin #### n = {}, m = {}, k = {}, t = {}, r = {}", n, m, k, t, r);
    assert!(k > 0 && (n > 0 || m > 0));
    assert!(n+m == k*t && r <= t);
    let one = BigUint::from(1u32);
    sum.add_assign(&one);
    let mut ret = BigUint::from(0u32);
    if matrix[n][m][k].ne(&ret) {
        hited.add_assign(&one);
        ret = matrix[n][m][k].clone();
    } else {
        for i in 0..=r {
            if n >= i && m >= t-i && r*(k-1) >= n-i {
                let factor = combination(n, i).mul(combination(m, t-i));
                // println!("Recall** n = {}, i = {}, m = {}, j = {}, factor = {}", n, i, m, t-i, factor);
                ret.add_assign(factor.mul(partition(n-i, m+i-t, k-1, t, r, matrix, sum, hited)));
            }
        }
        matrix[n][m][k] = ret.clone();
    }
    ret
}

// C(k, n) = k! / n!(k-n)!
// C(100, 20) = (100!) / ((20!) * (80!)) = 100*99*...*82*81 / 20*19*...*2*1
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
fn test_combination_easy() {
    let ret = combination(6, 3);
    println!("{}", ret);
}


fn main() {
    let N = 333;
    let mut m;
    let k = 100;
    let t = 10;
    let r = 5;
    for n in 0..=N {
        let mut sum = BigUint::from(0u32);
        let mut hited = BigUint::from(0u32);
        m = 1000 - n;
        let mut matrix = matrix_init(n, m, k, t, r);
        let ret = partition(n, m, k, t, r, &mut matrix, &mut sum, &mut hited);
        // println!("Count is  {}", ret);

        let mut total = BigUint::from(1u32);;
        for i in 2..=k {
            total.mul_assign(combination(i * t, t));
        }
        // println!("total is  {}", total);

        let tmp = format!("{}", ret);
        let up = BigDecimal::from_str(&tmp).unwrap();
        // println!("up == {}", up);

        let tmp = format!("{}", total);
        let down = BigDecimal::from_str(&tmp).unwrap();
        // println!("down == {}", down);

        let p = up.div(&down);
        let one = BigDecimal::from_str("1").unwrap();
        println!("#### n = {}", n);
        println!("==== total called {}, hited {}", sum, hited);
        println!("OK, {}", p);
        println!("ERR, {}", one.sub(&p));
    }
}

