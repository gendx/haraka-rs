use u64x2::u64x2;

#[inline(always)]
pub(crate) fn aesenc(block: &mut u64x2, key: &u64x2) {
    unsafe {
        asm!("aesenc $0, $1"
            : "+x"(*block)
            : "x"(*key)
            :
            : "intel", "alignstack"
        );
    }
}

#[inline(always)]
pub(crate) fn pxor(dst: &mut u64x2, src: &u64x2) {
    unsafe {
        asm!("pxor $0, $1"
            : "+x"(*dst)
            : "x"(*src)
            :
            : "intel", "alignstack"
        );
    }
}

#[inline(always)]
pub(crate) fn unpacklo_epi32(dst: &mut u64x2, src: &u64x2) {
    unsafe {
        asm!("punpckldq $0, $1"
            : "+x"(*dst)
            : "x"(*src)
            :
            : "intel", "alignstack"
        );
    }
}

#[inline(always)]
pub(crate) fn unpackhi_epi32(dst: &mut u64x2, src: &u64x2) {
    unsafe {
        asm!("punpckhdq $0, $1"
            : "+x"(*dst)
            : "x"(*src)
            :
            : "intel", "alignstack"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aesenc_slice(block: &mut [u8; 16], key: &[u8; 16]) {
        let mut block_xmm = u64x2::read(block);
        let key_xmm = u64x2::read(key);
        aesenc(&mut block_xmm, &key_xmm);
        block_xmm.write(block);
    }

    #[test]
    fn test_aesenc() {
        let mut dst = [0u8; 16];
        let key = [0u8; 16];
        let expect = [99u8; 16];
        aesenc_slice(&mut dst, &key);
        assert_eq!(dst, expect);
    }

    fn pxor_slice(dst: &mut [u8; 16], src: &[u8; 16]) {
        let mut dst_xmm = u64x2::read(dst);
        let src_xmm = u64x2::read(src);
        pxor(&mut dst_xmm, &src_xmm);
        dst_xmm.write(dst);
    }

    #[test]
    fn test_pxor() {
        let mut dst = [0xb2u8; 16];
        let src = [0xc5u8; 16];
        let expect = [(0xb2u8 ^ 0xc5u8); 16];
        pxor_slice(&mut dst, &src);
        assert_eq!(dst, expect);
    }

    fn unpacklo_epi32_slice(dst: &mut [u8; 16], src: &[u8; 16]) {
        let mut dst_xmm = u64x2::read(dst);
        let src_xmm = u64x2::read(src);
        unpacklo_epi32(&mut dst_xmm, &src_xmm);
        dst_xmm.write(dst);
    }

    #[test]
    fn test_unpacklo_epi32() {
        let mut dst = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let src = [
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let expect = [0, 1, 2, 3, 16, 17, 18, 19, 4, 5, 6, 7, 20, 21, 22, 23];
        unpacklo_epi32_slice(&mut dst, &src);
        assert_eq!(dst, expect);
    }

    fn unpackhi_epi32_slice(dst: &mut [u8; 16], src: &[u8; 16]) {
        let mut dst_xmm = u64x2::read(dst);
        let src_xmm = u64x2::read(src);
        unpackhi_epi32(&mut dst_xmm, &src_xmm);
        dst_xmm.write(dst);
    }

    #[test]
    fn test_unpackhi_epi32() {
        let mut dst = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let src = [
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let expect = [8, 9, 10, 11, 24, 25, 26, 27, 12, 13, 14, 15, 28, 29, 30, 31];
        unpackhi_epi32_slice(&mut dst, &src);
        assert_eq!(dst, expect);
    }
}
