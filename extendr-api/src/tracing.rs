//! Helpers for forwarding `tracing` output to R's console.
//!
//! Call [`init_tracing_to_r`] once when your package is loaded (for example,
//! inside `.onLoad()` on the R side) to install a global `tracing` subscriber
//! that prints events via R's output/error streams. This means messages respect
//! redirection such as `sink()` and appear in the same place as regular R
//! output.

use crate::{print_r_error, print_r_output};
use once_cell::sync::OnceCell;
use std::fmt::{self, Write as FmtWrite};
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::field::{Field, Visit};
use tracing::level_filters::LevelFilter;
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

pub use tracing::level_filters::LevelFilter;

/// Try to install a global subscriber that prints tracing events via R.
///
/// - The maximum log level is taken from the first level in `RUST_LOG` (e.g.
///   `RUST_LOG=debug`), or defaults to [`LevelFilter::INFO`] if none is set.
/// - The subscriber is only installed once; repeated calls are no-ops.
/// - WARN and ERROR events go to R's error stream (`REprintf`), other levels go
///   to R's regular output (`Rprintf`).
pub fn init_tracing_to_r() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let level = level_filter_from_env().unwrap_or(LevelFilter::INFO);
    init_tracing_to_r_with_filter(level)
}

/// Install a global subscriber that prints tracing events via R using the
/// provided maximum level.
///
/// See [`init_tracing_to_r`] for details on how events are routed.
pub fn init_tracing_to_r_with_filter(
    level: LevelFilter,
) -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    static SUBSCRIBER_SET: OnceCell<()> = OnceCell::new();
    SUBSCRIBER_SET
        .get_or_try_init(|| {
            tracing::subscriber::set_global_default(RConsoleSubscriber::new(level))?;
            Ok(())
        })
        .map(|_| ())
}

fn level_filter_from_env() -> Option<LevelFilter> {
    let raw = std::env::var("RUST_LOG").ok()?;
    raw.split(',')
        .find_map(|segment| parse_level_segment(segment.trim()))
}

fn parse_level_segment(segment: &str) -> Option<LevelFilter> {
    if segment.is_empty() {
        return None;
    }

    if let Some((_, level)) = segment.split_once('=') {
        level.trim().parse().ok()
    } else {
        segment.parse().ok()
    }
}

#[derive(Debug)]
struct RConsoleSubscriber {
    next_id: AtomicU64,
    max_level: LevelFilter,
}

impl RConsoleSubscriber {
    fn new(max_level: LevelFilter) -> Self {
        Self {
            next_id: AtomicU64::new(1),
            max_level,
        }
    }

    fn level_enabled(&self, level: &tracing::Level) -> bool {
        match self.max_level {
            LevelFilter::OFF => false,
            LevelFilter::ERROR => matches!(*level, tracing::Level::ERROR),
            LevelFilter::WARN => matches!(*level, tracing::Level::ERROR | tracing::Level::WARN),
            LevelFilter::INFO => {
                matches!(*level, tracing::Level::ERROR | tracing::Level::WARN | tracing::Level::INFO)
            }
            LevelFilter::DEBUG => matches!(
                *level,
                tracing::Level::ERROR
                    | tracing::Level::WARN
                    | tracing::Level::INFO
                    | tracing::Level::DEBUG
            ),
            LevelFilter::TRACE => true,
        }
    }

    fn format_event(&self, metadata: &Metadata<'_>, visitor: EventVisitor) -> String {
        let mut line = String::new();
        let _ = write!(line, "[{} {}]", metadata.level(), metadata.target());

        if let Some(message) = visitor.message {
            let _ = write!(line, " {}", message);
        }

        if !visitor.fields.is_empty() {
            let _ = write!(line, " {}", visitor.fields.join(" "));
        }

        line.push('\n');
        line
    }
}

impl Subscriber for RConsoleSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        self.level_enabled(metadata.level())
    }

    fn new_span(&self, _attrs: &Attributes<'_>) -> Id {
        Id::from_u64(self.next_id.fetch_add(1, Ordering::Relaxed))
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        if !self.enabled(event.metadata()) {
            return;
        }

        let mut visitor = EventVisitor::default();
        event.record(&mut visitor);

        let line = self.format_event(event.metadata(), visitor);

        if *event.metadata().level() <= tracing::Level::WARN {
            print_r_error(line);
        } else {
            print_r_output(line);
        }
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

#[derive(Default)]
struct EventVisitor {
    message: Option<String>,
    fields: Vec<String>,
}

impl Visit for EventVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{value:?}"));
        } else {
            self.fields
                .push(format!("{}={value:?}", field.name()));
        }
    }
}
