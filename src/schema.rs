use std::fmt::Debug;

use pyo3::prelude::*;

use serde::Deserialize;

use crate::abstract_text;

#[derive(Deserialize, Clone)]
#[pyclass]
pub struct Web {
    a: String,
    d: String,
    i: String,
    t: String,
    u: String,
}

#[pymethods]
impl Web {
    #[getter]
    fn raw_description(&self) -> String {
        self.a.to_owned()
    }

    #[getter]
    fn description(&self) -> String {
        html2text::from_read(self.a.as_bytes(), usize::MAX)
    }

    #[getter]
    fn domain(&self) -> String {
        self.i.to_owned()
    }

    #[getter]
    fn shortened_url(&self) -> String {
        self.d.to_owned()
    }

    #[getter]
    fn url(&self) -> String {
        self.u.to_owned()
    }

    #[getter]
    fn title(&self) -> String {
        html2text::from_read(self.t.as_bytes(), usize::MAX)
    }

    fn __repr__(&self) -> String {
        format!(
            "Web(raw_description={:?}, description={:?}, domain={:?}, shortened_url={:?}, url={:?}, title={:?})",
            self.raw_description(),
            self.description(),
            self.domain(),
            self.shortened_url(),
            self.url(),
            self.title()
        )
    }
}

#[derive(Deserialize, Clone)]
#[pyclass]
pub struct Image {
    #[pyo3(get)]
    height: u32,

    #[pyo3(get)]
    width: u32,

    #[pyo3(get)]
    image: String,

    #[pyo3(get)]
    thumbnail: String,

    #[pyo3(get)]
    title: String,

    #[pyo3(get)]
    url: String,
}

#[pymethods]
impl Image {
    fn __repr__(&self) -> String {
        format!(
            "Image(height={}, width={}, image={:?}, thumbnail={:?}, title={:?}, url={:?})",
            self.height, self.width, self.image, self.thumbnail, self.title, self.url
        )
    }
}

#[derive(Deserialize)]
pub struct ImageResults {
    results: Vec<Image>,
}

#[derive(Deserialize, Clone, Debug)]
#[pyclass]
pub struct NewsArticle {
    #[pyo3(get)]
    date: u64,

    #[pyo3(get)]
    excerpt: String,

    #[pyo3(get)]
    image: Option<String>,

    #[pyo3(get)]
    relative_time: String,

    #[pyo3(get)]
    source: String,

    #[pyo3(get)]
    title: String,

    #[pyo3(get)]
    url: String,
}

#[pymethods]
impl NewsArticle {
    fn __repr__(&self) -> String {
        format!(
            "NewsArticle(date={:?}, excerpt={:?}, image={:?}, relative_time={:?}, source={:?}, title={:?}, url={:?})",
            self.date, 
            self.excerpt, 
            self.image.to_owned().unwrap_or("None".to_string()),
            self.relative_time,
            self.source,
            self.title,
            self.url
        )
    }
}

#[derive(Deserialize)]
pub struct NewsResults {
    results: Vec<NewsArticle>,
}

#[pyclass]
pub struct Result {
    web_results: Vec<Web>,
    images: Vec<Image>,
    news_articles: Vec<NewsArticle>,
    #[pyo3(get, name = "abstract")]
    abstract_info: Option<abstract_text::Abstract>
}

#[pymethods]
impl Result {
    #[new]
    pub fn new(page_layout: String, images: String, news: String, abstracts: String) -> Self {
        let web_results = serde_json::from_str::<Vec<Web>>(&page_layout).unwrap();

        let images: Vec<Image> = {
            if images.is_empty() {
                vec![]
            } else {
                serde_json::from_str::<ImageResults>(&images)
                    .unwrap_or(ImageResults { results: Vec::new() })
                    .results
            }
        };

        let news_articles = {
            if images.is_empty() {
                vec![]
            } else {
                serde_json::from_str::<NewsResults>(&news)
                    .unwrap_or(NewsResults { results: Vec::new() })
                    .results
            }
        };

        let abstract_info = abstract_text::get_abstract(abstracts).ok();

        Self {
            web_results,
            images,
            news_articles,
            abstract_info
        }
    }

    #[getter]
    fn web(&self) -> Vec<Web> {
        self.web_results.to_owned()
    }

    #[getter]
    fn images(&self) -> Vec<Image> {
        self.images.to_owned()
    }

    #[getter]
    fn news(&self) -> Vec<NewsArticle> {
        self.news_articles.to_owned()
    }

    fn __repr__(&self) -> String {
        format!(
            "Result(web=[...{}], images=[...{}], news=[...{}], abstract={})",
            self.web().len(),
            self.images().len(),
            self.news().len(),
            {
                if let Some(_) = self.abstract_info {
                    "Abstract(...)"
                } else {
                    "None"
                }
            }
        )
    }
}
