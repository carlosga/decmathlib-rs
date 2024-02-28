/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_internal::*;

pub (crate) fn short_sqrt128(A10: &BID_UINT128) -> BID_UINT64 {
    let mut ARS: BID_UINT256;
    let ARS0: BID_UINT256;
    let AE0: BID_UINT256;
    let mut AE: BID_UINT256 = BID_UINT256::default();
    let mut S: BID_UINT256 = BID_UINT256::default();
    let MY: BID_UINT64;
    let mut ES: BID_UINT64;
    let mut CY: BID_UINT64;
    let lx: f64;
    let l64: f64;
    let mut f64: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut ly: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let ey: i32;
    let mut k: i32;

    unsafe {
        // 2^64
        f64.ui64 = 0x43f0000000000000u64;
        l64   = f64.d;
        lx    = (A10.w[1] as f64) * l64 + (A10.w[0] as f64);
        ly.d  = 1.0 / lx.sqrt();
        MY    = (ly.ui64 & 0x000fffffffffffffu64) | 0x0010000000000000u64;
        ey    = (0x3ff - (ly.ui64 >> 52)) as i32;
    }
    // A10*RS^2
    ARS0 = __mul_64x128_to_256(MY, A10);
    ARS  = __mul_64x256_to_256(MY, &ARS0);
    // shr by 2*ey+40, to get a 64-bit value
    k = (ey << 1) + 104 - 64;
    if k >= 128 {
        if k > 128 {
            ES = (ARS.w[2] >> (k - 128)) | (ARS.w[3] << (192 - k));
        } else {
            ES = ARS.w[2];
        }
    } else {
        if k >= 64 {
            ARS.w[0] = ARS.w[1];
            ARS.w[1] = ARS.w[2];
            k       -= 64;
        }
        if k != 0 {
            ARS = __shr_256(&ARS, k);
        }
        ES = ARS.w[0];
    }
    ES = ((ES as BID_SINT64) >> 1) as BID_UINT64;
    if (ES as BID_SINT64) < 0 {
        ES           = -(ES as BID_SINT64) as BID_UINT64;
        // A*RS*eps (scaled by 2^64)
        AE0          = __mul_64x256_to_256 (ES, &ARS0);
        AE.w[0]      = AE0.w[1];
        AE.w[1]      = AE0.w[2];
        AE.w[2]      = AE0.w[3];
        (S.w[0], CY) = __add_carry_out(ARS0.w[0], AE.w[0]);
        (S.w[1], CY) = __add_carry_in_out(ARS0.w[1], AE.w[1], CY);
        S.w[2]       = ARS0.w[2] + AE.w[2] + CY;
    } else {
        // A*RS*eps (scaled by 2^64)
        AE0          = __mul_64x256_to_256(ES, &ARS0);
        AE.w[0]      = AE0.w[1];
        AE.w[1]      = AE0.w[2];
        AE.w[2]      = AE0.w[3];
        (S.w[0], CY) = __sub_borrow_out(ARS0.w[0], AE.w[0]);
        (S.w[1], CY) = __sub_borrow_in_out(ARS0.w[1], AE.w[1], CY);
        S.w[2]       = ARS0.w[2] - AE.w[2] - CY;
    }
    k = ey + 51;
    if k >= 64 {
        if k >= 128 {
            S.w[0] = S.w[2];
            S.w[1] = 0;
            k     -= 128;
        } else {
            S.w[0] = S.w[1];
            S.w[1] = S.w[2];
        }
        k -= 64;
    }
    if k != 0 {
      S = __shr_256(&S, k);
    }
    ((S.w[0] + 1) >> 1) as BID_UINT64
}

pub (crate) fn bid_long_sqrt128(pCS: &mut BID_UINT128, C256: &BID_UINT256) {
    let ARS0: BID_UINT512;
    let ARS: BID_UINT512;
    let mut ARS00: BID_UINT256 = BID_UINT256::default();
    let AE: BID_UINT256;
    let AE2: BID_UINT256;
    let mut S: BID_UINT256 = BID_UINT256::default();
    let mut ES: BID_UINT128 = BID_UINT128::default();
    let ES2: BID_UINT128;
    let mut ARS1: BID_UINT128 = BID_UINT128::default();
    let ES32: BID_UINT64;
    let mut CY: BID_UINT64;
    let MY: BID_UINT64;
    let l64: f64;
    let l128: f64;
    let mut lx: f64;
    let l2: f64;
    let l1: f64;
    let l0: f64;
    let mut f64: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let mut ly: BID_UI64DOUBLE = BID_UI64DOUBLE::default();
    let ey: i32;
    let mut k: i32;
    let mut k2: i32;

    unsafe {
        // 2^64
        f64.ui64 = 0x43f0000000000000u64;
        l64   = f64.d;

        l128 = l64 * l64;
        lx   = (C256.w[3] as f64) * l64 * l128;
        l2   = (C256.w[2] as f64) * l128;
        lx  += l2;
        l1   = (C256.w[1] as f64) * l64;
        lx  += l1;
        l0   = C256.w[0] as f64;
        lx  += l0;
        // sqrt(C256)
        ly.d = 1.0 / lx.sqrt();

        MY = (ly.ui64 & 0x000fffffffffffffu64) | 0x0010000000000000u64;
        ey = (0x3ff - (ly.ui64 >> 52)) as i32;
    }

    // A10*RS^2, scaled by 2^(2*ey+104)
    ARS0 = __mul_64x256_to_320(MY, C256);
    ARS  = __mul_64x320_to_512(MY, &ARS0);

    // shr by k=(2*ey+104)-128
    // expect k is in the range (192, 256) if result in [10^33, 10^34)
    // apply an additional signed shift by 1 at the same time (to get eps=eps0/2)
    k = (ey << 1) + 104 - 128 - 192;
    k2 = 64 - k;
    ES.w[0] = (ARS.w[3] >> (k + 1)) | (ARS.w[4] << (k2 - 1));
    ES.w[1] = (ARS.w[4] >> k) | (ARS.w[5] << k2);
    ES.w[1] = (ES.w[1] as BID_SINT64 >> 1) as BID_UINT64;

    // A*RS >> 192 (for error term computation)
    ARS1.w[0] = ARS0.w[3];
    ARS1.w[1] = ARS0.w[4];

    // A*RS>>64
    ARS00.w[0] = ARS0.w[1];
    ARS00.w[1] = ARS0.w[2];
    ARS00.w[2] = ARS0.w[3];
    ARS00.w[3] = ARS0.w[4];

    if (ES.w[1] as BID_SINT64) < 0 {
        ES.w[0] = -(ES.w[0] as BID_SINT64) as BID_UINT64;
        ES.w[1] = -(ES.w[1] as BID_SINT64) as BID_UINT64;
        if ES.w[0] != 0 {
            ES.w[1] -= 1;
        }

        // A*RS*eps
        AE = __mul_128x128_to_256 (&ES, &ARS1);

        (S.w[0], CY) = __add_carry_out(ARS00.w[0], AE.w[0]);
        (S.w[1], CY) = __add_carry_in_out (ARS00.w[1], AE.w[1], CY);
        (S.w[2], CY) = __add_carry_in_out (ARS00.w[2], AE.w[2], CY);
        S.w[3]       = ARS00.w[3] + AE.w[3] + CY;
    } else {
        // A*RS*eps
        AE = __mul_128x128_to_256(&ES, &ARS1);

        (S.w[0], CY) = __sub_borrow_out(ARS00.w[0], AE.w[0]);
        (S.w[1], CY) = __sub_borrow_in_out (ARS00.w[1], AE.w[1], CY);
        (S.w[2], CY) = __sub_borrow_in_out (ARS00.w[2], AE.w[2], CY);
        S.w[3]       = ARS00.w[3] - AE.w[3] - CY;
    }

    // 3/2*eps^2, scaled by 2^128
    ES32 = ES.w[1] + (ES.w[1] >> 1);
    ES2  = __mul_64x64_to_128(ES32, ES.w[1]);
    // A*RS*3/2*eps^2
    AE2 = __mul_128x128_to_256(&ES2, &ARS1);

    // result, scaled by 2^(ey+52-64)
    (S.w[0], CY) = __add_carry_out(S.w[0], AE2.w[0]);
    (S.w[1], CY) = __add_carry_in_out(S.w[1], AE2.w[1], CY);
    (S.w[2], CY) = __add_carry_in_out(S.w[2], AE2.w[2], CY);
    S.w[3]       = S.w[3] + AE2.w[3] + CY;

    // k in (0, 64)
    k      = ey + 51 - 128;
    k2     = 64 - k;
    S.w[0] = (S.w[1] >> k) | (S.w[2] << k2);
    S.w[1] = (S.w[2] >> k) | (S.w[3] << k2);

    // round to nearest
    S.w[0] += 1;
    if S.w[0] == 0 {
        S.w[1] += 1;
    }

    pCS.w[0] = (S.w[1] << 63) | (S.w[0] >> 1);
    pCS.w[1] = S.w[1] >> 1;
}
