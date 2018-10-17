error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        Hidapi(t: ::hidapi::HidError) {
            description("hidapi error")
            display("hidapi error: '{}'", t)
        }
    }
}

impl From<::hidapi::HidError> for Error {
    fn from(hid_error: ::hidapi::HidError) -> Error {
        Error::from_kind(ErrorKind::Hidapi(hid_error))
    }
}
