use sce::SingleCellExperiment;
use std::fmt;
use rand::Rng;
use std::error::Error;
use std::path::PathBuf;

pub struct MultiModalExperiment<T> {
    assays: Vec<SingleCellExperiment<T>>,
    _pivot: usize,
}

impl<T> fmt::Debug for MultiModalExperiment<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MultiModalExperiment: {} modalitles\n", self.len())?;
        for (index, exp) in self.assays().into_iter().enumerate() {
            write!(f, "Modality {} Shape: {:?}\n", index, exp.shape())?;
        }

        Ok(())
    }
}

impl<T> MultiModalExperiment<T> {
    pub fn get_experiment(&self, index: usize) -> Option<&SingleCellExperiment<T>> {
        self.assays.get(index)
    }

    pub fn len(&self) -> usize {
        self.assays.len()
    }

    pub fn assays(&self) -> &Vec<SingleCellExperiment<T>> {
        &self.assays
    }

    pub fn _pivot(&self) -> usize {
        self._pivot
    }

    pub fn num_cells(&self) -> usize {
        self.get_experiment(0).unwrap().rows()
    }

    pub fn features(&self) -> Vec<&Vec<String>> {
        let mut features = Vec::new();
        for assay in &self.assays {
            features.push(assay.col_names());
        }

        features
    }
}

impl MultiModalExperiment<f32> {
    pub fn from_paths(paths: Vec<PathBuf>) -> MultiModalExperiment<f32> {
        let mut assays: Vec<sce::SingleCellExperiment<f32>> = Vec::new();
        for path in paths {
            let experiment = sce::SingleCellExperiment::from_tenx_v2(path)
                .expect("error reading the input matrix");

            info!("{:?}", experiment);
            assays.push(experiment);
        }

        MultiModalExperiment {
            assays: assays,
            _pivot: 0,
        }
    }

    pub fn choose_feature(
        &self, 
        features: &Vec<usize>, 
        coin_val: f32, 
        cell_id: usize,
        is_pivot_assay: bool
    ) -> Result<usize, Box<dyn Error>> {
        if features.len() == 1 {
            return Ok(0)
        }

        assert!(coin_val < 1.0 && coin_val >= 0.0, "wrong coin toss value");
        let mat = match is_pivot_assay {
            true => self.get_experiment(1).unwrap().counts(),
            false => self.get_experiment(0).unwrap().counts(),
        };

        let mut stats: Vec<f32> = features.iter().map(|&feature| {
            *mat.get(cell_id, feature).unwrap_or(&0.0)
        }).collect();

        let norm: f32 = stats.iter().sum();
        if norm == 0.0 {
            let mut rng = rand::thread_rng();
            return Ok(rng.gen_range(0, features.len()));
        }

        let mut cum_sum_iter = stats.iter_mut().scan(0.0_f32, |cusum, x| {
            *cusum = *cusum + (*x / norm);
            Some(*cusum)
        });

        let chosen_index = cum_sum_iter.position(|x| x > coin_val).unwrap();
        Ok(chosen_index)
    }
}
