use url::Url;
#[derive(Debug)]
pub struct Repository {
    name: String,
    description: String,
    url: Url,
    homepage: Option<Url>,
    branch: String,
    repo: Url,
    gpgkey: String,
}

impl Repository {}
