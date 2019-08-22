#![feature(asm)]
#![feature(repr_simd)]
#![feature(const_generics)]
#![feature(test)]

#[macro_use]
extern crate arrayref;
extern crate byteorder;
#[cfg(test)]
extern crate test;

mod constants;
mod haraka256;
mod haraka512;
mod intrinsics;
mod u64x2;

pub fn haraka256<const N_ROUNDS: usize>(dst: &mut [u8; 32], src: &[u8; 32]) {
    haraka256::haraka256::<{ N_ROUNDS }>(dst, src)
}

pub fn haraka512<const N_ROUNDS: usize>(dst: &mut [u8; 32], src: &[u8; 64]) {
    haraka512::haraka512::<{ N_ROUNDS }>(dst, src)
}
