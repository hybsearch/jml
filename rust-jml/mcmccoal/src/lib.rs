use std::fs::File;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// max # of species
pub const NSPECIES: i32 = 200;
// max # of sequences
const NS: i32 = 1000;
const NBRANCH: i32 = NS * 2 - 2;
const MAXNSONS: i32 = 2;
// max # of loci
const NGENE: i32 = 50000;
// # characters in sequence names
const LSPNAME: i32 = 50;

struct Tree {
    nbranch: i32,
    nnode: i32,
    root: i32,
    branches: Vec<[i32; 2]>,
    //    int  nbranch, nnode, root, branches[NBRANCH][2];
    lnL: f64,
}

enum OutTreeOptions {
    PrBranch = 1,
    PrNodeNum = 2,
    PrLabel = 4,
    PrAge = 8,
}

enum DataTypes {
    BASEseq = 0,
    CODONseq,
    AAseq,
    CODON2AAseq,
}

fn ReadSpeciesTree(datafile: String) -> i32 {
    1
}

fn DownSptreeSetPops(inode: i32) -> i32 {
    1
}

fn SimulateData() {}

fn GetRandomGtree(locus: i32) {}

fn Rates4Sites(rates: Vec<f64>, alpha: f64, ncatG: i32, ls: i32, cdf: i32, space: Vec<f64>) {}

fn abyx(a: f64, x: Vec<f64>, n: i32) -> Vec<f64> {
    // { int i; for (i=0; i<n; x[i]*=a,i++) ;  return(0); }
    //    for val in &mut x {
    //        val = &mut ((*val) * a);
    //    }
    x.into_iter().map(|val| val * a).collect()
}

fn EvolveJC(inode: i32) {}

fn PMismatch3s() {}

fn p0124Fromb0b1(itree: i32, p: [f64; 5], b: [f64; 2]) {}

fn ResetSpeciesGeneTree(locus: i32) -> i32 {
    1
}

fn Coalescence1Pop(ispecies: i32) -> i32 {
    1
}

fn starttime() {}

fn MCMCSetSeed(seed: u32) {}

fn error2(message: String) {}

fn ReadaTreeN(ftree: File, haslength: i32, haslabel: i32, copyname: i32, popline: i32) -> i32 {
    1
}

fn OutaTreeN(fout: File, spnames: i32, printopt: i32) -> i32 {
    1
}

fn rndu() -> f64 {
    1.0
}

fn printSeqs(fout: File, pose: i32, keep: String, format: i32) {}

fn MultiNomial2(n: i32, ncat: i32, prob: Vec<f64>, nobs: Vec<i32>, space: Vec<f64>) -> i32 {
    1
}

fn rndpoisson(m: f64) -> i32 {
    1
}

fn LnGamma(x: f64) -> f64 {
    1.0
}

fn xtoy(x: Vec<f64>, y: Vec<f64>, n: i32) -> i32 {
    1
}

fn OutSubTreeN(fout: File, inode: i32, spnames: i32, printopt: i32, labelfmt: String) -> i32 {
    1
}

fn DiscreteGamma(freqK: Vec<f64>, rK: Vec<f64>, alpha: f64, beta: f64, K: i32, mean: i32) -> i32 {
    1
}

fn MultiNomialAlias(n: i32, ncat: i32, F: Vec<f64>, L: Vec<i32>, nobs: Vec<i32>) -> i32 {
    1
}

fn MultiNomialAliasSetTable(
    ncat: i32,
    prob: Vec<f64>,
    F: Vec<f64>,
    L: Vec<i32>,
    space: Vec<f64>,
) -> i32 {
    1
}

fn rndgamma(s: f64) -> f64 {
    1.0
}

fn ClearNode(inode: i32) {}

fn ReadaTreeB(ftree: File, popline: i32) -> i32 {
    1
}

fn IsNameNumber(line: String) -> i32 {
    1
}

fn print1seq(fout: File, z: String, ls: i32, encoded: i32, pose: Vec<i32>) -> i32 {
    1
}

fn factorial(n: i32) -> i64 {
    1
}

fn IncompleteGamma(x: f32, alpha: f32, ln_gamma_alpha: f32) -> f64 {
    1.0
}

fn matIout(fout: File, x: Vec<i32>, n: i32, m: i32) -> i32 {
    1
}

fn BranchToNode() {}

fn getcodon(codon: String, icodon: i32) -> String {
    "".to_string()
}

fn PointChi2(prob: f64, v: f64) -> f64 {
    1.0
}

fn PointNormal(prob: f64) -> f64 {
    1.0
}

const OLDAGE: i32 = 99;
const LASTROUND: i32 = 1;
const debug: i32 = 0;
const testlnL: i32 = 0;
const totalTimes: i32 = 0;
const totalSeqs: i32 = 0;
const NPMat: i32 = 0;
const BASEs: &'static str = "TCAGUYRMKSWHBVDN?-";
const AAs: &'static str = "ARNDCQEGHILKMFPSTWYV*-?x";
const BINs: &'static str = "TC";
