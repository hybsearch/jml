#[macro_use]
extern crate log;
use std::option::Option;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct NexusData {
    // number of taxa in the dataset
    n_taxa: usize,
    // The number of characters
    n_chars: usize,
    // A vector containning the taxa's name
    taxa: Vec<String>,
    // A flag to show if taxa labels are read: 0=not read; 1=read.
    taxa_read: bool,
    // 0 = no diagonal, 1 = diagonal
    diagonal: bool,
    // Flag to indicate is labels are in the distance matrix
    labels: bool,
    // Flag to indicate if matrix is interleaved
    interleave: bool,
    // Indicate the dataype
    datatype: String,
    // Character indicating the symbol for missing characters
    missingchar: String,
    // Character for gaps
    gapchar: String,
    // 0=Lower, 1=Upper, 2=both
    triangle: i8,
    // The distance matrix
    dist_matrix: Vec<Vec<f64>>,
    // The character matrix
    char_matrix: Vec<String>,
}

impl NexusData {
    fn init() -> NexusData {
        NexusData {
            n_taxa: 0,
            n_chars: 0,
            taxa: vec![],
            taxa_read: false,
            diagonal: true,
            labels: true,
            interleave: false,
            triangle: 0,
            datatype: "".to_string(),
            gapchar: "".to_string(),
            missingchar: "".to_string(),
            dist_matrix: vec![vec![]],
            char_matrix: vec![],
        }
    }

    fn init_data_matrix(&mut self) {
        self.dist_matrix = vec![vec![0.0; self.n_taxa]; self.n_taxa];
    }

    fn init_char_matrix(&mut self) {
        self.char_matrix = vec!["".to_string(); self.n_taxa];
    }

    fn return_dist(&self, i: usize, j: usize) -> f64 {
        debug!("nex_data: {},{};{}", i, j, self.dist_matrix[i][j]);
        self.dist_matrix[i][j]
    }

    fn return_sequence(&self, i: usize) -> String {
        self.char_matrix[i].clone()
    }

    fn return_char(&self, i: usize, j: usize) -> char {
        self.char_matrix[i].chars().nth(j).unwrap()
    }

    fn return_character_for_taxa(&self, i: usize) -> usize {
        self.char_matrix[i].len()
    }

    fn add_characters(&mut self, i: usize, chars: String) {
        self.char_matrix[i] += &chars
    }

    fn compute_distances(&mut self) {
        for i in 0..self.n_taxa {
            for j in 0..(i + 1) {
                let mut total_distance: usize = 0;

                //#pragma omp parallel for schedule(dynamic)
                let i_chars: Vec<char> = self.char_matrix[i].chars().collect();
                let j_chars: Vec<char> = self.char_matrix[j].chars().collect();

                for k in 0..self.n_chars {
                    if (i_chars[k] == '-') || (j_chars[k] == '-') {
                        continue; // ignore gaps
                    }
                    if i_chars[k] != j_chars[k] {
                        total_distance += 1;
                    }
                }

                let final_distance: f64 = (total_distance as f64) / (self.n_chars as f64);
                self.dist_matrix[i][j] = final_distance;
                self.dist_matrix[j][i] = final_distance;
                debug!("{} {} -> Dist: {}", i, j, final_distance);
            }
        }
    }

    fn get_position_of_taxa(&self, taxa: String) -> Option<usize> {
        self.taxa.iter().position(|ref t| **t == taxa)
    }
}
