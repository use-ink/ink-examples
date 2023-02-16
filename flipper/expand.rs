#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod flipper {
    impl ::ink::traits::ContractEnv for Flipper {
        type Env = ::ink::env::DefaultEnvironment;
    }
    type Environment = <Flipper as ::ink::traits::ContractEnv>::Env;
    type AccountId = <<Flipper as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
    type Balance = <<Flipper as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Balance;
    type Hash = <<Flipper as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Hash;
    type Timestamp = <<Flipper as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
    type BlockNumber = <<Flipper as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
    const _: () = {
        struct Check {
            salt: (),
            field_0: bool,
        }
    };
    #[cfg(not(feature = "__ink_dylint_Storage"))]
    pub struct Flipper {
        value: <bool as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<2054318728u32, ()>,
        >>::Type,
    }
    const _: () = {
        impl<
            __ink_generic_salt: ::ink::storage::traits::StorageKey,
        > ::ink::storage::traits::StorableHint<__ink_generic_salt> for Flipper {
            type Type = Flipper;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for Flipper {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for Flipper {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Flipper {
                    value: <<bool as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<2054318728u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(__input)?,
                })
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __dest: &mut __ink_O,
            ) {
                match self {
                    Flipper { value: __binding_0 } => {
                        ::ink::storage::traits::Storable::encode(__binding_0, __dest);
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Flipper {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Flipper", "flipper::flipper"))
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <bool as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<2054318728u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("value")
                                    .type_name(
                                        "<bool as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<2054318728u32, ()>,>>::Type",
                                    )
                                    .docs(&[])
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for Flipper {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "Flipper",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "value",
                                <<bool as ::ink::storage::traits::AutoStorableHint<
                                    ::ink::storage::traits::ManualKey<2054318728u32, ()>,
                                >>::Type as ::ink::storage::traits::StorageLayout>::layout(
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
        impl ::ink::traits::ContractName for Flipper {
            const NAME: &'static str = "Flipper";
        }
    };
    const _: () = {
        impl<'a> ::ink::codegen::Env for &'a Flipper {
            type EnvAccess = ::ink::EnvAccess<
                'a,
                <Flipper as ::ink::traits::ContractEnv>::Env,
            >;
            fn env(self) -> Self::EnvAccess {
                <<Self as ::ink::codegen::Env>::EnvAccess as ::core::default::Default>::default()
            }
        }
        impl<'a> ::ink::codegen::StaticEnv for Flipper {
            type EnvAccess = ::ink::EnvAccess<
                'static,
                <Flipper as ::ink::traits::ContractEnv>::Env,
            >;
            fn env() -> Self::EnvAccess {
                <<Self as ::ink::codegen::StaticEnv>::EnvAccess as ::core::default::Default>::default()
            }
        }
    };
    const _: () = {
        #[allow(unused_imports)]
        use ::ink::codegen::{Env as _, StaticEnv as _};
    };
    impl ::ink::traits::ContractAmountDispatchables for Flipper {
        const MESSAGES: ::core::primitive::usize = 2usize;
        const CONSTRUCTORS: ::core::primitive::usize = 2usize;
    }
    impl ::ink::traits::ContractDispatchableMessages<
        { <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES },
    > for Flipper {
        const IDS: [::core::primitive::u32; <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES] = [
            0x633AA551_u32,
            0x2F865BD9_u32,
        ];
    }
    impl ::ink::traits::ContractDispatchableConstructors<
        { <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS },
    > for Flipper {
        const IDS: [::core::primitive::u32; <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS] = [
            0x9BAE9D5E_u32,
            0xED4B9D1B_u32,
        ];
    }
    impl ::ink::traits::DispatchableConstructorInfo<0x9BAE9D5E_u32> for Flipper {
        type Input = bool;
        type Output = Self;
        type Storage = Flipper;
        const CALLABLE: fn(Self::Input) -> Self::Storage = |__ink_binding_0| {
            Flipper::new(__ink_binding_0)
        };
        const PAYABLE: ::core::primitive::bool = false;
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x9B_u8,
            0xAE_u8,
            0x9D_u8,
            0x5E_u8,
        ];
        const LABEL: &'static ::core::primitive::str = "new";
    }
    impl ::ink::traits::DispatchableConstructorInfo<0xED4B9D1B_u32> for Flipper {
        type Input = ();
        type Output = Self;
        type Storage = Flipper;
        const CALLABLE: fn(Self::Input) -> Self::Storage = |_| { Flipper::default() };
        const PAYABLE: ::core::primitive::bool = false;
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0xED_u8,
            0x4B_u8,
            0x9D_u8,
            0x1B_u8,
        ];
        const LABEL: &'static ::core::primitive::str = "default";
    }
    impl ::ink::traits::DispatchableMessageInfo<0x633AA551_u32> for Flipper {
        type Input = ();
        type Output = ();
        type Storage = Flipper;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { Flipper::flip(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x63_u8,
            0x3A_u8,
            0xA5_u8,
            0x51_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "flip";
    }
    impl ::ink::traits::DispatchableMessageInfo<0x2F865BD9_u32> for Flipper {
        type Input = ();
        type Output = bool;
        type Storage = Flipper;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { Flipper::get(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x2F_u8,
            0x86_u8,
            0x5B_u8,
            0xD9_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = false;
        const LABEL: &'static ::core::primitive::str = "get";
    }
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_ConstructorDecoder {
            Constructor0(
                <Flipper as ::ink::traits::DispatchableConstructorInfo<
                    {
                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                            {
                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                            },
                        >>::IDS[0usize]
                    },
                >>::Input,
            ),
            Constructor1(
                <Flipper as ::ink::traits::DispatchableConstructorInfo<
                    {
                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                            {
                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                            },
                        >>::IDS[1usize]
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
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::traits::DispatchError::InvalidSelector)?
                {
                    <Flipper as ::ink::traits::DispatchableConstructorInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                },
                            >>::IDS[0usize]
                        },
                    >>::SELECTOR => {
                        ::core::result::Result::Ok(
                            Self::Constructor0(
                                <<Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
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
                    <Flipper as ::ink::traits::DispatchableConstructorInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                },
                            >>::IDS[1usize]
                        },
                    >>::SELECTOR => {
                        ::core::result::Result::Ok(
                            Self::Constructor1(
                                <<Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
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
                                || <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<Flipper as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Flipper as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Flipper as ::ink::traits::DispatchableConstructorInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                    },
                                >>::IDS[0usize]
                            },
                        >>::Output = <Flipper as ::ink::traits::DispatchableConstructorInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                    },
                                >>::IDS[0usize]
                            },
                        >>::CALLABLE(input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
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
                                <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::Output,
                            >(
                                ::ink::env::ReturnFlags::default().set_reverted(true),
                                &result,
                            )
                        }
                        ::ink::codegen::execute_constructor::<
                            Flipper,
                            _,
                            _,
                        >(move || {
                            <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::CALLABLE(input)
                        })
                    }
                    Self::Constructor1(input) => {
                        if {
                            false
                                || <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<Flipper as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Flipper as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Flipper as ::ink::traits::DispatchableConstructorInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                    },
                                >>::IDS[1usize]
                            },
                        >>::Output = <Flipper as ::ink::traits::DispatchableConstructorInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                    },
                                >>::IDS[1usize]
                            },
                        >>::CALLABLE(input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
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
                                <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::Output,
                            >(
                                ::ink::env::ReturnFlags::default().set_reverted(true),
                                &result,
                            )
                        }
                        ::ink::codegen::execute_constructor::<
                            Flipper,
                            _,
                            _,
                        >(move || {
                            <Flipper as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::CALLABLE(input)
                        })
                    }
                }
            }
        }
        impl ::ink::traits::ContractConstructorDecoder for Flipper {
            type Type = __ink_ConstructorDecoder;
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_MessageDecoder {
            Message0(
                <Flipper as ::ink::traits::DispatchableMessageInfo<
                    {
                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                            {
                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                            },
                        >>::IDS[0usize]
                    },
                >>::Input,
            ),
            Message1(
                <Flipper as ::ink::traits::DispatchableMessageInfo<
                    {
                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                            {
                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::traits::DispatchError::InvalidSelector)?
                {
                    <Flipper as ::ink::traits::DispatchableMessageInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[0usize]
                        },
                    >>::SELECTOR => {
                        ::core::result::Result::Ok(
                            Self::Message0(
                                <<Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                    <Flipper as ::ink::traits::DispatchableMessageInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[1usize]
                        },
                    >>::SELECTOR => {
                        ::core::result::Result::Ok(
                            Self::Message1(
                                <<Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
        fn push_contract(contract: ::core::mem::ManuallyDrop<Flipper>, mutates: bool) {
            if mutates {
                ::ink::env::set_contract_storage::<
                    ::ink::primitives::Key,
                    Flipper,
                >(&<Flipper as ::ink::storage::traits::StorageKey>::KEY, &contract);
            }
        }
        impl ::ink::traits::ExecuteDispatchable for __ink_MessageDecoder {
            #[allow(clippy::nonminimal_bool, clippy::let_unit_value)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::traits::DispatchError> {
                let key = <Flipper as ::ink::storage::traits::StorageKey>::KEY;
                let mut contract: ::core::mem::ManuallyDrop<Flipper> = ::core::mem::ManuallyDrop::new(
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
                                || <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<Flipper as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Flipper as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Flipper as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[0usize]
                            },
                        >>::Output = <Flipper as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[0usize]
                            },
                        >>::CALLABLE(&mut contract, input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                            <Flipper as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::MUTATES,
                        );
                        if ::core::any::TypeId::of::<
                            <Flipper as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::Output,
                        >() != ::core::any::TypeId::of::<()>()
                        {
                            ::ink::env::return_value::<
                                <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                || <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<Flipper as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Flipper as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Flipper as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[1usize]
                            },
                        >>::Output = <Flipper as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Flipper as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[1usize]
                            },
                        >>::CALLABLE(&mut contract, input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                            <Flipper as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::MUTATES,
                        );
                        if ::core::any::TypeId::of::<
                            <Flipper as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Flipper as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::Output,
                        >() != ::core::any::TypeId::of::<()>()
                        {
                            ::ink::env::return_value::<
                                <Flipper as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Flipper as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
        impl ::ink::traits::ContractMessageDecoder for Flipper {
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
                    || <Flipper as ::ink::traits::DispatchableConstructorInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                },
                            >>::IDS[0usize]
                        },
                    >>::PAYABLE
                    || <Flipper as ::ink::traits::DispatchableConstructorInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableConstructors<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                },
                            >>::IDS[1usize]
                        },
                    >>::PAYABLE
            } {
                ::ink::codegen::deny_payment::<
                    <Flipper as ::ink::traits::ContractEnv>::Env,
                >()
                    .unwrap_or_else(|error| ::core::panicking::panic_display(&error))
            }
            ::ink::env::decode_input::<
                <Flipper as ::ink::traits::ContractConstructorDecoder>::Type,
            >()
                .map_err(|_| ::ink::traits::DispatchError::CouldNotReadInput)
                .and_then(|decoder| {
                    <<Flipper as ::ink::traits::ContractConstructorDecoder>::Type as ::ink::traits::ExecuteDispatchable>::execute_dispatchable(
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
                    || <Flipper as ::ink::traits::DispatchableMessageInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[0usize]
                        },
                    >>::PAYABLE
                    || <Flipper as ::ink::traits::DispatchableMessageInfo<
                        {
                            <Flipper as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <Flipper as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[1usize]
                        },
                    >>::PAYABLE
            } {
                ::ink::codegen::deny_payment::<
                    <Flipper as ::ink::traits::ContractEnv>::Env,
                >()
                    .unwrap_or_else(|error| ::core::panicking::panic_display(&error))
            }
            ::ink::env::decode_input::<
                <Flipper as ::ink::traits::ContractMessageDecoder>::Type,
            >()
                .map_err(|_| ::ink::traits::DispatchError::CouldNotReadInput)
                .and_then(|decoder| {
                    <<Flipper as ::ink::traits::ContractMessageDecoder>::Type as ::ink::traits::ExecuteDispatchable>::execute_dispatchable(
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
        const _: ::ink::codegen::utils::IsSameType<Flipper> = ::ink::codegen::utils::IsSameType::<
            Flipper,
        >::new();
        impl Flipper {
            /// Creates a new flipper smart contract initialized with the given value.
            #[cfg(not(feature = "__ink_dylint_Constructor"))]
            pub fn new(init_value: bool) -> Self {
                Self { value: init_value }
            }
            /// Creates a new flipper smart contract initialized to `false`.
            #[cfg(not(feature = "__ink_dylint_Constructor"))]
            pub fn default() -> Self {
                Self::new(Default::default())
            }
            /// Flips the current value of the Flipper's boolean.
            pub fn flip(&mut self) {
                self.value = !self.value;
            }
            /// Returns the current value of the Flipper's boolean.
            pub fn get(&self) -> bool {
                self.value
            }
        }
        const _: () = {
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<bool>>();
            ::ink::codegen::utils::consume_type::<
                ::ink::codegen::DispatchOutput<bool>,
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
                        .path(::scale_info::Path::new("CallBuilder", "flipper::flipper"))
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
            impl ::ink::codegen::ContractCallBuilder for Flipper {
                type Type = CallBuilder;
            }
            impl ::ink::traits::ContractEnv for CallBuilder {
                type Env = <Flipper as ::ink::traits::ContractEnv>::Env;
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
            /// Flips the current value of the Flipper's boolean.
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn flip(
                &mut self,
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
                                0x63_u8,
                                0x3A_u8,
                                0xA5_u8,
                                0x51_u8,
                            ]),
                        ),
                    )
                    .returns::<()>()
            }
            /// Returns the current value of the Flipper's boolean.
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn get(
                &self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<bool>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call_type(
                        ::ink::env::call::Call::new()
                            .callee(::ink::ToAccountId::to_account_id(self)),
                    )
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0x2F_u8,
                                0x86_u8,
                                0x5B_u8,
                                0xD9_u8,
                            ]),
                        ),
                    )
                    .returns::<bool>()
            }
        }
    };
    pub struct FlipperRef {
        inner: <Flipper as ::ink::codegen::ContractCallBuilder>::Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FlipperRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "FlipperRef",
                "inner",
                &&self.inner,
            )
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for FlipperRef {
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
        impl ::scale::EncodeLike for FlipperRef {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for FlipperRef {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(FlipperRef {
                    inner: {
                        let __codec_res_edqy = <<Flipper as ::ink::codegen::ContractCallBuilder>::Type as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `FlipperRef::inner`"),
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
    impl ::core::hash::Hash for FlipperRef {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    impl ::core::marker::StructuralPartialEq for FlipperRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FlipperRef {
        #[inline]
        fn eq(&self, other: &FlipperRef) -> bool {
            self.inner == other.inner
        }
    }
    impl ::core::marker::StructuralEq for FlipperRef {}
    #[automatically_derived]
    impl ::core::cmp::Eq for FlipperRef {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Flipper as ::ink::codegen::ContractCallBuilder>::Type,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FlipperRef {
        #[inline]
        fn clone(&self) -> FlipperRef {
            FlipperRef {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for FlipperRef {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("FlipperRef", "flipper::flipper"))
                    .type_params(::alloc::vec::Vec::new())
                    .docs(&[])
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <Flipper as ::ink::codegen::ContractCallBuilder>::Type,
                                    >()
                                    .name("inner")
                                    .type_name(
                                        "<Flipper as::ink::codegen::ContractCallBuilder>::Type",
                                    )
                                    .docs(&[])
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for FlipperRef {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "FlipperRef",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "inner",
                                <<Flipper as ::ink::codegen::ContractCallBuilder>::Type as ::ink::storage::traits::StorageLayout>::layout(
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
        impl ::ink::traits::ContractReference for Flipper {
            type Type = FlipperRef;
        }
        impl ::ink::traits::ContractEnv for FlipperRef {
            type Env = <Flipper as ::ink::traits::ContractEnv>::Env;
        }
    };
    impl FlipperRef {
        /// Creates a new flipper smart contract initialized with the given value.
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new(
            __ink_binding_0: bool,
        ) -> ::ink::env::call::CreateBuilder<
            Environment,
            ::ink::env::call::utils::Unset<Hash>,
            ::ink::env::call::utils::Unset<u64>,
            ::ink::env::call::utils::Unset<Balance>,
            ::ink::env::call::utils::Set<
                ::ink::env::call::ExecutionInput<
                    ::ink::env::call::utils::ArgumentList<
                        ::ink::env::call::utils::Argument<bool>,
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
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
                        )
                        .push_arg(__ink_binding_0),
                )
        }
        /// Creates a new flipper smart contract initialized to `false`.
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn default() -> ::ink::env::call::CreateBuilder<
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
                            0xED_u8,
                            0x4B_u8,
                            0x9D_u8,
                            0x1B_u8,
                        ]),
                    ),
                )
        }
        /// Flips the current value of the Flipper's boolean.
        #[inline]
        pub fn flip(&mut self) {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .flip()
                .fire()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["encountered error while calling ", "::", ": "],
                        &match (&"Flipper", &"flip", &error) {
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
        /// Returns the current value of the Flipper's boolean.
        #[inline]
        pub fn get(&self) -> bool {
            <Self as ::ink::codegen::TraitCallBuilder>::call(self)
                .get()
                .fire()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["encountered error while calling ", "::", ": "],
                        &match (&"Flipper", &"get", &error) {
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
        impl ::ink::codegen::TraitCallBuilder for FlipperRef {
            type Builder = <Flipper as ::ink::codegen::ContractCallBuilder>::Type;
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
    impl ::ink::env::call::FromAccountId<Environment> for FlipperRef {
        #[inline]
        fn from_account_id(account_id: AccountId) -> Self {
            Self {
                inner: <<Flipper as ::ink::codegen::ContractCallBuilder>::Type as ::ink::env::call::FromAccountId<
                    Environment,
                >>::from_account_id(account_id),
            }
        }
    }
    impl ::ink::ToAccountId<Environment> for FlipperRef {
        #[inline]
        fn to_account_id(&self) -> AccountId {
            <<Flipper as ::ink::codegen::ContractCallBuilder>::Type as ::ink::ToAccountId<
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
                    >>::from(<Flipper as ::ink::storage::traits::StorageKey>::KEY),
                    <Flipper as ::ink::storage::traits::StorageLayout>::layout(
                        &<Flipper as ::ink::storage::traits::StorageKey>::KEY,
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
                            .args([
                                ::ink::metadata::MessageParamSpec::new("init_value")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            bool,
                                            _,
                                        >(
                                            ::core::iter::IntoIterator::into_iter(["bool"])
                                                .map(::core::convert::AsRef::as_ref),
                                        ),
                                    )
                                    .done(),
                            ])
                            .payable(false)
                            .docs([
                                " Creates a new flipper smart contract initialized with the given value.",
                            ])
                            .done(),
                        ::ink::metadata::ConstructorSpec::from_label("default")
                            .selector([0xED_u8, 0x4B_u8, 0x9D_u8, 0x1B_u8])
                            .args([])
                            .payable(false)
                            .docs([
                                " Creates a new flipper smart contract initialized to `false`.",
                            ])
                            .done(),
                    ])
                    .messages([
                        ::ink::metadata::MessageSpec::from_label("flip")
                            .selector([0x63_u8, 0x3A_u8, 0xA5_u8, 0x51_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::core::option::Option::None,
                                ),
                            )
                            .mutates(true)
                            .payable(false)
                            .docs([" Flips the current value of the Flipper's boolean."])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("get")
                            .selector([0x2F_u8, 0x86_u8, 0x5B_u8, 0xD9_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        bool,
                                        _,
                                    >(
                                        ::core::iter::IntoIterator::into_iter(["bool"])
                                            .map(::core::convert::AsRef::as_ref),
                                    ),
                                ),
                            )
                            .mutates(false)
                            .payable(false)
                            .docs([
                                " Returns the current value of the Flipper's boolean.",
                            ])
                            .done(),
                    ])
                    .events([])
                    .docs([])
                    .done(),
            )
        }
    };
}
