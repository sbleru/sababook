extern crate alloc;
use crate::http::alloc::string::ToString;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use noli::net::IpV4Addr;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        // localhostを127.0.0.1に変換
        let resolved_host = if host == "localhost" {
            "127.0.0.1".to_string()
        } else {
            host.clone()
        };

        // IPv4アドレスかどうかをチェック
        let socket_addr = if Self::is_ipv4_address(&resolved_host) {
            // IPv4アドレスの場合は直接SocketAddrを作成
            match Self::parse_ipv4(&resolved_host) {
                Ok(ip_bytes) => {
                    let ip = IpV4Addr::new(ip_bytes);
                    SocketAddr::from((ip, port))
                }
                Err(e) => {
                    return Err(Error::Network(format!("Invalid IPv4 address: {}", e)))
                }
            }
        } else {
            // ホスト名の場合はlookup_hostを使用
            let ips = match lookup_host(&resolved_host) {
                Ok(ips) => ips,
                Err(e) => {
                    return Err(Error::Network(format!(
                        "Failed to find IP addresses: {:#?}",
                        e
                    )))
                }
            };

            if ips.len() < 1 {
                return Err(Error::Network("Failed to find IP addresses".to_string()));
            }

            SocketAddr::from((ips[0], port))
        };

        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ))
            }
        };

        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        // ヘッダの追加
        request.push_str("Host: ");
        request.push_str(&host);
        request.push('\n');
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push('\n');

        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to send a request to TCP stream".to_string(),
                ))
            }
        };

        let mut received = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network(
                        "Failed to receive a request from TCP stream".to_string(),
                    ))
                }
            };
            if bytes_read == 0 {
                break;
            }
            received.extend_from_slice(&buf[..bytes_read]);
        }

        match core::str::from_utf8(&received) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(e) => Err(Error::Network(format!("Invalid received response: {}", e))),
        }
    }

    // IPv4アドレスかどうかをチェックする関数
    fn is_ipv4_address(addr: &str) -> bool {
        let parts: Vec<&str> = addr.split('.').collect();
        if parts.len() != 4 {
            return false;
        }
        
        for part in parts {
            if part.is_empty() {
                return false;
            }
            if let Ok(num) = part.parse::<u8>() {
                // 0-255の範囲内かチェック（u8なので自動的に範囲内）
                if part.len() > 1 && part.starts_with('0') {
                    return false; // 先頭0は無効（例：01, 001）
                }
            } else {
                return false;
            }
        }
        true
    }

    // IPv4アドレスをパースする関数
    fn parse_ipv4(addr: &str) -> Result<[u8; 4], String> {
        let parts: Vec<&str> = addr.split('.').collect();
        if parts.len() != 4 {
            return Err("IPv4 address must have 4 parts".to_string());
        }
        
        let mut bytes = [0u8; 4];
        for (i, part) in parts.iter().enumerate() {
            match part.parse::<u8>() {
                Ok(num) => bytes[i] = num,
                Err(_) => return Err(format!("Invalid number in IPv4 address: {}", part)),
            }
        }
        Ok(bytes)
    }
}
