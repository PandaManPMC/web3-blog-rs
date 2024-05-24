use std::net::SocketAddr;
use axum::extract::{ConnectInfo, Request};
use log::trace;

pub fn get_request_ip(request: &Request) -> String {

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
        return xip[0..index].to_string();
    }

    return xip;
}
