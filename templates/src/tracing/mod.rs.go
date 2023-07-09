use opentelemetry::global;

pub fn init_jaeger_tracer(service_name: &str) {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to initialize Jaeger Tracer");
}

pub fn shutdown_tracer_provider() {
    opentelemetry::global::shutdown_tracer_provider();
}