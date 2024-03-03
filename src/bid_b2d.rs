/* ---------------------------------------------------------------------------------------------------*/
/* Port of the Intel Decimal Floating-Point Math Library decimal128 type to Rust.                     */
/* decmathlib-rs - Copyright (C) 2023-2024 Carlos Guzmán Álvarez                                      */
/* -------------------------------------------------------------------------------------------------- */
/* Licensed under the MIT license. See LICENSE file in the project root for fu64 license information. */
/* -------------------------------------------------------------------------------------------------- */
/* Intel® Decimal Floating-Point Math Library - Copyright (c) 2018, Intel Corp.                       */
/* -------------------------------------------------------------------------------------------------- */

use crate::bid_internal::BID_UINT64;

pub (crate) const BID_D2B: [BID_UINT64; 1024] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 80, 81, 800, 801, 880, 881,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 90, 91, 810, 811, 890, 891,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 82, 83, 820, 821, 808, 809,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 92, 93, 830, 831, 818, 819,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 84, 85, 840, 841, 88, 89,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 94, 95, 850, 851, 98, 99,
    60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 86, 87, 860, 861, 888, 889,
    70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 96, 97, 870, 871, 898, 899,
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 180, 181, 900, 901,
    980, 981,
    110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 190, 191, 910, 911,
    990, 991,
    120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 182, 183, 920, 921,
    908, 909,
    130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 192, 193, 930, 931,
    918, 919,
    140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 184, 185, 940, 941,
    188, 189,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 194, 195, 950, 951,
    198, 199,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 186, 187, 960, 961,
    988, 989,
    170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 196, 197, 970, 971,
    998, 999,
    200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 280, 281, 802, 803,
    882, 883,
    210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 290, 291, 812, 813,
    892, 893,
    220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 282, 283, 822, 823,
    828, 829,
    230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 292, 293, 832, 833,
    838, 839,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 284, 285, 842, 843,
    288, 289,
    250, 251, 252, 253, 254, 255, 256, 257, 258, 259, 294, 295, 852, 853,
    298, 299,
    260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 286, 287, 862, 863,
    888, 889,
    270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 296, 297, 872, 873,
    898, 899,
    300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 380, 381, 902, 903,
    982, 983,
    310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 390, 391, 912, 913,
    992, 993,
    320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 382, 383, 922, 923,
    928, 929,
    330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 392, 393, 932, 933,
    938, 939,
    340, 341, 342, 343, 344, 345, 346, 347, 348, 349, 384, 385, 942, 943,
    388, 389,
    350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 394, 395, 952, 953,
    398, 399,
    360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 386, 387, 962, 963,
    988, 989,
    370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 396, 397, 972, 973,
    998, 999,
    400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 480, 481, 804, 805,
    884, 885,
    410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 490, 491, 814, 815,
    894, 895,
    420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 482, 483, 824, 825,
    848, 849,
    430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 492, 493, 834, 835,
    858, 859,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 484, 485, 844, 845,
    488, 489,
    450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 494, 495, 854, 855,
    498, 499,
    460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 486, 487, 864, 865,
    888, 889,
    470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 496, 497, 874, 875,
    898, 899,
    500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 580, 581, 904, 905,
    984, 985,
    510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 590, 591, 914, 915,
    994, 995,
    520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 582, 583, 924, 925,
    948, 949,
    530, 531, 532, 533, 534, 535, 536, 537, 538, 539, 592, 593, 934, 935,
    958, 959,
    540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 584, 585, 944, 945,
    588, 589,
    550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 594, 595, 954, 955,
    598, 599,
    560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 586, 587, 964, 965,
    988, 989,
    570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 596, 597, 974, 975,
    998, 999,
    600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 680, 681, 806, 807,
    886, 887,
    610, 611, 612, 613, 614, 615, 616, 617, 618, 619, 690, 691, 816, 817,
    896, 897,
    620, 621, 622, 623, 624, 625, 626, 627, 628, 629, 682, 683, 826, 827,
    868, 869,
    630, 631, 632, 633, 634, 635, 636, 637, 638, 639, 692, 693, 836, 837,
    878, 879,
    640, 641, 642, 643, 644, 645, 646, 647, 648, 649, 684, 685, 846, 847,
    688, 689,
    650, 651, 652, 653, 654, 655, 656, 657, 658, 659, 694, 695, 856, 857,
    698, 699,
    660, 661, 662, 663, 664, 665, 666, 667, 668, 669, 686, 687, 866, 867,
    888, 889,
    670, 671, 672, 673, 674, 675, 676, 677, 678, 679, 696, 697, 876, 877,
    898, 899,
    700, 701, 702, 703, 704, 705, 706, 707, 708, 709, 780, 781, 906, 907,
    986, 987,
    710, 711, 712, 713, 714, 715, 716, 717, 718, 719, 790, 791, 916, 917,
    996, 997,
    720, 721, 722, 723, 724, 725, 726, 727, 728, 729, 782, 783, 926, 927,
    968, 969,
    730, 731, 732, 733, 734, 735, 736, 737, 738, 739, 792, 793, 936, 937,
    978, 979,
    740, 741, 742, 743, 744, 745, 746, 747, 748, 749, 784, 785, 946, 947,
    788, 789,
    750, 751, 752, 753, 754, 755, 756, 757, 758, 759, 794, 795, 956, 957,
    798, 799,
    760, 761, 762, 763, 764, 765, 766, 767, 768, 769, 786, 787, 966, 967,
    988, 989,
    770, 771, 772, 773, 774, 775, 776, 777, 778, 779, 796, 797, 976, 977,
    998, 999
];

pub (crate) const BID_B2D: [BID_UINT64; 1000] = [
    0x000u64, 0x001u64, 0x002u64, 0x003u64, 0x004u64, 0x005u64,
    0x006u64, 0x007u64, 0x008u64, 0x009u64,
    0x010u64, 0x011u64, 0x012u64, 0x013u64, 0x014u64, 0x015u64, 0x016u64,
    0x017u64, 0x018u64, 0x019u64,
    0x020u64, 0x021u64, 0x022u64, 0x023u64, 0x024u64, 0x025u64, 0x026u64,
    0x027u64, 0x028u64, 0x029u64,
    0x030u64, 0x031u64, 0x032u64, 0x033u64, 0x034u64, 0x035u64, 0x036u64,
    0x037u64, 0x038u64, 0x039u64,
    0x040u64, 0x041u64, 0x042u64, 0x043u64, 0x044u64, 0x045u64, 0x046u64,
    0x047u64, 0x048u64, 0x049u64,
    0x050u64, 0x051u64, 0x052u64, 0x053u64, 0x054u64, 0x055u64, 0x056u64,
    0x057u64, 0x058u64, 0x059u64,
    0x060u64, 0x061u64, 0x062u64, 0x063u64, 0x064u64, 0x065u64, 0x066u64,
    0x067u64, 0x068u64, 0x069u64,
    0x070u64, 0x071u64, 0x072u64, 0x073u64, 0x074u64, 0x075u64, 0x076u64,
    0x077u64, 0x078u64, 0x079u64,
    0x00au64, 0x00bu64, 0x02au64, 0x02bu64, 0x04au64, 0x04bu64, 0x06au64,
    0x06bu64, 0x04eu64, 0x04fu64,
    0x01au64, 0x01bu64, 0x03au64, 0x03bu64, 0x05au64, 0x05bu64, 0x07au64,
    0x07bu64, 0x05eu64, 0x05fu64,
    0x080u64, 0x081u64, 0x082u64, 0x083u64, 0x084u64, 0x085u64, 0x086u64,
    0x087u64, 0x088u64, 0x089u64,
    0x090u64, 0x091u64, 0x092u64, 0x093u64, 0x094u64, 0x095u64, 0x096u64,
    0x097u64, 0x098u64, 0x099u64,
    0x0a0u64, 0x0a1u64, 0x0a2u64, 0x0a3u64, 0x0a4u64, 0x0a5u64, 0x0a6u64,
    0x0a7u64, 0x0a8u64, 0x0a9u64,
    0x0b0u64, 0x0b1u64, 0x0b2u64, 0x0b3u64, 0x0b4u64, 0x0b5u64, 0x0b6u64,
    0x0b7u64, 0x0b8u64, 0x0b9u64,
    0x0c0u64, 0x0c1u64, 0x0c2u64, 0x0c3u64, 0x0c4u64, 0x0c5u64, 0x0c6u64,
    0x0c7u64, 0x0c8u64, 0x0c9u64,
    0x0d0u64, 0x0d1u64, 0x0d2u64, 0x0d3u64, 0x0d4u64, 0x0d5u64, 0x0d6u64,
    0x0d7u64, 0x0d8u64, 0x0d9u64,
    0x0e0u64, 0x0e1u64, 0x0e2u64, 0x0e3u64, 0x0e4u64, 0x0e5u64, 0x0e6u64,
    0x0e7u64, 0x0e8u64, 0x0e9u64,
    0x0f0u64, 0x0f1u64, 0x0f2u64, 0x0f3u64, 0x0f4u64, 0x0f5u64, 0x0f6u64,
    0x0f7u64, 0x0f8u64, 0x0f9u64,
    0x08au64, 0x08bu64, 0x0aau64, 0x0abu64, 0x0cau64, 0x0cbu64, 0x0eau64,
    0x0ebu64, 0x0ceu64, 0x0cfu64,
    0x09au64, 0x09bu64, 0x0bau64, 0x0bbu64, 0x0dau64, 0x0dbu64, 0x0fau64,
    0x0fbu64, 0x0deu64, 0x0dfu64,
    0x100u64, 0x101u64, 0x102u64, 0x103u64, 0x104u64, 0x105u64, 0x106u64,
    0x107u64, 0x108u64, 0x109u64,
    0x110u64, 0x111u64, 0x112u64, 0x113u64, 0x114u64, 0x115u64, 0x116u64,
    0x117u64, 0x118u64, 0x119u64,
    0x120u64, 0x121u64, 0x122u64, 0x123u64, 0x124u64, 0x125u64, 0x126u64,
    0x127u64, 0x128u64, 0x129u64,
    0x130u64, 0x131u64, 0x132u64, 0x133u64, 0x134u64, 0x135u64, 0x136u64,
    0x137u64, 0x138u64, 0x139u64,
    0x140u64, 0x141u64, 0x142u64, 0x143u64, 0x144u64, 0x145u64, 0x146u64,
    0x147u64, 0x148u64, 0x149u64,
    0x150u64, 0x151u64, 0x152u64, 0x153u64, 0x154u64, 0x155u64, 0x156u64,
    0x157u64, 0x158u64, 0x159u64,
    0x160u64, 0x161u64, 0x162u64, 0x163u64, 0x164u64, 0x165u64, 0x166u64,
    0x167u64, 0x168u64, 0x169u64,
    0x170u64, 0x171u64, 0x172u64, 0x173u64, 0x174u64, 0x175u64, 0x176u64,
    0x177u64, 0x178u64, 0x179u64,
    0x10au64, 0x10bu64, 0x12au64, 0x12bu64, 0x14au64, 0x14bu64, 0x16au64,
    0x16bu64, 0x14eu64, 0x14fu64,
    0x11au64, 0x11bu64, 0x13au64, 0x13bu64, 0x15au64, 0x15bu64, 0x17au64,
    0x17bu64, 0x15eu64, 0x15fu64,
    0x180u64, 0x181u64, 0x182u64, 0x183u64, 0x184u64, 0x185u64, 0x186u64,
    0x187u64, 0x188u64, 0x189u64,
    0x190u64, 0x191u64, 0x192u64, 0x193u64, 0x194u64, 0x195u64, 0x196u64,
    0x197u64, 0x198u64, 0x199u64,
    0x1a0u64, 0x1a1u64, 0x1a2u64, 0x1a3u64, 0x1a4u64, 0x1a5u64, 0x1a6u64,
    0x1a7u64, 0x1a8u64, 0x1a9u64,
    0x1b0u64, 0x1b1u64, 0x1b2u64, 0x1b3u64, 0x1b4u64, 0x1b5u64, 0x1b6u64,
    0x1b7u64, 0x1b8u64, 0x1b9u64,
    0x1c0u64, 0x1c1u64, 0x1c2u64, 0x1c3u64, 0x1c4u64, 0x1c5u64, 0x1c6u64,
    0x1c7u64, 0x1c8u64, 0x1c9u64,
    0x1d0u64, 0x1d1u64, 0x1d2u64, 0x1d3u64, 0x1d4u64, 0x1d5u64, 0x1d6u64,
    0x1d7u64, 0x1d8u64, 0x1d9u64,
    0x1e0u64, 0x1e1u64, 0x1e2u64, 0x1e3u64, 0x1e4u64, 0x1e5u64, 0x1e6u64,
    0x1e7u64, 0x1e8u64, 0x1e9u64,
    0x1f0u64, 0x1f1u64, 0x1f2u64, 0x1f3u64, 0x1f4u64, 0x1f5u64, 0x1f6u64,
    0x1f7u64, 0x1f8u64, 0x1f9u64,
    0x18au64, 0x18bu64, 0x1aau64, 0x1abu64, 0x1cau64, 0x1cbu64, 0x1eau64,
    0x1ebu64, 0x1ceu64, 0x1cfu64,
    0x19au64, 0x19bu64, 0x1bau64, 0x1bbu64, 0x1dau64, 0x1dbu64, 0x1fau64,
    0x1fbu64, 0x1deu64, 0x1dfu64,
    0x200u64, 0x201u64, 0x202u64, 0x203u64, 0x204u64, 0x205u64, 0x206u64,
    0x207u64, 0x208u64, 0x209u64,
    0x210u64, 0x211u64, 0x212u64, 0x213u64, 0x214u64, 0x215u64, 0x216u64,
    0x217u64, 0x218u64, 0x219u64,
    0x220u64, 0x221u64, 0x222u64, 0x223u64, 0x224u64, 0x225u64, 0x226u64,
    0x227u64, 0x228u64, 0x229u64,
    0x230u64, 0x231u64, 0x232u64, 0x233u64, 0x234u64, 0x235u64, 0x236u64,
    0x237u64, 0x238u64, 0x239u64,
    0x240u64, 0x241u64, 0x242u64, 0x243u64, 0x244u64, 0x245u64, 0x246u64,
    0x247u64, 0x248u64, 0x249u64,
    0x250u64, 0x251u64, 0x252u64, 0x253u64, 0x254u64, 0x255u64, 0x256u64,
    0x257u64, 0x258u64, 0x259u64,
    0x260u64, 0x261u64, 0x262u64, 0x263u64, 0x264u64, 0x265u64, 0x266u64,
    0x267u64, 0x268u64, 0x269u64,
    0x270u64, 0x271u64, 0x272u64, 0x273u64, 0x274u64, 0x275u64, 0x276u64,
    0x277u64, 0x278u64, 0x279u64,
    0x20au64, 0x20bu64, 0x22au64, 0x22bu64, 0x24au64, 0x24bu64, 0x26au64,
    0x26bu64, 0x24eu64, 0x24fu64,
    0x21au64, 0x21bu64, 0x23au64, 0x23bu64, 0x25au64, 0x25bu64, 0x27au64,
    0x27bu64, 0x25eu64, 0x25fu64,
    0x280u64, 0x281u64, 0x282u64, 0x283u64, 0x284u64, 0x285u64, 0x286u64,
    0x287u64, 0x288u64, 0x289u64,
    0x290u64, 0x291u64, 0x292u64, 0x293u64, 0x294u64, 0x295u64, 0x296u64,
    0x297u64, 0x298u64, 0x299u64,
    0x2a0u64, 0x2a1u64, 0x2a2u64, 0x2a3u64, 0x2a4u64, 0x2a5u64, 0x2a6u64,
    0x2a7u64, 0x2a8u64, 0x2a9u64,
    0x2b0u64, 0x2b1u64, 0x2b2u64, 0x2b3u64, 0x2b4u64, 0x2b5u64, 0x2b6u64,
    0x2b7u64, 0x2b8u64, 0x2b9u64,
    0x2c0u64, 0x2c1u64, 0x2c2u64, 0x2c3u64, 0x2c4u64, 0x2c5u64, 0x2c6u64,
    0x2c7u64, 0x2c8u64, 0x2c9u64,
    0x2d0u64, 0x2d1u64, 0x2d2u64, 0x2d3u64, 0x2d4u64, 0x2d5u64, 0x2d6u64,
    0x2d7u64, 0x2d8u64, 0x2d9u64,
    0x2e0u64, 0x2e1u64, 0x2e2u64, 0x2e3u64, 0x2e4u64, 0x2e5u64, 0x2e6u64,
    0x2e7u64, 0x2e8u64, 0x2e9u64,
    0x2f0u64, 0x2f1u64, 0x2f2u64, 0x2f3u64, 0x2f4u64, 0x2f5u64, 0x2f6u64,
    0x2f7u64, 0x2f8u64, 0x2f9u64,
    0x28au64, 0x28bu64, 0x2aau64, 0x2abu64, 0x2cau64, 0x2cbu64, 0x2eau64,
    0x2ebu64, 0x2ceu64, 0x2cfu64,
    0x29au64, 0x29bu64, 0x2bau64, 0x2bbu64, 0x2dau64, 0x2dbu64, 0x2fau64,
    0x2fbu64, 0x2deu64, 0x2dfu64,
    0x300u64, 0x301u64, 0x302u64, 0x303u64, 0x304u64, 0x305u64, 0x306u64,
    0x307u64, 0x308u64, 0x309u64,
    0x310u64, 0x311u64, 0x312u64, 0x313u64, 0x314u64, 0x315u64, 0x316u64,
    0x317u64, 0x318u64, 0x319u64,
    0x320u64, 0x321u64, 0x322u64, 0x323u64, 0x324u64, 0x325u64, 0x326u64,
    0x327u64, 0x328u64, 0x329u64,
    0x330u64, 0x331u64, 0x332u64, 0x333u64, 0x334u64, 0x335u64, 0x336u64,
    0x337u64, 0x338u64, 0x339u64,
    0x340u64, 0x341u64, 0x342u64, 0x343u64, 0x344u64, 0x345u64, 0x346u64,
    0x347u64, 0x348u64, 0x349u64,
    0x350u64, 0x351u64, 0x352u64, 0x353u64, 0x354u64, 0x355u64, 0x356u64,
    0x357u64, 0x358u64, 0x359u64,
    0x360u64, 0x361u64, 0x362u64, 0x363u64, 0x364u64, 0x365u64, 0x366u64,
    0x367u64, 0x368u64, 0x369u64,
    0x370u64, 0x371u64, 0x372u64, 0x373u64, 0x374u64, 0x375u64, 0x376u64,
    0x377u64, 0x378u64, 0x379u64,
    0x30au64, 0x30bu64, 0x32au64, 0x32bu64, 0x34au64, 0x34bu64, 0x36au64,
    0x36bu64, 0x34eu64, 0x34fu64,
    0x31au64, 0x31bu64, 0x33au64, 0x33bu64, 0x35au64, 0x35bu64, 0x37au64,
    0x37bu64, 0x35eu64, 0x35fu64,
    0x380u64, 0x381u64, 0x382u64, 0x383u64, 0x384u64, 0x385u64, 0x386u64,
    0x387u64, 0x388u64, 0x389u64,
    0x390u64, 0x391u64, 0x392u64, 0x393u64, 0x394u64, 0x395u64, 0x396u64,
    0x397u64, 0x398u64, 0x399u64,
    0x3a0u64, 0x3a1u64, 0x3a2u64, 0x3a3u64, 0x3a4u64, 0x3a5u64, 0x3a6u64,
    0x3a7u64, 0x3a8u64, 0x3a9u64,
    0x3b0u64, 0x3b1u64, 0x3b2u64, 0x3b3u64, 0x3b4u64, 0x3b5u64, 0x3b6u64,
    0x3b7u64, 0x3b8u64, 0x3b9u64,
    0x3c0u64, 0x3c1u64, 0x3c2u64, 0x3c3u64, 0x3c4u64, 0x3c5u64, 0x3c6u64,
    0x3c7u64, 0x3c8u64, 0x3c9u64,
    0x3d0u64, 0x3d1u64, 0x3d2u64, 0x3d3u64, 0x3d4u64, 0x3d5u64, 0x3d6u64,
    0x3d7u64, 0x3d8u64, 0x3d9u64,
    0x3e0u64, 0x3e1u64, 0x3e2u64, 0x3e3u64, 0x3e4u64, 0x3e5u64, 0x3e6u64,
    0x3e7u64, 0x3e8u64, 0x3e9u64,
    0x3f0u64, 0x3f1u64, 0x3f2u64, 0x3f3u64, 0x3f4u64, 0x3f5u64, 0x3f6u64,
    0x3f7u64, 0x3f8u64, 0x3f9u64,
    0x38au64, 0x38bu64, 0x3aau64, 0x3abu64, 0x3cau64, 0x3cbu64, 0x3eau64,
    0x3ebu64, 0x3ceu64, 0x3cfu64,
    0x39au64, 0x39bu64, 0x3bau64, 0x3bbu64, 0x3dau64, 0x3dbu64, 0x3fau64,
    0x3fbu64, 0x3deu64, 0x3dfu64,
    0x00cu64, 0x00du64, 0x10cu64, 0x10du64, 0x20cu64, 0x20du64, 0x30cu64,
    0x30du64, 0x02eu64, 0x02fu64,
    0x01cu64, 0x01du64, 0x11cu64, 0x11du64, 0x21cu64, 0x21du64, 0x31cu64,
    0x31du64, 0x03eu64, 0x03fu64,
    0x02cu64, 0x02du64, 0x12cu64, 0x12du64, 0x22cu64, 0x22du64, 0x32cu64,
    0x32du64, 0x12eu64, 0x12fu64,
    0x03cu64, 0x03du64, 0x13cu64, 0x13du64, 0x23cu64, 0x23du64, 0x33cu64,
    0x33du64, 0x13eu64, 0x13fu64,
    0x04cu64, 0x04du64, 0x14cu64, 0x14du64, 0x24cu64, 0x24du64, 0x34cu64,
    0x34du64, 0x22eu64, 0x22fu64,
    0x05cu64, 0x05du64, 0x15cu64, 0x15du64, 0x25cu64, 0x25du64, 0x35cu64,
    0x35du64, 0x23eu64, 0x23fu64,
    0x06cu64, 0x06du64, 0x16cu64, 0x16du64, 0x26cu64, 0x26du64, 0x36cu64,
    0x36du64, 0x32eu64, 0x32fu64,
    0x07cu64, 0x07du64, 0x17cu64, 0x17du64, 0x27cu64, 0x27du64, 0x37cu64,
    0x37du64, 0x33eu64, 0x33fu64,
    0x00eu64, 0x00fu64, 0x10eu64, 0x10fu64, 0x20eu64, 0x20fu64, 0x30eu64,
    0x30fu64, 0x06eu64, 0x06fu64,
    0x01eu64, 0x01fu64, 0x11eu64, 0x11fu64, 0x21eu64, 0x21fu64, 0x31eu64,
    0x31fu64, 0x07eu64, 0x07fu64,
    0x08cu64, 0x08du64, 0x18cu64, 0x18du64, 0x28cu64, 0x28du64, 0x38cu64,
    0x38du64, 0x0aeu64, 0x0afu64,
    0x09cu64, 0x09du64, 0x19cu64, 0x19du64, 0x29cu64, 0x29du64, 0x39cu64,
    0x39du64, 0x0beu64, 0x0bfu64,
    0x0acu64, 0x0adu64, 0x1acu64, 0x1adu64, 0x2acu64, 0x2adu64, 0x3acu64,
    0x3adu64, 0x1aeu64, 0x1afu64,
    0x0bcu64, 0x0bdu64, 0x1bcu64, 0x1bdu64, 0x2bcu64, 0x2bdu64, 0x3bcu64,
    0x3bdu64, 0x1beu64, 0x1bfu64,
    0x0ccu64, 0x0cdu64, 0x1ccu64, 0x1cdu64, 0x2ccu64, 0x2cdu64, 0x3ccu64,
    0x3cdu64, 0x2aeu64, 0x2afu64,
    0x0dcu64, 0x0ddu64, 0x1dcu64, 0x1ddu64, 0x2dcu64, 0x2ddu64, 0x3dcu64,
    0x3ddu64, 0x2beu64, 0x2bfu64,
    0x0ecu64, 0x0edu64, 0x1ecu64, 0x1edu64, 0x2ecu64, 0x2edu64, 0x3ecu64,
    0x3edu64, 0x3aeu64, 0x3afu64,
    0x0fcu64, 0x0fdu64, 0x1fcu64, 0x1fdu64, 0x2fcu64, 0x2fdu64, 0x3fcu64,
    0x3fdu64, 0x3beu64, 0x3bfu64,
    0x08eu64, 0x08fu64, 0x18eu64, 0x18fu64, 0x28eu64, 0x28fu64, 0x38eu64,
    0x38fu64, 0x0eeu64, 0x0efu64,
    0x09eu64, 0x09fu64, 0x19eu64, 0x19fu64, 0x29eu64, 0x29fu64, 0x39eu64,
    0x39fu64, 0x0feu64, 0x0ffu64
];
