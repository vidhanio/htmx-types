mod request;
mod response;

macro_rules! define_header {
    {
        $(#[$docs:meta])*
        ($STATIC:ident, $name_bytes:literal)
        $($rest:tt)*
    } => {
        $(#[$docs])*
        pub static $STATIC: headers_core::HeaderName = headers_core::HeaderName::from_static($name_bytes);

        $(#[$docs])*
        $($rest)*
    };
}
use define_header;

macro_rules! true_header {
    {
        $(#[$docs:meta])*
        ($STATIC:ident, $UpCase:ident, $name_bytes:literal)
    } => {
        define_header! {
            $(#[$docs])*
            ($STATIC, $name_bytes)
            pub struct $UpCase;
        }

        impl headers_core::Header for $UpCase {
            fn name() -> &'static headers_core::HeaderName {
                &$STATIC
            }

            fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
            where
                Self: Sized,
                I: Iterator<Item = &'i headers_core::HeaderValue>,
            {
                match (values.next(), values.next()) {
                    (Some(value), None) => {
                        if value == "true" {
                            Ok(Self)
                        } else {
                            Err(headers_core::Error::invalid())
                        }
                    }
                    _ => Err(headers_core::Error::invalid()),
                }
            }

            fn encode<E: Extend<headers_core::HeaderValue>>(&self, values: &mut E) {
                values.extend(std::iter::once(headers_core::HeaderValue::from_static("true")));
            }
        }
    }
}
use true_header;

macro_rules! convert_header {
    {
        $(#[$docs:meta])*
        $Ty:ty => ($STATIC:ident, $UpCase:ident, $name_bytes:literal)
    } => {
        define_header! {
            $(#[$docs])*
            ($STATIC, $name_bytes)
            pub struct $UpCase(pub $Ty);
        }

        impl headers_core::Header for $UpCase {
            fn name() -> &'static headers_core::HeaderName {
                &$STATIC
            }

            fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
            where
                Self: Sized,
                I: Iterator<Item = &'i headers_core::HeaderValue>,
            {
                match (values.next(), values.next()) {
                    (Some(value), None) => {
                        value.as_bytes().try_into().map(Self).map_err(|_| headers_core::Error::invalid())
                    }
                    _ => Err(headers_core::Error::invalid()),
                }
            }

            /// NOTE: Panics if the value cannot be converted to a header value.
            fn encode<E: Extend<headers_core::HeaderValue>>(&self, values: &mut E) {
                let s = self.0.to_string();
                let header = headers_core::HeaderValue::from_str(&s).unwrap();
                values.extend(std::iter::once(header));
            }
        }
    }
}
use convert_header;

macro_rules! string_header {
    {
        $(#[$docs:meta])*
        ($STATIC:ident, $UpCase:ident, $name_bytes:literal)
    } => {
        define_header! {
            $(#[$docs])*
            ($STATIC, $name_bytes)
            pub struct $UpCase(pub String);
        }

        impl headers_core::Header for $UpCase {
            fn name() -> &'static headers_core::HeaderName {
                &$STATIC
            }

            fn decode<'i, I>(values: &mut I) -> Result<Self, headers_core::Error>
            where
                Self: Sized,
                I: Iterator<Item = &'i headers_core::HeaderValue>,
            {
                match (values.next(), values.next()) {
                    (Some(value), None) => {
                        let s = value.to_str().map_err(|_| headers_core::Error::invalid())?;
                        Ok(Self(s.to_owned()))
                    }
                    _ => Err(headers_core::Error::invalid()),
                }
            }

            /// NOTE: Panics if the value cannot be converted to a header value.
            fn encode<E: Extend<headers_core::HeaderValue>>(&self, values: &mut E) {
                values.extend(std::iter::once(headers_core::HeaderValue::from_str(&self.0).unwrap()));
            }
        }
    }
}
use string_header;
