use std::sync::Arc;

use divvunspell::{archive, speller};
use pyo3::prelude::*;

#[pymodule]
fn divvunspell(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SpellerArchive>()?;
    m.add_class::<Speller>()?;

    Ok(())
}

#[pyclass]
struct SpellerArchive {
    archive: Arc<dyn archive::SpellerArchive + Send + Sync>,
}

#[pymethods]
impl SpellerArchive {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        let ar = archive::open(std::path::Path::new(&path)).unwrap();
        Ok(SpellerArchive { archive: ar })
    }

    fn speller(&self) -> PyResult<Speller> {
        let speller = self.archive.speller();
        Ok(Speller { speller })
    }
}

#[pyclass]
struct Speller {
    speller: Arc<dyn speller::Speller + Send + Sync>,
}

#[pymethods]
impl Speller {
    fn suggest(&self, word: String) -> PyResult<Vec<(String, f32)>> {
        let speller = Arc::clone(&self.speller);
        let results = speller.suggest(&word);
        Ok(results
            .into_iter()
            .map(|x| (x.value.to_string(), x.weight))
            .collect::<Vec<_>>())
    }
}
