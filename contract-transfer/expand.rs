#![feature(prelude_import)]
//! A smart contract which demonstrates behavior of the `self.env().transfer()` function.
//! It transfers some of it's balance to the caller.
#![allow(clippy::new_without_default)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod give_me {
    impl ::ink::traits::ContractEnv for GiveMe {
        type Env = ::ink::env::DefaultEnvironment;
    }
    type Environment = <GiveMe as ::ink::traits::ContractEnv>::Env;
    type AccountId = <<GiveMe as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
    type Balance = <<GiveMe as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Balance;
    type Hash = <<GiveMe as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Hash;
    type Timestamp = <<GiveMe as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
    type BlockNumber = <<GiveMe as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
    const _: () = {
        struct Check {
            salt: (),
        }
    };
    /// No storage is needed for this simple contract.
    #[cfg(not(feature = "__ink_dylint_Storage"))]
    pub struct GiveMe {}
    const _: () = {
        impl<
            __ink_generic_salt: ::ink::storage::traits::StorageKey,
        > ::ink::storage::traits::StorableHint<__ink_generic_salt> for GiveMe {
            type Type = GiveMe;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for GiveMe {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for GiveMe {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(GiveMe {})
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __dest: &mut __ink_O,
            ) {
                match self {
                    GiveMe {} => {}
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for GiveMe {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new("GiveMe", "contract_transfer::give_me"),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&["No storage is needed for this simple contract."])
                    .composite(::scale_info::build::Fields::named())
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for GiveMe {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new("GiveMe", []),
                )
            }
        }
    };
    const _: () = {
        impl ::ink::traits::ContractName for GiveMe {
            const NAME: &'static str = "GiveMe";
        }
    };
    const _: () = {
        impl<'a> ::ink::codegen::Env for &'a GiveMe {
            type EnvAccess = ::ink::EnvAccess<
                'a,
                <GiveMe as ::ink::traits::ContractEnv>::Env,
            >;
            fn env(self) -> Self::EnvAccess {
                <<Self as ::ink::codegen::Env>::EnvAccess as ::core::default::Default>::default()
            }
        }
        impl<'a> ::ink::codegen::StaticEnv for GiveMe {
            type EnvAccess = ::ink::EnvAccess<
                'static,
                <GiveMe as ::ink::traits::ContractEnv>::Env,
            >;
            fn env() -> Self::EnvAccess {
                <<Self as ::ink::codegen::StaticEnv>::EnvAccess as ::core::default::Default>::default()
            }
        }
    };
    const _: () = {
        #[allow(unused_imports)]
        use ::ink::codegen::{Env as _, StaticEnv as _};
        use ::ink::codegen::EmitEvent as _;
    };
    const _: () = {
        impl<'a> ::ink::codegen::EmitEvent<GiveMe>
        for ::ink::EnvAccess<'a, Environment> {
            fn emit_event<E>(self, event: E)
            where
                E: Into<<GiveMe as ::ink::traits::ContractEventBase>::Type>,
            {
                ::ink::env::emit_event::<
                    Environment,
                    <GiveMe as ::ink::traits::ContractEventBase>::Type,
                >(event.into());
            }
        }
    };
    #[allow(non_camel_case_types)]
    #[cfg(not(feature = "__ink_dylint_EventBase"))]
    pub enum __ink_EventBase {
        Received(Received),
        WrongReceived(WrongReceived),
    }
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::scale::Encode for __ink_EventBase {
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    __ink_EventBase::Received(ref aa) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::scale::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    __ink_EventBase::WrongReceived(ref aa) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::scale::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for __ink_EventBase {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::scale::Decode for __ink_EventBase {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e
                            .chain(
                                "Could not decode `__ink_EventBase`, failed to read variant byte",
                            )
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            __ink_EventBase::Received({
                                let __codec_res_edqy = <Received as ::scale::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `__ink_EventBase::Received.0`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    __codec_x_edqy if __codec_x_edqy
                        == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(
                            __ink_EventBase::WrongReceived({
                                let __codec_res_edqy = <WrongReceived as ::scale::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `__ink_EventBase::WrongReceived.0`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }),
                        )
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into(
                                "Could not decode `__ink_EventBase`, variant doesn't exist",
                            ),
                        )
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink::traits::ContractEventBase for GiveMe {
            type Type = __ink_EventBase;
        }
    };
    const _: () = {
        impl From<Received> for __ink_EventBase {
            fn from(event: Received) -> Self {
                Self::Received(event)
            }
        }
    };
    const _: () = {
        impl From<WrongReceived> for __ink_EventBase {
            fn from(event: WrongReceived) -> Self {
                Self::WrongReceived(event)
            }
        }
    };
    const _: () = {
        pub enum __ink_UndefinedAmountOfTopics {}
        impl ::ink::env::topics::EventTopicsAmount for __ink_UndefinedAmountOfTopics {
            const AMOUNT: usize = 0;
        }
        impl ::ink::env::Topics for __ink_EventBase {
            type RemainingTopics = __ink_UndefinedAmountOfTopics;
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<
                    ::ink::env::topics::state::Uninit,
                    E,
                    B,
                >,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                match self {
                    Self::Received(event) => {
                        <Received as ::ink::env::Topics>::topics::<E, B>(event, builder)
                    }
                    Self::WrongReceived(event) => {
                        <WrongReceived as ::ink::env::Topics>::topics::<
                            E,
                            B,
                        >(event, builder)
                    }
                }
            }
        }
    };
    impl ::ink::codegen::EventLenTopics for Received {
        type LenTopics = ::ink::codegen::EventTopics<0usize>;
    }
    const _: () = ::ink::codegen::utils::consume_type::<
        ::ink::codegen::EventRespectsTopicLimit<
            Received,
            {
                <<GiveMe as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::MAX_EVENT_TOPICS
            },
        >,
    >();
    impl ::ink::codegen::EventLenTopics for WrongReceived {
        type LenTopics = ::ink::codegen::EventTopics<0usize>;
    }
    const _: () = ::ink::codegen::utils::consume_type::<
        ::ink::codegen::EventRespectsTopicLimit<
            WrongReceived,
            {
                <<GiveMe as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::MAX_EVENT_TOPICS
            },
        >,
    >();
    /// Received docs
    pub struct Received {
        value: Balance,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for Received {
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.value, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.value)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.value, f)
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for Received {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for Received {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Received {
                    value: {
                        let __codec_res_edqy = <Balance as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Received::value`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Received {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Received",
                "value",
                &&self.value,
            )
        }
    }
    /// Received docs
    pub struct WrongReceived {
        value: Balance,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for WrongReceived {
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.value, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.value)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.value, f)
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for WrongReceived {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for WrongReceived {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(WrongReceived {
                    value: {
                        let __codec_res_edqy = <Balance as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `WrongReceived::value`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for WrongReceived {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "WrongReceived",
                "value",
                &&self.value,
            )
        }
    }
    const _: () = {
        impl ::ink::env::Topics for Received {
            type RemainingTopics = [::ink::env::topics::state::HasRemainingTopics; 1usize];
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<
                    ::ink::env::topics::state::Uninit,
                    E,
                    B,
                >,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                builder
                    .build::<Self>()
                    .push_topic::<
                        ::ink::env::topics::PrefixedValue<[u8; 16usize]>,
                    >(
                        &::ink::env::topics::PrefixedValue {
                            value: b"GiveMe::Received",
                            prefix: b"",
                        },
                    )
                    .finish()
            }
        }
    };
    const _: () = {
        impl ::ink::env::Topics for WrongReceived {
            type RemainingTopics = [::ink::env::topics::state::HasRemainingTopics; 1usize];
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<
                    ::ink::env::topics::state::Uninit,
                    E,
                    B,
                >,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                builder
                    .build::<Self>()
                    .push_topic::<
                        ::ink::env::topics::PrefixedValue<[u8; 21usize]>,
                    >(
                        &::ink::env::topics::PrefixedValue {
                            value: b"GiveMe::WrongReceived",
                            prefix: b"",
                        },
                    )
                    .finish()
            }
        }
    };
    impl ::ink::traits::ContractAmountDispatchables for GiveMe {
        const MESSAGES: ::core::primitive::usize = 2usize;
        const CONSTRUCTORS: ::core::primitive::usize = 1usize;
    }
    impl ::ink::traits::ContractDispatchableMessages<
        { <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES },
    > for GiveMe {
        const IDS: [::core::primitive::u32; <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES] = [
            0x499BB739_u32,
            0xCAFEBABE_u32,
        ];
    }
    impl ::ink::traits::ContractDispatchableConstructors<
        { <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS },
    > for GiveMe {
        const IDS: [::core::primitive::u32; <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS] = [
            0x9BAE9D5E_u32,
        ];
    }
    impl ::ink::traits::DispatchableConstructorInfo<0x9BAE9D5E_u32> for GiveMe {
        type Input = ();
        type Storage = GiveMe;
        const CALLABLE: fn(Self::Input) -> Self::Storage = |_| { GiveMe::new() };
        const PAYABLE: ::core::primitive::bool = true;
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x9B_u8,
            0xAE_u8,
            0x9D_u8,
            0x5E_u8,
        ];
        const LABEL: &'static ::core::primitive::str = "new";
    }
    impl ::ink::traits::DispatchableMessageInfo<0x499BB739_u32> for GiveMe {
        type Input = Balance;
        type Output = ();
        type Storage = GiveMe;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            __ink_binding_0|
        { GiveMe::give_me(storage, __ink_binding_0) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x49_u8,
            0x9B_u8,
            0xB7_u8,
            0x39_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "give_me";
    }
    impl ::ink::traits::DispatchableMessageInfo<0xCAFEBABE_u32> for GiveMe {
        type Input = ();
        type Output = ();
        type Storage = GiveMe;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { GiveMe::was_it_ten(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0xCA_u8,
            0xFE_u8,
            0xBA_u8,
            0xBE_u8,
        ];
        const PAYABLE: ::core::primitive::bool = true;
        const MUTATES: ::core::primitive::bool = false;
        const LABEL: &'static ::core::primitive::str = "was_it_ten";
    }
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_ConstructorDecoder {
            Constructor0(
                <GiveMe as ::ink::traits::DispatchableConstructorInfo<
                    {
                        <GiveMe as ::ink::traits::ContractDispatchableConstructors<
                            {
                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                            },
                        >>::IDS[0usize]
                    },
                >>::Input,
            ),
        }
        impl ::ink::traits::DecodeDispatch for __ink_ConstructorDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::traits::DispatchError>
            where
                I: ::scale::Input,
            {
                const CONSTRUCTOR_0: [::core::primitive::u8; 4usize] = <GiveMe as ::ink::traits::DispatchableConstructorInfo<
                    {
                        <GiveMe as ::ink::traits::ContractDispatchableConstructors<
                            {
                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                            },
                        >>::IDS[0usize]
                    },
                >>::SELECTOR;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::traits::DispatchError::InvalidSelector)?
                {
                    CONSTRUCTOR_0 => {
                        ::core::result::Result::Ok(
                            Self::Constructor0(
                                <<GiveMe as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::traits::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    _invalid => {
                        ::core::result::Result::Err(
                            ::ink::traits::DispatchError::UnknownSelector,
                        )
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_ConstructorDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::traits::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        impl ::ink::traits::ExecuteDispatchable for __ink_ConstructorDecoder {
            #[allow(clippy::nonminimal_bool)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::traits::DispatchError> {
                match self {
                    Self::Constructor0(input) => {
                        if {
                            false
                                || <GiveMe as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<GiveMe as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <GiveMe as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        ::ink::codegen::execute_constructor::<
                            GiveMe,
                            _,
                            _,
                        >(move || {
                            <GiveMe as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::CALLABLE(input)
                        })
                    }
                }
            }
        }
        impl ::ink::traits::ContractConstructorDecoder for GiveMe {
            type Type = __ink_ConstructorDecoder;
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_MessageDecoder {
            Message0(
                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                    {
                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                            {
                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                            },
                        >>::IDS[0usize]
                    },
                >>::Input,
            ),
            Message1(
                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                    {
                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                            {
                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                            },
                        >>::IDS[1usize]
                    },
                >>::Input,
            ),
        }
        impl ::ink::traits::DecodeDispatch for __ink_MessageDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::traits::DispatchError>
            where
                I: ::scale::Input,
            {
                const MESSAGE_0: [::core::primitive::u8; 4usize] = <GiveMe as ::ink::traits::DispatchableMessageInfo<
                    {
                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                            {
                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                            },
                        >>::IDS[0usize]
                    },
                >>::SELECTOR;
                const MESSAGE_1: [::core::primitive::u8; 4usize] = <GiveMe as ::ink::traits::DispatchableMessageInfo<
                    {
                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                            {
                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                            },
                        >>::IDS[1usize]
                    },
                >>::SELECTOR;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::traits::DispatchError::InvalidSelector)?
                {
                    MESSAGE_0 => {
                        ::core::result::Result::Ok(
                            Self::Message0(
                                <<GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::traits::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    MESSAGE_1 => {
                        ::core::result::Result::Ok(
                            Self::Message1(
                                <<GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::traits::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    _invalid => {
                        ::core::result::Result::Err(
                            ::ink::traits::DispatchError::UnknownSelector,
                        )
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_MessageDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::traits::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        fn push_contract(contract: ::core::mem::ManuallyDrop<GiveMe>, mutates: bool) {
            if mutates {
                ::ink::env::set_contract_storage::<
                    ::ink::primitives::Key,
                    GiveMe,
                >(&<GiveMe as ::ink::storage::traits::StorageKey>::KEY, &contract);
            }
        }
        impl ::ink::traits::ExecuteDispatchable for __ink_MessageDecoder {
            #[allow(clippy::nonminimal_bool, clippy::let_unit_value)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::traits::DispatchError> {
                let key = <GiveMe as ::ink::storage::traits::StorageKey>::KEY;
                let mut contract: ::core::mem::ManuallyDrop<GiveMe> = ::core::mem::ManuallyDrop::new(
                    match ::ink::env::get_contract_storage(&key) {
                        ::core::result::Result::Ok(
                            ::core::option::Option::Some(value),
                        ) => value,
                        ::core::result::Result::Ok(::core::option::Option::None) => {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["storage entry was empty"],
                                    &[],
                                ),
                            )
                        }
                        ::core::result::Result::Err(_) => {
                            ::core::panicking::panic_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["could not properly decode storage entry"],
                                    &[],
                                ),
                            )
                        }
                    },
                );
                match self {
                    Self::Message0(input) => {
                        use ::core::default::Default;
                        if {
                            false
                                || <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<GiveMe as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <GiveMe as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <GiveMe as ::ink::traits::DispatchableMessageInfo<
                            {
                                <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[0usize]
                            },
                        >>::Output = <GiveMe as ::ink::traits::DispatchableMessageInfo<
                            {
                                <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[0usize]
                            },
                        >>::CALLABLE(&mut contract, input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if failure {
                            ::ink::env::return_value::<
                                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::Output,
                            >(
                                ::ink::env::ReturnFlags::default().set_reverted(true),
                                &result,
                            )
                        }
                        push_contract(
                            contract,
                            <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::MUTATES,
                        );
                        if ::core::any::TypeId::of::<
                            <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::Output,
                        >() != ::core::any::TypeId::of::<()>()
                        {
                            ::ink::env::return_value::<
                                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::Output,
                            >(::ink::env::ReturnFlags::default(), &result)
                        }
                    }
                    Self::Message1(input) => {
                        use ::core::default::Default;
                        if {
                            false
                                || <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<GiveMe as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <GiveMe as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <GiveMe as ::ink::traits::DispatchableMessageInfo<
                            {
                                <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[1usize]
                            },
                        >>::Output = <GiveMe as ::ink::traits::DispatchableMessageInfo<
                            {
                                <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[1usize]
                            },
                        >>::CALLABLE(&mut contract, input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if failure {
                            ::ink::env::return_value::<
                                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::Output,
                            >(
                                ::ink::env::ReturnFlags::default().set_reverted(true),
                                &result,
                            )
                        }
                        push_contract(
                            contract,
                            <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::MUTATES,
                        );
                        if ::core::any::TypeId::of::<
                            <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::Output,
                        >() != ::core::any::TypeId::of::<()>()
                        {
                            ::ink::env::return_value::<
                                <GiveMe as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::Output,
                            >(::ink::env::ReturnFlags::default(), &result)
                        }
                    }
                };
                ::core::result::Result::Ok(())
            }
        }
        impl ::ink::traits::ContractMessageDecoder for GiveMe {
            type Type = __ink_MessageDecoder;
        }
    };
    #[cfg(not(test))]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[cfg(not(test))]
        #[no_mangle]
        #[allow(clippy::nonminimal_bool)]
        fn deploy() {
            if !{
                false
                    || <GiveMe as ::ink::traits::DispatchableConstructorInfo<
                        {
                            <GiveMe as ::ink::traits::ContractDispatchableConstructors<
                                {
                                    <GiveMe as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                },
                            >>::IDS[0usize]
                        },
                    >>::PAYABLE
            } {
                ::ink::codegen::deny_payment::<
                    <GiveMe as ::ink::traits::ContractEnv>::Env,
                >()
                    .unwrap_or_else(|error| ::core::panicking::panic_display(&error))
            }
            ::ink::env::decode_input::<
                <GiveMe as ::ink::traits::ContractConstructorDecoder>::Type,
            >()
                .map_err(|_| ::ink::traits::DispatchError::CouldNotReadInput)
                .and_then(|decoder| {
                    <<GiveMe as ::ink::traits::ContractConstructorDecoder>::Type as ::ink::traits::ExecuteDispatchable>::execute_dispatchable(
                        decoder,
                    )
                })
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["dispatching ink! constructor failed: "],
                            &[::core::fmt::ArgumentV1::new_display(&error)],
                        ),
                    )
                })
        }
        #[cfg(not(test))]
        #[no_mangle]
        #[allow(clippy::nonminimal_bool)]
        fn call() {
            if !{
                false
                    || <GiveMe as ::ink::traits::DispatchableMessageInfo<
                        {
                            <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[0usize]
                        },
                    >>::PAYABLE
                    || <GiveMe as ::ink::traits::DispatchableMessageInfo<
                        {
                            <GiveMe as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <GiveMe as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[1usize]
                        },
                    >>::PAYABLE
            } {
                ::ink::codegen::deny_payment::<
                    <GiveMe as ::ink::traits::ContractEnv>::Env,
                >()
                    .unwrap_or_else(|error| ::core::panicking::panic_display(&error))
            }
            ::ink::env::decode_input::<
                <GiveMe as ::ink::traits::ContractMessageDecoder>::Type,
            >()
                .map_err(|_| ::ink::traits::DispatchError::CouldNotReadInput)
                .and_then(|decoder| {
                    <<GiveMe as ::ink::traits::ContractMessageDecoder>::Type as ::ink::traits::ExecuteDispatchable>::execute_dispatchable(
                        decoder,
                    )
                })
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["dispatching ink! message failed: "],
                            &[::core::fmt::ArgumentV1::new_display(&error)],
                        ),
                    )
                })
        }
    };
    const _: () = {
        use ::ink::codegen::{Env as _, StaticEnv as _};
        use ::ink::codegen::EmitEvent as _;
        const _: ::ink::codegen::utils::IsSameType<GiveMe> = ::ink::codegen::utils::IsSameType::<
            GiveMe,
        >::new();
        impl GiveMe {
            /// Creates a new instance of this contract.
            #[cfg(not(feature = "__ink_dylint_Constructor"))]
            pub fn new() -> Self {
                Self {}
            }
            /// Transfers `value` amount of tokens to the caller.
            ///
            /// # Errors
            ///
            /// - Panics in case the requested transfer exceeds the contract balance.
            /// - Panics in case the requested transfer would have brought this
            ///   contract's balance below the minimum balance (i.e. the chain's
            ///   existential deposit).
            /// - Panics in case the transfer failed for another reason.
            pub fn give_me(&mut self, value: Balance) {
                ::ink_env::debug_message(
                    &{
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                ::core::fmt::Arguments::new_v1(
                                                    &["requested value: "],
                                                    &[::core::fmt::ArgumentV1::new_display(&value)],
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ],
                            ),
                        );
                        res
                    },
                );
                ::ink_env::debug_message(
                    &{
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                ::core::fmt::Arguments::new_v1(
                                                    &["contract balance: "],
                                                    &[
                                                        ::core::fmt::ArgumentV1::new_display(&self.env().balance()),
                                                    ],
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ],
                            ),
                        );
                        res
                    },
                );
                self.env().emit_event(Received { value });
                if !(value <= self.env().balance()) {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(&["insufficient funds!"], &[]),
                    )
                }
                if self.env().transfer(self.env().caller(), value).is_err() {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "requested transfer failed. this can be the case if the contract does nothave sufficient free funds or if the transfer would have brought thecontract\'s balance below minimum balance.",
                            ],
                            &[],
                        ),
                    )
                }
            }
            /// Asserts that the token amount sent as payment with this call
            /// is exactly `10`. This method will fail otherwise, and the
            /// transaction would then be reverted.
            ///
            /// # Note
            ///
            /// The method needs to be annotated with `payable`; only then it is
            /// allowed to receive value as part of the call.
            pub fn was_it_ten(&self) {
                ::ink_env::debug_message(
                    &{
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "\n"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                ::core::fmt::Arguments::new_v1(
                                                    &["received payment: "],
                                                    &[
                                                        ::core::fmt::ArgumentV1::new_display(
                                                            &self.env().transferred_value(),
                                                        ),
                                                    ],
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ],
                            ),
                        );
                        res
                    },
                );
                self.env()
                    .emit_event(Received {
                        value: self.env().transferred_value(),
                    });
                if !(self.env().transferred_value() == 10) {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(&["payment was not ten"], &[]),
                    )
                }
            }
        }
        const _: () = {
            ::ink::codegen::utils::consume_type::<
                ::ink::codegen::DispatchInput<Balance>,
            >();
        };
    };
    const _: () = {
        /// The ink! smart contract's call builder.
        ///
        /// Implements the underlying on-chain calling of the ink! smart contract
        /// messages and trait implementations in a type safe way.
        #[repr(transparent)]
        pub struct CallBuilder {
            account_id: AccountId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CallBuilder {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CallBuilder",
                    "account_id",
                    &&self.account_id,
                )
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Encode for CallBuilder {
                fn encode_to<
                    __CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::scale::Encode::encode_to(&&self.account_id, __codec_dest_edqy)
                }
                fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                    ::scale::Encode::encode(&&self.account_id)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::scale::Encode::using_encoded(&&self.account_id, f)
                }
            }
            #[automatically_derived]
            impl ::scale::EncodeLike for CallBuilder {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Decode for CallBuilder {
                fn decode<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    ::core::result::Result::Ok(CallBuilder {
                        account_id: {
                            let __codec_res_edqy = <AccountId as ::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `CallBuilder::account_id`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        },
                    })
                }
            }
        };
        #[automatically_derived]
        impl ::core::hash::Hash for CallBuilder {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.account_id, state)
            }
        }
        impl ::core::marker::StructuralPartialEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CallBuilder {
            #[inline]
            fn eq(&self, other: &CallBuilder) -> bool {
                self.account_id == other.account_id
            }
        }
        impl ::core::marker::StructuralEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CallBuilder {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CallBuilder {
            #[inline]
            fn clone(&self) -> CallBuilder {
                CallBuilder {
                    account_id: ::core::clone::Clone::clone(&self.account_id),
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for CallBuilder {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "CallBuilder",
                                "contract_transfer::give_me",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "The ink! smart contract's call builder.",
                                "",
                                "Implements the underlying on-chain calling of the ink! smart contract",
                                "messages and trait implementations in a type safe way.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::named()
                                .field(|f| {
                                    f
                                        .ty::<AccountId>()
                                        .name("account_id")
                                        .type_name("AccountId")
                                        .docs(&[])
                                }),
                        )
                }
            }
        };
        const _: () = {
            impl ::ink::storage::traits::StorageLayout for CallBuilder {
                fn layout(
                    __key: &::ink::primitives::Key,
                ) -> ::ink::metadata::layout::Layout {
                    ::ink::metadata::layout::Layout::Struct(
                        ::ink::metadata::layout::StructLayout::new(
                            "CallBuilder",
                            [
                                ::ink::metadata::layout::FieldLayout::new(
                                    "account_id",
                                    <AccountId as ::ink::storage::traits::StorageLayout>::layout(
                                        __key,
                                    ),
                                ),
                            ],
                        ),
                    )
                }
            }
        };
        const _: () = {
            impl ::ink::codegen::ContractCallBuilder for GiveMe {
                type Type = CallBuilder;
            }
            impl ::ink::traits::ContractEnv for CallBuilder {
                type Env = <GiveMe as ::ink::traits::ContractEnv>::Env;
            }
        };
        impl ::ink::env::call::FromAccountId<Environment> for CallBuilder {
            #[inline]
            fn from_account_id(account_id: AccountId) -> Self {
                Self { account_id }
            }
        }
        impl ::ink::ToAccountId<Environment> for CallBuilder {
            #[inline]
            fn to_account_id(&self) -> AccountId {
                <AccountId as ::core::clone::Clone>::clone(&self.account_id)
            }
        }
        impl CallBuilder {
            /// Transfers `value` amount of tokens to the caller.
            ///
            /// # Errors
            ///
            /// - Panics in case the requested transfer exceeds the contract balance.
            /// - Panics in case the requested transfer would have brought this
            ///   contract's balance below the minimum balance (i.e. the chain's
            ///   existential deposit).
            /// - Panics in case the transfer failed for another reason.
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn give_me(
                &mut self,
                __ink_binding_0: Balance,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<Balance>,
                            ::ink::env::call::utils::EmptyArgumentList,
                        >,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call_type(
                        ::ink::env::call::Call::new()
                            .callee(::ink::ToAccountId::to_account_id(self)),
                    )
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                                ::ink::env::call::Selector::new([
                                    0x49_u8,
                                    0x9B_u8,
                                    0xB7_u8,
                                    0x39_u8,
                                ]),
                            )
                            .push_arg(__ink_binding_0),
                    )
                    .returns::<()>()
            }
            /// Asserts that the token amount sent as payment with this call
            /// is exactly `10`. This method will fail otherwise, and the
            /// transaction would then be reverted.
            ///
            /// # Note
            ///
            /// The method needs to be annotated with `payable`; only then it is
            /// allowed to receive value as part of the call.
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn was_it_ten(
                &self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call_type(
                        ::ink::env::call::Call::new()
                            .callee(::ink::ToAccountId::to_account_id(self)),
                    )
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0xCA_u8,
                                0xFE_u8,
                                0xBA_u8,
                                0xBE_u8,
                            ]),
                        ),
                    )
                    .returns::<()>()
            }
        }
    };
    /// No storage is needed for this simple contract.
    pub struct GiveMeRef {
        inner: <GiveMe as ::ink::codegen::ContractCallBuilder>::Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GiveMeRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "GiveMeRef",
                "inner",
                &&self.inner,
            )
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for GiveMeRef {
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.inner, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.inner)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.inner, f)
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for GiveMeRef {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for GiveMeRef {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(GiveMeRef {
                    inner: {
                        let __codec_res_edqy = <<GiveMe as ::ink::codegen::ContractCallBuilder>::Type as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `GiveMeRef::inner`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::hash::Hash for GiveMeRef {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    impl ::core::marker::StructuralPartialEq for GiveMeRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GiveMeRef {
        #[inline]
        fn eq(&self, other: &GiveMeRef) -> bool {
            self.inner == other.inner
        }
    }
    impl ::core::marker::StructuralEq for GiveMeRef {}
    #[automatically_derived]
    impl ::core::cmp::Eq for GiveMeRef {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <GiveMe as ::ink::codegen::ContractCallBuilder>::Type,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for GiveMeRef {
        #[inline]
        fn clone(&self) -> GiveMeRef {
            GiveMeRef {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for GiveMeRef {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "GiveMeRef",
                            "contract_transfer::give_me",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&["No storage is needed for this simple contract."])
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <GiveMe as ::ink::codegen::ContractCallBuilder>::Type,
                                    >()
                                    .name("inner")
                                    .type_name(
                                        "<GiveMe as::ink::codegen::ContractCallBuilder>::Type",
                                    )
                                    .docs(&[])
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for GiveMeRef {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "GiveMeRef",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "inner",
                                <<GiveMe as ::ink::codegen::ContractCallBuilder>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                        ],
                    ),
                )
            }
        }
    };
    const _: () = {
        impl ::ink::traits::ContractReference for GiveMe {
            type Type = GiveMeRef;
        }
        impl ::ink::traits::ContractEnv for GiveMeRef {
            type Env = <GiveMe as ::ink::traits::ContractEnv>::Env;
        }
    };
    impl GiveMeRef {
        /// Creates a new instance of this contract.
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new() -> ::ink::env::call::CreateBuilder<
            Environment,
            ::ink::env::call::utils::Unset<Hash>,
            ::ink::env::call::utils::Unset<u64>,
            ::ink::env::call::utils::Unset<Balance>,
            ::ink::env::call::utils::Set<
                ::ink::env::call::ExecutionInput<
                    ::ink::env::call::utils::EmptyArgumentList,
                >,
            >,
            ::ink::env::call::utils::Unset<::ink::env::call::state::Salt>,
            Self,
        > {
            ::ink::env::call::build_create::<Environment, Self>()
                .exec_input(
                    ::ink::env::call::ExecutionInput::new(
                        ::ink::env::call::Selector::new([
                            0x9B_u8,
                            0xAE_u8,
                            0x9D_u8,
                            0x5E_u8,
                        ]),
                    ),
                )
        }
        /// Transfers `value` amount of tokens to the caller.
        ///
        /// # Errors
        ///
        /// - Panics in case the requested transfer exceeds the contract balance.
        /// - Panics in case the requested transfer would have brought this
        ///   contract's balance below the minimum balance (i.e. the chain's
        ///   existential deposit).
        /// - Panics in case the transfer failed for another reason.
        #[inline]
        pub fn give_me(&mut self, value: Balance) {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .give_me(value)
                .fire()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["encountered error while calling ", "::", ": "],
                        &match (&"GiveMe", &"give_me", &error) {
                            args => {
                                [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                    ::core::fmt::ArgumentV1::new_debug(args.2),
                                ]
                            }
                        },
                    ),
                ))
        }
        /// Asserts that the token amount sent as payment with this call
        /// is exactly `10`. This method will fail otherwise, and the
        /// transaction would then be reverted.
        ///
        /// # Note
        ///
        /// The method needs to be annotated with `payable`; only then it is
        /// allowed to receive value as part of the call.
        #[inline]
        pub fn was_it_ten(&self) {
            <Self as ::ink::codegen::TraitCallBuilder>::call(self)
                .was_it_ten()
                .fire()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["encountered error while calling ", "::", ": "],
                        &match (&"GiveMe", &"was_it_ten", &error) {
                            args => {
                                [
                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                    ::core::fmt::ArgumentV1::new_debug(args.2),
                                ]
                            }
                        },
                    ),
                ))
        }
    }
    const _: () = {
        impl ::ink::codegen::TraitCallBuilder for GiveMeRef {
            type Builder = <GiveMe as ::ink::codegen::ContractCallBuilder>::Type;
            #[inline]
            fn call(&self) -> &Self::Builder {
                &self.inner
            }
            #[inline]
            fn call_mut(&mut self) -> &mut Self::Builder {
                &mut self.inner
            }
        }
    };
    impl ::ink::env::call::FromAccountId<Environment> for GiveMeRef {
        #[inline]
        fn from_account_id(account_id: AccountId) -> Self {
            Self {
                inner: <<GiveMe as ::ink::codegen::ContractCallBuilder>::Type as ::ink::env::call::FromAccountId<
                    Environment,
                >>::from_account_id(account_id),
            }
        }
    }
    impl ::ink::ToAccountId<Environment> for GiveMeRef {
        #[inline]
        fn to_account_id(&self) -> AccountId {
            <<GiveMe as ::ink::codegen::ContractCallBuilder>::Type as ::ink::ToAccountId<
                Environment,
            >>::to_account_id(&self.inner)
        }
    }
    #[cfg(feature = "std")]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[no_mangle]
        pub fn __ink_generate_metadata() -> ::ink::metadata::InkProject {
            let layout = ::ink::metadata::layout::Layout::Root(
                ::ink::metadata::layout::RootLayout::new(
                    <::ink::metadata::layout::LayoutKey as ::core::convert::From<
                        ::ink::primitives::Key,
                    >>::from(<GiveMe as ::ink::storage::traits::StorageKey>::KEY),
                    <GiveMe as ::ink::storage::traits::StorageLayout>::layout(
                        &<GiveMe as ::ink::storage::traits::StorageKey>::KEY,
                    ),
                ),
            );
            ::ink::metadata::layout::ValidateLayout::validate(&layout)
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["metadata ink! generation failed: "],
                            &[::core::fmt::ArgumentV1::new_display(&error)],
                        ),
                    )
                });
            ::ink::metadata::InkProject::new(
                layout,
                ::ink::metadata::ContractSpec::new()
                    .constructors([
                        ::ink::metadata::ConstructorSpec::from_label("new")
                            .selector([0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8])
                            .args([])
                            .payable(true)
                            .docs([" Creates a new instance of this contract."])
                            .done(),
                    ])
                    .messages([
                        ::ink::metadata::MessageSpec::from_label("give_me")
                            .selector([0x49_u8, 0x9B_u8, 0xB7_u8, 0x39_u8])
                            .args([
                                ::ink::metadata::MessageParamSpec::new("value")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            Balance,
                                            _,
                                        >(
                                            ::core::iter::IntoIterator::into_iter(["Balance"])
                                                .map(::core::convert::AsRef::as_ref),
                                        ),
                                    )
                                    .done(),
                            ])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::core::option::Option::None,
                                ),
                            )
                            .mutates(true)
                            .payable(false)
                            .docs([
                                " Transfers `value` amount of tokens to the caller.",
                                "",
                                " # Errors",
                                "",
                                " - Panics in case the requested transfer exceeds the contract balance.",
                                " - Panics in case the requested transfer would have brought this",
                                "   contract's balance below the minimum balance (i.e. the chain's",
                                "   existential deposit).",
                                " - Panics in case the transfer failed for another reason.",
                            ])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("was_it_ten")
                            .selector([0xCA_u8, 0xFE_u8, 0xBA_u8, 0xBE_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::core::option::Option::None,
                                ),
                            )
                            .mutates(false)
                            .payable(true)
                            .docs([
                                " Asserts that the token amount sent as payment with this call",
                                " is exactly `10`. This method will fail otherwise, and the",
                                " transaction would then be reverted.",
                                "",
                                " # Note",
                                "",
                                " The method needs to be annotated with `payable`; only then it is",
                                " allowed to receive value as part of the call.",
                            ])
                            .done(),
                    ])
                    .events([
                        ::ink::metadata::EventSpec::new("Received")
                            .args([
                                ::ink::metadata::EventParamSpec::new("value")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            Balance,
                                            _,
                                        >(
                                            ::core::iter::IntoIterator::into_iter(["Balance"])
                                                .map(::core::convert::AsRef::as_ref),
                                        ),
                                    )
                                    .indexed(false)
                                    .docs([])
                                    .done(),
                            ])
                            .docs([" Received docs"])
                            .done(),
                        ::ink::metadata::EventSpec::new("WrongReceived")
                            .args([
                                ::ink::metadata::EventParamSpec::new("value")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            Balance,
                                            _,
                                        >(
                                            ::core::iter::IntoIterator::into_iter(["Balance"])
                                                .map(::core::convert::AsRef::as_ref),
                                        ),
                                    )
                                    .indexed(false)
                                    .docs([])
                                    .done(),
                            ])
                            .docs([" Received docs"])
                            .done(),
                    ])
                    .docs([])
                    .done(),
            )
        }
    };
}
