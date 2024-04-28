//!
//! Abstraction over API differences between dynamic loading and static OpenSSL
//!

#[cfg(xmlsec_dynamic)]
use crate::bindings as backend;

#[cfg(xmlsec_static)]
#[cfg(target_os = "mac_os")]
mod backend {
    pub use crate::bindings::{
        xmlSecOpenSSLAppInit as xmlSecCryptoAppInit,
        xmlSecOpenSSLAppKeyCertLoad as xmlSecCryptoAppKeyCertLoad,
        xmlSecOpenSSLAppKeyCertLoadMemory as xmlSecCryptoAppKeyCertLoadMemory,
        xmlSecOpenSSLAppKeyLoadEx as xmlSecCryptoAppKeyLoad,
        xmlSecOpenSSLAppKeyLoadMemory as xmlSecCryptoAppKeyLoadMemory,
        xmlSecOpenSSLAppShutdown as xmlSecCryptoAppShutdown, xmlSecOpenSSLInit as xmlSecCryptoInit,
        xmlSecOpenSSLShutdown as xmlSecCryptoShutdown,
    };
}

#[cfg(xmlsec_static)]
#[cfg(target_os = "linux")]
mod backend {
    pub use crate::bindings::{
        xmlSecOpenSSLAppInit as xmlSecCryptoAppInit,
        xmlSecOpenSSLAppKeyCertLoad as xmlSecCryptoAppKeyCertLoad,
        xmlSecOpenSSLAppKeyCertLoadMemory as xmlSecCryptoAppKeyCertLoadMemory,
        xmlSecOpenSSLAppKeyLoad as xmlSecCryptoAppKeyLoad,
        xmlSecOpenSSLAppKeyLoadMemory as xmlSecCryptoAppKeyLoadMemory,
        xmlSecOpenSSLAppShutdown as xmlSecCryptoAppShutdown, xmlSecOpenSSLInit as xmlSecCryptoInit,
        xmlSecOpenSSLShutdown as xmlSecCryptoShutdown,
    };
}

pub use backend::*;
