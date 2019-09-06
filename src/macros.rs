extern crate paste;

#[macro_export]
macro_rules! try_ffi {
    ($expr:expr) => {
        match $expr {
            Ok(expr) => expr,
            Err(err) => {
                crate::errors::ERROR
                    .lock()
                    .replace(failure::Error::from(err));

                return crate::errors::HederaResult::Failure;
            }
        }
    };
}

#[macro_export]
macro_rules! def_to_str {
    ($name:ident: $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(p: *mut $ty) -> *mut libc::c_char {
            mbox::MString::from_str(&unsafe { &(*p) }.to_string())
                .into_mbox_with_sentinel()
                .into_raw() as _
        }
    };
}

#[macro_export]
macro_rules! def_from_str {
    ($name:ident: $ty:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(
            s: *const libc::c_char,
            out: *mut $ty,
        ) -> crate::errors::HederaResult {
            let s = try_ffi!(unsafe { std::ffi::CStr::from_ptr(s) }.to_str());

            unsafe {
                *out = try_ffi!(s.parse());
            }

            crate::errors::HederaResult::Success
        }
    };
}

#[macro_export]
macro_rules! def_query {
    // a single parameter query
    ($constructor:ident: $verb:ident($p:ty) -> $cty:ty) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [<hedera_query__ $verb __new>] (
                client: *mut hedera::Client,
                _1: $p,
            ) -> *mut hedera::query::Query<$constructor> {
                Box::into_raw(Box::new($constructor::new(&*client, _1.into())))
            }

            #[no_mangle]
            pub unsafe extern "C" fn [<hedera_query__ $verb __get>] (
                query: *mut hedera::query::Query<$constructor>,
                out: *mut $cty,
            ) -> crate::errors::HederaResult {
                *out = try_ffi!(Box::from_raw(query).get()).into();

                crate::errors::HederaResult::Success
            }
        }
    };

    // a double parameter query
    ($constructor:ident: $verb:ident($p1:ty, $p2:ty) -> $cty:ty) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [<hedera_query__ $verb __new>]  (
                client: *mut hedera::Client,
                _1: $p1,
                _2: $p2,
            ) -> *mut hedera::query::Query<$constructor> {
                Box::into_raw(Box::new($constructor::new(&*client, _1.into(), _2.into())))
            }

            #[no_mangle]
            pub unsafe extern "C" fn [<hedera_query__ $verb __get>] (
                query: *mut hedera::query::Query<$constructor>,
                out: *mut $cty,
            ) -> crate::errors::HederaResult {
                *out = try_ffi!(Box::from_raw(query).get()).into();

                crate::errors::HederaResult::Success
            }
        }
    };
}

#[macro_export]
macro_rules! def_query_new {
    // a single parameter query
    ($constructor:ident: $verb:ident($p:ty)) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [<hedera_query__ $verb __new>] (
                client: *mut hedera::Client,
                _1: $p,
            ) -> *mut hedera::query::Query<$constructor> {
                Box::into_raw(Box::new($constructor::new(&*client, _1.into())))
            }
        }
    };

    // a multi parameter query
    ($constructor:ident: $verb:ident($p1:ty, $p2:ty, array_of($p3:ty), $p4:ty) -> $cty:ty) => {
        use std::slice;

        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [<hedera_query__ $verb __new>]  (
                client: *mut hedera::Client,
                _1: $p1,
                _2: $p2,
                _3: *const $p3,
                _3_len: usize,
                _4: $p4
            ) -> *mut hedera::query::Query<$constructor> {
                let _3 = slice::from_raw_parts(_3, _3_len);
                Box::into_raw(Box::new($constructor::new(&*client, _1.into(), _2.into(), _3.into(), _4.into())))
            }
        }
    };
}

#[macro_export]
macro_rules! def_query_method {
    // a single parameter query function
    ($constructor:ident: $name:ident($ty:ty): $member:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            qu: *mut hedera::query::Query<$constructor>,
            _1: $ty,
        ) {
            (&mut *qu).$member(_1.into());
        }
    };

    // single parameter (array)
    ($constructor:ident: $name:ident(array_of($ty:ty)): $member:ident) => {
        use std::slice;

        #[no_mangle]
        pub unsafe extern "C" fn $name(
            qu: *mut hedera::query::Query<$constructor>,
            _1: *const $ty,
            _1_len: usize
        ) {
            let _1 = slice::from_raw_parts(_1, _1_len);

            (&mut *qu).$member(_1.into());
        }
    };
}

macro_rules! def_query_exec {
    ($constructor:ident: $verb:ident() -> $cty:ty) => {
        paste::item! {
            #[no_mangle]
            pub unsafe extern "C" fn [<hedera_query__ $verb __execute>] (
                query: *mut hedera::query::Query<$constructor>,
                out: *mut $cty,
            ) -> crate::errors::HederaResult {
                *out = try_ffi!(Box::from_raw(query).get()).into();

                crate::errors::HederaResult::Success
            }
        }
    };
}

#[macro_export]
macro_rules! def_tx_new {
    ($constructor:ident: $name:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
        ) -> *mut hedera::transaction::Transaction<$constructor> {
            Box::into_raw(Box::new($constructor::new(&*client)))
        }
    };

    ($constructor:ident: $name:ident(array_of($ty:ty))) => {
        use std::slice;

        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
            _1: *const $ty,
            _1_len: usize
        ) -> *mut hedera::transaction::Transaction<$constructor> {

            let _1 = slice::from_raw_parts(_1, _1_len);

            Box::into_raw(Box::new($constructor::new(&*client, _1.into())))
        }
    };


    ($constructor:ident: $name:ident($ty:ty)) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
            _1: $ty,
        ) -> *mut hedera::transaction::Transaction<$constructor> {
            Box::into_raw(Box::new($constructor::new(&*client, _1.into())))
        }
    };

    ($constructor:ident: $name:ident($p1:ty, array_of($p2:ty))) => {
        use std::slice;

        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
            _1: $p1,
            _2: *const $p2,
            _2_len: usize
        ) -> *mut hedera::transaction::Transaction<$constructor> {
            let _2 = slice::from_raw_parts(_2, _2_len);

            Box::into_raw(Box::new($constructor::new(&*client, _1.into(), _2.into())))
        }
    };

    ($constructor:ident: $name:ident($p1:ty, $p2:ty)) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            client: *mut hedera::Client,
            _1: $p1,
            _2: $p2,
        ) -> *mut hedera::transaction::Transaction<$constructor> {
            Box::into_raw(Box::new($constructor::new(&*client, _1.into(), _2.into())))
        }
    };

}

#[macro_export]
macro_rules! def_tx_method {
    ($constructor:ident: $name:ident(array_of($ty:ty)): $member:ident) => {
        use std::slice;

        #[no_mangle]
        pub unsafe extern "C" fn $name(
            tx: *mut hedera::transaction::Transaction<$constructor>,
            _1: *const $ty,
            _1_len: usize
        ) {
            let _1 = slice::from_raw_parts(_1, _1_len);

            (&mut *tx).$member(_1.into());
        }
    };

    ($constructor:ident: $name:ident($ty:ty): $member:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            tx: *mut hedera::transaction::Transaction<$constructor>,
            _1: $ty,
        ) {
            (&mut *tx).$member(_1.into());
        }
    };
    
    ($constructor:ident: $name:ident($p1:ty, array_of($p2:ty)): $member:ident) => {
        use std::slice;

        #[no_mangle]
        pub unsafe extern "C" fn $name(
            tx: *mut hedera::transaction::Transaction<$constructor>,
            _1: $p1,
            _2: *const $p2,
            _2_len: usize
        ) {

            let _2 = slice::from_raw_parts(_2, _2_len);

            (&mut *tx).$member(_1.into(), _2.into());
        }
    };

    ($constructor:ident: $name:ident($p1:ty, $p2:ty): $member:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            tx: *mut hedera::transaction::Transaction<$constructor>,
            _1: $p1,
            _2: $p2,
        ) {

            (&mut *tx).$member(_1.into(), _2.into());
        }
    };

}
