use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Ressource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    version: Version,
    method: Method,
    ressource: Ressource,
    headers: HashMap<String, String>,
    msg_body: String,
}

/*
GET /greeting HTTP/1.1
HOST: localhost:3000
User-Agent: firefox
Accept: *//*

DATA
*/

impl From<String> for HttpRequest {
    fn from(s: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Ressource::Path("".to_string());
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut parsed_msg_body = "";

        for line in s.lines() {
            if line.contains("HTTP") {
                let (method, version, ressource) = process_request_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = ressource;
            } else if line.contains(':') {
                let (key, value) = process_headers_line(line);
                headers.insert(key, value);
            } else if line.is_empty() {
                continue;
            } else {
                parsed_msg_body = line;
            }
        }

        Self {
            version: parsed_version,
            method: parsed_method,
            ressource: parsed_resource,
            headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_request_line(line: &str) -> (Method, Version, Ressource) {
    let mut parts = line.split_whitespace();

    let method = parts.next().unwrap();

    let path = parts.next().unwrap();

    let version = parts.next().unwrap();

    (
        method.into(),
        version.into(),
        Ressource::Path(path.to_string()),
    )
}

fn process_headers_line(line: &str) -> (String, String) {
    let parts = line.split_once(':');
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some((k,v)) = parts {
        key = k.to_string(); 
        value = v.trim().to_string();
    };

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Method tests
    #[test]
    fn method_get() {
        let method: Method = "GET".into();
        assert_eq!(Method::Get, method);
    }

    #[test]
    fn method_post() {
        let method: Method = "POST".into();
        assert_eq!(Method::Post, method);
    }

    #[test]
    fn wrong_method() {
        let method: Method = "WRONG".into();
        assert_eq!(Method::Uninitialized, method);
    }

    // Version tests
    #[test]
    fn http_version_1_1() {
        let version: Version = "HTTP/1.1".into();
        assert_eq!(Version::V1_1, version)
    }

    #[test]
    fn http_bad_version() {
        let version: Version = "BAD".into();
        assert_eq!(Version::Uninitialized, version);
    }

    // HttpRequest test

    #[test]
    fn test_process_request_line() {
        let line = "GET /greeting HTTP/1.1";

        let (method, version, ressource) = process_request_line(line);

        assert_eq!(Method::Get, method);
        assert_eq!(Version::V1_1, version);
        assert_eq!(Ressource::Path("/greeting".to_string()), ressource);
    }

    #[test]
    fn test_process_headers_line() {
        let line = "Key:Value";

        let (key, value) = process_headers_line(line);
        assert_eq!(("Key", "Value"), (key.as_str(), value.as_str()));
    }

    #[test]
    fn test_http_request() {
        let request = r#"GET /greeting HTTP/1.1
HOST: localhost:3000
User-Agent: firefox
Accept: */*

DATA
"#;
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("User-Agent".to_string(), "firefox".to_string());
        headers.insert("HOST".to_string(), "localhost:3000".to_string());
        let request: HttpRequest = request.to_string().into();

        let expected = HttpRequest {
            version: Version::V1_1,
            method: Method::Get,
            ressource: Ressource::Path("/greeting".to_string()),
            headers,
            msg_body: "DATA".to_string(),
        };

        assert_eq!(expected.version, request.version);
        assert_eq!(expected.method, request.method);
        assert_eq!(expected.ressource, request.ressource);
        assert_eq!(expected.msg_body, request.msg_body);
        assert_eq!(expected.headers, request.headers);
    }
}






