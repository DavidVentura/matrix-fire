use esp_idf_svc::http::server::EspHttpServer;
//use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use esp_idf_svc::http::Method;
use esp_idf_svc::io::EspIOError;

pub(crate) fn server(
    f: impl Fn(u8) + Send + Sync + 'static,
) -> Result<EspHttpServer<'static>, EspIOError> {
    // HTTP Configuration
    // Create HTTP Server Connection Handle
    //let mut httpserver = EspHttpServer::new(&HttpServerConfig::default())?;
    let mut httpserver = EspHttpServer::new(&Default::default())?;

    // Define Server Request Handler Behaviour on Get for Root URL
    httpserver.fn_handler("/", Method::Get, move |request| {
        // Retrieve html String
        let html = index_html();
        // Respond with OK status
        let mut response = request.into_ok_response()?;
        // Return Requested Object (Index Page)
        response.write(html.as_bytes())?;
        f(100);
        Ok(())
    })?;

    Ok(httpserver)
}

fn index_html() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
    Hello World from ESP!
    </body>
</html>
"#
    )
}
