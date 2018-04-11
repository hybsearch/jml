#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::fmt;

#[cfg(test)]
mod tests {
    use super::Phylip;
    use super::PhylipSample;

    #[test]
    fn simple_file() {
        let result = Phylip::from_str("8 843
blgcgdgca1-ggaacaataaattatcacctcaaaagacacataaaaccacaggaaccccta-ctcaaccataaaaaacacaaatccactattaaaaatcattaacaacacctttattgatctccccaccccatctaacatctctactttatgaaactttggttcattactaggggcatgcctcattctacaactagccacaggaatcttcttagctatacactactcatctgatatttccatagcattctcatcaatctcccacatccaacgagacgttcaatatggctgactaattcgaaatatacacgctaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatctactacggttcatacctttacaagaaaacctgaaacactggagtaatattactactcctagtcatagccactgcattcgtgggctacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaatctcctatcagccattccatatgtaggccctacacttgtagaatgaatttgaggggggttctccgtagataacgccacccttacccgattctttacattccacttcctaatcccattcgccatcctaggaataactatactacatctactactactacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcctataaagatctactaggccttattctaataataacactcctactcacccttaccctattctaccctaacctactaggagatccagacaactttacaccagccaacccactaaccacc
ahjmjgeja1ggaacaatcaattattaccccacaaagacacataaaaccacaggaacccctactccaaccataaaaaacacaaatccactattaaaaatcattaacaacaccttcgttgatcttcccaccccatccaacatctccgctttatgaaacttcggatcattactaggagcatgcctcattctacaactagccacaggaatcttcttagctatacactactcatccgacatttctacagcattctcatcaatttcccacatccaacgagatgttcaatatggatgacttattcgaaatatacacgccaacggtgcttcattattctttatatgcatctaccttcatattggacgagggatctactacggttcctacctttacaacaaaacctgaaacactggagtaatattactactcctagttatagccactgcattcgtgggctacgtgctaccatgaggccaaatatcattctgaggggctacagtaatcaccaacctattatcagccattccatacgtaggcccaacacttgtagagtggatttgaggagggttctccgtagacaacgctactcttactcgattcttcacatttcactttctaatcccattcgccatcctaggaataaccctactacacctcctacttctacacgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcgtataaagacctactaggcctcatcctaataattatatgtctactcacccttaccctatttcacccaaacctactaggagatccagacaactttacaccagccaatccactaaccacc
ahjmjgejabggaacaatcaattattaccccacaaagacacataaaaccacaggaacctctactccaaccataaaaaacacaaatccactattaaaaatcattaacaacaccttcgttgatcttcccaccccatccaacatctccgctttatgaaacttcggatcattactaggagcatgcctcattctacaactagccacaggaatcttcttagctatacactactcatccgacatttctacagcattctcatcaatttcccacatccaacgagatgttcaatatggatgacttattcgaaatatacacgccaacggtgcttcattattctttatatgcatctaccttcatattggacgagggatctactacggttcctacctttacaacaaaacctgaaacactggagtaatattactactcctagttatagccactgcattcgtgggctacgtgctaccatgaggccaaatatcattctgaggggctacagtaatcaccaacctattatcagccattccatacgtaggcccaacacttgtagagtggatttgaggagggttctccgtagacaacgctactcttactcgattcttcacatttcactttctaatcccattcgccatcctaggaataaccctactacacctcctacttctacacgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcgtataaagacctactaggcctcatcctaataattatatgtctactcacccttaccctatttcacccaaacctactaggagatccagacaactttacaccagccaatccactaaccacc
defdnggja1-ggaacaataa--attatccccaaaagacacataaaactacaggaa-cccctactcaaccataaaaaacacaaacccactactaaaaatcattaataatacctttattgatctccccactccacctaacatctccgctttgtgaaactttggatcattactaggggcatgcctcgctctacaactagccacaggaatcttcttagcgatacactactcatccgatatctccatagcattctcatcaatctcccacatccaacgagatgttcaatatggctgactaattcgaaatatacacgccaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatttactacggttcatatctttacaacaaaacctgaaacactggggtaatattactactcctagtcatagccactgcattcgtgggatacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaatctcttatcagccattccatacgtaggccctacacttgtagagtgaatctgagggggattctccgtagatagtgctacccttactcgattcttcacattccactttctaatcccattcgctatcctagggataactatactacatctactactgctacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagatctactaggcctcattctgataatcacacttctactcactcttaccctattctaccccaacctactaggagacccagacaactttacaccagccaacccactaaccacc
defdnggja2-ggaacaataa--attatccccaaaagacacataaaactacaggaa-cccctactcaaccataaaaaacacaaacccactattaaaaatcattaataatacctttattgacctccccactccacctaacatctccgctttgtgaaactttggatcattactaggggcatgcctcgctctacaactagccacaggaatcttcttagcgatacactactcatccgatatctccatagcattctcatcaatctcccacatccaacgagatgttcaatatggctgactaattcgaaatatacacgccaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatttactacggttcatatctttacaacaaaacctgaaacactggggtaatattactactcctagtcatagccactgcattcgtgggatacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaatctattatcagccattccatacgtaggccctacacttgtagagtgaatctgagggggattctccgtagatagtgctacccttactcgattcttcacattccactttctaatcccattcgctatcctagggataactatactacatctactattgctacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagatctactaggcctcattctgataatcacacttctactcactcttaccctattctaccccaacctactaggagacccagacaactttacaccagccaacccactaaccacc
blgcgdgca2-ggaacaataaattaccaccccaaaagacacataaaaccacaggaacccctaccccaaccataaaaaacacaaatccactattaaaaatcattaacaataccttcattgacctccccaccccatctaacatctccgctctatgaaactttgggtcattactaggggcatgccttattctacaactagccacaggaatcttcttagctatacactactcatccgacatttccatagcattctcatcaatctcccacatccaacgagacgttcaatatggctgactaattcgaaatatacacgctaacggtgcttcattattctttatatgcatttacctccatattgggcgaggaatttactacggttcatacctttacaagaaaacttgaaacactggagtaatattactactcctagtaatagccactgcattcgtgggctacgtactaccatgaggacaaatatcattctgaggggctacagtaatcactaatcttctatcagccattccatatgtaggccctacacttgtagaatgaatttgaggaggattctccgtagataacgccacccttacccgattctttacattccactttctaatcccattcgccatcctgggaataactatactacatctactactactacatgaaacaggatcaaataacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagacctactaggccttattctaataatcacactcctactcacccttaccctgttctaccccaacctactaggagacccagacaactttacaccagccaacccactgaccacc
blgcgdgca3-ggaacaataaattacctccccaaaagacacataaaactacaggaacccctacctcaaccataaaaaacacaaacccactattaaaaatcattaataatacctttattgatctccccactccacctaacatctccgctttgtgaaactttggatcattactaggggcatgcctcgctctacaactagccacaggaatcttcttagcgatacactactcatctgatatctccatagcattctcatcaatctcccacatccaacgagatgttcaatatggctgactaattcgaaatatacacgccaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatttactacggttcatacctttacaacaaaacctgaaacactggggtaatattactactcctagtcatagccactgcattcgtgggatacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaacctcttatcagccattccatacgtaggccctacacttgtagagtgaatctgagggggattctccgtagatagtgctacccttactcgattcttcacattccactttctaatcccattcgctatcctagggataactatactacatctactactgctacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagatctactaggcctcattctgataatcacacttctactcactcttaccctattctaccccaacctactaggagacccagacaactttacaccagccaacccactaaccacc
gpnkepgg1 -ggaacaatcaattattacccacaaagacacataaaaccacaggaacctctactccaaccataaaaaacacaaatccactattaaaaatcattaataacacctttgttgatcttcccaccccatccaatatctccgccttatgaaacttcggatcattactaggggcatgcctcattctacaactagccacaggaattttcttagccatacactactcatccgacatttctacagcattctcatcaatttcccacatccaacgagatgttcaatatggatgacttattcgaaatatacacgccaacggtgcttcattattctttatatgcatctaccttcatattggacgaggaatctattacggttcctacctttacaataaaacctgaaacactggagtaatattactactactagtcatagccactgcattcgtaggctacgtactaccatgaggccaaatatcattctgaggggctacagtaatcaccaacctattatcagccatcccatacgtaggcccaacacttgtagagtgaatttgaggagggttctccgtagacaacgccacccttactcgattcttcacatttcactttctaatcccattcgccatcctaggaataaccctactacacctcctacttctacacgaaacaggatcaaataacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcgtataaagacctactaggactcatcctaataatcatatgtctacttaccctcaccctattttacccaaacctactaggagatccagacaactttacaccagccaatccactaaccact
").unwrap();

        let expected = Phylip::new(vec![
            PhylipSample::new( "blgcgdgca1", "-ggaacaataaattatcacctcaaaagacacataaaaccacaggaaccccta-ctcaaccataaaaaacacaaatccactattaaaaatcattaacaacacctttattgatctccccaccccatctaacatctctactttatgaaactttggttcattactaggggcatgcctcattctacaactagccacaggaatcttcttagctatacactactcatctgatatttccatagcattctcatcaatctcccacatccaacgagacgttcaatatggctgactaattcgaaatatacacgctaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatctactacggttcatacctttacaagaaaacctgaaacactggagtaatattactactcctagtcatagccactgcattcgtgggctacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaatctcctatcagccattccatatgtaggccctacacttgtagaatgaatttgaggggggttctccgtagataacgccacccttacccgattctttacattccacttcctaatcccattcgccatcctaggaataactatactacatctactactactacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcctataaagatctactaggccttattctaataataacactcctactcacccttaccctattctaccctaacctactaggagatccagacaactttacaccagccaacccactaaccacc" ),
            PhylipSample::new( "ahjmjgeja1", "ggaacaatcaattattaccccacaaagacacataaaaccacaggaacccctactccaaccataaaaaacacaaatccactattaaaaatcattaacaacaccttcgttgatcttcccaccccatccaacatctccgctttatgaaacttcggatcattactaggagcatgcctcattctacaactagccacaggaatcttcttagctatacactactcatccgacatttctacagcattctcatcaatttcccacatccaacgagatgttcaatatggatgacttattcgaaatatacacgccaacggtgcttcattattctttatatgcatctaccttcatattggacgagggatctactacggttcctacctttacaacaaaacctgaaacactggagtaatattactactcctagttatagccactgcattcgtgggctacgtgctaccatgaggccaaatatcattctgaggggctacagtaatcaccaacctattatcagccattccatacgtaggcccaacacttgtagagtggatttgaggagggttctccgtagacaacgctactcttactcgattcttcacatttcactttctaatcccattcgccatcctaggaataaccctactacacctcctacttctacacgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcgtataaagacctactaggcctcatcctaataattatatgtctactcacccttaccctatttcacccaaacctactaggagatccagacaactttacaccagccaatccactaaccacc" ),
            PhylipSample::new( "ahjmjgejab", "ggaacaatcaattattaccccacaaagacacataaaaccacaggaacctctactccaaccataaaaaacacaaatccactattaaaaatcattaacaacaccttcgttgatcttcccaccccatccaacatctccgctttatgaaacttcggatcattactaggagcatgcctcattctacaactagccacaggaatcttcttagctatacactactcatccgacatttctacagcattctcatcaatttcccacatccaacgagatgttcaatatggatgacttattcgaaatatacacgccaacggtgcttcattattctttatatgcatctaccttcatattggacgagggatctactacggttcctacctttacaacaaaacctgaaacactggagtaatattactactcctagttatagccactgcattcgtgggctacgtgctaccatgaggccaaatatcattctgaggggctacagtaatcaccaacctattatcagccattccatacgtaggcccaacacttgtagagtggatttgaggagggttctccgtagacaacgctactcttactcgattcttcacatttcactttctaatcccattcgccatcctaggaataaccctactacacctcctacttctacacgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcgtataaagacctactaggcctcatcctaataattatatgtctactcacccttaccctatttcacccaaacctactaggagatccagacaactttacaccagccaatccactaaccacc" ),
            PhylipSample::new( "defdnggja1", "-ggaacaataa--attatccccaaaagacacataaaactacaggaa-cccctactcaaccataaaaaacacaaacccactactaaaaatcattaataatacctttattgatctccccactccacctaacatctccgctttgtgaaactttggatcattactaggggcatgcctcgctctacaactagccacaggaatcttcttagcgatacactactcatccgatatctccatagcattctcatcaatctcccacatccaacgagatgttcaatatggctgactaattcgaaatatacacgccaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatttactacggttcatatctttacaacaaaacctgaaacactggggtaatattactactcctagtcatagccactgcattcgtgggatacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaatctcttatcagccattccatacgtaggccctacacttgtagagtgaatctgagggggattctccgtagatagtgctacccttactcgattcttcacattccactttctaatcccattcgctatcctagggataactatactacatctactactgctacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagatctactaggcctcattctgataatcacacttctactcactcttaccctattctaccccaacctactaggagacccagacaactttacaccagccaacccactaaccacc" ),
            PhylipSample::new( "defdnggja2", "-ggaacaataa--attatccccaaaagacacataaaactacaggaa-cccctactcaaccataaaaaacacaaacccactattaaaaatcattaataatacctttattgacctccccactccacctaacatctccgctttgtgaaactttggatcattactaggggcatgcctcgctctacaactagccacaggaatcttcttagcgatacactactcatccgatatctccatagcattctcatcaatctcccacatccaacgagatgttcaatatggctgactaattcgaaatatacacgccaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatttactacggttcatatctttacaacaaaacctgaaacactggggtaatattactactcctagtcatagccactgcattcgtgggatacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaatctattatcagccattccatacgtaggccctacacttgtagagtgaatctgagggggattctccgtagatagtgctacccttactcgattcttcacattccactttctaatcccattcgctatcctagggataactatactacatctactattgctacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagatctactaggcctcattctgataatcacacttctactcactcttaccctattctaccccaacctactaggagacccagacaactttacaccagccaacccactaaccacc" ),
            PhylipSample::new( "blgcgdgca2", "-ggaacaataaattaccaccccaaaagacacataaaaccacaggaacccctaccccaaccataaaaaacacaaatccactattaaaaatcattaacaataccttcattgacctccccaccccatctaacatctccgctctatgaaactttgggtcattactaggggcatgccttattctacaactagccacaggaatcttcttagctatacactactcatccgacatttccatagcattctcatcaatctcccacatccaacgagacgttcaatatggctgactaattcgaaatatacacgctaacggtgcttcattattctttatatgcatttacctccatattgggcgaggaatttactacggttcatacctttacaagaaaacttgaaacactggagtaatattactactcctagtaatagccactgcattcgtgggctacgtactaccatgaggacaaatatcattctgaggggctacagtaatcactaatcttctatcagccattccatatgtaggccctacacttgtagaatgaatttgaggaggattctccgtagataacgccacccttacccgattctttacattccactttctaatcccattcgccatcctgggaataactatactacatctactactactacatgaaacaggatcaaataacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagacctactaggccttattctaataatcacactcctactcacccttaccctgttctaccccaacctactaggagacccagacaactttacaccagccaacccactgaccacc" ),
            PhylipSample::new( "blgcgdgca3", "-ggaacaataaattacctccccaaaagacacataaaactacaggaacccctacctcaaccataaaaaacacaaacccactattaaaaatcattaataatacctttattgatctccccactccacctaacatctccgctttgtgaaactttggatcattactaggggcatgcctcgctctacaactagccacaggaatcttcttagcgatacactactcatctgatatctccatagcattctcatcaatctcccacatccaacgagatgttcaatatggctgactaattcgaaatatacacgccaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatttactacggttcatacctttacaacaaaacctgaaacactggggtaatattactactcctagtcatagccactgcattcgtgggatacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaacctcttatcagccattccatacgtaggccctacacttgtagagtgaatctgagggggattctccgtagatagtgctacccttactcgattcttcacattccactttctaatcccattcgctatcctagggataactatactacatctactactgctacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcttataaagatctactaggcctcattctgataatcacacttctactcactcttaccctattctaccccaacctactaggagacccagacaactttacaccagccaacccactaaccacc" ),
            PhylipSample::new( "gpnkepgg1", "-ggaacaatcaattattacccacaaagacacataaaaccacaggaacctctactccaaccataaaaaacacaaatccactattaaaaatcattaataacacctttgttgatcttcccaccccatccaatatctccgccttatgaaacttcggatcattactaggggcatgcctcattctacaactagccacaggaattttcttagccatacactactcatccgacatttctacagcattctcatcaatttcccacatccaacgagatgttcaatatggatgacttattcgaaatatacacgccaacggtgcttcattattctttatatgcatctaccttcatattggacgaggaatctattacggttcctacctttacaataaaacctgaaacactggagtaatattactactactagtcatagccactgcattcgtaggctacgtactaccatgaggccaaatatcattctgaggggctacagtaatcaccaacctattatcagccatcccatacgtaggcccaacacttgtagagtgaatttgaggagggttctccgtagacaacgccacccttactcgattcttcacatttcactttctaatcccattcgccatcctaggaataaccctactacacctcctacttctacacgaaacaggatcaaataacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcgtataaagacctactaggactcatcctaataatcatatgtctacttaccctcaccctattttacccaaacctactaggagatccagacaactttacaccagccaatccactaaccact" )
        ]).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "number of samples (1) and expected sample count (8) do not match")]
    fn mismatched_sample_counts() {
        Phylip::from_str("8 843\nblgcgdgca1-ggaacaataaattatcacctcaaaagacacataaaaccacaggaaccccta-ctcaaccataaaaaacacaaatccactattaaaaatcattaacaacacctttattgatctccccaccccatctaacatctctactttatgaaactttggttcattactaggggcatgcctcattctacaactagccacaggaatcttcttagctatacactactcatctgatatttccatagcattctcatcaatctcccacatccaacgagacgttcaatatggctgactaattcgaaatatacacgctaacggtgcttcattattctttatatgcatttacctccatattggacgaggaatctactacggttcatacctttacaagaaaacctgaaacactggagtaatattactactcctagtcatagccactgcattcgtgggctacgtactaccatgaggacaaatatcattctgaggagctacagtaatcaccaatctcctatcagccattccatatgtaggccctacacttgtagaatgaatttgaggggggttctccgtagataacgccacccttacccgattctttacattccacttcctaatcccattcgccatcctaggaataactatactacatctactactactacatgaaacaggatcaaacaacccaacaggactaaactcaaactgtgacaaaatcccattccacccatatttctcctataaagatctactaggccttattctaataataacactcctactcacccttaccctattctaccctaacctactaggagatccagacaactttacaccagccaacccactaaccacc").unwrap();
    }

    #[test]
    #[should_panic(expected = "not all sequence lengths matched")]
    fn easy_new() {
        Phylip::new(vec![
            PhylipSample::new("blgcgdgca1", "ggac"),
            PhylipSample::new("ahjmjgeja1", "ggactt"),
            PhylipSample::new("ahjmjgejab", "ggac"),
        ]).unwrap();
    }

    #[test]
    #[should_panic(expected = "all samples did not conform to the expected length of 843 (including sample at 0)")]
    fn mismatched_sample_lengths() {
        Phylip::from_str("1 843\nblgcgdgca1-ggaa").unwrap();
    }

    #[test]
    #[should_panic(expected = "Need at least one line to parse")]
    fn empty_file() {
        Phylip::from_str("").unwrap();
    }

    #[test]
    #[should_panic(expected = "Did not find the sequence counter and sequence lengths on the first line")]
    fn missing_counters() {
        Phylip::from_str("blgcgdgca1-ggaa").unwrap();
    }

    #[test]
    fn simple_serialize() {
        let result = Phylip::from_str("1 18\nblgcgdgca1-ggaacaataaattatca").unwrap();
        let serialized = format!("{}", result);
        assert_eq!(serialized, "1 18\nblgcgdgca1-ggaacaataaattatca");
    }

    #[test]
    fn multisequence_serialize() {
        let phy: Phylip = Phylip::new(vec![
            PhylipSample::new("blgcgdgca1", "-ggaacaataaattatca"),
            PhylipSample::new("ahjmjgeja1", "ggaacaatcaattattac"),
            PhylipSample::new("ahjmjgejab", "ggaacaatcaattattac"),
        ]).unwrap();
        let serialized = format!("{}", phy);
        assert_eq!(serialized, "3 18\nblgcgdgca1-ggaacaataaattatca\nahjmjgeja1ggaacaatcaattattac\nahjmjgejabggaacaatcaattattac");
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhylipSample {
    pub identifier: String,
    pub sequence: String,
}

impl PhylipSample {
    pub fn new<S>(identifier: S, sequence: S) -> PhylipSample
    where
        S: Into<String>,
    {
        PhylipSample {
            identifier: identifier.into(),
            sequence: sequence.into(),
        }
    }
}

impl fmt::Display for PhylipSample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:width$}{}", self.identifier, self.sequence, width = 10)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Phylip {
    pub sequence_length: usize,
    pub sequences: Vec<PhylipSample>,
}

impl Phylip {
    pub fn from_str<S>(data: S) -> Result<Phylip, &'static str>
    where
        S: Into<String>,
    {
        let data_str = data.into();
        let first_line = data_str
            .lines()
            .take(1)
            .next()
            .expect("Need at least one line to parse");
        let (sequence_count, sequence_length) = match get_counts(first_line) {
            Ok(counts) => counts,
            Err(error) => return Err(error),
        };

        let samples: Vec<PhylipSample> = data_str.lines().skip(1).filter_map(parse_line).collect();

        debug_assert!(verify_lengths(
            samples.clone(),
            sequence_count,
            sequence_length
        ));

        Ok(Phylip {
            sequence_length,
            sequences: samples,
        })
    }

    fn hard_new(sequence_length: usize, sequences: Vec<PhylipSample>) -> Phylip {
        let sample_count = sequences.len();
        verify_lengths(sequences.clone(), sample_count, sequence_length);
        Phylip {
            sequence_length,
            sequences,
        }
    }

    pub fn new(sequences: Vec<PhylipSample>) -> Result<Phylip, &'static str> {
        if sequences.is_empty() {
            return Ok(Phylip {
                sequence_length: 0,
                sequences,
            });
        }

        let first_length = sequences[0].sequence.len();
        let lengths_are_good = sequences
            .iter()
            .all(|sample| sample.sequence.len() == first_length);

        if !lengths_are_good {
            return Err("not all sequence lengths matched");
        }

        Ok(Phylip {
            sequence_length: first_length,
            sequences,
        })
    }
}

impl fmt::Display for Phylip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self.sequences.iter().map(|sample| format!("{}", sample));
        let sequences = formatted.collect::<Vec<String>>().join("\n");
        write!(
            f,
            "{} {}\n{}",
            self.sequences.len(),
            self.sequence_length,
            sequences
        )
    }
}

fn verify_lengths(samples: Vec<PhylipSample>, sample_count: usize, sequence_length: usize) -> bool {
    if samples.len() != sample_count {
        panic!(format!(
            "number of samples ({:?}) and expected sample count ({:?}) do not match",
            samples.len(),
            sample_count
        ));
    }

    let lengths: Vec<usize> = samples.iter().map(|sample| sample.sequence.len()).collect();
    let lengths_are_good = lengths.iter().all(|seq_len| *seq_len == sequence_length);

    if !lengths_are_good {
        let problematic_position = lengths
            .iter()
            .position(|seq_len| *seq_len != sequence_length)
            .unwrap();

        panic!(format!(
            "all samples did not conform to the expected length of {:?} (including sample at {:?})",
            sequence_length, problematic_position
        ));
    }

    true
}

fn parse_line(line: &str) -> Option<PhylipSample> {
    let identifier_length = 10;

    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    let (identifier, sequence) = trimmed.split_at(identifier_length);

    Some(PhylipSample {
        identifier: identifier.trim().to_string(),
        sequence: sequence.to_string(),
    })
}

fn get_counts(line: &str) -> Result<(usize, usize), &'static str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)(?P<seq_count>\d+)\s+(?P<seq_len>\d+)").unwrap();
    }

    match RE.captures(line) {
        Some(captures) => {
            let sequence_count: usize = captures["seq_count"].parse().unwrap();
            let sequence_length: usize = captures["seq_len"].parse().unwrap();
            Ok((sequence_count, sequence_length))
        }
        None => Err("Did not find the sequence counter and sequence lengths on the first line"),
    }
}
