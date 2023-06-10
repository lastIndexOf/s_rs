// use tracing

use tracing::{field::Visit, Subscriber};
use tracing_subscriber::Layer;

#[derive(Default)]
pub struct MyLayer {}

struct MyVisitor<'a>(&'a mut std::collections::BTreeMap<String, serde_json::Value>);

impl<S> Layer<S> for MyLayer
where
    S: Subscriber,
{
    fn on_event(
        &self,
        _event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut fields = std::collections::BTreeMap::new();

        let mut visitor = MyVisitor(&mut fields);
        _event.record(&mut visitor);

        let output = serde_json::json!({
            "target": _event.metadata().target(),
            "name": _event.metadata().name(),
            "level": format!("{:?}", _event.metadata().level()),
            "fields": fields,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}

impl<'a> Visit for MyVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
}
