use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::http::Method;
use esp_idf_svc::io::EspIOError;
use serde::Deserialize;
use std::sync::mpsc::Sender;

pub(crate) enum Commands {
    Brightness(u8),
}

#[derive(Deserialize, Debug)]
struct Settings {
    value: u8,
}

const MAX_LEN: u8 = 200;

pub(crate) fn server(tx: Sender<Commands>) -> Result<EspHttpServer<'static>, EspIOError> {
    let mut httpserver = EspHttpServer::new(&Default::default())?;

    httpserver.fn_handler("/", Method::Post, move |mut req| {
        // Can't get `req.content_len()` to work, the Headers trait doesnt seem to work
        let mut buf: Vec<u8> = vec![0; MAX_LEN as usize];
        req.read(&mut buf)?;
        let mut resp = req.into_ok_response()?;

        let str_repr = std::str::from_utf8(&buf)?.trim_end_matches(char::from(0));
        match serde_json::from_str::<Settings>(str_repr) {
            Ok(s) => {
                resp.write("".as_bytes())?;
                tx.send(Commands::Brightness(s.value)).unwrap();
            }
            Err(e) => {
                resp.write(format!("{:?}", e).as_bytes())?;
            }
        };
        Ok(())
    })?;

    Ok(httpserver)
}
