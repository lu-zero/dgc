#[cfg(test)]
use dgc::*;
// Tests the library against some of the test data available at <https://github.com/eu-digital-green-certificates/dgc-testdata>
use rstest::rstest;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[rstest]
#[case::common_2dcode_raw_b1_json("common/2DCode/raw/B1.json")]
#[case::common_2dcode_raw_cbo1_json("common/2DCode/raw/CBO1.json")]
#[case::common_2dcode_raw_cbo2_json("common/2DCode/raw/CBO2.json")]
#[case::common_2dcode_raw_co1_json("common/2DCode/raw/CO1.json")]
#[case::common_2dcode_raw_co10_json("common/2DCode/raw/CO10.json")]
#[case::common_2dcode_raw_co11_json("common/2DCode/raw/CO11.json")]
#[case::common_2dcode_raw_co12_json("common/2DCode/raw/CO12.json")]
#[case::common_2dcode_raw_co13_json("common/2DCode/raw/CO13.json")]
#[case::common_2dcode_raw_co14_json("common/2DCode/raw/CO14.json")]
#[case::common_2dcode_raw_co15_json("common/2DCode/raw/CO15.json")]
#[case::common_2dcode_raw_co16_json("common/2DCode/raw/CO16.json")]
#[case::common_2dcode_raw_co17_json("common/2DCode/raw/CO17.json")]
#[case::common_2dcode_raw_co18_json("common/2DCode/raw/CO18.json")]
// #[case::common_2dcode_raw_co19_json("common/2DCode/raw/CO19.json")] // SKIPPED: issue #1
#[case::common_2dcode_raw_co2_json("common/2DCode/raw/CO2.json")]
// #[case::common_2dcode_raw_co20_json("common/2DCode/raw/CO20.json")] // SKIPPED: issue #1
#[case::common_2dcode_raw_co21_json("common/2DCode/raw/CO21.json")]
#[case::common_2dcode_raw_co22_json("common/2DCode/raw/CO22.json")]
#[case::common_2dcode_raw_co23_json("common/2DCode/raw/CO23.json")]
// #[case::common_2dcode_raw_co28_json("common/2DCode/raw/CO28.json")] // SKIPPED: flaked test. It says it should verify fine but it's not a valid CWT (invalid tag)
#[case::common_2dcode_raw_co3_json("common/2DCode/raw/CO3.json")]
#[case::common_2dcode_raw_co5_json("common/2DCode/raw/CO5.json")]
#[case::common_2dcode_raw_co6_json("common/2DCode/raw/CO6.json")]
#[case::common_2dcode_raw_co7_json("common/2DCode/raw/CO7.json")]
#[case::common_2dcode_raw_co8_json("common/2DCode/raw/CO8.json")]
#[case::common_2dcode_raw_co9_json("common/2DCode/raw/CO9.json")]
#[case::common_2dcode_raw_dgc1_json("common/2DCode/raw/DGC1.json")]
#[case::common_2dcode_raw_dgc2_json("common/2DCode/raw/DGC2.json")]
#[case::common_2dcode_raw_dgc3_json("common/2DCode/raw/DGC3.json")]
#[case::common_2dcode_raw_dgc4_json("common/2DCode/raw/DGC4.json")]
#[case::common_2dcode_raw_dgc5_json("common/2DCode/raw/DGC5.json")]
#[case::common_2dcode_raw_dgc6_json("common/2DCode/raw/DGC6.json")]
#[case::common_2dcode_raw_h1_json("common/2DCode/raw/H1.json")]
#[case::common_2dcode_raw_h2_json("common/2DCode/raw/H2.json")]
#[case::common_2dcode_raw_h3_json("common/2DCode/raw/H3.json")]
#[case::common_2dcode_raw_q1_json("common/2DCode/raw/Q1.json")]
#[case::common_2dcode_raw_z1_json("common/2DCode/raw/Z1.json")]
#[case::common_2dcode_raw_z2_json("common/2DCode/raw/Z2.json")]
#[case::ae_2dcode_raw_test_json("AE/2DCode/raw/test.json")]
#[case::ae_2dcode_raw_vaccine_json("AE/2DCode/raw/vaccine.json")]
#[case::at_2dcode_raw_1_json("AT/2DCode/raw/1.json")]
#[case::at_2dcode_raw_2_json("AT/2DCode/raw/2.json")]
#[case::at_2dcode_raw_3_json("AT/2DCode/raw/3.json")]
#[case::at_2dcode_raw_4_json("AT/2DCode/raw/4.json")]
#[case::be_2dcode_raw_1_json("BE/2DCode/raw/1.json")]
#[case::be_2dcode_raw_2_json("BE/2DCode/raw/2.json")]
#[case::be_2dcode_raw_3_json("BE/2DCode/raw/3.json")]
#[case::be_2dcode_raw_4_json("BE/2DCode/raw/4.json")]
#[case::be_2dcode_raw_5_json("BE/2DCode/raw/5.json")]
// #[case::bg_2dcode_raw_1_json("BG/2DCode/raw/1.json")] // SKIPPED. See #1
// #[case::bg_2dcode_raw_2_json("BG/2DCode/raw/2.json")] // SKIPPED. See #1
// #[case::bg_2dcode_raw_3_json("BG/2DCode/raw/3.json")] // SKIPPED. See #1
// #[case::bg_2dcode_raw_4_json("BG/2DCode/raw/4.json")] // SKIPPED. See #1
// #[case::bg_2dcode_raw_5_json("BG/2DCode/raw/5.json")] // SKIPPED. See #1
// #[case::ch_2dcode_raw_1_json("CH/2DCode/raw/1.json")] // SKIPPED. RSA signature. See #2
// #[case::ch_2dcode_raw_2_json("CH/2DCode/raw/2.json")] // SKIPPED. RSA signature. See #2
// #[case::ch_2dcode_raw_3_json("CH/2DCode/raw/3.json")] // SKIPPED. RSA signature. See #2
#[case::cy_2dcode_raw_5_json("CY/2DCode/raw/5.json")]
#[case::cy_2dcode_raw_6_json("CY/2DCode/raw/6.json")]
#[case::cy_2dcode_raw_7_json("CY/2DCode/raw/7.json")]
#[case::cy_2dcode_raw_8_json("CY/2DCode/raw/8.json")]
// #[case::cz_2dcode_raw_1_json("CZ/2DCode/raw/1.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_11_json("CZ/2DCode/raw/11.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_12_json("CZ/2DCode/raw/12.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_13_json("CZ/2DCode/raw/13.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_14_json("CZ/2DCode/raw/14.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_15_json("CZ/2DCode/raw/15.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_2_json("CZ/2DCode/raw/2.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_3_json("CZ/2DCode/raw/3.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_4_json("CZ/2DCode/raw/4.json")] // SKIPPED. See #1
// #[case::cz_2dcode_raw_5_json("CZ/2DCode/raw/5.json")] // SKIPPED. See #1
// #[case::de_2dcode_raw_1_json("DE/2DCode/raw/1.json")] // SKIPPED. See #1
// #[case::de_2dcode_raw_2_json("DE/2DCode/raw/2.json")] // SKIPPED. See #1
// #[case::de_2dcode_raw_3_json("DE/2DCode/raw/3.json")] // SKIPPED. See #1
// #[case::de_2dcode_raw_4_json("DE/2DCode/raw/4.json")] // SKIPPED. See #1
// #[case::dk_2dcode_raw_1_json("DK/2DCode/raw/1.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_10_json("DK/2DCode/raw/10.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_11_json("DK/2DCode/raw/11.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_12_json("DK/2DCode/raw/12.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_2_json("DK/2DCode/raw/2.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_3_json("DK/2DCode/raw/3.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_4_json("DK/2DCode/raw/4.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_5_json("DK/2DCode/raw/5.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_7_json("DK/2DCode/raw/7.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::dk_2dcode_raw_8_json("DK/2DCode/raw/8.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1001_json("ES/2DCode/raw/1001.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1002_json("ES/2DCode/raw/1002.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1003_json("ES/2DCode/raw/1003.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_101_json("ES/2DCode/raw/101.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_102_json("ES/2DCode/raw/102.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_103_json("ES/2DCode/raw/103.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1101_json("ES/2DCode/raw/1101.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1102_json("ES/2DCode/raw/1102.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1103_json("ES/2DCode/raw/1103.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1501_json("ES/2DCode/raw/1501.json")] // SKIPPED. Invalid COSE (missing tag)
// #[case::es_2dcode_raw_1502_json("ES/2DCode/raw/1502.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_1503_json("ES/2DCode/raw/1503.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_201_json("ES/2DCode/raw/201.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_202_json("ES/2DCode/raw/202.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_203_json("ES/2DCode/raw/203.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_2101_json("ES/2DCode/raw/2101.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_2102_json("ES/2DCode/raw/2102.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_2103_json("ES/2DCode/raw/2103.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_401_json("ES/2DCode/raw/401.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_402_json("ES/2DCode/raw/402.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_403_json("ES/2DCode/raw/403.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_501_json("ES/2DCode/raw/501.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_502_json("ES/2DCode/raw/502.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_503_json("ES/2DCode/raw/503.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_701_json("ES/2DCode/raw/701.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_702_json("ES/2DCode/raw/702.json")] // SKIPPED. `issued_at` is a float (see #3)
// #[case::es_2dcode_raw_703_json("ES/2DCode/raw/703.json")] // SKIPPED. `issued_at` is a float (see #3)
#[case::fi_2dcode_raw_1_json("FI/2DCode/raw/1.json")]
#[case::fi_2dcode_raw_10_json("FI/2DCode/raw/10.json")]
#[case::fi_2dcode_raw_2_json("FI/2DCode/raw/2.json")]
#[case::fi_2dcode_raw_3_json("FI/2DCode/raw/3.json")]
#[case::fi_2dcode_raw_4_json("FI/2DCode/raw/4.json")]
#[case::fi_2dcode_raw_5_json("FI/2DCode/raw/5.json")]
#[case::fi_2dcode_raw_6_json("FI/2DCode/raw/6.json")]
#[case::fi_2dcode_raw_7_json("FI/2DCode/raw/7.json")]
#[case::fi_2dcode_raw_8_json("FI/2DCode/raw/8.json")]
#[case::fi_2dcode_raw_9_json("FI/2DCode/raw/9.json")]
#[case::fr_2dcode_raw_dcc_test_0001_json("FR/2DCode/raw/DCC_Test_0001.json")]
#[case::fr_2dcode_raw_dcc_test_0002_json("FR/2DCode/raw/DCC_Test_0002.json")]
#[case::fr_2dcode_raw_dcc_test_0003_json("FR/2DCode/raw/DCC_Test_0003.json")]
#[case::fr_2dcode_raw_dcc_test_0004_json("FR/2DCode/raw/DCC_Test_0004.json")]
#[case::fr_2dcode_raw_dcc_test_0005_json("FR/2DCode/raw/DCC_Test_0005.json")]
#[case::fr_2dcode_raw_dcc_test_0006_json("FR/2DCode/raw/DCC_Test_0006.json")]
#[case::fr_2dcode_raw_dcc_test_0007_json("FR/2DCode/raw/DCC_Test_0007.json")]
#[case::fr_2dcode_raw_dcc_test_0008_json("FR/2DCode/raw/DCC_Test_0008.json")]
#[case::fr_2dcode_raw_dcc_test_0010_json("FR/2DCode/raw/DCC_Test_0010.json")]
#[case::fr_2dcode_raw_dcc_test_0011_json("FR/2DCode/raw/DCC_Test_0011.json")]
#[case::fr_2dcode_raw_dcc_test_0012_json("FR/2DCode/raw/DCC_Test_0012.json")]
#[case::fr_2dcode_raw_dcc_test_0013_json("FR/2DCode/raw/DCC_Test_0013.json")]
#[case::fr_2dcode_raw_dcc_test_0014_json("FR/2DCode/raw/DCC_Test_0014.json")]
#[case::fr_2dcode_raw_dcc_test_0015_json("FR/2DCode/raw/DCC_Test_0015.json")]
#[case::fr_2dcode_raw_dcc_test_0016_json("FR/2DCode/raw/DCC_Test_0016.json")]
#[case::fr_2dcode_raw_dcc_test_0017_json("FR/2DCode/raw/DCC_Test_0017.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00001_raw_json("FR/2DCode/raw/DGC_QrCode_00001_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00002_raw_json("FR/2DCode/raw/DGC_QrCode_00002_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00003_raw_json("FR/2DCode/raw/DGC_QrCode_00003_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00004_raw_json("FR/2DCode/raw/DGC_QrCode_00004_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00005_raw_json("FR/2DCode/raw/DGC_QrCode_00005_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00006_raw_json("FR/2DCode/raw/DGC_QrCode_00006_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00007_raw_json("FR/2DCode/raw/DGC_QrCode_00007_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00008_raw_json("FR/2DCode/raw/DGC_QrCode_00008_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00009_raw_json("FR/2DCode/raw/DGC_QrCode_00009_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00010_raw_json("FR/2DCode/raw/DGC_QrCode_00010_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00011_raw_json("FR/2DCode/raw/DGC_QrCode_00011_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00012_raw_json("FR/2DCode/raw/DGC_QrCode_00012_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00013_raw_json("FR/2DCode/raw/DGC_QrCode_00013_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00014_raw_json("FR/2DCode/raw/DGC_QrCode_00014_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00015_raw_json("FR/2DCode/raw/DGC_QrCode_00015_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00016_raw_json("FR/2DCode/raw/DGC_QrCode_00016_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00017_raw_json("FR/2DCode/raw/DGC_QrCode_00017_Raw.json")]
#[case::fr_2dcode_raw_dgc_qrcode_00018_raw_json("FR/2DCode/raw/DGC_QrCode_00018_Raw.json")]
#[case::fr_2dcode_raw_recovery_ok_json("FR/2DCode/raw/recovery_ok.json")]
// #[case::fr_2dcode_raw_test_pcr_ok_json("FR/2DCode/raw/test_pcr_ok.json")] // SKIPPED some dates in the given JSON seem wrong
#[case::fr_2dcode_raw_vaccin_ok_json("FR/2DCode/raw/vaccin_ok.json")]
// #[case::ge_2dcode_raw_1_json("GE/2DCode/raw/1.json")]
// #[case::ge_2dcode_raw_2_json("GE/2DCode/raw/2.json")]
// #[case::ge_2dcode_raw_3_json("GE/2DCode/raw/3.json")]
// #[case::gr_2dcode_raw_1_json("GR/2DCode/raw/1.json")]
// #[case::gr_2dcode_raw_2_json("GR/2DCode/raw/2.json")]
// #[case::gr_2dcode_raw_3_json("GR/2DCode/raw/3.json")]
// #[case::gr_2dcode_raw_4_json("GR/2DCode/raw/4.json")]
// #[case::hr_2dcode_raw_1_json("HR/2DCode/raw/1.json")]
// #[case::hr_2dcode_raw_2_json("HR/2DCode/raw/2.json")]
// #[case::hr_2dcode_raw_3_json("HR/2DCode/raw/3.json")]
// #[case::hr_2dcode_raw_4_json("HR/2DCode/raw/4.json")]
// #[case::hu_2dcode_raw_1_json("HU/2DCode/raw/1.json")]
// #[case::hu_2dcode_raw_2_json("HU/2DCode/raw/2.json")]
// #[case::hu_2dcode_raw_3_json("HU/2DCode/raw/3.json")]
// #[case::hu_2dcode_raw_4_json("HU/2DCode/raw/4.json")]
// #[case::ie_2dcode_raw_1_json("IE/2DCode/Raw/1.json")]
// #[case::ie_2dcode_raw_2_json("IE/2DCode/Raw/2.json")]
// #[case::ie_2dcode_raw_3_json("IE/2DCode/Raw/3.json")]
// #[case::ie_2dcode_raw_4_json("IE/2DCode/Raw/4.json")]
// #[case::is_2dcode_raw_1_json("IS/2DCode/raw/1.json")]
// #[case::is_2dcode_raw_2_json("IS/2DCode/raw/2.json")]
// #[case::is_2dcode_raw_3_json("IS/2DCode/raw/3.json")]
// #[case::is_2dcode_raw_4_json("IS/2DCode/raw/4.json")]
// #[case::is_2dcode_raw_5_json("IS/2DCode/raw/5.json")]
// #[case::it_2dcode_raw_1_json("IT/2DCode/raw/1.json")]
// #[case::it_2dcode_raw_2_json("IT/2DCode/raw/2.json")]
// #[case::it_2dcode_raw_3_json("IT/2DCode/raw/3.json")]
// #[case::it_2dcode_raw_4_json("IT/2DCode/raw/4.json")]
// #[case::li_2dcode_raw_1_json("LI/2DCode/raw/1.json")]
// #[case::li_2dcode_raw_2_json("LI/2DCode/raw/2.json")]
// #[case::li_2dcode_raw_3_json("LI/2DCode/raw/3.json")]
// #[case::li_2dcode_raw_4_json("LI/2DCode/raw/4.json")]
// #[case::lt_2dcode_raw_1_json("LT/2DCode/raw/1.json")]
// #[case::lt_2dcode_raw_2_json("LT/2DCode/raw/2.json")]
// #[case::lt_2dcode_raw_3_json("LT/2DCode/raw/3.json")]
// #[case::lt_2dcode_raw_4_json("LT/2DCode/raw/4.json")]
// #[case::lu_2dcode_raw_incert_r_dcc_naat_json("LU/2DCode/raw/INCERT_R_DCC_NAAT.json")]
// #[case::lu_2dcode_raw_incert_r_dcc_rat_json("LU/2DCode/raw/INCERT_R_DCC_RAT.json")]
// #[case::lu_2dcode_raw_incert_r_dcc_recovery_json("LU/2DCode/raw/INCERT_R_DCC_Recovery.json")]
// #[case::lu_2dcode_raw_incert_r_dcc_vaccination_json("LU/2DCode/raw/INCERT_R_DCC_Vaccination.json")]
// #[case::lv_2dcode_raw_1_json("LV/2DCode/raw/1.json")]
// #[case::lv_2dcode_raw_2_json("LV/2DCode/raw/2.json")]
// #[case::lv_2dcode_raw_3_json("LV/2DCode/raw/3.json")]
// #[case::nl_2dcode_raw_000_nl_test_json("NL/2DCode/raw/000-NL-test.json")]
// #[case::nl_2dcode_raw_001_nl_test_json("NL/2DCode/raw/001-NL-test.json")]
// #[case::nl_2dcode_raw_002_nl_test_json("NL/2DCode/raw/002-NL-test.json")]
// #[case::nl_2dcode_raw_003_nl_test_json("NL/2DCode/raw/003-NL-test.json")]
// #[case::nl_2dcode_raw_004_nl_test_json("NL/2DCode/raw/004-NL-test.json")]
// #[case::nl_2dcode_raw_005_nl_test_json("NL/2DCode/raw/005-NL-test.json")]
// #[case::nl_2dcode_raw_006_nl_test_json("NL/2DCode/raw/006-NL-test.json")]
// #[case::nl_2dcode_raw_007_nl_test_json("NL/2DCode/raw/007-NL-test.json")]
// #[case::nl_2dcode_raw_008_nl_test_json("NL/2DCode/raw/008-NL-test.json")]
// #[case::nl_2dcode_raw_009_nl_test_json("NL/2DCode/raw/009-NL-test.json")]
// #[case::nl_2dcode_raw_010_nl_test_json("NL/2DCode/raw/010-NL-test.json")]
// #[case::nl_2dcode_raw_011_nl_test_json("NL/2DCode/raw/011-NL-test.json")]
// #[case::nl_2dcode_raw_012_nl_test_json("NL/2DCode/raw/012-NL-test.json")]
// #[case::nl_2dcode_raw_013_nl_test_json("NL/2DCode/raw/013-NL-test.json")]
// #[case::nl_2dcode_raw_014_nl_test_json("NL/2DCode/raw/014-NL-test.json")]
// #[case::nl_2dcode_raw_015_nl_test_json("NL/2DCode/raw/015-NL-test.json")]
// #[case::nl_2dcode_raw_016_nl_test_json("NL/2DCode/raw/016-NL-test.json")]
// #[case::nl_2dcode_raw_017_nl_test_json("NL/2DCode/raw/017-NL-test.json")]
// #[case::nl_2dcode_raw_018_nl_test_json("NL/2DCode/raw/018-NL-test.json")]
// #[case::nl_2dcode_raw_019_nl_test_json("NL/2DCode/raw/019-NL-test.json")]
// #[case::nl_2dcode_raw_020_nl_test_json("NL/2DCode/raw/020-NL-test.json")]
// #[case::nl_2dcode_raw_021_nl_test_json("NL/2DCode/raw/021-NL-test.json")]
// #[case::nl_2dcode_raw_022_nl_test_json("NL/2DCode/raw/022-NL-test.json")]
// #[case::nl_2dcode_raw_023_nl_test_json("NL/2DCode/raw/023-NL-test.json")]
// #[case::nl_2dcode_raw_024_nl_test_json("NL/2DCode/raw/024-NL-test.json")]
// #[case::nl_2dcode_raw_025_nl_test_json("NL/2DCode/raw/025-NL-test.json")]
// #[case::nl_2dcode_raw_026_nl_test_json("NL/2DCode/raw/026-NL-test.json")]
// #[case::nl_2dcode_raw_027_nl_test_json("NL/2DCode/raw/027-NL-test.json")]
// #[case::nl_2dcode_raw_028_nl_test_json("NL/2DCode/raw/028-NL-test.json")]
// #[case::nl_2dcode_raw_029_nl_test_json("NL/2DCode/raw/029-NL-test.json")]
// #[case::nl_2dcode_raw_030_nl_test_json("NL/2DCode/raw/030-NL-test.json")]
// #[case::nl_2dcode_raw_031_nl_test_json("NL/2DCode/raw/031-NL-test.json")]
// #[case::nl_2dcode_raw_032_nl_test_json("NL/2DCode/raw/032-NL-test.json")]
// #[case::nl_2dcode_raw_033_nl_test_json("NL/2DCode/raw/033-NL-test.json")]
// #[case::nl_2dcode_raw_034_nl_test_json("NL/2DCode/raw/034-NL-test.json")]
// #[case::nl_2dcode_raw_035_nl_test_json("NL/2DCode/raw/035-NL-test.json")]
// #[case::nl_2dcode_raw_036_nl_test_json("NL/2DCode/raw/036-NL-test.json")]
// #[case::nl_2dcode_raw_037_nl_test_json("NL/2DCode/raw/037-NL-test.json")]
// #[case::nl_2dcode_raw_038_nl_test_json("NL/2DCode/raw/038-NL-test.json")]
// #[case::nl_2dcode_raw_039_nl_test_json("NL/2DCode/raw/039-NL-test.json")]
// #[case::nl_2dcode_raw_040_nl_test_json("NL/2DCode/raw/040-NL-test.json")]
// #[case::nl_2dcode_raw_041_nl_test_json("NL/2DCode/raw/041-NL-test.json")]
// #[case::nl_2dcode_raw_042_nl_test_json("NL/2DCode/raw/042-NL-test.json")]
// #[case::nl_2dcode_raw_043_nl_test_json("NL/2DCode/raw/043-NL-test.json")]
// #[case::nl_2dcode_raw_044_nl_test_json("NL/2DCode/raw/044-NL-test.json")]
// #[case::nl_2dcode_raw_045_nl_test_json("NL/2DCode/raw/045-NL-test.json")]
// #[case::nl_2dcode_raw_046_nl_test_json("NL/2DCode/raw/046-NL-test.json")]
// #[case::nl_2dcode_raw_047_nl_test_json("NL/2DCode/raw/047-NL-test.json")]
// #[case::nl_2dcode_raw_048_nl_test_json("NL/2DCode/raw/048-NL-test.json")]
// #[case::nl_2dcode_raw_049_nl_test_json("NL/2DCode/raw/049-NL-test.json")]
// #[case::nl_2dcode_raw_050_nl_test_json("NL/2DCode/raw/050-NL-test.json")]
// #[case::nl_2dcode_raw_051_nl_test_json("NL/2DCode/raw/051-NL-test.json")]
// #[case::nl_2dcode_raw_052_nl_test_json("NL/2DCode/raw/052-NL-test.json")]
// #[case::nl_2dcode_raw_053_nl_test_json("NL/2DCode/raw/053-NL-test.json")]
// #[case::nl_2dcode_raw_054_nl_test_json("NL/2DCode/raw/054-NL-test.json")]
// #[case::nl_2dcode_raw_055_nl_test_json("NL/2DCode/raw/055-NL-test.json")]
// #[case::nl_2dcode_raw_056_nl_test_json("NL/2DCode/raw/056-NL-test.json")]
// #[case::nl_2dcode_raw_057_nl_test_json("NL/2DCode/raw/057-NL-test.json")]
// #[case::nl_2dcode_raw_058_nl_test_json("NL/2DCode/raw/058-NL-test.json")]
// #[case::nl_2dcode_raw_059_nl_test_json("NL/2DCode/raw/059-NL-test.json")]
// #[case::nl_2dcode_raw_060_nl_test_json("NL/2DCode/raw/060-NL-test.json")]
// #[case::nl_2dcode_raw_061_nl_test_json("NL/2DCode/raw/061-NL-test.json")]
// #[case::nl_2dcode_raw_062_nl_test_json("NL/2DCode/raw/062-NL-test.json")]
// #[case::nl_2dcode_raw_063_nl_test_json("NL/2DCode/raw/063-NL-test.json")]
// #[case::nl_2dcode_raw_064_nl_test_json("NL/2DCode/raw/064-NL-test.json")]
// #[case::nl_2dcode_raw_065_nl_test_json("NL/2DCode/raw/065-NL-test.json")]
// #[case::nl_2dcode_raw_066_nl_test_json("NL/2DCode/raw/066-NL-test.json")]
// #[case::nl_2dcode_raw_067_nl_test_json("NL/2DCode/raw/067-NL-test.json")]
// #[case::nl_2dcode_raw_068_nl_test_json("NL/2DCode/raw/068-NL-test.json")]
// #[case::nl_2dcode_raw_069_nl_test_json("NL/2DCode/raw/069-NL-test.json")]
// #[case::nl_2dcode_raw_070_nl_test_json("NL/2DCode/raw/070-NL-test.json")]
// #[case::nl_2dcode_raw_071_nl_test_json("NL/2DCode/raw/071-NL-test.json")]
// #[case::nl_2dcode_raw_072_nl_vaccination_json("NL/2DCode/raw/072-NL-vaccination.json")]
// #[case::nl_2dcode_raw_073_nl_vaccination_json("NL/2DCode/raw/073-NL-vaccination.json")]
// #[case::nl_2dcode_raw_074_nl_vaccination_json("NL/2DCode/raw/074-NL-vaccination.json")]
// #[case::nl_2dcode_raw_075_nl_vaccination_json("NL/2DCode/raw/075-NL-vaccination.json")]
// #[case::nl_2dcode_raw_076_nl_vaccination_json("NL/2DCode/raw/076-NL-vaccination.json")]
// #[case::nl_2dcode_raw_077_nl_vaccination_json("NL/2DCode/raw/077-NL-vaccination.json")]
// #[case::nl_2dcode_raw_078_nl_vaccination_json("NL/2DCode/raw/078-NL-vaccination.json")]
// #[case::nl_2dcode_raw_079_nl_vaccination_json("NL/2DCode/raw/079-NL-vaccination.json")]
// #[case::nl_2dcode_raw_080_nl_vaccination_json("NL/2DCode/raw/080-NL-vaccination.json")]
// #[case::nl_2dcode_raw_081_nl_vaccination_json("NL/2DCode/raw/081-NL-vaccination.json")]
// #[case::nl_2dcode_raw_082_nl_vaccination_json("NL/2DCode/raw/082-NL-vaccination.json")]
// #[case::nl_2dcode_raw_083_nl_vaccination_json("NL/2DCode/raw/083-NL-vaccination.json")]
// #[case::nl_2dcode_raw_084_nl_vaccination_json("NL/2DCode/raw/084-NL-vaccination.json")]
// #[case::nl_2dcode_raw_085_nl_vaccination_json("NL/2DCode/raw/085-NL-vaccination.json")]
// #[case::nl_2dcode_raw_086_nl_vaccination_json("NL/2DCode/raw/086-NL-vaccination.json")]
// #[case::nl_2dcode_raw_087_nl_vaccination_json("NL/2DCode/raw/087-NL-vaccination.json")]
// #[case::nl_2dcode_raw_088_nl_vaccination_json("NL/2DCode/raw/088-NL-vaccination.json")]
// #[case::nl_2dcode_raw_089_nl_vaccination_json("NL/2DCode/raw/089-NL-vaccination.json")]
// #[case::nl_2dcode_raw_090_nl_vaccination_json("NL/2DCode/raw/090-NL-vaccination.json")]
// #[case::nl_2dcode_raw_091_nl_vaccination_json("NL/2DCode/raw/091-NL-vaccination.json")]
// #[case::nl_2dcode_raw_092_nl_vaccination_json("NL/2DCode/raw/092-NL-vaccination.json")]
// #[case::nl_2dcode_raw_093_nl_vaccination_json("NL/2DCode/raw/093-NL-vaccination.json")]
// #[case::nl_2dcode_raw_094_nl_vaccination_json("NL/2DCode/raw/094-NL-vaccination.json")]
// #[case::nl_2dcode_raw_095_nl_vaccination_json("NL/2DCode/raw/095-NL-vaccination.json")]
// #[case::nl_2dcode_raw_096_nl_vaccination_json("NL/2DCode/raw/096-NL-vaccination.json")]
// #[case::nl_2dcode_raw_097_nl_vaccination_json("NL/2DCode/raw/097-NL-vaccination.json")]
// #[case::nl_2dcode_raw_098_nl_vaccination_json("NL/2DCode/raw/098-NL-vaccination.json")]
// #[case::nl_2dcode_raw_099_nl_vaccination_json("NL/2DCode/raw/099-NL-vaccination.json")]
// #[case::nl_2dcode_raw_100_nl_vaccination_json("NL/2DCode/raw/100-NL-vaccination.json")]
// #[case::nl_2dcode_raw_101_nl_vaccination_json("NL/2DCode/raw/101-NL-vaccination.json")]
// #[case::nl_2dcode_raw_102_nl_vaccination_json("NL/2DCode/raw/102-NL-vaccination.json")]
// #[case::nl_2dcode_raw_103_nl_vaccination_json("NL/2DCode/raw/103-NL-vaccination.json")]
// #[case::nl_2dcode_raw_104_nl_vaccination_json("NL/2DCode/raw/104-NL-vaccination.json")]
// #[case::nl_2dcode_raw_105_nl_vaccination_json("NL/2DCode/raw/105-NL-vaccination.json")]
// #[case::nl_2dcode_raw_106_nl_vaccination_json("NL/2DCode/raw/106-NL-vaccination.json")]
// #[case::nl_2dcode_raw_107_nl_vaccination_json("NL/2DCode/raw/107-NL-vaccination.json")]
// #[case::nl_2dcode_raw_108_nl_vaccination_json("NL/2DCode/raw/108-NL-vaccination.json")]
// #[case::nl_2dcode_raw_109_nl_vaccination_json("NL/2DCode/raw/109-NL-vaccination.json")]
// #[case::nl_2dcode_raw_110_nl_vaccination_json("NL/2DCode/raw/110-NL-vaccination.json")]
// #[case::nl_2dcode_raw_111_nl_vaccination_json("NL/2DCode/raw/111-NL-vaccination.json")]
// #[case::nl_2dcode_raw_112_nl_vaccination_json("NL/2DCode/raw/112-NL-vaccination.json")]
// #[case::nl_2dcode_raw_113_nl_vaccination_json("NL/2DCode/raw/113-NL-vaccination.json")]
// #[case::nl_2dcode_raw_114_nl_vaccination_json("NL/2DCode/raw/114-NL-vaccination.json")]
// #[case::nl_2dcode_raw_115_nl_vaccination_json("NL/2DCode/raw/115-NL-vaccination.json")]
// #[case::nl_2dcode_raw_116_nl_vaccination_json("NL/2DCode/raw/116-NL-vaccination.json")]
// #[case::nl_2dcode_raw_117_nl_vaccination_json("NL/2DCode/raw/117-NL-vaccination.json")]
// #[case::nl_2dcode_raw_118_nl_vaccination_json("NL/2DCode/raw/118-NL-vaccination.json")]
// #[case::nl_2dcode_raw_119_nl_vaccination_json("NL/2DCode/raw/119-NL-vaccination.json")]
// #[case::nl_2dcode_raw_120_nl_vaccination_json("NL/2DCode/raw/120-NL-vaccination.json")]
// #[case::nl_2dcode_raw_121_nl_vaccination_json("NL/2DCode/raw/121-NL-vaccination.json")]
// #[case::nl_2dcode_raw_122_nl_vaccination_json("NL/2DCode/raw/122-NL-vaccination.json")]
// #[case::nl_2dcode_raw_123_nl_vaccination_json("NL/2DCode/raw/123-NL-vaccination.json")]
// #[case::nl_2dcode_raw_124_nl_vaccination_json("NL/2DCode/raw/124-NL-vaccination.json")]
// #[case::nl_2dcode_raw_125_nl_vaccination_json("NL/2DCode/raw/125-NL-vaccination.json")]
// #[case::nl_2dcode_raw_126_nl_vaccination_json("NL/2DCode/raw/126-NL-vaccination.json")]
// #[case::nl_2dcode_raw_127_nl_vaccination_json("NL/2DCode/raw/127-NL-vaccination.json")]
// #[case::nl_2dcode_raw_128_nl_vaccination_json("NL/2DCode/raw/128-NL-vaccination.json")]
// #[case::nl_2dcode_raw_129_nl_vaccination_json("NL/2DCode/raw/129-NL-vaccination.json")]
// #[case::nl_2dcode_raw_130_nl_vaccination_json("NL/2DCode/raw/130-NL-vaccination.json")]
// #[case::nl_2dcode_raw_131_nl_vaccination_json("NL/2DCode/raw/131-NL-vaccination.json")]
// #[case::nl_2dcode_raw_132_nl_vaccination_json("NL/2DCode/raw/132-NL-vaccination.json")]
// #[case::nl_2dcode_raw_133_nl_vaccination_json("NL/2DCode/raw/133-NL-vaccination.json")]
// #[case::nl_2dcode_raw_134_nl_vaccination_json("NL/2DCode/raw/134-NL-vaccination.json")]
// #[case::nl_2dcode_raw_135_nl_vaccination_json("NL/2DCode/raw/135-NL-vaccination.json")]
// #[case::nl_2dcode_raw_136_nl_vaccination_json("NL/2DCode/raw/136-NL-vaccination.json")]
// #[case::nl_2dcode_raw_137_nl_vaccination_json("NL/2DCode/raw/137-NL-vaccination.json")]
// #[case::nl_2dcode_raw_138_nl_vaccination_json("NL/2DCode/raw/138-NL-vaccination.json")]
// #[case::nl_2dcode_raw_139_nl_vaccination_json("NL/2DCode/raw/139-NL-vaccination.json")]
// #[case::nl_2dcode_raw_140_nl_vaccination_json("NL/2DCode/raw/140-NL-vaccination.json")]
// #[case::nl_2dcode_raw_141_nl_vaccination_json("NL/2DCode/raw/141-NL-vaccination.json")]
// #[case::nl_2dcode_raw_142_nl_vaccination_json("NL/2DCode/raw/142-NL-vaccination.json")]
// #[case::nl_2dcode_raw_143_nl_vaccination_json("NL/2DCode/raw/143-NL-vaccination.json")]
// #[case::nl_2dcode_raw_144_nl_recovery_json("NL/2DCode/raw/144-NL-recovery.json")]
// #[case::nl_2dcode_raw_145_nl_recovery_json("NL/2DCode/raw/145-NL-recovery.json")]
// #[case::nl_2dcode_raw_146_nl_recovery_json("NL/2DCode/raw/146-NL-recovery.json")]
// #[case::nl_2dcode_raw_147_nl_recovery_json("NL/2DCode/raw/147-NL-recovery.json")]
// #[case::nl_2dcode_raw_148_nl_recovery_json("NL/2DCode/raw/148-NL-recovery.json")]
// #[case::nl_2dcode_raw_149_nl_recovery_json("NL/2DCode/raw/149-NL-recovery.json")]
// #[case::nl_2dcode_raw_150_nl_recovery_json("NL/2DCode/raw/150-NL-recovery.json")]
// #[case::nl_2dcode_raw_151_nl_recovery_json("NL/2DCode/raw/151-NL-recovery.json")]
// #[case::nl_2dcode_raw_152_nl_recovery_json("NL/2DCode/raw/152-NL-recovery.json")]
// #[case::nl_2dcode_raw_153_nl_recovery_json("NL/2DCode/raw/153-NL-recovery.json")]
// #[case::nl_2dcode_raw_154_nl_recovery_json("NL/2DCode/raw/154-NL-recovery.json")]
// #[case::nl_2dcode_raw_155_nl_recovery_json("NL/2DCode/raw/155-NL-recovery.json")]
// #[case::nl_2dcode_raw_156_nl_recovery_json("NL/2DCode/raw/156-NL-recovery.json")]
// #[case::nl_2dcode_raw_157_nl_recovery_json("NL/2DCode/raw/157-NL-recovery.json")]
// #[case::nl_2dcode_raw_158_nl_recovery_json("NL/2DCode/raw/158-NL-recovery.json")]
// #[case::nl_2dcode_raw_159_nl_recovery_json("NL/2DCode/raw/159-NL-recovery.json")]
// #[case::nl_2dcode_raw_160_nl_recovery_json("NL/2DCode/raw/160-NL-recovery.json")]
// #[case::nl_2dcode_raw_161_nl_recovery_json("NL/2DCode/raw/161-NL-recovery.json")]
// #[case::nl_2dcode_raw_162_nl_recovery_json("NL/2DCode/raw/162-NL-recovery.json")]
// #[case::nl_2dcode_raw_163_nl_recovery_json("NL/2DCode/raw/163-NL-recovery.json")]
// #[case::nl_2dcode_raw_164_nl_recovery_json("NL/2DCode/raw/164-NL-recovery.json")]
// #[case::nl_2dcode_raw_165_nl_recovery_json("NL/2DCode/raw/165-NL-recovery.json")]
// #[case::nl_2dcode_raw_166_nl_recovery_json("NL/2DCode/raw/166-NL-recovery.json")]
// #[case::nl_2dcode_raw_167_nl_recovery_json("NL/2DCode/raw/167-NL-recovery.json")]
// #[case::nl_2dcode_raw_168_nl_recovery_json("NL/2DCode/raw/168-NL-recovery.json")]
// #[case::nl_2dcode_raw_169_nl_recovery_json("NL/2DCode/raw/169-NL-recovery.json")]
// #[case::nl_2dcode_raw_170_nl_recovery_json("NL/2DCode/raw/170-NL-recovery.json")]
// #[case::nl_2dcode_raw_171_nl_recovery_json("NL/2DCode/raw/171-NL-recovery.json")]
// #[case::nl_2dcode_raw_172_nl_recovery_json("NL/2DCode/raw/172-NL-recovery.json")]
// #[case::nl_2dcode_raw_173_nl_recovery_json("NL/2DCode/raw/173-NL-recovery.json")]
// #[case::nl_2dcode_raw_174_nl_recovery_json("NL/2DCode/raw/174-NL-recovery.json")]
// #[case::nl_2dcode_raw_175_nl_recovery_json("NL/2DCode/raw/175-NL-recovery.json")]
// #[case::nl_2dcode_raw_176_nl_recovery_json("NL/2DCode/raw/176-NL-recovery.json")]
// #[case::nl_2dcode_raw_177_nl_recovery_json("NL/2DCode/raw/177-NL-recovery.json")]
// #[case::nl_2dcode_raw_178_nl_recovery_json("NL/2DCode/raw/178-NL-recovery.json")]
// #[case::nl_2dcode_raw_179_nl_recovery_json("NL/2DCode/raw/179-NL-recovery.json")]
// #[case::nl_2dcode_raw_180_nl_recovery_json("NL/2DCode/raw/180-NL-recovery.json")]
// #[case::nl_2dcode_raw_181_nl_recovery_json("NL/2DCode/raw/181-NL-recovery.json")]
// #[case::nl_2dcode_raw_182_nl_recovery_json("NL/2DCode/raw/182-NL-recovery.json")]
// #[case::nl_2dcode_raw_183_nl_recovery_json("NL/2DCode/raw/183-NL-recovery.json")]
// #[case::nl_2dcode_raw_184_nl_recovery_json("NL/2DCode/raw/184-NL-recovery.json")]
// #[case::nl_2dcode_raw_185_nl_recovery_json("NL/2DCode/raw/185-NL-recovery.json")]
// #[case::nl_2dcode_raw_186_nl_recovery_json("NL/2DCode/raw/186-NL-recovery.json")]
// #[case::nl_2dcode_raw_187_nl_recovery_json("NL/2DCode/raw/187-NL-recovery.json")]
// #[case::nl_2dcode_raw_188_nl_recovery_json("NL/2DCode/raw/188-NL-recovery.json")]
// #[case::nl_2dcode_raw_189_nl_recovery_json("NL/2DCode/raw/189-NL-recovery.json")]
// #[case::nl_2dcode_raw_190_nl_recovery_json("NL/2DCode/raw/190-NL-recovery.json")]
// #[case::nl_2dcode_raw_191_nl_recovery_json("NL/2DCode/raw/191-NL-recovery.json")]
// #[case::nl_2dcode_raw_192_nl_recovery_json("NL/2DCode/raw/192-NL-recovery.json")]
// #[case::nl_2dcode_raw_193_nl_recovery_json("NL/2DCode/raw/193-NL-recovery.json")]
// #[case::nl_2dcode_raw_194_nl_recovery_json("NL/2DCode/raw/194-NL-recovery.json")]
// #[case::nl_2dcode_raw_195_nl_recovery_json("NL/2DCode/raw/195-NL-recovery.json")]
// #[case::nl_2dcode_raw_196_nl_recovery_json("NL/2DCode/raw/196-NL-recovery.json")]
// #[case::nl_2dcode_raw_197_nl_recovery_json("NL/2DCode/raw/197-NL-recovery.json")]
// #[case::nl_2dcode_raw_198_nl_recovery_json("NL/2DCode/raw/198-NL-recovery.json")]
// #[case::nl_2dcode_raw_199_nl_recovery_json("NL/2DCode/raw/199-NL-recovery.json")]
// #[case::nl_2dcode_raw_200_nl_recovery_json("NL/2DCode/raw/200-NL-recovery.json")]
// #[case::nl_2dcode_raw_201_nl_recovery_json("NL/2DCode/raw/201-NL-recovery.json")]
// #[case::nl_2dcode_raw_202_nl_recovery_json("NL/2DCode/raw/202-NL-recovery.json")]
// #[case::nl_2dcode_raw_203_nl_recovery_json("NL/2DCode/raw/203-NL-recovery.json")]
// #[case::nl_2dcode_raw_204_nl_recovery_json("NL/2DCode/raw/204-NL-recovery.json")]
// #[case::nl_2dcode_raw_205_nl_recovery_json("NL/2DCode/raw/205-NL-recovery.json")]
// #[case::nl_2dcode_raw_206_nl_recovery_json("NL/2DCode/raw/206-NL-recovery.json")]
// #[case::nl_2dcode_raw_207_nl_recovery_json("NL/2DCode/raw/207-NL-recovery.json")]
// #[case::nl_2dcode_raw_208_nl_recovery_json("NL/2DCode/raw/208-NL-recovery.json")]
// #[case::nl_2dcode_raw_209_nl_recovery_json("NL/2DCode/raw/209-NL-recovery.json")]
// #[case::nl_2dcode_raw_210_nl_recovery_json("NL/2DCode/raw/210-NL-recovery.json")]
// #[case::nl_2dcode_raw_211_nl_recovery_json("NL/2DCode/raw/211-NL-recovery.json")]
// #[case::nl_2dcode_raw_212_nl_recovery_json("NL/2DCode/raw/212-NL-recovery.json")]
// #[case::nl_2dcode_raw_213_nl_recovery_json("NL/2DCode/raw/213-NL-recovery.json")]
// #[case::nl_2dcode_raw_214_nl_recovery_json("NL/2DCode/raw/214-NL-recovery.json")]
// #[case::nl_2dcode_raw_215_nl_recovery_json("NL/2DCode/raw/215-NL-recovery.json")]
// #[case::nl_2dcode_raw_216_nl_test_wrong_key_json("NL/2DCode/raw/216-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_217_nl_test_wrong_key_json("NL/2DCode/raw/217-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_218_nl_test_wrong_key_json("NL/2DCode/raw/218-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_219_nl_test_wrong_key_json("NL/2DCode/raw/219-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_220_nl_test_wrong_key_json("NL/2DCode/raw/220-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_221_nl_test_wrong_key_json("NL/2DCode/raw/221-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_222_nl_test_wrong_key_json("NL/2DCode/raw/222-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_223_nl_test_wrong_key_json("NL/2DCode/raw/223-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_224_nl_test_wrong_key_json("NL/2DCode/raw/224-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_225_nl_test_wrong_key_json("NL/2DCode/raw/225-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_226_nl_test_wrong_key_json("NL/2DCode/raw/226-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_227_nl_test_wrong_key_json("NL/2DCode/raw/227-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_228_nl_test_wrong_key_json("NL/2DCode/raw/228-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_229_nl_test_wrong_key_json("NL/2DCode/raw/229-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_230_nl_test_wrong_key_json("NL/2DCode/raw/230-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_231_nl_test_wrong_key_json("NL/2DCode/raw/231-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_232_nl_test_wrong_key_json("NL/2DCode/raw/232-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_233_nl_test_wrong_key_json("NL/2DCode/raw/233-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_234_nl_test_wrong_key_json("NL/2DCode/raw/234-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_235_nl_test_wrong_key_json("NL/2DCode/raw/235-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_236_nl_test_wrong_key_json("NL/2DCode/raw/236-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_237_nl_test_wrong_key_json("NL/2DCode/raw/237-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_238_nl_test_wrong_key_json("NL/2DCode/raw/238-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_239_nl_test_wrong_key_json("NL/2DCode/raw/239-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_240_nl_test_wrong_key_json("NL/2DCode/raw/240-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_241_nl_test_wrong_key_json("NL/2DCode/raw/241-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_242_nl_test_wrong_key_json("NL/2DCode/raw/242-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_243_nl_test_wrong_key_json("NL/2DCode/raw/243-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_244_nl_test_wrong_key_json("NL/2DCode/raw/244-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_245_nl_test_wrong_key_json("NL/2DCode/raw/245-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_246_nl_test_wrong_key_json("NL/2DCode/raw/246-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_247_nl_test_wrong_key_json("NL/2DCode/raw/247-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_248_nl_test_wrong_key_json("NL/2DCode/raw/248-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_249_nl_test_wrong_key_json("NL/2DCode/raw/249-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_250_nl_test_wrong_key_json("NL/2DCode/raw/250-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_251_nl_test_wrong_key_json("NL/2DCode/raw/251-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_252_nl_test_wrong_key_json("NL/2DCode/raw/252-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_253_nl_test_wrong_key_json("NL/2DCode/raw/253-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_254_nl_test_wrong_key_json("NL/2DCode/raw/254-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_255_nl_test_wrong_key_json("NL/2DCode/raw/255-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_256_nl_test_wrong_key_json("NL/2DCode/raw/256-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_257_nl_test_wrong_key_json("NL/2DCode/raw/257-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_258_nl_test_wrong_key_json("NL/2DCode/raw/258-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_259_nl_test_wrong_key_json("NL/2DCode/raw/259-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_260_nl_test_wrong_key_json("NL/2DCode/raw/260-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_261_nl_test_wrong_key_json("NL/2DCode/raw/261-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_262_nl_test_wrong_key_json("NL/2DCode/raw/262-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_263_nl_test_wrong_key_json("NL/2DCode/raw/263-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_264_nl_test_wrong_key_json("NL/2DCode/raw/264-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_265_nl_test_wrong_key_json("NL/2DCode/raw/265-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_266_nl_test_wrong_key_json("NL/2DCode/raw/266-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_267_nl_test_wrong_key_json("NL/2DCode/raw/267-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_268_nl_test_wrong_key_json("NL/2DCode/raw/268-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_269_nl_test_wrong_key_json("NL/2DCode/raw/269-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_270_nl_test_wrong_key_json("NL/2DCode/raw/270-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_271_nl_test_wrong_key_json("NL/2DCode/raw/271-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_272_nl_test_wrong_key_json("NL/2DCode/raw/272-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_273_nl_test_wrong_key_json("NL/2DCode/raw/273-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_274_nl_test_wrong_key_json("NL/2DCode/raw/274-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_275_nl_test_wrong_key_json("NL/2DCode/raw/275-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_276_nl_test_wrong_key_json("NL/2DCode/raw/276-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_277_nl_test_wrong_key_json("NL/2DCode/raw/277-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_278_nl_test_wrong_key_json("NL/2DCode/raw/278-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_279_nl_test_wrong_key_json("NL/2DCode/raw/279-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_280_nl_test_wrong_key_json("NL/2DCode/raw/280-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_281_nl_test_wrong_key_json("NL/2DCode/raw/281-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_282_nl_test_wrong_key_json("NL/2DCode/raw/282-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_283_nl_test_wrong_key_json("NL/2DCode/raw/283-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_284_nl_test_wrong_key_json("NL/2DCode/raw/284-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_285_nl_test_wrong_key_json("NL/2DCode/raw/285-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_286_nl_test_wrong_key_json("NL/2DCode/raw/286-NL-test+wrong_key.json")]
// #[case::nl_2dcode_raw_287_nl_test_wrong_key_json("NL/2DCode/raw/287-NL-test+wrong_key.json")]
// #[case::pl_1_0_0_2dcode_raw_1_json("PL/1.0.0/2DCode/raw/1.json")]
// #[case::pl_1_0_0_2dcode_raw_10_json("PL/1.0.0/2DCode/raw/10.json")]
// #[case::pl_1_0_0_2dcode_raw_11_json("PL/1.0.0/2DCode/raw/11.json")]
// #[case::pl_1_0_0_2dcode_raw_12_json("PL/1.0.0/2DCode/raw/12.json")]
// #[case::pl_1_0_0_2dcode_raw_13_json("PL/1.0.0/2DCode/raw/13.json")]
// #[case::pl_1_0_0_2dcode_raw_14_json("PL/1.0.0/2DCode/raw/14.json")]
// #[case::pl_1_0_0_2dcode_raw_2_json("PL/1.0.0/2DCode/raw/2.json")]
// #[case::pl_1_0_0_2dcode_raw_3_json("PL/1.0.0/2DCode/raw/3.json")]
// #[case::pl_1_0_0_2dcode_raw_4_json("PL/1.0.0/2DCode/raw/4.json")]
// #[case::pl_1_0_0_2dcode_raw_5_json("PL/1.0.0/2DCode/raw/5.json")]
// #[case::pl_1_0_0_2dcode_raw_6_json("PL/1.0.0/2DCode/raw/6.json")]
// #[case::pl_1_0_0_2dcode_raw_7_json("PL/1.0.0/2DCode/raw/7.json")]
// #[case::pl_1_0_0_2dcode_raw_8_json("PL/1.0.0/2DCode/raw/8.json")]
// #[case::pl_1_0_0_2dcode_raw_9_json("PL/1.0.0/2DCode/raw/9.json")]
// #[case::pl_1_2_1_2dcode_raw_1_json("PL/1.2.1/2DCode/raw/1.json")]
// #[case::pl_1_2_1_2dcode_raw_10_json("PL/1.2.1/2DCode/raw/10.json")]
// #[case::pl_1_2_1_2dcode_raw_11_json("PL/1.2.1/2DCode/raw/11.json")]
// #[case::pl_1_2_1_2dcode_raw_12_json("PL/1.2.1/2DCode/raw/12.json")]
// #[case::pl_1_2_1_2dcode_raw_13_json("PL/1.2.1/2DCode/raw/13.json")]
// #[case::pl_1_2_1_2dcode_raw_14_json("PL/1.2.1/2DCode/raw/14.json")]
// #[case::pl_1_2_1_2dcode_raw_2_json("PL/1.2.1/2DCode/raw/2.json")]
// #[case::pl_1_2_1_2dcode_raw_3_json("PL/1.2.1/2DCode/raw/3.json")]
// #[case::pl_1_2_1_2dcode_raw_4_json("PL/1.2.1/2DCode/raw/4.json")]
// #[case::pl_1_2_1_2dcode_raw_5_json("PL/1.2.1/2DCode/raw/5.json")]
// #[case::pl_1_2_1_2dcode_raw_6_json("PL/1.2.1/2DCode/raw/6.json")]
// #[case::pl_1_2_1_2dcode_raw_7_json("PL/1.2.1/2DCode/raw/7.json")]
// #[case::pl_1_2_1_2dcode_raw_8_json("PL/1.2.1/2DCode/raw/8.json")]
// #[case::pl_1_2_1_2dcode_raw_9_json("PL/1.2.1/2DCode/raw/9.json")]
// #[case::pl_1_3_0_2dcode_raw_1_json("PL/1.3.0/2DCode/raw/1.json")]
// #[case::pl_1_3_0_2dcode_raw_10_json("PL/1.3.0/2DCode/raw/10.json")]
// #[case::pl_1_3_0_2dcode_raw_11_json("PL/1.3.0/2DCode/raw/11.json")]
// #[case::pl_1_3_0_2dcode_raw_12_json("PL/1.3.0/2DCode/raw/12.json")]
// #[case::pl_1_3_0_2dcode_raw_13_json("PL/1.3.0/2DCode/raw/13.json")]
// #[case::pl_1_3_0_2dcode_raw_14_json("PL/1.3.0/2DCode/raw/14.json")]
// #[case::pl_1_3_0_2dcode_raw_2_json("PL/1.3.0/2DCode/raw/2.json")]
// #[case::pl_1_3_0_2dcode_raw_3_json("PL/1.3.0/2DCode/raw/3.json")]
// #[case::pl_1_3_0_2dcode_raw_4_json("PL/1.3.0/2DCode/raw/4.json")]
// #[case::pl_1_3_0_2dcode_raw_5_json("PL/1.3.0/2DCode/raw/5.json")]
// #[case::pl_1_3_0_2dcode_raw_6_json("PL/1.3.0/2DCode/raw/6.json")]
// #[case::pl_1_3_0_2dcode_raw_7_json("PL/1.3.0/2DCode/raw/7.json")]
// #[case::pl_1_3_0_2dcode_raw_8_json("PL/1.3.0/2DCode/raw/8.json")]
// #[case::pl_1_3_0_2dcode_raw_9_json("PL/1.3.0/2DCode/raw/9.json")]
// #[case::pt_1_0_0_2dcode_raw_1_json("PT/1.0.0/2DCode/raw/1.json")]
// #[case::pt_1_0_0_2dcode_raw_2_json("PT/1.0.0/2DCode/raw/2.json")]
// #[case::pt_1_0_0_2dcode_raw_3_json("PT/1.0.0/2DCode/raw/3.json")]
// #[case::pt_1_0_0_2dcode_raw_4_json("PT/1.0.0/2DCode/raw/4.json")]
// #[case::pt_1_0_0_2dcode_raw_5_json("PT/1.0.0/2DCode/raw/5.json")]
// #[case::pt_1_3_0_2dcode_raw_1_json("PT/1.3.0/2DCode/raw/1.json")]
// #[case::pt_1_3_0_2dcode_raw_2_json("PT/1.3.0/2DCode/raw/2.json")]
// #[case::pt_1_3_0_2dcode_raw_3_json("PT/1.3.0/2DCode/raw/3.json")]
// #[case::pt_1_3_0_2dcode_raw_4_json("PT/1.3.0/2DCode/raw/4.json")]
// #[case::pt_1_3_0_2dcode_raw_5_json("PT/1.3.0/2DCode/raw/5.json")]
// #[case::ro_2dcode_raw_1_json("RO/2DCode/raw/1.json")]
// #[case::ro_2dcode_raw_2_json("RO/2DCode/raw/2.json")]
// #[case::ro_2dcode_raw_3_json("RO/2DCode/raw/3.json")]
// #[case::ro_2dcode_raw_4_json("RO/2DCode/raw/4.json")]
// #[case::se_2dcode_raw_1_json("SE/2DCode/raw/1.json")]
// #[case::se_2dcode_raw_2_json("SE/2DCode/raw/2.json")]
// #[case::se_2dcode_raw_3_json("SE/2DCode/raw/3.json")]
// #[case::se_2dcode_raw_4_json("SE/2DCode/raw/4.json")]
// #[case::se_2dcode_raw_5_json("SE/2DCode/raw/5.json")]
// #[case::sg_2dcode_raw_1_json("SG/2DCode/raw/1.json")]
// #[case::sg_2dcode_raw_2_json("SG/2DCode/raw/2.json")]
// #[case::sg_2dcode_raw_3_json("SG/2DCode/raw/3.json")]
// #[case::si_2dcode_raw_rec_json("SI/2DCode/raw/REC.json")]
// #[case::si_2dcode_raw_test_ag_json("SI/2DCode/raw/test-AG.json")]
// #[case::si_2dcode_raw_test_pcr_json("SI/2DCode/raw/test-PCR.json")]
// #[case::si_2dcode_raw_vac_json("SI/2DCode/raw/VAC.json")]
// #[case::sk_2dcode_raw_1_json("SK/2DCode/raw/1.json")]
// #[case::sk_2dcode_raw_2_json("SK/2DCode/raw/2.json")]
// #[case::sk_2dcode_raw_3_json("SK/2DCode/raw/3.json")]
// #[case::sk_2dcode_raw_4_json("SK/2DCode/raw/4.json")]
// #[case::sk_2dcode_raw_5_json("SK/2DCode/raw/5.json")]
// #[case::sk_2dcode_raw_6_json("SK/2DCode/raw/6.json")]
// #[case::sk_2dcode_raw_7_json("SK/2DCode/raw/7.json")]
// #[case::sk_2dcode_raw_8_json("SK/2DCode/raw/8.json")]
// #[case::sm_2dcode_raw_1_json("SM/2DCode/raw/1.json")]
// #[case::sm_2dcode_raw_2_json("SM/2DCode/raw/2.json")]
// #[case::sm_2dcode_raw_3_json("SM/2DCode/raw/3.json")]
// #[case::sm_2dcode_raw_4_json("SM/2DCode/raw/4.json")]
// #[case::sm_2dcode_raw_5_json("SM/2DCode/raw/5.json")]
// #[case::sm_2dcode_raw_6_json("SM/2DCode/raw/6.json")]
// #[case::ua_2dcode_raw_1_json("UA/2DCode/raw/1.json")]
// #[case::ua_2dcode_raw_2_json("UA/2DCode/raw/2.json")]
// #[case::ua_2dcode_raw_3_json("UA/2DCode/raw/3.json")]
// #[case::ua_2dcode_raw_4_json("UA/2DCode/raw/4.json")]
// #[case::ua_2dcode_raw_5_json("UA/2DCode/raw/5.json")]
// #[case::va_2dcode_raw_1_json("VA/2DCode/raw/1.json")]
// #[case::va_2dcode_raw_2_json("VA/2DCode/raw/2.json")]
// #[case::va_2dcode_raw_3_json("VA/2DCode/raw/3.json")]
fn test_case(#[case] test_file: &str) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("data");
    path.push(test_file);

    let file_content = fs::read_to_string(path).unwrap();
    let test_data: Value = serde_json::from_str(file_content.as_str()).unwrap();

    let raw_hcert = test_data["PREFIX"].as_str().unwrap();

    let cwt = decode_cwt(raw_hcert);
    if !test_data["EXPECTEDRESULTS"]["EXPECTEDUNPREFIX"]
        .as_bool()
        .unwrap_or(true)
    {
        assert!(matches!(cwt, Err(ParseError::InvalidPrefix(_))));
        return;
    }
    if !test_data["EXPECTEDRESULTS"]["EXPECTEDB45DECODE"]
        .as_bool()
        .unwrap_or(true)
    {
        assert!(matches!(cwt, Err(ParseError::Base45Decode(_))));
        return;
    }
    if !test_data["EXPECTEDRESULTS"]["EXPECTEDDECODE"]
        .as_bool()
        .unwrap_or(true)
    {
        assert!(matches!(cwt, Err(ParseError::CwtDecode(_))));
        return;
    }
    if !test_data["EXPECTEDRESULTS"]["EXPECTEDCOMPRESSION"]
        .as_bool()
        .unwrap_or(true)
    {
        assert!(matches!(cwt, Err(ParseError::Deflate(_))));
        return;
    }
    if !test_data["EXPECTEDRESULTS"]["EXPECTEDVERIFY"]
        .as_bool()
        .unwrap_or(true)
    {
        // in some files (e.g. "common/2DCode/raw/CBO2.json") EXPECTEDVERIFY = false
        // is used to indicate malformed codes. So we need to exit early if the file is malformed
        if matches!(cwt, Err(_)) {
            return;
        }
    }
    if !test_data["EXPECTEDRESULTS"]["EXPECTEDSCHEMAVALIDATION"]
        .as_bool()
        .unwrap_or(true)
    {
        // some files (e.g. "common/2DCode/raw/DGC2.json") have a schema that can be decoded but
        // it is considered malformed because it contains multiple certificates.
        // For this reasone we expect a decode error only if there is no sample JSON data
        if test_data.get("JSON").is_none() {
            assert!(matches!(cwt, Err(ParseError::CwtDecode(_))));
            return;
        }
    }

    let cwt = cwt.unwrap();

    // compare the content only if the JSON field is populated in test data
    if test_data.get("JSON").is_some() {
        let cert_content = serde_json::ser::to_string(&test_data["JSON"]).unwrap();
        let expected_cert_payload: DgcCert = serde_json::from_str(cert_content.as_str()).unwrap();
        // makes sure that the content of the decoded certificate matches the expectation
        assert_eq!(*cwt.payload.certs.get(&1).unwrap(), expected_cert_payload);
    }

    // Validates signature only if the CERTIFICATE field is populated in test data
    if test_data["TESTCTX"].get("CERTIFICATE").is_some() {
        let mut trustlist = TrustList::default();

        let add_key_result = trustlist.add_key_from_certificate(
            &cwt.header_protected.kid.unwrap(),
            test_data["TESTCTX"]["CERTIFICATE"].as_str().unwrap(),
        );

        // if the key is RSA skip the test (only EC supported)
        let testctx_description = test_data["TESTCTX"]["DESCRIPTION"].as_str().unwrap_or("");
        if testctx_description.contains("RSA") {
            assert!(matches!(
                add_key_result,
                Err(KeyParseError::PublicKeyParseError(_))
            ));
            return;
        }

        let (_, signature_validity) = validate(raw_hcert, &trustlist).unwrap();
        let expected_verify = test_data["EXPECTEDRESULTS"]["EXPECTEDVERIFY"]
            .as_bool()
            .unwrap_or(true);
        assert_eq!(signature_validity.is_valid(), expected_verify);
    }
}
