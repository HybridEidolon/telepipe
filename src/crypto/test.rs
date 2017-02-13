
use super::*;

static EXPECTED: &'static [u32] =
    &[413266057, 1000827338, 360505626, 3196339388, 3611380824, 3832591879, 2525604915,
      2225607555, 868678936, 3545210092, 3775161151, 3600362500, 3051433903, 1594241595,
      880463746, 441436159, 2370017031, 3386786995, 752155331, 2710288609, 4292876359, 3549569052,
      3495519434, 3382390979, 134403005, 2221634675, 4061918786, 1844493012, 1873431107,
      3087386970, 2778723389, 1687344094, 2606002709, 414227767, 1093072714, 553005718,
      1342816922, 1944062858, 2106074936, 418179744, 2036919241, 2812993139, 2656293768,
      3211078613, 3578986224, 4103011348, 1507603438, 1198422663, 2820919486, 2729412848,
      957291403, 2619302034, 3610731635, 2584646774, 1597001829, 3275499024, 2466288229,
      2010334986, 1310486243, 2319703158, 1621718281, 416494199, 318757998, 3856427859,
      2785573563, 4183371135, 2168826610, 1150002498, 232799984, 875379130, 251830674, 1038303105,
      900382664, 118576747, 2184599644, 4092819890, 3368234248, 1280263313, 2009480063,
      1085592906, 3915895159, 3030089185, 190570292, 1920572134, 3548343033, 2875554917,
      1986198837, 3212577962, 2145835509, 2615497054, 2921613938, 2153624526, 1496961624,
      3709859322, 2510558951, 704924813, 2406547301, 883602139, 3291422516, 1578509305, 763337804,
      1370107634, 1662066572, 4191068554, 2901496550, 1446813774, 4184584811, 3224083634,
      654770267, 191467425, 4129512874, 2243321740, 3279022067, 1911272106, 470394149, 2250681908,
      2058359290, 1553515757, 631852130, 3822431026, 646173595, 1437126091, 1926526994,
      1194618332, 505933404, 866278043, 3814279989, 907776026, 4034537705, 163765552, 1556928994,
      3457141429, 3560084157, 689757919, 1603144313, 1862262956, 4159480133, 977696035,
      3758038815, 3604776233, 953390814, 382355443, 1530653526, 3248547486, 3436764042,
      3093052132, 544365704, 3507837805, 2348761636, 3580611659, 3133410126, 2255713470,
      3490218734, 1922408916, 3816447579, 1821454361, 4161624530, 2533768433, 1856741636,
      3318444407, 2326600986, 1341207303, 1038158377, 2039054468, 3477896997, 3710307011,
      4177242310, 1605931388, 13669693, 2011815917, 2644009666, 2955815514, 3167890678,
      1436135400, 758053432, 2941001600, 344541454, 2578362775, 439370002, 244090322, 1290478573,
      3724914824, 3220819292, 3705016596, 1658222942, 4235038129, 178471296, 1811633813,
      1185234819, 1032844116, 3382273065, 3587097126, 365201026, 2458407386, 1493518842,
      3489796024, 958691881, 3486218481, 2341766167, 637202207, 2949350435, 11907813, 3627082565,
      402988782, 1386630257, 2469956488, 962348858, 766943323, 1052525080, 2146764336, 2463728788,
      1873905043, 3014465454, 2806706706, 3750705232, 3558720807, 1539973349, 1246394940,
      950428237, 2586468546, 3976410084, 3586830241, 299237762, 2361449170, 2707391731,
      2908782192, 3039367858, 4280207984, 919018747, 3784620418, 3908795934, 3230757116,
      1396479468, 563720401, 1065739232, 424212176, 2017359689, 2320798238, 1524398366,
      2610567539, 4073371313, 2333869710, 3007746567, 3929611618, 3525476076, 2949821616,
      1856424207, 1640399287, 535764989, 3925857061, 2170556884, 1902949807, 424716948,
      3180691677, 2998464225, 1039475693, 2215397046, 3704211637, 2609933308, 2556786546,
      688678477, 1599835927, 124574835, 2161917747, 1532217532, 2779199317, 929531741, 3714003207,
      181639506, 1083003794, 777261029, 1590359412, 2819158871, 4083586995, 2846580614,
      1474784573, 4008962830, 3361490633, 1138749607, 2048892600, 3820107612, 3185632455,
      393959172, 3113613468, 974164304, 2469451769, 1512570366, 2827524228, 316447997, 3106222006,
      1626470937, 2748975535, 1026388058, 3125370489, 3739791745, 2371206734, 3508823893,
      2138740519, 485567116, 2662195312, 3501254712, 2029613139, 2220123053, 2063607777,
      956909999, 1205197624, 2632295365, 2419347137, 1202378873, 1794637014, 1444448398,
      2531140177, 2981539938, 453578025, 2290578876, 3469222732, 4137712440, 3936094080,
      3277729201, 367242658, 3850316038, 853328875, 2928849210, 1281010461, 754548337, 273417615,
      2070350283, 1009542874, 351583618, 637090052, 2973596350, 1868177071, 3374403218,
      1431568529, 2503323527, 1304387153, 2630738373, 534024100, 3926116092, 1998973865,
      4189296406, 3241914644, 109445776, 3810006179, 2383572733, 1325437537, 3445099740,
      2456789459, 3314233716, 2359401332, 3295347107, 130932236, 788773116, 3448525499, 536433496,
      1640153517, 3040745574, 1046382014, 3026044607, 4230761141, 2917876610, 3545023435,
      3819597773, 2380486685, 1677827461, 3729247560, 1680210120, 3047562791, 3014762491,
      3453442514, 2421728635, 1013466139, 3937590039, 3652056149, 116906678, 1493814419, 64960818,
      3266748243, 658983524, 3246504904, 3476273926, 232560864, 2850982428, 3451148563,
      3731718620, 596339312, 3401795106, 2002612736, 2057841517, 4047830423, 3687286394,
      2162885276, 3379227904, 1342944847, 4187493866, 3422082359, 798969066, 2895749798,
      3704444387, 3534458831, 1531230479, 3042278586, 2368740633, 2618012576, 2620843957,
      718161355, 3777209864, 3697529737, 2449958655, 305095460, 903065390, 3233174340, 1528929506,
      779604019, 2098837145, 2364597086, 1801273602, 3968330684, 2982245298, 1024423531,
      3980487960, 936643130, 3525614569, 3595217408, 310618547, 1829420546, 4279261141,
      1752173820, 3395132466, 3142232026, 2733910785, 3996275568, 1091042768, 3228679717,
      506027676, 3342326055, 4072765841, 2128253844, 1672708569, 2539168908, 2538726744,
      1323786247, 1335420972, 2771177514, 3679059684, 3256114103, 793059746, 2947692683,
      395492213, 4294083498, 3984430853, 2736655593, 820472917, 4166517020, 845797596, 3735188622,
      2565463164, 885960652, 929653688, 557577680, 878072111, 1178032898, 2644572562, 1291264142,
      159181267, 3011711122, 1727376985, 3827573087, 2428261097, 3124788969, 878266885,
      1513495359, 494563281, 593496198, 3313220663, 426451405, 4050717413, 1725043016, 3885809972,
      783989412, 1774060752, 2154284376, 3377774479, 3852905887, 1248785334, 1051972489,
      1244266284, 1216668705, 3607779606, 1067182354, 2096714956, 1735778652, 2173524441,
      4080939217, 1464729265, 3445269484, 2674818286, 4147007244, 1531635027, 2629318149,
      1394409852, 2281855125, 1286032490, 3666021912, 3395849012, 1098335936, 3360242282,
      2927263898, 3819062, 3972258094, 2221440977, 3700347520, 717317566, 1572532889, 3684128747,
      1918438384, 1894810992, 3467936860, 2215827450, 2977928829, 3172639288, 0, 0, 0, 0, 0, 0, 0];

#[test]
fn test_create_crypto() {
    // Tests to ensure the key table generated during initialization matches an expected table,
    // given a specific seed.
    let seed = 0;
    let cipher = Cipher::new(seed);

    assert_eq!(&cipher.keys[0..128], &EXPECTED[0..128]);
    assert_eq!(&cipher.keys[128..256], &EXPECTED[128..256]);
    assert_eq!(&cipher.keys[256..384], &EXPECTED[256..384]);
    assert_eq!(&cipher.keys[384..512], &EXPECTED[384..512]);
    assert_eq!(&cipher.keys[512..528], &EXPECTED[512..528]);
}

#[test]
fn test_crypto_symmetricity() {
    // Tests to ensure the cipher has the symmetric property.
    let seed = 100;
    let mut cipher = Cipher::new(seed);

    let start = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let mut data = start;
    cipher.codec(&mut data).unwrap();
    cipher = Cipher::new(seed);
    cipher.codec(&mut data).unwrap();
    assert_eq!(&start[..], &data[..]);
}

#[test]
fn test_crypto_correct_ciphertext() {
    // Tests to ensure the cipher produces correct ciphertext from a given seed.
    let seed = 82376;
    let mut cipher = Cipher::new(seed);

    let start = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let expected = [187, 8, 83, 1, 32, 37, 47, 172, 14, 194, 240, 8, 106, 129, 148, 137];

    let mut data = start;
    cipher.codec(&mut data).unwrap();
    assert_eq!(&expected[..], &data[..]);
}

#[test]
fn test_codec_buffer_size() {
    let seed = 512;
    let mut cipher = Cipher::new(seed);

    assert_eq!(cipher.codec(&mut vec![0; 3]),
               Err(CodecError::IllegalBufferSize));
    assert_eq!(cipher.codec(&mut Vec::new()), Ok(()));
    assert_eq!(cipher.codec(&mut vec![0; 512]), Ok(()));
}