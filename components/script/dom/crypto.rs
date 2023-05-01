/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */


// The Document Object Model (DOM) is a programming interface for web documents 
// 	bindings: The code to expose the DOM to JavaScript through IDL bindings.
use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::CryptoBinding::CryptoMethods;
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::globalscope::GlobalScope;
use crate::script_runtime::JSContext;
use dom_struct::dom_struct;
use js::jsapi::JSObject;
use js::jsapi::Type;
use js::rust::CustomAutoRooterGuard;
use js::typedarray::ArrayBufferView;
use servo_rand::{RngCore, ServoRng};
use std::ptr::NonNull; 


// codegen: Generated JS-Rust bindings.
// JS-Rust bindings refer to a set of tools and techniques that allow 
// JavaScript and Rust code to communicate and interoperate with each other.


unsafe_no_jsmanaged_fields!(ServoRng);
//! The `unsafe_no_jsmanaged_fields!()` macro adds an empty implementation of
//! `JSTraceable` to a datatype.

// https://developer.mozilla.org/en-US/docs/Web/API/Crypto
#[dom_struct]
pub struct Crypto {
    reflector_: Reflector,
    #[ignore_malloc_size_of = "Defined in rand"]
    rng: DomRefCell<ServoRng>,
}
// Document Object Model (DOM) is a programming interface for web documents
// This standard defines an interface definition language, Web IDL, 
// that can be used to describe interfaces that are intended to be implemented in web browsers.
// (define in lib.rs)
// pub struct ServoRng {
//     pub(crate) rng: ReseedingRng<IsaacCore, ServoReseeder>,
// } 
// IsaacCore: A random number generator that uses the ISAAC algorithm
// ServoReseeder: The reseeder for the in-memory RNG 
// Trait rand_core::block::BlockRngCore
// DomRefCell wrapper allows safe interior mutability in a WebIDL context.


impl Crypto {
    fn new_inherited() -> Crypto {
        Crypto {
            reflector_: Reflector::new(),
            rng: DomRefCell::new(ServoRng::new()),
        }
    }

    pub fn new(global: &GlobalScope) -> DomRoot<Crypto> {
        reflect_dom_object(Box::new(Crypto::new_inherited()), global)
    }
}
// Reflector: A struct to store a reference to the reflector of a DOM object.
// a "reflector" is a mechanism used by the WebIDL bindings to create a bridge between JS and Rust 

impl CryptoMethods for Crypto {
    #[allow(unsafe_code)]
    // https://dvcs.w3.org/hg/webcrypto-api/raw-file/tip/spec/Overview.html#Crypto-method-getRandomValues
    // The Crypto interface represents an interface to general purpose cryptographic functionality 
    // including a cryptographically strong pseudo-random number generator seeded with truly random values.
    fn GetRandomValues(
        &self,
        _cx: JSContext,
        mut input: CustomAutoRooterGuard<ArrayBufferView>,
    ) -> Fallible<NonNull<JSObject>> {
        let array_type = input.get_array_type();

        if !is_integer_buffer(array_type) {
            return Err(Error::TypeMismatch);
        } else {
            let mut data = unsafe { input.as_mut_slice() };
            if data.len() > 65536 {
                return Err(Error::QuotaExceeded);
            }
            self.rng.borrow_mut().fill_bytes(&mut data);
        }

        unsafe { Ok(NonNull::new_unchecked(*input.underlying_object())) }
    }
}

// JSContext is a C++ struct that represents a JavaScript runtime environment. 
// It is used to manage the execution of JavaScript code, including memory allocation, 
// garbage collection, and exception handling.

fn is_integer_buffer(array_type: Type) -> bool {
    match array_type {
        Type::Uint8 |
        Type::Uint8Clamped |
        Type::Int8 |
        Type::Uint16 |
        Type::Int16 |
        Type::Uint32 |
        Type::Int32 => true,
        _ => false,
    }
}
