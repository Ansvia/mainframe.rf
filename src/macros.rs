//! Koleksi macro library internal

macro_rules! implement_crypto_wrapper {
    ( $(#[$attr:meta])*  struct $name:ident, $size:expr) => {
        implement_crypto_wrapper!( $(#[$attr])* struct $name, $crate::crypto::ds::$name, $name, $size );
    };
    ( $(#[$attr:meta])* struct $name:ident, $source:path, $source_name:ident, $size:expr) => {
        /// Crypto object wrapper
        #[derive(Clone)]
        $(#[$attr])*
        pub struct $name([u8; $size]);

        impl $name {
            #[doc(hidden)]
            pub fn new(bytes_array: [u8; $size]) -> Self {
                let a = {
                    use $source;
                    $source_name::from_bytes(&bytes_array).expect("from bytes")
                };
                $name(a.to_bytes())
            }

            /// Creates new instance from bytes slice.
            #[inline]
            pub fn from_slice(bytes: &[u8]) -> Option<Self> {
                // kode ini kelihatan aneh, tapi hanya dengan cara inilah
                // kode bagian ini bisa dicompile di Rust stable.
                // kemungkinan kalau nanti Rust stable sudah bisa menghandle
                // macro type path agar bisa langsung digunakan untuk memanggil
                // fungsi statis-nya kode ini akan dirubah.

                let a = {
                    use $source;
                    $source_name::from_bytes(bytes)
                };
                a.map(|a| $name(a.to_bytes())).ok()
            }

            /// Convert to hex string
            #[inline]
            pub fn to_hex(&self) -> String {
                hex::encode(&self.0[..])
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({}..)", stringify!($name), &self.to_hex()[..8])
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({}..)", stringify!($name), &self.to_hex()[..8])
            }
        }

        impl ::hex::FromHex for $name {
            type Error = ::hex::FromHexError;

            fn from_hex<T: AsRef<[u8]>>(v: T) -> Result<Self, Self::Error> {
                let bytes = Vec::<u8>::from_hex(v)?;
                if let Some(self_value) = Self::from_slice(bytes.as_ref()) {
                    Ok(self_value)
                } else {
                    Err(::hex::FromHexError::InvalidStringLength)
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::hex::FromHexError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use hex::FromHex;
                $name::from_hex(s)
            }
        }
    };
}

// macro_rules! api_endpoint {
//     ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
//         pub fn $name(state: &AppState, $query: $qt) -> ApiResult<$rv> {
//             let $schema = Schema::new(state.db());

//             {$($cs)+}
//         }
//     };
// }

macro_rules! api_endpoint_req {
    ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        pub fn $name(state: &AppState, $query: $qt, req: &ApiHttpRequest) -> ApiResult<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

macro_rules! api_endpoint_mut {
    ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        pub fn $name(state: &mut AppState, $query: $qt) -> ApiResult<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

macro_rules! api_tx_endpoint {
    ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        pub fn $name(state: &AppState, $query: TxQuery<$qt>) -> ApiResult<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

macro_rules! api_endpoint {
    ( #[authorized_only(user)] fn $name:ident ($state:ident, $query:ident : $query_type:ty, $req: ident) -> $rettype:ty { $($cs:tt)+ } ) => {

        #[authorized_only_macro(user)]
        pub fn $name($state: &AppState, $query: $query_type, $req: &ApiHttpRequest) -> ApiResult<$rettype> {
            $($cs)+
        }
    };
    ( $(#[$attr:meta])* fn $name:ident ($state:ident, $query:ident : $query_type:ty, $req: ident) -> $rettype:ty { $($cs:tt)+ } ) => {

        $(#[$attr])*
        pub fn $name($state: &AppState, $query: $query_type, $req: &ApiHttpRequest) -> ApiResult<$rettype> {
            $($cs)+
        }
    };
}
