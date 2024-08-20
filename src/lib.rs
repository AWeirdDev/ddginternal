use pyo3::prelude::*;
use regex::Regex;

mod abstract_text;
mod exceptions;
mod schema;

// extension modules (through nrj(...) in d.js)
mod module_base;
mod module_places;
mod module_recipes;
mod modules;

#[pyfunction]
fn get_djs(html: &str) -> PyResult<String> {
    let re = Regex::new(r"(?m)'(\/d\.js\?.+)'");

    if let Ok(re) = re {
        match re.captures(html) {
            Some(m) => Ok(m.get(1).unwrap().as_str().into()),
            None => Err(pyo3::exceptions::PyIndexError::new_err(
                "failed to get d.js from html",
            )),
        }
    } else {
        Err(exceptions::RegexError::new_err("failed to compile regex"))
    }
}

#[pyfunction]
fn get_embedded_abstract(html: &str) -> PyResult<String> {
    let re = Regex::new(r"DDG\.duckbar\.add\((.+?),null,.index.\);");

    if let Ok(re) = re {
        match re.captures(html) {
            Some(m) => Ok(m.get(1).unwrap().as_str().into()),
            None => Err(exceptions::RegexError::new_err(
                "failed to get embedded abstract from html",
            )),
        }
    } else {
        Err(pyo3::exceptions::PyRuntimeError::new_err(
            "failed to compile regex",
        ))
    }
}

fn get_page_layout(content: String) -> String {
    let page_layout = content
        .split_once(";if (DDG.pageLayout)")
        .unwrap()
        .1
        .split_once(";DDG.duckbar.load")
        .unwrap()
        .0
        .split_once("DDG.pageLayout.load('d',")
        .unwrap()
        .1
        .split_once(",{\"n\":")
        .unwrap()
        .0;

    page_layout.to_string() + "]"
}

fn get_images(content: String) -> Result<String, String> {
    let images = content
        .split_once("DDG.duckbar.load('images',")
        .unwrap_or(("", ""))
        .1
        .split_once("});")
        .unwrap_or(("", ""))
        .0;

    if images.is_empty() {
        Err("failed to get images (quiet possibly, no images found)".to_string())
    } else {
        Ok(images.to_string() + "}")
    }
}

fn get_news(content: String) -> Result<String, String> {
    let news = content
        .split_once("DDG.duckbar.load('news',")
        .unwrap_or(("", ""))
        .1
        .split_once("});")
        .unwrap_or(("", ""))
        .0;

    if news.is_empty() {
        Err("failed to get news (quiet possibly, no news found)".to_string())
    } else {
        Ok(news.to_string() + "}")
    }
}

#[pyfunction]
fn get_result_binding(html: String, djs: String) -> PyResult<schema::Result> {
    let page_layout = get_page_layout(djs.to_owned());

    let images = {
        let images = get_images(djs.to_owned());
        images.unwrap_or("".to_string())
    };

    let news = {
        let news = get_news(djs.to_owned());
        news.unwrap_or("".to_string())
    };

    let abstracts = {
        let em = get_embedded_abstract(html.as_str());
        em.unwrap_or("".to_string())
    };

    Ok(schema::Result::new(page_layout, images, news, abstracts))
}

#[pymodule]
fn ddginternal(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_djs, m)?)?;
    m.add_function(wrap_pyfunction!(get_embedded_abstract, m)?)?;
    m.add_function(wrap_pyfunction!(get_result_binding, m)?)?;
    m.add_function(wrap_pyfunction!(abstract_text::get_abstract, m)?)?;
    m.add_function(wrap_pyfunction!(modules::get_nrj_instances, m)?)?;
    m.add_function(wrap_pyfunction!(modules::assign_nrj_instances, m)?)?;

    m.add_class::<schema::Result>()?;
    m.add_class::<module_places::PlacesModule>()?;
    m.add_class::<module_recipes::RecipesModule>()?;
    m.add("RegexError", py.get_type_bound::<exceptions::RegexError>())?;
    Ok(())
}
