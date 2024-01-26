/* ----------------------------------------------------------------------------- */
/* decimal128 type from Intel decimal math library port to Rust.                 */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                 */
/* ----------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.  */
/* ----------------------------------------------------------------------------- */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use crate::d128::{BID_UINT128, BID_UINT192, BID_UINT256, BID_UINT64, DEC_DIGITS};

/// the first entry of bid_nr_digits[i - 1] (where 1 <= i <= 113), indicates
/// the number of decimal digits needed to represent a binary number with i bits;
/// however, if a binary number of i bits may require either k or k + 1 decimal
/// digits, then the first entry of bid_nr_digits[i - 1] is 0; in this case if the
/// number is less than the value represented by the second and third entries
/// concatenated, then the number of decimal digits k is the fourth entry, else
/// the number of decimal digits is the fourth entry plus 1
pub (crate) const bid_nr_digits: [DEC_DIGITS; 113] = [ // only the first entry is used if it is not 0
    DEC_DIGITS { digits: 1   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000000000000au64, digits1: 1},	//   1-bit n < 10^1
    DEC_DIGITS { digits: 1   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000000000000au64, digits1: 1},	//   2-bit n < 10^1
    DEC_DIGITS { digits: 1   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000000000000au64, digits1: 1},	//   3-bit n < 10^1
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000000000000au64, digits1: 1},	//   4-bit n ? 10^1
    DEC_DIGITS { digits: 2   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000000064u64, digits1: 2},	//   5-bit n < 10^2
    DEC_DIGITS { digits: 2   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000000064u64, digits1: 2},	//   6-bit n < 10^2
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000000064u64, digits1: 2},	//   7-bit n ? 10^2
    DEC_DIGITS { digits: 3   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000003e8u64, digits1: 3},	//   8-bit n < 10^3
    DEC_DIGITS { digits: 3   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000003e8u64, digits1: 3},	//   9-bit n < 10^3
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000003e8u64, digits1: 3},	//  10-bit n ? 10^3
    DEC_DIGITS { digits: 4   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000002710u64, digits1: 4},	//  11-bit n < 10^4
    DEC_DIGITS { digits: 4   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000002710u64, digits1: 4},	//  12-bit n < 10^4
    DEC_DIGITS { digits: 4   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000002710u64, digits1: 4},	//  13-bit n < 10^4
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000002710u64, digits1: 4},	//  14-bit n ? 10^4
    DEC_DIGITS { digits: 5   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000186a0u64, digits1: 5},	//  15-bit n < 10^5
    DEC_DIGITS { digits: 5   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000186a0u64, digits1: 5},	//  16-bit n < 10^5
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000186a0u64, digits1: 5},	//  17-bit n ? 10^5
    DEC_DIGITS { digits: 6   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000f4240u64, digits1: 6},	//  18-bit n < 10^6
    DEC_DIGITS { digits: 6   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000f4240u64, digits1: 6},	//  19-bit n < 10^6
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000000000f4240u64, digits1: 6},	//  20-bit n ? 10^6
    DEC_DIGITS { digits: 7   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000989680u64, digits1: 7},	//  21-bit n < 10^7
    DEC_DIGITS { digits: 7   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000989680u64, digits1: 7},	//  22-bit n < 10^7
    DEC_DIGITS { digits: 7   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000989680u64, digits1: 7},	//  23-bit n < 10^7
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000000989680u64, digits1: 7},	//  24-bit n ? 10^7
    DEC_DIGITS { digits: 8   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000005f5e100u64, digits1: 8},	//  25-bit n < 10^8
    DEC_DIGITS { digits: 8   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000005f5e100u64, digits1: 8},	//  26-bit n < 10^8
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0000000005f5e100u64, digits1: 8},	//  27-bit n ? 10^8
    DEC_DIGITS { digits: 9   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000003b9aca00u64, digits1: 9},	//  28-bit n < 10^9
    DEC_DIGITS { digits: 9   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000003b9aca00u64, digits1: 9},	//  29-bit n < 10^9
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000003b9aca00u64, digits1: 9},	//  30-bit n ? 10^9
    DEC_DIGITS { digits: 10  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000002540be400u64, digits1: 10},	//  31-bit n < 10^10
    DEC_DIGITS { digits: 10  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000002540be400u64, digits1: 10},	//  32-bit n < 10^10
    DEC_DIGITS { digits: 10  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000002540be400u64, digits1: 10},	//  33-bit n < 10^10
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00000002540be400u64, digits1: 10},	//  34-bit n ? 10^10
    DEC_DIGITS { digits: 11  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000174876e800u64, digits1: 11},	//  35-bit n < 10^11
    DEC_DIGITS { digits: 11  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000174876e800u64, digits1: 11},	//  36-bit n < 10^11
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000174876e800u64, digits1: 11},	//  37-bit n ? 10^11
    DEC_DIGITS { digits: 12  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000e8d4a51000u64, digits1: 12},	//  38-bit n < 10^12
    DEC_DIGITS { digits: 12  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000e8d4a51000u64, digits1: 12},	//  39-bit n < 10^12
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000000e8d4a51000u64, digits1: 12},	//  40-bit n ? 10^12
    DEC_DIGITS { digits: 13  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000009184e72a000u64, digits1: 13},	//  41-bit n < 10^13
    DEC_DIGITS { digits: 13  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000009184e72a000u64, digits1: 13},	//  42-bit n < 10^13
    DEC_DIGITS { digits: 13  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000009184e72a000u64, digits1: 13},	//  43-bit n < 10^13
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x000009184e72a000u64, digits1: 13},	//  44-bit n ? 10^13
    DEC_DIGITS { digits: 14  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00005af3107a4000u64, digits1: 14},	//  45-bit n < 10^14
    DEC_DIGITS { digits: 14  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00005af3107a4000u64, digits1: 14},	//  46-bit n < 10^14
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00005af3107a4000u64, digits1: 14},	//  47-bit n ? 10^14
    DEC_DIGITS { digits: 15  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00038d7ea4c68000u64, digits1: 15},	//  48-bit n < 10^15
    DEC_DIGITS { digits: 15  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00038d7ea4c68000u64, digits1: 15},	//  49-bit n < 10^15
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x00038d7ea4c68000u64, digits1: 15},	//  50-bit n ? 10^15
    DEC_DIGITS { digits: 16  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x002386f26fc10000u64, digits1: 16},	//  51-bit n < 10^16
    DEC_DIGITS { digits: 16  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x002386f26fc10000u64, digits1: 16},	//  52-bit n < 10^16
    DEC_DIGITS { digits: 16  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x002386f26fc10000u64, digits1: 16},	//  53-bit n < 10^16
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x002386f26fc10000u64, digits1: 16},	//  54-bit n ? 10^16
    DEC_DIGITS { digits: 17  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x016345785d8a0000u64, digits1: 17},	//  55-bit n < 10^17
    DEC_DIGITS { digits: 17  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x016345785d8a0000u64, digits1: 17},	//  56-bit n < 10^17
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x016345785d8a0000u64, digits1: 17},	//  57-bit n ? 10^17
    DEC_DIGITS { digits: 18  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0de0b6b3a7640000u64, digits1: 18},	//  58-bit n < 10^18
    DEC_DIGITS { digits: 18  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0de0b6b3a7640000u64, digits1: 18},	//  59-bit n < 10^18
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x0de0b6b3a7640000u64, digits1: 18},	//  60-bit n ? 10^18
    DEC_DIGITS { digits: 19  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x8ac7230489e80000u64, digits1: 19},	//  61-bit n < 10^19
    DEC_DIGITS { digits: 19  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x8ac7230489e80000u64, digits1: 19},	//  62-bit n < 10^19
    DEC_DIGITS { digits: 19  , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x8ac7230489e80000u64, digits1: 19},	//  63-bit n < 10^19
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000000u64, threshold_lo: 0x8ac7230489e80000u64, digits1: 19},	//  64-bit n ? 10^19
    DEC_DIGITS { digits: 20  , threshold_hi: 0x0000000000000005u64, threshold_lo: 0x6bc75e2d63100000u64, digits1: 20},	//  65-bit n < 10^20
    DEC_DIGITS { digits: 20  , threshold_hi: 0x0000000000000005u64, threshold_lo: 0x6bc75e2d63100000u64, digits1: 20},	//  66-bit n < 10^20
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000005u64, threshold_lo: 0x6bc75e2d63100000u64, digits1: 20},	//  67-bit n ? 10^20
    DEC_DIGITS { digits: 21  , threshold_hi: 0x0000000000000036u64, threshold_lo: 0x35c9adc5dea00000u64, digits1: 21},	//  68-bit n < 10^21
    DEC_DIGITS { digits: 21  , threshold_hi: 0x0000000000000036u64, threshold_lo: 0x35c9adc5dea00000u64, digits1: 21},	//  69-bit n < 10^21
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000000036u64, threshold_lo: 0x35c9adc5dea00000u64, digits1: 21},	//  70-bit n ? 10^21
    DEC_DIGITS { digits: 22  , threshold_hi: 0x000000000000021eu64, threshold_lo: 0x19e0c9bab2400000u64, digits1: 22},	//  71-bit n < 10^22
    DEC_DIGITS { digits: 22  , threshold_hi: 0x000000000000021eu64, threshold_lo: 0x19e0c9bab2400000u64, digits1: 22},	//  72-bit n < 10^22
    DEC_DIGITS { digits: 22  , threshold_hi: 0x000000000000021eu64, threshold_lo: 0x19e0c9bab2400000u64, digits1: 22},	//  73-bit n < 10^22
    DEC_DIGITS { digits: 0   , threshold_hi: 0x000000000000021eu64, threshold_lo: 0x19e0c9bab2400000u64, digits1: 22},	//  74-bit n ? 10^22
    DEC_DIGITS { digits: 23  , threshold_hi: 0x000000000000152du64, threshold_lo: 0x02c7e14af6800000u64, digits1: 23},	//  75-bit n < 10^23
    DEC_DIGITS { digits: 23  , threshold_hi: 0x000000000000152du64, threshold_lo: 0x02c7e14af6800000u64, digits1: 23},	//  76-bit n < 10^23
    DEC_DIGITS { digits: 0   , threshold_hi: 0x000000000000152du64, threshold_lo: 0x02c7e14af6800000u64, digits1: 23},	//  77-bit n ? 10^23
    DEC_DIGITS { digits: 24  , threshold_hi: 0x000000000000d3c2u64, threshold_lo: 0x1bcecceda1000000u64, digits1: 24},	//  78-bit n < 10^24
    DEC_DIGITS { digits: 24  , threshold_hi: 0x000000000000d3c2u64, threshold_lo: 0x1bcecceda1000000u64, digits1: 24},	//  79-bit n < 10^24
    DEC_DIGITS { digits: 0   , threshold_hi: 0x000000000000d3c2u64, threshold_lo: 0x1bcecceda1000000u64, digits1: 24},	//  80-bit n ? 10^24
    DEC_DIGITS { digits: 25  , threshold_hi: 0x0000000000084595u64, threshold_lo: 0x161401484a000000u64, digits1: 25},	//  81-bit n < 10^25
    DEC_DIGITS { digits: 25  , threshold_hi: 0x0000000000084595u64, threshold_lo: 0x161401484a000000u64, digits1: 25},	//  82-bit n < 10^25
    DEC_DIGITS { digits: 25  , threshold_hi: 0x0000000000084595u64, threshold_lo: 0x161401484a000000u64, digits1: 25},	//  83-bit n < 10^25
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000000084595u64, threshold_lo: 0x161401484a000000u64, digits1: 25},	//  84-bit n ? 10^25
    DEC_DIGITS { digits: 26  , threshold_hi: 0x000000000052b7d2u64, threshold_lo: 0xdcc80cd2e4000000u64, digits1: 26},	//  85-bit n < 10^26
    DEC_DIGITS { digits: 26  , threshold_hi: 0x000000000052b7d2u64, threshold_lo: 0xdcc80cd2e4000000u64, digits1: 26},	//  86-bit n < 10^26
    DEC_DIGITS { digits: 0   , threshold_hi: 0x000000000052b7d2u64, threshold_lo: 0xdcc80cd2e4000000u64, digits1: 26},	//  87-bit n ? 10^26
    DEC_DIGITS { digits: 27  , threshold_hi: 0x00000000033b2e3cu64, threshold_lo: 0x9fd0803ce8000000u64, digits1: 27},	//  88-bit n < 10^27
    DEC_DIGITS { digits: 27  , threshold_hi: 0x00000000033b2e3cu64, threshold_lo: 0x9fd0803ce8000000u64, digits1: 27},	//  89-bit n < 10^27
    DEC_DIGITS { digits: 0   , threshold_hi: 0x00000000033b2e3cu64, threshold_lo: 0x9fd0803ce8000000u64, digits1: 27},	//  90-bit n ? 10^27
    DEC_DIGITS { digits: 28  , threshold_hi: 0x00000000204fce5eu64, threshold_lo: 0x3e25026110000000u64, digits1: 28},	//  91-bit n < 10^28
    DEC_DIGITS { digits: 28  , threshold_hi: 0x00000000204fce5eu64, threshold_lo: 0x3e25026110000000u64, digits1: 28},	//  92-bit n < 10^28
    DEC_DIGITS { digits: 28  , threshold_hi: 0x00000000204fce5eu64, threshold_lo: 0x3e25026110000000u64, digits1: 28},	//  93-bit n < 10^28
    DEC_DIGITS { digits: 0   , threshold_hi: 0x00000000204fce5eu64, threshold_lo: 0x3e25026110000000u64, digits1: 28},	//  94-bit n ? 10^28
    DEC_DIGITS { digits: 29  , threshold_hi: 0x00000001431e0faeu64, threshold_lo: 0x6d7217caa0000000u64, digits1: 29},	//  95-bit n < 10^29
    DEC_DIGITS { digits: 29  , threshold_hi: 0x00000001431e0faeu64, threshold_lo: 0x6d7217caa0000000u64, digits1: 29},	//  96-bit n < 10^29
    DEC_DIGITS { digits: 0   , threshold_hi: 0x00000001431e0faeu64, threshold_lo: 0x6d7217caa0000000u64, digits1: 29},	//  97-bit n ? 10^29
    DEC_DIGITS { digits: 30  , threshold_hi: 0x0000000c9f2c9cd0u64, threshold_lo: 0x4674edea40000000u64, digits1: 30},	//  98-bit n < 10^30
    DEC_DIGITS { digits: 30  , threshold_hi: 0x0000000c9f2c9cd0u64, threshold_lo: 0x4674edea40000000u64, digits1: 30},	//  99-bit n < 10^30
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000000c9f2c9cd0u64, threshold_lo: 0x4674edea40000000u64, digits1: 30},	// 100-bit n ? 10^30
    DEC_DIGITS { digits: 31  , threshold_hi: 0x0000007e37be2022u64, threshold_lo: 0xc0914b2680000000u64, digits1: 31},	// 101-bit n < 10^31
    DEC_DIGITS { digits: 31  , threshold_hi: 0x0000007e37be2022u64, threshold_lo: 0xc0914b2680000000u64, digits1: 31},	// 102-bit n < 10^31
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000007e37be2022u64, threshold_lo: 0xc0914b2680000000u64, digits1: 31},	// 103-bit n ? 10^31
    DEC_DIGITS { digits: 32  , threshold_hi: 0x000004ee2d6d415bu64, threshold_lo: 0x85acef8100000000u64, digits1: 32},	// 104-bit n < 10^32
    DEC_DIGITS { digits: 32  , threshold_hi: 0x000004ee2d6d415bu64, threshold_lo: 0x85acef8100000000u64, digits1: 32},	// 105-bit n < 10^32
    DEC_DIGITS { digits: 32  , threshold_hi: 0x000004ee2d6d415bu64, threshold_lo: 0x85acef8100000000u64, digits1: 32},	// 106-bit n < 10^32
    DEC_DIGITS { digits: 0   , threshold_hi: 0x000004ee2d6d415bu64, threshold_lo: 0x85acef8100000000u64, digits1: 32},	// 107-bit n ? 10^32
    DEC_DIGITS { digits: 33  , threshold_hi: 0x0000314dc6448d93u64, threshold_lo: 0x38c15b0a00000000u64, digits1: 33},	// 108-bit n < 10^33
    DEC_DIGITS { digits: 33  , threshold_hi: 0x0000314dc6448d93u64, threshold_lo: 0x38c15b0a00000000u64, digits1: 33},	// 109-bit n < 10^33
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0000314dc6448d93u64, threshold_lo: 0x38c15b0a00000000u64, digits1: 33},	// 100-bit n ? 10^33
    DEC_DIGITS { digits: 34  , threshold_hi: 0x0001ed09bead87c0u64, threshold_lo: 0x378d8e6400000000u64, digits1: 34},	// 111-bit n < 10^34
    DEC_DIGITS { digits: 34  , threshold_hi: 0x0001ed09bead87c0u64, threshold_lo: 0x378d8e6400000000u64, digits1: 34},	// 112-bit n < 10^34
    DEC_DIGITS { digits: 0   , threshold_hi: 0x0001ed09bead87c0u64, threshold_lo: 0x378d8e6400000000u64, digits1: 34}	// 113-bit n ? 10^34
    //{ 35, 0x0013426172c74d82u64, 0x2b878fe800000000u64, 35 }  // 114-bit n < 10^35
];


/// bid_midpoint64[i - 1] = 1/2 * 10^i = 5 * 10^(i-1), 1 <= i <= 19
pub (crate) const bid_midpoint64: [BID_UINT64; 19] = [
    0x0000000000000005u64,    // 1/2 * 10^1 = 5 * 10^0
    0x0000000000000032u64,    // 1/2 * 10^2 = 5 * 10^1
    0x00000000000001f4u64,    // 1/2 * 10^3 = 5 * 10^2
    0x0000000000001388u64,    // 1/2 * 10^4 = 5 * 10^3
    0x000000000000c350u64,    // 1/2 * 10^5 = 5 * 10^4
    0x000000000007a120u64,    // 1/2 * 10^6 = 5 * 10^5
    0x00000000004c4b40u64,    // 1/2 * 10^7 = 5 * 10^6
    0x0000000002faf080u64,    // 1/2 * 10^8 = 5 * 10^7
    0x000000001dcd6500u64,    // 1/2 * 10^9 = 5 * 10^8
    0x000000012a05f200u64,    // 1/2 * 10^10 = 5 * 10^9
    0x0000000ba43b7400u64,    // 1/2 * 10^11 = 5 * 10^10
    0x000000746a528800u64,    // 1/2 * 10^12 = 5 * 10^11
    0x0000048c27395000u64,    // 1/2 * 10^13 = 5 * 10^12
    0x00002d79883d2000u64,    // 1/2 * 10^14 = 5 * 10^13
    0x0001c6bf52634000u64,    // 1/2 * 10^15 = 5 * 10^14
    0x0011c37937e08000u64,    // 1/2 * 10^16 = 5 * 10^15
    0x00b1a2bc2ec50000u64,    // 1/2 * 10^17 = 5 * 10^16
    0x06f05b59d3b20000u64,    // 1/2 * 10^18 = 5 * 10^17
    0x4563918244f40000u64	    // 1/2 * 10^19 = 5 * 10^18
];

/// bid_midpoint128[i - 20] = 1/2 * 10^i = 5 * 10^(i-1), 20 <= i <= 38
pub (crate) const bid_midpoint128: [BID_UINT128; 19] = [	         // the 64-bit word order is L, H
    BID_UINT128 { w: [0xb5e3af16b1880000u64, 0x0000000000000002u64] }, // 1/2 * 10^20 = 5 * 10^19
    BID_UINT128 { w: [0x1ae4d6e2ef500000u64, 0x000000000000001bu64] }, // 1/2 * 10^21 = 5 * 10^20
    BID_UINT128 { w: [0x0cf064dd59200000u64, 0x000000000000010fu64] }, // 1/2 * 10^22 = 5 * 10^21
    BID_UINT128 { w: [0x8163f0a57b400000u64, 0x0000000000000a96u64] }, // 1/2 * 10^23 = 5 * 10^22
    BID_UINT128 { w: [0x0de76676d0800000u64, 0x00000000000069e1u64] }, // 1/2 * 10^24 = 5 * 10^23
    BID_UINT128 { w: [0x8b0a00a425000000u64, 0x00000000000422cau64] }, // 1/2 * 10^25 = 5 * 10^24
    BID_UINT128 { w: [0x6e64066972000000u64, 0x0000000000295be9u64] }, // 1/2 * 10^26 = 5 * 10^25
    BID_UINT128 { w: [0x4fe8401e74000000u64, 0x00000000019d971eu64] }, // 1/2 * 10^27 = 5 * 10^26
    BID_UINT128 { w: [0x1f12813088000000u64, 0x000000001027e72fu64] }, // 1/2 * 10^28 = 5 * 10^27
    BID_UINT128 { w: [0x36b90be550000000u64, 0x00000000a18f07d7u64] }, // 1/2 * 10^29 = 5 * 10^28
    BID_UINT128 { w: [0x233a76f520000000u64, 0x000000064f964e68u64] }, // 1/2 * 10^30 = 5 * 10^29
    BID_UINT128 { w: [0x6048a59340000000u64, 0x0000003f1bdf1011u64] }, // 1/2 * 10^31 = 5 * 10^30
    BID_UINT128 { w: [0xc2d677c080000000u64, 0x0000027716b6a0adu64] }, // 1/2 * 10^32 = 5 * 10^31
    BID_UINT128 { w: [0x9c60ad8500000000u64, 0x000018a6e32246c9u64] }, // 1/2 * 10^33 = 5 * 10^32
    BID_UINT128 { w: [0x1bc6c73200000000u64, 0x0000f684df56c3e0u64] }, // 1/2 * 10^34 = 5 * 10^33
    BID_UINT128 { w: [0x15c3c7f400000000u64, 0x0009a130b963a6c1u64] }, // 1/2 * 10^35 = 5 * 10^34
    BID_UINT128 { w: [0xd9a5cf8800000000u64, 0x00604be73de4838au64] }, // 1/2 * 10^36 = 5 * 10^35
    BID_UINT128 { w: [0x807a1b5000000000u64, 0x03c2f7086aed236cu64] }, // 1/2 * 10^37 = 5 * 10^36
    BID_UINT128 { w: [0x04c5112000000000u64, 0x259da6542d43623du64] }  // 1/2 * 10^38 = 5 * 10^37
];

/// bid_midpoint192[i - 39] = 1/2 * 10^i = 5 * 10^(i-1), 39 <= i <= 58
pub (crate) const bid_midpoint192: [BID_UINT192; 20] = [ // the 64-bit word order is L, M, H
    BID_UINT192 { w: [0x2fb2ab4000000000u64, 0x78287f49c4a1d662u64, 0x0000000000000001u64 ] },  // 1/2 * 10^39 = 5 * 10^38
    BID_UINT192 { w: [0xdcfab08000000000u64, 0xb194f8e1ae525fd5u64, 0x000000000000000eu64 ] },  // 1/2 * 10^40 = 5 * 10^39
    BID_UINT192 { w: [0xa1cae50000000000u64, 0xefd1b8d0cf37be5au64, 0x0000000000000092u64 ] },  // 1/2 * 10^41 = 5 * 10^40
    BID_UINT192 { w: [0x51ecf20000000000u64, 0x5e313828182d6f8au64, 0x00000000000005bdu64 ] },  // 1/2 * 10^42 = 5 * 10^41
    BID_UINT192 { w: [0x3341740000000000u64, 0xadec3190f1c65b67u64, 0x0000000000003965u64 ] },  // 1/2 * 10^43 = 5 * 10^42
    BID_UINT192 { w: [0x008e880000000000u64, 0xcb39efa971bf9208u64, 0x0000000000023df8u64 ] },  // 1/2 * 10^44 = 5 * 10^43
    BID_UINT192 { w: [0x0591500000000000u64, 0xf0435c9e717bb450u64, 0x0000000000166bb7u64 ] },  // 1/2 * 10^45 = 5 * 10^44
    BID_UINT192 { w: [0x37ad200000000000u64, 0x62a19e306ed50b20u64, 0x0000000000e0352fu64 ] },  // 1/2 * 10^46 = 5 * 10^45
    BID_UINT192 { w: [0x2cc3400000000000u64, 0xda502de454526f42u64, 0x0000000008c213d9u64 ] },  // 1/2 * 10^47 = 5 * 10^46
    BID_UINT192 { w: [0xbfa0800000000000u64, 0x8721caeb4b385895u64, 0x000000005794c682u64 ] },  // 1/2 * 10^48 = 5 * 10^47
    BID_UINT192 { w: [0x7c45000000000000u64, 0x4751ed30f03375d9u64, 0x000000036bcfc119u64 ] },  // 1/2 * 10^49 = 5 * 10^48
    BID_UINT192 { w: [0xdab2000000000000u64, 0xc93343e962029a7eu64, 0x00000022361d8afcu64 ] },  // 1/2 * 10^50 = 5 * 10^49
    BID_UINT192 { w: [0x8af4000000000000u64, 0xdc00a71dd41a08f4u64, 0x000001561d276ddfu64 ] },  // 1/2 * 10^51 = 5 * 10^50
    BID_UINT192 { w: [0x6d88000000000000u64, 0x9806872a4904598du64, 0x00000d5d238a4abeu64 ] },  // 1/2 * 10^52 = 5 * 10^51
    BID_UINT192 { w: [0x4750000000000000u64, 0xf04147a6da2b7f86u64, 0x000085a36366eb71u64 ] },  // 1/2 * 10^53 = 5 * 10^52
    BID_UINT192 { w: [0xc920000000000000u64, 0x628ccc8485b2fb3eu64, 0x00053861e2053273u64 ] },  // 1/2 * 10^54 = 5 * 10^53
    BID_UINT192 { w: [0xdb40000000000000u64, 0xd97ffd2d38fdd073u64, 0x003433d2d433f881u64 ] },  // 1/2 * 10^55 = 5 * 10^54
    BID_UINT192 { w: [0x9080000000000000u64, 0x7effe3c439ea2486u64, 0x020a063c4a07b512u64 ] },  // 1/2 * 10^56 = 5 * 10^55
    BID_UINT192 { w: [0xa500000000000000u64, 0xf5fee5aa43256d41u64, 0x14643e5ae44d12b8u64 ] },  // 1/2 * 10^57 = 5 * 10^56
    BID_UINT192 { w: [0x7200000000000000u64, 0x9bf4f8a69f764490u64, 0xcbea6f8ceb02bb39u64 ] }   // 1/2 * 10^58 = 5 * 10^57
];

/// bid_midpoint256[i - 59] = 1/2 * 10^i = 5 * 10^(i-1), 59 <= i <= 68
pub (crate) const bid_midpoint256: [BID_UINT256; 19] = [  // the 64-bit word order is LL, LH, HL, HH
    BID_UINT256{ w: [0x7400000000000000u64, 0x1791b6823a9eada4u64, 0xf7285b812e1b5040u64, 0x0000000000000007u64 ] },	// 1/2 * 10^59 = 5 * 10^58
    BID_UINT256{ w: [0x8800000000000000u64, 0xebb121164a32c86cu64, 0xa793930bcd112280u64, 0x000000000000004fu64 ] },	// 1/2 * 10^60 = 5 * 10^59
    BID_UINT256{ w: [0x5000000000000000u64, 0x34eb4adee5fbd43du64, 0x8bc3be7602ab5909u64, 0x000000000000031cu64 ] },	// 1/2 * 10^61 = 5 * 10^60
    BID_UINT256{ w: [0x2000000000000000u64, 0x1130ecb4fbd64a65u64, 0x75a5709c1ab17a5cu64, 0x0000000000001f1du64 ] },	// 1/2 * 10^62 = 5 * 10^61
    BID_UINT256{ w: [0x4000000000000000u64, 0xabe93f11d65ee7f3u64, 0x987666190aeec798u64, 0x0000000000013726u64 ] },	// 1/2 * 10^63 = 5 * 10^62
    BID_UINT256{ w: [0x8000000000000000u64, 0xb71c76b25fb50f80u64, 0xf49ffcfa6d53cbf6u64, 0x00000000000c2781u64 ] },	// 1/2 * 10^64 = 5 * 10^63
    BID_UINT256{ w: [0x0000000000000000u64, 0x271ca2f7bd129b05u64, 0x8e3fe1c84545f7a3u64, 0x0000000000798b13u64 ] },	// 1/2 * 10^65 = 5 * 10^64
    BID_UINT256{ w: [0x0000000000000000u64, 0x871e5dad62ba0e32u64, 0x8e7ed1d2b4bbac5fu64, 0x0000000004bf6ec3u64 ] },	// 1/2 * 10^66 = 5 * 10^65
    BID_UINT256{ w: [0x0000000000000000u64, 0x472fa8c5db448df4u64, 0x90f4323b0f54bbbbu64, 0x000000002f7a53a3u64 ] },	// 1/2 * 10^67 = 5 * 10^66
    BID_UINT256{ w: [0x0000000000000000u64, 0xc7dc97ba90ad8b88u64, 0xa989f64e994f5550u64, 0x00000001dac74463u64 ] },	// 1/2 * 10^68 = 5 * 10^67
    BID_UINT256{ w: [0x0000000000000000u64, 0xce9ded49a6c77350u64, 0x9f639f11fd195527u64, 0x000000128bc8abe4u64 ] },	// 1/2 * 10^69 = 5 * 10^68
    BID_UINT256{ w: [0x0000000000000000u64, 0x122b44e083ca8120u64, 0x39e436b3e2fd538eu64, 0x000000b975d6b6eeu64 ] },	// 1/2 * 10^70 = 5 * 10^69
    BID_UINT256{ w: [0x0000000000000000u64, 0xb5b0b0c525e90b40u64, 0x42ea2306dde5438cu64, 0x0000073e9a63254eu64 ] },	// 1/2 * 10^71 = 5 * 10^70
    BID_UINT256{ w: [0x0000000000000000u64, 0x18e6e7b37b1a7080u64, 0x9d255e44aaf4a37fu64, 0x0000487207df750eu64 ] },	// 1/2 * 10^72 = 5 * 10^71
    BID_UINT256{ w: [0x0000000000000000u64, 0xf9050d02cf086500u64, 0x2375aeaead8e62f6u64, 0x0002d4744eba9292u64 ] },	// 1/2 * 10^73 = 5 * 10^72
    BID_UINT256{ w: [0x0000000000000000u64, 0xba32821c1653f200u64, 0x6298d2d2c78fdda5u64, 0x001c4c8b1349b9b5u64 ] },	// 1/2 * 10^74 = 5 * 10^73
    BID_UINT256{ w: [0x0000000000000000u64, 0x45f91518df477400u64, 0xd9f83c3bcb9ea879u64, 0x011afd6ec0e14115u64 ] },	// 1/2 * 10^75 = 5 * 10^74
    BID_UINT256{ w: [0x0000000000000000u64, 0xbbbad2f8b8ca8800u64, 0x83b25a55f43294bcu64, 0x0b0de65388cc8adau64 ] },	// 1/2 * 10^76 = 5 * 10^75
    BID_UINT256{ w: [0x0000000000000000u64, 0x554c3db737e95000u64, 0x24f7875b89f9cf5fu64, 0x6e8aff4357fd6c89u64 ] }	// 1/2 * 10^77 = 5 * 10^76
];

pub (crate) const bid_ten2k64: [BID_UINT64; 20] = [
    0x0000000000000001u64,    // 10^0
    0x000000000000000au64,    // 10^1
    0x0000000000000064u64,    // 10^2
    0x00000000000003e8u64,    // 10^3
    0x0000000000002710u64,    // 10^4
    0x00000000000186a0u64,    // 10^5
    0x00000000000f4240u64,    // 10^6
    0x0000000000989680u64,    // 10^7
    0x0000000005f5e100u64,    // 10^8
    0x000000003b9aca00u64,    // 10^9
    0x00000002540be400u64,    // 10^10
    0x000000174876e800u64,    // 10^11
    0x000000e8d4a51000u64,    // 10^12
    0x000009184e72a000u64,    // 10^13
    0x00005af3107a4000u64,    // 10^14
    0x00038d7ea4c68000u64,    // 10^15
    0x002386f26fc10000u64,    // 10^16
    0x016345785d8a0000u64,    // 10^17
    0x0de0b6b3a7640000u64,    // 10^18
    0x8ac7230489e80000u64	    // 10^19 (20 digits)
];

/// bid_ten2k128[i - 20] = 10^i, 20 <= i <= 38
pub (crate) const bid_ten2k128: [BID_UINT128; 19] = [                    // the 64-bit word order is L, H
      BID_UINT128 { w: [0x6bc75e2d63100000u64, 0x0000000000000005u64] }, // 10^20
      BID_UINT128 { w: [0x35c9adc5dea00000u64, 0x0000000000000036u64] }, // 10^21
      BID_UINT128 { w: [0x19e0c9bab2400000u64, 0x000000000000021eu64] }, // 10^22
      BID_UINT128 { w: [0x02c7e14af6800000u64, 0x000000000000152du64] }, // 10^23
      BID_UINT128 { w: [0x1bcecceda1000000u64, 0x000000000000d3c2u64] }, // 10^24
      BID_UINT128 { w: [0x161401484a000000u64, 0x0000000000084595u64] }, // 10^25
      BID_UINT128 { w: [0xdcc80cd2e4000000u64, 0x000000000052b7d2u64] }, // 10^26
      BID_UINT128 { w: [0x9fd0803ce8000000u64, 0x00000000033b2e3cu64] }, // 10^27
      BID_UINT128 { w: [0x3e25026110000000u64, 0x00000000204fce5eu64] }, // 10^28
      BID_UINT128 { w: [0x6d7217caa0000000u64, 0x00000001431e0faeu64] }, // 10^29
      BID_UINT128 { w: [0x4674edea40000000u64, 0x0000000c9f2c9cd0u64] }, // 10^30
      BID_UINT128 { w: [0xc0914b2680000000u64, 0x0000007e37be2022u64] }, // 10^31
      BID_UINT128 { w: [0x85acef8100000000u64, 0x000004ee2d6d415bu64] }, // 10^32
      BID_UINT128 { w: [0x38c15b0a00000000u64, 0x0000314dc6448d93u64] }, // 10^33
      BID_UINT128 { w: [0x378d8e6400000000u64, 0x0001ed09bead87c0u64] }, // 10^34
      BID_UINT128 { w: [0x2b878fe800000000u64, 0x0013426172c74d82u64] }, // 10^35
      BID_UINT128 { w: [0xb34b9f1000000000u64, 0x00c097ce7bc90715u64] }, // 10^36
      BID_UINT128 { w: [0x00f436a000000000u64, 0x0785ee10d5da46d9u64] }, // 10^37
      BID_UINT128 { w: [0x098a224000000000u64, 0x4b3b4ca85a86c47au64] }  // 10^38 (39 digits)
];

// might split into ten2k192[] and bid_ten2k256[]

/// bid_ten2k256[i - 39] = 10^i, 39 <= i <= 68
pub (crate) const bid_ten2k256: [BID_UINT256; 39] = [     // the 64-bit word order is LL, LH, HL, HH
    BID_UINT256 { w: [0x5f65568000000000u64, 0xf050fe938943acc4u64, 0x0000000000000002u64, 0x0000000000000000u64] },	// 10^39
    BID_UINT256 { w: [0xb9f5610000000000u64, 0x6329f1c35ca4bfabu64, 0x000000000000001du64, 0x0000000000000000u64] },	// 10^40
    BID_UINT256 { w: [0x4395ca0000000000u64, 0xdfa371a19e6f7cb5u64, 0x0000000000000125u64, 0x0000000000000000u64] },	// 10^41
    BID_UINT256 { w: [0xa3d9e40000000000u64, 0xbc627050305adf14u64, 0x0000000000000b7au64, 0x0000000000000000u64] },	// 10^42
    BID_UINT256 { w: [0x6682e80000000000u64, 0x5bd86321e38cb6ceu64, 0x00000000000072cbu64, 0x0000000000000000u64] },	// 10^43
    BID_UINT256 { w: [0x011d100000000000u64, 0x9673df52e37f2410u64, 0x0000000000047bf1u64, 0x0000000000000000u64] },	// 10^44
    BID_UINT256 { w: [0x0b22a00000000000u64, 0xe086b93ce2f768a0u64, 0x00000000002cd76fu64, 0x0000000000000000u64] },	// 10^45
    BID_UINT256 { w: [0x6f5a400000000000u64, 0xc5433c60ddaa1640u64, 0x0000000001c06a5eu64, 0x0000000000000000u64] },	// 10^46
    BID_UINT256 { w: [0x5986800000000000u64, 0xb4a05bc8a8a4de84u64, 0x00000000118427b3u64, 0x0000000000000000u64] },	// 10^47
    BID_UINT256 { w: [0x7f41000000000000u64, 0x0e4395d69670b12bu64, 0x00000000af298d05u64, 0x0000000000000000u64] },	// 10^48
    BID_UINT256 { w: [0xf88a000000000000u64, 0x8ea3da61e066ebb2u64, 0x00000006d79f8232u64, 0x0000000000000000u64] },	// 10^49
    BID_UINT256 { w: [0xb564000000000000u64, 0x926687d2c40534fdu64, 0x000000446c3b15f9u64, 0x0000000000000000u64] },	// 10^50
    BID_UINT256 { w: [0x15e8000000000000u64, 0xb8014e3ba83411e9u64, 0x000002ac3a4edbbfu64, 0x0000000000000000u64] },	// 10^51
    BID_UINT256 { w: [0xdb10000000000000u64, 0x300d0e549208b31au64, 0x00001aba4714957du64, 0x0000000000000000u64] },	// 10^52
    BID_UINT256 { w: [0x8ea0000000000000u64, 0xe0828f4db456ff0cu64, 0x00010b46c6cdd6e3u64, 0x0000000000000000u64] },	// 10^53
    BID_UINT256 { w: [0x9240000000000000u64, 0xc51999090b65f67du64, 0x000a70c3c40a64e6u64, 0x0000000000000000u64] },	// 10^54
    BID_UINT256 { w: [0xb680000000000000u64, 0xb2fffa5a71fba0e7u64, 0x006867a5a867f103u64, 0x0000000000000000u64] },	// 10^55
    BID_UINT256 { w: [0x2100000000000000u64, 0xfdffc78873d4490du64, 0x04140c78940f6a24u64, 0x0000000000000000u64] },	// 10^56
    BID_UINT256 { w: [0x4a00000000000000u64, 0xebfdcb54864ada83u64, 0x28c87cb5c89a2571u64, 0x0000000000000000u64] },	// 10^57 (58 digits)
    BID_UINT256 { w: [0xe400000000000000u64, 0x37e9f14d3eec8920u64, 0x97d4df19d6057673u64, 0x0000000000000001u64] },	// 10^58
    BID_UINT256 { w: [0xe800000000000000u64, 0x2f236d04753d5b48u64, 0xee50b7025c36a080u64, 0x000000000000000fu64] },	// 10^59
    BID_UINT256 { w: [0x1000000000000000u64, 0xd762422c946590d9u64, 0x4f2726179a224501u64, 0x000000000000009fu64] },	// 10^60
    BID_UINT256 { w: [0xa000000000000000u64, 0x69d695bdcbf7a87au64, 0x17877cec0556b212u64, 0x0000000000000639u64] },	// 10^61
    BID_UINT256 { w: [0x4000000000000000u64, 0x2261d969f7ac94cau64, 0xeb4ae1383562f4b8u64, 0x0000000000003e3au64] },	// 10^62
    BID_UINT256 { w: [0x8000000000000000u64, 0x57d27e23acbdcfe6u64, 0x30eccc3215dd8f31u64, 0x0000000000026e4du64] },	// 10^63
    BID_UINT256 { w: [0x0000000000000000u64, 0x6e38ed64bf6a1f01u64, 0xe93ff9f4daa797edu64, 0x0000000000184f03u64] },	// 10^64
    BID_UINT256 { w: [0x0000000000000000u64, 0x4e3945ef7a25360au64, 0x1c7fc3908a8bef46u64, 0x0000000000f31627u64] },	// 10^65
    BID_UINT256 { w: [0x0000000000000000u64, 0x0e3cbb5ac5741c64u64, 0x1cfda3a5697758bfu64, 0x00000000097edd87u64] },	// 10^66
    BID_UINT256 { w: [0x0000000000000000u64, 0x8e5f518bb6891be8u64, 0x21e864761ea97776u64, 0x000000005ef4a747u64] },	// 10^67
    BID_UINT256 { w: [0x0000000000000000u64, 0x8fb92f75215b1710u64, 0x5313ec9d329eaaa1u64, 0x00000003b58e88c7u64] },	// 10^68
    BID_UINT256 { w: [0x0000000000000000u64, 0x9d3bda934d8ee6a0u64, 0x3ec73e23fa32aa4fu64, 0x00000025179157c9u64] },	// 10^69
    BID_UINT256 { w: [0x0000000000000000u64, 0x245689c107950240u64, 0x73c86d67c5faa71cu64, 0x00000172ebad6ddcu64] },	// 10^70
    BID_UINT256 { w: [0x0000000000000000u64, 0x6b61618a4bd21680u64, 0x85d4460dbbca8719u64, 0x00000e7d34c64a9cu64] },	// 10^71
    BID_UINT256 { w: [0x0000000000000000u64, 0x31cdcf66f634e100u64, 0x3a4abc8955e946feu64, 0x000090e40fbeea1du64] },	// 10^72
    BID_UINT256 { w: [0x0000000000000000u64, 0xf20a1a059e10ca00u64, 0x46eb5d5d5b1cc5edu64, 0x0005a8e89d752524u64] },	// 10^73
    BID_UINT256 { w: [0x0000000000000000u64, 0x746504382ca7e400u64, 0xc531a5a58f1fbb4bu64, 0x003899162693736au64] },	// 10^74
    BID_UINT256 { w: [0x0000000000000000u64, 0x8bf22a31be8ee800u64, 0xb3f07877973d50f2u64, 0x0235fadd81c2822bu64] },	// 10^75
    BID_UINT256 { w: [0x0000000000000000u64, 0x7775a5f171951000u64, 0x0764b4abe8652979u64, 0x161bcca7119915b5u64] },	// 10^76
    BID_UINT256 { w: [0x0000000000000000u64, 0xaa987b6e6fd2a000u64, 0x49ef0eb713f39ebeu64, 0xdd15fe86affad912u64] }	// 10^77
];

/// bid_ten2mk128[k - 1] = 10^(-k) * 2^exp (k), where 1 <= k <= 34 and
/// exp (k) = bid_shiftright128[k - 1] + 128
pub (crate) const bid_ten2mk128: [BID_UINT128; 34] = [
    BID_UINT128 { w: [0x999999999999999au64, 0x1999999999999999u64] },  //  10^(-1) * 2^128
    BID_UINT128 { w: [0x28f5c28f5c28f5c3u64, 0x028f5c28f5c28f5cu64] },  //  10^(-2) * 2^128
    BID_UINT128 { w: [0x9db22d0e56041894u64, 0x004189374bc6a7efu64] },  //  10^(-3) * 2^128
    BID_UINT128 { w: [0x4af4f0d844d013aau64, 0x00346dc5d6388659u64] },  //  10^(-4) * 2^131
    BID_UINT128 { w: [0x08c3f3e0370cdc88u64, 0x0029f16b11c6d1e1u64] },  //  10^(-5) * 2^134
    BID_UINT128 { w: [0x6d698fe69270b06du64, 0x00218def416bdb1au64] },  //  10^(-6) * 2^137
    BID_UINT128 { w: [0xaf0f4ca41d811a47u64, 0x0035afe535795e90u64] },  //  10^(-7) * 2^141
    BID_UINT128 { w: [0xbf3f70834acdaea0u64, 0x002af31dc4611873u64] },  //  10^(-8) * 2^144
    BID_UINT128 { w: [0x65cc5a02a23e254du64, 0x00225c17d04dad29u64] },  //  10^(-9) * 2^147
    BID_UINT128 { w: [0x6fad5cd10396a214u64, 0x0036f9bfb3af7b75u64] },  // 10^(-10) * 2^151
    BID_UINT128 { w: [0xbfbde3da69454e76u64, 0x002bfaffc2f2c92au64] },  // 10^(-11) * 2^154
    BID_UINT128 { w: [0x32fe4fe1edd10b92u64, 0x00232f33025bd422u64] },  // 10^(-12) * 2^157
    BID_UINT128 { w: [0x84ca19697c81ac1cu64, 0x00384b84d092ed03u64] },  // 10^(-13) * 2^161
    BID_UINT128 { w: [0x03d4e1213067bce4u64, 0x002d09370d425736u64] },  // 10^(-14) * 2^164
    BID_UINT128 { w: [0x3643e74dc052fd83u64, 0x0024075f3dceac2bu64] },  // 10^(-15) * 2^167
    BID_UINT128 { w: [0x56d30baf9a1e626bu64, 0x0039a5652fb11378u64] },  // 10^(-16) * 2^171
    BID_UINT128 { w: [0x12426fbfae7eb522u64, 0x002e1dea8c8da92du64] },  // 10^(-17) * 2^174
    BID_UINT128 { w: [0x41cebfcc8b9890e8u64, 0x0024e4bba3a48757u64] },  // 10^(-18) * 2^177
    BID_UINT128 { w: [0x694acc7a78f41b0du64, 0x003b07929f6da558u64] },  // 10^(-19) * 2^181
    BID_UINT128 { w: [0xbaa23d2ec729af3eu64, 0x002f394219248446u64] },  // 10^(-20) * 2^184
    BID_UINT128 { w: [0xfbb4fdbf05baf298u64, 0x0025c768141d369eu64] },  // 10^(-21) * 2^187
    BID_UINT128 { w: [0x2c54c931a2c4b759u64, 0x003c7240202ebdcbu64] },  // 10^(-22) * 2^191
    BID_UINT128 { w: [0x89dd6dc14f03c5e1u64, 0x00305b66802564a2u64] },  // 10^(-23) * 2^194
    BID_UINT128 { w: [0xd4b1249aa59c9e4eu64, 0x0026af8533511d4eu64] },  // 10^(-24) * 2^197
    BID_UINT128 { w: [0x544ea0f76f60fd49u64, 0x003de5a1ebb4fbb1u64] },  // 10^(-25) * 2^201
    BID_UINT128 { w: [0x76a54d92bf80caa1u64, 0x00318481895d9627u64] },  // 10^(-26) * 2^204
    BID_UINT128 { w: [0x921dd7a89933d54eu64, 0x00279d346de4781fu64] },  // 10^(-27) * 2^207
    BID_UINT128 { w: [0x8362f2a75b862215u64, 0x003f61ed7ca0c032u64] },  // 10^(-28) * 2^211
    BID_UINT128 { w: [0xcf825bb91604e811u64, 0x0032b4bdfd4d668eu64] },  // 10^(-29) * 2^214
    BID_UINT128 { w: [0x0c684960de6a5341u64, 0x00289097fdd7853fu64] },  // 10^(-30) * 2^217
    BID_UINT128 { w: [0x3d203ab3e521dc34u64, 0x002073accb12d0ffu64] },  // 10^(-31) * 2^220
    BID_UINT128 { w: [0x2e99f7863b696053u64, 0x0033ec47ab514e65u64] },  // 10^(-32) * 2^224
    BID_UINT128 { w: [0x587b2c6b62bab376u64, 0x002989d2ef743eb7u64] },  // 10^(-33) * 2^227
    BID_UINT128 { w: [0xad2f56bc4efbc2c5u64, 0x00213b0f25f69892u64] },  // 10^(-34) * 2^230
];

/// bid_shiftright128[] contains the right shift count to obtain C2* from the top
/// 128 bits of the 128x128-bit product C2 * Kx
pub (crate) const bid_shiftright128: [i32; 34] = [
    0,   // 128 - 128
    0,   // 128 - 128
    0,   // 128 - 128

    3,   // 131 - 128
    6,   // 134 - 128
    9,   // 137 - 128
    13,  // 141 - 128
    16,  // 144 - 128
    19,  // 147 - 128
    23,  // 151 - 128
    26,  // 154 - 128
    29,  // 157 - 128
    33,  // 161 - 128
    36,  // 164 - 128
    39,  // 167 - 128
    43,  // 171 - 128
    46,  // 174 - 128
    49,  // 177 - 128
    53,  // 181 - 128
    56,  // 184 - 128
    59,  // 187 - 128
    63,  // 191 - 128

    66,  // 194 - 128
    69,  // 197 - 128
    73,  // 201 - 128
    76,  // 204 - 128
    79,  // 207 - 128
    83,  // 211 - 128
    86,  // 214 - 128
    89,  // 217 - 128
    92,  // 220 - 128
    96,  // 224 - 128
    99,  // 227 - 128
    102  // 230 - 128
];

/// bid_maskhigh128[] contains the mask to apply to the top 128 bits of the
/// 128x128-bit product in order to obtain the high bits of f2* the 64-bit word order is L, H
pub (crate) const bid_maskhigh128: [BID_UINT64; 34] = [
    0x0000000000000000u64,  //  0 = 128 - 128 bits
    0x0000000000000000u64,  //  0 = 128 - 128 bits
    0x0000000000000000u64,  //  0 = 128 - 128 bits
    0x0000000000000007u64,  //  3 = 131 - 128 bits
    0x000000000000003fu64,  //  6 = 134 - 128 bits
    0x00000000000001ffu64,  //  9 = 137 - 128 bits
    0x0000000000001fffu64,  // 13 = 141 - 128 bits
    0x000000000000ffffu64,  // 16 = 144 - 128 bits
    0x000000000007ffffu64,  // 19 = 147 - 128 bits
    0x00000000007fffffu64,  // 23 = 151 - 128 bits
    0x0000000003ffffffu64,  // 26 = 154 - 128 bits
    0x000000001fffffffu64,  // 29 = 157 - 128 bits
    0x00000001ffffffffu64,  // 33 = 161 - 128 bits
    0x0000000fffffffffu64,  // 36 = 164 - 128 bits
    0x0000007fffffffffu64,  // 39 = 167 - 128 bits
    0x000007ffffffffffu64,  // 43 = 171 - 128 bits
    0x00003fffffffffffu64,  // 46 = 174 - 128 bits
    0x0001ffffffffffffu64,  // 49 = 177 - 128 bits
    0x001fffffffffffffu64,  // 53 = 181 - 128 bits
    0x00ffffffffffffffu64,  // 56 = 184 - 128 bits
    0x07ffffffffffffffu64,  // 59 = 187 - 128 bits
    0x7fffffffffffffffu64,  // 63 = 191 - 128 bits
    0x0000000000000003u64,  //  2 = 194 - 192 bits
    0x000000000000001fu64,  //  5 = 197 - 192 bits
    0x00000000000001ffu64,  //  9 = 201 - 192 bits
    0x0000000000000fffu64,  // 12 = 204 - 192 bits
    0x0000000000007fffu64,  // 15 = 207 - 192 bits
    0x000000000007ffffu64,  // 21 = 211 - 192 bits
    0x00000000003fffffu64,  // 22 = 214 - 192 bits
    0x0000000001ffffffu64,  // 25 = 217 - 192 bits
    0x000000000fffffffu64,  // 28 = 220 - 192 bits
    0x00000000ffffffffu64,  // 32 = 224 - 192 bits
    0x00000007ffffffffu64,  // 35 = 227 - 192 bits
    0x0000003fffffffffu64   // 38 = 230 - 192 bits
];

/// bid_onehalf128[] contains the high bits of 1/2 positioned correctly for
/// comparison with the high bits of f2* the 64-bit word order is L, H
pub (crate) const bid_onehalf128: [BID_UINT64; 34] = [
    0x0000000000000000u64,  //  0 bits
    0x0000000000000000u64,  //  0 bits
    0x0000000000000000u64,  //  0 bits
    0x0000000000000004u64,  //  3 bits
    0x0000000000000020u64,  //  6 bits
    0x0000000000000100u64,  //  9 bits
    0x0000000000001000u64,  // 13 bits
    0x0000000000008000u64,  // 16 bits
    0x0000000000040000u64,  // 19 bits
    0x0000000000400000u64,  // 23 bits
    0x0000000002000000u64,  // 26 bits
    0x0000000010000000u64,  // 29 bits
    0x0000000100000000u64,  // 33 bits
    0x0000000800000000u64,  // 36 bits
    0x0000004000000000u64,  // 39 bits
    0x0000040000000000u64,  // 43 bits
    0x0000200000000000u64,  // 46 bits
    0x0001000000000000u64,  // 49 bits
    0x0010000000000000u64,  // 53 bits
    0x0080000000000000u64,  // 56 bits
    0x0400000000000000u64,  // 59 bits
    0x4000000000000000u64,  // 63 bits
    0x0000000000000002u64,  // 66 bits
    0x0000000000000010u64,  // 69 bits
    0x0000000000000100u64,  // 73 bits
    0x0000000000000800u64,  // 76 bits
    0x0000000000004000u64,  // 79 bits
    0x0000000000040000u64,  // 83 bits
    0x0000000000200000u64,  // 86 bits
    0x0000000001000000u64,  // 89 bits
    0x0000000008000000u64,  // 92 bits
    0x0000000080000000u64,  // 96 bits
    0x0000000400000000u64,  // 99 bits
    0x0000002000000000u64   // 102 bits
];

pub (crate) const bid_ten2mk64: [BID_UINT64; 16] = [
    0x199999999999999au64,  //  10^(-1) * 2^ 64
    0x028f5c28f5c28f5du64,  //  10^(-2) * 2^ 64
    0x004189374bc6a7f0u64,  //  10^(-3) * 2^ 64
    0x00346dc5d638865au64,  //  10^(-4) * 2^ 67
    0x0029f16b11c6d1e2u64,  //  10^(-5) * 2^ 70
    0x00218def416bdb1bu64,  //  10^(-6) * 2^ 73
    0x0035afe535795e91u64,  //  10^(-7) * 2^ 77
    0x002af31dc4611874u64,  //  10^(-8) * 2^ 80
    0x00225c17d04dad2au64,  //  10^(-9) * 2^ 83
    0x0036f9bfb3af7b76u64,  // 10^(-10) * 2^ 87
    0x002bfaffc2f2c92bu64,  // 10^(-11) * 2^ 90
    0x00232f33025bd423u64,  // 10^(-12) * 2^ 93
    0x00384b84d092ed04u64,  // 10^(-13) * 2^ 97
    0x002d09370d425737u64,  // 10^(-14) * 2^100
    0x0024075f3dceac2cu64,  // 10^(-15) * 2^103
    0x0039a5652fb11379u64,  // 10^(-16) * 2^107
];

/// bid_ten2mk128trunc[] contains T*, the top Ex >= 128 bits of 10^(-k),
/// for 1 <= k <= 34 the 64-bit word order is L, H
pub (crate) const bid_ten2mk128trunc: [BID_UINT128; 34] = [
    BID_UINT128 { w: [0x9999999999999999u64, 0x1999999999999999u64] },  //  10^(-1) * 2^128
    BID_UINT128 { w: [0x28f5c28f5c28f5c2u64, 0x028f5c28f5c28f5cu64] },  //  10^(-2) * 2^128
    BID_UINT128 { w: [0x9db22d0e56041893u64, 0x004189374bc6a7efu64] },  //  10^(-3) * 2^128
    BID_UINT128 { w: [0x4af4f0d844d013a9u64, 0x00346dc5d6388659u64] },  //  10^(-4) * 2^131
    BID_UINT128 { w: [0x08c3f3e0370cdc87u64, 0x0029f16b11c6d1e1u64] },  //  10^(-5) * 2^134
    BID_UINT128 { w: [0x6d698fe69270b06cu64, 0x00218def416bdb1au64] },  //  10^(-6) * 2^137
    BID_UINT128 { w: [0xaf0f4ca41d811a46u64, 0x0035afe535795e90u64] },  //  10^(-7) * 2^141
    BID_UINT128 { w: [0xbf3f70834acdae9fu64, 0x002af31dc4611873u64] },  //  10^(-8) * 2^144
    BID_UINT128 { w: [0x65cc5a02a23e254cu64, 0x00225c17d04dad29u64] },  //  10^(-9) * 2^147
    BID_UINT128 { w: [0x6fad5cd10396a213u64, 0x0036f9bfb3af7b75u64] },  // 10^(-10) * 2^151
    BID_UINT128 { w: [0xbfbde3da69454e75u64, 0x002bfaffc2f2c92au64] },  // 10^(-11) * 2^154
    BID_UINT128 { w: [0x32fe4fe1edd10b91u64, 0x00232f33025bd422u64] },  // 10^(-12) * 2^157
    BID_UINT128 { w: [0x84ca19697c81ac1bu64, 0x00384b84d092ed03u64] },  // 10^(-13) * 2^161
    BID_UINT128 { w: [0x03d4e1213067bce3u64, 0x002d09370d425736u64] },  // 10^(-14) * 2^164
    BID_UINT128 { w: [0x3643e74dc052fd82u64, 0x0024075f3dceac2bu64] },  // 10^(-15) * 2^167
    BID_UINT128 { w: [0x56d30baf9a1e626au64, 0x0039a5652fb11378u64] },  // 10^(-16) * 2^171
    BID_UINT128 { w: [0x12426fbfae7eb521u64, 0x002e1dea8c8da92du64] },  // 10^(-17) * 2^174
    BID_UINT128 { w: [0x41cebfcc8b9890e7u64, 0x0024e4bba3a48757u64] },  // 10^(-18) * 2^177
    BID_UINT128 { w: [0x694acc7a78f41b0cu64, 0x003b07929f6da558u64] },  // 10^(-19) * 2^181
    BID_UINT128 { w: [0xbaa23d2ec729af3du64, 0x002f394219248446u64] },  // 10^(-20) * 2^184
    BID_UINT128 { w: [0xfbb4fdbf05baf297u64, 0x0025c768141d369eu64] },  // 10^(-21) * 2^187
    BID_UINT128 { w: [0x2c54c931a2c4b758u64, 0x003c7240202ebdcbu64] },  // 10^(-22) * 2^191
    BID_UINT128 { w: [0x89dd6dc14f03c5e0u64, 0x00305b66802564a2u64] },  // 10^(-23) * 2^194
    BID_UINT128 { w: [0xd4b1249aa59c9e4du64, 0x0026af8533511d4eu64] },  // 10^(-24) * 2^197
    BID_UINT128 { w: [0x544ea0f76f60fd48u64, 0x003de5a1ebb4fbb1u64] },  // 10^(-25) * 2^201
    BID_UINT128 { w: [0x76a54d92bf80caa0u64, 0x00318481895d9627u64] },  // 10^(-26) * 2^204
    BID_UINT128 { w: [0x921dd7a89933d54du64, 0x00279d346de4781fu64] },  // 10^(-27) * 2^207
    BID_UINT128 { w: [0x8362f2a75b862214u64, 0x003f61ed7ca0c032u64] },  // 10^(-28) * 2^211
    BID_UINT128 { w: [0xcf825bb91604e810u64, 0x0032b4bdfd4d668eu64] },  // 10^(-29) * 2^214
    BID_UINT128 { w: [0x0c684960de6a5340u64, 0x00289097fdd7853fu64] },  // 10^(-30) * 2^217
    BID_UINT128 { w: [0x3d203ab3e521dc33u64, 0x002073accb12d0ffu64] },  // 10^(-31) * 2^220
    BID_UINT128 { w: [0x2e99f7863b696052u64, 0x0033ec47ab514e65u64] },  // 10^(-32) * 2^224
    BID_UINT128 { w: [0x587b2c6b62bab375u64, 0x002989d2ef743eb7u64] },  // 10^(-33) * 2^227
    BID_UINT128 { w: [0xad2f56bc4efbc2c4u64, 0x00213b0f25f69892u64] },  // 10^(-34) * 2^230
];

/// bid_ten2mk128M[k - 1] = 10^(-k) * 2^exp (k), where 1 <= k <= 4 and
/// exp (k) = bid_shiftright128[k - 1] + 128
/// the 64-bit word order is L, H
pub (crate) const bid_ten2mk128M: [BID_UINT128; 4] = [
    BID_UINT128 { w: [0xcccccccccccccccdu64, 0xccccccccccccccccu64 ] },  //  10^(-1) * 2^131
    BID_UINT128 { w: [0x3d70a3d70a3d70a4u64, 0xa3d70a3d70a3d70au64 ] },  //  10^(-2) * 2^134
    BID_UINT128 { w: [0x645a1cac083126eau64, 0x83126e978d4fdf3bu64 ] },  //  10^(-3) * 2^137
    BID_UINT128 { w: [0xd3c36113404ea4a9u64, 0xd1b71758e219652bu64 ] }   //  10^(-4) * 2^141
];

/// bid_ten2mk128truncM[] contains T*, the top Ex >= 128 bits of 10^(-k),
/// for 1 <= k <= 4; the top bits which are 0 are not represented
/// the 64-bit word order is L, H
pub (crate) const bid_ten2mk128truncM: [BID_UINT128; 4] = [
    BID_UINT128 { w: [0xccccccccccccccccu64, 0xccccccccccccccccu64] },  //  10^(-1) * 2^131
    BID_UINT128 { w: [0x3d70a3d70a3d70a3u64, 0xa3d70a3d70a3d70au64] },  //  10^(-2) * 2^134
    BID_UINT128 { w: [0x645a1cac083126e9u64, 0x83126e978d4fdf3bu64] },  //  10^(-3) * 2^137
    BID_UINT128 { w: [0xd3c36113404ea4a8u64, 0xd1b71758e219652bu64] }   //  10^(-4) * 2^141
];

/// bid_shiftright128M[] contains the right shift count to obtain C2* from the top
/// 128 bits of the 128x128-bit product C2 * Kx
pub (crate) const bid_shiftright128M: [i32; 4] = [
    3,  // 131 - 128
    6,  // 134 - 128
    9,  // 137 - 128
    13  // 141 - 128
];

/// bid_maskhigh128M[] contains the mask to apply to the top 128 bits of the
/// 128x128-bit product in order to obtain the high bits of f*
/// the high 64 bits of the mask are 0, so only the low 64 bits are represented
pub (crate) const bid_maskhigh128M: [BID_UINT64; 4] = [
    0x0000000000000007u64,  //  3 = 131 - 128 bits
    0x000000000000003fu64,  //  6 = 134 - 128 bits
    0x00000000000001ffu64,  //  9 = 137 - 128 bits
    0x0000000000001fffu64   // 13 = 141 - 128 bits
];

/// bid_onehalf128M[] contains 1/2 positioned correctly for
/// comparison with the high bits of f*
/// the high 64 bits are 0, so only the low 64 bits are represented
pub (crate) const bid_onehalf128M: [BID_UINT64; 4] = [
    0x0000000000000004u64,  //  3 bits
    0x0000000000000020u64,  //  6 bits
    0x0000000000000100u64,  //  9 bits
    0x0000000000001000u64   // 13 bits
];

/// bid_ten2mk192M[k - 1] = 10^(-k-4) * 2^exp (k), where 1 <= k <= 19 and
/// exp (k) = bid_shiftright128[k - 1] + 128
/// the 64-bit word order is L, M, H
pub (crate) const bid_ten2mk192M: [BID_UINT192; 19] = [
    BID_UINT192 { w: [0xcddd6e04c0592104u64, 0x0fcf80dc33721d53u64, 0xa7c5ac471b478423u64] }, //  10^(-5) * 2^208
    BID_UINT192 { w: [0xd7e45803cd141a6au64, 0xa63f9a49c2c1b10fu64, 0x8637bd05af6c69b5u64] }, //  10^(-6) * 2^211
    BID_UINT192 { w: [0x8ca08cd2e1b9c3dcu64, 0x3d32907604691b4cu64, 0xd6bf94d5e57a42bcu64] }, //  10^(-7) * 2^215
    BID_UINT192 { w: [0x3d4d3d758161697du64, 0xfdc20d2b36ba7c3du64, 0xabcc77118461cefcu64] }, //  10^(-8) * 2^218
    BID_UINT192 { w: [0xfdd7645e011abacau64, 0x31680a88f8953030u64, 0x89705f4136b4a597u64] }, //  10^(-9) * 2^221
    BID_UINT192 { w: [0x2fbf06fcce912addu64, 0xb573440e5a884d1bu64, 0xdbe6fecebdedd5beu64] }, //  10^(-10) * 2^225
    BID_UINT192 { w: [0xf2ff38ca3eda88b1u64, 0xf78f69a51539d748u64, 0xafebff0bcb24aafeu64] }, //  10^(-11) * 2^228
    BID_UINT192 { w: [0xf598fa3b657ba08eu64, 0xf93f87b7442e45d3u64, 0x8cbccc096f5088cbu64] }, //  10^(-12) * 2^231
    BID_UINT192 { w: [0x88f4c3923bf900e3u64, 0x2865a5f206b06fb9u64, 0xe12e13424bb40e13u64] }, //  10^(-13) * 2^235
    BID_UINT192 { w: [0x6d909c74fcc733e9u64, 0x538484c19ef38c94u64, 0xb424dc35095cd80fu64] }, //  10^(-14) * 2^238
    BID_UINT192 { w: [0x57a6e390ca38f654u64, 0x0f9d37014bf60a10u64, 0x901d7cf73ab0acd9u64] }, //  10^(-15) * 2^241
    BID_UINT192 { w: [0xbf716c1add27f086u64, 0x4c2ebe687989a9b3u64, 0xe69594bec44de15bu64] }, //  10^(-16) * 2^245
    BID_UINT192 { w: [0xff8df0157db98d38u64, 0x09befeb9fad487c2u64, 0xb877aa3236a4b449u64] }, //  10^(-17) * 2^248
    BID_UINT192 { w: [0x32d7f344649470fau64, 0x3aff322e62439fcfu64, 0x9392ee8e921d5d07u64] }, //  10^(-18) * 2^251
    BID_UINT192 { w: [0x1e2652070753e7f5u64, 0x2b31e9e3d06c32e5u64, 0xec1e4a7db69561a5u64] }, //  10^(-19) * 2^255
    BID_UINT192 { w: [0x181ea8059f76532bu64, 0x88f4bb1ca6bcf584u64, 0xbce5086492111aeau64] }, //  10^(-20) * 2^258
    BID_UINT192 { w: [0x467eecd14c5ea8efu64, 0xd3f6fc16ebca5e03u64, 0x971da05074da7beeu64] }, //  10^(-21) * 2^261
    BID_UINT192 { w: [0x70cb148213caa7e5u64, 0x5324c68b12dd6338u64, 0xf1c90080baf72cb1u64] }, //  10^(-22) * 2^265
    BID_UINT192 { w: [0x8d6f439b43088651u64, 0x75b7053c0f178293u64, 0xc16d9a0095928a27u64] }  //  10^(-23) * 2^268
];

/// bid_ten2mk192truncM[] contains T*, the top Ex >= 192 bits of 10^(-k),
/// for 5 <= k <= 23; the top bits which are 0 are not represented the 64-bit word order is L, M, H
pub (crate) const bid_ten2mk192truncM: [BID_UINT192; 19] = [
    BID_UINT192 { w: [0xcddd6e04c0592103u64, 0x0fcf80dc33721d53u64, 0xa7c5ac471b478423u64] }, //  10^(-5) * 2^208
    BID_UINT192 { w: [0xd7e45803cd141a69u64, 0xa63f9a49c2c1b10fu64, 0x8637bd05af6c69b5u64] }, //  10^(-6) * 2^211
    BID_UINT192 { w: [0x8ca08cd2e1b9c3dbu64, 0x3d32907604691b4cu64, 0xd6bf94d5e57a42bcu64] }, //  10^(-7) * 2^215
    BID_UINT192 { w: [0x3d4d3d758161697cu64, 0xfdc20d2b36ba7c3du64, 0xabcc77118461cefcu64] }, //  10^(-8) * 2^218
    BID_UINT192 { w: [0xfdd7645e011abac9u64, 0x31680a88f8953030u64, 0x89705f4136b4a597u64] }, //  10^(-9) * 2^221
    BID_UINT192 { w: [0x2fbf06fcce912adcu64, 0xb573440e5a884d1bu64, 0xdbe6fecebdedd5beu64] }, //  10^(-10) * 2^225
    BID_UINT192 { w: [0xf2ff38ca3eda88b0u64, 0xf78f69a51539d748u64, 0xafebff0bcb24aafeu64] }, //  10^(-11) * 2^228
    BID_UINT192 { w: [0xf598fa3b657ba08du64, 0xf93f87b7442e45d3u64, 0x8cbccc096f5088cbu64] }, //  10^(-12) * 2^231
    BID_UINT192 { w: [0x88f4c3923bf900e2u64, 0x2865a5f206b06fb9u64, 0xe12e13424bb40e13u64] }, //  10^(-13) * 2^235
    BID_UINT192 { w: [0x6d909c74fcc733e8u64, 0x538484c19ef38c94u64, 0xb424dc35095cd80fu64] }, //  10^(-14) * 2^238
    BID_UINT192 { w: [0x57a6e390ca38f653u64, 0x0f9d37014bf60a10u64, 0x901d7cf73ab0acd9u64] }, //  10^(-15) * 2^241
    BID_UINT192 { w: [0xbf716c1add27f085u64, 0x4c2ebe687989a9b3u64, 0xe69594bec44de15bu64] }, //  10^(-16) * 2^245
    BID_UINT192 { w: [0xff8df0157db98d37u64, 0x09befeb9fad487c2u64, 0xb877aa3236a4b449u64] }, //  10^(-17) * 2^248
    BID_UINT192 { w: [0x32d7f344649470f9u64, 0x3aff322e62439fcfu64, 0x9392ee8e921d5d07u64] }, //  10^(-18) * 2^251
    BID_UINT192 { w: [0x1e2652070753e7f4u64, 0x2b31e9e3d06c32e5u64, 0xec1e4a7db69561a5u64] }, //  10^(-19) * 2^255
    BID_UINT192 { w: [0x181ea8059f76532au64, 0x88f4bb1ca6bcf584u64, 0xbce5086492111aeau64] }, //  10^(-20) * 2^258
    BID_UINT192 { w: [0x467eecd14c5ea8eeu64, 0xd3f6fc16ebca5e03u64, 0x971da05074da7beeu64] }, //  10^(-21) * 2^261
    BID_UINT192 { w: [0x70cb148213caa7e4u64, 0x5324c68b12dd6338u64, 0xf1c90080baf72cb1u64] }, //  10^(-22) * 2^265
    BID_UINT192 { w: [0x8d6f439b43088650u64, 0x75b7053c0f178293u64, 0xc16d9a0095928a27u64] }  //  10^(-23) * 2^268
];

/// bid_shiftright192M[] contains the right shift count to obtain C2* from the top
/// 192 bits of the 192x192-bit product C2 * Kx if 0 <= ind <= 14 where ind is
/// the index in the table, or from the top 128 bits if 15 <= ind <= 18
pub (crate) const bid_shiftright192M: [i32; 19] = [
    16,  // 208 - 192
    19,  // 211 - 192
    23,  // 215 - 192
    26,  // 218 - 192
    29,  // 221 - 192
    33,  // 225 - 192
    36,  // 228 - 192
    39,  // 231 - 192
    43,  // 235 - 192
    46,  // 238 - 192
    49,  // 241 - 192
    53,  // 245 - 192
    56,  // 248 - 192
    59,  // 251 - 192
    63,  // 255 - 192
    2,   // 258 - 256
    5,   // 261 - 256
    9,   // 265 - 256
    12   // 268 - 256
];

/// bid_maskhigh192M[] contains the mask to apply to the top 192 bits of the
/// 192x192-bit product in order to obtain the high bits of f*
/// if 0 <= ind <= 14 where ind is the index in the table, then the high 128 bits
/// of the 384-bit mask are 0;  if 15 <= ind <= 18 then the high 64 bits are 0
pub (crate) const bid_maskhigh192M: [BID_UINT64; 19] = [
    0x000000000000ffffu64,  //  16 = 208 - 192 bits
    0x000000000007ffffu64,  //  19 = 211 - 192 bits
    0x00000000007fffffu64,  //  23 = 215 - 192 bits
    0x0000000003ffffffu64,  //  26 = 218 - 192 bits
    0x000000001fffffffu64,  //  29 = 221 - 192 bits
    0x00000001ffffffffu64,  //  33 = 225 - 192 bits
    0x0000000fffffffffu64,  //  36 = 228 - 192 bits
    0x0000007fffffffffu64,  //  39 = 231 - 192 bits
    0x000007ffffffffffu64,  //  43 = 235 - 192 bits
    0x00003fffffffffffu64,  //  46 = 238 - 192 bits
    0x0001ffffffffffffu64,  //  49 = 241 - 192 bits
    0x001fffffffffffffu64,  //  53 = 245 - 192 bits
    0x00ffffffffffffffu64,  //  56 = 248 - 192 bits
    0x07ffffffffffffffu64,  //  59 = 251 - 192 bits
    0x7fffffffffffffffu64,  //  63 = 255 - 192 bits
    0x0000000000000003u64,  //   2 = 258 - 256 bits
    0x000000000000001fu64,  //   5 = 261 - 256 bits
    0x00000000000001ffu64,  //   9 = 265 - 256 bits
    0x0000000000000fffu64   //  12 = 268 - 256 bits
];

/// bid_onehalf192M[] contains 1/2 positioned correctly for
/// comparison with the high bits of f*
/// if 0 <= ind <= 14 where ind is the index in the table, then the high 128 bits
/// of the 384-bit mask are 0;  if 15 <= ind <= 18 then the high 648 bits are 0
pub (crate) const bid_onehalf192M: [BID_UINT64; 19] = [
    0x0000000000008000u64,  //  16 = 208 - 192 bits
    0x0000000000040000u64,  //  19 = 211 - 192 bits
    0x0000000000400000u64,  //  23 = 215 - 192 bits
    0x0000000002000000u64,  //  26 = 218 - 192 bits
    0x0000000010000000u64,  //  29 = 221 - 192 bits
    0x0000000100000000u64,  //  33 = 225 - 192 bits
    0x0000000800000000u64,  //  36 = 228 - 192 bits
    0x0000004000000000u64,  //  39 = 231 - 192 bits
    0x0000040000000000u64,  //  43 = 235 - 192 bits
    0x0000200000000000u64,  //  46 = 238 - 192 bits
    0x0001000000000000u64,  //  49 = 241 - 192 bits
    0x0010000000000000u64,  //  53 = 245 - 192 bits
    0x0080000000000000u64,  //  56 = 248 - 192 bits
    0x0400000000000000u64,  //  59 = 251 - 192 bits
    0x4000000000000000u64,  //  63 = 255 - 192 bits
    0x0000000000000002u64,  //   2 = 258 - 256 bits
    0x0000000000000010u64,  //   5 = 261 - 256 bits
    0x0000000000000100u64,  //   9 = 265 - 256 bits
    0x0000000000000800u64   //  12 = 268 - 256 bits
];

/// bid_ten2mk256M[k - 1] = 10^(-k-23) * 2^exp (k), where 1 <= k <= 11 and
/// exp (k) = bid_shiftright128[k - 1] + 128
pub (crate) const bid_ten2mk256M: [BID_UINT256; 11] = [
    // the 64-bit word order is LL, LH, HL, HH
    BID_UINT256 { w: [0xf23472530ce6e3edu64, 0xd78c3615cf3a050cu64, 0xc4926a9672793542u64, 0x9abe14cd44753b52u64] },  //  10^(-24) * 2^335
    BID_UINT256 { w: [0xe9ed83b814a49fe1u64, 0x8c1389bc7ec33b47u64, 0x3a83ddbd83f52204u64, 0xf79687aed3eec551u64] },  //  10^(-25) * 2^339
    BID_UINT256 { w: [0x87f1362cdd507fe7u64, 0x3cdc6e306568fc39u64, 0x95364afe032a819du64, 0xc612062576589ddau64] },  //  10^(-26) * 2^342
    BID_UINT256 { w: [0x9ff42b5717739986u64, 0xca49f1c05120c9c7u64, 0x775ea264cf55347du64, 0x9e74d1b791e07e48u64] },  //  10^(-27) * 2^345
    BID_UINT256 { w: [0xccb9def1bf1f5c09u64, 0x76dcb60081ce0fa5u64, 0x8bca9d6e188853fcu64, 0xfd87b5f28300ca0du64] },  //  10^(-28) * 2^349
    BID_UINT256 { w: [0xa3c7e58e327f7cd4u64, 0x5f16f80067d80c84u64, 0x096ee45813a04330u64, 0xcad2f7f5359a3b3eu64] },  //  10^(-29) * 2^352
    BID_UINT256 { w: [0xb6398471c1ff9710u64, 0x18df2ccd1fe00a03u64, 0xa1258379a94d028du64, 0xa2425ff75e14fc31u64] },  //  10^(-30) * 2^355
    BID_UINT256 { w: [0xf82e038e34cc78dau64, 0x4718f0a419800802u64, 0x80eacf948770ced7u64, 0x81ceb32c4b43fcf4u64] },  //  10^(-31) * 2^358
    BID_UINT256 { w: [0x59e338e387ad8e29u64, 0x0b5b1aa028ccd99eu64, 0x67de18eda5814af2u64, 0xcfb11ead453994bau64] },  //  10^(-32) * 2^362
    BID_UINT256 { w: [0x47e8fa4f9fbe0b54u64, 0x6f7c154ced70ae18u64, 0xecb1ad8aeacdd58eu64, 0xa6274bbdd0fadd61u64] },  //  10^(-33) * 2^365
    BID_UINT256 { w: [0xd320c83fb2fe6f76u64, 0xbf967770bdf3be79u64, 0xbd5af13bef0b113eu64, 0x84ec3c97da624ab4u64] }   //  10^(-34) * 2^368
];

/// bid_ten2mk256truncM[] contains T*, the top Ex >= 256 bits of 10^(-k),
/// for 24 <= k <= 34; the top bits which are 0 are not represented
pub (crate) const bid_ten2mk256truncM: [BID_UINT256; 11] = [
    // the 64-bit word order is LL, LH, HL, HH
    BID_UINT256 { w: [0xf23472530ce6e3ecu64, 0xd78c3615cf3a050cu64, 0xc4926a9672793542u64, 0x9abe14cd44753b52u64] },  //  10^(-24) * 2^335
    BID_UINT256 { w: [0xe9ed83b814a49fe0u64, 0x8c1389bc7ec33b47u64, 0x3a83ddbd83f52204u64, 0xf79687aed3eec551u64] },  //  10^(-25) * 2^339
    BID_UINT256 { w: [0x87f1362cdd507fe6u64, 0x3cdc6e306568fc39u64, 0x95364afe032a819du64, 0xc612062576589ddau64] },  //  10^(-26) * 2^342
    BID_UINT256 { w: [0x775ea264cf55347cu64, 0x9ff42b5717739986u64, 0xca49f1c05120c9c7u64, 0x9e74d1b791e07e48u64] },  //  10^(-27) * 2^345
    BID_UINT256 { w: [0xccb9def1bf1f5c08u64, 0x76dcb60081ce0fa5u64, 0x8bca9d6e188853fcu64, 0xfd87b5f28300ca0du64] },  //  10^(-28) * 2^349
    BID_UINT256 { w: [0xa3c7e58e327f7cd3u64, 0x5f16f80067d80c84u64, 0x096ee45813a04330u64, 0xcad2f7f5359a3b3eu64] },  //  10^(-29) * 2^352
    BID_UINT256 { w: [0xb6398471c1ff970fu64, 0x18df2ccd1fe00a03u64, 0xa1258379a94d028du64, 0xa2425ff75e14fc31u64] },  //  10^(-30) * 2^355
    BID_UINT256 { w: [0xf82e038e34cc78d9u64, 0x4718f0a419800802u64, 0x80eacf948770ced7u64, 0x81ceb32c4b43fcf4u64] },  //  10^(-31) * 2^358
    BID_UINT256 { w: [0x59e338e387ad8e28u64, 0x0b5b1aa028ccd99eu64, 0x67de18eda5814af2u64, 0xcfb11ead453994bau64] },  //  10^(-32) * 2^362
    BID_UINT256 { w: [0x47e8fa4f9fbe0b53u64, 0x6f7c154ced70ae18u64, 0xecb1ad8aeacdd58eu64, 0xa6274bbdd0fadd61u64] },  //  10^(-33) * 2^365
    BID_UINT256 { w: [0xd320c83fb2fe6f75u64, 0xbf967770bdf3be79u64, 0xbd5af13bef0b113eu64, 0x84ec3c97da624ab4u64] }   //  10^(-34) * 2^368
];

/// bid_shiftright256M[] contains the right shift count to obtain C2* from the top
/// 192 bits of the 256x256-bit product C2 * Kx
pub (crate) const bid_shiftright256M: [i32; 11] = [
    15,  // 335 - 320
    19,  // 339 - 320
    22,  // 342 - 320
    25,  // 345 - 320
    29,  // 349 - 320
    32,  // 352 - 320 // careful of 32-bit machines!
    35,  // 355 - 320
    38,  // 358 - 320
    42,  // 362 - 320
    45,  // 365 - 320
    48   // 368 - 320
];

/// bid_maskhigh256M[] contains the mask to apply to the top 192 bits of the
/// 256x256-bit product in order to obtain the high bits of f*
pub (crate) const bid_maskhigh256M: [BID_UINT64; 11] = [
    0x0000000000007fffu64,  //  15 = 335 - 320 bits
    0x000000000007ffffu64,  //  19 = 339 - 320 bits
    0x00000000003fffffu64,  //  22 = 342 - 320 bits
    0x0000000001ffffffu64,  //  25 = 345 - 320 bits
    0x000000001fffffffu64,  //  29 = 349 - 320 bits
    0x00000000ffffffffu64,  //  32 = 352 - 320 bits
    0x00000007ffffffffu64,  //  35 = 355 - 320 bits
    0x0000003fffffffffu64,  //  38 = 358 - 320 bits
    0x000003ffffffffffu64,  //  42 = 362 - 320 bits
    0x00001fffffffffffu64,  //  45 = 365 - 320 bits
    0x0000ffffffffffffu64   //  48 = 368 - 320 bits
];

/// bid_onehalf256M[] contains 1/2 positioned correctly for comparison with the
/// high bits of f*; the high 128 bits of the 512-bit mask are 0
pub (crate) const bid_onehalf256M: [BID_UINT64; 11] = [
    0x0000000000004000u64,  //  15 = 335 - 320 bits
    0x0000000000040000u64,  //  19 = 339 - 320 bits
    0x0000000000200000u64,  //  22 = 342 - 320 bits
    0x0000000001000000u64,  //  25 = 345 - 320 bits
    0x0000000010000000u64,  //  29 = 349 - 320 bits
    0x0000000080000000u64,  //  32 = 352 - 320 bits
    0x0000000400000000u64,  //  35 = 355 - 320 bits
    0x0000002000000000u64,  //  38 = 358 - 320 bits
    0x0000020000000000u64,  //  42 = 362 - 320 bits
    0x0000100000000000u64,  //  45 = 365 - 320 bits
    0x0000800000000000u64   //  48 = 368 - 320 bits
];

/// bid_char_table2[] is used to convert n to string, where 10 <= n <= 99
pub (crate) const bid_char_table2: [char; 180] = [
    '1', '0', '1', '1', '1', '2', '1', '3', '1', '4', '1', '5', '1', '6', '1', '7', '1', '8', '1', '9', '2', '0', '2',
    '1', '2', '2', '2', '3', '2', '4', '2', '5', '2', '6', '2', '7', '2', '8', '2', '9', '3', '0', '3', '1', '3', '2',
    '3', '3', '3', '4', '3', '5', '3', '6', '3', '7', '3', '8', '3', '9', '4', '0', '4', '1', '4', '2', '4', '3', '4',
    '4', '4', '5', '4', '6', '4', '7', '4', '8', '4', '9', '5', '0', '5', '1', '5', '2', '5', '3', '5', '4', '5', '5',
    '5', '6', '5', '7', '5', '8', '5', '9', '6', '0', '6', '1', '6', '2', '6', '3', '6', '4', '6', '5', '6', '6', '6',
    '7', '6', '8', '6', '9', '7', '0', '7', '1', '7', '2', '7', '3', '7', '4', '7', '5', '7', '6', '7', '7', '7', '8',
    '7', '9', '8', '0', '8', '1', '8', '2', '8', '3', '8', '4', '8', '5', '8', '6', '8', '7', '8', '8', '8', '9', '9',
    '0', '9', '1', '9', '2', '9', '3', '9', '4', '9', '5', '9', '6', '9', '7', '9', '8', '9', '9'
];

/// bid_char_table3[] is used to convert n to string, where 000 <= n <= 999
pub (crate) const bid_char_table3: [char; 3000] = [
    '0', '0', '0', '0', '0', '1', '0', '0', '2', '0', '0', '3', '0', '0', '4', '0', '0', '5', '0', '0', '6', '0', '0',
    '7', '0', '0', '8', '0', '0', '9', '0', '1', '0', '0', '1', '1', '0', '1', '2', '0', '1', '3', '0', '1', '4', '0',
    '1', '5', '0', '1', '6', '0', '1', '7', '0', '1', '8', '0', '1', '9', '0', '2', '0', '0', '2', '1', '0', '2', '2',
    '0', '2', '3', '0', '2', '4', '0', '2', '5', '0', '2', '6', '0', '2', '7', '0', '2', '8', '0', '2', '9', '0', '3',
    '0', '0', '3', '1', '0', '3', '2', '0', '3', '3', '0', '3', '4', '0', '3', '5', '0', '3', '6', '0', '3', '7', '0',
    '3', '8', '0', '3', '9', '0', '4', '0', '0', '4', '1', '0', '4', '2', '0', '4', '3', '0', '4', '4', '0', '4', '5',
    '0', '4', '6', '0', '4', '7', '0', '4', '8', '0', '4', '9', '0', '5', '0', '0', '5', '1', '0', '5', '2', '0', '5',
    '3', '0', '5', '4', '0', '5', '5', '0', '5', '6', '0', '5', '7', '0', '5', '8', '0', '5', '9', '0', '6', '0', '0',
    '6', '1', '0', '6', '2', '0', '6', '3', '0', '6', '4', '0', '6', '5', '0', '6', '6', '0', '6', '7', '0', '6', '8',
    '0', '6', '9', '0', '7', '0', '0', '7', '1', '0', '7', '2', '0', '7', '3', '0', '7', '4', '0', '7', '5', '0', '7',
    '6', '0', '7', '7', '0', '7', '8', '0', '7', '9', '0', '8', '0', '0', '8', '1', '0', '8', '2', '0', '8', '3', '0',
    '8', '4', '0', '8', '5', '0', '8', '6', '0', '8', '7', '0', '8', '8', '0', '8', '9', '0', '9', '0', '0', '9', '1',
    '0', '9', '2', '0', '9', '3', '0', '9', '4', '0', '9', '5', '0', '9', '6', '0', '9', '7', '0', '9', '8', '0', '9',
    '9', '1', '0', '0', '1', '0', '1', '1', '0', '2', '1', '0', '3', '1', '0', '4', '1', '0', '5', '1', '0', '6', '1',
    '0', '7', '1', '0', '8', '1', '0', '9', '1', '1', '0', '1', '1', '1', '1', '1', '2', '1', '1', '3', '1', '1', '4',
    '1', '1', '5', '1', '1', '6', '1', '1', '7', '1', '1', '8', '1', '1', '9', '1', '2', '0', '1', '2', '1', '1', '2',
    '2', '1', '2', '3', '1', '2', '4', '1', '2', '5', '1', '2', '6', '1', '2', '7', '1', '2', '8', '1', '2', '9', '1',
    '3', '0', '1', '3', '1', '1', '3', '2', '1', '3', '3', '1', '3', '4', '1', '3', '5', '1', '3', '6', '1', '3', '7',
    '1', '3', '8', '1', '3', '9', '1', '4', '0', '1', '4', '1', '1', '4', '2', '1', '4', '3', '1', '4', '4', '1', '4',
    '5', '1', '4', '6', '1', '4', '7', '1', '4', '8', '1', '4', '9', '1', '5', '0', '1', '5', '1', '1', '5', '2', '1',
    '5', '3', '1', '5', '4', '1', '5', '5', '1', '5', '6', '1', '5', '7', '1', '5', '8', '1', '5', '9', '1', '6', '0',
    '1', '6', '1', '1', '6', '2', '1', '6', '3', '1', '6', '4', '1', '6', '5', '1', '6', '6', '1', '6', '7', '1', '6',
    '8', '1', '6', '9', '1', '7', '0', '1', '7', '1', '1', '7', '2', '1', '7', '3', '1', '7', '4', '1', '7', '5', '1',
    '7', '6', '1', '7', '7', '1', '7', '8', '1', '7', '9', '1', '8', '0', '1', '8', '1', '1', '8', '2', '1', '8', '3',
    '1', '8', '4', '1', '8', '5', '1', '8', '6', '1', '8', '7', '1', '8', '8', '1', '8', '9', '1', '9', '0', '1', '9',
    '1', '1', '9', '2', '1', '9', '3', '1', '9', '4', '1', '9', '5', '1', '9', '6', '1', '9', '7', '1', '9', '8', '1',
    '9', '9', '2', '0', '0', '2', '0', '1', '2', '0', '2', '2', '0', '3', '2', '0', '4', '2', '0', '5', '2', '0', '6',
    '2', '0', '7', '2', '0', '8', '2', '0', '9', '2', '1', '0', '2', '1', '1', '2', '1', '2', '2', '1', '3', '2', '1',
    '4', '2', '1', '5', '2', '1', '6', '2', '1', '7', '2', '1', '8', '2', '1', '9', '2', '2', '0', '2', '2', '1', '2',
    '2', '2', '2', '2', '3', '2', '2', '4', '2', '2', '5', '2', '2', '6', '2', '2', '7', '2', '2', '8', '2', '2', '9',
    '2', '3', '0', '2', '3', '1', '2', '3', '2', '2', '3', '3', '2', '3', '4', '2', '3', '5', '2', '3', '6', '2', '3',
    '7', '2', '3', '8', '2', '3', '9', '2', '4', '0', '2', '4', '1', '2', '4', '2', '2', '4', '3', '2', '4', '4', '2',
    '4', '5', '2', '4', '6', '2', '4', '7', '2', '4', '8', '2', '4', '9', '2', '5', '0', '2', '5', '1', '2', '5', '2',
    '2', '5', '3', '2', '5', '4', '2', '5', '5', '2', '5', '6', '2', '5', '7', '2', '5', '8', '2', '5', '9', '2', '6',
    '0', '2', '6', '1', '2', '6', '2', '2', '6', '3', '2', '6', '4', '2', '6', '5', '2', '6', '6', '2', '6', '7', '2',
    '6', '8', '2', '6', '9', '2', '7', '0', '2', '7', '1', '2', '7', '2', '2', '7', '3', '2', '7', '4', '2', '7', '5',
    '2', '7', '6', '2', '7', '7', '2', '7', '8', '2', '7', '9', '2', '8', '0', '2', '8', '1', '2', '8', '2', '2', '8',
    '3', '2', '8', '4', '2', '8', '5', '2', '8', '6', '2', '8', '7', '2', '8', '8', '2', '8', '9', '2', '9', '0', '2',
    '9', '1', '2', '9', '2', '2', '9', '3', '2', '9', '4', '2', '9', '5', '2', '9', '6', '2', '9', '7', '2', '9', '8',
    '2', '9', '9', '3', '0', '0', '3', '0', '1', '3', '0', '2', '3', '0', '3', '3', '0', '4', '3', '0', '5', '3', '0',
    '6', '3', '0', '7', '3', '0', '8', '3', '0', '9', '3', '1', '0', '3', '1', '1', '3', '1', '2', '3', '1', '3', '3',
    '1', '4', '3', '1', '5', '3', '1', '6', '3', '1', '7', '3', '1', '8', '3', '1', '9', '3', '2', '0', '3', '2', '1',
    '3', '2', '2', '3', '2', '3', '3', '2', '4', '3', '2', '5', '3', '2', '6', '3', '2', '7', '3', '2', '8', '3', '2',
    '9', '3', '3', '0', '3', '3', '1', '3', '3', '2', '3', '3', '3', '3', '3', '4', '3', '3', '5', '3', '3', '6', '3',
    '3', '7', '3', '3', '8', '3', '3', '9', '3', '4', '0', '3', '4', '1', '3', '4', '2', '3', '4', '3', '3', '4', '4',
    '3', '4', '5', '3', '4', '6', '3', '4', '7', '3', '4', '8', '3', '4', '9', '3', '5', '0', '3', '5', '1', '3', '5',
    '2', '3', '5', '3', '3', '5', '4', '3', '5', '5', '3', '5', '6', '3', '5', '7', '3', '5', '8', '3', '5', '9', '3',
    '6', '0', '3', '6', '1', '3', '6', '2', '3', '6', '3', '3', '6', '4', '3', '6', '5', '3', '6', '6', '3', '6', '7',
    '3', '6', '8', '3', '6', '9', '3', '7', '0', '3', '7', '1', '3', '7', '2', '3', '7', '3', '3', '7', '4', '3', '7',
    '5', '3', '7', '6', '3', '7', '7', '3', '7', '8', '3', '7', '9', '3', '8', '0', '3', '8', '1', '3', '8', '2', '3',
    '8', '3', '3', '8', '4', '3', '8', '5', '3', '8', '6', '3', '8', '7', '3', '8', '8', '3', '8', '9', '3', '9', '0',
    '3', '9', '1', '3', '9', '2', '3', '9', '3', '3', '9', '4', '3', '9', '5', '3', '9', '6', '3', '9', '7', '3', '9',
    '8', '3', '9', '9', '4', '0', '0', '4', '0', '1', '4', '0', '2', '4', '0', '3', '4', '0', '4', '4', '0', '5', '4',
    '0', '6', '4', '0', '7', '4', '0', '8', '4', '0', '9', '4', '1', '0', '4', '1', '1', '4', '1', '2', '4', '1', '3',
    '4', '1', '4', '4', '1', '5', '4', '1', '6', '4', '1', '7', '4', '1', '8', '4', '1', '9', '4', '2', '0', '4', '2',
    '1', '4', '2', '2', '4', '2', '3', '4', '2', '4', '4', '2', '5', '4', '2', '6', '4', '2', '7', '4', '2', '8', '4',
    '2', '9', '4', '3', '0', '4', '3', '1', '4', '3', '2', '4', '3', '3', '4', '3', '4', '4', '3', '5', '4', '3', '6',
    '4', '3', '7', '4', '3', '8', '4', '3', '9', '4', '4', '0', '4', '4', '1', '4', '4', '2', '4', '4', '3', '4', '4',
    '4', '4', '4', '5', '4', '4', '6', '4', '4', '7', '4', '4', '8', '4', '4', '9', '4', '5', '0', '4', '5', '1', '4',
    '5', '2', '4', '5', '3', '4', '5', '4', '4', '5', '5', '4', '5', '6', '4', '5', '7', '4', '5', '8', '4', '5', '9',
    '4', '6', '0', '4', '6', '1', '4', '6', '2', '4', '6', '3', '4', '6', '4', '4', '6', '5', '4', '6', '6', '4', '6',
    '7', '4', '6', '8', '4', '6', '9', '4', '7', '0', '4', '7', '1', '4', '7', '2', '4', '7', '3', '4', '7', '4', '4',
    '7', '5', '4', '7', '6', '4', '7', '7', '4', '7', '8', '4', '7', '9', '4', '8', '0', '4', '8', '1', '4', '8', '2',
    '4', '8', '3', '4', '8', '4', '4', '8', '5', '4', '8', '6', '4', '8', '7', '4', '8', '8', '4', '8', '9', '4', '9',
    '0', '4', '9', '1', '4', '9', '2', '4', '9', '3', '4', '9', '4', '4', '9', '5', '4', '9', '6', '4', '9', '7', '4',
    '9', '8', '4', '9', '9', '5', '0', '0', '5', '0', '1', '5', '0', '2', '5', '0', '3', '5', '0', '4', '5', '0', '5',
    '5', '0', '6', '5', '0', '7', '5', '0', '8', '5', '0', '9', '5', '1', '0', '5', '1', '1', '5', '1', '2', '5', '1',
    '3', '5', '1', '4', '5', '1', '5', '5', '1', '6', '5', '1', '7', '5', '1', '8', '5', '1', '9', '5', '2', '0', '5',
    '2', '1', '5', '2', '2', '5', '2', '3', '5', '2', '4', '5', '2', '5', '5', '2', '6', '5', '2', '7', '5', '2', '8',
    '5', '2', '9', '5', '3', '0', '5', '3', '1', '5', '3', '2', '5', '3', '3', '5', '3', '4', '5', '3', '5', '5', '3',
    '6', '5', '3', '7', '5', '3', '8', '5', '3', '9', '5', '4', '0', '5', '4', '1', '5', '4', '2', '5', '4', '3', '5',
    '4', '4', '5', '4', '5', '5', '4', '6', '5', '4', '7', '5', '4', '8', '5', '4', '9', '5', '5', '0', '5', '5', '1',
    '5', '5', '2', '5', '5', '3', '5', '5', '4', '5', '5', '5', '5', '5', '6', '5', '5', '7', '5', '5', '8', '5', '5',
    '9', '5', '6', '0', '5', '6', '1', '5', '6', '2', '5', '6', '3', '5', '6', '4', '5', '6', '5', '5', '6', '6', '5',
    '6', '7', '5', '6', '8', '5', '6', '9', '5', '7', '0', '5', '7', '1', '5', '7', '2', '5', '7', '3', '5', '7', '4',
    '5', '7', '5', '5', '7', '6', '5', '7', '7', '5', '7', '8', '5', '7', '9', '5', '8', '0', '5', '8', '1', '5', '8',
    '2', '5', '8', '3', '5', '8', '4', '5', '8', '5', '5', '8', '6', '5', '8', '7', '5', '8', '8', '5', '8', '9', '5',
    '9', '0', '5', '9', '1', '5', '9', '2', '5', '9', '3', '5', '9', '4', '5', '9', '5', '5', '9', '6', '5', '9', '7',
    '5', '9', '8', '5', '9', '9', '6', '0', '0', '6', '0', '1', '6', '0', '2', '6', '0', '3', '6', '0', '4', '6', '0',
    '5', '6', '0', '6', '6', '0', '7', '6', '0', '8', '6', '0', '9', '6', '1', '0', '6', '1', '1', '6', '1', '2', '6',
    '1', '3', '6', '1', '4', '6', '1', '5', '6', '1', '6', '6', '1', '7', '6', '1', '8', '6', '1', '9', '6', '2', '0',
    '6', '2', '1', '6', '2', '2', '6', '2', '3', '6', '2', '4', '6', '2', '5', '6', '2', '6', '6', '2', '7', '6', '2',
    '8', '6', '2', '9', '6', '3', '0', '6', '3', '1', '6', '3', '2', '6', '3', '3', '6', '3', '4', '6', '3', '5', '6',
    '3', '6', '6', '3', '7', '6', '3', '8', '6', '3', '9', '6', '4', '0', '6', '4', '1', '6', '4', '2', '6', '4', '3',
    '6', '4', '4', '6', '4', '5', '6', '4', '6', '6', '4', '7', '6', '4', '8', '6', '4', '9', '6', '5', '0', '6', '5',
    '1', '6', '5', '2', '6', '5', '3', '6', '5', '4', '6', '5', '5', '6', '5', '6', '6', '5', '7', '6', '5', '8', '6',
    '5', '9', '6', '6', '0', '6', '6', '1', '6', '6', '2', '6', '6', '3', '6', '6', '4', '6', '6', '5', '6', '6', '6',
    '6', '6', '7', '6', '6', '8', '6', '6', '9', '6', '7', '0', '6', '7', '1', '6', '7', '2', '6', '7', '3', '6', '7',
    '4', '6', '7', '5', '6', '7', '6', '6', '7', '7', '6', '7', '8', '6', '7', '9', '6', '8', '0', '6', '8', '1', '6',
    '8', '2', '6', '8', '3', '6', '8', '4', '6', '8', '5', '6', '8', '6', '6', '8', '7', '6', '8', '8', '6', '8', '9',
    '6', '9', '0', '6', '9', '1', '6', '9', '2', '6', '9', '3', '6', '9', '4', '6', '9', '5', '6', '9', '6', '6', '9',
    '7', '6', '9', '8', '6', '9', '9', '7', '0', '0', '7', '0', '1', '7', '0', '2', '7', '0', '3', '7', '0', '4', '7',
    '0', '5', '7', '0', '6', '7', '0', '7', '7', '0', '8', '7', '0', '9', '7', '1', '0', '7', '1', '1', '7', '1', '2',
    '7', '1', '3', '7', '1', '4', '7', '1', '5', '7', '1', '6', '7', '1', '7', '7', '1', '8', '7', '1', '9', '7', '2',
    '0', '7', '2', '1', '7', '2', '2', '7', '2', '3', '7', '2', '4', '7', '2', '5', '7', '2', '6', '7', '2', '7', '7',
    '2', '8', '7', '2', '9', '7', '3', '0', '7', '3', '1', '7', '3', '2', '7', '3', '3', '7', '3', '4', '7', '3', '5',
    '7', '3', '6', '7', '3', '7', '7', '3', '8', '7', '3', '9', '7', '4', '0', '7', '4', '1', '7', '4', '2', '7', '4',
    '3', '7', '4', '4', '7', '4', '5', '7', '4', '6', '7', '4', '7', '7', '4', '8', '7', '4', '9', '7', '5', '0', '7',
    '5', '1', '7', '5', '2', '7', '5', '3', '7', '5', '4', '7', '5', '5', '7', '5', '6', '7', '5', '7', '7', '5', '8',
    '7', '5', '9', '7', '6', '0', '7', '6', '1', '7', '6', '2', '7', '6', '3', '7', '6', '4', '7', '6', '5', '7', '6',
    '6', '7', '6', '7', '7', '6', '8', '7', '6', '9', '7', '7', '0', '7', '7', '1', '7', '7', '2', '7', '7', '3', '7',
    '7', '4', '7', '7', '5', '7', '7', '6', '7', '7', '7', '7', '7', '8', '7', '7', '9', '7', '8', '0', '7', '8', '1',
    '7', '8', '2', '7', '8', '3', '7', '8', '4', '7', '8', '5', '7', '8', '6', '7', '8', '7', '7', '8', '8', '7', '8',
    '9', '7', '9', '0', '7', '9', '1', '7', '9', '2', '7', '9', '3', '7', '9', '4', '7', '9', '5', '7', '9', '6', '7',
    '9', '7', '7', '9', '8', '7', '9', '9', '8', '0', '0', '8', '0', '1', '8', '0', '2', '8', '0', '3', '8', '0', '4',
    '8', '0', '5', '8', '0', '6', '8', '0', '7', '8', '0', '8', '8', '0', '9', '8', '1', '0', '8', '1', '1', '8', '1',
    '2', '8', '1', '3', '8', '1', '4', '8', '1', '5', '8', '1', '6', '8', '1', '7', '8', '1', '8', '8', '1', '9', '8',
    '2', '0', '8', '2', '1', '8', '2', '2', '8', '2', '3', '8', '2', '4', '8', '2', '5', '8', '2', '6', '8', '2', '7',
    '8', '2', '8', '8', '2', '9', '8', '3', '0', '8', '3', '1', '8', '3', '2', '8', '3', '3', '8', '3', '4', '8', '3',
    '5', '8', '3', '6', '8', '3', '7', '8', '3', '8', '8', '3', '9', '8', '4', '0', '8', '4', '1', '8', '4', '2', '8',
    '4', '3', '8', '4', '4', '8', '4', '5', '8', '4', '6', '8', '4', '7', '8', '4', '8', '8', '4', '9', '8', '5', '0',
    '8', '5', '1', '8', '5', '2', '8', '5', '3', '8', '5', '4', '8', '5', '5', '8', '5', '6', '8', '5', '7', '8', '5',
    '8', '8', '5', '9', '8', '6', '0', '8', '6', '1', '8', '6', '2', '8', '6', '3', '8', '6', '4', '8', '6', '5', '8',
    '6', '6', '8', '6', '7', '8', '6', '8', '8', '6', '9', '8', '7', '0', '8', '7', '1', '8', '7', '2', '8', '7', '3',
    '8', '7', '4', '8', '7', '5', '8', '7', '6', '8', '7', '7', '8', '7', '8', '8', '7', '9', '8', '8', '0', '8', '8',
    '1', '8', '8', '2', '8', '8', '3', '8', '8', '4', '8', '8', '5', '8', '8', '6', '8', '8', '7', '8', '8', '8', '8',
    '8', '9', '8', '9', '0', '8', '9', '1', '8', '9', '2', '8', '9', '3', '8', '9', '4', '8', '9', '5', '8', '9', '6',
    '8', '9', '7', '8', '9', '8', '8', '9', '9', '9', '0', '0', '9', '0', '1', '9', '0', '2', '9', '0', '3', '9', '0',
    '4', '9', '0', '5', '9', '0', '6', '9', '0', '7', '9', '0', '8', '9', '0', '9', '9', '1', '0', '9', '1', '1', '9',
    '1', '2', '9', '1', '3', '9', '1', '4', '9', '1', '5', '9', '1', '6', '9', '1', '7', '9', '1', '8', '9', '1', '9',
    '9', '2', '0', '9', '2', '1', '9', '2', '2', '9', '2', '3', '9', '2', '4', '9', '2', '5', '9', '2', '6', '9', '2',
    '7', '9', '2', '8', '9', '2', '9', '9', '3', '0', '9', '3', '1', '9', '3', '2', '9', '3', '3', '9', '3', '4', '9',
    '3', '5', '9', '3', '6', '9', '3', '7', '9', '3', '8', '9', '3', '9', '9', '4', '0', '9', '4', '1', '9', '4', '2',
    '9', '4', '3', '9', '4', '4', '9', '4', '5', '9', '4', '6', '9', '4', '7', '9', '4', '8', '9', '4', '9', '9', '5',
    '0', '9', '5', '1', '9', '5', '2', '9', '5', '3', '9', '5', '4', '9', '5', '5', '9', '5', '6', '9', '5', '7', '9',
    '5', '8', '9', '5', '9', '9', '6', '0', '9', '6', '1', '9', '6', '2', '9', '6', '3', '9', '6', '4', '9', '6', '5',
    '9', '6', '6', '9', '6', '7', '9', '6', '8', '9', '6', '9', '9', '7', '0', '9', '7', '1', '9', '7', '2', '9', '7',
    '3', '9', '7', '4', '9', '7', '5', '9', '7', '6', '9', '7', '7', '9', '7', '8', '9', '7', '9', '9', '8', '0', '9',
    '8', '1', '9', '8', '2', '9', '8', '3', '9', '8', '4', '9', '8', '5', '9', '8', '6', '9', '8', '7', '9', '8', '8',
    '9', '8', '9', '9', '9', '0', '9', '9', '1', '9', '9', '2', '9', '9', '3', '9', '9', '4', '9', '9', '5', '9', '9',
    '6', '9', '9', '7', '9', '9', '8', '9', '9', '9'
];

/// bid_ten2m3k64[], bid_shift_ten2m3k64[] used for conversion from BID128 to string
pub (crate) const bid_ten2m3k64: [BID_UINT64; 5] = [
    0x4189374bc6a7ef9eu64,  // 4189374bc6a7ef9e * 2^-72 = (10^-3)RP,63
    0x10c6f7a0b5ed8d37u64,  // 10c6f7a0b5ed8d37 * 2^-80 = (10^-6)RP,61
    0x44b82fa09b5a52ccu64,  // 44b82fa09b5a52cc * 2^-92 = (10^-9)RP,63
    0x119799812dea111au64,  // 119799812dea111a * 2^-100 = (10^-12)RP,61
    0x480ebe7b9d58566du64   // 480ebe7b9d58566d * 2^-112 = (10^-15)RP,63
];

pub (crate) const bid_shift_ten2m3k64: [u32; 5] = [
    8,   // 72 - 64
    16,  // 80 - 64
    28,  // 92 - 64
    36,  // 100 - 64
    48   // 112 - 64
];

pub (crate) const bid_ten2m3k128: [BID_UINT128; 11] = [
    BID_UINT128 { w: [0xb22d0e5604189375u64, 0x4189374bc6a7ef9du64] }, // 4189374bc6a7ef9d  b22d0e5604189375  * 2^-136 = (10^-3)RP,127
    BID_UINT128 { w: [0xb4c7f34938583622u64, 0x10c6f7a0b5ed8d36u64] }, // 10c6f7a0b5ed8d36  b4c7f34938583622  * 2^-144 = (10^-6)RP,125
    BID_UINT128 { w: [0x98b405447c4a9819u64, 0x44b82fa09b5a52cbu64] }, // 44b82fa09b5a52cb  98b405447c4a9819  * 2^-156 = (10^-9)RP,127
    BID_UINT128 { w: [0x7f27f0f6e885c8bbu64, 0x119799812dea1119u64] }, // 119799812dea1119  7f27f0f6e885c8bb  * 2^-164 = (10^-12)RP,125
    BID_UINT128 { w: [0x87ce9b80a5fb0509u64, 0x480ebe7b9d58566cu64] }, // 480ebe7b9d58566c  87ce9b80a5fb0509  * 2^-176 = (10^-15)RP,127
    BID_UINT128 { w: [0xe75fe645cc4873fau64, 0x12725dd1d243aba0u64] }, // 12725dd1d243aba0  e75fe645cc4873fa  * 2^-184 = (10^-18)RP,125
    BID_UINT128 { w: [0x69fb7e0b75e52f02u64, 0x4b8ed0283a6d3df7u64] }, // 4b8ed0283a6d3df7  69fb7e0b75e52f02  * 2^-196 = (10^-21)RP,127
    BID_UINT128 { w: [0x58924d52ce4f26a9u64, 0x1357c299a88ea76au64] }, // 1357c299a88ea76a  58924d52ce4f26a9  * 2^-204 = (10^-24)RP,125
    BID_UINT128 { w: [0x3baf513267aa9a3fu64, 0x4f3a68dbc8f03f24u64] }, // 4f3a68dbc8f03f24  3baf513267aa9a3f  * 2^-216 = (10^-27)RP,127
    BID_UINT128 { w: [0x3424b06f3529a052u64, 0x14484bfeebc29f86u64] }, // 14484bfeebc29f86  3424b06f3529a052  * 2^-224 = (10^-30)RP,125
    BID_UINT128 { w: [0xf658d6c57566eac8u64, 0x5313a5dee87d6eb0u64] }  // 5313a5dee87d6eb0  f658d6c57566eac8  * 2^-236 = (10^-33)RP,127
];

pub (crate) const bid_shift_ten2m3k128: [u32; 11] = [
    8,   // 136 - 128
    16,  // 144 - 128
    28,  // 156 - 128
    36,  // 164 - 128
    48,  // 176 - 128
    56,  // 184 - 128
    4,   // 196 - 192
    12,  // 204 - 192
    24,  // 216 - 192
    32,  // 224 - 192
    44   // 236 - 192
];

/***************************************************************************
 *************** TABLES FOR GENERAL ROUNDING FUNCTIONS *********************
 ***************************************************************************/
// Note: not all entries in these tables will be used with IEEE 754 decimal
// floating-point arithmetic
// a) In round128_2_18() numbers with 2 <= q <= 18 will be rounded only
//    for 1 <= x <= 3:
//     x = 1 or x = 2 when q = 17
//     x = 2 or x = 3 when q = 18
// b) In bid_round128_19_38() numbers with 19 <= q <= 38 will be rounded only
//    for 1 <= x <= 23:
//     x = 3 or x = 4 when q = 19
//     x = 4 or x = 5 when q = 20
//     ...
//     x = 18 or x = 19 when q = 34
//     x = 1 or x = 2 or x = 19 or x = 20 when q = 35
//     x = 2 or x = 3 or x = 20 or x = 21 when q = 36
//     x = 3 or x = 4 or x = 21 or x = 22 when q = 37
//     x = 4 or x = 5 or x = 22 or x = 23 when q = 38
// c) ...
// However, for generality and possible uses outside the frame of IEEE 754
// this implementation includes table values for all x in [1, q - 1]

// Note: 64-bit tables generated with ten2mx64.ma; output in ten2mx64.out

/// Kx from 10^(-x) ~= Kx * 2^(-Ex); Kx rounded up to 64 bits, 1 <= x <= 17
pub (crate) const bid_Kx64: [BID_UINT64; 17] = [
    0xcccccccccccccccdu64,  // 10^-1 ~= cccccccccccccccd * 2^-67
    0xa3d70a3d70a3d70bu64,  // 10^-2 ~= a3d70a3d70a3d70b * 2^-70
    0x83126e978d4fdf3cu64,  // 10^-3 ~= 83126e978d4fdf3c * 2^-73
    0xd1b71758e219652cu64,  // 10^-4 ~= d1b71758e219652c * 2^-77
    0xa7c5ac471b478424u64,  // 10^-5 ~= a7c5ac471b478424 * 2^-80
    0x8637bd05af6c69b6u64,  // 10^-6 ~= 8637bd05af6c69b6 * 2^-83
    0xd6bf94d5e57a42bdu64,  // 10^-7 ~= d6bf94d5e57a42bd * 2^-87
    0xabcc77118461cefdu64,  // 10^-8 ~= abcc77118461cefd * 2^-90
    0x89705f4136b4a598u64,  // 10^-9 ~= 89705f4136b4a598 * 2^-93
    0xdbe6fecebdedd5bfu64,  // 10^-10 ~= dbe6fecebdedd5bf * 2^-97
    0xafebff0bcb24aaffu64,  // 10^-11 ~= afebff0bcb24aaff * 2^-100
    0x8cbccc096f5088ccu64,  // 10^-12 ~= 8cbccc096f5088cc * 2^-103
    0xe12e13424bb40e14u64,  // 10^-13 ~= e12e13424bb40e14 * 2^-107
    0xb424dc35095cd810u64,  // 10^-14 ~= b424dc35095cd810 * 2^-110
    0x901d7cf73ab0acdau64,  // 10^-15 ~= 901d7cf73ab0acda * 2^-113
    0xe69594bec44de15cu64,  // 10^-16 ~= e69594bec44de15c * 2^-117
    0xb877aa3236a4b44au64   // 10^-17 ~= b877aa3236a4b44a * 2^-120
];

/// Ex-64 from 10^(-x) ~= Kx * 2^(-Ex); Kx rounded up to 64 bits, 1 <= x <= 17
pub (crate) const bid_Ex64m64: [u32; 17] = [
    3,   // 67 - 64, Ex = 67
    6,   // 70 - 64, Ex = 70
    9,   // 73 - 64, Ex = 73
    13,  // 77 - 64, Ex = 77
    16,  // 80 - 64, Ex = 80
    19,  // 83 - 64, Ex = 83
    23,  // 87 - 64, Ex = 87
    26,  // 90 - 64, Ex = 90
    29,  // 93 - 64, Ex = 93
    33,  // 97 - 64, Ex = 97
    36,  // 100 - 64, Ex = 100
    39,  // 103 - 64, Ex = 103
    43,  // 107 - 64, Ex = 107
    46,  // 110 - 64, Ex = 110
    49,  // 113 - 64, Ex = 113
    53,  // 117 - 64, Ex = 117
    56   // 120 - 64, Ex = 120
];

/// Values of 1/2 in the right position to be compared with the fraction from
/// C * kx, 1 <= x <= 17; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the high 64 bits of the fraction)
pub (crate) const bid_half64: [BID_UINT64; 17] = [
    0x0000000000000004u64,  // half / 2^64 = 4
    0x0000000000000020u64,  // half / 2^64 = 20
    0x0000000000000100u64,  // half / 2^64 = 100
    0x0000000000001000u64,  // half / 2^64 = 1000
    0x0000000000008000u64,  // half / 2^64 = 8000
    0x0000000000040000u64,  // half / 2^64 = 40000
    0x0000000000400000u64,  // half / 2^64 = 400000
    0x0000000002000000u64,  // half / 2^64 = 2000000
    0x0000000010000000u64,  // half / 2^64 = 10000000
    0x0000000100000000u64,  // half / 2^64 = 100000000
    0x0000000800000000u64,  // half / 2^64 = 800000000
    0x0000004000000000u64,  // half / 2^64 = 4000000000
    0x0000040000000000u64,  // half / 2^64 = 40000000000
    0x0000200000000000u64,  // half / 2^64 = 200000000000
    0x0001000000000000u64,  // half / 2^64 = 1000000000000
    0x0010000000000000u64,  // half / 2^64 = 10000000000000
    0x0080000000000000u64   // half / 2^64 = 80000000000000
];

/// Values of mask in the right position to obtain the high Ex - 64 bits
/// of the fraction from C * kx, 1 <= x <= 17; the fraction consists of
/// the low Ex bits in C * kx
pub (crate) const bid_mask64: [BID_UINT64; 17] = [
    0x0000000000000007u64,  // mask / 2^64
    0x000000000000003fu64,  // mask / 2^64
    0x00000000000001ffu64,  // mask / 2^64
    0x0000000000001fffu64,  // mask / 2^64
    0x000000000000ffffu64,  // mask / 2^64
    0x000000000007ffffu64,  // mask / 2^64
    0x00000000007fffffu64,  // mask / 2^64
    0x0000000003ffffffu64,  // mask / 2^64
    0x000000001fffffffu64,  // mask / 2^64
    0x00000001ffffffffu64,  // mask / 2^64
    0x0000000fffffffffu64,  // mask / 2^64
    0x0000007fffffffffu64,  // mask / 2^64
    0x000007ffffffffffu64,  // mask / 2^64
    0x00003fffffffffffu64,  // mask / 2^64
    0x0001ffffffffffffu64,  // mask / 2^64
    0x001fffffffffffffu64,  // mask / 2^64
    0x00ffffffffffffffu64   // mask / 2^64
];

/// Values of 10^(-x) trancated to Ex bits beyond the binary point, and
/// in the right position to be compared with the fraction from C * kx,
/// 1 <= x <= 17; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the low 64 bits of the fraction)
pub (crate) const bid_ten2mxtrunc64: [BID_UINT64; 17] = [
    0xccccccccccccccccu64,  // (ten2mx >> 64) = cccccccccccccccc
    0xa3d70a3d70a3d70au64,  // (ten2mx >> 64) = a3d70a3d70a3d70a
    0x83126e978d4fdf3bu64,  // (ten2mx >> 64) = 83126e978d4fdf3b
    0xd1b71758e219652bu64,  // (ten2mx >> 64) = d1b71758e219652b
    0xa7c5ac471b478423u64,  // (ten2mx >> 64) = a7c5ac471b478423
    0x8637bd05af6c69b5u64,  // (ten2mx >> 64) = 8637bd05af6c69b5
    0xd6bf94d5e57a42bcu64,  // (ten2mx >> 64) = d6bf94d5e57a42bc
    0xabcc77118461cefcu64,  // (ten2mx >> 64) = abcc77118461cefc
    0x89705f4136b4a597u64,  // (ten2mx >> 64) = 89705f4136b4a597
    0xdbe6fecebdedd5beu64,  // (ten2mx >> 64) = dbe6fecebdedd5be
    0xafebff0bcb24aafeu64,  // (ten2mx >> 64) = afebff0bcb24aafe
    0x8cbccc096f5088cbu64,  // (ten2mx >> 64) = 8cbccc096f5088cb
    0xe12e13424bb40e13u64,  // (ten2mx >> 64) = e12e13424bb40e13
    0xb424dc35095cd80fu64,  // (ten2mx >> 64) = b424dc35095cd80f
    0x901d7cf73ab0acd9u64,  // (ten2mx >> 64) = 901d7cf73ab0acd9
    0xe69594bec44de15bu64,  // (ten2mx >> 64) = e69594bec44de15b
    0xb877aa3236a4b449u64   // (ten2mx >> 64) = b877aa3236a4b449
];

// Note: 128-bit tables generated with ten2mx128.ma; output in ten2mx128.out
// The order of the 64-bit components is L, H

/// Kx from 10^(-x) ~= Kx * 2^(-Ex); Kx rounded up to 128 bits, 1 <= x <= 37
pub (crate) const bid_Kx128: [BID_UINT128; 37] = [
    BID_UINT128 { w: [0xcccccccccccccccdu64, 0xccccccccccccccccu64] }, // 10^-1 ~= cccccccccccccccccccccccccccccccd * 2^-131
    BID_UINT128 { w: [0x3d70a3d70a3d70a4u64, 0xa3d70a3d70a3d70au64] }, // 10^-2 ~= a3d70a3d70a3d70a3d70a3d70a3d70a4 * 2^-134
    BID_UINT128 { w: [0x645a1cac083126eau64, 0x83126e978d4fdf3bu64] }, // 10^-3 ~= 83126e978d4fdf3b645a1cac083126ea * 2^-137
    BID_UINT128 { w: [0xd3c36113404ea4a9u64, 0xd1b71758e219652bu64] }, // 10^-4 ~= d1b71758e219652bd3c36113404ea4a9 * 2^-141
    BID_UINT128 { w: [0x0fcf80dc33721d54u64, 0xa7c5ac471b478423u64] }, // 10^-5 ~= a7c5ac471b4784230fcf80dc33721d54 * 2^-144
    BID_UINT128 { w: [0xa63f9a49c2c1b110u64, 0x8637bd05af6c69b5u64] }, // 10^-6 ~= 8637bd05af6c69b5a63f9a49c2c1b110 * 2^-147
    BID_UINT128 { w: [0x3d32907604691b4du64, 0xd6bf94d5e57a42bcu64] }, // 10^-7 ~= d6bf94d5e57a42bc3d32907604691b4d * 2^-151
    BID_UINT128 { w: [0xfdc20d2b36ba7c3eu64, 0xabcc77118461cefcu64] }, // 10^-8 ~= abcc77118461cefcfdc20d2b36ba7c3e * 2^-154
    BID_UINT128 { w: [0x31680a88f8953031u64, 0x89705f4136b4a597u64] }, // 10^-9 ~= 89705f4136b4a59731680a88f8953031 * 2^-157
    BID_UINT128 { w: [0xb573440e5a884d1cu64, 0xdbe6fecebdedd5beu64] }, // 10^-10 ~= dbe6fecebdedd5beb573440e5a884d1c * 2^-161
    BID_UINT128 { w: [0xf78f69a51539d749u64, 0xafebff0bcb24aafeu64] }, // 10^-11 ~= afebff0bcb24aafef78f69a51539d749 * 2^-164
    BID_UINT128 { w: [0xf93f87b7442e45d4u64, 0x8cbccc096f5088cbu64] }, // 10^-12 ~= 8cbccc096f5088cbf93f87b7442e45d4 * 2^-167
    BID_UINT128 { w: [0x2865a5f206b06fbau64, 0xe12e13424bb40e13u64] }, // 10^-13 ~= e12e13424bb40e132865a5f206b06fba * 2^-171
    BID_UINT128 { w: [0x538484c19ef38c95u64, 0xb424dc35095cd80fu64] }, // 10^-14 ~= b424dc35095cd80f538484c19ef38c95 * 2^-174
    BID_UINT128 { w: [0x0f9d37014bf60a11u64, 0x901d7cf73ab0acd9u64] }, // 10^-15 ~= 901d7cf73ab0acd90f9d37014bf60a11 * 2^-177
    BID_UINT128 { w: [0x4c2ebe687989a9b4u64, 0xe69594bec44de15bu64] }, // 10^-16 ~= e69594bec44de15b4c2ebe687989a9b4 * 2^-181
    BID_UINT128 { w: [0x09befeb9fad487c3u64, 0xb877aa3236a4b449u64] }, // 10^-17 ~= b877aa3236a4b44909befeb9fad487c3 * 2^-184
    BID_UINT128 { w: [0x3aff322e62439fd0u64, 0x9392ee8e921d5d07u64] }, // 10^-18 ~= 9392ee8e921d5d073aff322e62439fd0 * 2^-187
    BID_UINT128 { w: [0x2b31e9e3d06c32e6u64, 0xec1e4a7db69561a5u64] }, // 10^-19 ~= ec1e4a7db69561a52b31e9e3d06c32e6 * 2^-191
    BID_UINT128 { w: [0x88f4bb1ca6bcf585u64, 0xbce5086492111aeau64] }, // 10^-20 ~= bce5086492111aea88f4bb1ca6bcf585 * 2^-194
    BID_UINT128 { w: [0xd3f6fc16ebca5e04u64, 0x971da05074da7beeu64] }, // 10^-21 ~= 971da05074da7beed3f6fc16ebca5e04 * 2^-197
    BID_UINT128 { w: [0x5324c68b12dd6339u64, 0xf1c90080baf72cb1u64] }, // 10^-22 ~= f1c90080baf72cb15324c68b12dd6339 * 2^-201
    BID_UINT128 { w: [0x75b7053c0f178294u64, 0xc16d9a0095928a27u64] }, // 10^-23 ~= c16d9a0095928a2775b7053c0f178294 * 2^-204
    BID_UINT128 { w: [0xc4926a9672793543u64, 0x9abe14cd44753b52u64] }, // 10^-24 ~= 9abe14cd44753b52c4926a9672793543 * 2^-207
    BID_UINT128 { w: [0x3a83ddbd83f52205u64, 0xf79687aed3eec551u64] }, // 10^-25 ~= f79687aed3eec5513a83ddbd83f52205 * 2^-211
    BID_UINT128 { w: [0x95364afe032a819eu64, 0xc612062576589ddau64] }, // 10^-26 ~= c612062576589dda95364afe032a819e * 2^-214
    BID_UINT128 { w: [0x775ea264cf55347eu64, 0x9e74d1b791e07e48u64] }, // 10^-27 ~= 9e74d1b791e07e48775ea264cf55347e * 2^-217
    BID_UINT128 { w: [0x8bca9d6e188853fdu64, 0xfd87b5f28300ca0du64] }, // 10^-28 ~= fd87b5f28300ca0d8bca9d6e188853fd * 2^-221
    BID_UINT128 { w: [0x096ee45813a04331u64, 0xcad2f7f5359a3b3eu64] }, // 10^-29 ~= cad2f7f5359a3b3e096ee45813a04331 * 2^-224
    BID_UINT128 { w: [0xa1258379a94d028eu64, 0xa2425ff75e14fc31u64] }, // 10^-30 ~= a2425ff75e14fc31a1258379a94d028e * 2^-227
    BID_UINT128 { w: [0x80eacf948770ced8u64, 0x81ceb32c4b43fcf4u64] }, // 10^-31 ~= 81ceb32c4b43fcf480eacf948770ced8 * 2^-230
    BID_UINT128 { w: [0x67de18eda5814af3u64, 0xcfb11ead453994bau64] }, // 10^-32 ~= cfb11ead453994ba67de18eda5814af3 * 2^-234
    BID_UINT128 { w: [0xecb1ad8aeacdd58fu64, 0xa6274bbdd0fadd61u64] }, // 10^-33 ~= a6274bbdd0fadd61ecb1ad8aeacdd58f * 2^-237
    BID_UINT128 { w: [0xbd5af13bef0b113fu64, 0x84ec3c97da624ab4u64] }, // 10^-34 ~= 84ec3c97da624ab4bd5af13bef0b113f * 2^-240
    BID_UINT128 { w: [0x955e4ec64b44e865u64, 0xd4ad2dbfc3d07787u64] }, // 10^-35 ~= d4ad2dbfc3d07787955e4ec64b44e865 * 2^-244
    BID_UINT128 { w: [0xdde50bd1d5d0b9eau64, 0xaa242499697392d2u64] }, // 10^-36 ~= aa242499697392d2dde50bd1d5d0b9ea * 2^-247
    BID_UINT128 { w: [0x7e50d64177da2e55u64, 0x881cea14545c7575u64] }  // 10^-37 ~= 881cea14545c75757e50d64177da2e55 * 2^-250
];

/// Ex-128 from 10^(-x) ~= Kx*2^(-Ex); Kx rounded up to 128 bits, 1 <= x <= 37
pub (crate) const bid_Ex128m128: [u32; 37] = [
    3,   // 131 - 128, Ex = 131
    6,   // 134 - 128, Ex = 134
    9,   // 137 - 128, Ex = 137
    13,  // 141 - 128, Ex = 141
    16,  // 144 - 128, Ex = 144
    19,  // 147 - 128, Ex = 147
    23,  // 151 - 128, Ex = 151
    26,  // 154 - 128, Ex = 154
    29,  // 157 - 128, Ex = 157
    33,  // 161 - 128, Ex = 161
    36,  // 164 - 128, Ex = 164
    39,  // 167 - 128, Ex = 167
    43,  // 171 - 128, Ex = 171
    46,  // 174 - 128, Ex = 174
    49,  // 177 - 128, Ex = 177
    53,  // 181 - 128, Ex = 181
    56,  // 184 - 128, Ex = 184
    59,  // 187 - 128, Ex = 187
    63,  // 191 - 128, Ex = 191
    2,   // 194 - 192, Ex = 194
    5,   // 197 - 192, Ex = 197
    9,   // 201 - 192, Ex = 201
    12,  // 204 - 192, Ex = 204
    15,  // 207 - 192, Ex = 207
    19,  // 211 - 192, Ex = 211
    22,  // 214 - 192, Ex = 214
    25,  // 217 - 192, Ex = 217
    29,  // 221 - 192, Ex = 221
    32,  // 224 - 192, Ex = 224
    35,  // 227 - 192, Ex = 227
    38,  // 230 - 192, Ex = 230
    42,  // 234 - 192, Ex = 234
    45,  // 237 - 192, Ex = 237
    48,  // 240 - 192, Ex = 240
    52,  // 244 - 192, Ex = 244
    55,  // 247 - 192, Ex = 247
    58   // 250 - 192, Ex = 250
];

/// Values of 1/2 in the right position to be compared with the fraction from
/// C * kx, 1 <= x <= 37; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the high 128 bits of the fraction)
pub (crate) const bid_half128: [BID_UINT64; 37] = [
    0x0000000000000004u64,  // half / 2^128 = 4
    0x0000000000000020u64,  // half / 2^128 = 20
    0x0000000000000100u64,  // half / 2^128 = 100
    0x0000000000001000u64,  // half / 2^128 = 1000
    0x0000000000008000u64,  // half / 2^128 = 8000
    0x0000000000040000u64,  // half / 2^128 = 40000
    0x0000000000400000u64,  // half / 2^128 = 400000
    0x0000000002000000u64,  // half / 2^128 = 2000000
    0x0000000010000000u64,  // half / 2^128 = 10000000
    0x0000000100000000u64,  // half / 2^128 = 100000000
    0x0000000800000000u64,  // half / 2^128 = 800000000
    0x0000004000000000u64,  // half / 2^128 = 4000000000
    0x0000040000000000u64,  // half / 2^128 = 40000000000
    0x0000200000000000u64,  // half / 2^128 = 200000000000
    0x0001000000000000u64,  // half / 2^128 = 1000000000000
    0x0010000000000000u64,  // half / 2^128 = 10000000000000
    0x0080000000000000u64,  // half / 2^128 = 80000000000000
    0x0400000000000000u64,  // half / 2^128 = 400000000000000
    0x4000000000000000u64,  // half / 2^128 = 4000000000000000
    0x0000000000000002u64,  // half / 2^192 = 2
    0x0000000000000010u64,  // half / 2^192 = 10
    0x0000000000000100u64,  // half / 2^192 = 100
    0x0000000000000800u64,  // half / 2^192 = 800
    0x0000000000004000u64,  // half / 2^192 = 4000
    0x0000000000040000u64,  // half / 2^192 = 40000
    0x0000000000200000u64,  // half / 2^192 = 200000
    0x0000000001000000u64,  // half / 2^192 = 1000000
    0x0000000010000000u64,  // half / 2^192 = 10000000
    0x0000000080000000u64,  // half / 2^192 = 80000000
    0x0000000400000000u64,  // half / 2^192 = 400000000
    0x0000002000000000u64,  // half / 2^192 = 2000000000
    0x0000020000000000u64,  // half / 2^192 = 20000000000
    0x0000100000000000u64,  // half / 2^192 = 100000000000
    0x0000800000000000u64,  // half / 2^192 = 800000000000
    0x0008000000000000u64,  // half / 2^192 = 8000000000000
    0x0040000000000000u64,  // half / 2^192 = 40000000000000
    0x0200000000000000u64   // half / 2^192 = 200000000000000
];

/// Values of mask in the right position to obtain the high Ex - 128 or Ex - 192
/// bits of the fraction from C * kx, 1 <= x <= 37; the fraction consists of
/// the low Ex bits in C * kx
pub (crate) const bid_mask128: [BID_UINT64; 37] = [
    0x0000000000000007u64,  // mask / 2^128
    0x000000000000003fu64,  // mask / 2^128
    0x00000000000001ffu64,  // mask / 2^128
    0x0000000000001fffu64,  // mask / 2^128
    0x000000000000ffffu64,  // mask / 2^128
    0x000000000007ffffu64,  // mask / 2^128
    0x00000000007fffffu64,  // mask / 2^128
    0x0000000003ffffffu64,  // mask / 2^128
    0x000000001fffffffu64,  // mask / 2^128
    0x00000001ffffffffu64,  // mask / 2^128
    0x0000000fffffffffu64,  // mask / 2^128
    0x0000007fffffffffu64,  // mask / 2^128
    0x000007ffffffffffu64,  // mask / 2^128
    0x00003fffffffffffu64,  // mask / 2^128
    0x0001ffffffffffffu64,  // mask / 2^128
    0x001fffffffffffffu64,  // mask / 2^128
    0x00ffffffffffffffu64,  // mask / 2^128
    0x07ffffffffffffffu64,  // mask / 2^128
    0x7fffffffffffffffu64,  // mask / 2^128
    0x0000000000000003u64,  // mask / 2^192
    0x000000000000001fu64,  // mask / 2^192
    0x00000000000001ffu64,  // mask / 2^192
    0x0000000000000fffu64,  // mask / 2^192
    0x0000000000007fffu64,  // mask / 2^192
    0x000000000007ffffu64,  // mask / 2^192
    0x00000000003fffffu64,  // mask / 2^192
    0x0000000001ffffffu64,  // mask / 2^192
    0x000000001fffffffu64,  // mask / 2^192
    0x00000000ffffffffu64,  // mask / 2^192
    0x00000007ffffffffu64,  // mask / 2^192
    0x0000003fffffffffu64,  // mask / 2^192
    0x000003ffffffffffu64,  // mask / 2^192
    0x00001fffffffffffu64,  // mask / 2^192
    0x0000ffffffffffffu64,  // mask / 2^192
    0x000fffffffffffffu64,  // mask / 2^192
    0x007fffffffffffffu64,  // mask / 2^192
    0x03ffffffffffffffu64   // mask / 2^192
];

/// Values of 10^(-x) trancated to Ex bits beyond the binary point, and
/// in the right position to be compared with the fraction from C * kx,
/// 1 <= x <= 37; the fraction consists of the low Ex bits in C * kx
/// (these values are aligned with the low 128 bits of the fraction)
pub (crate) const bid_ten2mxtrunc128: [BID_UINT128; 37] = [
    BID_UINT128 { w: [0xccccccccccccccccu64, 0xccccccccccccccccu64] }, // (ten2mx >> 128) = cccccccccccccccccccccccccccccccc
    BID_UINT128 { w: [0x3d70a3d70a3d70a3u64, 0xa3d70a3d70a3d70au64] }, // (ten2mx >> 128) = a3d70a3d70a3d70a3d70a3d70a3d70a3
    BID_UINT128 { w: [0x645a1cac083126e9u64, 0x83126e978d4fdf3bu64] }, // (ten2mx >> 128) = 83126e978d4fdf3b645a1cac083126e9
    BID_UINT128 { w: [0xd3c36113404ea4a8u64, 0xd1b71758e219652bu64] }, // (ten2mx >> 128) = d1b71758e219652bd3c36113404ea4a8
    BID_UINT128 { w: [0x0fcf80dc33721d53u64, 0xa7c5ac471b478423u64] }, // (ten2mx >> 128) = a7c5ac471b4784230fcf80dc33721d53
    BID_UINT128 { w: [0xa63f9a49c2c1b10fu64, 0x8637bd05af6c69b5u64] }, // (ten2mx >> 128) = 8637bd05af6c69b5a63f9a49c2c1b10f
    BID_UINT128 { w: [0x3d32907604691b4cu64, 0xd6bf94d5e57a42bcu64] }, // (ten2mx >> 128) = d6bf94d5e57a42bc3d32907604691b4c
    BID_UINT128 { w: [0xfdc20d2b36ba7c3du64, 0xabcc77118461cefcu64] }, // (ten2mx >> 128) = abcc77118461cefcfdc20d2b36ba7c3d
    BID_UINT128 { w: [0x31680a88f8953030u64, 0x89705f4136b4a597u64] }, // (ten2mx >> 128) = 89705f4136b4a59731680a88f8953030
    BID_UINT128 { w: [0xb573440e5a884d1bu64, 0xdbe6fecebdedd5beu64] }, // (ten2mx >> 128) = dbe6fecebdedd5beb573440e5a884d1b
    BID_UINT128 { w: [0xf78f69a51539d748u64, 0xafebff0bcb24aafeu64] }, // (ten2mx >> 128) = afebff0bcb24aafef78f69a51539d748
    BID_UINT128 { w: [0xf93f87b7442e45d3u64, 0x8cbccc096f5088cbu64] }, // (ten2mx >> 128) = 8cbccc096f5088cbf93f87b7442e45d3
    BID_UINT128 { w: [0x2865a5f206b06fb9u64, 0xe12e13424bb40e13u64] }, // (ten2mx >> 128) = e12e13424bb40e132865a5f206b06fb9
    BID_UINT128 { w: [0x538484c19ef38c94u64, 0xb424dc35095cd80fu64] }, // (ten2mx >> 128) = b424dc35095cd80f538484c19ef38c94
    BID_UINT128 { w: [0x0f9d37014bf60a10u64, 0x901d7cf73ab0acd9u64] }, // (ten2mx >> 128) = 901d7cf73ab0acd90f9d37014bf60a10
    BID_UINT128 { w: [0x4c2ebe687989a9b3u64, 0xe69594bec44de15bu64] }, // (ten2mx >> 128) = e69594bec44de15b4c2ebe687989a9b3
    BID_UINT128 { w: [0x09befeb9fad487c2u64, 0xb877aa3236a4b449u64] }, // (ten2mx >> 128) = b877aa3236a4b44909befeb9fad487c2
    BID_UINT128 { w: [0x3aff322e62439fcfu64, 0x9392ee8e921d5d07u64] }, // (ten2mx >> 128) = 9392ee8e921d5d073aff322e62439fcf
    BID_UINT128 { w: [0x2b31e9e3d06c32e5u64, 0xec1e4a7db69561a5u64] }, // (ten2mx >> 128) = ec1e4a7db69561a52b31e9e3d06c32e5
    BID_UINT128 { w: [0x88f4bb1ca6bcf584u64, 0xbce5086492111aeau64] }, // (ten2mx >> 128) = bce5086492111aea88f4bb1ca6bcf584
    BID_UINT128 { w: [0xd3f6fc16ebca5e03u64, 0x971da05074da7beeu64] }, // (ten2mx >> 128) = 971da05074da7beed3f6fc16ebca5e03
    BID_UINT128 { w: [0x5324c68b12dd6338u64, 0xf1c90080baf72cb1u64] }, // (ten2mx >> 128) = f1c90080baf72cb15324c68b12dd6338
    BID_UINT128 { w: [0x75b7053c0f178293u64, 0xc16d9a0095928a27u64] }, // (ten2mx >> 128) = c16d9a0095928a2775b7053c0f178293
    BID_UINT128 { w: [0xc4926a9672793542u64, 0x9abe14cd44753b52u64] }, // (ten2mx >> 128) = 9abe14cd44753b52c4926a9672793542
    BID_UINT128 { w: [0x3a83ddbd83f52204u64, 0xf79687aed3eec551u64] }, // (ten2mx >> 128) = f79687aed3eec5513a83ddbd83f52204
    BID_UINT128 { w: [0x95364afe032a819du64, 0xc612062576589ddau64] }, // (ten2mx >> 128) = c612062576589dda95364afe032a819d
    BID_UINT128 { w: [0x775ea264cf55347du64, 0x9e74d1b791e07e48u64] }, // (ten2mx >> 128) = 9e74d1b791e07e48775ea264cf55347d
    BID_UINT128 { w: [0x8bca9d6e188853fcu64, 0xfd87b5f28300ca0du64] }, // (ten2mx >> 128) = fd87b5f28300ca0d8bca9d6e188853fc
    BID_UINT128 { w: [0x096ee45813a04330u64, 0xcad2f7f5359a3b3eu64] }, // (ten2mx >> 128) = cad2f7f5359a3b3e096ee45813a04330
    BID_UINT128 { w: [0xa1258379a94d028du64, 0xa2425ff75e14fc31u64] }, // (ten2mx >> 128) = a2425ff75e14fc31a1258379a94d028d
    BID_UINT128 { w: [0x80eacf948770ced7u64, 0x81ceb32c4b43fcf4u64] }, // (ten2mx >> 128) = 81ceb32c4b43fcf480eacf948770ced7
    BID_UINT128 { w: [0x67de18eda5814af2u64, 0xcfb11ead453994bau64] }, // (ten2mx >> 128) = cfb11ead453994ba67de18eda5814af2
    BID_UINT128 { w: [0xecb1ad8aeacdd58eu64, 0xa6274bbdd0fadd61u64] }, // (ten2mx >> 128) = a6274bbdd0fadd61ecb1ad8aeacdd58e
    BID_UINT128 { w: [0xbd5af13bef0b113eu64, 0x84ec3c97da624ab4u64] }, // (ten2mx >> 128) = 84ec3c97da624ab4bd5af13bef0b113e
    BID_UINT128 { w: [0x955e4ec64b44e864u64, 0xd4ad2dbfc3d07787u64] }, // (ten2mx >> 128) = d4ad2dbfc3d07787955e4ec64b44e864
    BID_UINT128 { w: [0xdde50bd1d5d0b9e9u64, 0xaa242499697392d2u64] }, // (ten2mx >> 128) = aa242499697392d2dde50bd1d5d0b9e9
    BID_UINT128 { w: [0x7e50d64177da2e54u64, 0x881cea14545c7575u64] }  // (ten2mx >> 128) = 881cea14545c75757e50d64177da2e54
];

pub (crate) const bid_Kx192: [BID_UINT192; 56] = [
    BID_UINT192 { w: [0xcccccccccccccccdu64, 0xccccccccccccccccu64, 0xccccccccccccccccu64] }, // 10^-1 ~= cccccccccccccccccccccccccccccccccccccccccccccccd * 2^-195
    BID_UINT192 { w: [0xd70a3d70a3d70a3eu64, 0x3d70a3d70a3d70a3u64, 0xa3d70a3d70a3d70au64] }, // 10^-2 ~= a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3e * 2^-198
    BID_UINT192 { w: [0x78d4fdf3b645a1cbu64, 0x645a1cac083126e9u64, 0x83126e978d4fdf3bu64] }, // 10^-3 ~= 83126e978d4fdf3b645a1cac083126e978d4fdf3b645a1cb * 2^-201
    BID_UINT192 { w: [0xc154c985f06f6945u64, 0xd3c36113404ea4a8u64, 0xd1b71758e219652bu64] }, // 10^-4 ~= d1b71758e219652bd3c36113404ea4a8c154c985f06f6945 * 2^-205
    BID_UINT192 { w: [0xcddd6e04c0592104u64, 0x0fcf80dc33721d53u64, 0xa7c5ac471b478423u64] }, // 10^-5 ~= a7c5ac471b4784230fcf80dc33721d53cddd6e04c0592104 * 2^-208
    BID_UINT192 { w: [0xd7e45803cd141a6au64, 0xa63f9a49c2c1b10fu64, 0x8637bd05af6c69b5u64] }, // 10^-6 ~= 8637bd05af6c69b5a63f9a49c2c1b10fd7e45803cd141a6a * 2^-211
    BID_UINT192 { w: [0x8ca08cd2e1b9c3dcu64, 0x3d32907604691b4cu64, 0xd6bf94d5e57a42bcu64] }, // 10^-7 ~= d6bf94d5e57a42bc3d32907604691b4c8ca08cd2e1b9c3dc * 2^-215
    BID_UINT192 { w: [0x3d4d3d758161697du64, 0xfdc20d2b36ba7c3du64, 0xabcc77118461cefcu64] }, // 10^-8 ~= abcc77118461cefcfdc20d2b36ba7c3d3d4d3d758161697d * 2^-218
    BID_UINT192 { w: [0xfdd7645e011abacau64, 0x31680a88f8953030u64, 0x89705f4136b4a597u64] }, // 10^-9 ~= 89705f4136b4a59731680a88f8953030fdd7645e011abaca * 2^-221
    BID_UINT192 { w: [0x2fbf06fcce912addu64, 0xb573440e5a884d1bu64, 0xdbe6fecebdedd5beu64] }, // 10^-10 ~= dbe6fecebdedd5beb573440e5a884d1b2fbf06fcce912add * 2^-225
    BID_UINT192 { w: [0xf2ff38ca3eda88b1u64, 0xf78f69a51539d748u64, 0xafebff0bcb24aafeu64] }, // 10^-11 ~= afebff0bcb24aafef78f69a51539d748f2ff38ca3eda88b1 * 2^-228
    BID_UINT192 { w: [0xf598fa3b657ba08eu64, 0xf93f87b7442e45d3u64, 0x8cbccc096f5088cbu64] }, // 10^-12 ~= 8cbccc096f5088cbf93f87b7442e45d3f598fa3b657ba08e * 2^-231
    BID_UINT192 { w: [0x88f4c3923bf900e3u64, 0x2865a5f206b06fb9u64, 0xe12e13424bb40e13u64] }, // 10^-13 ~= e12e13424bb40e132865a5f206b06fb988f4c3923bf900e3 * 2^-235
    BID_UINT192 { w: [0x6d909c74fcc733e9u64, 0x538484c19ef38c94u64, 0xb424dc35095cd80fu64] }, // 10^-14 ~= b424dc35095cd80f538484c19ef38c946d909c74fcc733e9 * 2^-238
    BID_UINT192 { w: [0x57a6e390ca38f654u64, 0x0f9d37014bf60a10u64, 0x901d7cf73ab0acd9u64] }, // 10^-15 ~= 901d7cf73ab0acd90f9d37014bf60a1057a6e390ca38f654 * 2^-241
    BID_UINT192 { w: [0xbf716c1add27f086u64, 0x4c2ebe687989a9b3u64, 0xe69594bec44de15bu64] }, // 10^-16 ~= e69594bec44de15b4c2ebe687989a9b3bf716c1add27f086 * 2^-245
    BID_UINT192 { w: [0xff8df0157db98d38u64, 0x09befeb9fad487c2u64, 0xb877aa3236a4b449u64] }, // 10^-17 ~= b877aa3236a4b44909befeb9fad487c2ff8df0157db98d38 * 2^-248
    BID_UINT192 { w: [0x32d7f344649470fau64, 0x3aff322e62439fcfu64, 0x9392ee8e921d5d07u64] }, // 10^-18 ~= 9392ee8e921d5d073aff322e62439fcf32d7f344649470fa * 2^-251
    BID_UINT192 { w: [0x1e2652070753e7f5u64, 0x2b31e9e3d06c32e5u64, 0xec1e4a7db69561a5u64] }, // 10^-19 ~= ec1e4a7db69561a52b31e9e3d06c32e51e2652070753e7f5 * 2^-255
    BID_UINT192 { w: [0x181ea8059f76532bu64, 0x88f4bb1ca6bcf584u64, 0xbce5086492111aeau64] }, // 10^-20 ~= bce5086492111aea88f4bb1ca6bcf584181ea8059f76532b * 2^-258
    BID_UINT192 { w: [0x467eecd14c5ea8efu64, 0xd3f6fc16ebca5e03u64, 0x971da05074da7beeu64] }, // 10^-21 ~= 971da05074da7beed3f6fc16ebca5e03467eecd14c5ea8ef * 2^-261
    BID_UINT192 { w: [0x70cb148213caa7e5u64, 0x5324c68b12dd6338u64, 0xf1c90080baf72cb1u64] }, // 10^-22 ~= f1c90080baf72cb15324c68b12dd633870cb148213caa7e5 * 2^-265
    BID_UINT192 { w: [0x8d6f439b43088651u64, 0x75b7053c0f178293u64, 0xc16d9a0095928a27u64] }, // 10^-23 ~= c16d9a0095928a2775b7053c0f1782938d6f439b43088651 * 2^-268
    BID_UINT192 { w: [0xd78c3615cf3a050du64, 0xc4926a9672793542u64, 0x9abe14cd44753b52u64] }, // 10^-24 ~= 9abe14cd44753b52c4926a9672793542d78c3615cf3a050d * 2^-271
    BID_UINT192 { w: [0x8c1389bc7ec33b48u64, 0x3a83ddbd83f52204u64, 0xf79687aed3eec551u64] }, // 10^-25 ~= f79687aed3eec5513a83ddbd83f522048c1389bc7ec33b48 * 2^-275
    BID_UINT192 { w: [0x3cdc6e306568fc3au64, 0x95364afe032a819du64, 0xc612062576589ddau64] }, // 10^-26 ~= c612062576589dda95364afe032a819d3cdc6e306568fc3a * 2^-278
    BID_UINT192 { w: [0xca49f1c05120c9c8u64, 0x775ea264cf55347du64, 0x9e74d1b791e07e48u64] }, // 10^-27 ~= 9e74d1b791e07e48775ea264cf55347dca49f1c05120c9c8 * 2^-281
    BID_UINT192 { w: [0x76dcb60081ce0fa6u64, 0x8bca9d6e188853fcu64, 0xfd87b5f28300ca0du64] }, // 10^-28 ~= fd87b5f28300ca0d8bca9d6e188853fc76dcb60081ce0fa6 * 2^-285
    BID_UINT192 { w: [0x5f16f80067d80c85u64, 0x096ee45813a04330u64, 0xcad2f7f5359a3b3eu64] }, // 10^-29 ~= cad2f7f5359a3b3e096ee45813a043305f16f80067d80c85 * 2^-288
    BID_UINT192 { w: [0x18df2ccd1fe00a04u64, 0xa1258379a94d028du64, 0xa2425ff75e14fc31u64] }, // 10^-30 ~= a2425ff75e14fc31a1258379a94d028d18df2ccd1fe00a04 * 2^-291
    BID_UINT192 { w: [0x4718f0a419800803u64, 0x80eacf948770ced7u64, 0x81ceb32c4b43fcf4u64] }, // 10^-31 ~= 81ceb32c4b43fcf480eacf948770ced74718f0a419800803 * 2^-294
    BID_UINT192 { w: [0x0b5b1aa028ccd99fu64, 0x67de18eda5814af2u64, 0xcfb11ead453994bau64] }, // 10^-32 ~= cfb11ead453994ba67de18eda5814af20b5b1aa028ccd99f * 2^-298
    BID_UINT192 { w: [0x6f7c154ced70ae19u64, 0xecb1ad8aeacdd58eu64, 0xa6274bbdd0fadd61u64] }, // 10^-33 ~= a6274bbdd0fadd61ecb1ad8aeacdd58e6f7c154ced70ae19 * 2^-301
    BID_UINT192 { w: [0xbf967770bdf3be7au64, 0xbd5af13bef0b113eu64, 0x84ec3c97da624ab4u64] }, // 10^-34 ~= 84ec3c97da624ab4bd5af13bef0b113ebf967770bdf3be7a * 2^-304
    BID_UINT192 { w: [0x65bd8be79652ca5du64, 0x955e4ec64b44e864u64, 0xd4ad2dbfc3d07787u64] }, // 10^-35 ~= d4ad2dbfc3d07787955e4ec64b44e86465bd8be79652ca5d * 2^-308
    BID_UINT192 { w: [0xeafe098611dbd517u64, 0xdde50bd1d5d0b9e9u64, 0xaa242499697392d2u64] }, // 10^-36 ~= aa242499697392d2dde50bd1d5d0b9e9eafe098611dbd517 * 2^-311
    BID_UINT192 { w: [0xbbfe6e04db164413u64, 0x7e50d64177da2e54u64, 0x881cea14545c7575u64] }, // 10^-37 ~= 881cea14545c75757e50d64177da2e54bbfe6e04db164413 * 2^-314
    BID_UINT192 { w: [0x2cca49a15e8a0684u64, 0x96e7bd358c904a21u64, 0xd9c7dced53c72255u64] }, // 10^-38 ~= d9c7dced53c7225596e7bd358c904a212cca49a15e8a0684 * 2^-318
    BID_UINT192 { w: [0x8a3b6e1ab2080537u64, 0xabec975e0a0d081au64, 0xae397d8aa96c1b77u64] }, // 10^-39 ~= ae397d8aa96c1b77abec975e0a0d081a8a3b6e1ab2080537 * 2^-321
    BID_UINT192 { w: [0x3b62be7bc1a0042cu64, 0x2323ac4b3b3da015u64, 0x8b61313bbabce2c6u64] }, // 10^-40 ~= 8b61313bbabce2c62323ac4b3b3da0153b62be7bc1a0042c * 2^-324
    BID_UINT192 { w: [0x5f0463f935ccd379u64, 0x6b6c46dec52f6688u64, 0xdf01e85f912e37a3u64] }, // 10^-41 ~= df01e85f912e37a36b6c46dec52f66885f0463f935ccd379 * 2^-328
    BID_UINT192 { w: [0x7f36b660f7d70f94u64, 0x55f038b237591ed3u64, 0xb267ed1940f1c61cu64] }, // 10^-42 ~= b267ed1940f1c61c55f038b237591ed37f36b660f7d70f94 * 2^-331
    BID_UINT192 { w: [0xcc2bc51a5fdf3faau64, 0x77f3608e92adb242u64, 0x8eb98a7a9a5b04e3u64] }, // 10^-43 ~= 8eb98a7a9a5b04e377f3608e92adb242cc2bc51a5fdf3faa * 2^-334
    BID_UINT192 { w: [0xe046082a32fecc42u64, 0x8cb89a7db77c506au64, 0xe45c10c42a2b3b05u64] }, // 10^-44 ~= e45c10c42a2b3b058cb89a7db77c506ae046082a32fecc42 * 2^-338
    BID_UINT192 { w: [0x4d04d354f598a368u64, 0x3d607b97c5fd0d22u64, 0xb6b00d69bb55c8d1u64] }, // 10^-45 ~= b6b00d69bb55c8d13d607b97c5fd0d224d04d354f598a368 * 2^-341
    BID_UINT192 { w: [0x3d9d75dd9146e920u64, 0xcab3961304ca70e8u64, 0x9226712162ab070du64] }, // 10^-46 ~= 9226712162ab070dcab3961304ca70e83d9d75dd9146e920 * 2^-344
    BID_UINT192 { w: [0xc8fbefc8e8717500u64, 0xaab8f01e6e10b4a6u64, 0xe9d71b689dde71afu64] }, // 10^-47 ~= e9d71b689dde71afaab8f01e6e10b4a6c8fbefc8e8717500 * 2^-348
    BID_UINT192 { w: [0x3a63263a538df734u64, 0x5560c018580d5d52u64, 0xbb127c53b17ec159u64] }, // 10^-48 ~= bb127c53b17ec1595560c018580d5d523a63263a538df734 * 2^-351
    BID_UINT192 { w: [0x2eb5b82ea93e5f5du64, 0xdde7001379a44aa8u64, 0x95a8637627989aadu64] }, // 10^-49 ~= 95a8637627989aaddde7001379a44aa82eb5b82ea93e5f5d * 2^-354
    BID_UINT192 { w: [0x4abc59e441fd6561u64, 0x963e66858f6d4440u64, 0xef73d256a5c0f77cu64] }, // 10^-50 ~= ef73d256a5c0f77c963e66858f6d44404abc59e441fd6561 * 2^-358
    BID_UINT192 { w: [0x6efd14b69b311de7u64, 0xde98520472bdd033u64, 0xbf8fdb78849a5f96u64] }, // 10^-51 ~= bf8fdb78849a5f96de98520472bdd0336efd14b69b311de7 * 2^-361
    BID_UINT192 { w: [0x259743c548f417ecu64, 0xe546a8038efe4029u64, 0x993fe2c6d07b7fabu64] }, // 10^-52 ~= 993fe2c6d07b7fabe546a8038efe4029259743c548f417ec * 2^-364
    BID_UINT192 { w: [0x3c25393ba7ecf313u64, 0xd53dd99f4b3066a8u64, 0xf53304714d9265dfu64] }, // 10^-53 ~= f53304714d9265dfd53dd99f4b3066a83c25393ba7ecf313 * 2^-368
    BID_UINT192 { w: [0x96842dc95323f5a9u64, 0xaa97e14c3c26b886u64, 0xc428d05aa4751e4cu64] }, // 10^-54 ~= c428d05aa4751e4caa97e14c3c26b88696842dc95323f5a9 * 2^-371
    BID_UINT192 { w: [0xab9cf16ddc1cc487u64, 0x55464dd69685606bu64, 0x9ced737bb6c4183du64] }, // 10^-55 ~= 9ced737bb6c4183d55464dd69685606bab9cf16ddc1cc487 * 2^-374
    BID_UINT192 { w: [0xac2e4f162cfad40bu64, 0xeed6e2f0f0d56712u64, 0xfb158592be068d2eu64] } // 10^-56 ~= fb158592be068d2eeed6e2f0f0d56712ac2e4f162cfad40b * 2^-378
];

pub (crate) const bid_Ex192m192: [u32; 56] = [
    3,   // 195 - 192, Ex = 195
    6,   // 198 - 192, Ex = 198
    9,   // 201 - 192, Ex = 201
    13,  // 205 - 192, Ex = 205
    16,  // 208 - 192, Ex = 208
    19,  // 211 - 192, Ex = 211
    23,  // 215 - 192, Ex = 215
    26,  // 218 - 192, Ex = 218
    29,  // 221 - 192, Ex = 221
    33,  // 225 - 192, Ex = 225
    36,  // 228 - 192, Ex = 228
    39,  // 231 - 192, Ex = 231
    43,  // 235 - 192, Ex = 235
    46,  // 238 - 192, Ex = 238
    49,  // 241 - 192, Ex = 241
    53,  // 245 - 192, Ex = 245
    56,  // 248 - 192, Ex = 248
    59,  // 251 - 192, Ex = 251
    63,  // 255 - 192, Ex = 255
    2,   // 258 - 256, Ex = 258
    5,   // 261 - 256, Ex = 261
    9,   // 265 - 256, Ex = 265
    12,  // 268 - 256, Ex = 268
    15,  // 271 - 256, Ex = 271
    19,  // 275 - 256, Ex = 275
    22,  // 278 - 256, Ex = 278
    25,  // 281 - 256, Ex = 281
    29,  // 285 - 256, Ex = 285
    32,  // 288 - 256, Ex = 288
    35,  // 291 - 256, Ex = 291
    38,  // 294 - 256, Ex = 294
    42,  // 298 - 256, Ex = 298
    45,  // 301 - 256, Ex = 301
    48,  // 304 - 256, Ex = 304
    52,  // 308 - 256, Ex = 308
    55,  // 311 - 256, Ex = 311
    58,  // 314 - 256, Ex = 314
    62,  // 318 - 256, Ex = 318
    1,   // 321 - 320, Ex = 321
    4,   // 324 - 320, Ex = 324
    8,   // 328 - 320, Ex = 328
    11,  // 331 - 320, Ex = 331
    14,  // 334 - 320, Ex = 334
    18,  // 338 - 320, Ex = 338
    21,  // 341 - 320, Ex = 341
    24,  // 344 - 320, Ex = 344
    28,  // 348 - 320, Ex = 348
    31,  // 351 - 320, Ex = 351
    34,  // 354 - 320, Ex = 354
    38,  // 358 - 320, Ex = 358
    41,  // 361 - 320, Ex = 361
    44,  // 364 - 320, Ex = 364
    48,  // 368 - 320, Ex = 368
    51,  // 371 - 320, Ex = 371
    54,  // 374 - 320, Ex = 374
    58   // 378 - 320, Ex = 378
];

pub (crate) const bid_half192: [BID_UINT64; 56] = [
    0x0000000000000004u64,  // half / 2^192 = 4
    0x0000000000000020u64,  // half / 2^192 = 20
    0x0000000000000100u64,  // half / 2^192 = 100
    0x0000000000001000u64,  // half / 2^192 = 1000
    0x0000000000008000u64,  // half / 2^192 = 8000
    0x0000000000040000u64,  // half / 2^192 = 40000
    0x0000000000400000u64,  // half / 2^192 = 400000
    0x0000000002000000u64,  // half / 2^192 = 2000000
    0x0000000010000000u64,  // half / 2^192 = 10000000
    0x0000000100000000u64,  // half / 2^192 = 100000000
    0x0000000800000000u64,  // half / 2^192 = 800000000
    0x0000004000000000u64,  // half / 2^192 = 4000000000
    0x0000040000000000u64,  // half / 2^192 = 40000000000
    0x0000200000000000u64,  // half / 2^192 = 200000000000
    0x0001000000000000u64,  // half / 2^192 = 1000000000000
    0x0010000000000000u64,  // half / 2^192 = 10000000000000
    0x0080000000000000u64,  // half / 2^192 = 80000000000000
    0x0400000000000000u64,  // half / 2^192 = 400000000000000
    0x4000000000000000u64,  // half / 2^192 = 4000000000000000
    0x0000000000000002u64,  // half / 2^256 = 2
    0x0000000000000010u64,  // half / 2^256 = 10
    0x0000000000000100u64,  // half / 2^256 = 100
    0x0000000000000800u64,  // half / 2^256 = 800
    0x0000000000004000u64,  // half / 2^256 = 4000
    0x0000000000040000u64,  // half / 2^256 = 40000
    0x0000000000200000u64,  // half / 2^256 = 200000
    0x0000000001000000u64,  // half / 2^256 = 1000000
    0x0000000010000000u64,  // half / 2^256 = 10000000
    0x0000000080000000u64,  // half / 2^256 = 80000000
    0x0000000400000000u64,  // half / 2^256 = 400000000
    0x0000002000000000u64,  // half / 2^256 = 2000000000
    0x0000020000000000u64,  // half / 2^256 = 20000000000
    0x0000100000000000u64,  // half / 2^256 = 100000000000
    0x0000800000000000u64,  // half / 2^256 = 800000000000
    0x0008000000000000u64,  // half / 2^256 = 8000000000000
    0x0040000000000000u64,  // half / 2^256 = 40000000000000
    0x0200000000000000u64,  // half / 2^256 = 200000000000000
    0x2000000000000000u64,  // half / 2^256 = 2000000000000000
    0x0000000000000001u64,  // half / 2^320 = 1
    0x0000000000000008u64,  // half / 2^320 = 8
    0x0000000000000080u64,  // half / 2^320 = 80
    0x0000000000000400u64,  // half / 2^320 = 400
    0x0000000000002000u64,  // half / 2^320 = 2000
    0x0000000000020000u64,  // half / 2^320 = 20000
    0x0000000000100000u64,  // half / 2^320 = 100000
    0x0000000000800000u64,  // half / 2^320 = 800000
    0x0000000008000000u64,  // half / 2^320 = 8000000
    0x0000000040000000u64,  // half / 2^320 = 40000000
    0x0000000200000000u64,  // half / 2^320 = 200000000
    0x0000002000000000u64,  // half / 2^320 = 2000000000
    0x0000010000000000u64,  // half / 2^320 = 10000000000
    0x0000080000000000u64,  // half / 2^320 = 80000000000
    0x0000800000000000u64,  // half / 2^320 = 800000000000
    0x0004000000000000u64,  // half / 2^320 = 4000000000000
    0x0020000000000000u64,  // half / 2^320 = 20000000000000
    0x0200000000000000u64   // half / 2^320 = 200000000000000
];

pub (crate) const bid_mask192: [BID_UINT64; 56] = [
    0x0000000000000007u64,  // mask / 2^192
    0x000000000000003fu64,  // mask / 2^192
    0x00000000000001ffu64,  // mask / 2^192
    0x0000000000001fffu64,  // mask / 2^192
    0x000000000000ffffu64,  // mask / 2^192
    0x000000000007ffffu64,  // mask / 2^192
    0x00000000007fffffu64,  // mask / 2^192
    0x0000000003ffffffu64,  // mask / 2^192
    0x000000001fffffffu64,  // mask / 2^192
    0x00000001ffffffffu64,  // mask / 2^192
    0x0000000fffffffffu64,  // mask / 2^192
    0x0000007fffffffffu64,  // mask / 2^192
    0x000007ffffffffffu64,  // mask / 2^192
    0x00003fffffffffffu64,  // mask / 2^192
    0x0001ffffffffffffu64,  // mask / 2^192
    0x001fffffffffffffu64,  // mask / 2^192
    0x00ffffffffffffffu64,  // mask / 2^192
    0x07ffffffffffffffu64,  // mask / 2^192
    0x7fffffffffffffffu64,  // mask / 2^192
    0x0000000000000003u64,  // mask / 2^256
    0x000000000000001fu64,  // mask / 2^256
    0x00000000000001ffu64,  // mask / 2^256
    0x0000000000000fffu64,  // mask / 2^256
    0x0000000000007fffu64,  // mask / 2^256
    0x000000000007ffffu64,  // mask / 2^256
    0x00000000003fffffu64,  // mask / 2^256
    0x0000000001ffffffu64,  // mask / 2^256
    0x000000001fffffffu64,  // mask / 2^256
    0x00000000ffffffffu64,  // mask / 2^256
    0x00000007ffffffffu64,  // mask / 2^256
    0x0000003fffffffffu64,  // mask / 2^256
    0x000003ffffffffffu64,  // mask / 2^256
    0x00001fffffffffffu64,  // mask / 2^256
    0x0000ffffffffffffu64,  // mask / 2^256
    0x000fffffffffffffu64,  // mask / 2^256
    0x007fffffffffffffu64,  // mask / 2^256
    0x03ffffffffffffffu64,  // mask / 2^256
    0x3fffffffffffffffu64,  // mask / 2^256
    0x0000000000000001u64,  // mask / 2^320
    0x000000000000000fu64,  // mask / 2^320
    0x00000000000000ffu64,  // mask / 2^320
    0x00000000000007ffu64,  // mask / 2^320
    0x0000000000003fffu64,  // mask / 2^320
    0x000000000003ffffu64,  // mask / 2^320
    0x00000000001fffffu64,  // mask / 2^320
    0x0000000000ffffffu64,  // mask / 2^320
    0x000000000fffffffu64,  // mask / 2^320
    0x000000007fffffffu64,  // mask / 2^320
    0x00000003ffffffffu64,  // mask / 2^320
    0x0000003fffffffffu64,  // mask / 2^320
    0x000001ffffffffffu64,  // mask / 2^320
    0x00000fffffffffffu64,  // mask / 2^320
    0x0000ffffffffffffu64,  // mask / 2^320
    0x0007ffffffffffffu64,  // mask / 2^320
    0x003fffffffffffffu64,  // mask / 2^320
    0x03ffffffffffffffu64   // mask / 2^320
];

pub (crate) const bid_ten2mxtrunc192: [BID_UINT192; 56] = [
    BID_UINT192 { w: [0xccccccccccccccccu64, 0xccccccccccccccccu64, 0xccccccccccccccccu64] },    // (ten2mx >> 192) = cccccccccccccccccccccccccccccccccccccccccccccccc
    BID_UINT192 { w: [0xd70a3d70a3d70a3du64, 0x3d70a3d70a3d70a3u64, 0xa3d70a3d70a3d70au64] },    // (ten2mx >> 192) = a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d70a3d
    BID_UINT192 { w: [0x78d4fdf3b645a1cau64, 0x645a1cac083126e9u64, 0x83126e978d4fdf3bu64] },    // (ten2mx >> 192) = 83126e978d4fdf3b645a1cac083126e978d4fdf3b645a1ca
    BID_UINT192 { w: [0xc154c985f06f6944u64, 0xd3c36113404ea4a8u64, 0xd1b71758e219652bu64] },    // (ten2mx >> 192) = d1b71758e219652bd3c36113404ea4a8c154c985f06f6944
    BID_UINT192 { w: [0xcddd6e04c0592103u64, 0x0fcf80dc33721d53u64, 0xa7c5ac471b478423u64] },    // (ten2mx >> 192) = a7c5ac471b4784230fcf80dc33721d53cddd6e04c0592103
    BID_UINT192 { w: [0xd7e45803cd141a69u64, 0xa63f9a49c2c1b10fu64, 0x8637bd05af6c69b5u64] },    // (ten2mx >> 192) = 8637bd05af6c69b5a63f9a49c2c1b10fd7e45803cd141a69
    BID_UINT192 { w: [0x8ca08cd2e1b9c3dbu64, 0x3d32907604691b4cu64, 0xd6bf94d5e57a42bcu64] },    // (ten2mx >> 192) = d6bf94d5e57a42bc3d32907604691b4c8ca08cd2e1b9c3db
    BID_UINT192 { w: [0x3d4d3d758161697cu64, 0xfdc20d2b36ba7c3du64, 0xabcc77118461cefcu64] },    // (ten2mx >> 192) = abcc77118461cefcfdc20d2b36ba7c3d3d4d3d758161697c
    BID_UINT192 { w: [0xfdd7645e011abac9u64, 0x31680a88f8953030u64, 0x89705f4136b4a597u64] },    // (ten2mx >> 192) = 89705f4136b4a59731680a88f8953030fdd7645e011abac9
    BID_UINT192 { w: [0x2fbf06fcce912adcu64, 0xb573440e5a884d1bu64, 0xdbe6fecebdedd5beu64] },    // (ten2mx >> 192) = dbe6fecebdedd5beb573440e5a884d1b2fbf06fcce912adc
    BID_UINT192 { w: [0xf2ff38ca3eda88b0u64, 0xf78f69a51539d748u64, 0xafebff0bcb24aafeu64] },    // (ten2mx >> 192) = afebff0bcb24aafef78f69a51539d748f2ff38ca3eda88b0
    BID_UINT192 { w: [0xf598fa3b657ba08du64, 0xf93f87b7442e45d3u64, 0x8cbccc096f5088cbu64] },    // (ten2mx >> 192) = 8cbccc096f5088cbf93f87b7442e45d3f598fa3b657ba08d
    BID_UINT192 { w: [0x88f4c3923bf900e2u64, 0x2865a5f206b06fb9u64, 0xe12e13424bb40e13u64] },    // (ten2mx >> 192) = e12e13424bb40e132865a5f206b06fb988f4c3923bf900e2
    BID_UINT192 { w: [0x6d909c74fcc733e8u64, 0x538484c19ef38c94u64, 0xb424dc35095cd80fu64] },    // (ten2mx >> 192) = b424dc35095cd80f538484c19ef38c946d909c74fcc733e8
    BID_UINT192 { w: [0x57a6e390ca38f653u64, 0x0f9d37014bf60a10u64, 0x901d7cf73ab0acd9u64] },    // (ten2mx >> 192) = 901d7cf73ab0acd90f9d37014bf60a1057a6e390ca38f653
    BID_UINT192 { w: [0xbf716c1add27f085u64, 0x4c2ebe687989a9b3u64, 0xe69594bec44de15bu64] },    // (ten2mx >> 192) = e69594bec44de15b4c2ebe687989a9b3bf716c1add27f085
    BID_UINT192 { w: [0xff8df0157db98d37u64, 0x09befeb9fad487c2u64, 0xb877aa3236a4b449u64] },    // (ten2mx >> 192) = b877aa3236a4b44909befeb9fad487c2ff8df0157db98d37
    BID_UINT192 { w: [0x32d7f344649470f9u64, 0x3aff322e62439fcfu64, 0x9392ee8e921d5d07u64] },    // (ten2mx >> 192) = 9392ee8e921d5d073aff322e62439fcf32d7f344649470f9
    BID_UINT192 { w: [0x1e2652070753e7f4u64, 0x2b31e9e3d06c32e5u64, 0xec1e4a7db69561a5u64] },    // (ten2mx >> 192) = ec1e4a7db69561a52b31e9e3d06c32e51e2652070753e7f4
    BID_UINT192 { w: [0x181ea8059f76532au64, 0x88f4bb1ca6bcf584u64, 0xbce5086492111aeau64] },    // (ten2mx >> 192) = bce5086492111aea88f4bb1ca6bcf584181ea8059f76532a
    BID_UINT192 { w: [0x467eecd14c5ea8eeu64, 0xd3f6fc16ebca5e03u64, 0x971da05074da7beeu64] },    // (ten2mx >> 192) = 971da05074da7beed3f6fc16ebca5e03467eecd14c5ea8ee
    BID_UINT192 { w: [0x70cb148213caa7e4u64, 0x5324c68b12dd6338u64, 0xf1c90080baf72cb1u64] },    // (ten2mx >> 192) = f1c90080baf72cb15324c68b12dd633870cb148213caa7e4
    BID_UINT192 { w: [0x8d6f439b43088650u64, 0x75b7053c0f178293u64, 0xc16d9a0095928a27u64] },    // (ten2mx >> 192) = c16d9a0095928a2775b7053c0f1782938d6f439b43088650
    BID_UINT192 { w: [0xd78c3615cf3a050cu64, 0xc4926a9672793542u64, 0x9abe14cd44753b52u64] },    // (ten2mx >> 192) = 9abe14cd44753b52c4926a9672793542d78c3615cf3a050c
    BID_UINT192 { w: [0x8c1389bc7ec33b47u64, 0x3a83ddbd83f52204u64, 0xf79687aed3eec551u64] },    // (ten2mx >> 192) = f79687aed3eec5513a83ddbd83f522048c1389bc7ec33b47
    BID_UINT192 { w: [0x3cdc6e306568fc39u64, 0x95364afe032a819du64, 0xc612062576589ddau64] },    // (ten2mx >> 192) = c612062576589dda95364afe032a819d3cdc6e306568fc39
    BID_UINT192 { w: [0xca49f1c05120c9c7u64, 0x775ea264cf55347du64, 0x9e74d1b791e07e48u64] },    // (ten2mx >> 192) = 9e74d1b791e07e48775ea264cf55347dca49f1c05120c9c7
    BID_UINT192 { w: [0x76dcb60081ce0fa5u64, 0x8bca9d6e188853fcu64, 0xfd87b5f28300ca0du64] },    // (ten2mx >> 192) = fd87b5f28300ca0d8bca9d6e188853fc76dcb60081ce0fa5
    BID_UINT192 { w: [0x5f16f80067d80c84u64, 0x096ee45813a04330u64, 0xcad2f7f5359a3b3eu64] },    // (ten2mx >> 192) = cad2f7f5359a3b3e096ee45813a043305f16f80067d80c84
    BID_UINT192 { w: [0x18df2ccd1fe00a03u64, 0xa1258379a94d028du64, 0xa2425ff75e14fc31u64] },    // (ten2mx >> 192) = a2425ff75e14fc31a1258379a94d028d18df2ccd1fe00a03
    BID_UINT192 { w: [0x4718f0a419800802u64, 0x80eacf948770ced7u64, 0x81ceb32c4b43fcf4u64] },    // (ten2mx >> 192) = 81ceb32c4b43fcf480eacf948770ced74718f0a419800802
    BID_UINT192 { w: [0x0b5b1aa028ccd99eu64, 0x67de18eda5814af2u64, 0xcfb11ead453994bau64] },    // (ten2mx >> 192) = cfb11ead453994ba67de18eda5814af20b5b1aa028ccd99e
    BID_UINT192 { w: [0x6f7c154ced70ae18u64, 0xecb1ad8aeacdd58eu64, 0xa6274bbdd0fadd61u64] },    // (ten2mx >> 192) = a6274bbdd0fadd61ecb1ad8aeacdd58e6f7c154ced70ae18
    BID_UINT192 { w: [0xbf967770bdf3be79u64, 0xbd5af13bef0b113eu64, 0x84ec3c97da624ab4u64] },    // (ten2mx >> 192) = 84ec3c97da624ab4bd5af13bef0b113ebf967770bdf3be79
    BID_UINT192 { w: [0x65bd8be79652ca5cu64, 0x955e4ec64b44e864u64, 0xd4ad2dbfc3d07787u64] },    // (ten2mx >> 192) = d4ad2dbfc3d07787955e4ec64b44e86465bd8be79652ca5c
    BID_UINT192 { w: [0xeafe098611dbd516u64, 0xdde50bd1d5d0b9e9u64, 0xaa242499697392d2u64] },    // (ten2mx >> 192) = aa242499697392d2dde50bd1d5d0b9e9eafe098611dbd516
    BID_UINT192 { w: [0xbbfe6e04db164412u64, 0x7e50d64177da2e54u64, 0x881cea14545c7575u64] },    // (ten2mx >> 192) = 881cea14545c75757e50d64177da2e54bbfe6e04db164412
    BID_UINT192 { w: [0x2cca49a15e8a0683u64, 0x96e7bd358c904a21u64, 0xd9c7dced53c72255u64] },    // (ten2mx >> 192) = d9c7dced53c7225596e7bd358c904a212cca49a15e8a0683
    BID_UINT192 { w: [0x8a3b6e1ab2080536u64, 0xabec975e0a0d081au64, 0xae397d8aa96c1b77u64] },    // (ten2mx >> 192) = ae397d8aa96c1b77abec975e0a0d081a8a3b6e1ab2080536
    BID_UINT192 { w: [0x3b62be7bc1a0042bu64, 0x2323ac4b3b3da015u64, 0x8b61313bbabce2c6u64] },    // (ten2mx >> 192) = 8b61313bbabce2c62323ac4b3b3da0153b62be7bc1a0042b
    BID_UINT192 { w: [0x5f0463f935ccd378u64, 0x6b6c46dec52f6688u64, 0xdf01e85f912e37a3u64] },    // (ten2mx >> 192) = df01e85f912e37a36b6c46dec52f66885f0463f935ccd378
    BID_UINT192 { w: [0x7f36b660f7d70f93u64, 0x55f038b237591ed3u64, 0xb267ed1940f1c61cu64] },    // (ten2mx >> 192) = b267ed1940f1c61c55f038b237591ed37f36b660f7d70f93
    BID_UINT192 { w: [0xcc2bc51a5fdf3fa9u64, 0x77f3608e92adb242u64, 0x8eb98a7a9a5b04e3u64] },    // (ten2mx >> 192) = 8eb98a7a9a5b04e377f3608e92adb242cc2bc51a5fdf3fa9
    BID_UINT192 { w: [0xe046082a32fecc41u64, 0x8cb89a7db77c506au64, 0xe45c10c42a2b3b05u64] },    // (ten2mx >> 192) = e45c10c42a2b3b058cb89a7db77c506ae046082a32fecc41
    BID_UINT192 { w: [0x4d04d354f598a367u64, 0x3d607b97c5fd0d22u64, 0xb6b00d69bb55c8d1u64] },    // (ten2mx >> 192) = b6b00d69bb55c8d13d607b97c5fd0d224d04d354f598a367
    BID_UINT192 { w: [0x3d9d75dd9146e91fu64, 0xcab3961304ca70e8u64, 0x9226712162ab070du64] },    // (ten2mx >> 192) = 9226712162ab070dcab3961304ca70e83d9d75dd9146e91f
    BID_UINT192 { w: [0xc8fbefc8e87174ffu64, 0xaab8f01e6e10b4a6u64, 0xe9d71b689dde71afu64] },    // (ten2mx >> 192) = e9d71b689dde71afaab8f01e6e10b4a6c8fbefc8e87174ff
    BID_UINT192 { w: [0x3a63263a538df733u64, 0x5560c018580d5d52u64, 0xbb127c53b17ec159u64] },    // (ten2mx >> 192) = bb127c53b17ec1595560c018580d5d523a63263a538df733
    BID_UINT192 { w: [0x2eb5b82ea93e5f5cu64, 0xdde7001379a44aa8u64, 0x95a8637627989aadu64] },    // (ten2mx >> 192) = 95a8637627989aaddde7001379a44aa82eb5b82ea93e5f5c
    BID_UINT192 { w: [0x4abc59e441fd6560u64, 0x963e66858f6d4440u64, 0xef73d256a5c0f77cu64] },    // (ten2mx >> 192) = ef73d256a5c0f77c963e66858f6d44404abc59e441fd6560
    BID_UINT192 { w: [0x6efd14b69b311de6u64, 0xde98520472bdd033u64, 0xbf8fdb78849a5f96u64] },    // (ten2mx >> 192) = bf8fdb78849a5f96de98520472bdd0336efd14b69b311de6
    BID_UINT192 { w: [0x259743c548f417ebu64, 0xe546a8038efe4029u64, 0x993fe2c6d07b7fabu64] },    // (ten2mx >> 192) = 993fe2c6d07b7fabe546a8038efe4029259743c548f417eb
    BID_UINT192 { w: [0x3c25393ba7ecf312u64, 0xd53dd99f4b3066a8u64, 0xf53304714d9265dfu64] },    // (ten2mx >> 192) = f53304714d9265dfd53dd99f4b3066a83c25393ba7ecf312
    BID_UINT192 { w: [0x96842dc95323f5a8u64, 0xaa97e14c3c26b886u64, 0xc428d05aa4751e4cu64] },    // (ten2mx >> 192) = c428d05aa4751e4caa97e14c3c26b88696842dc95323f5a8
    BID_UINT192 { w: [0xab9cf16ddc1cc486u64, 0x55464dd69685606bu64, 0x9ced737bb6c4183du64] },    // (ten2mx >> 192) = 9ced737bb6c4183d55464dd69685606bab9cf16ddc1cc486
    BID_UINT192 { w: [0xac2e4f162cfad40au64, 0xeed6e2f0f0d56712u64, 0xfb158592be068d2eu64] }     // (ten2mx >> 192) = fb158592be068d2eeed6e2f0f0d56712ac2e4f162cfad40a
];

pub (crate) const bid_Kx256: [BID_UINT256; 75] = [
    BID_UINT256 { w: [0xcccccccccccccccdu64, 0xccccccccccccccccu64, 0xccccccccccccccccu64, 0xccccccccccccccccu64] },
    // 10^-1 ~= cccccccccccccccc  cccccccccccccccc
    //   cccccccccccccccccccccccccccccccd   * 2^-259
    BID_UINT256 { w: [0x70a3d70a3d70a3d8u64, 0xd70a3d70a3d70a3du64, 0x3d70a3d70a3d70a3u64, 0xa3d70a3d70a3d70au64] },
    // 10^-2 ~= a3d70a3d70a3d70a  3d70a3d70a3d70a3
    //   d70a3d70a3d70a3d70a3d70a3d70a3d8   * 2^-262
    BID_UINT256 { w: [0xc083126e978d4fe0u64, 0x78d4fdf3b645a1cau64, 0x645a1cac083126e9u64, 0x83126e978d4fdf3bu64] },
    // 10^-3 ~= 83126e978d4fdf3b  645a1cac083126e9
    //   78d4fdf3b645a1cac083126e978d4fe0   * 2^-265
    BID_UINT256 { w: [0x67381d7dbf487fccu64, 0xc154c985f06f6944u64, 0xd3c36113404ea4a8u64, 0xd1b71758e219652bu64] },
    // 10^-4 ~= d1b71758e219652b  d3c36113404ea4a8
    //   c154c985f06f694467381d7dbf487fcc   * 2^-269
    BID_UINT256 { w: [0x85c67dfe32a0663du64, 0xcddd6e04c0592103u64, 0x0fcf80dc33721d53u64, 0xa7c5ac471b478423u64] },
    // 10^-5 ~= a7c5ac471b478423  fcf80dc33721d53
    //   cddd6e04c059210385c67dfe32a0663d   * 2^-272
    BID_UINT256 { w: [0x37d1fe64f54d1e97u64, 0xd7e45803cd141a69u64, 0xa63f9a49c2c1b10fu64, 0x8637bd05af6c69b5u64] },
    // 10^-6 ~= 8637bd05af6c69b5  a63f9a49c2c1b10f
    //   d7e45803cd141a6937d1fe64f54d1e97   * 2^-275
    BID_UINT256 { w: [0x8c8330a1887b6425u64, 0x8ca08cd2e1b9c3dbu64, 0x3d32907604691b4cu64, 0xd6bf94d5e57a42bcu64] },
    // 10^-7 ~= d6bf94d5e57a42bc  3d32907604691b4c
    //   8ca08cd2e1b9c3db8c8330a1887b6425   * 2^-279
    BID_UINT256 { w: [0x7068f3b46d2f8351u64, 0x3d4d3d758161697cu64, 0xfdc20d2b36ba7c3du64, 0xabcc77118461cefcu64] },
    // 10^-8 ~= abcc77118461cefc  fdc20d2b36ba7c3d
    //   3d4d3d758161697c7068f3b46d2f8351   * 2^-282
    BID_UINT256 { w: [0xf387295d242602a7u64, 0xfdd7645e011abac9u64, 0x31680a88f8953030u64, 0x89705f4136b4a597u64] },
    // 10^-9 ~= 89705f4136b4a597  31680a88f8953030
    //   fdd7645e011abac9f387295d242602a7   * 2^-285
    BID_UINT256 { w: [0xb8d8422ea03cd10bu64, 0x2fbf06fcce912adcu64, 0xb573440e5a884d1bu64, 0xdbe6fecebdedd5beu64] },
    // 10^-10 ~= dbe6fecebdedd5be  b573440e5a884d1b
    //   2fbf06fcce912adcb8d8422ea03cd10b   * 2^-289
    BID_UINT256 { w: [0x93e034f219ca40d6u64, 0xf2ff38ca3eda88b0u64, 0xf78f69a51539d748u64, 0xafebff0bcb24aafeu64] },
    // 10^-11 ~= afebff0bcb24aafe  f78f69a51539d748
    //   f2ff38ca3eda88b093e034f219ca40d6   * 2^-292
    BID_UINT256 { w: [0x4319c3f4e16e9a45u64, 0xf598fa3b657ba08du64, 0xf93f87b7442e45d3u64, 0x8cbccc096f5088cbu64] },
    // 10^-12 ~= 8cbccc096f5088cb  f93f87b7442e45d3
    //   f598fa3b657ba08d4319c3f4e16e9a45   * 2^-295
    BID_UINT256 { w: [0x04f606549be42a07u64, 0x88f4c3923bf900e2u64, 0x2865a5f206b06fb9u64, 0xe12e13424bb40e13u64] },
    // 10^-13 ~= e12e13424bb40e13  2865a5f206b06fb9
    //   88f4c3923bf900e204f606549be42a07   * 2^-299
    BID_UINT256 { w: [0x03f805107cb68806u64, 0x6d909c74fcc733e8u64, 0x538484c19ef38c94u64, 0xb424dc35095cd80fu64] },
    // 10^-14 ~= b424dc35095cd80f  538484c19ef38c94
    //   6d909c74fcc733e803f805107cb68806   * 2^-302
    BID_UINT256 { w: [0x3660040d3092066bu64, 0x57a6e390ca38f653u64, 0x0f9d37014bf60a10u64, 0x901d7cf73ab0acd9u64] },
    // 10^-15 ~= 901d7cf73ab0acd9  f9d37014bf60a10
    //   57a6e390ca38f6533660040d3092066b   * 2^-305
    BID_UINT256 { w: [0x23ccd3484db670abu64, 0xbf716c1add27f085u64, 0x4c2ebe687989a9b3u64, 0xe69594bec44de15bu64] },
    // 10^-16 ~= e69594bec44de15b  4c2ebe687989a9b3
    //   bf716c1add27f08523ccd3484db670ab   * 2^-309
    BID_UINT256 { w: [0x4fd70f6d0af85a23u64, 0xff8df0157db98d37u64, 0x09befeb9fad487c2u64, 0xb877aa3236a4b449u64] },
    // 10^-17 ~= b877aa3236a4b449  9befeb9fad487c2
    //   ff8df0157db98d374fd70f6d0af85a23   * 2^-312
    BID_UINT256 { w: [0x0cac0c573bf9e1b6u64, 0x32d7f344649470f9u64, 0x3aff322e62439fcfu64, 0x9392ee8e921d5d07u64] },
    // 10^-18 ~= 9392ee8e921d5d07  3aff322e62439fcf
    //   32d7f344649470f90cac0c573bf9e1b6   * 2^-315
    BID_UINT256 { w: [0xe11346f1f98fcf89u64, 0x1e2652070753e7f4u64, 0x2b31e9e3d06c32e5u64, 0xec1e4a7db69561a5u64] },
    // 10^-19 ~= ec1e4a7db69561a5  2b31e9e3d06c32e5
    //   1e2652070753e7f4e11346f1f98fcf89   * 2^-319
    BID_UINT256 { w: [0x4da9058e613fd93au64, 0x181ea8059f76532au64, 0x88f4bb1ca6bcf584u64, 0xbce5086492111aeau64] },
    // 10^-20 ~= bce5086492111aea  88f4bb1ca6bcf584
    //   181ea8059f76532a4da9058e613fd93a   * 2^-322
    BID_UINT256 { w: [0xa48737a51a997a95u64, 0x467eecd14c5ea8eeu64, 0xd3f6fc16ebca5e03u64, 0x971da05074da7beeu64] },
    // 10^-21 ~= 971da05074da7bee  d3f6fc16ebca5e03
    //   467eecd14c5ea8eea48737a51a997a95   * 2^-325
    BID_UINT256 { w: [0x3a71f2a1c428c421u64, 0x70cb148213caa7e4u64, 0x5324c68b12dd6338u64, 0xf1c90080baf72cb1u64] },
    // 10^-22 ~= f1c90080baf72cb1  5324c68b12dd6338
    //   70cb148213caa7e43a71f2a1c428c421   * 2^-329
    BID_UINT256 { w: [0x2ec18ee7d0209ce8u64, 0x8d6f439b43088650u64, 0x75b7053c0f178293u64, 0xc16d9a0095928a27u64] },
    // 10^-23 ~= c16d9a0095928a27  75b7053c0f178293
    //   8d6f439b430886502ec18ee7d0209ce8   * 2^-332
    BID_UINT256 { w: [0xf23472530ce6e3edu64, 0xd78c3615cf3a050cu64, 0xc4926a9672793542u64, 0x9abe14cd44753b52u64] },
    // 10^-24 ~= 9abe14cd44753b52  c4926a9672793542
    //   d78c3615cf3a050cf23472530ce6e3ed   * 2^-335
    BID_UINT256 { w: [0xe9ed83b814a49fe1u64, 0x8c1389bc7ec33b47u64, 0x3a83ddbd83f52204u64, 0xf79687aed3eec551u64] },
    // 10^-25 ~= f79687aed3eec551  3a83ddbd83f52204
    //   8c1389bc7ec33b47e9ed83b814a49fe1   * 2^-339
    BID_UINT256 { w: [0x87f1362cdd507fe7u64, 0x3cdc6e306568fc39u64, 0x95364afe032a819du64, 0xc612062576589ddau64] },
    // 10^-26 ~= c612062576589dda  95364afe032a819d
    //   3cdc6e306568fc3987f1362cdd507fe7   * 2^-342
    BID_UINT256 { w: [0x9ff42b5717739986u64, 0xca49f1c05120c9c7u64, 0x775ea264cf55347du64, 0x9e74d1b791e07e48u64] },
    // 10^-27 ~= 9e74d1b791e07e48  775ea264cf55347d
    //   ca49f1c05120c9c79ff42b5717739986   * 2^-345
    BID_UINT256 { w: [0xccb9def1bf1f5c09u64, 0x76dcb60081ce0fa5u64, 0x8bca9d6e188853fcu64, 0xfd87b5f28300ca0du64] },
    // 10^-28 ~= fd87b5f28300ca0d  8bca9d6e188853fc
    //   76dcb60081ce0fa5ccb9def1bf1f5c09   * 2^-349
    BID_UINT256 { w: [0xa3c7e58e327f7cd4u64, 0x5f16f80067d80c84u64, 0x096ee45813a04330u64, 0xcad2f7f5359a3b3eu64] },
    // 10^-29 ~= cad2f7f5359a3b3e  96ee45813a04330
    //   5f16f80067d80c84a3c7e58e327f7cd4   * 2^-352
    BID_UINT256 { w: [0xb6398471c1ff9710u64, 0x18df2ccd1fe00a03u64, 0xa1258379a94d028du64, 0xa2425ff75e14fc31u64] },
    // 10^-30 ~= a2425ff75e14fc31  a1258379a94d028d
    //   18df2ccd1fe00a03b6398471c1ff9710   * 2^-355
    BID_UINT256 { w: [0xf82e038e34cc78dau64, 0x4718f0a419800802u64, 0x80eacf948770ced7u64, 0x81ceb32c4b43fcf4u64] },
    // 10^-31 ~= 81ceb32c4b43fcf4  80eacf948770ced7
    //   4718f0a419800802f82e038e34cc78da   * 2^-358
    BID_UINT256 { w: [0x59e338e387ad8e29u64, 0x0b5b1aa028ccd99eu64, 0x67de18eda5814af2u64, 0xcfb11ead453994bau64] },
    // 10^-32 ~= cfb11ead453994ba  67de18eda5814af2
    //   b5b1aa028ccd99e59e338e387ad8e29   * 2^-362
    BID_UINT256 { w: [0x47e8fa4f9fbe0b54u64, 0x6f7c154ced70ae18u64, 0xecb1ad8aeacdd58eu64, 0xa6274bbdd0fadd61u64] },
    // 10^-33 ~= a6274bbdd0fadd61  ecb1ad8aeacdd58e
    //   6f7c154ced70ae1847e8fa4f9fbe0b54   * 2^-365
    BID_UINT256 { w: [0xd320c83fb2fe6f76u64, 0xbf967770bdf3be79u64, 0xbd5af13bef0b113eu64, 0x84ec3c97da624ab4u64] },
    // 10^-34 ~= 84ec3c97da624ab4  bd5af13bef0b113e
    //   bf967770bdf3be79d320c83fb2fe6f76   * 2^-368
    BID_UINT256 { w: [0x85014065eb30b257u64, 0x65bd8be79652ca5cu64, 0x955e4ec64b44e864u64, 0xd4ad2dbfc3d07787u64] },
    // 10^-35 ~= d4ad2dbfc3d07787  955e4ec64b44e864
    //   65bd8be79652ca5c85014065eb30b257   * 2^-372
    BID_UINT256 { w: [0xd0cdcd1e55c08eacu64, 0xeafe098611dbd516u64, 0xdde50bd1d5d0b9e9u64, 0xaa242499697392d2u64] },
    // 10^-36 ~= aa242499697392d2  dde50bd1d5d0b9e9
    //   eafe098611dbd516d0cdcd1e55c08eac   * 2^-375
    BID_UINT256 { w: [0x40a4a418449a0bbdu64, 0xbbfe6e04db164412u64, 0x7e50d64177da2e54u64, 0x881cea14545c7575u64] },
    // 10^-37 ~= 881cea14545c7575  7e50d64177da2e54
    //   bbfe6e04db16441240a4a418449a0bbd   * 2^-378
    BID_UINT256 { w: [0x9aa1068d3a9012c8u64, 0x2cca49a15e8a0683u64, 0x96e7bd358c904a21u64, 0xd9c7dced53c72255u64] },
    // 10^-38 ~= d9c7dced53c72255  96e7bd358c904a21
    //   2cca49a15e8a06839aa1068d3a9012c8   * 2^-382
    BID_UINT256 { w: [0x154d9ed7620cdbd3u64, 0x8a3b6e1ab2080536u64, 0xabec975e0a0d081au64, 0xae397d8aa96c1b77u64] },
    // 10^-39 ~= ae397d8aa96c1b77  abec975e0a0d081a
    //   8a3b6e1ab2080536154d9ed7620cdbd3   * 2^-385
    BID_UINT256 { w: [0x443e18ac4e70afdcu64, 0x3b62be7bc1a0042bu64, 0x2323ac4b3b3da015u64, 0x8b61313bbabce2c6u64] },
    // 10^-40 ~= 8b61313bbabce2c6  2323ac4b3b3da015
    //   3b62be7bc1a0042b443e18ac4e70afdc   * 2^-388
    BID_UINT256 { w: [0x6d30277a171ab2f9u64, 0x5f0463f935ccd378u64, 0x6b6c46dec52f6688u64, 0xdf01e85f912e37a3u64] },
    // 10^-41 ~= df01e85f912e37a3  6b6c46dec52f6688
    //   5f0463f935ccd3786d30277a171ab2f9   * 2^-392
    BID_UINT256 { w: [0x8a8cec61ac155bfbu64, 0x7f36b660f7d70f93u64, 0x55f038b237591ed3u64, 0xb267ed1940f1c61cu64] },
    // 10^-42 ~= b267ed1940f1c61c  55f038b237591ed3
    //   7f36b660f7d70f938a8cec61ac155bfb   * 2^-395
    BID_UINT256 { w: [0x3ba3f04e23444996u64, 0xcc2bc51a5fdf3fa9u64, 0x77f3608e92adb242u64, 0x8eb98a7a9a5b04e3u64] },
    // 10^-43 ~= 8eb98a7a9a5b04e3  77f3608e92adb242
    //   cc2bc51a5fdf3fa93ba3f04e23444996   * 2^-398
    BID_UINT256 { w: [0xf9064d49d206dc22u64, 0xe046082a32fecc41u64, 0x8cb89a7db77c506au64, 0xe45c10c42a2b3b05u64] },
    // 10^-44 ~= e45c10c42a2b3b05  8cb89a7db77c506a
    //   e046082a32fecc41f9064d49d206dc22   * 2^-402
    BID_UINT256 { w: [0xfa6b7107db38b01bu64, 0x4d04d354f598a367u64, 0x3d607b97c5fd0d22u64, 0xb6b00d69bb55c8d1u64] },
    // 10^-45 ~= b6b00d69bb55c8d1  3d607b97c5fd0d22
    //   4d04d354f598a367fa6b7107db38b01b   * 2^-405
    BID_UINT256 { w: [0xfb8927397c2d59b0u64, 0x3d9d75dd9146e91fu64, 0xcab3961304ca70e8u64, 0x9226712162ab070du64] },
    // 10^-46 ~= 9226712162ab070d  cab3961304ca70e8
    //   3d9d75dd9146e91ffb8927397c2d59b0   * 2^-408
    BID_UINT256 { w: [0xf8db71f5937bc2b2u64, 0xc8fbefc8e87174ffu64, 0xaab8f01e6e10b4a6u64, 0xe9d71b689dde71afu64] },
    // 10^-47 ~= e9d71b689dde71af  aab8f01e6e10b4a6
    //   c8fbefc8e87174fff8db71f5937bc2b2   * 2^-412
    BID_UINT256 { w: [0x2d7c5b2adc630228u64, 0x3a63263a538df733u64, 0x5560c018580d5d52u64, 0xbb127c53b17ec159u64] },
    // 10^-48 ~= bb127c53b17ec159  5560c018580d5d52
    //   3a63263a538df7332d7c5b2adc630228   * 2^-415
    BID_UINT256 { w: [0x24637c2249e8ce87u64, 0x2eb5b82ea93e5f5cu64, 0xdde7001379a44aa8u64, 0x95a8637627989aadu64] },
    // 10^-49 ~= 95a8637627989aad  dde7001379a44aa8
    //   2eb5b82ea93e5f5c24637c2249e8ce87   * 2^-418
    BID_UINT256 { w: [0x3a38c69d430e173eu64, 0x4abc59e441fd6560u64, 0x963e66858f6d4440u64, 0xef73d256a5c0f77cu64] },
    // 10^-50 ~= ef73d256a5c0f77c  963e66858f6d4440
    //   4abc59e441fd65603a38c69d430e173e   * 2^-422
    BID_UINT256 { w: [0x94fa387dcf3e78feu64, 0x6efd14b69b311de6u64, 0xde98520472bdd033u64, 0xbf8fdb78849a5f96u64] },
    // 10^-51 ~= bf8fdb78849a5f96  de98520472bdd033
    //   6efd14b69b311de694fa387dcf3e78fe   * 2^-425
    BID_UINT256 { w: [0xaa61c6cb0c31fa65u64, 0x259743c548f417ebu64, 0xe546a8038efe4029u64, 0x993fe2c6d07b7fabu64] },
    // 10^-52 ~= 993fe2c6d07b7fab  e546a8038efe4029
    //   259743c548f417ebaa61c6cb0c31fa65   * 2^-428
    BID_UINT256 { w: [0xaa360ade79e990a2u64, 0x3c25393ba7ecf312u64, 0xd53dd99f4b3066a8u64, 0xf53304714d9265dfu64] },
    // 10^-53 ~= f53304714d9265df  d53dd99f4b3066a8
    //   3c25393ba7ecf312aa360ade79e990a2   * 2^-432
    BID_UINT256 { w: [0x882b3be52e5473b5u64, 0x96842dc95323f5a8u64, 0xaa97e14c3c26b886u64, 0xc428d05aa4751e4cu64] },
    // 10^-54 ~= c428d05aa4751e4c  aa97e14c3c26b886
    //   96842dc95323f5a8882b3be52e5473b5   * 2^-435
    BID_UINT256 { w: [0xd355c98425105c91u64, 0xab9cf16ddc1cc486u64, 0x55464dd69685606bu64, 0x9ced737bb6c4183du64] },
    // 10^-55 ~= 9ced737bb6c4183d  55464dd69685606b
    //   ab9cf16ddc1cc486d355c98425105c91   * 2^-438
    BID_UINT256 { w: [0xebbc75a03b4d60e7u64, 0xac2e4f162cfad40au64, 0xeed6e2f0f0d56712u64, 0xfb158592be068d2eu64] },
    // 10^-56 ~= fb158592be068d2e  eed6e2f0f0d56712
    //   ac2e4f162cfad40aebbc75a03b4d60e7   * 2^-442
    BID_UINT256 { w: [0x8963914cfc3de71fu64, 0x568b727823fbdcd5u64, 0xf245825a5a445275u64, 0xc8de047564d20a8bu64] },
    // 10^-57 ~= c8de047564d20a8b  f245825a5a445275
    //   568b727823fbdcd58963914cfc3de71f   * 2^-445
    BID_UINT256 { w: [0xd44fa770c9cb1f4cu64, 0x453c5b934ffcb0aau64, 0x5b6aceaeae9d0ec4u64, 0xa0b19d2ab70e6ed6u64] },
    // 10^-58 ~= a0b19d2ab70e6ed6  5b6aceaeae9d0ec4
    //   453c5b934ffcb0aad44fa770c9cb1f4c   * 2^-448
    BID_UINT256 { w: [0xdd0c85f3d4a27f70u64, 0x37637c75d996f3bbu64, 0xe2bbd88bbee40bd0u64, 0x808e17555f3ebf11u64] },
    // 10^-59 ~= 808e17555f3ebf11  e2bbd88bbee40bd0
    //   37637c75d996f3bbdd0c85f3d4a27f70   * 2^-451
    BID_UINT256 { w: [0x61ada31fba9d98b3u64, 0x256bfa5628f185f9u64, 0x3792f412cb06794du64, 0xcdb02555653131b6u64] },
    // 10^-60 ~= cdb02555653131b6  3792f412cb06794d
    //   256bfa5628f185f961ada31fba9d98b3   * 2^-455
    BID_UINT256 { w: [0xe7be1c196217ad5cu64, 0x51232eab53f46b2du64, 0x5fa8c3423c052dd7u64, 0xa48ceaaab75a8e2bu64] },
    // 10^-61 ~= a48ceaaab75a8e2b  5fa8c3423c052dd7
    //   51232eab53f46b2de7be1c196217ad5c   * 2^-458
    BID_UINT256 { w: [0x52fe7ce11b46244au64, 0x40e8f222a99055beu64, 0x1953cf68300424acu64, 0x83a3eeeef9153e89u64] },
    // 10^-62 ~= 83a3eeeef9153e89  1953cf68300424ac
    //   40e8f222a99055be52fe7ce11b46244a   * 2^-461
    BID_UINT256 { w: [0x51972e34f8703a10u64, 0x34a7e9d10f4d55fdu64, 0x8eec7f0d19a03aadu64, 0xd29fe4b18e88640eu64] },
    // 10^-63 ~= d29fe4b18e88640e  8eec7f0d19a03aad
    //   34a7e9d10f4d55fd51972e34f8703a10   * 2^-465
    BID_UINT256 { w: [0x0e128b5d938cfb40u64, 0x2a1fee40d90aab31u64, 0x3f2398d747b36224u64, 0xa87fea27a539e9a5u64] },
    // 10^-64 ~= a87fea27a539e9a5  3f2398d747b36224
    //   2a1fee40d90aab310e128b5d938cfb40   * 2^-468
    BID_UINT256 { w: [0x3e753c4adc70c900u64, 0xbb4cbe9a473bbc27u64, 0x98e947129fc2b4e9u64, 0x86ccbb52ea94baeau64] },
    // 10^-65 ~= 86ccbb52ea94baea  98e947129fc2b4e9
    //   bb4cbe9a473bbc273e753c4adc70c900   * 2^-471
    BID_UINT256 { w: [0x30bb93aafa4e0e66u64, 0x9214642a0b92c6a5u64, 0x5b0ed81dcc6abb0fu64, 0xd7adf884aa879177u64] },
    // 10^-66 ~= d7adf884aa879177  5b0ed81dcc6abb0f
    //   9214642a0b92c6a530bb93aafa4e0e66   * 2^-475
    BID_UINT256 { w: [0xc0960fbbfb71a51fu64, 0xa8105021a2dbd21du64, 0xe272467e3d222f3fu64, 0xac8b2d36eed2dac5u64] },
    // 10^-67 ~= ac8b2d36eed2dac5  e272467e3d222f3f
    //   a8105021a2dbd21dc0960fbbfb71a51f   * 2^-478
    BID_UINT256 { w: [0x66de72fcc927b74cu64, 0xb9a6a6814f1641b1u64, 0x1b8e9ecb641b58ffu64, 0x8a08f0f8bf0f156bu64] },
    // 10^-68 ~= 8a08f0f8bf0f156b  1b8e9ecb641b58ff
    //   b9a6a6814f1641b166de72fcc927b74c   * 2^-481
    BID_UINT256 { w: [0xd7ca5194750c5879u64, 0xf5d770cee4f0691bu64, 0xf8e431456cf88e65u64, 0xdcdb1b2798182244u64] },
    // 10^-69 ~= dcdb1b2798182244  f8e431456cf88e65
    //   f5d770cee4f0691bd7ca5194750c5879   * 2^-485
    BID_UINT256 { w: [0xdfd50e105da379fau64, 0x9179270bea59edafu64, 0x2d835a9df0c6d851u64, 0xb0af48ec79ace837u64] },
    // 10^-70 ~= b0af48ec79ace837  2d835a9df0c6d851
    //   9179270bea59edafdfd50e105da379fa   * 2^-488
    BID_UINT256 { w: [0x19773e737e1c6195u64, 0x0dfa85a321e18af3u64, 0x579c487e5a38ad0eu64, 0x8d590723948a535fu64] },
    // 10^-71 ~= 8d590723948a535f  579c487e5a38ad0e
    //   dfa85a321e18af319773e737e1c6195   * 2^-491
    BID_UINT256 { w: [0xf58b971f302d68efu64, 0x165da29e9c9c1184u64, 0x25c6da63c38de1b0u64, 0xe2280b6c20dd5232u64] },
    // 10^-72 ~= e2280b6c20dd5232  25c6da63c38de1b0
    //   165da29e9c9c1184f58b971f302d68ef   * 2^-495
    BID_UINT256 { w: [0xc46fac18f3578725u64, 0x4517b54bb07cdad0u64, 0x1e38aeb6360b1af3u64, 0xb4ecd5f01a4aa828u64] },
    // 10^-73 ~= b4ecd5f01a4aa828  1e38aeb6360b1af3
    //   4517b54bb07cdad0c46fac18f3578725   * 2^-498
    BID_UINT256 { w: [0x36bfbce0c2ac6c1eu64, 0x9dac910959fd7bdau64, 0xb1c6f22b5e6f48c2u64, 0x90bd77f3483bb9b9u64] },
    // 10^-74 ~= 90bd77f3483bb9b9  b1c6f22b5e6f48c2
    //   9dac910959fd7bda36bfbce0c2ac6c1e   * 2^-501
    BID_UINT256 { w: [0x2465fb01377a4696u64, 0x2f7a81a88ffbf95du64, 0xb60b1d1230b20e04u64, 0xe7958cb87392c2c2u64] }
    // 10^-75 ~= e7958cb87392c2c2  b60b1d1230b20e04
    //   2f7a81a88ffbf95d2465fb01377a4696   * 2^-505
];

pub (crate) const bid_Ex256m256: [u32; 75] = [
    3,   // 259 - 256, Ex = 259
    6,   // 262 - 256, Ex = 262
    9,   // 265 - 256, Ex = 265
    13,  // 269 - 256, Ex = 269
    16,  // 272 - 256, Ex = 272
    19,  // 275 - 256, Ex = 275
    23,  // 279 - 256, Ex = 279
    26,  // 282 - 256, Ex = 282
    29,  // 285 - 256, Ex = 285
    33,  // 289 - 256, Ex = 289
    36,  // 292 - 256, Ex = 292
    39,  // 295 - 256, Ex = 295
    43,  // 299 - 256, Ex = 299
    46,  // 302 - 256, Ex = 302
    49,  // 305 - 256, Ex = 305
    53,  // 309 - 256, Ex = 309
    56,  // 312 - 256, Ex = 312
    59,  // 315 - 256, Ex = 315
    63,  // 319 - 256, Ex = 319
    2,   // 322 - 320, Ex = 322
    5,   // 325 - 320, Ex = 325
    9,   // 329 - 320, Ex = 329
    12,  // 332 - 320, Ex = 332
    15,  // 335 - 320, Ex = 335
    19,  // 339 - 320, Ex = 339
    22,  // 342 - 320, Ex = 342
    25,  // 345 - 320, Ex = 345
    29,  // 349 - 320, Ex = 349
    32,  // 352 - 320, Ex = 352
    35,  // 355 - 320, Ex = 355
    38,  // 358 - 320, Ex = 358
    42,  // 362 - 320, Ex = 362
    45,  // 365 - 320, Ex = 365
    48,  // 368 - 320, Ex = 368
    52,  // 372 - 320, Ex = 372
    55,  // 375 - 320, Ex = 375
    58,  // 378 - 320, Ex = 378
    62,  // 382 - 320, Ex = 382
    1,   // 385 - 384, Ex = 385
    4,   // 388 - 384, Ex = 388
    8,   // 392 - 384, Ex = 392
    11,  // 395 - 384, Ex = 395
    14,  // 398 - 384, Ex = 398
    18,  // 402 - 384, Ex = 402
    21,  // 405 - 384, Ex = 405
    24,  // 408 - 384, Ex = 408
    28,  // 412 - 384, Ex = 412
    31,  // 415 - 384, Ex = 415
    34,  // 418 - 384, Ex = 418
    38,  // 422 - 384, Ex = 422
    41,  // 425 - 384, Ex = 425
    44,  // 428 - 384, Ex = 428
    48,  // 432 - 384, Ex = 432
    51,  // 435 - 384, Ex = 435
    54,  // 438 - 384, Ex = 438
    58,  // 442 - 384, Ex = 442
    61,  // 445 - 384, Ex = 445
    0,   // 448 - 448, Ex = 448
    3,   // 451 - 448, Ex = 451
    7,   // 455 - 448, Ex = 455
    10,  // 458 - 448, Ex = 458
    13,  // 461 - 448, Ex = 461
    17,  // 465 - 448, Ex = 465
    20,  // 468 - 448, Ex = 468
    23,  // 471 - 448, Ex = 471
    27,  // 475 - 448, Ex = 475
    30,  // 478 - 448, Ex = 478
    33,  // 481 - 448, Ex = 481
    37,  // 485 - 448, Ex = 485
    40,  // 488 - 448, Ex = 488
    43,  // 491 - 448, Ex = 491
    47,  // 495 - 448, Ex = 495
    50,  // 498 - 448, Ex = 498
    53,  // 501 - 448, Ex = 501
    57   // 505 - 448, Ex = 505
];

pub (crate) const bid_half256: [BID_UINT64; 75] = [
    0x0000000000000004u64,  // half / 2^256 = 4
    0x0000000000000020u64,  // half / 2^256 = 20
    0x0000000000000100u64,  // half / 2^256 = 100
    0x0000000000001000u64,  // half / 2^256 = 1000
    0x0000000000008000u64,  // half / 2^256 = 8000
    0x0000000000040000u64,  // half / 2^256 = 40000
    0x0000000000400000u64,  // half / 2^256 = 400000
    0x0000000002000000u64,  // half / 2^256 = 2000000
    0x0000000010000000u64,  // half / 2^256 = 10000000
    0x0000000100000000u64,  // half / 2^256 = 100000000
    0x0000000800000000u64,  // half / 2^256 = 800000000
    0x0000004000000000u64,  // half / 2^256 = 4000000000
    0x0000040000000000u64,  // half / 2^256 = 40000000000
    0x0000200000000000u64,  // half / 2^256 = 200000000000
    0x0001000000000000u64,  // half / 2^256 = 1000000000000
    0x0010000000000000u64,  // half / 2^256 = 10000000000000
    0x0080000000000000u64,  // half / 2^256 = 80000000000000
    0x0400000000000000u64,  // half / 2^256 = 400000000000000
    0x4000000000000000u64,  // half / 2^256 = 4000000000000000
    0x0000000000000002u64,  // half / 2^320 = 2
    0x0000000000000010u64,  // half / 2^320 = 10
    0x0000000000000100u64,  // half / 2^320 = 100
    0x0000000000000800u64,  // half / 2^320 = 800
    0x0000000000004000u64,  // half / 2^320 = 4000
    0x0000000000040000u64,  // half / 2^320 = 40000
    0x0000000000200000u64,  // half / 2^320 = 200000
    0x0000000001000000u64,  // half / 2^320 = 1000000
    0x0000000010000000u64,  // half / 2^320 = 10000000
    0x0000000080000000u64,  // half / 2^320 = 80000000
    0x0000000400000000u64,  // half / 2^320 = 400000000
    0x0000002000000000u64,  // half / 2^320 = 2000000000
    0x0000020000000000u64,  // half / 2^320 = 20000000000
    0x0000100000000000u64,  // half / 2^320 = 100000000000
    0x0000800000000000u64,  // half / 2^320 = 800000000000
    0x0008000000000000u64,  // half / 2^320 = 8000000000000
    0x0040000000000000u64,  // half / 2^320 = 40000000000000
    0x0200000000000000u64,  // half / 2^320 = 200000000000000
    0x2000000000000000u64,  // half / 2^320 = 2000000000000000
    0x0000000000000001u64,  // half / 2^384 = 1
    0x0000000000000008u64,  // half / 2^384 = 8
    0x0000000000000080u64,  // half / 2^384 = 80
    0x0000000000000400u64,  // half / 2^384 = 400
    0x0000000000002000u64,  // half / 2^384 = 2000
    0x0000000000020000u64,  // half / 2^384 = 20000
    0x0000000000100000u64,  // half / 2^384 = 100000
    0x0000000000800000u64,  // half / 2^384 = 800000
    0x0000000008000000u64,  // half / 2^384 = 8000000
    0x0000000040000000u64,  // half / 2^384 = 40000000
    0x0000000200000000u64,  // half / 2^384 = 200000000
    0x0000002000000000u64,  // half / 2^384 = 2000000000
    0x0000010000000000u64,  // half / 2^384 = 10000000000
    0x0000080000000000u64,  // half / 2^384 = 80000000000
    0x0000800000000000u64,  // half / 2^384 = 800000000000
    0x0004000000000000u64,  // half / 2^384 = 4000000000000
    0x0020000000000000u64,  // half / 2^384 = 20000000000000
    0x0200000000000000u64,  // half / 2^384 = 200000000000000
    0x1000000000000000u64,  // half / 2^384 = 1000000000000000
    0x8000000000000000u64,  // half / 2^384 = 8000000000000000
    0x0000000000000004u64,  // half / 2^448 = 4
    0x0000000000000040u64,  // half / 2^448 = 40
    0x0000000000000200u64,  // half / 2^448 = 200
    0x0000000000001000u64,  // half / 2^448 = 1000
    0x0000000000010000u64,  // half / 2^448 = 10000
    0x0000000000080000u64,  // half / 2^448 = 80000
    0x0000000000400000u64,  // half / 2^448 = 400000
    0x0000000004000000u64,  // half / 2^448 = 4000000
    0x0000000020000000u64,  // half / 2^448 = 20000000
    0x0000000100000000u64,  // half / 2^448 = 100000000
    0x0000001000000000u64,  // half / 2^448 = 1000000000
    0x0000008000000000u64,  // half / 2^448 = 8000000000
    0x0000040000000000u64,  // half / 2^448 = 40000000000
    0x0000400000000000u64,  // half / 2^448 = 400000000000
    0x0002000000000000u64,  // half / 2^448 = 2000000000000
    0x0010000000000000u64,  // half / 2^448 = 10000000000000
    0x0100000000000000u64   // half / 2^448 = 100000000000000
];

pub (crate) const bid_mask256: [BID_UINT64; 75] = [
    0x0000000000000007u64,  // mask / 2^256
    0x000000000000003fu64,  // mask / 2^256
    0x00000000000001ffu64,  // mask / 2^256
    0x0000000000001fffu64,  // mask / 2^256
    0x000000000000ffffu64,  // mask / 2^256
    0x000000000007ffffu64,  // mask / 2^256
    0x00000000007fffffu64,  // mask / 2^256
    0x0000000003ffffffu64,  // mask / 2^256
    0x000000001fffffffu64,  // mask / 2^256
    0x00000001ffffffffu64,  // mask / 2^256
    0x0000000fffffffffu64,  // mask / 2^256
    0x0000007fffffffffu64,  // mask / 2^256
    0x000007ffffffffffu64,  // mask / 2^256
    0x00003fffffffffffu64,  // mask / 2^256
    0x0001ffffffffffffu64,  // mask / 2^256
    0x001fffffffffffffu64,  // mask / 2^256
    0x00ffffffffffffffu64,  // mask / 2^256
    0x07ffffffffffffffu64,  // mask / 2^256
    0x7fffffffffffffffu64,  // mask / 2^256
    0x0000000000000003u64,  // mask / 2^320
    0x000000000000001fu64,  // mask / 2^320
    0x00000000000001ffu64,  // mask / 2^320
    0x0000000000000fffu64,  // mask / 2^320
    0x0000000000007fffu64,  // mask / 2^320
    0x000000000007ffffu64,  // mask / 2^320
    0x00000000003fffffu64,  // mask / 2^320
    0x0000000001ffffffu64,  // mask / 2^320
    0x000000001fffffffu64,  // mask / 2^320
    0x00000000ffffffffu64,  // mask / 2^320
    0x00000007ffffffffu64,  // mask / 2^320
    0x0000003fffffffffu64,  // mask / 2^320
    0x000003ffffffffffu64,  // mask / 2^320
    0x00001fffffffffffu64,  // mask / 2^320
    0x0000ffffffffffffu64,  // mask / 2^320
    0x000fffffffffffffu64,  // mask / 2^320
    0x007fffffffffffffu64,  // mask / 2^320
    0x03ffffffffffffffu64,  // mask / 2^320
    0x3fffffffffffffffu64,  // mask / 2^320
    0x0000000000000001u64,  // mask / 2^384
    0x000000000000000fu64,  // mask / 2^384
    0x00000000000000ffu64,  // mask / 2^384
    0x00000000000007ffu64,  // mask / 2^384
    0x0000000000003fffu64,  // mask / 2^384
    0x000000000003ffffu64,  // mask / 2^384
    0x00000000001fffffu64,  // mask / 2^384
    0x0000000000ffffffu64,  // mask / 2^384
    0x000000000fffffffu64,  // mask / 2^384
    0x000000007fffffffu64,  // mask / 2^384
    0x00000003ffffffffu64,  // mask / 2^384
    0x0000003fffffffffu64,  // mask / 2^384
    0x000001ffffffffffu64,  // mask / 2^384
    0x00000fffffffffffu64,  // mask / 2^384
    0x0000ffffffffffffu64,  // mask / 2^384
    0x0007ffffffffffffu64,  // mask / 2^384
    0x003fffffffffffffu64,  // mask / 2^384
    0x03ffffffffffffffu64,  // mask / 2^384
    0x1fffffffffffffffu64,  // mask / 2^384
    0xffffffffffffffffu64,  // mask / 2^384
    0x0000000000000007u64,  // mask / 2^448
    0x000000000000007fu64,  // mask / 2^448
    0x00000000000003ffu64,  // mask / 2^448
    0x0000000000001fffu64,  // mask / 2^448
    0x000000000001ffffu64,  // mask / 2^448
    0x00000000000fffffu64,  // mask / 2^448
    0x00000000007fffffu64,  // mask / 2^448
    0x0000000007ffffffu64,  // mask / 2^448
    0x000000003fffffffu64,  // mask / 2^448
    0x00000001ffffffffu64,  // mask / 2^448
    0x0000001fffffffffu64,  // mask / 2^448
    0x000000ffffffffffu64,  // mask / 2^448
    0x000007ffffffffffu64,  // mask / 2^448
    0x00007fffffffffffu64,  // mask / 2^448
    0x0003ffffffffffffu64,  // mask / 2^448
    0x001fffffffffffffu64,  // mask / 2^448
    0x01ffffffffffffffu64   // mask / 2^448
];

pub (crate) const bid_ten2mxtrunc256: [BID_UINT256; 75] = [
    BID_UINT256 { w: [0xccccccccccccccccu64, 0xccccccccccccccccu64, 0xccccccccccccccccu64, 0xccccccccccccccccu64] },
    // (ten2mx >> 256) = cccccccccccccccc  cccccccccccccccc
    //   cccccccccccccccccccccccccccccccc
    BID_UINT256 { w: [0x70a3d70a3d70a3d7u64, 0xd70a3d70a3d70a3du64, 0x3d70a3d70a3d70a3u64, 0xa3d70a3d70a3d70au64] },
    // (ten2mx >> 256) = a3d70a3d70a3d70a  3d70a3d70a3d70a3
    //   d70a3d70a3d70a3d70a3d70a3d70a3d7
    BID_UINT256 { w: [0xc083126e978d4fdfu64, 0x78d4fdf3b645a1cau64, 0x645a1cac083126e9u64, 0x83126e978d4fdf3bu64] },
    // (ten2mx >> 256) = 83126e978d4fdf3b  645a1cac083126e9
    //   78d4fdf3b645a1cac083126e978d4fdf
    BID_UINT256 { w: [0x67381d7dbf487fcbu64, 0xc154c985f06f6944u64, 0xd3c36113404ea4a8u64, 0xd1b71758e219652bu64] },
    // (ten2mx >> 256) = d1b71758e219652b  d3c36113404ea4a8
    //   c154c985f06f694467381d7dbf487fcb
    BID_UINT256 { w: [0x85c67dfe32a0663cu64, 0xcddd6e04c0592103u64, 0x0fcf80dc33721d53u64, 0xa7c5ac471b478423u64] },
    // (ten2mx >> 256) = a7c5ac471b478423  fcf80dc33721d53
    //   cddd6e04c059210385c67dfe32a0663c
    BID_UINT256 { w: [0x37d1fe64f54d1e96u64, 0xd7e45803cd141a69u64, 0xa63f9a49c2c1b10fu64, 0x8637bd05af6c69b5u64] },
    // (ten2mx >> 256) = 8637bd05af6c69b5  a63f9a49c2c1b10f
    //   d7e45803cd141a6937d1fe64f54d1e96
    BID_UINT256 { w: [0x8c8330a1887b6424u64, 0x8ca08cd2e1b9c3dbu64, 0x3d32907604691b4cu64, 0xd6bf94d5e57a42bcu64] },
    // (ten2mx >> 256) = d6bf94d5e57a42bc  3d32907604691b4c
    //   8ca08cd2e1b9c3db8c8330a1887b6424
    BID_UINT256 { w: [0x7068f3b46d2f8350u64, 0x3d4d3d758161697cu64, 0xfdc20d2b36ba7c3du64, 0xabcc77118461cefcu64] },
    // (ten2mx >> 256) = abcc77118461cefc  fdc20d2b36ba7c3d
    //   3d4d3d758161697c7068f3b46d2f8350
    BID_UINT256 { w: [0xf387295d242602a6u64, 0xfdd7645e011abac9u64, 0x31680a88f8953030u64, 0x89705f4136b4a597u64] },
    // (ten2mx >> 256) = 89705f4136b4a597  31680a88f8953030
    //   fdd7645e011abac9f387295d242602a6
    BID_UINT256 { w: [0xb8d8422ea03cd10au64, 0x2fbf06fcce912adcu64, 0xb573440e5a884d1bu64, 0xdbe6fecebdedd5beu64] },
    // (ten2mx >> 256) = dbe6fecebdedd5be  b573440e5a884d1b
    //   2fbf06fcce912adcb8d8422ea03cd10a
    BID_UINT256 { w: [0x93e034f219ca40d5u64, 0xf2ff38ca3eda88b0u64, 0xf78f69a51539d748u64, 0xafebff0bcb24aafeu64] },
    // (ten2mx >> 256) = afebff0bcb24aafe  f78f69a51539d748
    //   f2ff38ca3eda88b093e034f219ca40d5
    BID_UINT256 { w: [0x4319c3f4e16e9a44u64, 0xf598fa3b657ba08du64, 0xf93f87b7442e45d3u64, 0x8cbccc096f5088cbu64] },
    // (ten2mx >> 256) = 8cbccc096f5088cb  f93f87b7442e45d3
    //   f598fa3b657ba08d4319c3f4e16e9a44
    BID_UINT256 { w: [0x04f606549be42a06u64, 0x88f4c3923bf900e2u64, 0x2865a5f206b06fb9u64, 0xe12e13424bb40e13u64] },
    // (ten2mx >> 256) = e12e13424bb40e13  2865a5f206b06fb9
    //   88f4c3923bf900e204f606549be42a06
    BID_UINT256 { w: [0x03f805107cb68805u64, 0x6d909c74fcc733e8u64, 0x538484c19ef38c94u64, 0xb424dc35095cd80fu64] },
    // (ten2mx >> 256) = b424dc35095cd80f  538484c19ef38c94
    //   6d909c74fcc733e803f805107cb68805
    BID_UINT256 { w: [0x3660040d3092066au64, 0x57a6e390ca38f653u64, 0x0f9d37014bf60a10u64, 0x901d7cf73ab0acd9u64] },
    // (ten2mx >> 256) = 901d7cf73ab0acd9  f9d37014bf60a10
    //   57a6e390ca38f6533660040d3092066a
    BID_UINT256 { w: [0x23ccd3484db670aau64, 0xbf716c1add27f085u64, 0x4c2ebe687989a9b3u64, 0xe69594bec44de15bu64] },
    // (ten2mx >> 256) = e69594bec44de15b  4c2ebe687989a9b3
    //   bf716c1add27f08523ccd3484db670aa
    BID_UINT256 { w: [0x4fd70f6d0af85a22u64, 0xff8df0157db98d37u64, 0x09befeb9fad487c2u64, 0xb877aa3236a4b449u64] },
    // (ten2mx >> 256) = b877aa3236a4b449  9befeb9fad487c2
    //   ff8df0157db98d374fd70f6d0af85a22
    BID_UINT256 { w: [0x0cac0c573bf9e1b5u64, 0x32d7f344649470f9u64, 0x3aff322e62439fcfu64, 0x9392ee8e921d5d07u64] },
    // (ten2mx >> 256) = 9392ee8e921d5d07  3aff322e62439fcf
    //   32d7f344649470f90cac0c573bf9e1b5
    BID_UINT256 { w: [0xe11346f1f98fcf88u64, 0x1e2652070753e7f4u64, 0x2b31e9e3d06c32e5u64, 0xec1e4a7db69561a5u64] },
    // (ten2mx >> 256) = ec1e4a7db69561a5  2b31e9e3d06c32e5
    //   1e2652070753e7f4e11346f1f98fcf88
    BID_UINT256 { w: [0x4da9058e613fd939u64, 0x181ea8059f76532au64, 0x88f4bb1ca6bcf584u64, 0xbce5086492111aeau64] },
    // (ten2mx >> 256) = bce5086492111aea  88f4bb1ca6bcf584
    //   181ea8059f76532a4da9058e613fd939
    BID_UINT256 { w: [0xa48737a51a997a94u64, 0x467eecd14c5ea8eeu64, 0xd3f6fc16ebca5e03u64, 0x971da05074da7beeu64] },
    // (ten2mx >> 256) = 971da05074da7bee  d3f6fc16ebca5e03
    //   467eecd14c5ea8eea48737a51a997a94
    BID_UINT256 { w: [0x3a71f2a1c428c420u64, 0x70cb148213caa7e4u64, 0x5324c68b12dd6338u64, 0xf1c90080baf72cb1u64] },
    // (ten2mx >> 256) = f1c90080baf72cb1  5324c68b12dd6338
    //   70cb148213caa7e43a71f2a1c428c420
    BID_UINT256 { w: [0x2ec18ee7d0209ce7u64, 0x8d6f439b43088650u64, 0x75b7053c0f178293u64, 0xc16d9a0095928a27u64] },
    // (ten2mx >> 256) = c16d9a0095928a27  75b7053c0f178293
    //   8d6f439b430886502ec18ee7d0209ce7
    BID_UINT256 { w: [0xf23472530ce6e3ecu64, 0xd78c3615cf3a050cu64, 0xc4926a9672793542u64, 0x9abe14cd44753b52u64] },
    // (ten2mx >> 256) = 9abe14cd44753b52  c4926a9672793542
    //   d78c3615cf3a050cf23472530ce6e3ec
    BID_UINT256 { w: [0xe9ed83b814a49fe0u64, 0x8c1389bc7ec33b47u64, 0x3a83ddbd83f52204u64, 0xf79687aed3eec551u64] },
    // (ten2mx >> 256) = f79687aed3eec551  3a83ddbd83f52204
    //   8c1389bc7ec33b47e9ed83b814a49fe0
    BID_UINT256 { w: [0x87f1362cdd507fe6u64, 0x3cdc6e306568fc39u64, 0x95364afe032a819du64, 0xc612062576589ddau64] },
    // (ten2mx >> 256) = c612062576589dda  95364afe032a819d
    //   3cdc6e306568fc3987f1362cdd507fe6
    BID_UINT256 { w: [0x9ff42b5717739985u64, 0xca49f1c05120c9c7u64, 0x775ea264cf55347du64, 0x9e74d1b791e07e48u64] },
    // (ten2mx >> 256) = 9e74d1b791e07e48  775ea264cf55347d
    //   ca49f1c05120c9c79ff42b5717739985
    BID_UINT256 { w: [0xccb9def1bf1f5c08u64, 0x76dcb60081ce0fa5u64, 0x8bca9d6e188853fcu64, 0xfd87b5f28300ca0du64] },
    // (ten2mx >> 256) = fd87b5f28300ca0d  8bca9d6e188853fc
    //   76dcb60081ce0fa5ccb9def1bf1f5c08
    BID_UINT256 { w: [0xa3c7e58e327f7cd3u64, 0x5f16f80067d80c84u64, 0x096ee45813a04330u64, 0xcad2f7f5359a3b3eu64] },
    // (ten2mx >> 256) = cad2f7f5359a3b3e  96ee45813a04330
    //   5f16f80067d80c84a3c7e58e327f7cd3
    BID_UINT256 { w: [0xb6398471c1ff970fu64, 0x18df2ccd1fe00a03u64, 0xa1258379a94d028du64, 0xa2425ff75e14fc31u64] },
    // (ten2mx >> 256) = a2425ff75e14fc31  a1258379a94d028d
    //   18df2ccd1fe00a03b6398471c1ff970f
    BID_UINT256 { w: [0xf82e038e34cc78d9u64, 0x4718f0a419800802u64, 0x80eacf948770ced7u64, 0x81ceb32c4b43fcf4u64] },
    // (ten2mx >> 256) = 81ceb32c4b43fcf4  80eacf948770ced7
    //   4718f0a419800802f82e038e34cc78d9
    BID_UINT256 { w: [0x59e338e387ad8e28u64, 0x0b5b1aa028ccd99eu64, 0x67de18eda5814af2u64, 0xcfb11ead453994bau64] },
    // (ten2mx >> 256) = cfb11ead453994ba  67de18eda5814af2
    //   b5b1aa028ccd99e59e338e387ad8e28
    BID_UINT256 { w: [0x47e8fa4f9fbe0b53u64, 0x6f7c154ced70ae18u64, 0xecb1ad8aeacdd58eu64, 0xa6274bbdd0fadd61u64] },
    // (ten2mx >> 256) = a6274bbdd0fadd61  ecb1ad8aeacdd58e
    //   6f7c154ced70ae1847e8fa4f9fbe0b53
    BID_UINT256 { w: [0xd320c83fb2fe6f75u64, 0xbf967770bdf3be79u64, 0xbd5af13bef0b113eu64, 0x84ec3c97da624ab4u64] },
    // (ten2mx >> 256) = 84ec3c97da624ab4  bd5af13bef0b113e
    //   bf967770bdf3be79d320c83fb2fe6f75
    BID_UINT256 { w: [0x85014065eb30b256u64, 0x65bd8be79652ca5cu64, 0x955e4ec64b44e864u64, 0xd4ad2dbfc3d07787u64] },
    // (ten2mx >> 256) = d4ad2dbfc3d07787  955e4ec64b44e864
    //   65bd8be79652ca5c85014065eb30b256
    BID_UINT256 { w: [0xd0cdcd1e55c08eabu64, 0xeafe098611dbd516u64, 0xdde50bd1d5d0b9e9u64, 0xaa242499697392d2u64] },
    // (ten2mx >> 256) = aa242499697392d2  dde50bd1d5d0b9e9
    //   eafe098611dbd516d0cdcd1e55c08eab
    BID_UINT256 { w: [0x40a4a418449a0bbcu64, 0xbbfe6e04db164412u64, 0x7e50d64177da2e54u64, 0x881cea14545c7575u64] },
    // (ten2mx >> 256) = 881cea14545c7575  7e50d64177da2e54
    //   bbfe6e04db16441240a4a418449a0bbc
    BID_UINT256 { w: [0x9aa1068d3a9012c7u64, 0x2cca49a15e8a0683u64, 0x96e7bd358c904a21u64, 0xd9c7dced53c72255u64] },
    // (ten2mx >> 256) = d9c7dced53c72255  96e7bd358c904a21
    //   2cca49a15e8a06839aa1068d3a9012c7
    BID_UINT256 { w: [0x154d9ed7620cdbd2u64, 0x8a3b6e1ab2080536u64, 0xabec975e0a0d081au64, 0xae397d8aa96c1b77u64] },
    // (ten2mx >> 256) = ae397d8aa96c1b77  abec975e0a0d081a
    //   8a3b6e1ab2080536154d9ed7620cdbd2
    BID_UINT256 { w: [0x443e18ac4e70afdbu64, 0x3b62be7bc1a0042bu64, 0x2323ac4b3b3da015u64, 0x8b61313bbabce2c6u64] },
    // (ten2mx >> 256) = 8b61313bbabce2c6  2323ac4b3b3da015
    //   3b62be7bc1a0042b443e18ac4e70afdb
    BID_UINT256 { w: [0x6d30277a171ab2f8u64, 0x5f0463f935ccd378u64, 0x6b6c46dec52f6688u64, 0xdf01e85f912e37a3u64] },
    // (ten2mx >> 256) = df01e85f912e37a3  6b6c46dec52f6688
    //   5f0463f935ccd3786d30277a171ab2f8
    BID_UINT256 { w: [0x8a8cec61ac155bfau64, 0x7f36b660f7d70f93u64, 0x55f038b237591ed3u64, 0xb267ed1940f1c61cu64] },
    // (ten2mx >> 256) = b267ed1940f1c61c  55f038b237591ed3
    //   7f36b660f7d70f938a8cec61ac155bfa
    BID_UINT256 { w: [0x3ba3f04e23444995u64, 0xcc2bc51a5fdf3fa9u64, 0x77f3608e92adb242u64, 0x8eb98a7a9a5b04e3u64] },
    // (ten2mx >> 256) = 8eb98a7a9a5b04e3  77f3608e92adb242
    //   cc2bc51a5fdf3fa93ba3f04e23444995
    BID_UINT256 { w: [0xf9064d49d206dc21u64, 0xe046082a32fecc41u64, 0x8cb89a7db77c506au64, 0xe45c10c42a2b3b05u64] },
    // (ten2mx >> 256) = e45c10c42a2b3b05  8cb89a7db77c506a
    //   e046082a32fecc41f9064d49d206dc21
    BID_UINT256 { w: [0xfa6b7107db38b01au64, 0x4d04d354f598a367u64, 0x3d607b97c5fd0d22u64, 0xb6b00d69bb55c8d1u64] },
    // (ten2mx >> 256) = b6b00d69bb55c8d1  3d607b97c5fd0d22
    //   4d04d354f598a367fa6b7107db38b01a
    BID_UINT256 { w: [0xfb8927397c2d59afu64, 0x3d9d75dd9146e91fu64, 0xcab3961304ca70e8u64, 0x9226712162ab070du64] },
    // (ten2mx >> 256) = 9226712162ab070d  cab3961304ca70e8
    //   3d9d75dd9146e91ffb8927397c2d59af
    BID_UINT256 { w: [0xf8db71f5937bc2b1u64, 0xc8fbefc8e87174ffu64, 0xaab8f01e6e10b4a6u64, 0xe9d71b689dde71afu64] },
    // (ten2mx >> 256) = e9d71b689dde71af  aab8f01e6e10b4a6
    //   c8fbefc8e87174fff8db71f5937bc2b1
    BID_UINT256 { w: [0x2d7c5b2adc630227u64, 0x3a63263a538df733u64, 0x5560c018580d5d52u64, 0xbb127c53b17ec159u64] },
    // (ten2mx >> 256) = bb127c53b17ec159  5560c018580d5d52
    //   3a63263a538df7332d7c5b2adc630227
    BID_UINT256 { w: [0x24637c2249e8ce86u64, 0x2eb5b82ea93e5f5cu64, 0xdde7001379a44aa8u64, 0x95a8637627989aadu64] },
    // (ten2mx >> 256) = 95a8637627989aad  dde7001379a44aa8
    //   2eb5b82ea93e5f5c24637c2249e8ce86
    BID_UINT256 { w: [0x3a38c69d430e173du64, 0x4abc59e441fd6560u64, 0x963e66858f6d4440u64, 0xef73d256a5c0f77cu64] },
    // (ten2mx >> 256) = ef73d256a5c0f77c  963e66858f6d4440
    //   4abc59e441fd65603a38c69d430e173d
    BID_UINT256 { w: [0x94fa387dcf3e78fdu64, 0x6efd14b69b311de6u64, 0xde98520472bdd033u64, 0xbf8fdb78849a5f96u64] },
    // (ten2mx >> 256) = bf8fdb78849a5f96  de98520472bdd033
    //   6efd14b69b311de694fa387dcf3e78fd
    BID_UINT256 { w: [0xaa61c6cb0c31fa64u64, 0x259743c548f417ebu64, 0xe546a8038efe4029u64, 0x993fe2c6d07b7fabu64] },
    // (ten2mx >> 256) = 993fe2c6d07b7fab  e546a8038efe4029
    //   259743c548f417ebaa61c6cb0c31fa64
    BID_UINT256 { w: [0xaa360ade79e990a1u64, 0x3c25393ba7ecf312u64, 0xd53dd99f4b3066a8u64, 0xf53304714d9265dfu64] },
    // (ten2mx >> 256) = f53304714d9265df  d53dd99f4b3066a8
    //   3c25393ba7ecf312aa360ade79e990a1
    BID_UINT256 { w: [0x882b3be52e5473b4u64, 0x96842dc95323f5a8u64, 0xaa97e14c3c26b886u64, 0xc428d05aa4751e4cu64] },
    // (ten2mx >> 256) = c428d05aa4751e4c  aa97e14c3c26b886
    //   96842dc95323f5a8882b3be52e5473b4
    BID_UINT256 { w: [0xd355c98425105c90u64, 0xab9cf16ddc1cc486u64, 0x55464dd69685606bu64, 0x9ced737bb6c4183du64] },
    // (ten2mx >> 256) = 9ced737bb6c4183d  55464dd69685606b
    //   ab9cf16ddc1cc486d355c98425105c90
    BID_UINT256 { w: [0xebbc75a03b4d60e6u64, 0xac2e4f162cfad40au64, 0xeed6e2f0f0d56712u64, 0xfb158592be068d2eu64] },
    // (ten2mx >> 256) = fb158592be068d2e  eed6e2f0f0d56712
    //   ac2e4f162cfad40aebbc75a03b4d60e6
    BID_UINT256 { w: [0x8963914cfc3de71eu64, 0x568b727823fbdcd5u64, 0xf245825a5a445275u64, 0xc8de047564d20a8bu64] },
    // (ten2mx >> 256) = c8de047564d20a8b  f245825a5a445275
    //   568b727823fbdcd58963914cfc3de71e
    BID_UINT256 { w: [0xd44fa770c9cb1f4bu64, 0x453c5b934ffcb0aau64, 0x5b6aceaeae9d0ec4u64, 0xa0b19d2ab70e6ed6u64] },
    // (ten2mx >> 256) = a0b19d2ab70e6ed6  5b6aceaeae9d0ec4
    //   453c5b934ffcb0aad44fa770c9cb1f4b
    BID_UINT256 { w: [0xdd0c85f3d4a27f6fu64, 0x37637c75d996f3bbu64, 0xe2bbd88bbee40bd0u64, 0x808e17555f3ebf11u64] },
    // (ten2mx >> 256) = 808e17555f3ebf11  e2bbd88bbee40bd0
    //   37637c75d996f3bbdd0c85f3d4a27f6f
    BID_UINT256 { w: [0x61ada31fba9d98b2u64, 0x256bfa5628f185f9u64, 0x3792f412cb06794du64, 0xcdb02555653131b6u64] },
    // (ten2mx >> 256) = cdb02555653131b6  3792f412cb06794d
    //   256bfa5628f185f961ada31fba9d98b2
    BID_UINT256 { w: [0xe7be1c196217ad5bu64, 0x51232eab53f46b2du64, 0x5fa8c3423c052dd7u64, 0xa48ceaaab75a8e2bu64] },
    // (ten2mx >> 256) = a48ceaaab75a8e2b  5fa8c3423c052dd7
    //   51232eab53f46b2de7be1c196217ad5b
    BID_UINT256 { w: [0x52fe7ce11b462449u64, 0x40e8f222a99055beu64, 0x1953cf68300424acu64, 0x83a3eeeef9153e89u64] },
    // (ten2mx >> 256) = 83a3eeeef9153e89  1953cf68300424ac
    //   40e8f222a99055be52fe7ce11b462449
    BID_UINT256 { w: [0x51972e34f8703a0fu64, 0x34a7e9d10f4d55fdu64, 0x8eec7f0d19a03aadu64, 0xd29fe4b18e88640eu64] },
    // (ten2mx >> 256) = d29fe4b18e88640e  8eec7f0d19a03aad
    //   34a7e9d10f4d55fd51972e34f8703a0f
    BID_UINT256 { w: [0x0e128b5d938cfb3fu64, 0x2a1fee40d90aab31u64, 0x3f2398d747b36224u64, 0xa87fea27a539e9a5u64] },
    // (ten2mx >> 256) = a87fea27a539e9a5  3f2398d747b36224
    //   2a1fee40d90aab310e128b5d938cfb3f
    BID_UINT256 { w: [0x3e753c4adc70c8ffu64, 0xbb4cbe9a473bbc27u64, 0x98e947129fc2b4e9u64, 0x86ccbb52ea94baeau64] },
    // (ten2mx >> 256) = 86ccbb52ea94baea  98e947129fc2b4e9
    //   bb4cbe9a473bbc273e753c4adc70c8ff
    BID_UINT256 { w: [0x30bb93aafa4e0e65u64, 0x9214642a0b92c6a5u64, 0x5b0ed81dcc6abb0fu64, 0xd7adf884aa879177u64] },
    // (ten2mx >> 256) = d7adf884aa879177  5b0ed81dcc6abb0f
    //   9214642a0b92c6a530bb93aafa4e0e65
    BID_UINT256 { w: [0xc0960fbbfb71a51eu64, 0xa8105021a2dbd21du64, 0xe272467e3d222f3fu64, 0xac8b2d36eed2dac5u64] },
    // (ten2mx >> 256) = ac8b2d36eed2dac5  e272467e3d222f3f
    //   a8105021a2dbd21dc0960fbbfb71a51e
    BID_UINT256 { w: [0x66de72fcc927b74bu64, 0xb9a6a6814f1641b1u64, 0x1b8e9ecb641b58ffu64, 0x8a08f0f8bf0f156bu64] },
    // (ten2mx >> 256) = 8a08f0f8bf0f156b  1b8e9ecb641b58ff
    //   b9a6a6814f1641b166de72fcc927b74b
    BID_UINT256 { w: [0xd7ca5194750c5878u64, 0xf5d770cee4f0691bu64, 0xf8e431456cf88e65u64, 0xdcdb1b2798182244u64] },
    // (ten2mx >> 256) = dcdb1b2798182244  f8e431456cf88e65
    //   f5d770cee4f0691bd7ca5194750c5878
    BID_UINT256 { w: [0xdfd50e105da379f9u64, 0x9179270bea59edafu64, 0x2d835a9df0c6d851u64, 0xb0af48ec79ace837u64] },
    // (ten2mx >> 256) = b0af48ec79ace837  2d835a9df0c6d851
    //   9179270bea59edafdfd50e105da379f9
    BID_UINT256 { w: [0x19773e737e1c6194u64, 0x0dfa85a321e18af3u64, 0x579c487e5a38ad0eu64, 0x8d590723948a535fu64] },
    // (ten2mx >> 256) = 8d590723948a535f  579c487e5a38ad0e
    //   dfa85a321e18af319773e737e1c6194
    BID_UINT256 { w: [0xf58b971f302d68eeu64, 0x165da29e9c9c1184u64, 0x25c6da63c38de1b0u64, 0xe2280b6c20dd5232u64] },
    // (ten2mx >> 256) = e2280b6c20dd5232  25c6da63c38de1b0
    //   165da29e9c9c1184f58b971f302d68ee
    BID_UINT256 { w: [0xc46fac18f3578724u64, 0x4517b54bb07cdad0u64, 0x1e38aeb6360b1af3u64, 0xb4ecd5f01a4aa828u64] },
    // (ten2mx >> 256) = b4ecd5f01a4aa828  1e38aeb6360b1af3
    //   4517b54bb07cdad0c46fac18f3578724
    BID_UINT256 { w: [0x36bfbce0c2ac6c1du64, 0x9dac910959fd7bdau64, 0xb1c6f22b5e6f48c2u64, 0x90bd77f3483bb9b9u64] },
    // (ten2mx >> 256) = 90bd77f3483bb9b9  b1c6f22b5e6f48c2
    //   9dac910959fd7bda36bfbce0c2ac6c1d
    BID_UINT256 { w: [0x2465fb01377a4695u64, 0x2f7a81a88ffbf95du64, 0xb60b1d1230b20e04u64, 0xe7958cb87392c2c2u64] }
    // (ten2mx >> 256) = e7958cb87392c2c2  b60b1d1230b20e04
    //   2f7a81a88ffbf95d2465fb01377a4695
];