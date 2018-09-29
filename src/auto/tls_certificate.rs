// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use SocketConnectable;
use TlsCertificateFlags;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TlsCertificate(Object<ffi::GTlsCertificate, ffi::GTlsCertificateClass>);

    match fn {
        get_type => || ffi::g_tls_certificate_get_type(),
    }
}

impl TlsCertificate {
    pub fn new_from_file<P: AsRef<std::path::Path>>(file: P) -> Result<TlsCertificate, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_files<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(cert_file: P, key_file: Q) -> Result<TlsCertificate, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_files(cert_file.as_ref().to_glib_none().0, key_file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_pem(data: &str) -> Result<TlsCertificate, Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_pem(data.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn list_new_from_file<P: AsRef<std::path::Path>>(file: P) -> Result<Vec<TlsCertificate>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_list_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait TlsCertificateExt {
    fn get_issuer(&self) -> Option<TlsCertificate>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn is_same(&self, cert_two: &TlsCertificate) -> bool;

    fn verify<'a, 'b, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsCertificate>>>(&self, identity: Q, trusted_ca: R) -> TlsCertificateFlags;

    //fn get_property_certificate(&self) -> /*Ignored*/Option<glib::ByteArray>;

    fn get_property_certificate_pem(&self) -> Option<String>;
}

impl<O: IsA<TlsCertificate> + IsA<glib::object::Object>> TlsCertificateExt for O {
    fn get_issuer(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_certificate_get_issuer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn is_same(&self, cert_two: &TlsCertificate) -> bool {
        unsafe {
            from_glib(ffi::g_tls_certificate_is_same(self.to_glib_none().0, cert_two.to_glib_none().0))
        }
    }

    fn verify<'a, 'b, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsCertificate>>>(&self, identity: Q, trusted_ca: R) -> TlsCertificateFlags {
        let identity = identity.into();
        let identity = identity.to_glib_none();
        let trusted_ca = trusted_ca.into();
        let trusted_ca = trusted_ca.to_glib_none();
        unsafe {
            from_glib(ffi::g_tls_certificate_verify(self.to_glib_none().0, identity.0, trusted_ca.0))
        }
    }

    //fn get_property_certificate(&self) -> /*Ignored*/Option<glib::ByteArray> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "certificate".to_glib_none().0, value.to_glib_none_mut().0);
    //        value.get()
    //    }
    //}

    fn get_property_certificate_pem(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "certificate-pem".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }
}
