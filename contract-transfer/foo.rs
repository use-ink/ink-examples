pub mod subber
{
    use super::*;

    pub const CONTRACT_PATH: &'static str =
        "/Users/michi/projects/ink/examples/delegator/target/ink/subber/subber.contract"
    ;

    pub mod constructors
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct New
        {
            accumulator: super::accumulator::accumulator::AccumulatorRef,
        }

        impl ::ink_e2e::InkConstructor for New
        {
            type ReturnType = Self;
            const SELECTOR: [u8; 4] =
                [0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/subber/subber.contract"
            ;
        }

        pub fn
        new(accumulator: super::accumulator::accumulator::AccumulatorRef) -> New { New { accumulator } }
    }

    pub mod messages
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct Dec
        {
            by: ::core::primitive::i32,
        }

        impl ::ink_e2e::InkMessage for
        Dec
        {
            type ReturnType = ();
            const SELECTOR: [u8; 4] =
                [0xB5_u8, 0xD7_u8, 0xB4_u8, 0xF0_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/subber/subber.contract"
            ;
        }

        pub fn dec(by: ::core::primitive::i32) -> Dec { Dec { by } }
    }
}

pub mod accumulator
{
    use super::*;

    pub const CONTRACT_PATH: &'static str =
        "/Users/michi/projects/ink/examples/delegator/target/ink/accumulator/accumulator.contract"
    ;

    pub mod constructors
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct New
        {
            init_value: ::core::primitive::i32,
        }

        impl ::ink_e2e::
        InkConstructor for New
        {
            type ReturnType = Self;
            const SELECTOR: [u8; 4] =
                [0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/accumulator/accumulator.contract"
            ;
        }

        pub fn new(init_value: ::core::primitive::i32) -> New
        { New { init_value } }
    }

    pub mod messages
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct Inc
        {
            by: ::core::primitive::i32,
        }

        impl ::ink_e2e::InkMessage for
        Inc
        {
            type ReturnType = ();
            const SELECTOR: [u8; 4] =
                [0x1D_u8, 0x32_u8, 0x61_u8, 0x9F_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/accumulator/accumulator.contract"
            ;
        }

        pub fn inc(by: ::core::primitive::i32) -> Inc { Inc { by } }

        #[derive(::scale::Encode)]
        pub struct Get {}

        impl ::ink_e2e::
        InkMessage for Get
        {
            type ReturnType = ::core::primitive::i32;
            const SELECTOR:
            [u8; 4] = [0x2F_u8, 0x86_u8, 0x5B_u8, 0xD9_u8];
            const
            CONTRACT_PATH: &'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/accumulator/accumulator.contract"
            ;
        }

        pub fn get() -> Get { Get {} }
    }
}

pub mod adder
{
    use super::*;

    pub const CONTRACT_PATH: &'static str =
        "/Users/michi/projects/ink/examples/delegator/target/ink/adder/adder.contract"
    ;

    pub mod constructors
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct New
        {
            accumulator: super::accumulator::accumulator::AccumulatorRef,
        }

        impl ::ink_e2e::InkConstructor for New
        {
            type ReturnType = Self;
            const SELECTOR: [u8; 4] =
                [0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/adder/adder.contract"
            ;
        }

        pub fn
        new(accumulator: super::accumulator::accumulator::AccumulatorRef) -> New { New { accumulator } }
    }

    pub mod messages
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct Inc
        {
            by: ::core::primitive::i32,
        }

        impl ::ink_e2e::InkMessage for
        Inc
        {
            type ReturnType = ();
            const SELECTOR: [u8; 4] =
                [0x1D_u8, 0x32_u8, 0x61_u8, 0x9F_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/adder/adder.contract"
            ;
        }

        pub fn inc(by: ::core::primitive::i32) -> Inc { Inc { by } }
    }
}

pub mod delegator
{
    use super::*;

    pub const CONTRACT_PATH: &'static str =
        "/Users/michi/projects/ink/examples/delegator/target/ink/delegator.contract"
    ;

    pub mod constructors
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct New
        {
            init_value: ::core::primitive::i32,
            version: ::core::
            primitive::u32,
            accumulator_code_hash: ::ink::primitives::
            Hash,
            adder_code_hash: ::ink::primitives::Hash,
            subber_code_hash: ::ink::primitives::Hash,
        }

        impl ::ink_e2e::InkConstructor for New
        {
            type ReturnType = Self;
            const SELECTOR: [u8; 4] =
                [0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/delegator.contract"
            ;
        }

        pub fn
        new(init_value: ::core::primitive::i32, version: ::core::
        primitive::u32, accumulator_code_hash: ::ink::primitives::
        Hash, adder_code_hash: ::ink::primitives::Hash, subber_code_hash
            : ::ink::primitives::Hash) -> New
        {
            New
            {
                init_value,
                version,
                accumulator_code_hash,
                adder_code_hash,
                subber_code_hash,
            }
        }
    }

    pub mod messages
    {
        use super::*;

        #[derive(::scale::Encode)]
        pub struct Get {}

        impl
        ::ink_e2e::InkMessage for Get
        {
            type ReturnType = ::core::primitive::i32;
            const SELECTOR:
            [u8; 4] = [0x2F_u8, 0x86_u8, 0x5B_u8, 0xD9_u8];
            const
            CONTRACT_PATH: &'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/delegator.contract"
            ;
        }

        pub fn get() -> Get { Get {} }

        #[derive(::scale::Encode)]
        pub
        struct Change {
            by: ::core::primitive::i32,
        }

        impl ::ink_e2e::
        InkMessage for Change
        {
            type ReturnType = ();
            const SELECTOR: [u8; 4] =
                [0xBF_u8, 0x90_u8, 0xA6_u8, 0x40_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/delegator.contract"
            ;
        }

        pub fn change(by: ::core::primitive::i32) -> Change
        { Change { by } }

        #[derive(::scale::Encode)]
        pub struct Switch {}

        impl ::ink_e2e::InkMessage for Switch
        {
            type ReturnType = ();
            const SELECTOR: [u8; 4] =
                [0x1F_u8, 0x28_u8, 0xC9_u8, 0xDB_u8];
            const CONTRACT_PATH: &
            'static str =
                "/Users/michi/projects/ink/examples/delegator/target/ink/delegator.contract"
            ;
        }

        pub fn switch() -> Switch { Switch {} }
    }
}
