use std::net::SocketAddr;
use axum::extract::{ConnectInfo, Request};
use axum::http::HeaderMap;
use log::trace;

pub mod rsp;

pub fn get_request_ip(request: &mut Request) -> String {
    request.headers_mut().insert("x_client_real_ip", "".to_string().parse().unwrap());

    fn get_ip  (request: &Request) -> String {
        let head = request.headers().clone();

        let remote_addr = head.get("X-Forwarded-For");
        trace!("X-Forwarded-For={:?}", remote_addr);
        if let Some(x) = remote_addr {
            return x.to_str().unwrap().to_string();
        }

        let remote_addr = head.get("x-forwarded-for");
        trace!("x-forwarded-for={:?}", remote_addr);
        if let Some(x) = remote_addr {
            return x.to_str().unwrap().to_string();
        }

        let remote_addr = head.get("X-Real-IP");
        trace!("X-Real-IP={:?}", remote_addr);
        if let Some(x) = remote_addr {
            return x.to_str().unwrap().to_string();
        }

        let remote_addr = head.get("Remote_addr");
        trace!("remote_addr={:?}", remote_addr);
        if let Some(x) = remote_addr {
            return x.to_str().unwrap().to_string();
        }

        let addr = request.extensions().get::<ConnectInfo<SocketAddr>>().map(|ci| ci.0);
        trace!("Client IP: {:?}", addr);

        if let Some(addr) = addr {
            return addr.to_string();
        }

        return "".to_string();
    }

    let xip = get_ip(request);
    if let Some(index) = xip.find(":") {
        let ip = xip[0..index].to_string();
        request.headers_mut().insert("x_client_real_ip", ip.parse().unwrap());
        return ip;
    }

    request.headers_mut().insert("x_client_real_ip", xip.parse().unwrap());
    return xip;
}

pub fn get_client_real_ip(headers: &HeaderMap) -> String {
    let real_ip = headers.get("x_client_real_ip");
    if let Some(x) = real_ip {
        return x.to_str().unwrap().to_string();
    }
    return "".to_string();
}
