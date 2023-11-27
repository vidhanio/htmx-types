use std::collections::HashMap;

use headers_core::{Header, HeaderValue};
use http::{HeaderName, Uri};
use serde::{Deserialize, Serialize};

use super::{convert_header, define_header, string_header, true_header};
use crate::Swap;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AjaxContext {
    pub source: Option<String>,
    pub event: Option<String>,
    pub handler: Option<String>,
    pub target: Option<String>,
    pub swap: Option<String>,
    pub values: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub select: Option<String>,
}

define_header! {
    /// allows you to do a client-side redirect that does not do a full page reload
    ///
    /// [htmx docs](https://htmx.org/headers/hx-location/)
    (HX_LOCATION, "hx-location")


    #[derive(Serialize, Deserialize)]
    pub struct HxLocation {
        #[serde(with = "http_serde::uri")]
        pub path: Uri,

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

define_header! {
    /// pushes a new url into the history stack
    ///
    /// [htmx docs](https://htmx.org/headers/hx-push-url/)
    (HX_PUSH_URL, "hx-push-url")

    pub enum HxPushUrl {
        Uri(Uri),
        PreventHistoryUpdate,
    }
}

impl Header for HxPushUrl {
    fn name() -> &'static HeaderName {
        &HX_PUSH_URL
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        match (values.next(), values.next()) {
            (Some(value), None) => {
                if value == "false" {
                    Ok(Self::PreventHistoryUpdate)
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
            Self::PreventHistoryUpdate => HeaderValue::from_static("false"),
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
    /// replaces the current URL in the location bar
    ///
    /// [htmx docs](https://htmx.org/headers/hx-replace-url/)
    (HX_REPLACE_URL, "hx-replace-url")

    pub enum HxReplaceUrl {
        Uri(Uri),
        PreventHistoryUpdate,
    }
}

impl Header for HxReplaceUrl {
    fn name() -> &'static HeaderName {
        &HX_REPLACE_URL
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        match (values.next(), values.next()) {
            (Some(value), None) => {
                if value == "false" {
                    Ok(Self::PreventHistoryUpdate)
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
            Self::PreventHistoryUpdate => HeaderValue::from_static("false"),
        };

        values.extend(std::iter::once(header));
    }
}

define_header! {
    /// allows you to specify how the response will be swapped. See [hx-swap](https://htmx.org/attributes/hx-swap/) for possible values
    (HX_RESWAP, "hx-reswap")
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
        List(Vec<String>),
        WithDetails(HashMap<String, serde_json::Value>),
        #[doc(hidden)]
        #[allow(dead_code)]
        Phantom(std::marker::PhantomData<After>),
    }
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
    pub struct AfterSwap;
}

impl TriggerAfter for AfterSwap {
    fn name() -> &'static HeaderName {
        &HX_TRIGGER_AFTER_SWAP
    }
}

pub trait TriggerAfter {
    fn name() -> &'static HeaderName;
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
                    .map_err(|_| headers_core::Error::invalid())
            }
            (Some(val1), Some(val2)) => {
                let s1 = val1.to_str().map_err(|_| headers_core::Error::invalid())?;
                let s2 = val2.to_str().map_err(|_| headers_core::Error::invalid())?;
                let mut list = vec![s1.to_owned(), s2.to_owned()];
                list.reserve(values.size_hint().0);
                values
                    .try_fold(list, |mut list, value| {
                        let s = value.to_str().map_err(|_| headers_core::Error::invalid())?;
                        list.push(s.to_owned());
                        Ok(list)
                    })
                    .map(Self::List)
            }
            (None, _) => Ok(Self::List(Vec::new())),
        }
    }

    /// NOTE: Panics if the value cannot be converted to a header value.
    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        match self {
            Self::List(list) => {
                values.extend(list.iter().map(|s| HeaderValue::from_str(s).unwrap()));
            }
            Self::WithDetails(details) => {
                let s = serde_json::to_string(details).unwrap();
                values.extend(std::iter::once(HeaderValue::from_str(&s).unwrap()));
            }
            Self::Phantom(_) => {}
        };
    }
}
