//! Types for working with [htmx](https://htmx.org/).

use http::HeaderValue;
use serde::{Deserialize, Serialize};

mod headers;

/// The hx-swap attribute allows you to specify how the response will be swapped in relative to the [target](https://htmx.org/attributes/hx-target/) of an AJAX request.
///
/// [htmx docs](https://htmx.org/attributes/hx-swap/)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Swap {
    /// Replace the inner html of the target element
    #[serde(rename = "innerHtml")]
    InnerHtml,

    /// Replace the entire target element with the response
    #[serde(rename = "outerHtml")]
    OuterHtml,

    /// Insert the response before the target element
    #[serde(rename = "beforebegin")]
    BeforeBegin,

    /// Insert the response before the first child of the target element
    #[serde(rename = "afterbegin")]
    AfterBegin,

    /// Insert the response after the last child of the target element
    #[serde(rename = "beforeend")]
    BeforeEnd,

    /// Insert the response after the target element
    #[serde(rename = "afterend")]
    AfterEnd,

    /// Deletes the target element regardless of the response
    #[serde(rename = "delete")]
    Delete,

    /// Does not append content from response (out of band items will still be
    /// processed).
    #[serde(rename = "none")]
    None,
}

impl From<Swap> for HeaderValue {
    fn from(swap: Swap) -> Self {
        match swap {
            Swap::InnerHtml => Self::from_static("innerHtml"),
            Swap::OuterHtml => Self::from_static("outerHtml"),
            Swap::BeforeBegin => Self::from_static("beforebegin"),
            Swap::AfterBegin => Self::from_static("afterbegin"),
            Swap::BeforeEnd => Self::from_static("beforeend"),
            Swap::AfterEnd => Self::from_static("afterend"),
            Swap::Delete => Self::from_static("delete"),
            Swap::None => Self::from_static("none"),
        }
    }
}

impl TryFrom<&[u8]> for Swap {
    type Error = ();

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes {
            b"innerHtml" => Ok(Self::InnerHtml),
            b"outerHtml" => Ok(Self::OuterHtml),
            b"beforebegin" => Ok(Self::BeforeBegin),
            b"afterbegin" => Ok(Self::AfterBegin),
            b"beforeend" => Ok(Self::BeforeEnd),
            b"afterend" => Ok(Self::AfterEnd),
            b"delete" => Ok(Self::Delete),
            b"none" => Ok(Self::None),
            _ => Err(()),
        }
    }
}
