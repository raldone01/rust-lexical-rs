//! Pre-computed powers for the Grisu algorithm.

#![cfg(feature = "compact")]
#![doc(hidden)]

/// Cached powers of ten as specified by the Grisu algorithm.
///
/// Cached powers of `10^k`, calculated as if by:
/// `ceil((alpha-e+63) * ONE_LOG_TEN);`
///
/// The estimation of the exponents can be trivially shown to be true,
/// using the following Python code:
///
/// ```text
/// import math
///
/// def power(x):
///     '''Calculate the binary power from the decimal one.'''
///     return ((x * (152_170 + 65536)) >> 16) - 63
///
/// def power_data(decimal, binary):
///     '''Calculate binary power and get useful data.'''
///
///     binary_calc = power(decimal)
///     return (decimal, binary, binary_calc, binary - binary_calc)
///
/// def run():
///     '''Run our exponent estimation over the entire input.'''
///
///     for index, (mant, b) in enumerate(GRISU_POWERS_OF_TEN):
///         e = index * 8 - 348
///         # Check our decimal exponent approximation is valid.
///         try:
///             f = mant * 2.0**b
///             if f != 0 and math.isfinite(f):
///                 assert math.log10(f) == e
///         except OverflowError:
///             pass
///         print(power_data(e, b))
///
/// GRISU_POWERS_OF_TEN = [
///     (18054884314459144840, -1220),
///     (13451937075301367670, -1193),
///     (10022474136428063862, -1166),
///     (14934650266808366570, -1140),
///     (11127181549972568877, -1113),
///     (16580792590934885855, -1087),
///     (12353653155963782858, -1060),
///     (18408377700990114895, -1034),
///     (13715310171984221708, -1007),
///     (10218702384817765436, -980),
///     (15227053142812498563, -954),
///     (11345038669416679861, -927),
///     (16905424996341287883, -901),
///     (12595523146049147757, -874),
///     (9384396036005875287, -847),
///     (13983839803942852151, -821),
///     (10418772551374772303, -794),
///     (15525180923007089351, -768),
///     (11567161174868858868, -741),
///     (17236413322193710309, -715),
///     (12842128665889583758, -688),
///     (9568131466127621947, -661),
///     (14257626930069360058, -635),
///     (10622759856335341974, -608),
///     (15829145694278690180, -582),
///     (11793632577567316726, -555),
///     (17573882009934360870, -529),
///     (13093562431584567480, -502),
///     (9755464219737475723, -475),
///     (14536774485912137811, -449),
///     (10830740992659433045, -422),
///     (16139061738043178685, -396),
///     (12024538023802026127, -369),
///     (17917957937422433684, -343),
///     (13349918974505688015, -316),
///     (9946464728195732843, -289),
///     (14821387422376473014, -263),
///     (11042794154864902060, -236),
///     (16455045573212060422, -210),
///     (12259964326927110867, -183),
///     (18268770466636286478, -157),
///     (13611294676837538539, -130),
///     (10141204801825835212, -103),
///     (15111572745182864684, -77),
///     (11258999068426240000, -50),
///     (16777216000000000000, -24),
///     (12500000000000000000, 3),
///     (9313225746154785156, 30),
///     (13877787807814456755, 56),
///     (10339757656912845936, 83),
///     (15407439555097886824, 109),
///     (11479437019748901445, 136),
///     (17105694144590052135, 162),
///     (12744735289059618216, 189),
///     (9495567745759798747, 216),
///     (14149498560666738074, 242),
///     (10542197943230523224, 269),
///     (15709099088952724970, 295),
///     (11704190886730495818, 322),
///     (17440603504673385349, 348),
///     (12994262207056124023, 375),
///     (9681479787123295682, 402),
///     (14426529090290212157, 428),
///     (10748601772107342003, 455),
///     (16016664761464807395, 481),
///     (11933345169920330789, 508),
///     (17782069995880619868, 534),
///     (13248674568444952270, 561),
///     (9871031767461413346, 588),
///     (14708983551653345445, 614),
///     (10959046745042015199, 641),
///     (16330252207878254650, 667),
///     (12166986024289022870, 694),
///     (18130221999122236476, 720),
///     (13508068024458167312, 747),
///     (10064294952495520794, 774),
///     (14996968138956309548, 800),
///     (11173611982879273257, 827),
///     (16649979327439178909, 853),
///     (12405201291620119593, 880),
///     (9242595204427927429, 907),
///     (13772540099066387757, 933),
///     (10261342003245940623, 960),
///     (15290591125556738113, 986),
///     (11392378155556871081, 1013),
///     (16975966327722178521, 1039),
///     (12648080533535911531, 1066),
/// ]
///
/// # Expected Output:
/// #   (-348, -1220, -1220, 0)
/// #   (-340, -1193, -1193, 0)
/// #   (-332, -1166, -1166, 0)
/// #   (-324, -1140, -1140, 0)
/// #   (-316, -1113, -1113, 0)
/// #   (-308, -1087, -1087, 0)
/// #   (-300, -1060, -1060, 0)
/// #   (-292, -1034, -1034, 0)
/// #   (-284, -1007, -1007, 0)
/// #   (-276, -980, -980, 0)
/// #   (-268, -954, -954, 0)
/// #   (-260, -927, -927, 0)
/// #   (-252, -901, -901, 0)
/// #   (-244, -874, -874, 0)
/// #   (-236, -847, -847, 0)
/// #   (-228, -821, -821, 0)
/// #   (-220, -794, -794, 0)
/// #   (-212, -768, -768, 0)
/// #   (-204, -741, -741, 0)
/// #   (-196, -715, -715, 0)
/// #   (-188, -688, -688, 0)
/// #   (-180, -661, -661, 0)
/// #   (-172, -635, -635, 0)
/// #   (-164, -608, -608, 0)
/// #   (-156, -582, -582, 0)
/// #   (-148, -555, -555, 0)
/// #   (-140, -529, -529, 0)
/// #   (-132, -502, -502, 0)
/// #   (-124, -475, -475, 0)
/// #   (-116, -449, -449, 0)
/// #   (-108, -422, -422, 0)
/// #   (-100, -396, -396, 0)
/// #   (-92, -369, -369, 0)
/// #   (-84, -343, -343, 0)
/// #   (-76, -316, -316, 0)
/// #   (-68, -289, -289, 0)
/// #   (-60, -263, -263, 0)
/// #   (-52, -236, -236, 0)
/// #   (-44, -210, -210, 0)
/// #   (-36, -183, -183, 0)
/// #   (-28, -157, -157, 0)
/// #   (-20, -130, -130, 0)
/// #   (-12, -103, -103, 0)
/// #   (-4, -77, -77, 0)
/// #   (4, -50, -50, 0)
/// #   (12, -24, -24, 0)
/// #   (20, 3, 3, 0)
/// #   (28, 30, 30, 0)
/// #   (36, 56, 56, 0)
/// #   (44, 83, 83, 0)
/// #   (52, 109, 109, 0)
/// #   (60, 136, 136, 0)
/// #   (68, 162, 162, 0)
/// #   (76, 189, 189, 0)
/// #   (84, 216, 216, 0)
/// #   (92, 242, 242, 0)
/// #   (100, 269, 269, 0)
/// #   (108, 295, 295, 0)
/// #   (116, 322, 322, 0)
/// #   (124, 348, 348, 0)
/// #   (132, 375, 375, 0)
/// #   (140, 402, 402, 0)
/// #   (148, 428, 428, 0)
/// #   (156, 455, 455, 0)
/// #   (164, 481, 481, 0)
/// #   (172, 508, 508, 0)
/// #   (180, 534, 534, 0)
/// #   (188, 561, 561, 0)
/// #   (196, 588, 588, 0)
/// #   (204, 614, 614, 0)
/// #   (212, 641, 641, 0)
/// #   (220, 667, 667, 0)
/// #   (228, 694, 694, 0)
/// #   (236, 720, 720, 0)
/// #   (244, 747, 747, 0)
/// #   (252, 774, 774, 0)
/// #   (260, 800, 800, 0)
/// #   (268, 827, 827, 0)
/// #   (276, 853, 853, 0)
/// #   (284, 880, 880, 0)
/// #   (292, 907, 907, 0)
/// #   (300, 933, 933, 0)
/// #   (308, 960, 960, 0)
/// #   (316, 986, 986, 0)
/// #   (324, 1013, 1013, 0)
/// #   (332, 1039, 1039, 0)
/// #   (340, 1066, 1066, 0)
///
/// if __name__ == '__main__':
///     run()
/// ```
pub const GRISU_POWERS_OF_TEN: [u64; 87] = [
    0xfa8fd5a0081c0288, // 10^-348
    0xbaaee17fa23ebf76, // 10^-340
    0x8b16fb203055ac76, // 10^-332
    0xcf42894a5dce35ea, // 10^-324
    0x9a6bb0aa55653b2d, // 10^-316
    0xe61acf033d1a45df, // 10^-308
    0xab70fe17c79ac6ca, // 10^-300
    0xff77b1fcbebcdc4f, // 10^-292
    0xbe5691ef416bd60c, // 10^-284
    0x8dd01fad907ffc3c, // 10^-276
    0xd3515c2831559a83, // 10^-268
    0x9d71ac8fada6c9b5, // 10^-260
    0xea9c227723ee8bcb, // 10^-252
    0xaecc49914078536d, // 10^-244
    0x823c12795db6ce57, // 10^-236
    0xc21094364dfb5637, // 10^-228
    0x9096ea6f3848984f, // 10^-220
    0xd77485cb25823ac7, // 10^-212
    0xa086cfcd97bf97f4, // 10^-204
    0xef340a98172aace5, // 10^-196
    0xb23867fb2a35b28e, // 10^-188
    0x84c8d4dfd2c63f3b, // 10^-180
    0xc5dd44271ad3cdba, // 10^-172
    0x936b9fcebb25c996, // 10^-164
    0xdbac6c247d62a584, // 10^-156
    0xa3ab66580d5fdaf6, // 10^-148
    0xf3e2f893dec3f126, // 10^-140
    0xb5b5ada8aaff80b8, // 10^-132
    0x87625f056c7c4a8b, // 10^-124
    0xc9bcff6034c13053, // 10^-116
    0x964e858c91ba2655, // 10^-108
    0xdff9772470297ebd, // 10^-100
    0xa6dfbd9fb8e5b88f, // 10^-92
    0xf8a95fcf88747d94, // 10^-84
    0xb94470938fa89bcf, // 10^-76
    0x8a08f0f8bf0f156b, // 10^-68
    0xcdb02555653131b6, // 10^-60
    0x993fe2c6d07b7fac, // 10^-52
    0xe45c10c42a2b3b06, // 10^-44
    0xaa242499697392d3, // 10^-36
    0xfd87b5f28300ca0e, // 10^-28
    0xbce5086492111aeb, // 10^-20
    0x8cbccc096f5088cc, // 10^-12
    0xd1b71758e219652c, // 10^-4
    0x9c40000000000000, // 10^4
    0xe8d4a51000000000, // 10^12
    0xad78ebc5ac620000, // 10^20
    0x813f3978f8940984, // 10^28
    0xc097ce7bc90715b3, // 10^36
    0x8f7e32ce7bea5c70, // 10^44
    0xd5d238a4abe98068, // 10^52
    0x9f4f2726179a2245, // 10^60
    0xed63a231d4c4fb27, // 10^68
    0xb0de65388cc8ada8, // 10^76
    0x83c7088e1aab65db, // 10^84
    0xc45d1df942711d9a, // 10^92
    0x924d692ca61be758, // 10^100
    0xda01ee641a708dea, // 10^108
    0xa26da3999aef774a, // 10^116
    0xf209787bb47d6b85, // 10^124
    0xb454e4a179dd1877, // 10^132
    0x865b86925b9bc5c2, // 10^140
    0xc83553c5c8965d3d, // 10^148
    0x952ab45cfa97a0b3, // 10^156
    0xde469fbd99a05fe3, // 10^164
    0xa59bc234db398c25, // 10^172
    0xf6c69a72a3989f5c, // 10^180
    0xb7dcbf5354e9bece, // 10^188
    0x88fcf317f22241e2, // 10^196
    0xcc20ce9bd35c78a5, // 10^204
    0x98165af37b2153df, // 10^212
    0xe2a0b5dc971f303a, // 10^220
    0xa8d9d1535ce3b396, // 10^228
    0xfb9b7cd9a4a7443c, // 10^236
    0xbb764c4ca7a44410, // 10^244
    0x8bab8eefb6409c1a, // 10^252
    0xd01fef10a657842c, // 10^260
    0x9b10a4e5e9913129, // 10^268
    0xe7109bfba19c0c9d, // 10^276
    0xac2820d9623bf429, // 10^284
    0x80444b5e7aa7cf85, // 10^292
    0xbf21e44003acdd2d, // 10^300
    0x8e679c2f5e44ff8f, // 10^308
    0xd433179d9c8cb841, // 10^316
    0x9e19db92b4e31ba9, // 10^324
    0xeb96bf6ebadf77d9, // 10^332
    0xaf87023b9bf0ee6b, // 10^340
];
