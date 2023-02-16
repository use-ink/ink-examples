#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use self::accumulator::{Accumulator, AccumulatorRef};
pub mod accumulator {
    impl ::ink::traits::ContractEnv for Accumulator {
        type Env = ::ink::env::DefaultEnvironment;
    }
    type Environment = <Accumulator as ::ink::traits::ContractEnv>::Env;
    type AccountId = <<Accumulator as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
    type Balance = <<Accumulator as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Balance;
    type Hash = <<Accumulator as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Hash;
    type Timestamp = <<Accumulator as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
    type BlockNumber = <<Accumulator as ::ink::traits::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
    const _: () = {
        struct Check {
            salt: (),
            field_0: i32,
        }
    };
    /// Holds a simple `i32` value that can be incremented and decremented.
    #[cfg(not(feature = "__ink_dylint_Storage"))]
    pub struct Accumulator {
        value: <i32 as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<774319373u32, ()>,
        >>::Type,
    }
    const _: () = {
        impl<
            __ink_generic_salt: ::ink::storage::traits::StorageKey,
        > ::ink::storage::traits::StorableHint<__ink_generic_salt> for Accumulator {
            type Type = Accumulator;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for Accumulator {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for Accumulator {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Accumulator {
                    value: <<i32 as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<774319373u32, ()>,
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
                    Accumulator { value: __binding_0 } => {
                        ::ink::storage::traits::Storable::encode(__binding_0, __dest);
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Accumulator {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "Accumulator",
                            "accumulator::accumulator",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(
                        &[
                            "Holds a simple `i32` value that can be incremented and decremented.",
                        ],
                    )
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <i32 as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<774319373u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("value")
                                    .type_name(
                                        "<i32 as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<774319373u32, ()>,>>::Type",
                                    )
                                    .docs(&[])
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for Accumulator {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "Accumulator",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "value",
                                <<i32 as ::ink::storage::traits::AutoStorableHint<
                                    ::ink::storage::traits::ManualKey<774319373u32, ()>,
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
        impl ::ink::traits::ContractName for Accumulator {
            const NAME: &'static str = "Accumulator";
        }
    };
    const _: () = {
        impl<'a> ::ink::codegen::Env for &'a Accumulator {
            type EnvAccess = ::ink::EnvAccess<
                'a,
                <Accumulator as ::ink::traits::ContractEnv>::Env,
            >;
            fn env(self) -> Self::EnvAccess {
                <<Self as ::ink::codegen::Env>::EnvAccess as ::core::default::Default>::default()
            }
        }
        impl<'a> ::ink::codegen::StaticEnv for Accumulator {
            type EnvAccess = ::ink::EnvAccess<
                'static,
                <Accumulator as ::ink::traits::ContractEnv>::Env,
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
    impl ::ink::traits::ContractAmountDispatchables for Accumulator {
        const MESSAGES: ::core::primitive::usize = 2usize;
        const CONSTRUCTORS: ::core::primitive::usize = 1usize;
    }
    impl ::ink::traits::ContractDispatchableMessages<
        { <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES },
    > for Accumulator {
        const IDS: [::core::primitive::u32; <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES] = [
            0x1D32619F_u32,
            0x2F865BD9_u32,
        ];
    }
    impl ::ink::traits::ContractDispatchableConstructors<
        { <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS },
    > for Accumulator {
        const IDS: [::core::primitive::u32; <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS] = [
            0x9BAE9D5E_u32,
        ];
    }
    impl ::ink::traits::DispatchableConstructorInfo<0x9BAE9D5E_u32> for Accumulator {
        type Input = i32;
        type Storage = Accumulator;
        const CALLABLE: fn(Self::Input) -> Self::Storage = |__ink_binding_0| {
            Accumulator::new(__ink_binding_0)
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
    impl ::ink::traits::DispatchableMessageInfo<0x1D32619F_u32> for Accumulator {
        type Input = i32;
        type Output = ();
        type Storage = Accumulator;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            __ink_binding_0|
        { Accumulator::inc(storage, __ink_binding_0) };
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x1D_u8,
            0x32_u8,
            0x61_u8,
            0x9F_u8,
        ];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "inc";
    }
    impl ::ink::traits::DispatchableMessageInfo<0x2F865BD9_u32> for Accumulator {
        type Input = ();
        type Output = i32;
        type Storage = Accumulator;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { Accumulator::get(storage) };
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
                <Accumulator as ::ink::traits::DispatchableConstructorInfo<
                    {
                        <Accumulator as ::ink::traits::ContractDispatchableConstructors<
                            {
                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
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
                const CONSTRUCTOR_0: [::core::primitive::u8; 4usize] = <Accumulator as ::ink::traits::DispatchableConstructorInfo<
                    {
                        <Accumulator as ::ink::traits::ContractDispatchableConstructors<
                            {
                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
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
                                <<Accumulator as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
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
                                || <Accumulator as ::ink::traits::DispatchableConstructorInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableConstructors<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<Accumulator as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Accumulator as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        ::ink::codegen::execute_constructor::<
                            Accumulator,
                            _,
                            _,
                        >(move || {
                            <Accumulator as ::ink::traits::DispatchableConstructorInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableConstructors<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::CALLABLE(input)
                        })
                    }
                }
            }
        }
        impl ::ink::traits::ContractConstructorDecoder for Accumulator {
            type Type = __ink_ConstructorDecoder;
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_MessageDecoder {
            Message0(
                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                    {
                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                            {
                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                            },
                        >>::IDS[0usize]
                    },
                >>::Input,
            ),
            Message1(
                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                    {
                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                            {
                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                const MESSAGE_0: [::core::primitive::u8; 4usize] = <Accumulator as ::ink::traits::DispatchableMessageInfo<
                    {
                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                            {
                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                            },
                        >>::IDS[0usize]
                    },
                >>::SELECTOR;
                const MESSAGE_1: [::core::primitive::u8; 4usize] = <Accumulator as ::ink::traits::DispatchableMessageInfo<
                    {
                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                            {
                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                <<Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                <<Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
        fn push_contract(
            contract: ::core::mem::ManuallyDrop<Accumulator>,
            mutates: bool,
        ) {
            if mutates {
                ::ink::env::set_contract_storage::<
                    ::ink::primitives::Key,
                    Accumulator,
                >(&<Accumulator as ::ink::storage::traits::StorageKey>::KEY, &contract);
            }
        }
        impl ::ink::traits::ExecuteDispatchable for __ink_MessageDecoder {
            #[allow(clippy::nonminimal_bool, clippy::let_unit_value)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::traits::DispatchError> {
                let key = <Accumulator as ::ink::storage::traits::StorageKey>::KEY;
                let mut contract: ::core::mem::ManuallyDrop<Accumulator> = ::core::mem::ManuallyDrop::new(
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
                                || <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<Accumulator as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Accumulator as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Accumulator as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[0usize]
                            },
                        >>::Output = <Accumulator as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[0usize]
                            },
                        >>::CALLABLE(&mut contract, input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                            <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::MUTATES,
                        );
                        if ::core::any::TypeId::of::<
                            <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[0usize]
                                },
                            >>::Output,
                        >() != ::core::any::TypeId::of::<()>()
                        {
                            ::ink::env::return_value::<
                                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                || <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[0usize]
                                    },
                                >>::PAYABLE
                                || <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                            },
                                        >>::IDS[1usize]
                                    },
                                >>::PAYABLE
                        }
                            && !<Accumulator as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Accumulator as ::ink::traits::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Accumulator as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[1usize]
                            },
                        >>::Output = <Accumulator as ::ink::traits::DispatchableMessageInfo<
                            {
                                <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                    {
                                        <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                    },
                                >>::IDS[1usize]
                            },
                        >>::CALLABLE(&mut contract, input);
                        let failure = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
                            <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::MUTATES,
                        );
                        if ::core::any::TypeId::of::<
                            <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                {
                                    <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                        {
                                            <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                        },
                                    >>::IDS[1usize]
                                },
                            >>::Output,
                        >() != ::core::any::TypeId::of::<()>()
                        {
                            ::ink::env::return_value::<
                                <Accumulator as ::ink::traits::DispatchableMessageInfo<
                                    {
                                        <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                            {
                                                <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
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
        impl ::ink::traits::ContractMessageDecoder for Accumulator {
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
                    || <Accumulator as ::ink::traits::DispatchableConstructorInfo<
                        {
                            <Accumulator as ::ink::traits::ContractDispatchableConstructors<
                                {
                                    <Accumulator as ::ink::traits::ContractAmountDispatchables>::CONSTRUCTORS
                                },
                            >>::IDS[0usize]
                        },
                    >>::PAYABLE
            } {
                ::ink::codegen::deny_payment::<
                    <Accumulator as ::ink::traits::ContractEnv>::Env,
                >()
                    .unwrap_or_else(|error| ::core::panicking::panic_display(&error))
            }
            ::ink::env::decode_input::<
                <Accumulator as ::ink::traits::ContractConstructorDecoder>::Type,
            >()
                .map_err(|_| ::ink::traits::DispatchError::CouldNotReadInput)
                .and_then(|decoder| {
                    <<Accumulator as ::ink::traits::ContractConstructorDecoder>::Type as ::ink::traits::ExecuteDispatchable>::execute_dispatchable(
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
                    || <Accumulator as ::ink::traits::DispatchableMessageInfo<
                        {
                            <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[0usize]
                        },
                    >>::PAYABLE
                    || <Accumulator as ::ink::traits::DispatchableMessageInfo<
                        {
                            <Accumulator as ::ink::traits::ContractDispatchableMessages<
                                {
                                    <Accumulator as ::ink::traits::ContractAmountDispatchables>::MESSAGES
                                },
                            >>::IDS[1usize]
                        },
                    >>::PAYABLE
            } {
                ::ink::codegen::deny_payment::<
                    <Accumulator as ::ink::traits::ContractEnv>::Env,
                >()
                    .unwrap_or_else(|error| ::core::panicking::panic_display(&error))
            }
            ::ink::env::decode_input::<
                <Accumulator as ::ink::traits::ContractMessageDecoder>::Type,
            >()
                .map_err(|_| ::ink::traits::DispatchError::CouldNotReadInput)
                .and_then(|decoder| {
                    <<Accumulator as ::ink::traits::ContractMessageDecoder>::Type as ::ink::traits::ExecuteDispatchable>::execute_dispatchable(
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
        const _: ::ink::codegen::utils::IsSameType<Accumulator> = ::ink::codegen::utils::IsSameType::<
            Accumulator,
        >::new();
        impl Accumulator {
            /// Initializes the value to the initial value.
            #[cfg(not(feature = "__ink_dylint_Constructor"))]
            pub fn new(init_value: i32) -> Self {
                Self { value: init_value }
            }
            /// Mutates the internal value.
            pub fn inc(&mut self, by: i32) {
                self.value += by;
            }
            /// Returns the current state.
            pub fn get(&self) -> i32 {
                self.value
            }
        }
        const _: () = {
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<i32>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<i32>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<i32>>();
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
                                "accumulator::accumulator",
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
            impl ::ink::codegen::ContractCallBuilder for Accumulator {
                type Type = CallBuilder;
            }
            impl ::ink::traits::ContractEnv for CallBuilder {
                type Env = <Accumulator as ::ink::traits::ContractEnv>::Env;
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
            /// Mutates the internal value.
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn inc(
                &mut self,
                __ink_binding_0: i32,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<i32>,
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
                                    0x1D_u8,
                                    0x32_u8,
                                    0x61_u8,
                                    0x9F_u8,
                                ]),
                            )
                            .push_arg(__ink_binding_0),
                    )
                    .returns::<()>()
            }
            /// Returns the current state.
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
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<i32>>,
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
                    .returns::<i32>()
            }
        }
    };
    /// Holds a simple `i32` value that can be incremented and decremented.
    pub struct AccumulatorRef {
        inner: <Accumulator as ::ink::codegen::ContractCallBuilder>::Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AccumulatorRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "AccumulatorRef",
                "inner",
                &&self.inner,
            )
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for AccumulatorRef {
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
        impl ::scale::EncodeLike for AccumulatorRef {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for AccumulatorRef {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(AccumulatorRef {
                    inner: {
                        let __codec_res_edqy = <<Accumulator as ::ink::codegen::ContractCallBuilder>::Type as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `AccumulatorRef::inner`"),
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
    impl ::core::hash::Hash for AccumulatorRef {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    impl ::core::marker::StructuralPartialEq for AccumulatorRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AccumulatorRef {
        #[inline]
        fn eq(&self, other: &AccumulatorRef) -> bool {
            self.inner == other.inner
        }
    }
    impl ::core::marker::StructuralEq for AccumulatorRef {}
    #[automatically_derived]
    impl ::core::cmp::Eq for AccumulatorRef {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Accumulator as ::ink::codegen::ContractCallBuilder>::Type,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AccumulatorRef {
        #[inline]
        fn clone(&self) -> AccumulatorRef {
            AccumulatorRef {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for AccumulatorRef {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new(
                            "AccumulatorRef",
                            "accumulator::accumulator",
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(
                        &[
                            "Holds a simple `i32` value that can be incremented and decremented.",
                        ],
                    )
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <Accumulator as ::ink::codegen::ContractCallBuilder>::Type,
                                    >()
                                    .name("inner")
                                    .type_name(
                                        "<Accumulator as::ink::codegen::ContractCallBuilder>::Type",
                                    )
                                    .docs(&[])
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for AccumulatorRef {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "AccumulatorRef",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "inner",
                                <<Accumulator as ::ink::codegen::ContractCallBuilder>::Type as ::ink::storage::traits::StorageLayout>::layout(
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
        impl ::ink::traits::ContractReference for Accumulator {
            type Type = AccumulatorRef;
        }
        impl ::ink::traits::ContractEnv for AccumulatorRef {
            type Env = <Accumulator as ::ink::traits::ContractEnv>::Env;
        }
    };
    impl AccumulatorRef {
        /// Initializes the value to the initial value.
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new(
            __ink_binding_0: i32,
        ) -> ::ink::env::call::CreateBuilder<
            Environment,
            ::ink::env::call::utils::Unset<Hash>,
            ::ink::env::call::utils::Unset<u64>,
            ::ink::env::call::utils::Unset<Balance>,
            ::ink::env::call::utils::Set<
                ::ink::env::call::ExecutionInput<
                    ::ink::env::call::utils::ArgumentList<
                        ::ink::env::call::utils::Argument<i32>,
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
        /// Mutates the internal value.
        #[inline]
        pub fn inc(&mut self, by: i32) {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .inc(by)
                .fire()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["encountered error while calling ", "::", ": "],
                        &match (&"Accumulator", &"inc", &error) {
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
        /// Returns the current state.
        #[inline]
        pub fn get(&self) -> i32 {
            <Self as ::ink::codegen::TraitCallBuilder>::call(self)
                .get()
                .fire()
                .unwrap_or_else(|error| ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["encountered error while calling ", "::", ": "],
                        &match (&"Accumulator", &"get", &error) {
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
        impl ::ink::codegen::TraitCallBuilder for AccumulatorRef {
            type Builder = <Accumulator as ::ink::codegen::ContractCallBuilder>::Type;
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
    impl ::ink::env::call::FromAccountId<Environment> for AccumulatorRef {
        #[inline]
        fn from_account_id(account_id: AccountId) -> Self {
            Self {
                inner: <<Accumulator as ::ink::codegen::ContractCallBuilder>::Type as ::ink::env::call::FromAccountId<
                    Environment,
                >>::from_account_id(account_id),
            }
        }
    }
    impl ::ink::ToAccountId<Environment> for AccumulatorRef {
        #[inline]
        fn to_account_id(&self) -> AccountId {
            <<Accumulator as ::ink::codegen::ContractCallBuilder>::Type as ::ink::ToAccountId<
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
                    >>::from(<Accumulator as ::ink::storage::traits::StorageKey>::KEY),
                    <Accumulator as ::ink::storage::traits::StorageLayout>::layout(
                        &<Accumulator as ::ink::storage::traits::StorageKey>::KEY,
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
                                            i32,
                                            _,
                                        >(
                                            ::core::iter::IntoIterator::into_iter(["i32"])
                                                .map(::core::convert::AsRef::as_ref),
                                        ),
                                    )
                                    .done(),
                            ])
                            .payable(false)
                            .docs([" Initializes the value to the initial value."])
                            .done(),
                    ])
                    .messages([
                        ::ink::metadata::MessageSpec::from_label("inc")
                            .selector([0x1D_u8, 0x32_u8, 0x61_u8, 0x9F_u8])
                            .args([
                                ::ink::metadata::MessageParamSpec::new("by")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            i32,
                                            _,
                                        >(
                                            ::core::iter::IntoIterator::into_iter(["i32"])
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
                            .docs([" Mutates the internal value."])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("get")
                            .selector([0x2F_u8, 0x86_u8, 0x5B_u8, 0xD9_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        i32,
                                        _,
                                    >(
                                        ::core::iter::IntoIterator::into_iter(["i32"])
                                            .map(::core::convert::AsRef::as_ref),
                                    ),
                                ),
                            )
                            .mutates(false)
                            .payable(false)
                            .docs([" Returns the current state."])
                            .done(),
                    ])
                    .events([])
                    .docs([])
                    .done(),
            )
        }
    };
}
