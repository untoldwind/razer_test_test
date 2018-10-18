error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        CString(::std::ffi::NulError);
        CStr(::std::ffi::FromBytesWithNulError);
    }

    errors {
        Hidapi(t: ::hidapi::HidError) {
            description("hidapi error")
            display("hidapi error: '{}'", t)
        }

        NotSuccessful {
            description("not successful")
            display("not successful")
        }

        NotSupported {
            description("not supported")
            display("not supported")
        }

        InvalidColorFormat {
            description("invalid color format")
            display("invalid color format")
        }
    }
}

impl From<::hidapi::HidError> for Error {
    fn from(hid_error: ::hidapi::HidError) -> Error {
        Error::from_kind(ErrorKind::Hidapi(hid_error))
    }
}
