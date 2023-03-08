use quinn::EndpointConfig;

extern crate quinn;

fn server() {
    quinn::Endpoint::new(EndpointConfig::, server_config, socket, runtime)
}
