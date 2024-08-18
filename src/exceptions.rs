use pyo3::create_exception;

create_exception!(module, RegexError, pyo3::exceptions::PyException);
