error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        CApi(::std::ffi::NulError);
    }

    errors {
        Hidapi(t: ::hidapi::HidError) {
            description("hidapi error")
            display("hidapi error: '{}'", t)
        }

        NotSupported {
            description("not supported")
            display("not supported")
        }
    }
}

impl From<::hidapi::HidError> for Error {
    fn from(hid_error: ::hidapi::HidError) -> Error {
        Error::from_kind(ErrorKind::Hidapi(hid_error))
    }
}
