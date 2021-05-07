//! Correct algorithms for string-to-float conversions.

use crate::util::*;

use super::alias::*;
use super::format::*;
use super::power_of_n as pown;
#[cfg(feature = "power_of_two")]
use super::power_of_two as pow2;

// DISPATCHER

/// Parse native float from string.
///
/// The float string must be non-special, non-zero, and positive.
#[inline(always)]
pub(crate) fn to_native<'a, F, Data>(
    data: Data,
    bytes: &'a [u8],
    sign: Sign,
    radix: u32,
    is_incorrect: bool,
    is_lossy: bool,
    rounding: RoundingKind,
) -> ParseResult<(F, *const u8)>
where
    F: FloatType,
    Data: FastDataInterface<'a>,
{
    #[cfg(not(feature = "power_of_two"))]
    {
        pown::to_native(data, bytes, radix, is_incorrect, is_lossy, sign, rounding)
    }

    #[cfg(feature = "power_of_two")]
    {
        let pow2_exp = log2(radix);
        match pow2_exp {
            0 => pown::to_native(data, bytes, radix, is_incorrect, is_lossy, sign, rounding),
            _ => pow2::to_native(data, bytes, radix, pow2_exp, sign, rounding),
        }
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atof_test() {
        let atof10 = move |x| f32::from_lexical_partial(x);

        assert_eq!(Ok((0.0, 1)), atof10(b"0"));
        assert_eq!(Ok((1.2345, 6)), atof10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atof10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atof10(b"12345.6789"));
        assert_eq!(Ok((1.2345e10, 9)), atof10(b"1.2345e10"));
        assert_eq!(Ok((1.2345e-38, 10)), atof10(b"1.2345e-38"));

        // Check expected rounding, using borderline cases.
        // Round-down, halfway
        assert_eq!(Ok((16777216.0, 8)), atof10(b"16777216"));
        assert_eq!(Ok((16777216.0, 8)), atof10(b"16777217"));
        assert_eq!(Ok((16777218.0, 8)), atof10(b"16777218"));
        assert_eq!(Ok((33554432.0, 8)), atof10(b"33554432"));
        assert_eq!(Ok((33554432.0, 8)), atof10(b"33554434"));
        assert_eq!(Ok((33554436.0, 8)), atof10(b"33554436"));
        assert_eq!(Ok((17179869184.0, 11)), atof10(b"17179869184"));
        assert_eq!(Ok((17179869184.0, 11)), atof10(b"17179870208"));
        assert_eq!(Ok((17179871232.0, 11)), atof10(b"17179871232"));

        // Round-up, halfway
        assert_eq!(Ok((16777218.0, 8)), atof10(b"16777218"));
        assert_eq!(Ok((16777220.0, 8)), atof10(b"16777219"));
        assert_eq!(Ok((16777220.0, 8)), atof10(b"16777220"));
        assert_eq!(Ok((33554436.0, 8)), atof10(b"33554436"));
        assert_eq!(Ok((33554440.0, 8)), atof10(b"33554438"));
        assert_eq!(Ok((33554440.0, 8)), atof10(b"33554440"));
        assert_eq!(Ok((17179871232.0, 11)), atof10(b"17179871232"));
        assert_eq!(Ok((17179873280.0, 11)), atof10(b"17179872256"));
        assert_eq!(Ok((17179873280.0, 11)), atof10(b"17179873280"));

        // Round-up, above halfway
        assert_eq!(Ok((33554436.0, 8)), atof10(b"33554435"));
        assert_eq!(Ok((17179871232.0, 11)), atof10(b"17179870209"));

        // Check exactly halfway, round-up at halfway
        assert_eq!(Ok((1.0000001, 28)), atof10(b"1.00000017881393432617187499"));
        assert_eq!(Ok((1.0000002, 26)), atof10(b"1.000000178813934326171875"));
        assert_eq!(Ok((1.0000002, 28)), atof10(b"1.00000017881393432617187501"));

        // Invalid or partially-parsed
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), atof10(b"e10"));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), atof10(b"."));
        assert_eq!(Err((ErrorCode::EmptyMantissa, 0).into()), atof10(b".e10"));
        assert_eq!(Err((ErrorCode::EmptyExponent, 2).into()), atof10(b"0e"));
        assert_eq!(Ok((1.23, 4)), atof10(b"1.23/"));

        // Errors identified via test-parse-random.
        assert_eq!(Ok((0.0, 21)), atof10(b"1.565385248817619e-82"));
        assert_eq!(Ok((5.483634359675252e34, 20)), atof10(b"5.483634359675252e34"));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn atof_test_roundtrip() {
        let mut buffer = new_buffer();
        let values: [f32; 8] = [
            1.2345678901234567890e0,
            1.2345678901234567890e1,
            1.2345678901234567890e2,
            1.2345678901234567890e3,
            -1.2345678901234567890e0,
            -1.2345678901234567890e1,
            -1.2345678901234567890e2,
            -1.2345678901234567890e3,
        ];
        let radixes = [2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16];
        for &radix in radixes.iter() {
            // Round-trip for base2.
            let parse_opts = ParseFloatOptions::builder().radix(radix).build().unwrap();
            let write_opts = WriteFloatOptions::builder().radix(radix).build().unwrap();
            let mut roundtrip = |x| {
                let written = f32::to_lexical_with_options(x, &mut buffer, &write_opts);
                let parsed = f32::from_lexical_with_options(written, &parse_opts);
                parsed == Ok(x)
            };
            for &value in values.iter() {
                assert!(roundtrip(value));
            }
        }
    }

    #[test]
    fn atod_test() {
        let atod10 = move |x| f64::from_lexical_partial(x);

        #[cfg(feature = "power_of_two")]
        let atod2 = move |x| {
            let options = ParseFloatOptions::binary();
            f64::from_lexical_partial_with_options(x, &options)
        };

        assert_eq!(Ok((0.0, 1)), atod10(b"0"));
        assert_eq!(Ok((1.2345, 6)), atod10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atod10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atod10(b"12345.6789"));
        assert_eq!(Ok((1.2345e10, 9)), atod10(b"1.2345e10"));
        assert_eq!(Ok((1e-323, 6)), atod10(b"1e-323"));
        assert_eq!(Ok((1.2345e-308, 11)), atod10(b"1.2345e-308"));

        // Check expected rounding, using borderline cases.
        // Round-down, halfway
        assert_eq!(Ok((9007199254740992.0, 16)), atod10(b"9007199254740992"));
        assert_eq!(Ok((9007199254740992.0, 16)), atod10(b"9007199254740993"));
        assert_eq!(Ok((9007199254740994.0, 16)), atod10(b"9007199254740994"));
        assert_eq!(Ok((18014398509481984.0, 17)), atod10(b"18014398509481984"));
        assert_eq!(Ok((18014398509481984.0, 17)), atod10(b"18014398509481986"));
        assert_eq!(Ok((18014398509481988.0, 17)), atod10(b"18014398509481988"));
        assert_eq!(Ok((9223372036854775808.0, 19)), atod10(b"9223372036854775808"));
        assert_eq!(Ok((9223372036854775808.0, 19)), atod10(b"9223372036854776832"));
        assert_eq!(Ok((9223372036854777856.0, 19)), atod10(b"9223372036854777856"));
        assert_eq!(
            Ok((11417981541647679048466287755595961091061972992.0, 47)),
            atod10(b"11417981541647679048466287755595961091061972992")
        );
        assert_eq!(
            Ok((11417981541647679048466287755595961091061972992.0, 47)),
            atod10(b"11417981541647680316116887983825362587765178368")
        );
        assert_eq!(
            Ok((11417981541647681583767488212054764084468383744.0, 47)),
            atod10(b"11417981541647681583767488212054764084468383744")
        );

        // Round-up, halfway
        assert_eq!(Ok((9007199254740994.0, 16)), atod10(b"9007199254740994"));
        assert_eq!(Ok((9007199254740996.0, 16)), atod10(b"9007199254740995"));
        assert_eq!(Ok((9007199254740996.0, 16)), atod10(b"9007199254740996"));
        assert_eq!(Ok((18014398509481988.0, 17)), atod10(b"18014398509481988"));
        assert_eq!(Ok((18014398509481992.0, 17)), atod10(b"18014398509481990"));
        assert_eq!(Ok((18014398509481992.0, 17)), atod10(b"18014398509481992"));
        assert_eq!(Ok((9223372036854777856.0, 19)), atod10(b"9223372036854777856"));
        assert_eq!(Ok((9223372036854779904.0, 19)), atod10(b"9223372036854778880"));
        assert_eq!(Ok((9223372036854779904.0, 19)), atod10(b"9223372036854779904"));
        assert_eq!(
            Ok((11417981541647681583767488212054764084468383744.0, 47)),
            atod10(b"11417981541647681583767488212054764084468383744")
        );
        assert_eq!(
            Ok((11417981541647684119068688668513567077874794496.0, 47)),
            atod10(b"11417981541647682851418088440284165581171589120")
        );
        assert_eq!(
            Ok((11417981541647684119068688668513567077874794496.0, 47)),
            atod10(b"11417981541647684119068688668513567077874794496")
        );

        // Round-up, above halfway
        assert_eq!(Ok((9223372036854777856.0, 19)), atod10(b"9223372036854776833"));
        assert_eq!(
            Ok((11417981541647681583767488212054764084468383744.0, 47)),
            atod10(b"11417981541647680316116887983825362587765178369")
        );

        // Rounding error
        // Adapted from failures in strtod.
        assert_eq!(Ok((2.2250738585072014e-308, 23)), atod10(b"2.2250738585072014e-308"));
        assert_eq!(Ok((2.225073858507201e-308, 776)), atod10(b"2.2250738585072011360574097967091319759348195463516456480234261097248222220210769455165295239081350879141491589130396211068700864386945946455276572074078206217433799881410632673292535522868813721490129811224514518898490572223072852551331557550159143974763979834118019993239625482890171070818506906306666559949382757725720157630626906633326475653000092458883164330377797918696120494973903778297049050510806099407302629371289589500035837999672072543043602840788957717961509455167482434710307026091446215722898802581825451803257070188608721131280795122334262883686223215037756666225039825343359745688844239002654981983854879482922068947216898310996983658468140228542433306603398508864458040010349339704275671864433837704860378616227717385456230658746790140867233276367187499e-308"));
        assert_eq!(Ok((2.2250738585072014e-308, 774)), atod10(b"2.22507385850720113605740979670913197593481954635164564802342610972482222202107694551652952390813508791414915891303962110687008643869459464552765720740782062174337998814106326732925355228688137214901298112245145188984905722230728525513315575501591439747639798341180199932396254828901710708185069063066665599493827577257201576306269066333264756530000924588831643303777979186961204949739037782970490505108060994073026293712895895000358379996720725430436028407889577179615094551674824347103070260914462157228988025818254518032570701886087211312807951223342628836862232150377566662250398253433597456888442390026549819838548794829220689472168983109969836584681402285424333066033985088644580400103493397042756718644338377048603786162277173854562306587467901408672332763671875e-308"));
        assert_eq!(Ok((2.2250738585072014e-308, 776)), atod10(b"2.2250738585072011360574097967091319759348195463516456480234261097248222220210769455165295239081350879141491589130396211068700864386945946455276572074078206217433799881410632673292535522868813721490129811224514518898490572223072852551331557550159143974763979834118019993239625482890171070818506906306666559949382757725720157630626906633326475653000092458883164330377797918696120494973903778297049050510806099407302629371289589500035837999672072543043602840788957717961509455167482434710307026091446215722898802581825451803257070188608721131280795122334262883686223215037756666225039825343359745688844239002654981983854879482922068947216898310996983658468140228542433306603398508864458040010349339704275671864433837704860378616227717385456230658746790140867233276367187501e-308"));
        assert_eq!(Ok((1.7976931348623157e+308, 380)), atod10(b"179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791.9999999999999999999999999999999999999999999999999999999999999999999999"));
        assert_eq!(Ok((5e-324, 761)), atod10(b"7.4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984374999e-324"));
        assert_eq!(Ok((1e-323, 758)), atod10(b"7.4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375e-324"));
        assert_eq!(Ok((1e-323, 761)), atod10(b"7.4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375001e-324"));
        assert_eq!(Ok((1e-320, 6)), atod10(b"1e-320"));
        // Highest denormal float.
        assert_eq!(Ok((2.2250738585072009e-308, 23)), atod10(b"2.2250738585072009e-308"));

        // Rounding error
        // Adapted from:
        //  https://www.exploringbinary.com/glibc-strtod-incorrectly-converts-2-to-the-negative-1075/
        #[cfg(feature = "power_of_two")]
        assert_eq!(Ok((5e-324, 14)), atod2(b"1^-10000110010"));

        #[cfg(feature = "power_of_two")]
        assert_eq!(Ok((0.0, 14)), atod2(b"1^-10000110011"));
        assert_eq!(Ok((0.0, 1077)), atod10(b"0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000024703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125"));

        // Rounding error
        // Adapted from:
        //  https://www.exploringbinary.com/how-glibc-strtod-works/
        assert_eq!(Ok((2.2250738585072011e-308, 1076)), atod10(b"0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072008890245868760858598876504231122409594654935248025624400092282356951787758888037591552642309780950434312085877387158357291821993020294379224223559819827501242041788969571311791082261043971979604000454897391938079198936081525613113376149842043271751033627391549782731594143828136275113838604094249464942286316695429105080201815926642134996606517803095075913058719846423906068637102005108723282784678843631944515866135041223479014792369585208321597621066375401613736583044193603714778355306682834535634005074073040135602968046375918583163124224521599262546494300836851861719422417646455137135420132217031370496583210154654068035397417906022589503023501937519773030945763173210852507299305089761582519159720757232455434770912461317493580281734466552734375"));

        // Rounding error
        // Adapted from test-parse-random failures.
        assert_eq!(Ok((1.009e-28, 8)), atod10(b"1009e-31"));
        assert_eq!(Ok((f64::INFINITY, 9)), atod10(b"18294e304"));

        // Rounding error
        // Adapted from a @dangrabcad's issue #20.
        assert_eq!(Ok((7.689539722041643e164, 21)), atod10(b"7.689539722041643e164"));
        assert_eq!(Ok((7.689539722041643e164, 165)), atod10(b"768953972204164300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));
        assert_eq!(Ok((7.689539722041643e164, 167)), atod10(b"768953972204164300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0"));

        // Check other cases similar to @dangrabcad's issue #20.
        assert_eq!(Ok((9223372036854777856.0, 21)), atod10(b"9223372036854776833.0"));
        assert_eq!(
            Ok((11417981541647681583767488212054764084468383744.0, 49)),
            atod10(b"11417981541647680316116887983825362587765178369.0")
        );
        assert_eq!(Ok((9007199254740996.0, 18)), atod10(b"9007199254740995.0"));
        assert_eq!(Ok((18014398509481992.0, 19)), atod10(b"18014398509481990.0"));
        assert_eq!(Ok((9223372036854779904.0, 21)), atod10(b"9223372036854778880.0"));
        assert_eq!(
            Ok((11417981541647684119068688668513567077874794496.0, 49)),
            atod10(b"11417981541647682851418088440284165581171589120.0")
        );

        // Check other cases ostensibly identified via proptest.
        assert_eq!(Ok((71610528364411830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 310)), atod10(b"71610528364411830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0"));
        assert_eq!(Ok((126769393745745060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 311)), atod10(b"126769393745745060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0"));
        assert_eq!(Ok((38652960461239320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 310)), atod10(b"38652960461239320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0"));

        // Round-trip for base2.
        #[cfg(feature = "power_of_two")]
        {
            assert_eq!(
                Ok((f64::from_bits(0x3bcd261840000000), 33)),
                atod2(b"1.1101001001100001100001^-1000011")
            );
        }

        // Check other bugs in Golang.
        assert_eq!(Ok((1.0905441441816094e+30, 31)), atod10(b"1090544144181609348835077142190"));

        // Errors identified via test-parse-random.
        assert_eq!(Ok((-7.014172639932773e-283, 23)), atod10(b"-7.014172639932773e-283"));
        assert_eq!(Ok((1.565385248817619e-82, 21)), atod10(b"1.565385248817619e-82"));
    }

    #[test]
    fn atod_large_zeros_test() {
        // Test numbers with a massive number of 0s in the integer component.
        let atod10 = move |x| f64::from_lexical_partial(x);
        assert_eq!(Ok((71610528364411830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 308)), atod10(b"71610528364411830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));
        assert_eq!(Ok((126769393745745060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 309)), atod10(b"126769393745745060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));
        assert_eq!(Ok((38652960461239320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 308)), atod10(b"38652960461239320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));
    }

    #[test]
    #[cfg(feature = "power_of_two")]
    fn atod_pow2_test() {
        let atod2 = move |x| {
            let options = ParseFloatOptions::binary();
            f64::from_lexical_partial_with_options(x, &options)
        };

        // Test a wide variety of denormal floats here.

        // Halfway, round-up.
        assert_eq!(
            Ok((2.2250738585072009e-308, 65)),
            atod2(b"1111111111111111111111111111111111111111111111111111^-10000110010")
        );
        assert_eq!(
            Ok((2.2250738585072014e-308, 66)),
            atod2(b"10000000000000000000000000000000000000000000000000000^-10000110010")
        );
        assert_eq!(
            Ok((2.2250738585072009e-308, 66)),
            atod2(b"11111111111111111111111111111111111111111111111111110^-10000110011")
        );
        assert_eq!(
            Ok((2.2250738585072014e-308, 66)),
            atod2(b"11111111111111111111111111111111111111111111111111111^-10000110011")
        );

        // Halfway, round-down.
        assert_eq!(
            Ok((2.2250738585072004e-308, 65)),
            atod2(b"1111111111111111111111111111111111111111111111111110^-10000110010")
        );
        assert_eq!(
            Ok((2.2250738585072004e-308, 66)),
            atod2(b"11111111111111111111111111111111111111111111111111101^-10000110011")
        );

        // Force the moderate path (round-up).
        assert_eq!(
            Ok((2.2250738585072009e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111100000^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072009e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111101000^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072009e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111101111^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072014e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111110000^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072014e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111111111^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072014e-308, 70)),
            atod2(b"100000000000000000000000000000000000000000000000000000000^-10000110110")
        );

        // Force the moderate path (round-down).
        assert_eq!(
            Ok((2.2250738585072004e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111000000^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072004e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111001000^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072004e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111001111^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072004e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111010000^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072009e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111010001^-10000110111")
        );
        assert_eq!(
            Ok((2.2250738585072009e-308, 70)),
            atod2(b"111111111111111111111111111111111111111111111111111011111^-10000110111")
        );

        // Let's test comically long digits (round-up).
        assert_eq!(Ok((2.2250738585072009e-308, 170)), atod2(b"1111111111111111111111111111111111111111111111111111000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000^-10010011011"));
        assert_eq!(Ok((2.2250738585072009e-308, 170)), atod2(b"1111111111111111111111111111111111111111111111111111011111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111^-10010011011"));
        assert_eq!(Ok((2.2250738585072014e-308, 170)), atod2(b"1111111111111111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000^-10010011011"));
        assert_eq!(Ok((2.2250738585072014e-308, 170)), atod2(b"1111111111111111111111111111111111111111111111111111111111111111111111000000000000000000000000000000000000000000000000000000000000000000000000000000000000000^-10010011011"));
    }

    #[test]
    #[cfg(feature = "radix")]
    fn atod_test_roundtrip() {
        let mut buffer = new_buffer();
        let values: [f64; 8] = [
            1.2345678901234567890e0,
            1.2345678901234567890e1,
            1.2345678901234567890e2,
            1.2345678901234567890e3,
            -1.2345678901234567890e0,
            -1.2345678901234567890e1,
            -1.2345678901234567890e2,
            -1.2345678901234567890e3,
        ];
        let radixes = [2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16];
        for &radix in radixes.iter() {
            // Round-trip for base2.
            let parse_opts = ParseFloatOptions::builder().radix(radix).build().unwrap();
            let write_opts = WriteFloatOptions::builder().radix(radix).build().unwrap();
            let mut roundtrip = |x| {
                let written = f64::to_lexical_with_options(x, &mut buffer, &write_opts);
                let parsed = f64::from_lexical_with_options(written, &parse_opts);
                parsed == Ok(x)
            };
            for &value in values.iter() {
                assert!(roundtrip(value));
            }
        }
    }

    #[test]
    fn atof_lossy_test() {
        let options = ParseFloatOptions::builder().lossy(true).build().unwrap();
        let atof10 = move |x| f32::from_lexical_partial_with_options(x, &options);

        assert_eq!(Ok((1.2345, 6)), atof10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atof10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atof10(b"12345.6789"));
        assert_eq!(Ok((1.2345e10, 9)), atof10(b"1.2345e10"));
    }

    #[test]
    fn atod_lossy_test() {
        let options = ParseFloatOptions::builder().lossy(true).build().unwrap();
        let atod10 = move |x| f64::from_lexical_partial_with_options(x, &options);

        assert_eq!(Ok((1.2345, 6)), atod10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atod10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atod10(b"12345.6789"));
        assert_eq!(Ok((1.2345e10, 9)), atod10(b"1.2345e10"));
    }
}
