use hyper::{Client, Request};
use hyper::body::Body;
use tokio::runtime::Runtime;
use futures::Future;

const SOAP_ACTION_PREFIX: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#";
const ACTION_SET_URI: &'static str = "SetAVTransportURI";
const ACTION_PLAY: &'static str = "Play";
const ACTION_PAUSE: &'static str = "Pause";
const ACTION_STOP: &'static str = "Stop";

pub const BODY_SET_URI: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:SetAVTransportURI xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
            <CurrentURI>http://10.5.1.243:51497/0</CurrentURI>
            <CurrentURIMetaData>&lt;DIDL-Lite xmlns="urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:sec="http://www.sec.co.kr/" xmlns:upnp="urn:schemas-upnp-org:metadata-1-0/upnp/"&gt;&lt;item id="0" parentID="-1" restricted="false"&gt;&lt;upnp:class&gt;object.item.videoItem.movie&lt;/upnp:class&gt;&lt;dc:title&gt;dlnanow&lt;/dc:title&gt;&lt;res protocolInfo="http-get:*:video/mp4:*"&gt;http://10.5.1.243:51497/0&lt;/res&gt;&lt;/item&gt;&lt;/DIDL-Lite&gt;</CurrentURIMetaData>
        </u:SetAVTransportURI>
    </s:Body>
</s:Envelope>"#;

pub const BODY_PLAY: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:Play xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
            <Speed>1</Speed>
        </u:Play>
    </s:Body>
</s:Envelope>"#;

pub const BODY_PAUSE: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:Pause xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
        </u:Pause>
    </s:Body>
</s:Envelope>"#;

pub const BODY_STOP: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:Stop xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
        </u:Stop>
    </s:Body>
</s:Envelope>"#;


pub const A_SET: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#SetAVTransportURI";
pub const A_PLAY: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#Play";
pub const A_STOP: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#Stop";
pub const A_PAUSE: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#Pause";

fn build_request(action: &'static str, body: &'static str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("http://10.5.1.201:38400/serviceControl/AVTransport")
        .header("Content-Type", "text/xml")
        .header("SOAPACTION", action)
        .body(Body::from(body))
        .unwrap()
}

fn spawn_request(rt: &mut Runtime, req: Request<Body>) {
    let client = Client::new();
    let f = client
        .request(req)
        .map(|_res| {
            println!("Request good")
        })
        .map_err(|_err| {
            println!("Request error");
        });
    rt.spawn(f);
}

pub fn set_uri(rt: &mut Runtime) {
    let req = build_request(A_SET, BODY_SET_URI);
    spawn_request(rt, req);
}

pub fn play(rt: &mut Runtime) {
    let req = build_request(A_PLAY, BODY_PLAY);
    spawn_request(rt, req);
}

pub fn pause(rt: &mut Runtime) {
    let req = build_request(A_PAUSE, BODY_PAUSE);
    spawn_request(rt, req);
}

pub fn stop(rt: &mut Runtime) {
    let req = build_request(A_STOP, BODY_STOP);
    spawn_request(rt, req);
}
