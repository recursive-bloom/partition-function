use std::io;
use num::integer::gcd;
use num::bigint::BigInt;
use num::bigint::BigUint;
use std::ops::{Div, Mul, AddAssign, DivAssign, Sub, MulAssign, Add};

use bigdecimal::{BigDecimal, ToPrimitive};
use std::str::FromStr;

pub const BASE_UNIT: u128 = 1_000_000_000_000;
pub const BASE_AMOUNT: u128 = 100;
pub const INIT_PRICE: u128 = 1; // The token-amount 1 EHT could buy.
pub const RCW: u128 = 2; // Reciprocal CW, CW is 0.5 (50%, 1/2).
pub const baseBalance: u128 = BASE_AMOUNT * BASE_UNIT;
pub const baseSupply: u128 = baseBalance * RCW * INIT_PRICE;


pub fn realSupply(virtualSupply: &u128) -> u128 {
    virtualSupply - baseSupply
}

pub fn realBanlance(virtualBalance: &u128) -> u128 {
    virtualBalance - baseBalance
}

pub fn toBigDecimal(value: u128) -> BigDecimal {
    BigDecimal::from(BigInt::from(value))
}

pub fn PriceAsToken(virtualSupply: &u128, virtualBalance: &u128) -> BigDecimal {
    toBigDecimal(*virtualSupply).div(toBigDecimal((*virtualBalance) * 2))
}

pub fn PriceAsETH(virtualSupply: &u128, virtualBalance: &u128) -> BigDecimal {
    toBigDecimal((*virtualBalance) * 2).div(toBigDecimal(*virtualSupply))
}

/*****************************************************************
tknWei = supply*((1+ethWei/ethBlance)^(1/2)-1)
       = supply*(sqrt((ethBlance+ethWei)/ethBlance)-1);
       = supply*sqrt((ethBlance+ethWei)/ethBlance)-supply;
       = sqrt(supply*supply*(ethBlance+ethWei)/ethBlance)-supply;
       = sqrt(supply*supply*sum/ethBlance)-supply;
*****************************************************************/
// When ethWei is ZERO, tknWei might be NON-ZERO.
// This is because sell fn retun eth value is less than precise value.
// So it will Accumulate small amount of differences.
pub fn bancorBuyToken(ethWei: u128, virtualSupply: &mut u128, virtualBalance: &mut u128) -> u128 {
    let savedSupply = *virtualSupply;
    *virtualBalance = *virtualBalance + ethWei; //sum is new ethBlance.
    *virtualSupply = toBigDecimal(baseSupply)
        .mul(toBigDecimal(baseSupply))
        .mul(toBigDecimal(*virtualBalance))
        .div(toBigDecimal(baseBalance))
        .sqrt().unwrap().to_u128().unwrap();
    let mut tknWei = *virtualSupply - savedSupply;
    if(ethWei == 0) { // to reduce Accumulated differences.
        tknWei = 0;
    }
    tknWei
}

// ETH = (((Token/Supply)+1)^2 - 1) * Balance
pub fn bancorBuyTokenReverse(tknWei: u128, virtualSupply: &u128, virtualBalance: &u128) -> u128 {
    let ethWei = toBigDecimal(tknWei)
        .div(toBigDecimal(*virtualSupply))
        .add(BigInt::from(1u8)).square().sub(BigInt::from(1u8))
        .mul(toBigDecimal(*virtualBalance)).to_u128().unwrap();
    ethWei
}

/*****************************************************************
ethWei = ethBlance*(1-(1-(tknWei/supply))^2);
       = ethBlance*(1-((supply-tknWei)/supply)^2)
       = ethBlance*(1-((supply-tknWei)^2)/(supply^2))
       = ethBlance*(1-delta^2/supply^2)
       = ethBlance*(supply^2-delta^2)/supply^2
       = ethBlance*(supply+delta)*(supply-delta)/(supply*supply)
*****************************************************************/
pub fn bancorSellToken(tknWei: u128, virtualSupply: &mut u128, virtualBalance: &mut u128)  -> u128 {
    let delta = *virtualSupply - tknWei;
    // println!("delta = {:?}", delta);
    assert!(delta >= baseSupply);
    let ethWei = toBigDecimal(*virtualBalance)
        .mul(toBigDecimal(*virtualSupply + delta))
        .mul(toBigDecimal(*virtualSupply - delta))
        .div(toBigDecimal(*virtualSupply).mul(toBigDecimal(*virtualSupply))).to_u128().unwrap();
    *virtualSupply = *virtualSupply - tknWei;
    *virtualBalance = *virtualBalance - ethWei;
    ethWei
}

// Token = (1 - (1-(ETH/Balance))^(1/2)) * Supply
pub fn bancorSellTokenReverse(ethWei: u128, virtualSupply: &u128, virtualBalance: &u128) -> u128 {
    let tknWei = toBigDecimal(1)
        .sub(toBigDecimal(1)
            .sub(toBigDecimal(ethWei).
                div(toBigDecimal(*virtualBalance)))
            .sqrt().unwrap()
        ).mul(toBigDecimal(*virtualSupply)).to_u128().unwrap();
    tknWei
}

fn main() {
    let mut virtualSupply = baseSupply;
    let mut virtualBalance = baseBalance;

    println!("{:?}, {:?}, {:?}", virtualSupply, virtualBalance, PriceAsToken(&virtualSupply, &virtualBalance));
    // let bar = bancorSell(1, &mut virtualSupply, &mut virtualBalance);

    let foo = bancorBuyTokenReverse(82842712474619, &virtualSupply, &virtualBalance);
    println!("foo = {:?}, baseBalance = {:?}", foo, baseBalance);

    let foo = bancorBuyTokenReverse(2*baseBalance, &virtualSupply, &virtualBalance);
    println!("foo = {:?}, baseBalance = {:?}", foo, baseBalance);

    let foo = bancorBuyToken(baseBalance, &mut virtualSupply, &mut virtualBalance);
    println!("{:?}, {:?}, foo = {:?}, price = {:?}", virtualSupply, virtualBalance, foo, PriceAsToken(&virtualSupply, &virtualBalance));

    let bar = bancorSellTokenReverse(baseBalance, &virtualSupply, &virtualBalance);
    println!("bar = {:?}, baseBalance = {:?}", bar, baseBalance);

    let bar = bancorSellToken(foo, &mut virtualSupply, &mut virtualBalance);
    println!("{:?}, {:?}, bar = {:?}, price = {:?}", virtualSupply, virtualBalance, bar, PriceAsToken(&virtualSupply, &virtualBalance));

}
