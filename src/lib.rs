use pyo3::prelude::*;
use regex::Regex;

mod schema;

#[pyfunction]
fn get_djs(html: &str) -> PyResult<String> {
    let re = Regex::new(r"(?m)'(\/d\.js\?.+)'");

    if let Ok(re) = re {
        match re.captures(html) {
            Some(m) => Ok(m.get(1).unwrap().as_str().into()),
            None => Err(pyo3::exceptions::PyRuntimeError::new_err(
                "failed to get djs from html",
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
fn get_result_binding(content: String) -> PyResult<schema::Result> {
    let page_layout = get_page_layout(content.to_owned());

    let images = {
        let images = get_images(content.to_owned());
        images.unwrap_or("".to_string())
    };

    let news = {
        let news = get_news(content.to_owned());
        news.unwrap_or("".to_string())
    };

    Ok(schema::Result::new(page_layout, images, news))
}

#[pymodule]
fn ddginternal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_djs, m)?)?;
    m.add_function(wrap_pyfunction!(get_result_binding, m)?)?;
    m.add_class::<schema::Result>()?;
    Ok(())
}
