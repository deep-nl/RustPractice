use tracing::{Event, Subscriber, span};
use tracing::field::{Field, Visit};

struct MyVisitor;

impl Visit for MyVisitor {
    fn record_i64(&mut self, field: &Field, value: i64) {
        println!("{}: {}", field.name(), value);
    }
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        println!("field: {:?}, value: {:?}", field.name(), value);
    }
}

struct MySubscriber;

impl Subscriber for MySubscriber {
    fn new_span(&self, span: &span::Attributes) -> span::Id {
        span::Id::from_u64(0)
    }

    fn event(&self, event: &Event) {
        let mut visitor = MyVisitor;
        event.record(&mut visitor);
    }

    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }
}

fn event() {
    let subscriber = MySubscriber;
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global default subscriber");

    tracing::event!(
        tracing::Level::INFO,
        value1 = 10i64,
        value2 = "Hello, World!",
        value3 = true
    );
}
#[test]
fn test_event(){
    event();
}
