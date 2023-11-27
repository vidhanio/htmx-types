//! htmx response headers.

use std::collections::HashMap;

use headers_core::{Header, HeaderValue};
use http::{HeaderName, Uri};
use serde::{Deserialize, Serialize};

use super::{convert_header, define_header, string_header, true_header};
use crate::Swap;

/// ajax context for use with [`HxLocation`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AjaxContext {
    /// the source element of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// an event that “triggered” the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,

    /// a callback that will handle the response HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,

    /// the target to swap the response into
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// how the response will be swapped in relative to the target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<String>,

    /// values to submit with the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<HashMap<String, String>>,

    /// headers to submit with the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// allows you to select the content you want swapped from a response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,
}

define_header! {
    /// allows you to do a client-side redirect that does not do a full page reload
    ///
    /// [htmx docs](https://htmx.org/headers/hx-location/)
    (HX_LOCATION, "hx-location")


    #[derive(Serialize, Deserialize)]
    pub struct HxLocation {
        /// url to load the response from.
        #[serde(with = "http_serde::uri")]
        pub path: Uri,

        /// other data, which mirrors the [ajax](https://htmx.org/api/#ajax) api context.
        #[serde(flatten)]
        pub context: Option<AjaxContext>,
    }
}

impl Header for HxLocation {
    fn name() -> &'static HeaderName {
        &HX_LOCATION
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        match (values.next(), values.next()) {
            (Some(value), None) => {
                serde_json::from_slice(value.as_bytes()).map_err(|_| headers_core::Error::invalid())
            }
            _ => Err(headers_core::Error::invalid()),
        }
    }

    /// NOTE: Panics if the value cannot be converted to a header value.
    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let header = match self {
            Self {
                path,
                context: None,
            } => HeaderValue::from_str(&path.to_string()).unwrap(),
            Self {
                context: Some(_), ..
            } => {
                let s = serde_json::to_string(self).unwrap();
                HeaderValue::from_str(&s).unwrap()
            }
        };

        values.extend(std::iter::once(header));
    }
}

/// to be used with [`HxPushUrl`] or [`HxReplaceUrl`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HxModifyHistory<M: HistoryModification> {
    /// a url to modify the history with.
    Uri(Uri),

    /// do not change the history.
    NoChange,
    #[doc(hidden)]
    #[allow(dead_code)]
    Phantom(std::marker::PhantomData<M>),
}

/// history modification headers.
pub trait HistoryModification {
    /// the name of the header.
    fn name() -> &'static HeaderName;
}

define_header! {
    /// pushes a new url into the history stack
    ///
    /// [htmx docs](https://htmx.org/headers/hx-push-url/)
    (HX_PUSH_URL, "hx-push-url")

    #[derive(Copy)]
    pub struct HxPushUrl;
}

impl HistoryModification for HxPushUrl {
    fn name() -> &'static HeaderName {
        &HX_PUSH_URL
    }
}

define_header! {
    /// replaces the current url in the history stack
    ///
    /// [htmx docs](https://htmx.org/headers/hx-replace-url/)
    (HX_REPLACE_URL, "hx-replace-url")

    #[derive(Copy)]
    pub struct HxReplaceUrl;
}

impl HistoryModification for HxReplaceUrl {
    fn name() -> &'static HeaderName {
        &HX_REPLACE_URL
    }
}

impl<M: HistoryModification> Header for HxModifyHistory<M> {
    fn name() -> &'static HeaderName {
        M::name()
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        match (values.next(), values.next()) {
            (Some(value), None) => {
                if value == "false" {
                    Ok(Self::NoChange)
                } else {
                    value
                        .as_bytes()
                        .try_into()
                        .map(Self::Uri)
                        .map_err(|_| headers_core::Error::invalid())
                }
            }
            _ => Err(headers_core::Error::invalid()),
        }
    }

    /// NOTE: Panics if the value cannot be converted to a header value.
    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let header = match self {
            Self::Uri(uri) => HeaderValue::from_str(&uri.to_string()).unwrap(),
            Self::NoChange => HeaderValue::from_static("false"),
            Self::Phantom(_) => return,
        };

        values.extend(std::iter::once(header));
    }
}

convert_header! {
    /// can be used to do a client-side redirect to a new location
    Uri => (HX_REDIRECT, HxRedirect, "hx-redirect")
}

true_header! {
    /// if set to “true” the client-side will do a full refresh of the page
    (HX_REFRESH, HxRefresh, "hx-refresh")
}

define_header! {
    /// allows you to specify how the response will be swapped. See [hx-swap](https://htmx.org/attributes/hx-swap/) for possible values
    (HX_RESWAP, "hx-reswap")

    #[derive(Copy)]
    pub struct HxReswap(pub Swap);
}

impl Header for HxReswap {
    fn name() -> &'static HeaderName {
        &HX_RESWAP
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        match (values.next(), values.next()) {
            (Some(value), None) => value
                .as_bytes()
                .try_into()
                .map(Self)
                .map_err(|()| headers_core::Error::invalid()),
            _ => Err(headers_core::Error::invalid()),
        }
    }

    /// NOTE: Panics if the value cannot be converted to a header value.
    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(self.0.into()));
    }
}

string_header! {
    /// a CSS selector that updates the target of the content update to a different element on the page
    (HX_RETARGET, HxRetarget, "hx-retarget")
}

string_header! {
    /// a CSS selector that allows you to choose which part of the response is used to be swapped in. Overrides an existing [hx-select](https://htmx.org/attributes/hx-select/) on the triggering element
    (HX_RESELECT, HxReselect, "hx-reselect")
}

define_header! {
    /// allows you to trigger client-side events
    ///
    /// [htmx docs](https://htmx.org/headers/hx-trigger/)
    (HX_TRIGGER, "hx-trigger")

    pub enum HxTrigger<After: TriggerAfter = ()> {
        /// a list of events to trigger
        List(Vec<String>),

        /// a map of events to trigger with details
        WithDetails(HashMap<String, serde_json::Value>),
        #[doc(hidden)]
        #[allow(dead_code)]
        Phantom(std::marker::PhantomData<After>),
    }
}

/// trigger after headers.
pub trait TriggerAfter {
    /// the name of the header.
    fn name() -> &'static HeaderName;
}

impl TriggerAfter for () {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER
    }
}

define_header! {
    /// allows you to trigger client-side events after the settle step
    ///
    /// [htmx docs](https://htmx.org/headers/hx-trigger/)
    (HX_TRIGGER_AFTER_SETTLE, "hx-trigger-after-settle")

    #[derive(Copy)]
    pub struct AfterSettle;
}

impl TriggerAfter for AfterSettle {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER_AFTER_SETTLE
    }
}

define_header! {
    /// allows you to trigger client-side events after the swap step
    ///
    /// [htmx docs](https://htmx.org/headers/hx-trigger/)
    (HX_TRIGGER_AFTER_SWAP, "hx-trigger-after-swap")

    #[derive(Copy)]
    pub struct AfterSwap;
}

impl TriggerAfter for AfterSwap {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER_AFTER_SWAP
    }
}

impl<After: TriggerAfter> Header for HxTrigger<After> {
    fn name() -> &'static HeaderName {
        After::name()
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        match (values.next(), values.next()) {
            (Some(value), None) => {
                let bytes = value.as_bytes();
                serde_json::from_slice(bytes)
                    .map(Self::WithDetails)
                    .or_else(|_| {
                        let items = value
                            .to_str()
                            .map_err(|_| headers_core::Error::invalid())?
                            .split(',')
                            .map(|s| s.trim().to_owned())
                            .collect();

                        Ok(Self::List(items))
                    })
            }
            _ => Err(headers_core::Error::invalid()),
        }
    }

    /// NOTE: Panics if the value cannot be converted to a header value.
    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let val = match self {
            Self::List(list) => {
                let s = list.join(", ");
                HeaderValue::from_str(&s).unwrap()
            }
            Self::WithDetails(details) => {
                let s = serde_json::to_string(details).unwrap();
                HeaderValue::from_str(&s).unwrap()
            }
            Self::Phantom(_) => return,
        };

        values.extend(std::iter::once(val));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_works() {
        let val = HeaderValue::from_static(r#"{"event1":"A message", "event2":"Another message"}"#);

        claims::assert_ok_eq!(
            HxTrigger::<()>::decode(&mut std::iter::once(&val)),
            HxTrigger::WithDetails(
                vec![
                    ("event1".to_owned(), "A message".into()),
                    ("event2".to_owned(), "Another message".into()),
                ]
                .into_iter()
                .collect()
            )
        );

        let val = HeaderValue::from_static("event1, event2");

        claims::assert_ok_eq!(
            HxTrigger::<()>::decode(&mut std::iter::once(&val)),
            HxTrigger::List(vec!["event1".to_owned(), "event2".to_owned()])
        );
    }
}
