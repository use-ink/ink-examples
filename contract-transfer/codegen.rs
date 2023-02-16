#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;

    pub static PALLETS: [&str; 8usize] = ["System", "RandomnessCollectiveFlip", "Timestamp", "Balances", "Authorship", "TransactionPayment", "Sudo", "Contracts", ];

    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
    pub enum Event { #[codec(index = 0)] System(system::Event), #[codec(index = 3)] Balances(balances::Event), #[codec(index = 5)] TransactionPayment(transaction_payment::Event), #[codec(index = 6)] Sudo(sudo::Event), #[codec(index = 7)] Contracts(contracts::Event) }

    pub mod system {
        use super::root_mod;
        use super::runtime_types;

        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;

            type DispatchError = runtime_types::sp_runtime::DispatchError;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }

            #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(::std::vec::Vec<::core::primitive::u8>, ::std::vec::Vec<::core::primitive::u8>, )>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }

            pub struct TransactionApi;

            impl TransactionApi {
                #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                pub fn fill_block(&self, ratio: runtime_types::sp_arithmetic::per_things::Perbill) -> ::subxt::tx::StaticTxPayload<FillBlock> { ::subxt::tx::StaticTxPayload::new("System", "fill_block", FillBlock { ratio }, [48u8, 18u8, 205u8, 90u8, 222u8, 4u8, 20u8, 251u8, 173u8, 76u8, 167u8, 4u8, 83u8, 203u8, 160u8, 89u8, 132u8, 218u8, 191u8, 145u8, 130u8, 245u8, 177u8, 201u8, 169u8, 129u8, 173u8, 105u8, 88u8, 45u8, 136u8, 191u8, ]) }
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(&self, remark: ::std::vec::Vec<::core::primitive::u8>) -> ::subxt::tx::StaticTxPayload<Remark> { ::subxt::tx::StaticTxPayload::new("System", "remark", Remark { remark }, [101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8, 147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8, 146u8, 87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8, 160u8, ]) }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(&self, pages: ::core::primitive::u64) -> ::subxt::tx::StaticTxPayload<SetHeapPages> { ::subxt::tx::StaticTxPayload::new("System", "set_heap_pages", SetHeapPages { pages }, [43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8, 86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8, 125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8, ]) }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                #[doc = "  expensive)."]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                #[doc = "expensive. We will treat this as a full block."]
                #[doc = "# </weight>"]
                pub fn set_code(&self, code: ::std::vec::Vec<::core::primitive::u8>) -> ::subxt::tx::StaticTxPayload<SetCode> { ::subxt::tx::StaticTxPayload::new("System", "set_code", SetCode { code }, [27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8, 244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8, 4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8, ]) }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                #[doc = "block. # </weight>"]
                pub fn set_code_without_checks(&self, code: ::std::vec::Vec<::core::primitive::u8>) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> { ::subxt::tx::StaticTxPayload::new("System", "set_code_without_checks", SetCodeWithoutChecks { code }, [102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8, 159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8, 40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8, ]) }
                #[doc = "Set some items of storage."]
                pub fn set_storage(&self, items: ::std::vec::Vec<(::std::vec::Vec<::core::primitive::u8>, ::std::vec::Vec<::core::primitive::u8>, )>) -> ::subxt::tx::StaticTxPayload<SetStorage> { ::subxt::tx::StaticTxPayload::new("System", "set_storage", SetStorage { items }, [74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8, 45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8, 140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8, ]) }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(&self, keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>) -> ::subxt::tx::StaticTxPayload<KillStorage> { ::subxt::tx::StaticTxPayload::new("System", "kill_storage", KillStorage { keys }, [174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8, 85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8, 67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8, 15u8, ]) }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(&self, prefix: ::std::vec::Vec<::core::primitive::u8>, subkeys: ::core::primitive::u32) -> ::subxt::tx::StaticTxPayload<KillPrefix> { ::subxt::tx::StaticTxPayload::new("System", "kill_prefix", KillPrefix { prefix, subkeys }, [203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8, 193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8, 80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8, 48u8, ]) }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(&self, remark: ::std::vec::Vec<::core::primitive::u8>) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> { ::subxt::tx::StaticTxPayload::new("System", "remark_with_event", RemarkWithEvent { remark }, [123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8, 134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8, 136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8, 46u8, ]) }
            }
        }

        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;

        pub mod events {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }

            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }

            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;

            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }

            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
            }

            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
                pub hash: ::subxt::ext::sp_core::H256,
            }

            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::frame_system::AccountInfo<::core::primitive::u32, runtime_types::pallet_balances::AccountData<::core::primitive::u128>>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "Account", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Blake2_128Concat)], [176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8, 114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8, 42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8, ]) }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::frame_system::AccountInfo<::core::primitive::u32, runtime_types::pallet_balances::AccountData<::core::primitive::u128>>>, (), ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "Account", Vec::new(), [176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8, 114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8, 42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8, ]) }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::u32>, ::subxt::storage::address::Yes, (), ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "ExtrinsicCount", vec![], [223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8, 222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8, 41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8, ]) }
                #[doc = " The current weight for the block."]
                pub fn block_weight(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "BlockWeight", vec![], [91u8, 211u8, 177u8, 36u8, 147u8, 249u8, 55u8, 164u8, 48u8, 49u8, 55u8, 11u8, 121u8, 193u8, 103u8, 69u8, 38u8, 142u8, 148u8, 36u8, 137u8, 41u8, 115u8, 195u8, 31u8, 174u8, 163u8, 125u8, 69u8, 5u8, 94u8, 79u8, ]) }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::u32>, ::subxt::storage::address::Yes, (), ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "AllExtrinsicsLen", vec![], [202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8, 254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8, 219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8, 134u8, 60u8, ]) }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(&self, _0: impl ::std::borrow::Borrow<::core::primitive::u32>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "BlockHash", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Twox64Concat)], [50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8, 21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8, 80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8, ]) }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>, (), ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "BlockHash", Vec::new(), [50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8, 21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8, 80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8, ]) }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(&self, _0: impl ::std::borrow::Borrow<::core::primitive::u32>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "ExtrinsicData", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Twox64Concat)], [210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8, 254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8, 59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8, ]) }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>, (), ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "ExtrinsicData", Vec::new(), [210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8, 254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8, 59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8, ]) }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::u32>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "Number", vec![], [228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8, 246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8, 36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8, 164u8, 191u8, ]) }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "ParentHash", vec![], [232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8, 169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8, 15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8, 36u8, ]) }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::generic::digest::Digest>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "Digest", vec![], [83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8, 31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8, 216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8, ]) }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::std::vec::Vec<runtime_types::frame_system::EventRecord<runtime_types::contracts_node_runtime::Event, ::subxt::ext::sp_core::H256>>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "Events", vec![], [155u8, 23u8, 131u8, 160u8, 247u8, 184u8, 188u8, 91u8, 5u8, 245u8, 50u8, 182u8, 148u8, 189u8, 4u8, 241u8, 226u8, 55u8, 40u8, 150u8, 106u8, 102u8, 39u8, 77u8, 73u8, 128u8, 24u8, 217u8, 199u8, 22u8, 147u8, 233u8, ]) }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::u32>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "EventCount", vec![], [236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8, 203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8, 161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8, 112u8, ]) }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32, )>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "EventTopics", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Blake2_128Concat)], [205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8, 63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8, 111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8, 38u8, ]) }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32, )>>, (), ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("System", "EventTopics", Vec::new(), [205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8, 63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8, 111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8, 38u8, ]) }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::frame_system::LastRuntimeUpgradeInfo>, ::subxt::storage::address::Yes, (), ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "LastRuntimeUpgrade", vec![], [52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8, 126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8, 185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8, ]) }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::bool>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "UpgradedToU32RefCount", vec![], [171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8, 178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8, 83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8, ]) }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::bool>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "UpgradedToTripleRefCount", vec![], [90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8, 210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8, 155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8, 185u8, ]) }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>, ::subxt::storage::address::Yes, (), ()> { ::subxt::storage::address::StaticStorageAddress::new("System", "ExecutionPhase", vec![], [230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8, 103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8, 201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8, 172u8, 93u8, ]) }
            }
        }

        pub mod constants {
            use super::runtime_types;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<runtime_types::frame_system::limits::BlockWeights>> { ::subxt::constants::StaticConstantAddress::new("System", "BlockWeights", [153u8, 164u8, 86u8, 79u8, 97u8, 114u8, 248u8, 181u8, 179u8, 186u8, 214u8, 124u8, 215u8, 96u8, 116u8, 109u8, 215u8, 182u8, 61u8, 10u8, 77u8, 74u8, 29u8, 125u8, 131u8, 111u8, 249u8, 208u8, 233u8, 170u8, 11u8, 14u8, ]) }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<runtime_types::frame_system::limits::BlockLength>> { ::subxt::constants::StaticConstantAddress::new("System", "BlockLength", [116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8, 220u8, 234u8, 198u8, 150u8, 108u8, 205u8, 87u8, 194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8, 12u8, 200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8, 205u8, 203u8, 236u8, ]) }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u32>> { ::subxt::constants::StaticConstantAddress::new("System", "BlockHashCount", [98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8, ]) }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<runtime_types::frame_support::weights::RuntimeDbWeight>> { ::subxt::constants::StaticConstantAddress::new("System", "DbWeight", [124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8, 199u8, 181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8, 20u8, 129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8, 118u8, ]) }
                #[doc = " Get the chain's current version."]
                pub fn version(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>> { ::subxt::constants::StaticConstantAddress::new("System", "Version", [93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8, 47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8, 165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8, ]) }
                #[doc = " The designated SS85 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u16>> { ::subxt::constants::StaticConstantAddress::new("System", "SS58Prefix", [116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8, 41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8, 90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8, ]) }
            }
        }
    }

    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub fn random_material(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<::subxt::ext::sp_core::H256>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("RandomnessCollectiveFlip", "RandomMaterial", vec![], [152u8, 126u8, 73u8, 88u8, 54u8, 147u8, 6u8, 19u8, 214u8, 40u8, 159u8, 30u8, 236u8, 61u8, 240u8, 65u8, 178u8, 94u8, 146u8, 152u8, 135u8, 252u8, 160u8, 86u8, 123u8, 114u8, 251u8, 140u8, 98u8, 143u8, 217u8, 242u8, ]) }
            }
        }
    }

    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;

        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;

            type DispatchError = runtime_types::sp_runtime::DispatchError;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct Set {
                #[codec(compact)] pub now: ::core::primitive::u64,
            }

            pub struct TransactionApi;

            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                #[doc = "# </weight>"]
                pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::StaticTxPayload<Set> { ::subxt::tx::StaticTxPayload::new("Timestamp", "set", Set { now }, [6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8, 102u8, 85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8, 214u8, 182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8, ]) }
            }
        }

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::u64>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Timestamp", "Now", vec![], [148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8, 221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8, 185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8, ]) }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::bool>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Timestamp", "DidUpdate", vec![], [70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8, 13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8, 27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8, ]) }
            }
        }

        pub mod constants {
            use super::runtime_types;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u64>> { ::subxt::constants::StaticConstantAddress::new("Timestamp", "MinimumPeriod", [128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8, ]) }
            }
        }
    }

    pub mod balances {
        use super::root_mod;
        use super::runtime_types;

        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;

            type DispatchError = runtime_types::sp_runtime::DispatchError;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct Transfer {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)] pub value: ::core::primitive::u128,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetBalance {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)] pub new_free: ::core::primitive::u128,
                #[codec(compact)] pub new_reserved: ::core::primitive::u128,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct ForceTransfer {
                pub source: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)] pub value: ::core::primitive::u128,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct TransferKeepAlive {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)] pub value: ::core::primitive::u128,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct TransferAll {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct ForceUnreserve {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }

            pub struct TransactionApi;

            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                #[doc = "  types. See related functions below."]
                #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                #[doc = "  computation."]
                #[doc = ""]
                #[doc = "Related functions:"]
                #[doc = ""]
                #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                #[doc = "    that the transfer will not kill the origin account."]
                #[doc = "---------------------------------"]
                #[doc = "- Origin account is already in memory, so no DB operations for them."]
                #[doc = "# </weight>"]
                pub fn transfer(&self, dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, value: ::core::primitive::u128) -> ::subxt::tx::StaticTxPayload<Transfer> { ::subxt::tx::StaticTxPayload::new("Balances", "transfer", Transfer { dest, value }, [111u8, 222u8, 32u8, 56u8, 171u8, 77u8, 252u8, 29u8, 194u8, 155u8, 200u8, 192u8, 198u8, 81u8, 23u8, 115u8, 236u8, 91u8, 218u8, 114u8, 107u8, 141u8, 138u8, 100u8, 237u8, 21u8, 58u8, 172u8, 3u8, 20u8, 216u8, 38u8, ]) }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                #[doc = "If the new free or reserved balance is below the existential deposit,"]
                #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(&self, who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, new_free: ::core::primitive::u128, new_reserved: ::core::primitive::u128) -> ::subxt::tx::StaticTxPayload<SetBalance> { ::subxt::tx::StaticTxPayload::new("Balances", "set_balance", SetBalance { who, new_free, new_reserved }, [234u8, 215u8, 97u8, 98u8, 243u8, 199u8, 57u8, 76u8, 59u8, 161u8, 118u8, 207u8, 34u8, 197u8, 198u8, 61u8, 231u8, 210u8, 169u8, 235u8, 150u8, 137u8, 173u8, 49u8, 28u8, 77u8, 84u8, 149u8, 143u8, 210u8, 139u8, 193u8, ]) }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "# <weight>"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                #[doc = "# </weight>"]
                pub fn force_transfer(&self, source: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, value: ::core::primitive::u128) -> ::subxt::tx::StaticTxPayload<ForceTransfer> { ::subxt::tx::StaticTxPayload::new("Balances", "force_transfer", ForceTransfer { source, dest, value }, [79u8, 174u8, 212u8, 108u8, 184u8, 33u8, 170u8, 29u8, 232u8, 254u8, 195u8, 218u8, 221u8, 134u8, 57u8, 99u8, 6u8, 70u8, 181u8, 227u8, 56u8, 239u8, 243u8, 158u8, 157u8, 245u8, 36u8, 162u8, 11u8, 237u8, 147u8, 15u8, ]) }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(&self, dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, value: ::core::primitive::u128) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> { ::subxt::tx::StaticTxPayload::new("Balances", "transfer_keep_alive", TransferKeepAlive { dest, value }, [112u8, 179u8, 75u8, 168u8, 193u8, 221u8, 9u8, 82u8, 190u8, 113u8, 253u8, 13u8, 130u8, 134u8, 170u8, 216u8, 136u8, 111u8, 242u8, 220u8, 202u8, 112u8, 47u8, 79u8, 73u8, 244u8, 226u8, 59u8, 240u8, 188u8, 210u8, 208u8, ]) }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true). # <weight>"]
                #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                #[doc = "  #</weight>"]
                pub fn transfer_all(&self, dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, keep_alive: ::core::primitive::bool) -> ::subxt::tx::StaticTxPayload<TransferAll> { ::subxt::tx::StaticTxPayload::new("Balances", "transfer_all", TransferAll { dest, keep_alive }, [46u8, 129u8, 29u8, 177u8, 221u8, 107u8, 245u8, 69u8, 238u8, 126u8, 145u8, 26u8, 219u8, 208u8, 14u8, 80u8, 149u8, 1u8, 214u8, 63u8, 67u8, 201u8, 144u8, 45u8, 129u8, 145u8, 174u8, 71u8, 238u8, 113u8, 208u8, 34u8, ]) }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(&self, who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, amount: ::core::primitive::u128) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> { ::subxt::tx::StaticTxPayload::new("Balances", "force_unreserve", ForceUnreserve { who, amount }, [160u8, 146u8, 137u8, 76u8, 157u8, 187u8, 66u8, 148u8, 207u8, 76u8, 32u8, 254u8, 82u8, 215u8, 35u8, 161u8, 213u8, 52u8, 32u8, 98u8, 102u8, 106u8, 234u8, 123u8, 6u8, 175u8, 184u8, 188u8, 174u8, 106u8, 176u8, 78u8, ]) }
            }
        }

        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;

        pub mod events {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::ext::sp_core::crypto::AccountId32,
                pub to: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }

            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::u128>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "TotalIssuance", vec![], [1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8, 156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8, 20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8, ]) }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::AccountData<::core::primitive::u128>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "Account", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Blake2_128Concat)], [246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8, 40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8, 217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8, ]) }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::AccountData<::core::primitive::u128>>, (), ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "Account", Vec::new(), [246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8, 40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8, 217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8, ]) }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "Locks", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Blake2_128Concat)], [216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8, 58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8, 136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8, ]) }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>>>, (), ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "Locks", Vec::new(), [216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8, 58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8, 136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8, ]) }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<runtime_types::pallet_balances::ReserveData<[::core::primitive::u8; 8usize], ::core::primitive::u128>>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "Reserves", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Blake2_128Concat)], [17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8, 167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8, 183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8, ]) }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<runtime_types::pallet_balances::ReserveData<[::core::primitive::u8; 8usize], ::core::primitive::u128>>>, (), ::subxt::storage::address::Yes, ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "Reserves", Vec::new(), [17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8, 167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8, 183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8, ]) }
                #[doc = " Storage version of the pallet."]
                #[doc = ""]
                #[doc = " This is set to v2.0.0 for new networks."]
                pub fn storage_version(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::Releases>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Balances", "StorageVersion", vec![], [135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8, 235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8, 174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8, 173u8, 96u8, ]) }
            }
        }

        pub mod constants {
            use super::runtime_types;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u128>> { ::subxt::constants::StaticConstantAddress::new("Balances", "ExistentialDeposit", [84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8, ]) }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u32>> { ::subxt::constants::StaticConstantAddress::new("Balances", "MaxLocks", [98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8, ]) }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u32>> { ::subxt::constants::StaticConstantAddress::new("Balances", "MaxReserves", [98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8, ]) }
            }
        }
    }

    pub mod authorship {
        use super::root_mod;
        use super::runtime_types;

        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;

            type DispatchError = runtime_types::sp_runtime::DispatchError;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetUncles {
                pub new_uncles: ::std::vec::Vec<runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32, runtime_types::sp_runtime::traits::BlakeTwo256>>,
            }

            pub struct TransactionApi;

            impl TransactionApi {
                #[doc = "Provide a set of uncles."]
                pub fn set_uncles(&self, new_uncles: ::std::vec::Vec<runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32, runtime_types::sp_runtime::traits::BlakeTwo256>>) -> ::subxt::tx::StaticTxPayload<SetUncles> { ::subxt::tx::StaticTxPayload::new("Authorship", "set_uncles", SetUncles { new_uncles }, [181u8, 70u8, 222u8, 83u8, 154u8, 215u8, 200u8, 64u8, 154u8, 228u8, 115u8, 247u8, 117u8, 89u8, 229u8, 102u8, 128u8, 189u8, 90u8, 60u8, 223u8, 19u8, 111u8, 172u8, 5u8, 223u8, 132u8, 37u8, 235u8, 119u8, 42u8, 64u8, ]) }
            }
        }

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                #[doc = " Uncles"]
                pub fn uncles(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<runtime_types::pallet_authorship::UncleEntryItem<::core::primitive::u32, ::subxt::ext::sp_core::H256, ::subxt::ext::sp_core::crypto::AccountId32>>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Authorship", "Uncles", vec![], [193u8, 226u8, 196u8, 151u8, 233u8, 82u8, 60u8, 164u8, 27u8, 156u8, 231u8, 51u8, 79u8, 134u8, 170u8, 166u8, 71u8, 120u8, 250u8, 255u8, 52u8, 168u8, 74u8, 199u8, 122u8, 253u8, 248u8, 178u8, 39u8, 233u8, 132u8, 67u8, ]) }
                #[doc = " Author of current block."]
                pub fn author(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>, ::subxt::storage::address::Yes, (), ()> { ::subxt::storage::address::StaticStorageAddress::new("Authorship", "Author", vec![], [149u8, 42u8, 33u8, 147u8, 190u8, 207u8, 174u8, 227u8, 190u8, 110u8, 25u8, 131u8, 5u8, 167u8, 237u8, 188u8, 188u8, 33u8, 177u8, 126u8, 181u8, 49u8, 126u8, 118u8, 46u8, 128u8, 154u8, 95u8, 15u8, 91u8, 103u8, 113u8, ]) }
                #[doc = " Whether uncles were already set in this block."]
                pub fn did_set_uncles(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::bool>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Authorship", "DidSetUncles", vec![], [64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8, 126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8, 83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8, ]) }
            }
        }

        pub mod constants {
            use super::runtime_types;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #[doc = " The number of blocks back we should accept uncles."]
                #[doc = " This means that we will deal with uncle-parents that are"]
                #[doc = " `UncleGenerations + 1` before `now`."]
                pub fn uncle_generations(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u32>> { ::subxt::constants::StaticConstantAddress::new("Authorship", "UncleGenerations", [98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8, ]) }
            }
        }
    }

    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;

        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;

        pub mod events {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: ::subxt::ext::sp_core::crypto::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }

            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                pub fn next_fee_multiplier(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_arithmetic::fixed_point::FixedU128>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("TransactionPayment", "NextFeeMultiplier", vec![], [210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8, 197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8, 237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8, 180u8, ]) }
                pub fn storage_version(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_transaction_payment::Releases>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("TransactionPayment", "StorageVersion", vec![], [219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8, 200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8, 58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8, ]) }
            }
        }

        pub mod constants {
            use super::runtime_types;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u8>> { ::subxt::constants::StaticConstantAddress::new("TransactionPayment", "OperationalFeeMultiplier", [141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8, 28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8, 114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8, 165u8, ]) }
            }
        }
    }

    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;

        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;

            type DispatchError = runtime_types::sp_runtime::DispatchError;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::contracts_node_runtime::Call>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::contracts_node_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetKey {
                pub new: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SudoAs {
                pub who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::contracts_node_runtime::Call>,
            }

            pub struct TransactionApi;

            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo(&self, call: runtime_types::contracts_node_runtime::Call) -> ::subxt::tx::StaticTxPayload<Sudo> { ::subxt::tx::StaticTxPayload::new("Sudo", "sudo", Sudo { call: ::std::boxed::Box::new(call) }, [140u8, 190u8, 95u8, 189u8, 152u8, 74u8, 211u8, 245u8, 255u8, 121u8, 217u8, 88u8, 217u8, 69u8, 210u8, 41u8, 140u8, 118u8, 39u8, 200u8, 60u8, 163u8, 228u8, 16u8, 106u8, 173u8, 217u8, 53u8, 218u8, 32u8, 243u8, 53u8, ]) }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- The weight of this call is defined by the caller."]
                #[doc = "# </weight>"]
                pub fn sudo_unchecked_weight(&self, call: runtime_types::contracts_node_runtime::Call, weight: ::core::primitive::u64) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> { ::subxt::tx::StaticTxPayload::new("Sudo", "sudo_unchecked_weight", SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight }, [122u8, 45u8, 179u8, 118u8, 60u8, 6u8, 139u8, 190u8, 92u8, 219u8, 178u8, 5u8, 2u8, 182u8, 246u8, 117u8, 222u8, 56u8, 120u8, 74u8, 163u8, 230u8, 114u8, 33u8, 233u8, 96u8, 250u8, 106u8, 94u8, 108u8, 247u8, 60u8, ]) }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB change."]
                #[doc = "# </weight>"]
                pub fn set_key(&self, new: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>) -> ::subxt::tx::StaticTxPayload<SetKey> { ::subxt::tx::StaticTxPayload::new("Sudo", "set_key", SetKey { new }, [23u8, 224u8, 218u8, 169u8, 8u8, 28u8, 111u8, 199u8, 26u8, 88u8, 225u8, 105u8, 17u8, 19u8, 87u8, 156u8, 97u8, 67u8, 89u8, 173u8, 70u8, 0u8, 5u8, 246u8, 198u8, 135u8, 182u8, 180u8, 44u8, 9u8, 212u8, 95u8, ]) }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo_as(&self, who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, call: runtime_types::contracts_node_runtime::Call) -> ::subxt::tx::StaticTxPayload<SudoAs> { ::subxt::tx::StaticTxPayload::new("Sudo", "sudo_as", SudoAs { who, call: ::std::boxed::Box::new(call) }, [47u8, 65u8, 47u8, 89u8, 53u8, 63u8, 102u8, 10u8, 76u8, 42u8, 188u8, 100u8, 163u8, 254u8, 9u8, 86u8, 162u8, 52u8, 248u8, 230u8, 53u8, 79u8, 251u8, 230u8, 72u8, 250u8, 49u8, 117u8, 194u8, 205u8, 254u8, 25u8, ]) }
            }
        }

        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;

        pub mod events {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }

            impl ::subxt::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
            }

            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }

            impl ::subxt::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>, ::subxt::storage::address::Yes, (), ()> { ::subxt::storage::address::StaticStorageAddress::new("Sudo", "Key", vec![], [244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8, 179u8, 122u8, 173u8, 34u8, 108u8, 137u8, 28u8, 182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8, 80u8, 199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8, 167u8, 20u8, 142u8, ]) }
            }
        }
    }

    pub mod contracts {
        use super::root_mod;
        use super::runtime_types;

        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;

            type DispatchError = runtime_types::sp_runtime::DispatchError;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct Call {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)] pub value: ::core::primitive::u128,
                #[codec(compact)] pub gas_limit: ::core::primitive::u64,
                pub storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct InstantiateWithCode {
                #[codec(compact)] pub value: ::core::primitive::u128,
                #[codec(compact)] pub gas_limit: ::core::primitive::u64,
                pub storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>,
                pub code: ::std::vec::Vec<::core::primitive::u8>,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct Instantiate {
                #[codec(compact)] pub value: ::core::primitive::u128,
                #[codec(compact)] pub gas_limit: ::core::primitive::u64,
                pub storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>,
                pub code_hash: ::subxt::ext::sp_core::H256,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::std::vec::Vec<::core::primitive::u8>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct UploadCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
                pub storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct RemoveCode {
                pub code_hash: ::subxt::ext::sp_core::H256,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct SetCode {
                pub dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>,
                pub code_hash: ::subxt::ext::sp_core::H256,
            }

            pub struct TransactionApi;

            impl TransactionApi {
                #[doc = "Makes a call to an account, optionally transferring some balance."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = ""]
                #[doc = "* `dest`: Address of the contract to call."]
                #[doc = "* `value`: The balance to transfer from the `origin` to `dest`."]
                #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged from the"]
                #[doc = "  caller to pay for the storage consumed."]
                #[doc = "* `data`: The input data to pass to the contract."]
                #[doc = ""]
                #[doc = "* If the account is a smart-contract account, the associated code will be"]
                #[doc = "executed and any value will be transferred."]
                #[doc = "* If the account is a regular account, any value will be transferred."]
                #[doc = "* If no account exists and the call value is not less than `existential_deposit`,"]
                #[doc = "a regular account will be created and any value will be transferred."]
                pub fn call(&self, dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, value: ::core::primitive::u128, gas_limit: ::core::primitive::u64, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>, data: ::std::vec::Vec<::core::primitive::u8>) -> ::subxt::tx::StaticTxPayload<Call> { ::subxt::tx::StaticTxPayload::new("Contracts", "call", Call { dest, value, gas_limit, storage_deposit_limit, data }, [22u8, 148u8, 225u8, 3u8, 113u8, 132u8, 125u8, 117u8, 187u8, 51u8, 32u8, 8u8, 2u8, 125u8, 76u8, 14u8, 227u8, 139u8, 67u8, 136u8, 55u8, 98u8, 181u8, 163u8, 105u8, 175u8, 43u8, 125u8, 17u8, 123u8, 191u8, 228u8, ]) }
                #[doc = "Instantiates a new contract from the supplied `code` optionally transferring"]
                #[doc = "some balance."]
                #[doc = ""]
                #[doc = "This dispatchable has the same effect as calling [`Self::upload_code`] +"]
                #[doc = "[`Self::instantiate`]. Bundling them together provides efficiency gains. Please"]
                #[doc = "also check the documentation of [`Self::upload_code`]."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = ""]
                #[doc = "* `value`: The balance to transfer from the `origin` to the newly created contract."]
                #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged/reserved"]
                #[doc = "  from the caller to pay for the storage consumed."]
                #[doc = "* `code`: The contract code to deploy in raw bytes."]
                #[doc = "* `data`: The input data to pass to the contract constructor."]
                #[doc = "* `salt`: Used for the address derivation. See [`Pallet::contract_address`]."]
                #[doc = ""]
                #[doc = "Instantiation is executed as follows:"]
                #[doc = ""]
                #[doc = "- The supplied `code` is instrumented, deployed, and a `code_hash` is created for that"]
                #[doc = "  code."]
                #[doc = "- If the `code_hash` already exists on the chain the underlying `code` will be shared."]
                #[doc = "- The destination address is computed based on the sender, code_hash and the salt."]
                #[doc = "- The smart-contract account is created at the computed address."]
                #[doc = "- The `value` is transferred to the new account."]
                #[doc = "- The `deploy` function is executed in the context of the newly-created account."]
                pub fn instantiate_with_code(&self, value: ::core::primitive::u128, gas_limit: ::core::primitive::u64, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>, code: ::std::vec::Vec<::core::primitive::u8>, data: ::std::vec::Vec<::core::primitive::u8>, salt: ::std::vec::Vec<::core::primitive::u8>) -> ::subxt::tx::StaticTxPayload<InstantiateWithCode> { ::subxt::tx::StaticTxPayload::new("Contracts", "instantiate_with_code", InstantiateWithCode { value, gas_limit, storage_deposit_limit, code, data, salt }, [232u8, 229u8, 46u8, 91u8, 11u8, 117u8, 3u8, 81u8, 213u8, 34u8, 184u8, 125u8, 77u8, 214u8, 71u8, 103u8, 244u8, 131u8, 1u8, 211u8, 191u8, 153u8, 1u8, 80u8, 177u8, 177u8, 205u8, 126u8, 194u8, 166u8, 136u8, 191u8, ]) }
                #[doc = "Instantiates a contract from a previously deployed wasm binary."]
                #[doc = ""]
                #[doc = "This function is identical to [`Self::instantiate_with_code`] but without the"]
                #[doc = "code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary"]
                #[doc = "must be supplied."]
                pub fn instantiate(&self, value: ::core::primitive::u128, gas_limit: ::core::primitive::u64, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>, code_hash: ::subxt::ext::sp_core::H256, data: ::std::vec::Vec<::core::primitive::u8>, salt: ::std::vec::Vec<::core::primitive::u8>) -> ::subxt::tx::StaticTxPayload<Instantiate> { ::subxt::tx::StaticTxPayload::new("Contracts", "instantiate", Instantiate { value, gas_limit, storage_deposit_limit, code_hash, data, salt }, [69u8, 161u8, 178u8, 243u8, 14u8, 29u8, 15u8, 210u8, 29u8, 106u8, 129u8, 211u8, 90u8, 73u8, 66u8, 177u8, 245u8, 1u8, 232u8, 117u8, 119u8, 216u8, 84u8, 160u8, 207u8, 7u8, 237u8, 88u8, 25u8, 85u8, 213u8, 235u8, ]) }
                #[doc = "Upload new `code` without instantiating a contract from it."]
                #[doc = ""]
                #[doc = "If the code does not already exist a deposit is reserved from the caller"]
                #[doc = "and unreserved only when [`Self::remove_code`] is called. The size of the reserve"]
                #[doc = "depends on the instrumented size of the the supplied `code`."]
                #[doc = ""]
                #[doc = "If the code already exists in storage it will still return `Ok` and upgrades"]
                #[doc = "the in storage version to the current"]
                #[doc = "[`InstructionWeights::version`](InstructionWeights)."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "Anyone can instantiate a contract from any uploaded code and thus prevent its removal."]
                #[doc = "To avoid this situation a constructor could employ access control so that it can"]
                #[doc = "only be instantiated by permissioned entities. The same is true when uploading"]
                #[doc = "through [`Self::instantiate_with_code`]."]
                pub fn upload_code(&self, code: ::std::vec::Vec<::core::primitive::u8>, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>) -> ::subxt::tx::StaticTxPayload<UploadCode> { ::subxt::tx::StaticTxPayload::new("Contracts", "upload_code", UploadCode { code, storage_deposit_limit }, [8u8, 32u8, 174u8, 226u8, 212u8, 86u8, 47u8, 247u8, 123u8, 155u8, 40u8, 192u8, 184u8, 216u8, 61u8, 57u8, 94u8, 23u8, 76u8, 59u8, 4u8, 124u8, 252u8, 248u8, 87u8, 233u8, 13u8, 184u8, 133u8, 236u8, 174u8, 85u8, ]) }
                #[doc = "Remove the code stored under `code_hash` and refund the deposit to its owner."]
                #[doc = ""]
                #[doc = "A code can only be removed by its original uploader (its owner) and only if it is"]
                #[doc = "not used by any contract."]
                pub fn remove_code(&self, code_hash: ::subxt::ext::sp_core::H256) -> ::subxt::tx::StaticTxPayload<RemoveCode> { ::subxt::tx::StaticTxPayload::new("Contracts", "remove_code", RemoveCode { code_hash }, [43u8, 192u8, 198u8, 182u8, 108u8, 76u8, 21u8, 42u8, 169u8, 41u8, 195u8, 73u8, 31u8, 179u8, 162u8, 56u8, 91u8, 5u8, 64u8, 7u8, 252u8, 194u8, 255u8, 170u8, 67u8, 137u8, 143u8, 192u8, 2u8, 149u8, 38u8, 180u8, ]) }
                #[doc = "Privileged function that changes the code of an existing contract."]
                #[doc = ""]
                #[doc = "This takes care of updating refcounts and all other necessary operations. Returns"]
                #[doc = "an error if either the `code_hash` or `dest` do not exist."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "This does **not** change the address of the contract in question. This means"]
                #[doc = "that the contract address is no longer derived from its code hash after calling"]
                #[doc = "this dispatchable."]
                pub fn set_code(&self, dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, code_hash: ::subxt::ext::sp_core::H256) -> ::subxt::tx::StaticTxPayload<SetCode> { ::subxt::tx::StaticTxPayload::new("Contracts", "set_code", SetCode { dest, code_hash }, [106u8, 141u8, 239u8, 113u8, 99u8, 74u8, 14u8, 171u8, 80u8, 115u8, 214u8, 203u8, 232u8, 142u8, 48u8, 207u8, 214u8, 59u8, 204u8, 157u8, 101u8, 142u8, 12u8, 69u8, 230u8, 188u8, 60u8, 197u8, 238u8, 146u8, 17u8, 190u8, ]) }
            }
        }

        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_contracts::pallet::Event;

        pub mod events {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Contract deployed by address at the specified address."]
            pub struct Instantiated {
                pub deployer: ::subxt::ext::sp_core::crypto::AccountId32,
                pub contract: ::subxt::ext::sp_core::crypto::AccountId32,
            }

            impl ::subxt::events::StaticEvent for Instantiated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Instantiated";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Contract has been removed."]
            #[doc = ""]
            #[doc = "# Note"]
            #[doc = ""]
            #[doc = "The only way for a contract to be removed and emitting this event is by calling"]
            #[doc = "`seal_terminate`."]
            pub struct Terminated {
                pub contract: ::subxt::ext::sp_core::crypto::AccountId32,
                pub beneficiary: ::subxt::ext::sp_core::crypto::AccountId32,
            }

            impl ::subxt::events::StaticEvent for Terminated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Terminated";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "Code with the specified hash has been stored."]
            pub struct CodeStored {
                pub code_hash: ::subxt::ext::sp_core::H256,
            }

            impl ::subxt::events::StaticEvent for CodeStored {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeStored";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A custom event emitted by the contract."]
            pub struct ContractEmitted {
                pub contract: ::subxt::ext::sp_core::crypto::AccountId32,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }

            impl ::subxt::events::StaticEvent for ContractEmitted {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractEmitted";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A code with the specified hash was removed."]
            pub struct CodeRemoved {
                pub code_hash: ::subxt::ext::sp_core::H256,
            }

            impl ::subxt::events::StaticEvent for CodeRemoved {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeRemoved";
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            #[doc = "A contract's code was updated."]
            pub struct ContractCodeUpdated {
                pub contract: ::subxt::ext::sp_core::crypto::AccountId32,
                pub new_code_hash: ::subxt::ext::sp_core::H256,
                pub old_code_hash: ::subxt::ext::sp_core::H256,
            }

            impl ::subxt::events::StaticEvent for ContractCodeUpdated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractCodeUpdated";
            }
        }

        pub mod storage {
            use super::runtime_types;

            pub struct StorageApi;

            impl StorageApi {
                #[doc = " A mapping from an original code hash to the original code, untouched by instrumentation."]
                pub fn pristine_code(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<::core::primitive::u8>>, ::subxt::storage::address::Yes, (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "PristineCode", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Identity)], [244u8, 169u8, 220u8, 235u8, 62u8, 153u8, 226u8, 187u8, 220u8, 141u8, 149u8, 75u8, 224u8, 117u8, 181u8, 147u8, 140u8, 84u8, 9u8, 109u8, 230u8, 25u8, 186u8, 26u8, 171u8, 147u8, 19u8, 78u8, 62u8, 170u8, 27u8, 105u8, ]) }
                #[doc = " A mapping from an original code hash to the original code, untouched by instrumentation."]
                pub fn pristine_code_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<::core::primitive::u8>>, (), (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "PristineCode", Vec::new(), [244u8, 169u8, 220u8, 235u8, 62u8, 153u8, 226u8, 187u8, 220u8, 141u8, 149u8, 75u8, 224u8, 117u8, 181u8, 147u8, 140u8, 84u8, 9u8, 109u8, 230u8, 25u8, 186u8, 26u8, 171u8, 147u8, 19u8, 78u8, 62u8, 170u8, 27u8, 105u8, ]) }
                #[doc = " A mapping between an original code hash and instrumented wasm code, ready for execution."]
                pub fn code_storage(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_contracts::wasm::PrefabWasmModule>, ::subxt::storage::address::Yes, (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "CodeStorage", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Identity)], [167u8, 247u8, 131u8, 220u8, 90u8, 100u8, 172u8, 16u8, 129u8, 235u8, 119u8, 88u8, 60u8, 196u8, 74u8, 173u8, 192u8, 110u8, 106u8, 187u8, 111u8, 255u8, 114u8, 39u8, 76u8, 52u8, 245u8, 79u8, 132u8, 108u8, 141u8, 176u8, ]) }
                #[doc = " A mapping between an original code hash and instrumented wasm code, ready for execution."]
                pub fn code_storage_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_contracts::wasm::PrefabWasmModule>, (), (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "CodeStorage", Vec::new(), [167u8, 247u8, 131u8, 220u8, 90u8, 100u8, 172u8, 16u8, 129u8, 235u8, 119u8, 88u8, 60u8, 196u8, 74u8, 173u8, 192u8, 110u8, 106u8, 187u8, 111u8, 255u8, 114u8, 39u8, 76u8, 52u8, 245u8, 79u8, 132u8, 108u8, 141u8, 176u8, ]) }
                #[doc = " A mapping between an original code hash and its owner information."]
                pub fn owner_info_of(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_contracts::wasm::OwnerInfo>, ::subxt::storage::address::Yes, (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "OwnerInfoOf", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Identity)], [147u8, 6u8, 225u8, 62u8, 211u8, 236u8, 61u8, 116u8, 152u8, 219u8, 220u8, 17u8, 82u8, 221u8, 156u8, 88u8, 63u8, 204u8, 16u8, 11u8, 184u8, 236u8, 181u8, 189u8, 170u8, 160u8, 60u8, 64u8, 71u8, 250u8, 202u8, 186u8, ]) }
                #[doc = " A mapping between an original code hash and its owner information."]
                pub fn owner_info_of_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_contracts::wasm::OwnerInfo>, (), (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "OwnerInfoOf", Vec::new(), [147u8, 6u8, 225u8, 62u8, 211u8, 236u8, 61u8, 116u8, 152u8, 219u8, 220u8, 17u8, 82u8, 221u8, 156u8, 88u8, 63u8, 204u8, 16u8, 11u8, 184u8, 236u8, 181u8, 189u8, 170u8, 160u8, 60u8, 64u8, 71u8, 250u8, 202u8, 186u8, ]) }
                #[doc = " This is a **monotonic** counter incremented on contract instantiation."]
                #[doc = ""]
                #[doc = " This is used in order to generate unique trie ids for contracts."]
                #[doc = " The trie id of a new contract is calculated from hash(account_id, nonce)."]
                #[doc = " The nonce is required because otherwise the following sequence would lead to"]
                #[doc = " a possible collision of storage:"]
                #[doc = ""]
                #[doc = " 1. Create a new contract."]
                #[doc = " 2. Terminate the contract."]
                #[doc = " 3. Immediately recreate the contract with the same account_id."]
                #[doc = ""]
                #[doc = " This is bad because the contents of a trie are deleted lazily and there might be"]
                #[doc = " storage of the old instantiation still in it when the new contract is created. Please"]
                #[doc = " note that we can't replace the counter by the block number because the sequence above"]
                #[doc = " can happen in the same block. We also can't keep the account counter in memory only"]
                #[doc = " because storage is the only way to communicate across different extrinsics in the"]
                #[doc = " same block."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Do not use it to determine the number of contracts. It won't be decremented if"]
                #[doc = " a contract is destroyed."]
                pub fn nonce(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<::core::primitive::u64>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "Nonce", vec![], [122u8, 169u8, 95u8, 131u8, 85u8, 32u8, 154u8, 114u8, 143u8, 56u8, 12u8, 182u8, 64u8, 150u8, 241u8, 249u8, 254u8, 251u8, 160u8, 235u8, 192u8, 41u8, 101u8, 232u8, 186u8, 108u8, 187u8, 149u8, 210u8, 91u8, 179u8, 98u8, ]) }
                #[doc = " The code associated with a given account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn contract_info_of(&self, _0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_contracts::storage::RawContractInfo<::subxt::ext::sp_core::H256, ::core::primitive::u128>>, ::subxt::storage::address::Yes, (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "ContractInfoOf", vec![::subxt::storage::address::StorageMapKey::new(_0.borrow(), ::subxt::storage::address::StorageHasher::Twox64Concat)], [249u8, 249u8, 163u8, 158u8, 30u8, 66u8, 127u8, 176u8, 156u8, 185u8, 75u8, 198u8, 120u8, 208u8, 233u8, 131u8, 161u8, 49u8, 45u8, 175u8, 242u8, 171u8, 63u8, 39u8, 76u8, 31u8, 167u8, 140u8, 210u8, 235u8, 185u8, 240u8, ]) }
                #[doc = " The code associated with a given account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn contract_info_of_root(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::pallet_contracts::storage::RawContractInfo<::subxt::ext::sp_core::H256, ::core::primitive::u128>>, (), (), ::subxt::storage::address::Yes> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "ContractInfoOf", Vec::new(), [249u8, 249u8, 163u8, 158u8, 30u8, 66u8, 127u8, 176u8, 156u8, 185u8, 75u8, 198u8, 120u8, 208u8, 233u8, 131u8, 161u8, 49u8, 45u8, 175u8, 242u8, 171u8, 63u8, 39u8, 76u8, 31u8, 167u8, 140u8, 210u8, 235u8, 185u8, 240u8, ]) }
                #[doc = " Evicted contracts that await child trie deletion."]
                #[doc = ""]
                #[doc = " Child trie deletion is a heavy operation depending on the amount of storage items"]
                #[doc = " stored in said trie. Therefore this operation is performed lazily in `on_initialize`."]
                pub fn deletion_queue(&self) -> ::subxt::storage::address::StaticStorageAddress::<::subxt::metadata::DecodeStaticType<runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<runtime_types::pallet_contracts::storage::DeletedContract>>, ::subxt::storage::address::Yes, ::subxt::storage::address::Yes, ()> { ::subxt::storage::address::StaticStorageAddress::new("Contracts", "DeletionQueue", vec![], [119u8, 169u8, 146u8, 210u8, 21u8, 216u8, 51u8, 225u8, 107u8, 61u8, 42u8, 155u8, 169u8, 127u8, 140u8, 106u8, 255u8, 137u8, 163u8, 199u8, 91u8, 137u8, 73u8, 61u8, 9u8, 167u8, 16u8, 157u8, 183u8, 212u8, 35u8, 88u8, ]) }
            }
        }

        pub mod constants {
            use super::runtime_types;

            pub struct ConstantsApi;

            impl ConstantsApi {
                #[doc = " Cost schedule and limits."]
                pub fn schedule(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<runtime_types::pallet_contracts::schedule::Schedule>> { ::subxt::constants::StaticConstantAddress::new("Contracts", "Schedule", [106u8, 133u8, 138u8, 78u8, 95u8, 52u8, 197u8, 85u8, 4u8, 33u8, 173u8, 239u8, 169u8, 196u8, 91u8, 38u8, 210u8, 50u8, 62u8, 67u8, 180u8, 184u8, 32u8, 190u8, 106u8, 252u8, 104u8, 173u8, 5u8, 140u8, 244u8, 249u8, ]) }
                #[doc = " The maximum number of contracts that can be pending for deletion."]
                #[doc = ""]
                #[doc = " When a contract is deleted by calling `seal_terminate` it becomes inaccessible"]
                #[doc = " immediately, but the deletion of the storage items it has accumulated is performed"]
                #[doc = " later. The contract is put into the deletion queue. This defines how many"]
                #[doc = " contracts can be queued up at the same time. If that limit is reached `seal_terminate`"]
                #[doc = " will fail. The action must be retried in a later block in that case."]
                #[doc = ""]
                #[doc = " The reasons for limiting the queue depth are:"]
                #[doc = ""]
                #[doc = " 1. The queue is in storage in order to be persistent between blocks. We want to limit"]
                #[doc = " \tthe amount of storage that can be consumed."]
                #[doc = " 2. The queue is stored in a vector and needs to be decoded as a whole when reading"]
                #[doc = "\t\tit at the end of each block. Longer queues take more weight to decode and hence"]
                #[doc = "\t\tlimit the amount of items that can be deleted per block."]
                pub fn deletion_queue_depth(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u32>> { ::subxt::constants::StaticConstantAddress::new("Contracts", "DeletionQueueDepth", [98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8, 125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8, 178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8, 145u8, ]) }
                #[doc = " The maximum amount of weight that can be consumed per block for lazy trie removal."]
                #[doc = ""]
                #[doc = " The amount of weight that is dedicated per block to work on the deletion queue. Larger"]
                #[doc = " values allow more trie keys to be deleted in each block but reduce the amount of"]
                #[doc = " weight that is left for transactions. See [`Self::DeletionQueueDepth`] for more"]
                #[doc = " information about the deletion queue."]
                pub fn deletion_weight_limit(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u64>> { ::subxt::constants::StaticConstantAddress::new("Contracts", "DeletionWeightLimit", [128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8, ]) }
                #[doc = " The amount of balance a caller has to pay for each byte of storage."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Changing this value for an existing chain might need a storage migration."]
                pub fn deposit_per_byte(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u128>> { ::subxt::constants::StaticConstantAddress::new("Contracts", "DepositPerByte", [84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8, ]) }
                #[doc = " The weight per byte of code that is charged when loading a contract from storage."]
                #[doc = ""]
                #[doc = " Currently, FRAME only charges fees for computation incurred but not for PoV"]
                #[doc = " consumption caused for storage access. This is usually not exploitable because"]
                #[doc = " accessing storage carries some substantial weight costs, too. However in case"]
                #[doc = " of contract code very much PoV consumption can be caused while consuming very little"]
                #[doc = " computation. This could be used to keep the chain busy without paying the"]
                #[doc = " proper fee for it. Until this is resolved we charge from the weight meter for"]
                #[doc = " contract access."]
                #[doc = ""]
                #[doc = " For more information check out: <https://github.com/paritytech/substrate/issues/10301>"]
                #[doc = ""]
                #[doc = " [`DefaultContractAccessWeight`] is a safe default to be used for Polkadot or Kusama"]
                #[doc = " parachains."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " This is only relevant for parachains. Set to zero in case of a standalone chain."]
                pub fn contract_access_weight(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u64>> { ::subxt::constants::StaticConstantAddress::new("Contracts", "ContractAccessWeight", [128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8, 59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8, 103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8, 246u8, ]) }
                #[doc = " The amount of balance a caller has to pay for each storage item."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Changing this value for an existing chain might need a storage migration."]
                pub fn deposit_per_item(&self) -> ::subxt::constants::StaticConstantAddress<::subxt::metadata::DecodeStaticType<::core::primitive::u128>> { ::subxt::constants::StaticConstantAddress::new("Contracts", "DepositPerItem", [84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8, 27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8, 136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8, ]) }
            }
        }
    }

    pub mod runtime_types {
        use super::runtime_types;

        pub mod contracts_node_runtime {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum Call { #[codec(index = 0)] System(runtime_types::frame_system::pallet::Call), #[codec(index = 2)] Timestamp(runtime_types::pallet_timestamp::pallet::Call), #[codec(index = 3)] Balances(runtime_types::pallet_balances::pallet::Call), #[codec(index = 4)] Authorship(runtime_types::pallet_authorship::pallet::Call), #[codec(index = 6)] Sudo(runtime_types::pallet_sudo::pallet::Call), #[codec(index = 7)] Contracts(runtime_types::pallet_contracts::pallet::Call) }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum Event { #[codec(index = 0)] System(runtime_types::frame_system::pallet::Event), #[codec(index = 3)] Balances(runtime_types::pallet_balances::pallet::Event), #[codec(index = 5)] TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event), #[codec(index = 6)] Sudo(runtime_types::pallet_sudo::pallet::Event), #[codec(index = 7)] Contracts(runtime_types::pallet_contracts::pallet::Event) }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct Runtime;
        }

        pub mod frame_support {
            use super::runtime_types;

            pub mod traits {
                use super::runtime_types;

                pub mod tokens {
                    use super::runtime_types;

                    pub mod misc {
                        use super::runtime_types;

                        #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                        pub enum BalanceStatus { #[codec(index = 0)] Free, #[codec(index = 1)] Reserved }
                    }
                }
            }

            pub mod weights {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub enum DispatchClass { #[codec(index = 0)] Normal, #[codec(index = 1)] Operational, #[codec(index = 2)] Mandatory }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub enum Pays { #[codec(index = 0)] Yes, #[codec(index = 1)] No }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
            }
        }

        pub mod frame_system {
            use super::runtime_types;

            pub mod extensions {
                use super::runtime_types;

                pub mod check_genesis {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct CheckGenesis;
                }

                pub mod check_mortality {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }

                pub mod check_non_zero_sender {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct CheckNonZeroSender;
                }

                pub mod check_nonce {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }

                pub mod check_spec_version {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct CheckSpecVersion;
                }

                pub mod check_tx_version {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct CheckTxVersion;
                }

                pub mod check_weight {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct CheckWeight;
                }
            }

            pub mod limits {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u32>,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<runtime_types::frame_system::limits::WeightsPerClass>,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }

            pub mod pallet {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A dispatch that will fill the block weight up to the given ratio."] fill_block { ratio: runtime_types::sp_arithmetic::per_things::Perbill },
                    #[codec(index = 1)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`"]
                    #[doc = "# </weight>"] remark { remark: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 2)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."] set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                    #[doc = "  expensive)."]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                    #[doc = "expensive. We will treat this as a full block."]
                    #[doc = "# </weight>"] set_code { code: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 4)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                    #[doc = "block. # </weight>"] set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 5)]
                    #[doc = "Set some items of storage."] set_storage { items: ::std::vec::Vec<(::std::vec::Vec<::core::primitive::u8>, ::std::vec::Vec<::core::primitive::u8>, )> },
                    #[codec(index = 6)]
                    #[doc = "Kill some items from storage."] kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
                    #[codec(index = 7)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."] kill_prefix { prefix: ::std::vec::Vec<::core::primitive::u8>, subkeys: ::core::primitive::u32 },
                    #[codec(index = 8)]
                    #[doc = "Make some on-chain remark and emit event."] remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."] InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."] SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."] FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."] NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."] NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."] CallFiltered,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."] ExtrinsicSuccess { dispatch_info: runtime_types::frame_support::weights::DispatchInfo },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."] ExtrinsicFailed { dispatch_error: runtime_types::sp_runtime::DispatchError, dispatch_info: runtime_types::frame_support::weights::DispatchInfo },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."] CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."] NewAccount { account: ::subxt::ext::sp_core::crypto::AccountId32 },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."] KilledAccount { account: ::subxt::ext::sp_core::crypto::AccountId32 },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."] Remarked { sender: ::subxt::ext::sp_core::crypto::AccountId32, hash: ::subxt::ext::sp_core::H256 },
                }
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)] pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum Phase { #[codec(index = 0)] ApplyExtrinsic(::core::primitive::u32), #[codec(index = 1)] Finalization, #[codec(index = 2)] Initialization }
        }

        pub mod pallet_authorship {
            use super::runtime_types;

            pub mod pallet {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Provide a set of uncles."] set_uncles { new_uncles: ::std::vec::Vec<runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32, runtime_types::sp_runtime::traits::BlakeTwo256>> },
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The uncle parent not in the chain."] InvalidUncleParent,
                    #[codec(index = 1)]
                    #[doc = "Uncles already set in the block."] UnclesAlreadySet,
                    #[codec(index = 2)]
                    #[doc = "Too many uncles."] TooManyUncles,
                    #[codec(index = 3)]
                    #[doc = "The uncle is genesis."] GenesisUncle,
                    #[codec(index = 4)]
                    #[doc = "The uncle is too high in chain."] TooHighUncle,
                    #[codec(index = 5)]
                    #[doc = "The uncle is already included."] UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    #[doc = "The uncle isn't recent enough to be included."] OldUncle,
                }
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum UncleEntryItem<_0, _1, _2> { #[codec(index = 0)] InclusionHeight(_0), #[codec(index = 1)] Uncle(_1, ::core::option::Option<_2>) }
        }

        pub mod pallet_balances {
            use super::runtime_types;

            pub mod pallet {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                    #[doc = "  types. See related functions below."]
                    #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                    #[doc = "  computation."]
                    #[doc = ""]
                    #[doc = "Related functions:"]
                    #[doc = ""]
                    #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                    #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                    #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                    #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                    #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                    #[doc = "    that the transfer will not kill the origin account."]
                    #[doc = "---------------------------------"]
                    #[doc = "- Origin account is already in memory, so no DB operations for them."]
                    #[doc = "# </weight>"] transfer { dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, #[codec(compact)] value: ::core::primitive::u128 },
                    #[codec(index = 1)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                    #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                    #[doc = "If the new free or reserved balance is below the existential deposit,"]
                    #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."] set_balance { who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, #[codec(compact)] new_free: ::core::primitive::u128, #[codec(compact)] new_reserved: ::core::primitive::u128 },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                    #[doc = "specified."]
                    #[doc = "# <weight>"]
                    #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                    #[doc = "  assumed to be in the overlay."]
                    #[doc = "# </weight>"] force_transfer { source: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, #[codec(compact)] value: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                    #[doc = "origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"] transfer_keep_alive { dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, #[codec(compact)] value: ::core::primitive::u128 },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true). # <weight>"]
                    #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                    #[doc = "  #</weight>"] transfer_all { dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, keep_alive: ::core::primitive::bool },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."] force_unreserve { who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, amount: ::core::primitive::u128 },
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value"] VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal"] LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value"] InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit"] ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"] KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account"] ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"] DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed MaxReserves"] TooManyReserves,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."] Endowed { account: ::subxt::ext::sp_core::crypto::AccountId32, free_balance: ::core::primitive::u128 },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."] DustLost { account: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."] Transfer { from: ::subxt::ext::sp_core::crypto::AccountId32, to: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."] BalanceSet { who: ::subxt::ext::sp_core::crypto::AccountId32, free: ::core::primitive::u128, reserved: ::core::primitive::u128 },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."] Reserved { who: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."] Unreserved { who: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."] ReserveRepatriated { from: ::subxt::ext::sp_core::crypto::AccountId32, to: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128, destination_status: runtime_types::frame_support::traits::tokens::misc::BalanceStatus },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."] Deposit { who: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."] Withdraw { who: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."] Slashed { who: ::subxt::ext::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
                }
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum Reasons { #[codec(index = 0)] Fee, #[codec(index = 1)] Misc, #[codec(index = 2)] All }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum Releases { #[codec(index = 0)] V1_0_0, #[codec(index = 1)] V2_0_0 }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }

        pub mod pallet_contracts {
            use super::runtime_types;

            pub mod pallet {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Makes a call to an account, optionally transferring some balance."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `dest`: Address of the contract to call."]
                    #[doc = "* `value`: The balance to transfer from the `origin` to `dest`."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged from the"]
                    #[doc = "  caller to pay for the storage consumed."]
                    #[doc = "* `data`: The input data to pass to the contract."]
                    #[doc = ""]
                    #[doc = "* If the account is a smart-contract account, the associated code will be"]
                    #[doc = "executed and any value will be transferred."]
                    #[doc = "* If the account is a regular account, any value will be transferred."]
                    #[doc = "* If no account exists and the call value is not less than `existential_deposit`,"]
                    #[doc = "a regular account will be created and any value will be transferred."] call { dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, #[codec(compact)] value: ::core::primitive::u128, #[codec(compact)] gas_limit: ::core::primitive::u64, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>, data: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 1)]
                    #[doc = "Instantiates a new contract from the supplied `code` optionally transferring"]
                    #[doc = "some balance."]
                    #[doc = ""]
                    #[doc = "This dispatchable has the same effect as calling [`Self::upload_code`] +"]
                    #[doc = "[`Self::instantiate`]. Bundling them together provides efficiency gains. Please"]
                    #[doc = "also check the documentation of [`Self::upload_code`]."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `value`: The balance to transfer from the `origin` to the newly created contract."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged/reserved"]
                    #[doc = "  from the caller to pay for the storage consumed."]
                    #[doc = "* `code`: The contract code to deploy in raw bytes."]
                    #[doc = "* `data`: The input data to pass to the contract constructor."]
                    #[doc = "* `salt`: Used for the address derivation. See [`Pallet::contract_address`]."]
                    #[doc = ""]
                    #[doc = "Instantiation is executed as follows:"]
                    #[doc = ""]
                    #[doc = "- The supplied `code` is instrumented, deployed, and a `code_hash` is created for that"]
                    #[doc = "  code."]
                    #[doc = "- If the `code_hash` already exists on the chain the underlying `code` will be shared."]
                    #[doc = "- The destination address is computed based on the sender, code_hash and the salt."]
                    #[doc = "- The smart-contract account is created at the computed address."]
                    #[doc = "- The `value` is transferred to the new account."]
                    #[doc = "- The `deploy` function is executed in the context of the newly-created account."] instantiate_with_code { #[codec(compact)] value: ::core::primitive::u128, #[codec(compact)] gas_limit: ::core::primitive::u64, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>, code: ::std::vec::Vec<::core::primitive::u8>, data: ::std::vec::Vec<::core::primitive::u8>, salt: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 2)]
                    #[doc = "Instantiates a contract from a previously deployed wasm binary."]
                    #[doc = ""]
                    #[doc = "This function is identical to [`Self::instantiate_with_code`] but without the"]
                    #[doc = "code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary"]
                    #[doc = "must be supplied."] instantiate { #[codec(compact)] value: ::core::primitive::u128, #[codec(compact)] gas_limit: ::core::primitive::u64, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>>, code_hash: ::subxt::ext::sp_core::H256, data: ::std::vec::Vec<::core::primitive::u8>, salt: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 3)]
                    #[doc = "Upload new `code` without instantiating a contract from it."]
                    #[doc = ""]
                    #[doc = "If the code does not already exist a deposit is reserved from the caller"]
                    #[doc = "and unreserved only when [`Self::remove_code`] is called. The size of the reserve"]
                    #[doc = "depends on the instrumented size of the the supplied `code`."]
                    #[doc = ""]
                    #[doc = "If the code already exists in storage it will still return `Ok` and upgrades"]
                    #[doc = "the in storage version to the current"]
                    #[doc = "[`InstructionWeights::version`](InstructionWeights)."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "Anyone can instantiate a contract from any uploaded code and thus prevent its removal."]
                    #[doc = "To avoid this situation a constructor could employ access control so that it can"]
                    #[doc = "only be instantiated by permissioned entities. The same is true when uploading"]
                    #[doc = "through [`Self::instantiate_with_code`]."] upload_code { code: ::std::vec::Vec<::core::primitive::u8>, storage_deposit_limit: ::core::option::Option<::subxt::ext::codec::Compact<::core::primitive::u128>> },
                    #[codec(index = 4)]
                    #[doc = "Remove the code stored under `code_hash` and refund the deposit to its owner."]
                    #[doc = ""]
                    #[doc = "A code can only be removed by its original uploader (its owner) and only if it is"]
                    #[doc = "not used by any contract."] remove_code { code_hash: ::subxt::ext::sp_core::H256 },
                    #[codec(index = 5)]
                    #[doc = "Privileged function that changes the code of an existing contract."]
                    #[doc = ""]
                    #[doc = "This takes care of updating refcounts and all other necessary operations. Returns"]
                    #[doc = "an error if either the `code_hash` or `dest` do not exist."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "This does **not** change the address of the contract in question. This means"]
                    #[doc = "that the contract address is no longer derived from its code hash after calling"]
                    #[doc = "this dispatchable."] set_code { dest: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, code_hash: ::subxt::ext::sp_core::H256 },
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "A new schedule must have a greater version than the current one."] InvalidScheduleVersion,
                    #[codec(index = 1)]
                    #[doc = "Invalid combination of flags supplied to `seal_call` or `seal_delegate_call`."] InvalidCallFlags,
                    #[codec(index = 2)]
                    #[doc = "The executed contract exhausted its gas limit."] OutOfGas,
                    #[codec(index = 3)]
                    #[doc = "The output buffer supplied to a contract API call was too small."] OutputBufferTooSmall,
                    #[codec(index = 4)]
                    #[doc = "Performing the requested transfer failed. Probably because there isn't enough"]
                    #[doc = "free balance in the sender's account."] TransferFailed,
                    #[codec(index = 5)]
                    #[doc = "Performing a call was denied because the calling depth reached the limit"]
                    #[doc = "of what is specified in the schedule."] MaxCallDepthReached,
                    #[codec(index = 6)]
                    #[doc = "No contract was found at the specified address."] ContractNotFound,
                    #[codec(index = 7)]
                    #[doc = "The code supplied to `instantiate_with_code` exceeds the limit specified in the"]
                    #[doc = "current schedule."] CodeTooLarge,
                    #[codec(index = 8)]
                    #[doc = "No code could be found at the supplied code hash."] CodeNotFound,
                    #[codec(index = 9)]
                    #[doc = "A buffer outside of sandbox memory was passed to a contract API function."] OutOfBounds,
                    #[codec(index = 10)]
                    #[doc = "Input passed to a contract API function failed to decode as expected type."] DecodingFailed,
                    #[codec(index = 11)]
                    #[doc = "Contract trapped during execution."] ContractTrapped,
                    #[codec(index = 12)]
                    #[doc = "The size defined in `T::MaxValueSize` was exceeded."] ValueTooLarge,
                    #[codec(index = 13)]
                    #[doc = "Termination of a contract is not allowed while the contract is already"]
                    #[doc = "on the call stack. Can be triggered by `seal_terminate`."] TerminatedWhileReentrant,
                    #[codec(index = 14)]
                    #[doc = "`seal_call` forwarded this contracts input. It therefore is no longer available."] InputForwarded,
                    #[codec(index = 15)]
                    #[doc = "The subject passed to `seal_random` exceeds the limit."] RandomSubjectTooLong,
                    #[codec(index = 16)]
                    #[doc = "The amount of topics passed to `seal_deposit_events` exceeds the limit."] TooManyTopics,
                    #[codec(index = 17)]
                    #[doc = "The topics passed to `seal_deposit_events` contains at least one duplicate."] DuplicateTopics,
                    #[codec(index = 18)]
                    #[doc = "The chain does not provide a chain extension. Calling the chain extension results"]
                    #[doc = "in this error. Note that this usually  shouldn't happen as deploying such contracts"]
                    #[doc = "is rejected."] NoChainExtension,
                    #[codec(index = 19)]
                    #[doc = "Removal of a contract failed because the deletion queue is full."]
                    #[doc = ""]
                    #[doc = "This can happen when calling `seal_terminate`."]
                    #[doc = "The queue is filled by deleting contracts and emptied by a fixed amount each block."]
                    #[doc = "Trying again during another block is the only way to resolve this issue."] DeletionQueueFull,
                    #[codec(index = 20)]
                    #[doc = "A contract with the same AccountId already exists."] DuplicateContract,
                    #[codec(index = 21)]
                    #[doc = "A contract self destructed in its constructor."]
                    #[doc = ""]
                    #[doc = "This can be triggered by a call to `seal_terminate`."] TerminatedInConstructor,
                    #[codec(index = 22)]
                    #[doc = "The debug message specified to `seal_debug_message` does contain invalid UTF-8."] DebugMessageInvalidUTF8,
                    #[codec(index = 23)]
                    #[doc = "A call tried to invoke a contract that is flagged as non-reentrant."] ReentranceDenied,
                    #[codec(index = 24)]
                    #[doc = "Origin doesn't have enough balance to pay the required storage deposits."] StorageDepositNotEnoughFunds,
                    #[codec(index = 25)]
                    #[doc = "More storage was created than allowed by the storage deposit limit."] StorageDepositLimitExhausted,
                    #[codec(index = 26)]
                    #[doc = "Code removal was denied because the code is still in use by at least one contract."] CodeInUse,
                    #[codec(index = 27)]
                    #[doc = "The contract ran to completion but decided to revert its storage changes."]
                    #[doc = "Please note that this error is only returned from extrinsics. When called directly"]
                    #[doc = "or via RPC an `Ok` will be returned. In this case the caller needs to inspect the flags"]
                    #[doc = "to determine whether a reversion has taken place."] ContractReverted,
                    #[codec(index = 28)]
                    #[doc = "The contract's code was found to be invalid during validation or instrumentation."]
                    #[doc = "A more detailed error can be found on the node console if debug messages are enabled"]
                    #[doc = "or in the debug buffer which is returned to RPC clients."] CodeRejected,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Contract deployed by address at the specified address."] Instantiated { deployer: ::subxt::ext::sp_core::crypto::AccountId32, contract: ::subxt::ext::sp_core::crypto::AccountId32 },
                    #[codec(index = 1)]
                    #[doc = "Contract has been removed."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "The only way for a contract to be removed and emitting this event is by calling"]
                    #[doc = "`seal_terminate`."] Terminated { contract: ::subxt::ext::sp_core::crypto::AccountId32, beneficiary: ::subxt::ext::sp_core::crypto::AccountId32 },
                    #[codec(index = 2)]
                    #[doc = "Code with the specified hash has been stored."] CodeStored { code_hash: ::subxt::ext::sp_core::H256 },
                    #[codec(index = 3)]
                    #[doc = "A custom event emitted by the contract."] ContractEmitted { contract: ::subxt::ext::sp_core::crypto::AccountId32, data: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 4)]
                    #[doc = "A code with the specified hash was removed."] CodeRemoved { code_hash: ::subxt::ext::sp_core::H256 },
                    #[codec(index = 5)]
                    #[doc = "A contract's code was updated."] ContractCodeUpdated { contract: ::subxt::ext::sp_core::crypto::AccountId32, new_code_hash: ::subxt::ext::sp_core::H256, old_code_hash: ::subxt::ext::sp_core::H256 },
                }
            }

            pub mod schedule {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct HostFnWeights {
                    pub caller: ::core::primitive::u64,
                    pub is_contract: ::core::primitive::u64,
                    pub code_hash: ::core::primitive::u64,
                    pub own_code_hash: ::core::primitive::u64,
                    pub caller_is_origin: ::core::primitive::u64,
                    pub address: ::core::primitive::u64,
                    pub gas_left: ::core::primitive::u64,
                    pub balance: ::core::primitive::u64,
                    pub value_transferred: ::core::primitive::u64,
                    pub minimum_balance: ::core::primitive::u64,
                    pub block_number: ::core::primitive::u64,
                    pub now: ::core::primitive::u64,
                    pub weight_to_fee: ::core::primitive::u64,
                    pub gas: ::core::primitive::u64,
                    pub input: ::core::primitive::u64,
                    pub input_per_byte: ::core::primitive::u64,
                    pub r#return: ::core::primitive::u64,
                    pub return_per_byte: ::core::primitive::u64,
                    pub terminate: ::core::primitive::u64,
                    pub random: ::core::primitive::u64,
                    pub deposit_event: ::core::primitive::u64,
                    pub deposit_event_per_topic: ::core::primitive::u64,
                    pub deposit_event_per_byte: ::core::primitive::u64,
                    pub debug_message: ::core::primitive::u64,
                    pub set_storage: ::core::primitive::u64,
                    pub set_storage_per_new_byte: ::core::primitive::u64,
                    pub set_storage_per_old_byte: ::core::primitive::u64,
                    pub set_code_hash: ::core::primitive::u64,
                    pub clear_storage: ::core::primitive::u64,
                    pub clear_storage_per_byte: ::core::primitive::u64,
                    pub contains_storage: ::core::primitive::u64,
                    pub contains_storage_per_byte: ::core::primitive::u64,
                    pub get_storage: ::core::primitive::u64,
                    pub get_storage_per_byte: ::core::primitive::u64,
                    pub take_storage: ::core::primitive::u64,
                    pub take_storage_per_byte: ::core::primitive::u64,
                    pub transfer: ::core::primitive::u64,
                    pub call: ::core::primitive::u64,
                    pub delegate_call: ::core::primitive::u64,
                    pub call_transfer_surcharge: ::core::primitive::u64,
                    pub call_per_cloned_byte: ::core::primitive::u64,
                    pub instantiate: ::core::primitive::u64,
                    pub instantiate_transfer_surcharge: ::core::primitive::u64,
                    pub instantiate_per_salt_byte: ::core::primitive::u64,
                    pub hash_sha2_256: ::core::primitive::u64,
                    pub hash_sha2_256_per_byte: ::core::primitive::u64,
                    pub hash_keccak_256: ::core::primitive::u64,
                    pub hash_keccak_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_256: ::core::primitive::u64,
                    pub hash_blake2_256_per_byte: ::core::primitive::u64,
                    pub hash_blake2_128: ::core::primitive::u64,
                    pub hash_blake2_128_per_byte: ::core::primitive::u64,
                    pub ecdsa_recover: ::core::primitive::u64,
                    pub ecdsa_to_eth_address: ::core::primitive::u64,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct InstructionWeights {
                    pub version: ::core::primitive::u32,
                    pub i64const: ::core::primitive::u32,
                    pub i64load: ::core::primitive::u32,
                    pub i64store: ::core::primitive::u32,
                    pub select: ::core::primitive::u32,
                    pub r#if: ::core::primitive::u32,
                    pub br: ::core::primitive::u32,
                    pub br_if: ::core::primitive::u32,
                    pub br_table: ::core::primitive::u32,
                    pub br_table_per_entry: ::core::primitive::u32,
                    pub call: ::core::primitive::u32,
                    pub call_indirect: ::core::primitive::u32,
                    pub call_indirect_per_param: ::core::primitive::u32,
                    pub local_get: ::core::primitive::u32,
                    pub local_set: ::core::primitive::u32,
                    pub local_tee: ::core::primitive::u32,
                    pub global_get: ::core::primitive::u32,
                    pub global_set: ::core::primitive::u32,
                    pub memory_current: ::core::primitive::u32,
                    pub memory_grow: ::core::primitive::u32,
                    pub i64clz: ::core::primitive::u32,
                    pub i64ctz: ::core::primitive::u32,
                    pub i64popcnt: ::core::primitive::u32,
                    pub i64eqz: ::core::primitive::u32,
                    pub i64extendsi32: ::core::primitive::u32,
                    pub i64extendui32: ::core::primitive::u32,
                    pub i32wrapi64: ::core::primitive::u32,
                    pub i64eq: ::core::primitive::u32,
                    pub i64ne: ::core::primitive::u32,
                    pub i64lts: ::core::primitive::u32,
                    pub i64ltu: ::core::primitive::u32,
                    pub i64gts: ::core::primitive::u32,
                    pub i64gtu: ::core::primitive::u32,
                    pub i64les: ::core::primitive::u32,
                    pub i64leu: ::core::primitive::u32,
                    pub i64ges: ::core::primitive::u32,
                    pub i64geu: ::core::primitive::u32,
                    pub i64add: ::core::primitive::u32,
                    pub i64sub: ::core::primitive::u32,
                    pub i64mul: ::core::primitive::u32,
                    pub i64divs: ::core::primitive::u32,
                    pub i64divu: ::core::primitive::u32,
                    pub i64rems: ::core::primitive::u32,
                    pub i64remu: ::core::primitive::u32,
                    pub i64and: ::core::primitive::u32,
                    pub i64or: ::core::primitive::u32,
                    pub i64xor: ::core::primitive::u32,
                    pub i64shl: ::core::primitive::u32,
                    pub i64shrs: ::core::primitive::u32,
                    pub i64shru: ::core::primitive::u32,
                    pub i64rotl: ::core::primitive::u32,
                    pub i64rotr: ::core::primitive::u32,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Limits {
                    pub event_topics: ::core::primitive::u32,
                    pub stack_height: ::core::option::Option<::core::primitive::u32>,
                    pub globals: ::core::primitive::u32,
                    pub parameters: ::core::primitive::u32,
                    pub memory_pages: ::core::primitive::u32,
                    pub table_size: ::core::primitive::u32,
                    pub br_table_size: ::core::primitive::u32,
                    pub subject_len: ::core::primitive::u32,
                    pub call_depth: ::core::primitive::u32,
                    pub payload_len: ::core::primitive::u32,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Schedule {
                    pub limits: runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights: runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights: runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }

            pub mod storage {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct DeletedContract {
                    pub trie_id: runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct RawContractInfo<_0, _1> {
                    pub trie_id: runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                    pub code_hash: _0,
                    pub storage_deposit: _1,
                }
            }

            pub mod wasm {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct OwnerInfo {
                    pub owner: ::subxt::ext::sp_core::crypto::AccountId32,
                    #[codec(compact)] pub deposit: ::core::primitive::u128,
                    #[codec(compact)] pub refcount: ::core::primitive::u64,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct PrefabWasmModule {
                    #[codec(compact)] pub instruction_weights_version: ::core::primitive::u32,
                    #[codec(compact)] pub initial: ::core::primitive::u32,
                    #[codec(compact)] pub maximum: ::core::primitive::u32,
                    pub code: runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<::core::primitive::u8>,
                }
            }
        }

        pub mod pallet_sudo {
            use super::runtime_types;

            pub mod pallet {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"] sudo { call: ::std::boxed::Box<runtime_types::contracts_node_runtime::Call> },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- The weight of this call is defined by the caller."]
                    #[doc = "# </weight>"] sudo_unchecked_weight { call: ::std::boxed::Box<runtime_types::contracts_node_runtime::Call>, weight: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB change."]
                    #[doc = "# </weight>"] set_key { new: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()> },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"] sudo_as { who: ::subxt::ext::sp_runtime::MultiAddress<::subxt::ext::sp_core::crypto::AccountId32, ()>, call: ::std::boxed::Box<runtime_types::contracts_node_runtime::Call> },
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"] RequireSudo,
                }

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"] Sudid { sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError> },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."] KeyChanged { old_sudoer: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32> },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"] SudoAsDone { sudo_result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError> },
                }
            }
        }

        pub mod pallet_timestamp {
            use super::runtime_types;

            pub mod pallet {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    #[doc = "# </weight>"] set { #[codec(compact)] now: ::core::primitive::u64 },
                }
            }
        }

        pub mod pallet_transaction_payment {
            use super::runtime_types;

            pub mod pallet {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."] TransactionFeePaid { who: ::subxt::ext::sp_core::crypto::AccountId32, actual_fee: ::core::primitive::u128, tip: ::core::primitive::u128 },
                }
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum Releases { #[codec(index = 0)] V1Ancient, #[codec(index = 1)] V2 }
        }

        pub mod primitive_types {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }

        pub mod sp_arithmetic {
            use super::runtime_types;

            pub mod fixed_point {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct FixedU128(pub ::core::primitive::u128);
            }

            pub mod per_things {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::CompactAs, ::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }

        pub mod sp_core {
            use super::runtime_types;

            pub mod crypto {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
            }

            pub mod ecdsa {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }

            pub mod ed25519 {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }

            pub mod sr25519 {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
        }

        pub mod sp_runtime {
            use super::runtime_types;

            pub mod bounded {
                use super::runtime_types;

                pub mod bounded_vec {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct BoundedVec<_0> (pub ::std::vec::Vec<_0>);
                }

                pub mod weak_bounded_vec {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct WeakBoundedVec<_0> (pub ::std::vec::Vec<_0>);
                }
            }

            pub mod generic {
                use super::runtime_types;

                pub mod digest {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct Digest {
                        pub logs: ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub enum DigestItem { #[codec(index = 6)] PreRuntime([::core::primitive::u8; 4usize], ::std::vec::Vec<::core::primitive::u8>), #[codec(index = 4)] Consensus([::core::primitive::u8; 4usize], ::std::vec::Vec<::core::primitive::u8>), #[codec(index = 5)] Seal([::core::primitive::u8; 4usize], ::std::vec::Vec<::core::primitive::u8>), #[codec(index = 0)] Other(::std::vec::Vec<::core::primitive::u8>), #[codec(index = 8)] RuntimeEnvironmentUpdated }
                }

                pub mod era {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub enum Era {
                        #[codec(index = 0)] Immortal,
                        #[codec(index = 1)] Mortal1(::core::primitive::u8),
                        #[codec(index = 2)] Mortal2(::core::primitive::u8),
                        #[codec(index = 3)] Mortal3(::core::primitive::u8),
                        #[codec(index = 4)] Mortal4(::core::primitive::u8),
                        #[codec(index = 5)] Mortal5(::core::primitive::u8),
                        #[codec(index = 6)] Mortal6(::core::primitive::u8),
                        #[codec(index = 7)] Mortal7(::core::primitive::u8),
                        #[codec(index = 8)] Mortal8(::core::primitive::u8),
                        #[codec(index = 9)] Mortal9(::core::primitive::u8),
                        #[codec(index = 10)] Mortal10(::core::primitive::u8),
                        #[codec(index = 11)] Mortal11(::core::primitive::u8),
                        #[codec(index = 12)] Mortal12(::core::primitive::u8),
                        #[codec(index = 13)] Mortal13(::core::primitive::u8),
                        #[codec(index = 14)] Mortal14(::core::primitive::u8),
                        #[codec(index = 15)] Mortal15(::core::primitive::u8),
                        #[codec(index = 16)] Mortal16(::core::primitive::u8),
                        #[codec(index = 17)] Mortal17(::core::primitive::u8),
                        #[codec(index = 18)] Mortal18(::core::primitive::u8),
                        #[codec(index = 19)] Mortal19(::core::primitive::u8),
                        #[codec(index = 20)] Mortal20(::core::primitive::u8),
                        #[codec(index = 21)] Mortal21(::core::primitive::u8),
                        #[codec(index = 22)] Mortal22(::core::primitive::u8),
                        #[codec(index = 23)] Mortal23(::core::primitive::u8),
                        #[codec(index = 24)] Mortal24(::core::primitive::u8),
                        #[codec(index = 25)] Mortal25(::core::primitive::u8),
                        #[codec(index = 26)] Mortal26(::core::primitive::u8),
                        #[codec(index = 27)] Mortal27(::core::primitive::u8),
                        #[codec(index = 28)] Mortal28(::core::primitive::u8),
                        #[codec(index = 29)] Mortal29(::core::primitive::u8),
                        #[codec(index = 30)] Mortal30(::core::primitive::u8),
                        #[codec(index = 31)] Mortal31(::core::primitive::u8),
                        #[codec(index = 32)] Mortal32(::core::primitive::u8),
                        #[codec(index = 33)] Mortal33(::core::primitive::u8),
                        #[codec(index = 34)] Mortal34(::core::primitive::u8),
                        #[codec(index = 35)] Mortal35(::core::primitive::u8),
                        #[codec(index = 36)] Mortal36(::core::primitive::u8),
                        #[codec(index = 37)] Mortal37(::core::primitive::u8),
                        #[codec(index = 38)] Mortal38(::core::primitive::u8),
                        #[codec(index = 39)] Mortal39(::core::primitive::u8),
                        #[codec(index = 40)] Mortal40(::core::primitive::u8),
                        #[codec(index = 41)] Mortal41(::core::primitive::u8),
                        #[codec(index = 42)] Mortal42(::core::primitive::u8),
                        #[codec(index = 43)] Mortal43(::core::primitive::u8),
                        #[codec(index = 44)] Mortal44(::core::primitive::u8),
                        #[codec(index = 45)] Mortal45(::core::primitive::u8),
                        #[codec(index = 46)] Mortal46(::core::primitive::u8),
                        #[codec(index = 47)] Mortal47(::core::primitive::u8),
                        #[codec(index = 48)] Mortal48(::core::primitive::u8),
                        #[codec(index = 49)] Mortal49(::core::primitive::u8),
                        #[codec(index = 50)] Mortal50(::core::primitive::u8),
                        #[codec(index = 51)] Mortal51(::core::primitive::u8),
                        #[codec(index = 52)] Mortal52(::core::primitive::u8),
                        #[codec(index = 53)] Mortal53(::core::primitive::u8),
                        #[codec(index = 54)] Mortal54(::core::primitive::u8),
                        #[codec(index = 55)] Mortal55(::core::primitive::u8),
                        #[codec(index = 56)] Mortal56(::core::primitive::u8),
                        #[codec(index = 57)] Mortal57(::core::primitive::u8),
                        #[codec(index = 58)] Mortal58(::core::primitive::u8),
                        #[codec(index = 59)] Mortal59(::core::primitive::u8),
                        #[codec(index = 60)] Mortal60(::core::primitive::u8),
                        #[codec(index = 61)] Mortal61(::core::primitive::u8),
                        #[codec(index = 62)] Mortal62(::core::primitive::u8),
                        #[codec(index = 63)] Mortal63(::core::primitive::u8),
                        #[codec(index = 64)] Mortal64(::core::primitive::u8),
                        #[codec(index = 65)] Mortal65(::core::primitive::u8),
                        #[codec(index = 66)] Mortal66(::core::primitive::u8),
                        #[codec(index = 67)] Mortal67(::core::primitive::u8),
                        #[codec(index = 68)] Mortal68(::core::primitive::u8),
                        #[codec(index = 69)] Mortal69(::core::primitive::u8),
                        #[codec(index = 70)] Mortal70(::core::primitive::u8),
                        #[codec(index = 71)] Mortal71(::core::primitive::u8),
                        #[codec(index = 72)] Mortal72(::core::primitive::u8),
                        #[codec(index = 73)] Mortal73(::core::primitive::u8),
                        #[codec(index = 74)] Mortal74(::core::primitive::u8),
                        #[codec(index = 75)] Mortal75(::core::primitive::u8),
                        #[codec(index = 76)] Mortal76(::core::primitive::u8),
                        #[codec(index = 77)] Mortal77(::core::primitive::u8),
                        #[codec(index = 78)] Mortal78(::core::primitive::u8),
                        #[codec(index = 79)] Mortal79(::core::primitive::u8),
                        #[codec(index = 80)] Mortal80(::core::primitive::u8),
                        #[codec(index = 81)] Mortal81(::core::primitive::u8),
                        #[codec(index = 82)] Mortal82(::core::primitive::u8),
                        #[codec(index = 83)] Mortal83(::core::primitive::u8),
                        #[codec(index = 84)] Mortal84(::core::primitive::u8),
                        #[codec(index = 85)] Mortal85(::core::primitive::u8),
                        #[codec(index = 86)] Mortal86(::core::primitive::u8),
                        #[codec(index = 87)] Mortal87(::core::primitive::u8),
                        #[codec(index = 88)] Mortal88(::core::primitive::u8),
                        #[codec(index = 89)] Mortal89(::core::primitive::u8),
                        #[codec(index = 90)] Mortal90(::core::primitive::u8),
                        #[codec(index = 91)] Mortal91(::core::primitive::u8),
                        #[codec(index = 92)] Mortal92(::core::primitive::u8),
                        #[codec(index = 93)] Mortal93(::core::primitive::u8),
                        #[codec(index = 94)] Mortal94(::core::primitive::u8),
                        #[codec(index = 95)] Mortal95(::core::primitive::u8),
                        #[codec(index = 96)] Mortal96(::core::primitive::u8),
                        #[codec(index = 97)] Mortal97(::core::primitive::u8),
                        #[codec(index = 98)] Mortal98(::core::primitive::u8),
                        #[codec(index = 99)] Mortal99(::core::primitive::u8),
                        #[codec(index = 100)] Mortal100(::core::primitive::u8),
                        #[codec(index = 101)] Mortal101(::core::primitive::u8),
                        #[codec(index = 102)] Mortal102(::core::primitive::u8),
                        #[codec(index = 103)] Mortal103(::core::primitive::u8),
                        #[codec(index = 104)] Mortal104(::core::primitive::u8),
                        #[codec(index = 105)] Mortal105(::core::primitive::u8),
                        #[codec(index = 106)] Mortal106(::core::primitive::u8),
                        #[codec(index = 107)] Mortal107(::core::primitive::u8),
                        #[codec(index = 108)] Mortal108(::core::primitive::u8),
                        #[codec(index = 109)] Mortal109(::core::primitive::u8),
                        #[codec(index = 110)] Mortal110(::core::primitive::u8),
                        #[codec(index = 111)] Mortal111(::core::primitive::u8),
                        #[codec(index = 112)] Mortal112(::core::primitive::u8),
                        #[codec(index = 113)] Mortal113(::core::primitive::u8),
                        #[codec(index = 114)] Mortal114(::core::primitive::u8),
                        #[codec(index = 115)] Mortal115(::core::primitive::u8),
                        #[codec(index = 116)] Mortal116(::core::primitive::u8),
                        #[codec(index = 117)] Mortal117(::core::primitive::u8),
                        #[codec(index = 118)] Mortal118(::core::primitive::u8),
                        #[codec(index = 119)] Mortal119(::core::primitive::u8),
                        #[codec(index = 120)] Mortal120(::core::primitive::u8),
                        #[codec(index = 121)] Mortal121(::core::primitive::u8),
                        #[codec(index = 122)] Mortal122(::core::primitive::u8),
                        #[codec(index = 123)] Mortal123(::core::primitive::u8),
                        #[codec(index = 124)] Mortal124(::core::primitive::u8),
                        #[codec(index = 125)] Mortal125(::core::primitive::u8),
                        #[codec(index = 126)] Mortal126(::core::primitive::u8),
                        #[codec(index = 127)] Mortal127(::core::primitive::u8),
                        #[codec(index = 128)] Mortal128(::core::primitive::u8),
                        #[codec(index = 129)] Mortal129(::core::primitive::u8),
                        #[codec(index = 130)] Mortal130(::core::primitive::u8),
                        #[codec(index = 131)] Mortal131(::core::primitive::u8),
                        #[codec(index = 132)] Mortal132(::core::primitive::u8),
                        #[codec(index = 133)] Mortal133(::core::primitive::u8),
                        #[codec(index = 134)] Mortal134(::core::primitive::u8),
                        #[codec(index = 135)] Mortal135(::core::primitive::u8),
                        #[codec(index = 136)] Mortal136(::core::primitive::u8),
                        #[codec(index = 137)] Mortal137(::core::primitive::u8),
                        #[codec(index = 138)] Mortal138(::core::primitive::u8),
                        #[codec(index = 139)] Mortal139(::core::primitive::u8),
                        #[codec(index = 140)] Mortal140(::core::primitive::u8),
                        #[codec(index = 141)] Mortal141(::core::primitive::u8),
                        #[codec(index = 142)] Mortal142(::core::primitive::u8),
                        #[codec(index = 143)] Mortal143(::core::primitive::u8),
                        #[codec(index = 144)] Mortal144(::core::primitive::u8),
                        #[codec(index = 145)] Mortal145(::core::primitive::u8),
                        #[codec(index = 146)] Mortal146(::core::primitive::u8),
                        #[codec(index = 147)] Mortal147(::core::primitive::u8),
                        #[codec(index = 148)] Mortal148(::core::primitive::u8),
                        #[codec(index = 149)] Mortal149(::core::primitive::u8),
                        #[codec(index = 150)] Mortal150(::core::primitive::u8),
                        #[codec(index = 151)] Mortal151(::core::primitive::u8),
                        #[codec(index = 152)] Mortal152(::core::primitive::u8),
                        #[codec(index = 153)] Mortal153(::core::primitive::u8),
                        #[codec(index = 154)] Mortal154(::core::primitive::u8),
                        #[codec(index = 155)] Mortal155(::core::primitive::u8),
                        #[codec(index = 156)] Mortal156(::core::primitive::u8),
                        #[codec(index = 157)] Mortal157(::core::primitive::u8),
                        #[codec(index = 158)] Mortal158(::core::primitive::u8),
                        #[codec(index = 159)] Mortal159(::core::primitive::u8),
                        #[codec(index = 160)] Mortal160(::core::primitive::u8),
                        #[codec(index = 161)] Mortal161(::core::primitive::u8),
                        #[codec(index = 162)] Mortal162(::core::primitive::u8),
                        #[codec(index = 163)] Mortal163(::core::primitive::u8),
                        #[codec(index = 164)] Mortal164(::core::primitive::u8),
                        #[codec(index = 165)] Mortal165(::core::primitive::u8),
                        #[codec(index = 166)] Mortal166(::core::primitive::u8),
                        #[codec(index = 167)] Mortal167(::core::primitive::u8),
                        #[codec(index = 168)] Mortal168(::core::primitive::u8),
                        #[codec(index = 169)] Mortal169(::core::primitive::u8),
                        #[codec(index = 170)] Mortal170(::core::primitive::u8),
                        #[codec(index = 171)] Mortal171(::core::primitive::u8),
                        #[codec(index = 172)] Mortal172(::core::primitive::u8),
                        #[codec(index = 173)] Mortal173(::core::primitive::u8),
                        #[codec(index = 174)] Mortal174(::core::primitive::u8),
                        #[codec(index = 175)] Mortal175(::core::primitive::u8),
                        #[codec(index = 176)] Mortal176(::core::primitive::u8),
                        #[codec(index = 177)] Mortal177(::core::primitive::u8),
                        #[codec(index = 178)] Mortal178(::core::primitive::u8),
                        #[codec(index = 179)] Mortal179(::core::primitive::u8),
                        #[codec(index = 180)] Mortal180(::core::primitive::u8),
                        #[codec(index = 181)] Mortal181(::core::primitive::u8),
                        #[codec(index = 182)] Mortal182(::core::primitive::u8),
                        #[codec(index = 183)] Mortal183(::core::primitive::u8),
                        #[codec(index = 184)] Mortal184(::core::primitive::u8),
                        #[codec(index = 185)] Mortal185(::core::primitive::u8),
                        #[codec(index = 186)] Mortal186(::core::primitive::u8),
                        #[codec(index = 187)] Mortal187(::core::primitive::u8),
                        #[codec(index = 188)] Mortal188(::core::primitive::u8),
                        #[codec(index = 189)] Mortal189(::core::primitive::u8),
                        #[codec(index = 190)] Mortal190(::core::primitive::u8),
                        #[codec(index = 191)] Mortal191(::core::primitive::u8),
                        #[codec(index = 192)] Mortal192(::core::primitive::u8),
                        #[codec(index = 193)] Mortal193(::core::primitive::u8),
                        #[codec(index = 194)] Mortal194(::core::primitive::u8),
                        #[codec(index = 195)] Mortal195(::core::primitive::u8),
                        #[codec(index = 196)] Mortal196(::core::primitive::u8),
                        #[codec(index = 197)] Mortal197(::core::primitive::u8),
                        #[codec(index = 198)] Mortal198(::core::primitive::u8),
                        #[codec(index = 199)] Mortal199(::core::primitive::u8),
                        #[codec(index = 200)] Mortal200(::core::primitive::u8),
                        #[codec(index = 201)] Mortal201(::core::primitive::u8),
                        #[codec(index = 202)] Mortal202(::core::primitive::u8),
                        #[codec(index = 203)] Mortal203(::core::primitive::u8),
                        #[codec(index = 204)] Mortal204(::core::primitive::u8),
                        #[codec(index = 205)] Mortal205(::core::primitive::u8),
                        #[codec(index = 206)] Mortal206(::core::primitive::u8),
                        #[codec(index = 207)] Mortal207(::core::primitive::u8),
                        #[codec(index = 208)] Mortal208(::core::primitive::u8),
                        #[codec(index = 209)] Mortal209(::core::primitive::u8),
                        #[codec(index = 210)] Mortal210(::core::primitive::u8),
                        #[codec(index = 211)] Mortal211(::core::primitive::u8),
                        #[codec(index = 212)] Mortal212(::core::primitive::u8),
                        #[codec(index = 213)] Mortal213(::core::primitive::u8),
                        #[codec(index = 214)] Mortal214(::core::primitive::u8),
                        #[codec(index = 215)] Mortal215(::core::primitive::u8),
                        #[codec(index = 216)] Mortal216(::core::primitive::u8),
                        #[codec(index = 217)] Mortal217(::core::primitive::u8),
                        #[codec(index = 218)] Mortal218(::core::primitive::u8),
                        #[codec(index = 219)] Mortal219(::core::primitive::u8),
                        #[codec(index = 220)] Mortal220(::core::primitive::u8),
                        #[codec(index = 221)] Mortal221(::core::primitive::u8),
                        #[codec(index = 222)] Mortal222(::core::primitive::u8),
                        #[codec(index = 223)] Mortal223(::core::primitive::u8),
                        #[codec(index = 224)] Mortal224(::core::primitive::u8),
                        #[codec(index = 225)] Mortal225(::core::primitive::u8),
                        #[codec(index = 226)] Mortal226(::core::primitive::u8),
                        #[codec(index = 227)] Mortal227(::core::primitive::u8),
                        #[codec(index = 228)] Mortal228(::core::primitive::u8),
                        #[codec(index = 229)] Mortal229(::core::primitive::u8),
                        #[codec(index = 230)] Mortal230(::core::primitive::u8),
                        #[codec(index = 231)] Mortal231(::core::primitive::u8),
                        #[codec(index = 232)] Mortal232(::core::primitive::u8),
                        #[codec(index = 233)] Mortal233(::core::primitive::u8),
                        #[codec(index = 234)] Mortal234(::core::primitive::u8),
                        #[codec(index = 235)] Mortal235(::core::primitive::u8),
                        #[codec(index = 236)] Mortal236(::core::primitive::u8),
                        #[codec(index = 237)] Mortal237(::core::primitive::u8),
                        #[codec(index = 238)] Mortal238(::core::primitive::u8),
                        #[codec(index = 239)] Mortal239(::core::primitive::u8),
                        #[codec(index = 240)] Mortal240(::core::primitive::u8),
                        #[codec(index = 241)] Mortal241(::core::primitive::u8),
                        #[codec(index = 242)] Mortal242(::core::primitive::u8),
                        #[codec(index = 243)] Mortal243(::core::primitive::u8),
                        #[codec(index = 244)] Mortal244(::core::primitive::u8),
                        #[codec(index = 245)] Mortal245(::core::primitive::u8),
                        #[codec(index = 246)] Mortal246(::core::primitive::u8),
                        #[codec(index = 247)] Mortal247(::core::primitive::u8),
                        #[codec(index = 248)] Mortal248(::core::primitive::u8),
                        #[codec(index = 249)] Mortal249(::core::primitive::u8),
                        #[codec(index = 250)] Mortal250(::core::primitive::u8),
                        #[codec(index = 251)] Mortal251(::core::primitive::u8),
                        #[codec(index = 252)] Mortal252(::core::primitive::u8),
                        #[codec(index = 253)] Mortal253(::core::primitive::u8),
                        #[codec(index = 254)] Mortal254(::core::primitive::u8),
                        #[codec(index = 255)] Mortal255(::core::primitive::u8),
                    }
                }

                pub mod header {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::ext::sp_core::H256,
                        #[codec(compact)] pub number: _0,
                        pub state_root: ::subxt::ext::sp_core::H256,
                        pub extrinsics_root: ::subxt::ext::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)] pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }

                pub mod unchecked_extrinsic {
                    use super::runtime_types;

                    #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3> (pub ::std::vec::Vec<::core::primitive::u8>, #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>);
                }
            }

            pub mod multiaddress {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub enum MultiAddress<_0, _1> { #[codec(index = 0)] Id(_0), #[codec(index = 1)] Index(#[codec(compact)] _1), #[codec(index = 2)] Raw(::std::vec::Vec<::core::primitive::u8>), #[codec(index = 3)] Address32([::core::primitive::u8; 32usize]), #[codec(index = 4)] Address20([::core::primitive::u8; 20usize]) }
            }

            pub mod traits {
                use super::runtime_types;

                #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
                pub struct BlakeTwo256;
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum ArithmeticError { #[codec(index = 0)] Underflow, #[codec(index = 1)] Overflow, #[codec(index = 2)] DivisionByZero }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum DispatchError { #[codec(index = 0)] Other, #[codec(index = 1)] CannotLookup, #[codec(index = 2)] BadOrigin, #[codec(index = 3)] Module(runtime_types::sp_runtime::ModuleError), #[codec(index = 4)] ConsumerRemaining, #[codec(index = 5)] NoProviders, #[codec(index = 6)] TooManyConsumers, #[codec(index = 7)] Token(runtime_types::sp_runtime::TokenError), #[codec(index = 8)] Arithmetic(runtime_types::sp_runtime::ArithmeticError), #[codec(index = 9)] Transactional(runtime_types::sp_runtime::TransactionalError) }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum MultiSignature { #[codec(index = 0)] Ed25519(runtime_types::sp_core::ed25519::Signature), #[codec(index = 1)] Sr25519(runtime_types::sp_core::sr25519::Signature), #[codec(index = 2)] Ecdsa(runtime_types::sp_core::ecdsa::Signature) }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum TokenError { #[codec(index = 0)] NoFunds, #[codec(index = 1)] WouldDie, #[codec(index = 2)] BelowMinimum, #[codec(index = 3)] CannotCreate, #[codec(index = 4)] UnknownAsset, #[codec(index = 5)] Frozen, #[codec(index = 6)] Unsupported }

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub enum TransactionalError { #[codec(index = 0)] LimitReached, #[codec(index = 1)] NoLayer }
        }

        pub mod sp_version {
            use super::runtime_types;

            #[derive(::subxt::ext::codec::Decode, ::subxt::ext::codec::Encode, Debug)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32, )>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
    }

    #[doc = r" The default error type returned when there is a runtime issue,"]
    #[doc = r" exposed here for ease of use."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;

    pub fn constants() -> ConstantsApi { ConstantsApi }

    pub fn storage() -> StorageApi { StorageApi }

    pub fn tx() -> TransactionApi { TransactionApi }

    pub struct ConstantsApi;

    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi { system::constants::ConstantsApi }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi { timestamp::constants::ConstantsApi }
        pub fn balances(&self) -> balances::constants::ConstantsApi { balances::constants::ConstantsApi }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi { authorship::constants::ConstantsApi }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi { transaction_payment::constants::ConstantsApi }
        pub fn contracts(&self) -> contracts::constants::ConstantsApi { contracts::constants::ConstantsApi }
    }

    pub struct StorageApi;

    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi { system::storage::StorageApi }
        pub fn randomness_collective_flip(&self) -> randomness_collective_flip::storage::StorageApi { randomness_collective_flip::storage::StorageApi }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi { timestamp::storage::StorageApi }
        pub fn balances(&self) -> balances::storage::StorageApi { balances::storage::StorageApi }
        pub fn authorship(&self) -> authorship::storage::StorageApi { authorship::storage::StorageApi }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi { transaction_payment::storage::StorageApi }
        pub fn sudo(&self) -> sudo::storage::StorageApi { sudo::storage::StorageApi }
        pub fn contracts(&self) -> contracts::storage::StorageApi { contracts::storage::StorageApi }
    }

    pub struct TransactionApi;

    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi { system::calls::TransactionApi }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi { timestamp::calls::TransactionApi }
        pub fn balances(&self) -> balances::calls::TransactionApi { balances::calls::TransactionApi }
        pub fn authorship(&self) -> authorship::calls::TransactionApi { authorship::calls::TransactionApi }
        pub fn sudo(&self) -> sudo::calls::TransactionApi { sudo::calls::TransactionApi }
        pub fn contracts(&self) -> contracts::calls::TransactionApi { contracts::calls::TransactionApi }
    }

    #[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
    pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(client: &C) -> Result<(), ::subxt::error::MetadataError> {
        let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
        if runtime_metadata_hash != [107u8, 60u8, 54u8, 83u8, 46u8, 188u8, 187u8, 179u8, 90u8, 168u8, 77u8, 163u8, 181u8, 134u8, 68u8, 181u8, 100u8, 67u8, 26u8, 120u8, 88u8, 254u8, 191u8, 177u8, 189u8, 239u8, 116u8, 99u8, 12u8, 153u8, 208u8, 133u8, ] { Err(::subxt::error::MetadataError::IncompatibleMetadata) } else { Ok(()) }
    }
}
