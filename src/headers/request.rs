use http::Uri;

use super::{convert_header, define_header, string_header, true_header};

true_header! {
    /// indicates that the request is via an element using [hx-boost](https://htmx.org/attributes/hx-boost/)
    (HX_BOOSTED, HxBoosted, "hx-boosted")
}

convert_header! {
    /// the current URL of the browser
    Uri => (HX_CURRENT_URL, HxCurrentUrl, "hx-current-url")
}

true_header! {
    /// “true” if the request is for history restoration after a miss in the local history cache
    (HX_HISTORY_RESTORE_REQUEST, HxHistoryRestoreRequest, "hx-history-restore-request")
}

string_header! {
    /// the user response to an hx-prompt
    (HX_PROMPT, HxPrompt, "hx-prompt")
}

true_header! {
    /// always “true”
    (HX_REQUEST, HxRequest, "hx-request")
}

string_header! {
    /// the `id` of the target element if it exists
    (HX_TARGET, HxTarget, "hx-target")
}

string_header! {
    ///	the `name` of the triggered element if it exists
    (HX_TRIGGER_NAME, HxTriggerName, "hx-trigger-name")
}

string_header! {
    /// the `id`` of the triggered element if it exists
    (HX_TRIGGER, HxTrigger, "hx-trigger")
}
