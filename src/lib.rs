use std::sync::Arc;

use divvunspell::{archive, speller};
use pyo3::prelude::*;

/// Use spell checker archives compatible with the [divvunspell] project.
///
/// divvunspell is intended to be a Rust implementation of [hfst-ospell],
/// so `.zhfst` speller archives should work.
///
/// To get started, open a speller archive:
///
///     from divvunspell import SpellerArchive
///
///     archive = SpellerArchive("path/to/my-spellers.zhfst")
///     speller = archive.speller()
///
/// Then use the speller to suggest corrections!
///
///     suggestions = speller.suggest("teh")
///     # should give you something like:
///     # [('the', 1.0), ('ten', 4.0)]
///
/// [divvunspell]: https://github.com/divvun/divvunspell
/// [hfst-ospell]: https://github.com/hfst/hfst-ospell
#[pymodule]
fn divvunspell(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SpellerArchive>()?;
    m.add_class::<Speller>()?;

    Ok(())
}

/// Opens a speller archive, such as a .zhfst file
///
/// Usage:
///
/// >>> ar = SpellerArchive("path/to/my-speller.zhfst")
/// >>> speller = ar.speller()
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

    /// Returns a Speller instance. You probably want this.
    fn speller(&self) -> PyResult<Speller> {
        let speller = self.archive.speller();
        Ok(Speller { speller })
    }
}

/// Speller that can suggest spelling corrections.
///
/// **NOTE**: You must use `SpellerArchive.speller()` to get a Speller instance!
///
/// Usage:
///
/// >>> speller.suggest("teh")
/// [('the', 1.0,), ('ten', 1.5), ('tea', 1.75), ('tee', 2.0)]
#[pyclass]
struct Speller {
    speller: Arc<dyn speller::Speller + Send + Sync>,
}

#[pymethods]
impl Speller {
    /// Given a possibly misspelled word, returns a list spelling suggestions.
    /// Each suggestion is a tuple of (correction, weight).
    ///
    /// The correction is a string that could replace the given word. Note
    /// that some spellers may suggest the input (no change). The weight is a
    /// floating point number from 0 to infinity. A smaller value means a
    /// more likely suggestion. A bigger value is a less-likely suggestion.
    fn suggest(&self, word: String) -> PyResult<Vec<(String, f32)>> {
        let speller = Arc::clone(&self.speller);
        let results = speller.suggest(&word);
        Ok(results
            .into_iter()
            .map(|x| (x.value.to_string(), x.weight))
            .collect::<Vec<_>>())
    }
}
