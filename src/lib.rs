#![feature(asm)]
#![feature(repr_simd)]

#[macro_use]
extern crate arrayref;

mod u64x2;
mod intrinsics;
mod haraka256;
mod haraka512;
mod constants;

pub fn haraka256_5round(dst: &mut [u8; 32], src: &[u8; 32]) {
    haraka256::haraka256_5round(dst, src)
}

pub fn haraka512_5round(dst: &mut [u8; 32], src: &[u8; 64]) {
    haraka512::haraka512_5round(dst, src)
}

pub fn haraka256_6round(dst: &mut [u8; 32], src: &[u8; 32]) {
    haraka256::haraka256_6round(dst, src)
}

pub fn haraka512_6round(dst: &mut [u8; 32], src: &[u8; 64]) {
    haraka512::haraka512_6round(dst, src)
}
