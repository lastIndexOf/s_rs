use color_eyre::eyre::eyre;
use tracing::{debug, error, event, info, span, trace, Level};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use self::layer::MyLayer;

mod layer;

pub fn init_tracing() {
    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(MyLayer::default())
        .init();

    let _ = color_eyre::install();
}

pub fn hello_tracing() {
    let msg = "This is some message";
    info!(message2 = msg, "Tracing Info Message");
    debug!(message2 = msg, "Tracing Debug Message");
    error!(message3 = msg, "Tracing Error Message");

    log::info!("this is log.info in tracing");

    let _my_span = span!(Level::TRACE, "My Span");
    let _guard = _my_span.enter();
    trace!("Enter My Span");
    trace!("In My Span");

    event!(Level::INFO, "this is a info event in my span");
    drop(_guard);

    trace!("Out My Span");

    my_other_span();
}

#[tracing::instrument]
fn my_other_span() {
    error!("In My Other Span");

    let err = eyre!("Error");
    error!(?err, "In My Other Span Print Some eyre Error");
}
