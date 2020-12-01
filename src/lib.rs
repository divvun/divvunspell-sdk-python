use std::sync::Arc;

use cpython::{PyResult, py_class, py_module_initializer};
use divvunspell::{archive, speller};

py_module_initializer!(divvunspell, initdivvunspell, PyInit_divvunspell, |py, m| {
    m.add_class::<SpellerArchive>(py)?;
    m.add_class::<Speller>(py)?;
    Ok(())
});

py_class!(class SpellerArchive |py| {
    data archive: Arc<dyn archive::SpellerArchive + Send + Sync>;

    def __new__(_cls, path: String) -> PyResult<Self> {
        let ar = archive::open(std::path::Path::new(&path)).unwrap();
        Self::create_instance(py, ar)
    }

    def speller(&self) -> PyResult<Speller> {
        let speller = self.archive(py).speller();
        Speller::create_instance(py, speller)
    }
});

py_class!(class Speller |py| {
    data speller: Arc<dyn speller::Speller + Send + Sync>;

    def suggest(&self, word: String) -> PyResult<Vec<(String, f32)>> {
        let speller = Arc::clone(&self.speller(py));
        let results = speller.suggest(&word);
        Ok(results.into_iter().map(|x| (x.value.to_string(), x.weight)).collect::<Vec<_>>())
    }
});
