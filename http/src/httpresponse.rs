use std::io::Result;
use std::{collections::HashMap, io::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        HttpResponse {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_code = status_code;
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;

        response
    }

    pub fn version(&self) -> &str {
        self.version
    }

    pub fn status_code(&self) -> &str {
        self.status_code
    }

    pub fn status_text(&self) -> &str {
        self.status_text
    }

    pub fn headers(&self) -> String {
        let mut headers_string: String = "".into();
        match &self.headers {
            Some(h) => {
                for (k, v) in h.iter() {
                    headers_string = format!("{}{}:{}\r\n", headers_string, k, v);
                }
                headers_string
            }
            None => "".into(),
        }
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(v) => v.as_str(),
            None => "",
        }
    }
}
