use regex::Regex;

/*
    Uri parsing regex:
    ^                                       // start of string anchor
    (([\w\d\+\-.])+:)?                      // parses optional scheme

    (//                                     // open authority (optional)
        ((([\w\d\+\._~!$&'()*+,;=:]*))@)?   // user info (optional)
        (([\w\d\+\._~!$&'()*+,;=]|:{2})*)   // host (non optional)
        (:(\d*))?                           // port (optional)
    /)?                                     // close authority (optional)

    (/?[\w\d\+\._~!$&'()*+,;=]+)?           // path

    (\?[\w\d\+\._~!$&'()*+,;=]+)?           // query

    (#[\w\d\+\._~!$&'()*+,;=]+)?            // fragment

    scheme: group 1
    userinfo: group 2
    host: group 3
    port: group 4
    path: group 5
    query: group 6
    fragment: group 7


    progress so far: (next up: path) (note that path includes optional '/')
    ^(?:([\w\d\+\-\.]+):)?(?://(?:([\w\d\+\._~!$&'()*+,;=:]*)@)?((?:[\w\d\+\._\-~!$&'()*+,;=]|:{2})*)(?::(\d*))?)?(/?[\w\d\+\._~!$&'()*+,;=]+)?(?:\?([\w\d\+\._~!$&'()*+,;=]+))?(?:#([\w\d\+\._~!$&'()*+,;=]*))?$
*/

/*
unreserved: ALPHA / DIGIT / - / . / _ / ~
    [\w\d\+\._~]

gen-delims: ":" / "/" / "?" / "#" / "[" / "]" / "@"
    [:/\?\#\[\]@]

sub-delims: "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
    [!$&'()*+,;=]
*/

///             userinfo                 port
///       |---------------|             |---|
/// abc://username:password@example.com:12345/path/data?key=value&key2=value2#fragid1
/// |-|   |-------------------------------||--------| |-------------------| |-----|
///  |                  |                       |               |              |
/// scheme          authority                 path            query         fragment

static URI_REGEX: &str = r"^(?:([\w\d\+\-\.]+):)?(?://(?:([\w\d\+\._~!$&'()*+,;=:]*)@)?((?:[\w\d\+\._\-~!$&'()*+,;=]|:{2})*)(?::(\d*))?)?(/?[\w\d\+\._~!$&'()*+,;=]+)?(?:\?([\w\d\+\._~!$&'()*+,;=]+))?(?:#([\w\d\+\._~!$&'()*+,;=]*))?$";

pub struct Uri {
    scheme: String,
    userinfo: String,
    host: String,
    port: u16,
    path: String,
    query: String,
    fragment: String
}

#[derive(Debug)]
pub enum UriParseError {
    NotAnUri,
    InvalidPort
}

#[allow(dead_code)]
impl Uri {

    // constructors

    pub fn new() -> Self {
        Uri {
            scheme: String::new(),
            userinfo: String::new(),
            host: String::new(),
            port: 0,
            path: String::new(),
            query: String::new(),
            fragment: String::new()
        }
    }

    pub fn from(str: &str) -> Result<Self, UriParseError> {
        let mut uri = Uri::new();
        let regex = Regex::new(URI_REGEX).expect("Syntax error in uri parsing regex");
        
        let Some(captures) = regex.captures(str) else {
            return Err(UriParseError::NotAnUri);
        };

        uri.scheme = String::from(captures.get(1).map_or("", |m| m.as_str()));
        uri.userinfo = String::from(captures.get(2).map_or("", |m| m.as_str()));
        uri.host = String::from(captures.get(3).map_or("", |m| m.as_str()));
        uri.path = String::from(captures.get(5).map_or("", |m| m.as_str()));
        uri.query = String::from(captures.get(6).map_or("", |m| m.as_str()));
        uri.fragment = String::from(captures.get(7).map_or("", |m| m.as_str()));

        let Ok(port) = captures.get(4).map_or("0", |m| m.as_str()).parse::<u16>() else {
            return Err(UriParseError::InvalidPort)
        };
        
        uri.port = port;

        Ok(uri)
    }
    
    // public

    pub fn clear(&mut self) {
        *self = Uri::new();
    }

    pub fn is_valid_url(&self) -> bool {
        !self.host.is_empty()
    }

    /// Will return the port if its hard set, or else try to return the default scheme port if known.
    pub fn determine_port(&self) -> Option<u16> {
        if self.port == 0 {
            return self.scheme_default_port();
        }
        Some(self.port)
    }

    pub fn scheme_default_port(&self) -> Option<u16> {
        match self.scheme.to_lowercase().as_str() {
            "dns"       => Some(53),
            "ftp"       => Some(21),
            "git"       => Some(9418),
            "http"      => Some(80),
            "https"     => Some(443),
            "irc"       => Some(194),
            "sftp"      => Some(22),
            "ssh"       => Some(22),
            "telnet"    => Some(23),
            "rsync"     => Some(873),
            "ws"        => Some(80),
            "wss"       => Some(443),
            _ => None
        }
    }

    pub fn as_string(&self) -> String {
        todo!();
    }

    pub fn debug_print(&self) {
        println!("Scheme: {}", self.scheme);
        println!("Userinfo: {}", self.userinfo);
        println!("Host: {}", self.host);
        println!("Port: {} as u16", self.port);
        println!("Path: {}", self.path);
        println!("Query: {}", self.query);
        println!("Fragment: {}", self.fragment);
    }

}