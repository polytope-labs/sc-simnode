#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
	#[allow(unused_imports)]
	mod root_mod {
		pub use super::*;
	}
	pub static PALLETS: [&str; 16usize] = [
		"System",
		"ParachainSystem",
		"Timestamp",
		"ParachainInfo",
		"Balances",
		"TransactionPayment",
		"Sudo",
		"Authorship",
		"CollatorSelection",
		"Session",
		"Aura",
		"AuraExt",
		"XcmpQueue",
		"PolkadotXcm",
		"CumulusXcm",
		"MessageQueue",
	];
	pub static RUNTIME_APIS: [&str; 0usize] = [];
	#[doc = r" The error type returned when there is a runtime issue."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	#[doc = r" The outer event enum."]
	pub type Event = runtime_types::parachain_runtime::RuntimeEvent;
	#[doc = r" The outer extrinsic enum."]
	pub type Call = runtime_types::parachain_runtime::RuntimeCall;
	#[doc = r" The outer error enum representing the DispatchError's Module variant."]
	pub type Error = runtime_types::parachain_runtime::RuntimeError;
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
	}
	pub fn apis() -> runtime_apis::RuntimeApi {
		runtime_apis::RuntimeApi
	}
	pub mod runtime_apis {
		use super::root_mod;
		use super::runtime_types;
		use ::subxt::ext::codec::Encode;
		pub struct RuntimeApi;
		impl RuntimeApi {}
	}
	pub struct ConstantsApi;
	impl ConstantsApi {
		pub fn system(&self) -> system::constants::ConstantsApi {
			system::constants::ConstantsApi
		}
		pub fn parachain_system(&self) -> parachain_system::constants::ConstantsApi {
			parachain_system::constants::ConstantsApi
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
			timestamp::constants::ConstantsApi
		}
		pub fn balances(&self) -> balances::constants::ConstantsApi {
			balances::constants::ConstantsApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
			transaction_payment::constants::ConstantsApi
		}
		pub fn collator_selection(&self) -> collator_selection::constants::ConstantsApi {
			collator_selection::constants::ConstantsApi
		}
		pub fn session(&self) -> session::constants::ConstantsApi {
			session::constants::ConstantsApi
		}
		pub fn aura(&self) -> aura::constants::ConstantsApi {
			aura::constants::ConstantsApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::constants::ConstantsApi {
			xcmp_queue::constants::ConstantsApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::constants::ConstantsApi {
			polkadot_xcm::constants::ConstantsApi
		}
		pub fn message_queue(&self) -> message_queue::constants::ConstantsApi {
			message_queue::constants::ConstantsApi
		}
	}
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn parachain_system(&self) -> parachain_system::storage::StorageApi {
			parachain_system::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn parachain_info(&self) -> parachain_info::storage::StorageApi {
			parachain_info::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn collator_selection(&self) -> collator_selection::storage::StorageApi {
			collator_selection::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn aura(&self) -> aura::storage::StorageApi {
			aura::storage::StorageApi
		}
		pub fn aura_ext(&self) -> aura_ext::storage::StorageApi {
			aura_ext::storage::StorageApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi {
			xcmp_queue::storage::StorageApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::storage::StorageApi {
			polkadot_xcm::storage::StorageApi
		}
		pub fn message_queue(&self) -> message_queue::storage::StorageApi {
			message_queue::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi {
			parachain_system::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn parachain_info(&self) -> parachain_info::calls::TransactionApi {
			parachain_info::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi {
			collator_selection::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi {
			xcmp_queue::calls::TransactionApi
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi {
			polkadot_xcm::calls::TransactionApi
		}
		pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi {
			cumulus_xcm::calls::TransactionApi
		}
		pub fn message_queue(&self) -> message_queue::calls::TransactionApi {
			message_queue::calls::TransactionApi
		}
	}
	#[doc = r" check whether the metadata provided is aligned with this statically generated code."]
	pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
		let runtime_metadata_hash = metadata
			.hasher()
			.only_these_pallets(&PALLETS)
			.only_these_runtime_apis(&RUNTIME_APIS)
			.hash();
		runtime_metadata_hash
			== [
				198u8, 208u8, 190u8, 214u8, 89u8, 123u8, 78u8, 91u8, 57u8, 5u8, 58u8, 9u8, 83u8,
				244u8, 111u8, 130u8, 75u8, 141u8, 165u8, 208u8, 218u8, 95u8, 82u8, 34u8, 4u8, 71u8,
				65u8, 29u8, 34u8, 27u8, 45u8, 110u8,
			]
	}
	pub mod system {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the System pallet"]
		pub type Error = runtime_types::frame_system::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::frame_system::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Remark {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Remark {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetHeapPages {
					pub pages: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_heap_pages";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCode {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCode {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCodeWithoutChecks {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_code_without_checks";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetStorage {
					pub items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "set_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KillStorage {
					pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillStorage {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_storage";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KillPrefix {
					pub prefix: ::std::vec::Vec<::core::primitive::u8>,
					pub subkeys: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "kill_prefix";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemarkWithEvent {
					pub remark: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "remark_with_event";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AuthorizeUpgrade {
					pub code_hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for AuthorizeUpgrade {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "authorize_upgrade";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AuthorizeUpgradeWithoutChecks {
					pub code_hash: ::subxt::utils::H256,
				}
				impl ::subxt::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "authorize_upgrade_without_checks";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ApplyAuthorizedUpgrade {
					pub code: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
					const PALLET: &'static str = "System";
					const CALL: &'static str = "apply_authorized_upgrade";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "Can be executed by every `origin`."]
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::Remark> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark",
						types::Remark { remark },
						[
							43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
							216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
							250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
							13u8,
						],
					)
				}
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::Payload<types::SetHeapPages> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_heap_pages",
						types::SetHeapPages { pages },
						[
							188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
							215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
							134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
							57u8, 147u8,
						],
					)
				}
				#[doc = "Set the new runtime code."]
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCode> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code",
						types::SetCode { code },
						[
							233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
							203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
							27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
						],
					)
				}
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
				#[doc = "version!"]
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_code_without_checks",
						types::SetCodeWithoutChecks { code },
						[
							82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
							157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
							147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
							115u8,
						],
					)
				}
				#[doc = "Set some items of storage."]
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::Payload<types::SetStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"set_storage",
						types::SetStorage { items },
						[
							141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
							163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
							150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
							234u8, 43u8,
						],
					)
				}
				#[doc = "Kill some items from storage."]
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::Payload<types::KillStorage> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_storage",
						types::KillStorage { keys },
						[
							73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
							234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
							156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
							35u8,
						],
					)
				}
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::KillPrefix> {
					::subxt::tx::Payload::new_static(
						"System",
						"kill_prefix",
						types::KillPrefix { prefix, subkeys },
						[
							184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
							175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
							67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
							85u8,
						],
					)
				}
				#[doc = "Make some on-chain remark and emit event."]
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
					::subxt::tx::Payload::new_static(
						"System",
						"remark_with_event",
						types::RemarkWithEvent { remark },
						[
							120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
							228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
							147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
						],
					)
				}
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub fn authorize_upgrade(
					&self,
					code_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::AuthorizeUpgrade> {
					::subxt::tx::Payload::new_static(
						"System",
						"authorize_upgrade",
						types::AuthorizeUpgrade { code_hash },
						[
							4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
							254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
							58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
							172u8,
						],
					)
				}
				#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
				#[doc = "later."]
				#[doc = ""]
				#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
				#[doc = "example that the spec name remains the same and that the version number increases. Not"]
				#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
				#[doc = ""]
				#[doc = "This call requires Root origin."]
				pub fn authorize_upgrade_without_checks(
					&self,
					code_hash: ::subxt::utils::H256,
				) -> ::subxt::tx::Payload<types::AuthorizeUpgradeWithoutChecks> {
					::subxt::tx::Payload::new_static(
						"System",
						"authorize_upgrade_without_checks",
						types::AuthorizeUpgradeWithoutChecks { code_hash },
						[
							126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
							136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
							197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
							233u8,
						],
					)
				}
				#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
				#[doc = ""]
				#[doc = "If the authorization required a version check, this call will ensure the spec name"]
				#[doc = "remains unchanged and that the spec version has increased."]
				#[doc = ""]
				#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
				#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
				#[doc = ""]
				#[doc = "All origins are allowed."]
				pub fn apply_authorized_upgrade(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::ApplyAuthorizedUpgrade> {
					::subxt::tx::Payload::new_static(
						"System",
						"apply_authorized_upgrade",
						types::ApplyAuthorizedUpgrade { code },
						[
							232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8,
							156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8,
							32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_system::DispatchEventInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_system::DispatchEventInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "`:code` was updated to the code with the given hash."]
			pub struct CodeUpdated {
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "On on-chain remark happened."]
			pub struct Remarked {
				pub sender: ::subxt::utils::AccountId32,
				pub hash: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An upgrade was authorized."]
			pub struct UpgradeAuthorized {
				pub code_hash: ::subxt::utils::H256,
				pub check_version: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for UpgradeAuthorized {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An invalid authorized upgrade was rejected while trying to apply it."]
			pub struct RejectedInvalidAuthorizedUpgrade {
				pub code_hash: ::subxt::utils::H256,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for RejectedInvalidAuthorizedUpgrade {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "RejectedInvalidAuthorizedUpgrade";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
							175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
							124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
						],
					)
				}
				#[doc = " The full account information for a particular account ID."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Account",
						Vec::new(),
						[
							14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
							175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
							124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
						],
					)
				}
				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicCount",
						vec![],
						[
							102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
							153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
							120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
							159u8, 79u8,
						],
					)
				}
				#[doc = " Whether all inherents have been applied."]
				pub fn inherents_applied(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"InherentsApplied",
						vec![],
						[
							132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8,
							223u8, 101u8, 55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8,
							109u8, 34u8, 65u8, 185u8, 150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
						],
					)
				}
				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::sp_weights::weight_v2::Weight,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockWeight",
						vec![],
						[
							158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
							62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
							229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
						],
					)
				}
				#[doc = " Total size (in bytes) of the current block."]
				#[doc = ""]
				#[doc = " Tracks the size of the header and all extrinsics."]
				pub fn block_size(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockSize",
						vec![],
						[
							77u8, 144u8, 151u8, 134u8, 144u8, 38u8, 126u8, 109u8, 143u8, 21u8,
							62u8, 112u8, 157u8, 215u8, 82u8, 57u8, 128u8, 199u8, 44u8, 36u8, 56u8,
							29u8, 160u8, 127u8, 93u8, 214u8, 74u8, 241u8, 114u8, 253u8, 244u8,
							33u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlockHash",
						Vec::new(),
						[
							217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
							103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
							164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
							202u8, 118u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
							220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
							128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
						],
					)
				}
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Number",
						vec![],
						[
							30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
							9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
							200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
						],
					)
				}
				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::H256,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ParentHash",
						vec![],
						[
							26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
							192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
							71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
						],
					)
				}
				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_runtime::generic::digest::Digest,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Digest",
						vec![],
						[
							61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
							91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
							58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
						],
					)
				}
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::parachain_runtime::RuntimeEvent,
							::subxt::utils::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"Events",
						vec![],
						[
							90u8, 184u8, 128u8, 30u8, 169u8, 227u8, 35u8, 99u8, 119u8, 120u8, 27u8,
							82u8, 130u8, 120u8, 172u8, 142u8, 44u8, 6u8, 253u8, 169u8, 187u8, 55u8,
							220u8, 186u8, 46u8, 93u8, 128u8, 125u8, 11u8, 191u8, 116u8, 192u8,
						],
					)
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventCount",
						vec![],
						[
							175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
							151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
							254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
							133u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
							133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
							120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
						],
					)
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(BlockNumberFor<T>, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"EventTopics",
						Vec::new(),
						[
							40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
							133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
							120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
						],
					)
				}
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::LastRuntimeUpgradeInfo,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"LastRuntimeUpgrade",
						vec![],
						[
							197u8, 212u8, 249u8, 209u8, 79u8, 34u8, 55u8, 203u8, 31u8, 42u8, 199u8,
							242u8, 188u8, 74u8, 234u8, 250u8, 245u8, 44u8, 139u8, 162u8, 45u8,
							150u8, 230u8, 249u8, 135u8, 100u8, 158u8, 167u8, 118u8, 219u8, 28u8,
							98u8,
						],
					)
				}
				#[doc = " Number of blocks till the pending code upgrade is applied."]
				pub fn blocks_till_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u8,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"BlocksTillUpgrade",
						vec![],
						[
							184u8, 49u8, 238u8, 134u8, 27u8, 13u8, 184u8, 216u8, 19u8, 142u8,
							140u8, 16u8, 147u8, 29u8, 112u8, 20u8, 53u8, 194u8, 123u8, 137u8, 79u8,
							56u8, 139u8, 136u8, 50u8, 155u8, 131u8, 108u8, 62u8, 230u8, 106u8,
							173u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToU32RefCount",
						vec![],
						[
							229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
							130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
							107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"UpgradedToTripleRefCount",
						vec![],
						[
							97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
							101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
							167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
							151u8,
						],
					)
				}
				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::Phase,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExecutionPhase",
						vec![],
						[
							191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
							0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
							35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
						],
					)
				}
				#[doc = " `Some` if a code upgrade has been authorized."]
				pub fn authorized_upgrade(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::frame_system::CodeUpgradeAuthorization,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"AuthorizedUpgrade",
						vec![],
						[
							165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
							169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
							214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
						],
					)
				}
				#[doc = " The weight reclaimed for the extrinsic."]
				#[doc = ""]
				#[doc = " This information is available until the end of the extrinsic execution."]
				#[doc = " More precisely this information is removed in `note_applied_extrinsic`."]
				#[doc = ""]
				#[doc = " Logic doing some post dispatch weight reduction must update this storage to avoid duplicate"]
				#[doc = " reduction."]
				pub fn extrinsic_weight_reclaimed(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_weights::weight_v2::Weight,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"System",
						"ExtrinsicWeightReclaimed",
						vec![],
						[
							195u8, 143u8, 164u8, 84u8, 225u8, 194u8, 227u8, 128u8, 196u8, 241u8,
							188u8, 159u8, 59u8, 197u8, 11u8, 12u8, 119u8, 164u8, 46u8, 229u8, 92u8,
							212u8, 236u8, 255u8, 238u8, 54u8, 105u8, 200u8, 229u8, 191u8, 221u8,
							202u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
				{
					::subxt::constants::Address::new_static(
						"System",
						"BlockWeights",
						[
							176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
							190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
							163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
						],
					)
				}
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
				{
					::subxt::constants::Address::new_static(
						"System",
						"BlockLength",
						[
							25u8, 97u8, 176u8, 77u8, 2u8, 60u8, 44u8, 69u8, 161u8, 69u8, 251u8,
							229u8, 198u8, 186u8, 185u8, 237u8, 105u8, 56u8, 122u8, 35u8, 78u8,
							195u8, 98u8, 222u8, 215u8, 49u8, 249u8, 146u8, 231u8, 21u8, 224u8,
							134u8,
						],
					)
				}
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight> {
					::subxt::constants::Address::new_static(
						"System",
						"DbWeight",
						[
							42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
							200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
							183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
							177u8,
						],
					)
				}
				#[doc = " Get the chain's in-code version."]
				pub fn version(
					&self,
				) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion> {
					::subxt::constants::Address::new_static(
						"System",
						"Version",
						[
							214u8, 43u8, 96u8, 193u8, 96u8, 213u8, 63u8, 124u8, 22u8, 111u8, 41u8,
							78u8, 146u8, 77u8, 34u8, 163u8, 117u8, 100u8, 6u8, 216u8, 238u8, 54u8,
							80u8, 185u8, 219u8, 11u8, 192u8, 200u8, 129u8, 88u8, 161u8, 250u8,
						],
					)
				}
				#[doc = " The designated SS58 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
					::subxt::constants::Address::new_static(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_system {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::cumulus_pallet_parachain_system::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::cumulus_pallet_parachain_system::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetValidationData { pub data : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: BasicParachainInherentData , pub inbound_messages_data : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: InboundMessagesData , }
				impl ::subxt::blocks::StaticExtrinsic for SetValidationData {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "set_validation_data";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoSendUpwardMessage {
					pub message: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoSendUpwardMessage {
					const PALLET: &'static str = "ParachainSystem";
					const CALL: &'static str = "sudo_send_upward_message";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current validation data."]
				#[doc = ""]
				#[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase if the call was not invoked."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`"]
				#[doc = ""]
				#[doc = "As a side effect, this function upgrades the current validation function"]
				#[doc = "if the appropriate time has come."]
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: BasicParachainInherentData,
					inbound_messages_data : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: InboundMessagesData,
				) -> ::subxt::tx::Payload<types::SetValidationData> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"set_validation_data",
						types::SetValidationData { data, inbound_messages_data },
						[
							24u8, 248u8, 40u8, 54u8, 164u8, 120u8, 191u8, 152u8, 197u8, 48u8, 72u8,
							153u8, 142u8, 201u8, 158u8, 234u8, 61u8, 234u8, 41u8, 241u8, 25u8,
							129u8, 21u8, 111u8, 226u8, 120u8, 125u8, 118u8, 216u8, 226u8, 70u8,
							28u8,
						],
					)
				}
				pub fn sudo_send_upward_message(
					&self,
					message: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SudoSendUpwardMessage> {
					::subxt::tx::Payload::new_static(
						"ParachainSystem",
						"sudo_send_upward_message",
						types::SudoSendUpwardMessage { message },
						[
							1u8, 231u8, 11u8, 78u8, 127u8, 117u8, 248u8, 67u8, 230u8, 199u8, 126u8,
							47u8, 20u8, 62u8, 252u8, 138u8, 199u8, 48u8, 41u8, 21u8, 28u8, 157u8,
							218u8, 143u8, 4u8, 253u8, 62u8, 192u8, 94u8, 252u8, 92u8, 180u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The validation function has been scheduled to apply."]
			pub struct ValidationFunctionStored;
			impl ::subxt::events::StaticEvent for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The validation function was applied as of the contained relay chain block number."]
			pub struct ValidationFunctionApplied {
				pub relay_chain_block_num: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The relay-chain aborted the upgrade process."]
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::events::StaticEvent for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some downward messages have been received and will be processed."]
			pub struct DownwardMessagesReceived {
				pub count: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Downward messages were processed using the given weight."]
			pub struct DownwardMessagesProcessed {
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
				pub dmq_head: ::subxt::utils::H256,
			}
			impl ::subxt::events::StaticEvent for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An upward message was sent to the relay chain."]
			pub struct UpwardMessageSent {
				pub message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
			}
			impl ::subxt::events::StaticEvent for UpwardMessageSent {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpwardMessageSent";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current block weight mode."]
				#[doc = ""]
				#[doc = " This is used to determine what is the maximum allowed block weight, for more information see"]
				#[doc = " [`block_weight`]."]
				#[doc = ""]
				#[doc = " Killed in [`Self::on_initialize`] and set by the [`block_weight`] logic."]
				pub fn block_weight_mode(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_parachain_system::block_weight::BlockWeightMode,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"BlockWeightMode",
						vec![],
						[
							140u8, 255u8, 133u8, 169u8, 34u8, 223u8, 67u8, 94u8, 116u8, 29u8,
							151u8, 29u8, 168u8, 9u8, 107u8, 124u8, 17u8, 176u8, 119u8, 163u8, 31u8,
							105u8, 199u8, 98u8, 62u8, 68u8, 179u8, 170u8, 135u8, 237u8, 30u8,
							141u8,
						],
					)
				}
				#[doc = " The core count available to the parachain in the previous block."]
				#[doc = ""]
				#[doc = " This is mainly used for offchain functionality to calculate the correct target block weight."]
				pub fn previous_core_count(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::ext::codec::Compact<::core::primitive::u16>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PreviousCoreCount",
						vec![],
						[
							157u8, 187u8, 34u8, 152u8, 156u8, 193u8, 155u8, 209u8, 202u8, 3u8, 6u8,
							183u8, 125u8, 41u8, 106u8, 26u8, 235u8, 240u8, 127u8, 211u8, 140u8,
							187u8, 42u8, 242u8, 229u8, 153u8, 29u8, 198u8, 53u8, 189u8, 228u8,
							224u8,
						],
					)
				}
				#[doc = " Latest included block descendants the runtime accepted. In other words, these are"]
				#[doc = " ancestors of the currently executing block which have not been included in the observed"]
				#[doc = " relay-chain state."]
				#[doc = ""]
				#[doc = " The segment length is limited by the capacity returned from the [`ConsensusHook`] configured"]
				#[doc = " in the pallet."]				pub fn unincluded_segment (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , :: std :: vec :: Vec < runtime_types :: cumulus_pallet_parachain_system :: unincluded_segment :: Ancestor < :: subxt :: utils :: H256 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UnincludedSegment",
						vec![],
						[
							73u8, 83u8, 226u8, 16u8, 203u8, 233u8, 221u8, 109u8, 23u8, 114u8, 56u8,
							154u8, 100u8, 116u8, 253u8, 10u8, 164u8, 22u8, 110u8, 73u8, 245u8,
							226u8, 54u8, 146u8, 67u8, 109u8, 149u8, 142u8, 154u8, 218u8, 55u8,
							178u8,
						],
					)
				}
				#[doc = " Storage field that keeps track of bandwidth used by the unincluded segment along with the"]
				#[doc = " latest HRMP watermark. Used for limiting the acceptance of new blocks with"]
				#[doc = " respect to relay chain constraints."]				pub fn aggregated_unincluded_segment (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: unincluded_segment :: SegmentTracker < :: subxt :: utils :: H256 > , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"AggregatedUnincludedSegment",
						vec![],
						[
							165u8, 51u8, 182u8, 156u8, 65u8, 114u8, 167u8, 133u8, 245u8, 52u8,
							32u8, 119u8, 159u8, 65u8, 201u8, 108u8, 99u8, 43u8, 84u8, 63u8, 95u8,
							182u8, 134u8, 163u8, 51u8, 202u8, 243u8, 82u8, 225u8, 192u8, 186u8,
							2u8,
						],
					)
				}
				#[doc = " In case of a scheduled upgrade, this storage field contains the validation code to be"]
				#[doc = " applied."]
				#[doc = ""]
				#[doc = " As soon as the relay chain gives us the go-ahead signal, we will overwrite the"]
				#[doc = " [`:pending_code`][sp_core::storage::well_known_keys::PENDING_CODE] which will result the"]
				#[doc = " next block to be processed with the new validation code. This concludes the upgrade process."]
				pub fn pending_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingValidationCode",
						vec![],
						[
							78u8, 159u8, 219u8, 211u8, 177u8, 80u8, 102u8, 93u8, 83u8, 146u8, 90u8,
							233u8, 232u8, 11u8, 104u8, 172u8, 93u8, 68u8, 44u8, 228u8, 99u8, 197u8,
							254u8, 28u8, 181u8, 215u8, 247u8, 238u8, 49u8, 49u8, 195u8, 249u8,
						],
					)
				}
				#[doc = " Validation code that is set by the parachain and is to be communicated to collator and"]
				#[doc = " consequently the relay-chain."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block if no other pallet already set"]
				#[doc = " the value."]
				pub fn new_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"NewValidationCode",
						vec![],
						[
							185u8, 123u8, 152u8, 122u8, 230u8, 136u8, 79u8, 73u8, 206u8, 19u8,
							59u8, 57u8, 75u8, 250u8, 83u8, 185u8, 29u8, 76u8, 89u8, 137u8, 77u8,
							163u8, 25u8, 125u8, 182u8, 67u8, 2u8, 180u8, 48u8, 237u8, 49u8, 171u8,
						],
					)
				}
				#[doc = " The [`PersistedValidationData`] set for this block."]
				#[doc = ""]
				#[doc = " This value is expected to be set only once by the [`Pallet::set_validation_data`] inherent."]
				pub fn validation_data(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v9::PersistedValidationData<
						::subxt::utils::H256,
						::core::primitive::u32,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ValidationData",
						vec![],
						[
							193u8, 240u8, 25u8, 56u8, 103u8, 173u8, 56u8, 56u8, 229u8, 243u8, 91u8,
							25u8, 249u8, 95u8, 122u8, 93u8, 37u8, 181u8, 54u8, 244u8, 217u8, 200u8,
							62u8, 136u8, 80u8, 148u8, 16u8, 177u8, 124u8, 211u8, 95u8, 24u8,
						],
					)
				}
				#[doc = " Were the validation data set to notify the relay chain?"]
				pub fn did_set_validation_code(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"DidSetValidationCode",
						vec![],
						[
							233u8, 228u8, 48u8, 111u8, 200u8, 35u8, 30u8, 139u8, 251u8, 77u8,
							196u8, 252u8, 35u8, 222u8, 129u8, 235u8, 7u8, 19u8, 156u8, 82u8, 126u8,
							173u8, 29u8, 62u8, 20u8, 67u8, 166u8, 116u8, 108u8, 182u8, 57u8, 246u8,
						],
					)
				}
				#[doc = " The relay chain block number associated with the last parachain block."]
				#[doc = ""]
				#[doc = " This is updated in `on_finalize`."]
				pub fn last_relay_chain_block_number(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastRelayChainBlockNumber",
						vec![],
						[
							17u8, 65u8, 131u8, 169u8, 195u8, 243u8, 195u8, 93u8, 220u8, 174u8,
							75u8, 216u8, 214u8, 227u8, 96u8, 40u8, 8u8, 153u8, 116u8, 160u8, 79u8,
							255u8, 35u8, 232u8, 242u8, 42u8, 100u8, 150u8, 208u8, 210u8, 142u8,
							186u8,
						],
					)
				}
				#[doc = " An option which indicates if the relay-chain restricts signalling a validation code upgrade."]
				#[doc = " In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced"]
				#[doc = " candidate will be invalid."]
				#[doc = ""]
				#[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
				#[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
				#[doc = " set after the inherent."]
				pub fn upgrade_restriction_signal(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<
						runtime_types::polkadot_primitives::v9::UpgradeRestriction,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpgradeRestrictionSignal",
						vec![],
						[
							235u8, 240u8, 37u8, 44u8, 181u8, 52u8, 7u8, 216u8, 20u8, 139u8, 69u8,
							124u8, 21u8, 173u8, 237u8, 64u8, 105u8, 88u8, 49u8, 69u8, 123u8, 55u8,
							181u8, 167u8, 112u8, 183u8, 190u8, 231u8, 231u8, 127u8, 77u8, 148u8,
						],
					)
				}
				#[doc = " Optional upgrade go-ahead signal from the relay-chain."]
				#[doc = ""]
				#[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
				#[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
				#[doc = " set after the inherent."]
				pub fn upgrade_go_ahead(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::option::Option<runtime_types::polkadot_primitives::v9::UpgradeGoAhead>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpgradeGoAhead",
						vec![],
						[
							149u8, 144u8, 186u8, 88u8, 180u8, 34u8, 82u8, 226u8, 100u8, 148u8,
							246u8, 55u8, 233u8, 97u8, 43u8, 0u8, 48u8, 31u8, 69u8, 154u8, 29u8,
							147u8, 241u8, 91u8, 81u8, 126u8, 206u8, 117u8, 14u8, 149u8, 87u8, 88u8,
						],
					)
				}
				#[doc = " The state proof for the last relay parent block."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn relay_state_proof(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_trie::storage_proof::StorageProof,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"RelayStateProof",
						vec![],
						[
							46u8, 115u8, 163u8, 190u8, 246u8, 47u8, 200u8, 159u8, 206u8, 204u8,
							94u8, 250u8, 127u8, 112u8, 109u8, 111u8, 210u8, 195u8, 244u8, 41u8,
							36u8, 187u8, 71u8, 150u8, 149u8, 253u8, 143u8, 33u8, 83u8, 189u8,
							182u8, 238u8,
						],
					)
				}
				#[doc = " The snapshot of some state related to messaging relevant to the current parachain as per"]
				#[doc = " the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]				pub fn relevant_messaging_state (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"RelevantMessagingState",
						vec![],
						[
							117u8, 166u8, 186u8, 126u8, 21u8, 174u8, 86u8, 253u8, 163u8, 90u8,
							54u8, 226u8, 186u8, 253u8, 126u8, 168u8, 145u8, 45u8, 155u8, 32u8,
							97u8, 110u8, 208u8, 125u8, 47u8, 113u8, 165u8, 199u8, 210u8, 118u8,
							217u8, 73u8,
						],
					)
				}
				#[doc = " The parachain host configuration that was obtained from the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub fn host_configuration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_primitives::v9::AbridgedHostConfiguration,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HostConfiguration",
						vec![],
						[
							252u8, 23u8, 111u8, 189u8, 120u8, 204u8, 129u8, 223u8, 248u8, 179u8,
							239u8, 173u8, 133u8, 61u8, 140u8, 2u8, 75u8, 32u8, 204u8, 178u8, 69u8,
							21u8, 44u8, 227u8, 178u8, 179u8, 33u8, 26u8, 131u8, 156u8, 78u8, 85u8,
						],
					)
				}
				#[doc = " The last downward message queue chain head we have observed."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_dmq_mqc_head(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastDmqMqcHead",
						vec![],
						[
							1u8, 70u8, 140u8, 40u8, 51u8, 127u8, 75u8, 80u8, 5u8, 49u8, 196u8,
							31u8, 30u8, 61u8, 54u8, 252u8, 0u8, 0u8, 100u8, 115u8, 177u8, 250u8,
							138u8, 48u8, 107u8, 41u8, 93u8, 87u8, 195u8, 107u8, 206u8, 227u8,
						],
					)
				}
				#[doc = " The message queue chain heads we have observed per each channel incoming channel."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub fn last_hrmp_mqc_heads(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::KeyedVec<
						runtime_types::polkadot_parachain_primitives::primitives::Id,
						runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastHrmpMqcHeads",
						vec![],
						[
							131u8, 170u8, 142u8, 30u8, 101u8, 113u8, 131u8, 81u8, 38u8, 168u8,
							98u8, 3u8, 9u8, 109u8, 96u8, 179u8, 115u8, 177u8, 128u8, 11u8, 238u8,
							54u8, 81u8, 60u8, 97u8, 112u8, 224u8, 175u8, 86u8, 133u8, 182u8, 76u8,
						],
					)
				}
				#[doc = " Number of downward messages processed in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn processed_downward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ProcessedDownwardMessages",
						vec![],
						[
							151u8, 234u8, 196u8, 87u8, 130u8, 79u8, 4u8, 102u8, 47u8, 10u8, 33u8,
							132u8, 149u8, 118u8, 61u8, 141u8, 5u8, 1u8, 30u8, 120u8, 220u8, 156u8,
							16u8, 11u8, 14u8, 52u8, 126u8, 151u8, 244u8, 149u8, 197u8, 51u8,
						],
					)
				}
				#[doc = " The last processed downward message."]
				#[doc = ""]
				#[doc = " We need to keep track of this to filter the messages that have been already processed."]				pub fn last_processed_downward_message (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: InboundMessageId , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastProcessedDownwardMessage",
						vec![],
						[
							75u8, 141u8, 16u8, 195u8, 57u8, 191u8, 162u8, 116u8, 7u8, 137u8, 238u8,
							206u8, 120u8, 162u8, 226u8, 113u8, 40u8, 71u8, 47u8, 192u8, 124u8,
							120u8, 66u8, 84u8, 172u8, 32u8, 122u8, 251u8, 26u8, 6u8, 52u8, 151u8,
						],
					)
				}
				#[doc = " HRMP watermark that was set in a block."]
				pub fn hrmp_watermark(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HrmpWatermark",
						vec![],
						[
							77u8, 62u8, 59u8, 220u8, 7u8, 125u8, 98u8, 249u8, 108u8, 212u8, 223u8,
							99u8, 152u8, 13u8, 29u8, 80u8, 166u8, 65u8, 232u8, 113u8, 145u8, 128u8,
							123u8, 35u8, 238u8, 31u8, 113u8, 156u8, 220u8, 104u8, 217u8, 165u8,
						],
					)
				}
				#[doc = " The last processed HRMP message."]
				#[doc = ""]
				#[doc = " We need to keep track of this to filter the messages that have been already processed."]				pub fn last_processed_hrmp_message (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: InboundMessageId , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"LastProcessedHrmpMessage",
						vec![],
						[
							174u8, 198u8, 88u8, 234u8, 120u8, 225u8, 81u8, 245u8, 0u8, 11u8, 124u8,
							147u8, 11u8, 224u8, 133u8, 151u8, 152u8, 194u8, 195u8, 99u8, 61u8,
							34u8, 142u8, 173u8, 241u8, 198u8, 19u8, 136u8, 127u8, 160u8, 173u8,
							111u8,
						],
					)
				}
				#[doc = " HRMP messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub fn hrmp_outbound_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
							runtime_types::polkadot_parachain_primitives::primitives::Id,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"HrmpOutboundMessages",
						vec![],
						[
							42u8, 9u8, 96u8, 217u8, 25u8, 101u8, 129u8, 147u8, 150u8, 20u8, 164u8,
							186u8, 217u8, 178u8, 15u8, 201u8, 233u8, 104u8, 92u8, 120u8, 29u8,
							245u8, 196u8, 13u8, 141u8, 210u8, 102u8, 62u8, 216u8, 80u8, 246u8,
							145u8,
						],
					)
				}
				#[doc = " Upward messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` for each new block."]
				pub fn upward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpwardMessages",
						vec![],
						[
							179u8, 127u8, 8u8, 94u8, 194u8, 246u8, 53u8, 79u8, 80u8, 22u8, 18u8,
							75u8, 116u8, 163u8, 90u8, 161u8, 30u8, 140u8, 57u8, 126u8, 60u8, 91u8,
							23u8, 30u8, 120u8, 245u8, 125u8, 96u8, 152u8, 25u8, 248u8, 85u8,
						],
					)
				}
				#[doc = " Upward messages that are still pending and not yet sent to the relay chain."]
				pub fn pending_upward_messages(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingUpwardMessages",
						vec![],
						[
							239u8, 45u8, 18u8, 173u8, 148u8, 150u8, 55u8, 176u8, 173u8, 156u8,
							246u8, 226u8, 198u8, 214u8, 104u8, 187u8, 186u8, 13u8, 83u8, 194u8,
							153u8, 29u8, 228u8, 109u8, 26u8, 18u8, 212u8, 151u8, 246u8, 24u8,
							133u8, 216u8,
						],
					)
				}
				#[doc = " Upward signals that are still pending and not yet sent to the relay chain."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_finalize` for each block."]
				pub fn pending_upward_signals(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingUpwardSignals",
						vec![],
						[
							147u8, 182u8, 136u8, 253u8, 198u8, 173u8, 0u8, 194u8, 95u8, 74u8,
							136u8, 176u8, 60u8, 121u8, 145u8, 69u8, 223u8, 150u8, 137u8, 241u8,
							17u8, 38u8, 160u8, 116u8, 238u8, 242u8, 123u8, 169u8, 170u8, 244u8,
							83u8, 152u8,
						],
					)
				}
				#[doc = " The approved peer id to be sent as a UMP signal on the last block of the PoV."]
				pub fn pending_approved_peer(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PendingApprovedPeer",
						vec![],
						[
							131u8, 155u8, 118u8, 221u8, 165u8, 70u8, 171u8, 41u8, 94u8, 66u8, 55u8,
							197u8, 153u8, 72u8, 218u8, 175u8, 220u8, 20u8, 52u8, 215u8, 192u8,
							136u8, 244u8, 70u8, 188u8, 111u8, 72u8, 38u8, 140u8, 189u8, 40u8, 36u8,
						],
					)
				}
				#[doc = " The factor to multiply the base delivery fee by for UMP."]
				pub fn upward_delivery_fee_factor(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"UpwardDeliveryFeeFactor",
						vec![],
						[
							40u8, 217u8, 164u8, 111u8, 151u8, 132u8, 69u8, 226u8, 163u8, 175u8,
							43u8, 239u8, 179u8, 217u8, 136u8, 161u8, 13u8, 251u8, 163u8, 102u8,
							24u8, 27u8, 168u8, 89u8, 221u8, 83u8, 93u8, 64u8, 96u8, 117u8, 146u8,
							71u8,
						],
					)
				}
				#[doc = " The number of HRMP messages we observed in `on_initialize` and thus used that number for"]
				#[doc = " announcing the weight of `on_initialize` and `on_finalize`."]
				pub fn announced_hrmp_messages_per_candidate(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"AnnouncedHrmpMessagesPerCandidate",
						vec![],
						[
							93u8, 11u8, 229u8, 172u8, 73u8, 87u8, 13u8, 149u8, 15u8, 94u8, 163u8,
							107u8, 156u8, 22u8, 131u8, 177u8, 96u8, 247u8, 213u8, 224u8, 41u8,
							126u8, 157u8, 33u8, 154u8, 194u8, 95u8, 234u8, 65u8, 19u8, 58u8, 161u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing XCMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_xcmp_weight_override(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_weights::weight_v2::Weight,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ReservedXcmpWeightOverride",
						vec![],
						[
							176u8, 93u8, 203u8, 74u8, 18u8, 170u8, 246u8, 203u8, 109u8, 89u8, 86u8,
							77u8, 96u8, 66u8, 189u8, 79u8, 184u8, 253u8, 11u8, 230u8, 87u8, 120u8,
							1u8, 254u8, 215u8, 41u8, 210u8, 86u8, 239u8, 206u8, 60u8, 2u8,
						],
					)
				}
				#[doc = " The weight we reserve at the beginning of the block for processing DMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub fn reserved_dmp_weight_override(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_weights::weight_v2::Weight,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"ReservedDmpWeightOverride",
						vec![],
						[
							205u8, 124u8, 9u8, 156u8, 255u8, 207u8, 208u8, 23u8, 179u8, 132u8,
							254u8, 157u8, 237u8, 240u8, 167u8, 203u8, 253u8, 111u8, 136u8, 32u8,
							100u8, 152u8, 16u8, 19u8, 175u8, 14u8, 108u8, 61u8, 59u8, 231u8, 70u8,
							112u8,
						],
					)
				}
				#[doc = " A custom head data that should be returned as result of `validate_block`."]
				#[doc = ""]
				#[doc = " See `Pallet::set_custom_validation_head_data` for more information."]
				pub fn custom_validation_head_data(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"CustomValidationHeadData",
						vec![],
						[
							52u8, 186u8, 187u8, 57u8, 245u8, 171u8, 202u8, 23u8, 92u8, 80u8, 118u8,
							66u8, 251u8, 156u8, 175u8, 254u8, 141u8, 185u8, 115u8, 209u8, 170u8,
							165u8, 1u8, 242u8, 120u8, 234u8, 162u8, 24u8, 135u8, 105u8, 8u8, 177u8,
						],
					)
				}
				#[doc = " Tracks cumulative `UMP` and `HRMP` messages sent across blocks in the current `PoV`."]
				#[doc = ""]
				#[doc = " Across different candidates/PoVs the budgets are tracked by [`AggregatedUnincludedSegment`]."]
				pub fn po_v_messages_tracker(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_parachain_system::PoVMessages,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainSystem",
						"PoVMessagesTracker",
						vec![],
						[
							191u8, 231u8, 162u8, 83u8, 144u8, 109u8, 1u8, 31u8, 194u8, 150u8,
							110u8, 208u8, 92u8, 6u8, 142u8, 25u8, 220u8, 118u8, 19u8, 165u8, 77u8,
							243u8, 122u8, 12u8, 43u8, 157u8, 176u8, 238u8, 177u8, 196u8, 42u8,
							140u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Returns the parachain ID we are running with."]
				pub fn self_para_id(
					&self,
				) -> ::subxt::constants::Address<
					runtime_types::polkadot_parachain_primitives::primitives::Id,
				> {
					::subxt::constants::Address::new_static(
						"ParachainSystem",
						"SelfParaId",
						[
							65u8, 93u8, 120u8, 165u8, 204u8, 81u8, 159u8, 163u8, 93u8, 135u8,
							114u8, 121u8, 147u8, 35u8, 215u8, 213u8, 4u8, 223u8, 83u8, 37u8, 225u8,
							200u8, 189u8, 156u8, 140u8, 36u8, 58u8, 46u8, 42u8, 232u8, 155u8, 0u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_timestamp::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Set {
					#[codec(compact)]
					pub now: ::core::primitive::u64,
				}
				impl ::subxt::blocks::StaticExtrinsic for Set {
					const PALLET: &'static str = "Timestamp";
					const CALL: &'static str = "set";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "[`Config::MinimumPeriod`]."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _None_."]
				#[doc = ""]
				#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
				#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
				#[doc = "block to execute any other calls."]
				#[doc = ""]
				#[doc = "## Complexity"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
					::subxt::tx::Payload::new_static(
						"Timestamp",
						"set",
						types::Set { now },
						[
							37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
							199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
							200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"Now",
						vec![],
						[
							44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
							92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
							141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
						],
					)
				}
				#[doc = " Whether the timestamp has been updated in this block."]
				#[doc = ""]
				#[doc = " This value is updated to `true` upon successful submission of a timestamp by a node."]
				#[doc = " It is then checked at the end of each block execution in the `on_finalize` hook."]
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Timestamp",
						"DidUpdate",
						vec![],
						[
							229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
							205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
							248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
							214u8, 140u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum period between blocks."]
				#[doc = ""]
				#[doc = " Be aware that this is different to the *expected* period that the block production"]
				#[doc = " apparatus provides. Your chosen consensus system will generally work with this to"]
				#[doc = " determine a sensible block time. For example, in the Aura pallet it will be double this"]
				#[doc = " period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod parachain_info {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::staging_parachain_info::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn parachain_id(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::polkadot_parachain_primitives::primitives::Id,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"ParachainInfo",
						"ParachainId",
						vec![],
						[
							160u8, 130u8, 74u8, 181u8, 231u8, 180u8, 246u8, 152u8, 204u8, 44u8,
							245u8, 91u8, 113u8, 246u8, 218u8, 50u8, 254u8, 248u8, 35u8, 219u8,
							83u8, 144u8, 228u8, 245u8, 122u8, 53u8, 194u8, 172u8, 222u8, 118u8,
							202u8, 91u8,
						],
					)
				}
			}
		}
	}
	pub mod balances {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_balances::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_balances::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAllowDeath {
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_allow_death";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceTransfer {
					pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_transfer";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferKeepAlive {
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub value: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_keep_alive";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAll {
					pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAll {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "transfer_all";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceUnreserve {
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub amount: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_unreserve";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpgradeAccounts {
					pub who: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "upgrade_accounts";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSetBalance {
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					#[codec(compact)]
					pub new_free: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_set_balance";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceAdjustTotalIssuance {
					pub direction: runtime_types::pallet_balances::types::AdjustmentDirection,
					#[codec(compact)]
					pub delta: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "force_adjust_total_issuance";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Burn {
					#[codec(compact)]
					pub value: ::core::primitive::u128,
					pub keep_alive: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for Burn {
					const PALLET: &'static str = "Balances";
					const CALL: &'static str = "burn";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				pub fn transfer_allow_death(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_allow_death",
						types::TransferAllowDeath { dest, value },
						[
							51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
							140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
							219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
							130u8,
						],
					)
				}
				#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
				#[doc = "may be specified."]
				pub fn force_transfer(
					&self,
					source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceTransfer> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_transfer",
						types::ForceTransfer { source, dest, value },
						[
							154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
							153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
							180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
						],
					)
				}
				#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
				#[doc = "kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
				#[doc = ""]
				#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_keep_alive",
						types::TransferKeepAlive { dest, value },
						[
							245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
							55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
							208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
						],
					)
				}
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
				#[doc = "  keep the sender account alive (true)."]
				pub fn transfer_all(
					&self,
					dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::TransferAll> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"transfer_all",
						types::TransferAll { dest, keep_alive },
						[
							105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
							112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
							9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
						],
					)
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceUnreserve> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_unreserve",
						types::ForceUnreserve { who, amount },
						[
							142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
							140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
							199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
							171u8,
						],
					)
				}
				#[doc = "Upgrade a specified account."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `who`: The account to be upgraded."]
				#[doc = ""]
				#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
				#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
				#[doc = "possibility of churn)."]
				pub fn upgrade_accounts(
					&self,
					who: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"upgrade_accounts",
						types::UpgradeAccounts { who },
						[
							66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
							233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
							214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
						],
					)
				}
				#[doc = "Set the regular balance of a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn force_set_balance(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					new_free: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceSetBalance> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_set_balance",
						types::ForceSetBalance { who, new_free },
						[
							114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
							39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
							116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
						],
					)
				}
				#[doc = "Adjust the total issuance in a saturating way."]
				#[doc = ""]
				#[doc = "Can only be called by root and always needs a positive `delta`."]
				#[doc = ""]
				#[doc = "# Example"]
				pub fn force_adjust_total_issuance(
					&self,
					direction: runtime_types::pallet_balances::types::AdjustmentDirection,
					delta: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::ForceAdjustTotalIssuance> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"force_adjust_total_issuance",
						types::ForceAdjustTotalIssuance { direction, delta },
						[
							208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
							190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8,
							176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8, 150u8,
							202u8,
						],
					)
				}
				#[doc = "Burn the specified liquid free balance from the origin account."]
				#[doc = ""]
				#[doc = "If the origin's account ends up below the existential deposit as a result"]
				#[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
				#[doc = ""]
				#[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
				#[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
				pub fn burn(
					&self,
					value: ::core::primitive::u128,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::Burn> {
					::subxt::tx::Payload::new_static(
						"Balances",
						"burn",
						types::Burn { value, keep_alive },
						[
							176u8, 64u8, 7u8, 109u8, 16u8, 44u8, 145u8, 125u8, 147u8, 152u8, 130u8,
							114u8, 221u8, 201u8, 150u8, 162u8, 118u8, 71u8, 52u8, 92u8, 240u8,
							116u8, 203u8, 98u8, 5u8, 22u8, 43u8, 102u8, 94u8, 208u8, 101u8, 57u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: ::subxt::utils::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
				PartialEq,
				Eq,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: ::subxt::utils::AccountId32,
				pub free: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: ::subxt::utils::AccountId32,
				pub to: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was minted into an account."]
			pub struct Minted {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Minted {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Minted";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some credit was balanced and added to the TotalIssuance."]
			pub struct MintedCredit {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for MintedCredit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "MintedCredit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was burned from an account."]
			pub struct Burned {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Burned {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Burned";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some debt has been dropped from the Total Issuance."]
			pub struct BurnedDebt {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BurnedDebt {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BurnedDebt";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was suspended from an account (it can be restored later)."]
			pub struct Suspended {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Suspended {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Suspended";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some amount was restored into an account."]
			pub struct Restored {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Restored {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Restored";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was upgraded."]
			pub struct Upgraded {
				pub who: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Upgraded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Upgraded";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
			pub struct Issued {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Issued {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Issued";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
			pub struct Rescinded {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Rescinded {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Rescinded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was locked."]
			pub struct Locked {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Locked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Locked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was unlocked."]
			pub struct Unlocked {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unlocked {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unlocked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was frozen."]
			pub struct Frozen {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Frozen {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Frozen";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was thawed."]
			pub struct Thawed {
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Thawed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Thawed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The `TotalIssuance` was forcefully changed."]
			pub struct TotalIssuanceForced {
				pub old: ::core::primitive::u128,
				pub new: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TotalIssuanceForced {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "TotalIssuanceForced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was placed on hold."]
			pub struct Held {
				pub reason: runtime_types::parachain_runtime::RuntimeHoldReason,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Held {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Held";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Held balance was burned from an account."]
			pub struct BurnedHeld {
				pub reason: runtime_types::parachain_runtime::RuntimeHoldReason,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BurnedHeld {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BurnedHeld";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A transfer of `amount` on hold from `source` to `dest` was initiated."]
			pub struct TransferOnHold {
				pub reason: runtime_types::parachain_runtime::RuntimeHoldReason,
				pub source: ::subxt::utils::AccountId32,
				pub dest: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransferOnHold {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "TransferOnHold";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The `transferred` balance is placed on hold at the `dest` account."]
			pub struct TransferAndHold {
				pub reason: runtime_types::parachain_runtime::RuntimeHoldReason,
				pub source: ::subxt::utils::AccountId32,
				pub dest: ::subxt::utils::AccountId32,
				pub transferred: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransferAndHold {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "TransferAndHold";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some balance was released from hold."]
			pub struct Released {
				pub reason: runtime_types::parachain_runtime::RuntimeHoldReason,
				pub who: ::subxt::utils::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Released {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Released";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An unexpected/defensive event was triggered."]
			pub struct Unexpected(pub runtime_types::pallet_balances::pallet::UnexpectedKind);
			impl ::subxt::events::StaticEvent for Unexpected {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unexpected";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
							171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
							255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
							185u8,
						],
					)
				}
				#[doc = " The total units of outstanding deactivated balance in the system."]
				pub fn inactive_issuance(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"InactiveIssuance",
						vec![],
						[
							212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
							249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
							30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
						],
					)
				}
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
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
							90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
							18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
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
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Account",
						Vec::new(),
						[
							213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
							90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
							18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Locks",
						Vec::new(),
						[
							10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
							167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
							13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::types::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
							140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
							106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::frame_support::traits::tokens::misc::IdAmount<
							runtime_types::parachain_runtime::RuntimeHoldReason,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Holds",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							251u8, 10u8, 70u8, 247u8, 141u8, 217u8, 91u8, 242u8, 77u8, 92u8, 73u8,
							136u8, 112u8, 97u8, 46u8, 5u8, 253u8, 202u8, 100u8, 165u8, 224u8, 82u8,
							177u8, 126u8, 36u8, 82u8, 186u8, 188u8, 80u8, 15u8, 157u8, 207u8,
						],
					)
				}
				#[doc = " Holds on account balances."]
				pub fn holds_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::frame_support::traits::tokens::misc::IdAmount<
							runtime_types::parachain_runtime::RuntimeHoldReason,
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Holds",
						Vec::new(),
						[
							251u8, 10u8, 70u8, 247u8, 141u8, 217u8, 91u8, 242u8, 77u8, 92u8, 73u8,
							136u8, 112u8, 97u8, 46u8, 5u8, 253u8, 202u8, 100u8, 165u8, 224u8, 82u8,
							177u8, 126u8, 36u8, 82u8, 186u8, 188u8, 80u8, 15u8, 157u8, 207u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::frame_support::traits::tokens::misc::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Freezes",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
							112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
							163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
						],
					)
				}
				#[doc = " Freeze locks on account balances."]
				pub fn freezes_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::frame_support::traits::tokens::misc::IdAmount<
							(),
							::core::primitive::u128,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Balances",
						"Freezes",
						Vec::new(),
						[
							69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
							112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
							163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
				#[doc = ""]
				#[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
				#[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
				#[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
				#[doc = " behaviour if you set this to zero."]
				#[doc = ""]
				#[doc = " Bottom line: Do yourself a favour and make it at least one!"]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				#[doc = ""]
				#[doc = " Use of locks is deprecated in favour of freezes. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				#[doc = ""]
				#[doc = " Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`"]
				pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
				pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"Balances",
						"MaxFreezes",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: ::subxt::utils::AccountId32,
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
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
							147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
							159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
							197u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_transaction_payment::Releases,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
							178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
							251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
							144u8,
						],
					)
				}
				#[doc = " The `OnChargeTransaction` stores the withdrawn tx fee here."]
				#[doc = ""]
				#[doc = " Use `withdraw_txfee` and `remaining_txfee` to access from outside the crate."]				pub fn tx_payment_credit (& self ,) -> :: subxt :: storage :: address :: Address :: < :: subxt :: storage :: address :: StaticStorageMapKey , runtime_types :: frame_support :: traits :: storage :: NoDrop < runtime_types :: frame_support :: traits :: tokens :: fungible :: imbalance :: Imbalance < :: core :: primitive :: u128 > > , :: subxt :: storage :: address :: Yes , () , () >{
					::subxt::storage::address::Address::new_static(
						"TransactionPayment",
						"TxPaymentCredit",
						vec![],
						[
							39u8, 127u8, 132u8, 77u8, 25u8, 10u8, 195u8, 64u8, 255u8, 212u8, 183u8,
							177u8, 238u8, 24u8, 81u8, 65u8, 93u8, 177u8, 209u8, 134u8, 245u8,
							241u8, 252u8, 87u8, 179u8, 61u8, 168u8, 77u8, 65u8, 13u8, 72u8, 205u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A fee multiplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multiplied by the `final_fee` to obtain a \"virtual tip\" that is later"]
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
				pub fn operational_fee_multiplier(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u8> {
					::subxt::constants::Address::new_static(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the Sudo pallet."]
		pub type Error = runtime_types::pallet_sudo::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_sudo::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Sudo {
					pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Sudo {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoUncheckedWeight {
					pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
					pub weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_unchecked_weight";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetKey {
					pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "set_key";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SudoAs {
					pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					pub call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SudoAs {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "sudo_as";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveKey;
				impl ::subxt::blocks::StaticExtrinsic for RemoveKey {
					const PALLET: &'static str = "Sudo";
					const CALL: &'static str = "remove_key";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				pub fn sudo(
					&self,
					call: runtime_types::parachain_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::Sudo> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo",
						types::Sudo { call: ::std::boxed::Box::new(call) },
						[
							188u8, 28u8, 178u8, 68u8, 25u8, 170u8, 209u8, 238u8, 49u8, 167u8,
							224u8, 13u8, 177u8, 71u8, 69u8, 220u8, 241u8, 113u8, 0u8, 164u8, 170u8,
							88u8, 209u8, 212u8, 127u8, 13u8, 229u8, 144u8, 175u8, 33u8, 49u8, 61u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::parachain_runtime::RuntimeCall,
					weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_unchecked_weight",
						types::SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							132u8, 162u8, 104u8, 51u8, 121u8, 80u8, 235u8, 236u8, 189u8, 120u8,
							180u8, 69u8, 140u8, 223u8, 73u8, 73u8, 141u8, 187u8, 247u8, 232u8,
							24u8, 232u8, 132u8, 224u8, 31u8, 180u8, 227u8, 145u8, 178u8, 35u8,
							110u8, 150u8,
						],
					)
				}
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				pub fn set_key(
					&self,
					new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
				) -> ::subxt::tx::Payload<types::SetKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"set_key",
						types::SetKey { new },
						[
							9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
							227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
							158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
						],
					)
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn sudo_as(
					&self,
					who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
					call: runtime_types::parachain_runtime::RuntimeCall,
				) -> ::subxt::tx::Payload<types::SudoAs> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"sudo_as",
						types::SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							34u8, 128u8, 0u8, 185u8, 4u8, 37u8, 223u8, 245u8, 109u8, 13u8, 199u8,
							6u8, 32u8, 70u8, 13u8, 110u8, 43u8, 226u8, 148u8, 232u8, 113u8, 69u8,
							137u8, 62u8, 25u8, 40u8, 88u8, 140u8, 45u8, 6u8, 240u8, 23u8,
						],
					)
				}
				#[doc = "Permanently removes the sudo key."]
				#[doc = ""]
				#[doc = "**This cannot be un-done.**"]
				pub fn remove_key(&self) -> ::subxt::tx::Payload<types::RemoveKey> {
					::subxt::tx::Payload::new_static(
						"Sudo",
						"remove_key",
						types::RemoveKey {},
						[
							133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
							25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8,
							191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8, 211u8,
							79u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A sudo call just took place."]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The sudo key has been updated."]
			pub struct KeyChanged {
				pub old: ::core::option::Option<::subxt::utils::AccountId32>,
				pub new: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The key was permanently removed."]
			pub struct KeyRemoved;
			impl ::subxt::events::StaticEvent for KeyRemoved {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
			pub struct SudoAsDone {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
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
				pub fn key(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Sudo",
						"Key",
						vec![],
						[
							72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
							31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
							36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Authorship",
						"Author",
						vec![],
						[
							247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
							220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
							61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
							18u8,
						],
					)
				}
			}
		}
	}
	pub mod collator_selection {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_collator_selection::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_collator_selection::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetInvulnerables {
					pub new: ::std::vec::Vec<::subxt::utils::AccountId32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetInvulnerables {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_invulnerables";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetDesiredCandidates {
					pub max: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetDesiredCandidates {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_desired_candidates";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetCandidacyBond {
					pub bond: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetCandidacyBond {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "set_candidacy_bond";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RegisterAsCandidate;
				impl ::subxt::blocks::StaticExtrinsic for RegisterAsCandidate {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "register_as_candidate";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LeaveIntent;
				impl ::subxt::blocks::StaticExtrinsic for LeaveIntent {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "leave_intent";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AddInvulnerable {
					pub who: ::subxt::utils::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddInvulnerable {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "add_invulnerable";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveInvulnerable {
					pub who: ::subxt::utils::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveInvulnerable {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "remove_invulnerable";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateBond {
					pub new_deposit: ::core::primitive::u128,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateBond {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "update_bond";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TakeCandidateSlot {
					pub deposit: ::core::primitive::u128,
					pub target: ::subxt::utils::AccountId32,
				}
				impl ::subxt::blocks::StaticExtrinsic for TakeCandidateSlot {
					const PALLET: &'static str = "CollatorSelection";
					const CALL: &'static str = "take_candidate_slot";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the list of invulnerable (fixed) collators. These collators must do some"]
				#[doc = "preparation, namely to have registered session keys."]
				#[doc = ""]
				#[doc = "The call will remove any accounts that have not registered keys from the set. That is,"]
				#[doc = "it is non-atomic; the caller accepts all `AccountId`s passed in `new` _individually_ as"]
				#[doc = "acceptable Invulnerables, and is not proposing a _set_ of new Invulnerables."]
				#[doc = ""]
				#[doc = "This call does not maintain mutual exclusivity of `Invulnerables` and `Candidates`. It"]
				#[doc = "is recommended to use a batch of `add_invulnerable` and `remove_invulnerable` instead. A"]
				#[doc = "`batch_all` can also be used to enforce atomicity. If any candidates are included in"]
				#[doc = "`new`, they should be removed with `remove_invulnerable_candidate` after execution."]
				#[doc = ""]
				#[doc = "Must be called by the `UpdateOrigin`."]
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<::subxt::utils::AccountId32>,
				) -> ::subxt::tx::Payload<types::SetInvulnerables> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_invulnerables",
						types::SetInvulnerables { new },
						[
							113u8, 217u8, 14u8, 48u8, 6u8, 198u8, 8u8, 170u8, 8u8, 237u8, 230u8,
							184u8, 17u8, 181u8, 15u8, 126u8, 117u8, 3u8, 208u8, 215u8, 40u8, 16u8,
							150u8, 162u8, 37u8, 196u8, 235u8, 36u8, 247u8, 24u8, 187u8, 17u8,
						],
					)
				}
				#[doc = "Set the ideal number of non-invulnerable collators. If lowering this number, then the"]
				#[doc = "number of running collators could be higher than this figure. Aside from that edge case,"]
				#[doc = "there should be no other way to have more candidates than the desired number."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::SetDesiredCandidates> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_desired_candidates",
						types::SetDesiredCandidates { max },
						[
							174u8, 44u8, 232u8, 155u8, 228u8, 219u8, 239u8, 75u8, 86u8, 150u8,
							135u8, 214u8, 58u8, 9u8, 25u8, 133u8, 245u8, 101u8, 85u8, 246u8, 15u8,
							248u8, 165u8, 87u8, 88u8, 28u8, 10u8, 196u8, 86u8, 89u8, 28u8, 165u8,
						],
					)
				}
				#[doc = "Set the candidacy bond amount."]
				#[doc = ""]
				#[doc = "If the candidacy bond is increased by this call, all current candidates which have a"]
				#[doc = "deposit lower than the new bond will be kicked from the list and get their deposits"]
				#[doc = "back."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::SetCandidacyBond> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"set_candidacy_bond",
						types::SetCandidacyBond { bond },
						[
							250u8, 4u8, 185u8, 228u8, 101u8, 223u8, 49u8, 44u8, 172u8, 148u8,
							216u8, 242u8, 192u8, 88u8, 228u8, 59u8, 225u8, 222u8, 171u8, 40u8,
							23u8, 1u8, 46u8, 183u8, 189u8, 191u8, 156u8, 12u8, 218u8, 116u8, 76u8,
							59u8,
						],
					)
				}
				#[doc = "Register this account as a collator candidate. The account must (a) already have"]
				#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
				#[doc = ""]
				#[doc = "This call is not available to `Invulnerable` collators."]
				pub fn register_as_candidate(
					&self,
				) -> ::subxt::tx::Payload<types::RegisterAsCandidate> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"register_as_candidate",
						types::RegisterAsCandidate {},
						[
							69u8, 222u8, 214u8, 106u8, 105u8, 168u8, 82u8, 239u8, 158u8, 117u8,
							224u8, 89u8, 228u8, 51u8, 221u8, 244u8, 88u8, 63u8, 72u8, 119u8, 224u8,
							111u8, 93u8, 39u8, 18u8, 66u8, 72u8, 105u8, 70u8, 66u8, 178u8, 173u8,
						],
					)
				}
				#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
				#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
				#[doc = ""]
				#[doc = "This call will fail if the total number of candidates would drop below"]
				#[doc = "`MinEligibleCollators`."]
				pub fn leave_intent(&self) -> ::subxt::tx::Payload<types::LeaveIntent> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"leave_intent",
						types::LeaveIntent {},
						[
							126u8, 57u8, 10u8, 67u8, 120u8, 229u8, 70u8, 23u8, 154u8, 215u8, 226u8,
							178u8, 203u8, 152u8, 195u8, 177u8, 157u8, 158u8, 40u8, 17u8, 93u8,
							225u8, 253u8, 217u8, 48u8, 165u8, 55u8, 79u8, 43u8, 123u8, 193u8,
							147u8,
						],
					)
				}
				#[doc = "Add a new account `who` to the list of `Invulnerables` collators. `who` must have"]
				#[doc = "registered session keys. If `who` is a candidate, they will be removed."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn add_invulnerable(
					&self,
					who: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::Payload<types::AddInvulnerable> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"add_invulnerable",
						types::AddInvulnerable { who },
						[
							115u8, 109u8, 38u8, 19u8, 81u8, 194u8, 124u8, 140u8, 239u8, 23u8, 85u8,
							62u8, 241u8, 83u8, 11u8, 241u8, 14u8, 34u8, 206u8, 63u8, 104u8, 78u8,
							96u8, 182u8, 173u8, 198u8, 230u8, 107u8, 102u8, 6u8, 164u8, 75u8,
						],
					)
				}
				#[doc = "Remove an account `who` from the list of `Invulnerables` collators. `Invulnerables` must"]
				#[doc = "be sorted."]
				#[doc = ""]
				#[doc = "The origin for this call must be the `UpdateOrigin`."]
				pub fn remove_invulnerable(
					&self,
					who: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::Payload<types::RemoveInvulnerable> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"remove_invulnerable",
						types::RemoveInvulnerable { who },
						[
							103u8, 146u8, 23u8, 136u8, 61u8, 65u8, 172u8, 157u8, 216u8, 200u8,
							119u8, 28u8, 189u8, 215u8, 13u8, 100u8, 102u8, 13u8, 94u8, 12u8, 78u8,
							156u8, 149u8, 74u8, 126u8, 118u8, 127u8, 49u8, 129u8, 2u8, 12u8, 118u8,
						],
					)
				}
				#[doc = "Update the candidacy bond of collator candidate `origin` to a new amount `new_deposit`."]
				#[doc = ""]
				#[doc = "Setting a `new_deposit` that is lower than the current deposit while `origin` is"]
				#[doc = "occupying a top-`DesiredCandidates` slot is not allowed."]
				#[doc = ""]
				#[doc = "This call will fail if `origin` is not a collator candidate, the updated bond is lower"]
				#[doc = "than the minimum candidacy bond, and/or the amount cannot be reserved."]
				pub fn update_bond(
					&self,
					new_deposit: ::core::primitive::u128,
				) -> ::subxt::tx::Payload<types::UpdateBond> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"update_bond",
						types::UpdateBond { new_deposit },
						[
							47u8, 184u8, 193u8, 220u8, 160u8, 1u8, 253u8, 203u8, 8u8, 142u8, 43u8,
							151u8, 190u8, 138u8, 201u8, 174u8, 233u8, 112u8, 200u8, 247u8, 251u8,
							94u8, 23u8, 224u8, 150u8, 179u8, 190u8, 140u8, 199u8, 50u8, 2u8, 249u8,
						],
					)
				}
				#[doc = "The caller `origin` replaces a candidate `target` in the collator candidate list by"]
				#[doc = "reserving `deposit`. The amount `deposit` reserved by the caller must be greater than"]
				#[doc = "the existing bond of the target it is trying to replace."]
				#[doc = ""]
				#[doc = "This call will fail if the caller is already a collator candidate or invulnerable, the"]
				#[doc = "caller does not have registered session keys, the target is not a collator candidate,"]
				#[doc = "and/or the `deposit` amount cannot be reserved."]
				pub fn take_candidate_slot(
					&self,
					deposit: ::core::primitive::u128,
					target: ::subxt::utils::AccountId32,
				) -> ::subxt::tx::Payload<types::TakeCandidateSlot> {
					::subxt::tx::Payload::new_static(
						"CollatorSelection",
						"take_candidate_slot",
						types::TakeCandidateSlot { deposit, target },
						[
							48u8, 150u8, 189u8, 206u8, 199u8, 196u8, 173u8, 3u8, 206u8, 10u8, 50u8,
							160u8, 15u8, 53u8, 189u8, 126u8, 154u8, 36u8, 90u8, 66u8, 235u8, 12u8,
							107u8, 44u8, 117u8, 33u8, 207u8, 194u8, 251u8, 194u8, 224u8, 80u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New Invulnerables were set."]
			pub struct NewInvulnerables {
				pub invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new Invulnerable was added."]
			pub struct InvulnerableAdded {
				pub account_id: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for InvulnerableAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "InvulnerableAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An Invulnerable was removed."]
			pub struct InvulnerableRemoved {
				pub account_id: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for InvulnerableRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "InvulnerableRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The number of desired candidates was set."]
			pub struct NewDesiredCandidates {
				pub desired_candidates: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The candidacy bond was set."]
			pub struct NewCandidacyBond {
				pub bond_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A new candidate joined."]
			pub struct CandidateAdded {
				pub account_id: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Bond of a candidate updated."]
			pub struct CandidateBondUpdated {
				pub account_id: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateBondUpdated {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateBondUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A candidate was removed."]
			pub struct CandidateRemoved {
				pub account_id: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for CandidateRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was replaced in the candidate list by another one."]
			pub struct CandidateReplaced {
				pub old: ::subxt::utils::AccountId32,
				pub new: ::subxt::utils::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for CandidateReplaced {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateReplaced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An account was unable to be added to the Invulnerables because they did not have keys"]
			#[doc = "registered. Other Invulnerables may have been set."]
			pub struct InvalidInvulnerableSkipped {
				pub account_id: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for InvalidInvulnerableSkipped {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "InvalidInvulnerableSkipped";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The invulnerable, permissioned collators. This list must be sorted."]
				pub fn invulnerables(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::subxt::utils::AccountId32,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"Invulnerables",
						vec![],
						[
							109u8, 180u8, 25u8, 41u8, 152u8, 158u8, 186u8, 214u8, 89u8, 222u8,
							103u8, 14u8, 91u8, 3u8, 65u8, 6u8, 255u8, 62u8, 47u8, 255u8, 132u8,
							164u8, 217u8, 200u8, 130u8, 29u8, 168u8, 23u8, 81u8, 217u8, 35u8,
							123u8,
						],
					)
				}
				#[doc = " The (community, limited) collation candidates. `Candidates` and `Invulnerables` should be"]
				#[doc = " mutually exclusive."]
				#[doc = ""]
				#[doc = " This list is sorted in ascending order by deposit and when the deposits are equal, the least"]
				#[doc = " recently updated is considered greater."]
				pub fn candidate_list(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::pallet_collator_selection::pallet::CandidateInfo<
							::subxt::utils::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"CandidateList",
						vec![],
						[
							77u8, 195u8, 89u8, 139u8, 79u8, 111u8, 151u8, 215u8, 19u8, 152u8, 67u8,
							49u8, 74u8, 76u8, 3u8, 60u8, 51u8, 140u8, 6u8, 134u8, 159u8, 55u8,
							196u8, 57u8, 189u8, 31u8, 219u8, 218u8, 164u8, 189u8, 196u8, 60u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
							72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
							126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				#[doc = " Last block authored by collator."]
				pub fn last_authored_block_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"LastAuthoredBlock",
						Vec::new(),
						[
							176u8, 170u8, 165u8, 244u8, 101u8, 126u8, 24u8, 132u8, 228u8, 138u8,
							72u8, 241u8, 144u8, 100u8, 79u8, 112u8, 9u8, 46u8, 210u8, 80u8, 12u8,
							126u8, 32u8, 214u8, 26u8, 171u8, 155u8, 3u8, 233u8, 22u8, 164u8, 25u8,
						],
					)
				}
				#[doc = " Desired number of candidates."]
				#[doc = ""]
				#[doc = " This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct."]
				pub fn desired_candidates(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"DesiredCandidates",
						vec![],
						[
							69u8, 199u8, 130u8, 132u8, 10u8, 127u8, 204u8, 220u8, 59u8, 107u8,
							96u8, 180u8, 42u8, 235u8, 14u8, 126u8, 231u8, 242u8, 162u8, 126u8,
							63u8, 223u8, 15u8, 250u8, 22u8, 210u8, 54u8, 34u8, 235u8, 191u8, 250u8,
							21u8,
						],
					)
				}
				#[doc = " Fixed amount to deposit to become a collator."]
				#[doc = ""]
				#[doc = " When a collator calls `leave_intent` they immediately receive the deposit back."]
				pub fn candidacy_bond(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"CollatorSelection",
						"CandidacyBond",
						vec![],
						[
							71u8, 134u8, 156u8, 102u8, 201u8, 83u8, 240u8, 251u8, 189u8, 213u8,
							211u8, 182u8, 126u8, 122u8, 41u8, 174u8, 105u8, 29u8, 216u8, 23u8,
							255u8, 55u8, 245u8, 187u8, 234u8, 234u8, 178u8, 155u8, 145u8, 49u8,
							196u8, 214u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Account Identifier from which the internal Pot is generated."]
				pub fn pot_id(
					&self,
				) -> ::subxt::constants::Address<runtime_types::frame_support::PalletId> {
					::subxt::constants::Address::new_static(
						"CollatorSelection",
						"PotId",
						[
							56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
							161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
							129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
						],
					)
				}
				#[doc = " Maximum number of candidates that we should have."]
				#[doc = ""]
				#[doc = " This does not take into account the invulnerables."]
				pub fn max_candidates(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"CollatorSelection",
						"MaxCandidates",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Minimum number eligible collators. Should always be greater than zero. This includes"]
				#[doc = " Invulnerable collators. This ensures that there will always be one collator who can"]
				#[doc = " produce a block."]
				pub fn min_eligible_collators(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"CollatorSelection",
						"MinEligibleCollators",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Maximum number of invulnerables."]
				pub fn max_invulnerables(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"CollatorSelection",
						"MaxInvulnerables",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn kick_threshold(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"CollatorSelection",
						"KickThreshold",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Gets this pallet's derived pot account."]
				pub fn pot_account(
					&self,
				) -> ::subxt::constants::Address<::subxt::utils::AccountId32> {
					::subxt::constants::Address::new_static(
						"CollatorSelection",
						"pot_account",
						[
							115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8,
							155u8, 157u8, 224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8,
							204u8, 122u8, 166u8, 248u8, 23u8, 174u8, 225u8, 99u8, 108u8, 89u8,
							135u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Error for the session pallet."]
		pub type Error = runtime_types::pallet_session::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_session::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SetKeys {
					pub keys: runtime_types::parachain_runtime::SessionKeys,
					pub proof: ::std::vec::Vec<::core::primitive::u8>,
				}
				impl ::subxt::blocks::StaticExtrinsic for SetKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "set_keys";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PurgeKeys;
				impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
					const PALLET: &'static str = "Session";
					const CALL: &'static str = "purge_keys";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = ""]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "- `origin`: The dispatch origin of this function must be signed."]
				#[doc = "- `keys`: The new session keys to set. These are the public keys of all sessions keys"]
				#[doc = "  setup in the runtime."]
				#[doc = "- `proof`: The proof that `origin` has access to the private keys of `keys`. See"]
				#[doc = "  [`impl_opaque_keys`](sp_runtime::impl_opaque_keys) for more information about the"]
				#[doc = "  proof format."]
				pub fn set_keys(
					&self,
					keys: runtime_types::parachain_runtime::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::Payload<types::SetKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"set_keys",
						types::SetKeys { keys, proof },
						[
							219u8, 63u8, 235u8, 242u8, 176u8, 248u8, 204u8, 20u8, 121u8, 176u8,
							105u8, 242u8, 190u8, 124u8, 153u8, 219u8, 12u8, 224u8, 196u8, 18u8,
							183u8, 159u8, 33u8, 97u8, 44u8, 64u8, 0u8, 10u8, 52u8, 181u8, 70u8,
							206u8,
						],
					)
				}
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
					::subxt::tx::Payload::new_static(
						"Session",
						"purge_keys",
						types::PurgeKeys {},
						[
							215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
							151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
							67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
							209u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The `NewSession` event in the current block also implies a new validator set to be"]
			#[doc = "queued."]
			pub struct NewQueued;
			impl ::subxt::events::StaticEvent for NewQueued {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewQueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Validator has been disabled."]
			pub struct ValidatorDisabled {
				pub validator: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for ValidatorDisabled {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "ValidatorDisabled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Validator has been re-enabled."]
			pub struct ValidatorReenabled {
				pub validator: ::subxt::utils::AccountId32,
			}
			impl ::subxt::events::StaticEvent for ValidatorReenabled {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "ValidatorReenabled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<::subxt::utils::AccountId32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"Validators",
						vec![],
						[
							50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
							133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
							115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
							86u8,
						],
					)
				}
				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"CurrentIndex",
						vec![],
						[
							167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
							135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
							134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
							221u8, 230u8,
						],
					)
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedChanged",
						vec![],
						[
							184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
							198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
							36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
							153u8,
						],
					)
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::subxt::utils::AccountId32,
						runtime_types::parachain_runtime::SessionKeys,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"QueuedKeys",
						vec![],
						[
							205u8, 110u8, 116u8, 201u8, 29u8, 220u8, 3u8, 147u8, 3u8, 236u8, 73u8,
							108u8, 108u8, 173u8, 76u8, 44u8, 102u8, 69u8, 47u8, 90u8, 185u8, 162u8,
							57u8, 23u8, 210u8, 45u8, 18u8, 242u8, 10u8, 95u8, 67u8, 109u8,
						],
					)
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::std::vec::Vec<(
						::core::primitive::u32,
						runtime_types::sp_staking::offence::OffenceSeverity,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"DisabledValidators",
						vec![],
						[
							214u8, 48u8, 28u8, 150u8, 143u8, 29u8, 183u8, 40u8, 236u8, 227u8,
							195u8, 5u8, 202u8, 54u8, 184u8, 26u8, 239u8, 237u8, 113u8, 39u8, 200u8,
							111u8, 163u8, 3u8, 24u8, 101u8, 107u8, 91u8, 228u8, 135u8, 12u8, 86u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::parachain_runtime::SessionKeys,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							45u8, 92u8, 45u8, 21u8, 150u8, 181u8, 197u8, 56u8, 229u8, 146u8, 183u8,
							210u8, 56u8, 197u8, 9u8, 202u8, 226u8, 183u8, 110u8, 173u8, 100u8,
							75u8, 248u8, 207u8, 215u8, 163u8, 13u8, 113u8, 222u8, 128u8, 18u8,
							192u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::parachain_runtime::SessionKeys,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							45u8, 92u8, 45u8, 21u8, 150u8, 181u8, 197u8, 56u8, 229u8, 146u8, 183u8,
							210u8, 56u8, 197u8, 9u8, 202u8, 226u8, 183u8, 110u8, 173u8, 100u8,
							75u8, 248u8, 207u8, 215u8, 163u8, 13u8, 113u8, 222u8, 128u8, 18u8,
							192u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
							253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
							253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
							206u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::subxt::utils::AccountId32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
							253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
							253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
							206u8,
						],
					)
				}
				#[doc = " Accounts whose keys were set via `SessionInterface` (external path) without"]
				#[doc = " incrementing the consumer reference or placing a key deposit. `do_purge_keys`"]
				#[doc = " only decrements consumers for accounts that were registered through the local"]
				#[doc = " session pallet."]
				pub fn externally_set_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"ExternallySetKeys",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							103u8, 36u8, 104u8, 62u8, 115u8, 230u8, 203u8, 146u8, 219u8, 182u8,
							233u8, 0u8, 192u8, 190u8, 156u8, 191u8, 219u8, 33u8, 130u8, 215u8,
							198u8, 202u8, 146u8, 77u8, 184u8, 40u8, 152u8, 66u8, 192u8, 235u8,
							162u8, 109u8,
						],
					)
				}
				#[doc = " Accounts whose keys were set via `SessionInterface` (external path) without"]
				#[doc = " incrementing the consumer reference or placing a key deposit. `do_purge_keys`"]
				#[doc = " only decrements consumers for accounts that were registered through the local"]
				#[doc = " session pallet."]
				pub fn externally_set_keys_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"Session",
						"ExternallySetKeys",
						Vec::new(),
						[
							103u8, 36u8, 104u8, 62u8, 115u8, 230u8, 203u8, 146u8, 219u8, 182u8,
							233u8, 0u8, 192u8, 190u8, 156u8, 191u8, 219u8, 33u8, 130u8, 215u8,
							198u8, 202u8, 146u8, 77u8, 184u8, 40u8, 152u8, 66u8, 192u8, 235u8,
							162u8, 109u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount to be held when setting keys."]
				pub fn key_deposit(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
					::subxt::constants::Address::new_static(
						"Session",
						"KeyDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod aura {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current authority set."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Aura",
						"Authorities",
						vec![],
						[
							95u8, 52u8, 203u8, 53u8, 254u8, 107u8, 134u8, 122u8, 95u8, 253u8, 51u8,
							137u8, 142u8, 106u8, 237u8, 248u8, 159u8, 80u8, 41u8, 233u8, 137u8,
							133u8, 13u8, 217u8, 176u8, 88u8, 132u8, 199u8, 241u8, 47u8, 125u8,
							27u8,
						],
					)
				}
				#[doc = " The current slot of this block."]
				#[doc = ""]
				#[doc = " This will be set in `on_initialize`."]
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_consensus_slots::Slot,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"Aura",
						"CurrentSlot",
						vec![],
						[
							112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
							236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
							201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
							43u8, 57u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The slot duration Aura should run with, expressed in milliseconds."]
				#[doc = ""]
				#[doc = " The effective value of this type can be changed with a runtime upgrade."]
				#[doc = ""]
				#[doc = " For backwards compatibility either use [`MinimumPeriodTimesTwo`] or a const."]
				pub fn slot_duration(&self) -> ::subxt::constants::Address<::core::primitive::u64> {
					::subxt::constants::Address::new_static(
						"Aura",
						"SlotDuration",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod aura_ext {
		use super::root_mod;
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Serves as cache for the authorities."]
				#[doc = ""]
				#[doc = " The authorities in AuRa are overwritten in `on_initialize` when we switch to a new session,"]
				#[doc = " but we require the old authorities to verify the seal when validating a PoV. This will"]
				#[doc = " always be updated to the latest AuRa authorities in `on_finalize`."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"AuraExt",
						"Authorities",
						vec![],
						[
							95u8, 52u8, 203u8, 53u8, 254u8, 107u8, 134u8, 122u8, 95u8, 253u8, 51u8,
							137u8, 142u8, 106u8, 237u8, 248u8, 159u8, 80u8, 41u8, 233u8, 137u8,
							133u8, 13u8, 217u8, 176u8, 88u8, 132u8, 199u8, 241u8, 47u8, 125u8,
							27u8,
						],
					)
				}
				#[doc = " Current relay chain slot paired with a number of authored blocks."]
				#[doc = ""]
				#[doc = " This is updated in [`FixedVelocityConsensusHook::on_state_proof`] with the current relay"]
				#[doc = " chain slot as provided by the relay chain state proof."]
				pub fn relay_slot_info(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(runtime_types::sp_consensus_slots::Slot, ::core::primitive::u32),
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"AuraExt",
						"RelaySlotInfo",
						vec![],
						[
							11u8, 108u8, 55u8, 103u8, 229u8, 143u8, 64u8, 46u8, 237u8, 138u8,
							124u8, 27u8, 85u8, 52u8, 235u8, 93u8, 234u8, 78u8, 240u8, 22u8, 83u8,
							157u8, 169u8, 243u8, 220u8, 87u8, 174u8, 125u8, 63u8, 251u8, 83u8,
							228u8,
						],
					)
				}
			}
		}
	}
	pub mod xcmp_queue {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::cumulus_pallet_xcmp_queue::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::cumulus_pallet_xcmp_queue::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SuspendXcmExecution;
				impl ::subxt::blocks::StaticExtrinsic for SuspendXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "suspend_xcm_execution";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ResumeXcmExecution;
				impl ::subxt::blocks::StaticExtrinsic for ResumeXcmExecution {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "resume_xcm_execution";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateSuspendThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateSuspendThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_suspend_threshold";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateDropThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateDropThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_drop_threshold";
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UpdateResumeThreshold {
					pub new: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for UpdateResumeThreshold {
					const PALLET: &'static str = "XcmpQueue";
					const CALL: &'static str = "update_resume_threshold";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn suspend_xcm_execution(
					&self,
				) -> ::subxt::tx::Payload<types::SuspendXcmExecution> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"suspend_xcm_execution",
						types::SuspendXcmExecution {},
						[
							54u8, 120u8, 33u8, 251u8, 74u8, 56u8, 29u8, 76u8, 104u8, 218u8, 115u8,
							198u8, 148u8, 237u8, 9u8, 191u8, 241u8, 48u8, 33u8, 24u8, 60u8, 144u8,
							22u8, 78u8, 58u8, 50u8, 26u8, 188u8, 231u8, 42u8, 201u8, 76u8,
						],
					)
				}
				#[doc = "Resumes all XCM executions for the XCMP queue."]
				#[doc = ""]
				#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ControllerOrigin`."]
				pub fn resume_xcm_execution(
					&self,
				) -> ::subxt::tx::Payload<types::ResumeXcmExecution> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"resume_xcm_execution",
						types::ResumeXcmExecution {},
						[
							173u8, 231u8, 78u8, 253u8, 108u8, 234u8, 199u8, 124u8, 184u8, 154u8,
							95u8, 194u8, 13u8, 77u8, 175u8, 7u8, 7u8, 112u8, 161u8, 72u8, 133u8,
							71u8, 63u8, 218u8, 97u8, 226u8, 133u8, 6u8, 93u8, 177u8, 247u8, 109u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages which must be in the queue for the other side to be"]
				#[doc = "told to suspend their sending."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
				pub fn update_suspend_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateSuspendThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_suspend_threshold",
						types::UpdateSuspendThreshold { new },
						[
							64u8, 91u8, 172u8, 51u8, 220u8, 174u8, 54u8, 47u8, 57u8, 89u8, 75u8,
							39u8, 126u8, 198u8, 143u8, 35u8, 70u8, 125u8, 167u8, 14u8, 17u8, 18u8,
							146u8, 222u8, 100u8, 92u8, 81u8, 239u8, 173u8, 43u8, 42u8, 174u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages which must be in the queue after which we drop any"]
				#[doc = "further messages from the channel."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
				pub fn update_drop_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateDropThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_drop_threshold",
						types::UpdateDropThreshold { new },
						[
							123u8, 54u8, 12u8, 180u8, 165u8, 198u8, 141u8, 200u8, 149u8, 168u8,
							186u8, 237u8, 162u8, 91u8, 89u8, 242u8, 229u8, 16u8, 32u8, 254u8, 59u8,
							168u8, 31u8, 134u8, 217u8, 251u8, 0u8, 102u8, 113u8, 194u8, 175u8, 9u8,
						],
					)
				}
				#[doc = "Overwrites the number of pages which the queue must be reduced to before it signals"]
				#[doc = "that message sending may recommence after it has been suspended."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `Root`."]
				#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
				pub fn update_resume_threshold(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::UpdateResumeThreshold> {
					::subxt::tx::Payload::new_static(
						"XcmpQueue",
						"update_resume_threshold",
						types::UpdateResumeThreshold { new },
						[
							172u8, 136u8, 11u8, 106u8, 42u8, 157u8, 167u8, 183u8, 87u8, 62u8,
							182u8, 17u8, 184u8, 59u8, 215u8, 230u8, 18u8, 243u8, 212u8, 34u8, 54u8,
							188u8, 95u8, 119u8, 173u8, 20u8, 91u8, 206u8, 212u8, 57u8, 136u8, 77u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An HRMP message was sent to a sibling parachain."]
			pub struct XcmpMessageSent {
				pub message_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The suspended inbound XCMP channels. All others are not suspended."]
				#[doc = ""]
				#[doc = " This is a `StorageValue` instead of a `StorageMap` since we expect multiple reads per block"]
				#[doc = " to different keys with a one byte payload. The access to `BoundedBTreeSet` will be cached"]
				#[doc = " within the block and therefore only included once in the proof size."]
				#[doc = ""]
				#[doc = " NOTE: The PoV benchmarking cannot know this and will over-estimate, but the actual proof"]
				#[doc = " will be smaller."]
				pub fn inbound_xcmp_suspended(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
						runtime_types::polkadot_parachain_primitives::primitives::Id,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"InboundXcmpSuspended",
						vec![],
						[
							110u8, 23u8, 239u8, 104u8, 136u8, 224u8, 179u8, 180u8, 40u8, 159u8,
							54u8, 15u8, 55u8, 111u8, 75u8, 147u8, 131u8, 127u8, 9u8, 57u8, 133u8,
							70u8, 175u8, 181u8, 232u8, 49u8, 13u8, 19u8, 59u8, 151u8, 179u8, 215u8,
						],
					)
				}
				#[doc = " The non-empty XCMP channels in order of becoming non-empty, and the index of the first"]
				#[doc = " and last outbound message. If the two indices are equal, then it indicates an empty"]
				#[doc = " queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater"]
				#[doc = " than 65535 items. Queue indices for normal messages begin at one; zero is reserved in"]
				#[doc = " case of the need to send a high-priority signal message this block."]
				#[doc = " The bool is true if there is a signal message waiting to be sent."]
				pub fn outbound_xcmp_status(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<
						runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpStatus",
						vec![],
						[
							13u8, 206u8, 22u8, 3u8, 237u8, 137u8, 239u8, 6u8, 114u8, 145u8, 66u8,
							94u8, 105u8, 20u8, 47u8, 97u8, 240u8, 42u8, 86u8, 24u8, 164u8, 46u8,
							253u8, 201u8, 115u8, 155u8, 96u8, 7u8, 224u8, 126u8, 150u8, 115u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain_primitives::primitives::Id,
					>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u16>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							163u8, 69u8, 82u8, 238u8, 52u8, 57u8, 181u8, 23u8, 138u8, 75u8, 43u8,
							208u8, 209u8, 195u8, 180u8, 199u8, 174u8, 101u8, 28u8, 248u8, 76u8,
							190u8, 140u8, 116u8, 251u8, 123u8, 160u8, 119u8, 204u8, 91u8, 59u8,
							234u8,
						],
					)
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub fn outbound_xcmp_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						::core::primitive::u8,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"OutboundXcmpMessages",
						Vec::new(),
						[
							163u8, 69u8, 82u8, 238u8, 52u8, 57u8, 181u8, 23u8, 138u8, 75u8, 43u8,
							208u8, 209u8, 195u8, 180u8, 199u8, 174u8, 101u8, 28u8, 248u8, 76u8,
							190u8, 140u8, 116u8, 251u8, 123u8, 160u8, 119u8, 204u8, 91u8, 59u8,
							234u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain_primitives::primitives::Id,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						::core::primitive::u8,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"SignalMessages",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							35u8, 133u8, 54u8, 149u8, 97u8, 64u8, 30u8, 174u8, 154u8, 60u8, 119u8,
							92u8, 207u8, 67u8, 151u8, 242u8, 6u8, 128u8, 60u8, 204u8, 15u8, 135u8,
							36u8, 234u8, 29u8, 122u8, 220u8, 28u8, 243u8, 152u8, 217u8, 61u8,
						],
					)
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub fn signal_messages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
						::core::primitive::u8,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"SignalMessages",
						Vec::new(),
						[
							35u8, 133u8, 54u8, 149u8, 97u8, 64u8, 30u8, 174u8, 154u8, 60u8, 119u8,
							92u8, 207u8, 67u8, 151u8, 242u8, 6u8, 128u8, 60u8, 204u8, 15u8, 135u8,
							36u8, 234u8, 29u8, 122u8, 220u8, 28u8, 243u8, 152u8, 217u8, 61u8,
						],
					)
				}
				#[doc = " The configuration which controls the dynamics of the outbound queue."]
				pub fn queue_config(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"QueueConfig",
						vec![],
						[
							185u8, 67u8, 247u8, 243u8, 211u8, 232u8, 57u8, 240u8, 237u8, 181u8,
							23u8, 114u8, 215u8, 128u8, 193u8, 1u8, 176u8, 53u8, 110u8, 195u8,
							148u8, 80u8, 187u8, 143u8, 62u8, 30u8, 143u8, 34u8, 248u8, 109u8, 3u8,
							141u8,
						],
					)
				}
				#[doc = " Whether or not the XCMP queue is suspended from executing incoming XCMs or not."]
				pub fn queue_suspended(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"QueueSuspended",
						vec![],
						[
							165u8, 66u8, 105u8, 244u8, 113u8, 43u8, 177u8, 252u8, 212u8, 243u8,
							143u8, 184u8, 87u8, 51u8, 163u8, 104u8, 29u8, 84u8, 119u8, 74u8, 233u8,
							129u8, 203u8, 105u8, 2u8, 101u8, 19u8, 170u8, 69u8, 253u8, 80u8, 132u8,
						],
					)
				}
				#[doc = " The factor to multiply the base delivery fee by."]
				pub fn delivery_fee_factor(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain_primitives::primitives::Id,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"DeliveryFeeFactor",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							43u8, 5u8, 63u8, 235u8, 115u8, 155u8, 130u8, 27u8, 75u8, 216u8, 177u8,
							135u8, 203u8, 147u8, 167u8, 95u8, 208u8, 188u8, 25u8, 14u8, 84u8, 63u8,
							116u8, 41u8, 148u8, 110u8, 115u8, 215u8, 196u8, 36u8, 75u8, 102u8,
						],
					)
				}
				#[doc = " The factor to multiply the base delivery fee by."]
				pub fn delivery_fee_factor_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"XcmpQueue",
						"DeliveryFeeFactor",
						Vec::new(),
						[
							43u8, 5u8, 63u8, 235u8, 115u8, 155u8, 130u8, 27u8, 75u8, 216u8, 177u8,
							135u8, 203u8, 147u8, 167u8, 95u8, 208u8, 188u8, 25u8, 14u8, 84u8, 63u8,
							116u8, 41u8, 148u8, 110u8, 115u8, 215u8, 196u8, 36u8, 75u8, 102u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The maximum number of inbound XCMP channels that can be suspended simultaneously."]
				#[doc = ""]
				#[doc = " Any further channel suspensions will fail and messages may get dropped without further"]
				#[doc = " notice. Choosing a high value (1000) is okay; the trade-off that is described in"]
				#[doc = " [`InboundXcmpSuspended`] still applies at that scale."]
				pub fn max_inbound_suspended(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"XcmpQueue",
						"MaxInboundSuspended",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " Maximal number of outbound XCMP channels that can have messages queued at the same time."]
				#[doc = ""]
				#[doc = " If this is reached, then no further messages can be sent to channels that do not yet"]
				#[doc = " have a message queued. This should be set to the expected maximum of outbound channels"]
				#[doc = " which is determined by [`Self::ChannelInfo`]. It is important to set this large enough,"]
				#[doc = " since otherwise the congestion control protocol will not work as intended and messages"]
				#[doc = " may be dropped. This value increases the PoV and should therefore not be picked too"]
				#[doc = " high. Governance needs to pay attention to not open more channels than this value."]
				pub fn max_active_outbound_channels(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"XcmpQueue",
						"MaxActiveOutboundChannels",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximal page size for HRMP message pages."]
				#[doc = ""]
				#[doc = " A lower limit can be set dynamically, but this is the hard-limit for the PoV worst case"]
				#[doc = " benchmarking. The limit for the size of a message is slightly below this, since some"]
				#[doc = " overhead is incurred for encoding the format."]
				pub fn max_page_size(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"XcmpQueue",
						"MaxPageSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod polkadot_xcm {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_xcm::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_xcm::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Send {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
				}
				impl ::subxt::blocks::StaticExtrinsic for Send {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "send";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TeleportAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedAssets>,
					pub fee_asset_item: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for TeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "teleport_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReserveTransferAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedAssets>,
					pub fee_asset_item: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "reserve_transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Execute {
					pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for Execute {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "execute";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceXcmVersion {
					pub location:
						::std::boxed::Box<runtime_types::staging_xcm::v5::location::Location>,
					pub version: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_xcm_version";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceDefaultXcmVersion {
					pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceDefaultXcmVersion {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_default_xcm_version";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSubscribeVersionNotify {
					pub location: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_subscribe_version_notify";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceUnsubscribeVersionNotify {
					pub location: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceUnsubscribeVersionNotify {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_unsubscribe_version_notify";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LimitedReserveTransferAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedAssets>,
					pub fee_asset_item: ::core::primitive::u32,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for LimitedReserveTransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_reserve_transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct LimitedTeleportAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedAssets>,
					pub fee_asset_item: ::core::primitive::u32,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for LimitedTeleportAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "limited_teleport_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ForceSuspension {
					pub suspended: ::core::primitive::bool,
				}
				impl ::subxt::blocks::StaticExtrinsic for ForceSuspension {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "force_suspension";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAssets {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedAssets>,
					pub fee_asset_item: ::core::primitive::u32,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "transfer_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ClaimAssets {
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedAssets>,
					pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for ClaimAssets {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "claim_assets";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct TransferAssetsUsingTypeAndThen {
					pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedAssets>,
					pub assets_transfer_type: ::std::boxed::Box<
						runtime_types::staging_xcm_executor::traits::asset_transfer::TransferType,
					>,
					pub remote_fees_id: ::std::boxed::Box<runtime_types::xcm::VersionedAssetId>,
					pub fees_transfer_type: ::std::boxed::Box<
						runtime_types::staging_xcm_executor::traits::asset_transfer::TransferType,
					>,
					pub custom_xcm_on_dest: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					pub weight_limit: runtime_types::xcm::v3::WeightLimit,
				}
				impl ::subxt::blocks::StaticExtrinsic for TransferAssetsUsingTypeAndThen {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "transfer_assets_using_type_and_then";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AddAuthorizedAlias {
					pub aliaser: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
					pub expires: ::core::option::Option<::core::primitive::u64>,
				}
				impl ::subxt::blocks::StaticExtrinsic for AddAuthorizedAlias {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "add_authorized_alias";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveAuthorizedAlias {
					pub aliaser: ::std::boxed::Box<runtime_types::xcm::VersionedLocation>,
				}
				impl ::subxt::blocks::StaticExtrinsic for RemoveAuthorizedAlias {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "remove_authorized_alias";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoveAllAuthorizedAliases;
				impl ::subxt::blocks::StaticExtrinsic for RemoveAllAuthorizedAliases {
					const PALLET: &'static str = "PolkadotXcm";
					const CALL: &'static str = "remove_all_authorized_aliases";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::Payload<types::Send> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"send",
						types::Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						},
						[
							209u8, 111u8, 170u8, 6u8, 115u8, 11u8, 18u8, 171u8, 249u8, 3u8, 67u8,
							107u8, 212u8, 16u8, 140u8, 96u8, 29u8, 157u8, 20u8, 1u8, 21u8, 19u8,
							105u8, 188u8, 10u8, 5u8, 87u8, 67u8, 71u8, 188u8, 35u8, 66u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "**This function is deprecated: Use `limited_teleport_assets` instead.**"]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` chain."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedLocation,
					beneficiary: runtime_types::xcm::VersionedLocation,
					assets: runtime_types::xcm::VersionedAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::TeleportAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"teleport_assets",
						types::TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							31u8, 60u8, 0u8, 220u8, 157u8, 38u8, 28u8, 140u8, 79u8, 243u8, 182u8,
							229u8, 158u8, 45u8, 213u8, 132u8, 149u8, 196u8, 212u8, 239u8, 23u8,
							19u8, 69u8, 27u8, 250u8, 110u8, 193u8, 60u8, 227u8, 252u8, 174u8, 35u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
				#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
				#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
				#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
				#[doc = ""]
				#[doc = "**This function is deprecated: Use `limited_reserve_transfer_assets` instead.**"]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedLocation,
					beneficiary: runtime_types::xcm::VersionedLocation,
					assets: runtime_types::xcm::VersionedAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"reserve_transfer_assets",
						types::ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							76u8, 122u8, 201u8, 193u8, 160u8, 210u8, 58u8, 150u8, 236u8, 130u8,
							225u8, 28u8, 35u8, 9u8, 206u8, 235u8, 14u8, 101u8, 193u8, 118u8, 145u8,
							230u8, 112u8, 65u8, 172u8, 251u8, 62u8, 64u8, 130u8, 223u8, 153u8,
							139u8,
						],
					)
				}
				#[doc = "Execute an XCM message from a local, signed, origin."]
				#[doc = ""]
				#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
				#[doc = "partially."]
				#[doc = ""]
				#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than"]
				#[doc = "the maximum amount of weight that the message could take to be executed, then no"]
				#[doc = "execution attempt will be made."]
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm2,
					max_weight: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::Execute> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"execute",
						types::Execute { message: ::std::boxed::Box::new(message), max_weight },
						[
							122u8, 9u8, 129u8, 102u8, 188u8, 214u8, 143u8, 187u8, 175u8, 221u8,
							157u8, 67u8, 208u8, 30u8, 97u8, 133u8, 171u8, 14u8, 144u8, 97u8, 18u8,
							124u8, 196u8, 254u8, 70u8, 31u8, 175u8, 197u8, 230u8, 36u8, 147u8,
							211u8,
						],
					)
				}
				#[doc = "Extoll that a particular destination can be communicated with through a particular"]
				#[doc = "version of XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The destination that is being described."]
				#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
				pub fn force_xcm_version(
					&self,
					location: runtime_types::staging_xcm::v5::location::Location,
					version: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ForceXcmVersion> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_xcm_version",
						types::ForceXcmVersion {
							location: ::std::boxed::Box::new(location),
							version,
						},
						[
							136u8, 43u8, 72u8, 5u8, 164u8, 97u8, 177u8, 61u8, 8u8, 112u8, 148u8,
							43u8, 0u8, 23u8, 134u8, 21u8, 173u8, 181u8, 207u8, 249u8, 98u8, 122u8,
							74u8, 131u8, 172u8, 12u8, 146u8, 124u8, 220u8, 97u8, 126u8, 253u8,
						],
					)
				}
				#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
				#[doc = "version a destination can accept is unknown)."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::Payload<types::ForceDefaultXcmVersion> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_default_xcm_version",
						types::ForceDefaultXcmVersion { maybe_xcm_version },
						[
							43u8, 114u8, 102u8, 104u8, 209u8, 234u8, 108u8, 173u8, 109u8, 188u8,
							94u8, 214u8, 136u8, 43u8, 153u8, 75u8, 161u8, 192u8, 76u8, 12u8, 221u8,
							237u8, 158u8, 247u8, 41u8, 193u8, 35u8, 174u8, 183u8, 207u8, 79u8,
							213u8,
						],
					)
				}
				#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedLocation,
				) -> ::subxt::tx::Payload<types::ForceSubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_subscribe_version_notify",
						types::ForceSubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							51u8, 103u8, 204u8, 180u8, 35u8, 50u8, 212u8, 76u8, 243u8, 161u8, 5u8,
							180u8, 61u8, 194u8, 181u8, 13u8, 209u8, 18u8, 182u8, 26u8, 138u8,
							139u8, 205u8, 98u8, 62u8, 185u8, 194u8, 240u8, 5u8, 60u8, 245u8, 91u8,
						],
					)
				}
				#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
				#[doc = "version changes."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
				#[doc = "  notifications which we no longer desire."]
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedLocation,
				) -> ::subxt::tx::Payload<types::ForceUnsubscribeVersionNotify> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_unsubscribe_version_notify",
						types::ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							80u8, 153u8, 123u8, 155u8, 105u8, 164u8, 139u8, 252u8, 89u8, 174u8,
							54u8, 14u8, 99u8, 172u8, 85u8, 239u8, 45u8, 141u8, 84u8, 69u8, 47u8,
							18u8, 173u8, 201u8, 137u8, 186u8, 217u8, 105u8, 105u8, 20u8, 6u8,
							198u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location and may not be teleportable to `dest`."]
				#[doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `assets` have destination reserve: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"]
				#[doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"]
				#[doc = "   to mint and deposit reserve-based assets to `beneficiary`."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedLocation,
					beneficiary: runtime_types::xcm::VersionedLocation,
					assets: runtime_types::xcm::VersionedAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::LimitedReserveTransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"limited_reserve_transfer_assets",
						types::LimitedReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							72u8, 168u8, 103u8, 54u8, 253u8, 3u8, 152u8, 167u8, 60u8, 214u8, 24u8,
							47u8, 179u8, 36u8, 251u8, 15u8, 213u8, 191u8, 95u8, 128u8, 93u8, 42u8,
							205u8, 37u8, 214u8, 170u8, 241u8, 71u8, 176u8, 11u8, 43u8, 74u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` chain."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedLocation,
					beneficiary: runtime_types::xcm::VersionedLocation,
					assets: runtime_types::xcm::VersionedAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::LimitedTeleportAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"limited_teleport_assets",
						types::LimitedTeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							56u8, 190u8, 251u8, 133u8, 34u8, 100u8, 32u8, 57u8, 114u8, 73u8, 153u8,
							74u8, 178u8, 228u8, 239u8, 87u8, 242u8, 202u8, 56u8, 66u8, 22u8, 216u8,
							113u8, 25u8, 233u8, 238u8, 164u8, 76u8, 144u8, 204u8, 219u8, 91u8,
						],
					)
				}
				#[doc = "Set or unset the global suspension state of the XCM executor."]
				#[doc = ""]
				#[doc = "- `origin`: Must be an origin specified by AdminOrigin."]
				#[doc = "- `suspended`: `true` to suspend, `false` to resume."]
				pub fn force_suspension(
					&self,
					suspended: ::core::primitive::bool,
				) -> ::subxt::tx::Payload<types::ForceSuspension> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"force_suspension",
						types::ForceSuspension { suspended },
						[
							78u8, 125u8, 93u8, 55u8, 129u8, 44u8, 36u8, 227u8, 75u8, 46u8, 68u8,
							202u8, 81u8, 127u8, 111u8, 92u8, 149u8, 38u8, 225u8, 185u8, 183u8,
							154u8, 89u8, 159u8, 79u8, 10u8, 229u8, 1u8, 226u8, 243u8, 65u8, 238u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the destination chain through their local,"]
				#[doc = "destination or remote reserve, or through teleports."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item` (hence referred to as `fees`), up to enough to pay for"]
				#[doc = "`weight_limit` of weight. If more weight is needed than `weight_limit`, then the"]
				#[doc = "operation will fail and the sent assets may be at risk."]
				#[doc = ""]
				#[doc = "`assets` (excluding `fees`) must have same reserve location or otherwise be teleportable"]
				#[doc = "to `dest`, no limitations imposed on `fees`."]
				#[doc = " - for local reserve: transfer assets to sovereign account of destination chain and"]
				#[doc = "   forward a notification XCM to `dest` to mint and deposit reserve-based assets to"]
				#[doc = "   `beneficiary`."]
				#[doc = " - for destination reserve: burn local assets and forward a notification to `dest` chain"]
				#[doc = "   to withdraw the reserve assets from this chain's sovereign account and deposit them"]
				#[doc = "   to `beneficiary`."]
				#[doc = " - for remote reserve: burn local assets, forward XCM to reserve chain to move reserves"]
				#[doc = "   from this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint"]
				#[doc = "   and deposit reserve-based assets to `beneficiary`."]
				#[doc = " - for teleports: burn local assets and forward XCM to `dest` chain to mint/teleport"]
				#[doc = "   assets and deposit them to `beneficiary`."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent,"]
				#[doc = "  Parachain(..))` to send from parachain to parachain, or `X1(Parachain(..))` to send"]
				#[doc = "  from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"]
				#[doc = "  generally be an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedLocation,
					beneficiary: runtime_types::xcm::VersionedLocation,
					assets: runtime_types::xcm::VersionedAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::TransferAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"transfer_assets",
						types::TransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							204u8, 118u8, 44u8, 144u8, 51u8, 77u8, 235u8, 235u8, 86u8, 166u8, 92u8,
							106u8, 197u8, 151u8, 154u8, 136u8, 137u8, 206u8, 111u8, 118u8, 94u8,
							22u8, 7u8, 21u8, 13u8, 169u8, 214u8, 87u8, 84u8, 140u8, 6u8, 54u8,
						],
					)
				}
				#[doc = "Claims assets trapped on this pallet because of leftover assets during XCM execution."]
				#[doc = ""]
				#[doc = "- `origin`: Anyone can call this extrinsic."]
				#[doc = "- `assets`: The exact assets that were trapped. Use the version to specify what version"]
				#[doc = "was the latest when they were trapped."]
				#[doc = "- `beneficiary`: The location/account where the claimed assets will be deposited."]
				pub fn claim_assets(
					&self,
					assets: runtime_types::xcm::VersionedAssets,
					beneficiary: runtime_types::xcm::VersionedLocation,
				) -> ::subxt::tx::Payload<types::ClaimAssets> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"claim_assets",
						types::ClaimAssets {
							assets: ::std::boxed::Box::new(assets),
							beneficiary: ::std::boxed::Box::new(beneficiary),
						},
						[
							7u8, 158u8, 80u8, 180u8, 145u8, 151u8, 34u8, 132u8, 236u8, 243u8, 77u8,
							177u8, 66u8, 172u8, 57u8, 182u8, 226u8, 110u8, 246u8, 159u8, 61u8,
							31u8, 167u8, 210u8, 226u8, 215u8, 103u8, 234u8, 16u8, 95u8, 92u8,
							248u8,
						],
					)
				}
				#[doc = "Transfer assets from the local chain to the destination chain using explicit transfer"]
				#[doc = "types for assets and fees."]
				#[doc = ""]
				#[doc = "`assets` must have same reserve location or may be teleportable to `dest`. Caller must"]
				#[doc = "provide the `assets_transfer_type` to be used for `assets`:"]
				#[doc = " - `TransferType::LocalReserve`: transfer assets to sovereign account of destination"]
				#[doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"]
				#[doc = "   assets to `beneficiary`."]
				#[doc = " - `TransferType::DestinationReserve`: burn local assets and forward a notification to"]
				#[doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"]
				#[doc = "   deposit them to `beneficiary`."]
				#[doc = " - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve`"]
				#[doc = "   chain to move reserves from this chain's SA to `dest` chain's SA, and forward another"]
				#[doc = "   XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`. Typically"]
				#[doc = "   the remote `reserve` is Asset Hub."]
				#[doc = " - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to"]
				#[doc = "   mint/teleport assets and deposit them to `beneficiary`."]
				#[doc = ""]
				#[doc = "On the destination chain, as well as any intermediary hops, `BuyExecution` is used to"]
				#[doc = "buy execution using transferred `assets` identified by `remote_fees_id`."]
				#[doc = "Make sure enough of the specified `remote_fees_id` asset is included in the given list"]
				#[doc = "of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "`remote_fees_id` may use different transfer type than rest of `assets` and can be"]
				#[doc = "specified through `fees_transfer_type`."]
				#[doc = ""]
				#[doc = "The caller needs to specify what should happen to the transferred assets once they reach"]
				#[doc = "the `dest` chain. This is done through the `custom_xcm_on_dest` parameter, which"]
				#[doc = "contains the instructions to execute on `dest` as a final step."]
				#[doc = "  This is usually as simple as:"]
				#[doc = "  `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())), beneficiary }])`,"]
				#[doc = "  but could be something more exotic like sending the `assets` even further."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"]
				#[doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"]
				#[doc = "  relay to parachain, or `(parents: 2, (GlobalConsensus(..), ..))` to send from"]
				#[doc = "  parachain across a bridge to another ecosystem destination."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"]
				#[doc = "  fee on the `dest` (and possibly reserve) chains."]
				#[doc = "- `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`."]
				#[doc = "- `remote_fees_id`: One of the included `assets` to be used to pay fees."]
				#[doc = "- `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets."]
				#[doc = "- `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the"]
				#[doc = "  transfer, which also determines what happens to the assets on the destination chain."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn transfer_assets_using_type_and_then(
					&self,
					dest: runtime_types::xcm::VersionedLocation,
					assets: runtime_types::xcm::VersionedAssets,
					assets_transfer_type : runtime_types :: staging_xcm_executor :: traits :: asset_transfer :: TransferType,
					remote_fees_id: runtime_types::xcm::VersionedAssetId,
					fees_transfer_type : runtime_types :: staging_xcm_executor :: traits :: asset_transfer :: TransferType,
					custom_xcm_on_dest: runtime_types::xcm::VersionedXcm,
					weight_limit: runtime_types::xcm::v3::WeightLimit,
				) -> ::subxt::tx::Payload<types::TransferAssetsUsingTypeAndThen> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"transfer_assets_using_type_and_then",
						types::TransferAssetsUsingTypeAndThen {
							dest: ::std::boxed::Box::new(dest),
							assets: ::std::boxed::Box::new(assets),
							assets_transfer_type: ::std::boxed::Box::new(assets_transfer_type),
							remote_fees_id: ::std::boxed::Box::new(remote_fees_id),
							fees_transfer_type: ::std::boxed::Box::new(fees_transfer_type),
							custom_xcm_on_dest: ::std::boxed::Box::new(custom_xcm_on_dest),
							weight_limit,
						},
						[
							199u8, 248u8, 143u8, 192u8, 39u8, 87u8, 220u8, 150u8, 207u8, 131u8,
							122u8, 214u8, 240u8, 15u8, 201u8, 146u8, 166u8, 101u8, 154u8, 151u8,
							218u8, 25u8, 195u8, 200u8, 96u8, 141u8, 210u8, 113u8, 16u8, 238u8,
							208u8, 192u8,
						],
					)
				}
				#[doc = "Authorize another `aliaser` location to alias into the local `origin` making this call."]
				#[doc = "The `aliaser` is only authorized until the provided `expiry` block number."]
				#[doc = "The call can also be used for a previously authorized alias in order to update its"]
				#[doc = "`expiry` block number."]
				#[doc = ""]
				#[doc = "Usually useful to allow your local account to be aliased into from a remote location"]
				#[doc = "also under your control (like your account on another chain)."]
				#[doc = ""]
				#[doc = "WARNING: make sure the caller `origin` (you) trusts the `aliaser` location to act in"]
				#[doc = "their/your name. Once authorized using this call, the `aliaser` can freely impersonate"]
				#[doc = "`origin` in XCM programs executed on the local chain."]
				pub fn add_authorized_alias(
					&self,
					aliaser: runtime_types::xcm::VersionedLocation,
					expires: ::core::option::Option<::core::primitive::u64>,
				) -> ::subxt::tx::Payload<types::AddAuthorizedAlias> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"add_authorized_alias",
						types::AddAuthorizedAlias {
							aliaser: ::std::boxed::Box::new(aliaser),
							expires,
						},
						[
							223u8, 55u8, 95u8, 81u8, 3u8, 249u8, 197u8, 169u8, 247u8, 139u8, 84u8,
							142u8, 87u8, 70u8, 51u8, 169u8, 137u8, 190u8, 116u8, 253u8, 220u8,
							101u8, 221u8, 132u8, 245u8, 23u8, 0u8, 212u8, 3u8, 54u8, 60u8, 78u8,
						],
					)
				}
				#[doc = "Remove a previously authorized `aliaser` from the list of locations that can alias into"]
				#[doc = "the local `origin` making this call."]
				pub fn remove_authorized_alias(
					&self,
					aliaser: runtime_types::xcm::VersionedLocation,
				) -> ::subxt::tx::Payload<types::RemoveAuthorizedAlias> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"remove_authorized_alias",
						types::RemoveAuthorizedAlias { aliaser: ::std::boxed::Box::new(aliaser) },
						[
							210u8, 231u8, 143u8, 176u8, 120u8, 169u8, 22u8, 200u8, 5u8, 41u8, 51u8,
							229u8, 158u8, 72u8, 19u8, 54u8, 204u8, 207u8, 191u8, 47u8, 145u8, 71u8,
							204u8, 235u8, 75u8, 245u8, 190u8, 106u8, 119u8, 203u8, 66u8, 0u8,
						],
					)
				}
				#[doc = "Remove all previously authorized `aliaser`s that can alias into the local `origin`"]
				#[doc = "making this call."]
				pub fn remove_all_authorized_aliases(
					&self,
				) -> ::subxt::tx::Payload<types::RemoveAllAuthorizedAliases> {
					::subxt::tx::Payload::new_static(
						"PolkadotXcm",
						"remove_all_authorized_aliases",
						types::RemoveAllAuthorizedAliases {},
						[
							223u8, 17u8, 58u8, 180u8, 190u8, 164u8, 106u8, 17u8, 237u8, 243u8,
							160u8, 39u8, 13u8, 103u8, 166u8, 51u8, 192u8, 73u8, 193u8, 21u8, 69u8,
							170u8, 101u8, 195u8, 42u8, 123u8, 56u8, 90u8, 8u8, 109u8, 15u8, 110u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Execution of an XCM message was attempted."]
			pub struct Attempted {
				pub outcome: runtime_types::staging_xcm::v5::traits::Outcome,
			}
			impl ::subxt::events::StaticEvent for Attempted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An XCM message was sent."]
			pub struct Sent {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub destination: runtime_types::staging_xcm::v5::location::Location,
				pub message: runtime_types::staging_xcm::v5::Xcm,
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for Sent {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An XCM message failed to send."]
			pub struct SendFailed {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub destination: runtime_types::staging_xcm::v5::location::Location,
				pub error: runtime_types::xcm::v3::traits::SendError,
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for SendFailed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SendFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An XCM message failed to process."]
			pub struct ProcessXcmError {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub error: runtime_types::xcm::v5::traits::Error,
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for ProcessXcmError {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ProcessXcmError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Query response received which does not match a registered query. This may be because a"]
			#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
			#[doc = "because the query timed out."]
			pub struct UnexpectedResponse {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub query_id: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
			#[doc = "no registered notification call."]
			pub struct ResponseReady {
				pub query_id: ::core::primitive::u64,
				pub response: runtime_types::staging_xcm::v5::Response,
			}
			impl ::subxt::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. The registered notification has"]
			#[doc = "been dispatched and executed successfully."]
			pub struct Notified {
				pub query_id: ::core::primitive::u64,
				pub pallet_index: ::core::primitive::u8,
				pub call_index: ::core::primitive::u8,
			}
			impl ::subxt::events::StaticEvent for Notified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Notified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. The registered notification"]
			#[doc = "could not be dispatched because the dispatch weight is greater than the maximum weight"]
			#[doc = "originally budgeted by this runtime for the query result."]
			pub struct NotifyOverweight {
				pub query_id: ::core::primitive::u64,
				pub pallet_index: ::core::primitive::u8,
				pub call_index: ::core::primitive::u8,
				pub actual_weight: runtime_types::sp_weights::weight_v2::Weight,
				pub max_budgeted_weight: runtime_types::sp_weights::weight_v2::Weight,
			}
			impl ::subxt::events::StaticEvent for NotifyOverweight {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. There was a general error with"]
			#[doc = "dispatching the notification call."]
			pub struct NotifyDispatchError {
				pub query_id: ::core::primitive::u64,
				pub pallet_index: ::core::primitive::u8,
				pub call_index: ::core::primitive::u8,
			}
			impl ::subxt::events::StaticEvent for NotifyDispatchError {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
			#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
			#[doc = "is not `(origin, QueryId, Response)`."]
			pub struct NotifyDecodeFailed {
				pub query_id: ::core::primitive::u64,
				pub pallet_index: ::core::primitive::u8,
				pub call_index: ::core::primitive::u8,
			}
			impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the origin location of the response does"]
			#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
			#[doc = "be received and acted upon."]
			pub struct InvalidResponder {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub query_id: ::core::primitive::u64,
				pub expected_location:
					::core::option::Option<runtime_types::staging_xcm::v5::location::Location>,
			}
			impl ::subxt::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the expected origin location placed in"]
			#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
			#[doc = ""]
			#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
			#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
			#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
			#[doc = "needed."]
			pub struct InvalidResponderVersion {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub query_id: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for InvalidResponderVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Received query response has been read and removed."]
			pub struct ResponseTaken {
				pub query_id: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some assets have been placed in an asset trap."]
			pub struct AssetsTrapped {
				pub hash: ::subxt::utils::H256,
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub assets: runtime_types::xcm::VersionedAssets,
			}
			impl ::subxt::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An XCM version change notification message has been attempted to be sent."]
			#[doc = ""]
			#[doc = "The cost of sending it (borne by the chain) is included."]
			pub struct VersionChangeNotified {
				pub destination: runtime_types::staging_xcm::v5::location::Location,
				pub result: ::core::primitive::u32,
				pub cost: runtime_types::staging_xcm::v5::asset::Assets,
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "The supported version of a location has been changed. This might be through an"]
			#[doc = "automatic notification or a manual intervention."]
			pub struct SupportedVersionChanged {
				pub location: runtime_types::staging_xcm::v5::location::Location,
				pub version: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "sending the notification to it."]
			pub struct NotifyTargetSendFail {
				pub location: runtime_types::staging_xcm::v5::location::Location,
				pub query_id: ::core::primitive::u64,
				pub error: runtime_types::xcm::v5::traits::Error,
			}
			impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "migrating the location to our new XCM format."]
			pub struct NotifyTargetMigrationFail {
				pub location: runtime_types::xcm::VersionedLocation,
				pub query_id: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the expected querier location placed in"]
			#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
			#[doc = ""]
			#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
			#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
			#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
			#[doc = "needed."]
			pub struct InvalidQuerierVersion {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub query_id: ::core::primitive::u64,
			}
			impl ::subxt::events::StaticEvent for InvalidQuerierVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerierVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Expected query response has been received but the querier location of the response does"]
			#[doc = "not match the expected. The query remains registered for a later, valid, response to"]
			#[doc = "be received and acted upon."]
			pub struct InvalidQuerier {
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub query_id: ::core::primitive::u64,
				pub expected_querier: runtime_types::staging_xcm::v5::location::Location,
				pub maybe_actual_querier:
					::core::option::Option<runtime_types::staging_xcm::v5::location::Location>,
			}
			impl ::subxt::events::StaticEvent for InvalidQuerier {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidQuerier";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A remote has requested XCM version change notification from us and we have honored it."]
			#[doc = "A version information message is sent to them and its cost is included."]
			pub struct VersionNotifyStarted {
				pub destination: runtime_types::staging_xcm::v5::location::Location,
				pub cost: runtime_types::staging_xcm::v5::asset::Assets,
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for VersionNotifyStarted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "We have requested that a remote chain send us XCM version change notifications."]
			pub struct VersionNotifyRequested {
				pub destination: runtime_types::staging_xcm::v5::location::Location,
				pub cost: runtime_types::staging_xcm::v5::asset::Assets,
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for VersionNotifyRequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyRequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "We have requested that a remote chain stops sending us XCM version change"]
			#[doc = "notifications."]
			pub struct VersionNotifyUnrequested {
				pub destination: runtime_types::staging_xcm::v5::location::Location,
				pub cost: runtime_types::staging_xcm::v5::asset::Assets,
				pub message_id: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for VersionNotifyUnrequested {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionNotifyUnrequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
			pub struct FeesPaid {
				pub paying: runtime_types::staging_xcm::v5::location::Location,
				pub fees: runtime_types::staging_xcm::v5::asset::Assets,
			}
			impl ::subxt::events::StaticEvent for FeesPaid {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "FeesPaid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Some assets have been claimed from an asset trap"]
			pub struct AssetsClaimed {
				pub hash: ::subxt::utils::H256,
				pub origin: runtime_types::staging_xcm::v5::location::Location,
				pub assets: runtime_types::xcm::VersionedAssets,
			}
			impl ::subxt::events::StaticEvent for AssetsClaimed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsClaimed";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "A XCM version migration finished."]
			pub struct VersionMigrationFinished {
				pub version: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for VersionMigrationFinished {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionMigrationFinished";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "An `aliaser` location was authorized by `target` to alias it, authorization valid until"]
			#[doc = "`expiry` block number."]
			pub struct AliasAuthorized {
				pub aliaser: runtime_types::staging_xcm::v5::location::Location,
				pub target: runtime_types::staging_xcm::v5::location::Location,
				pub expiry: ::core::option::Option<::core::primitive::u64>,
			}
			impl ::subxt::events::StaticEvent for AliasAuthorized {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AliasAuthorized";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "`target` removed alias authorization for `aliaser`."]
			pub struct AliasAuthorizationRemoved {
				pub aliaser: runtime_types::staging_xcm::v5::location::Location,
				pub target: runtime_types::staging_xcm::v5::location::Location,
			}
			impl ::subxt::events::StaticEvent for AliasAuthorizationRemoved {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AliasAuthorizationRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "`target` removed all alias authorizations."]
			pub struct AliasesAuthorizationsRemoved {
				pub target: runtime_types::staging_xcm::v5::location::Location,
			}
			impl ::subxt::events::StaticEvent for AliasesAuthorizationsRemoved {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AliasesAuthorizationsRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The latest available query index."]
				pub fn query_counter(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"QueryCounter",
						vec![],
						[
							216u8, 73u8, 160u8, 232u8, 60u8, 245u8, 218u8, 219u8, 152u8, 68u8,
							146u8, 219u8, 255u8, 7u8, 86u8, 112u8, 83u8, 49u8, 94u8, 173u8, 64u8,
							203u8, 147u8, 226u8, 236u8, 39u8, 129u8, 106u8, 209u8, 113u8, 150u8,
							50u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"Queries",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							134u8, 206u8, 252u8, 211u8, 156u8, 173u8, 214u8, 205u8, 17u8, 177u8,
							139u8, 121u8, 43u8, 29u8, 30u8, 233u8, 210u8, 222u8, 172u8, 171u8,
							13u8, 223u8, 153u8, 88u8, 43u8, 44u8, 183u8, 253u8, 252u8, 251u8,
							184u8, 249u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"Queries",
						Vec::new(),
						[
							134u8, 206u8, 252u8, 211u8, 156u8, 173u8, 214u8, 205u8, 17u8, 177u8,
							139u8, 121u8, 43u8, 29u8, 30u8, 233u8, 210u8, 222u8, 172u8, 171u8,
							13u8, 223u8, 153u8, 88u8, 43u8, 44u8, 183u8, 253u8, 252u8, 251u8,
							184u8, 249u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `Assets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AssetTraps",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
							50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
							126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `Assets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AssetTraps",
						Vec::new(),
						[
							148u8, 41u8, 254u8, 134u8, 61u8, 172u8, 126u8, 146u8, 78u8, 178u8,
							50u8, 77u8, 226u8, 8u8, 200u8, 78u8, 77u8, 91u8, 26u8, 133u8, 104u8,
							126u8, 28u8, 28u8, 202u8, 62u8, 87u8, 183u8, 231u8, 191u8, 5u8, 181u8,
						],
					)
				}
				#[doc = " Default version to encode XCM when latest version of destination is unknown. If `None`,"]
				#[doc = " then the destinations whose XCM version is unknown are considered unreachable."]
				pub fn safe_xcm_version(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SafeXcmVersion",
						vec![],
						[
							187u8, 8u8, 74u8, 126u8, 80u8, 215u8, 177u8, 60u8, 223u8, 123u8, 196u8,
							155u8, 166u8, 66u8, 25u8, 164u8, 191u8, 66u8, 116u8, 131u8, 116u8,
							188u8, 224u8, 122u8, 75u8, 195u8, 246u8, 188u8, 83u8, 134u8, 49u8,
							143u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							156u8, 153u8, 195u8, 67u8, 72u8, 227u8, 183u8, 107u8, 71u8, 221u8,
							125u8, 172u8, 34u8, 22u8, 56u8, 182u8, 219u8, 223u8, 183u8, 137u8,
							243u8, 231u8, 153u8, 254u8, 144u8, 104u8, 48u8, 189u8, 232u8, 104u8,
							180u8, 65u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u32,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"SupportedVersion",
						Vec::new(),
						[
							156u8, 153u8, 195u8, 67u8, 72u8, 227u8, 183u8, 107u8, 71u8, 221u8,
							125u8, 172u8, 34u8, 22u8, 56u8, 182u8, 219u8, 223u8, 183u8, 137u8,
							243u8, 231u8, 153u8, 254u8, 144u8, 104u8, 48u8, 189u8, 232u8, 104u8,
							180u8, 65u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							216u8, 78u8, 44u8, 71u8, 246u8, 59u8, 163u8, 153u8, 68u8, 31u8, 197u8,
							114u8, 33u8, 203u8, 20u8, 60u8, 61u8, 177u8, 94u8, 13u8, 213u8, 203u8,
							150u8, 145u8, 134u8, 249u8, 53u8, 21u8, 122u8, 208u8, 66u8, 67u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::u64,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifiers",
						Vec::new(),
						[
							216u8, 78u8, 44u8, 71u8, 246u8, 59u8, 163u8, 153u8, 68u8, 31u8, 197u8,
							114u8, 33u8, 203u8, 20u8, 60u8, 61u8, 177u8, 94u8, 13u8, 213u8, 203u8,
							150u8, 145u8, 134u8, 249u8, 53u8, 21u8, 122u8, 208u8, 66u8, 67u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u64,
						runtime_types::sp_weights::weight_v2::Weight,
						::core::primitive::u32,
					),
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							166u8, 29u8, 245u8, 121u8, 177u8, 119u8, 188u8, 0u8, 32u8, 188u8, 9u8,
							180u8, 60u8, 28u8, 161u8, 5u8, 189u8, 78u8, 238u8, 14u8, 148u8, 5u8,
							151u8, 153u8, 62u8, 163u8, 144u8, 82u8, 91u8, 227u8, 210u8, 205u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					(
						::core::primitive::u64,
						runtime_types::sp_weights::weight_v2::Weight,
						::core::primitive::u32,
					),
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionNotifyTargets",
						Vec::new(),
						[
							166u8, 29u8, 245u8, 121u8, 177u8, 119u8, 188u8, 0u8, 32u8, 188u8, 9u8,
							180u8, 60u8, 28u8, 161u8, 5u8, 189u8, 78u8, 238u8, 14u8, 148u8, 5u8,
							151u8, 153u8, 62u8, 163u8, 144u8, 82u8, 91u8, 227u8, 210u8, 205u8,
						],
					)
				}
				#[doc = " Destinations whose latest XCM version we would like to know. Duplicates not allowed, and"]
				#[doc = " the `u32` counter is the number of times that a send to the destination has been attempted,"]
				#[doc = " which is used as a prioritization."]
				pub fn version_discovery_queue(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						runtime_types::xcm::VersionedLocation,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"VersionDiscoveryQueue",
						vec![],
						[
							206u8, 152u8, 58u8, 105u8, 70u8, 142u8, 210u8, 246u8, 107u8, 8u8,
							190u8, 195u8, 255u8, 27u8, 199u8, 241u8, 221u8, 238u8, 61u8, 92u8,
							245u8, 162u8, 151u8, 234u8, 151u8, 6u8, 216u8, 115u8, 214u8, 138u8,
							8u8, 27u8,
						],
					)
				}
				#[doc = " The current migration's stage, if any."]
				pub fn current_migration(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::VersionMigrationStage,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"CurrentMigration",
						vec![],
						[
							74u8, 138u8, 181u8, 162u8, 59u8, 251u8, 37u8, 28u8, 232u8, 51u8, 30u8,
							152u8, 252u8, 133u8, 95u8, 195u8, 47u8, 127u8, 21u8, 44u8, 62u8, 143u8,
							170u8, 234u8, 160u8, 37u8, 131u8, 179u8, 57u8, 241u8, 140u8, 124u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on a remote chain."]
				pub fn remote_locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
					_2: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedAssetId>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
						],
						[
							166u8, 178u8, 87u8, 32u8, 245u8, 121u8, 41u8, 67u8, 60u8, 239u8, 43u8,
							155u8, 114u8, 241u8, 54u8, 176u8, 63u8, 204u8, 197u8, 250u8, 60u8,
							185u8, 88u8, 124u8, 242u8, 145u8, 45u8, 16u8, 248u8, 181u8, 236u8,
							11u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on a remote chain."]
				pub fn remote_locked_fungibles_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"RemoteLockedFungibles",
						Vec::new(),
						[
							166u8, 178u8, 87u8, 32u8, 245u8, 121u8, 41u8, 67u8, 60u8, 239u8, 43u8,
							155u8, 114u8, 241u8, 54u8, 176u8, 63u8, 204u8, 197u8, 250u8, 60u8,
							185u8, 88u8, 124u8, 242u8, 145u8, 45u8, 16u8, 248u8, 181u8, 236u8,
							11u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on this chain."]
				pub fn locked_fungibles(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u128,
						runtime_types::xcm::VersionedLocation,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							112u8, 157u8, 87u8, 224u8, 37u8, 77u8, 11u8, 17u8, 173u8, 230u8, 168u8,
							230u8, 33u8, 8u8, 209u8, 110u8, 182u8, 34u8, 118u8, 28u8, 15u8, 14u8,
							185u8, 50u8, 16u8, 52u8, 90u8, 125u8, 46u8, 20u8, 120u8, 136u8,
						],
					)
				}
				#[doc = " Fungible assets which we know are locked on this chain."]
				pub fn locked_fungibles_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						::core::primitive::u128,
						runtime_types::xcm::VersionedLocation,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"LockedFungibles",
						Vec::new(),
						[
							112u8, 157u8, 87u8, 224u8, 37u8, 77u8, 11u8, 17u8, 173u8, 230u8, 168u8,
							230u8, 33u8, 8u8, 209u8, 110u8, 182u8, 34u8, 118u8, 28u8, 15u8, 14u8,
							185u8, 50u8, 16u8, 52u8, 90u8, 125u8, 46u8, 20u8, 120u8, 136u8,
						],
					)
				}
				#[doc = " Global suspension state of the XCM executor."]
				pub fn xcm_execution_suspended(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"XcmExecutionSuspended",
						vec![],
						[
							182u8, 54u8, 69u8, 68u8, 78u8, 76u8, 103u8, 79u8, 47u8, 136u8, 99u8,
							104u8, 128u8, 129u8, 249u8, 54u8, 214u8, 136u8, 97u8, 48u8, 178u8,
							42u8, 26u8, 27u8, 82u8, 24u8, 33u8, 77u8, 33u8, 27u8, 20u8, 127u8,
						],
					)
				}
				#[doc = " Whether or not incoming XCMs (both executed locally and received) should be recorded."]
				#[doc = " Only one XCM program will be recorded at a time."]
				#[doc = " This is meant to be used in runtime APIs, and it's advised it stays false"]
				#[doc = " for all other use cases, so as to not degrade regular performance."]
				#[doc = ""]
				#[doc = " Only relevant if this pallet is being used as the [`xcm_executor::traits::RecordXcm`]"]
				#[doc = " implementation in the XCM executor configuration."]
				pub fn should_record_xcm(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					::core::primitive::bool,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"ShouldRecordXcm",
						vec![],
						[
							77u8, 184u8, 154u8, 92u8, 185u8, 225u8, 131u8, 210u8, 55u8, 115u8, 3u8,
							182u8, 191u8, 132u8, 51u8, 136u8, 42u8, 136u8, 54u8, 36u8, 229u8,
							229u8, 47u8, 88u8, 4u8, 175u8, 136u8, 78u8, 226u8, 253u8, 13u8, 178u8,
						],
					)
				}
				#[doc = " If [`ShouldRecordXcm`] is set to true, then the last XCM program executed locally"]
				#[doc = " will be stored here."]
				#[doc = " Runtime APIs can fetch the XCM that was executed by accessing this value."]
				#[doc = ""]
				#[doc = " Only relevant if this pallet is being used as the [`xcm_executor::traits::RecordXcm`]"]
				#[doc = " implementation in the XCM executor configuration."]
				pub fn recorded_xcm(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::staging_xcm::v5::Xcm,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"RecordedXcm",
						vec![],
						[
							21u8, 172u8, 234u8, 160u8, 115u8, 240u8, 135u8, 8u8, 11u8, 62u8, 121u8,
							113u8, 13u8, 164u8, 179u8, 0u8, 139u8, 216u8, 216u8, 236u8, 135u8,
							116u8, 200u8, 199u8, 199u8, 249u8, 211u8, 0u8, 4u8, 86u8, 187u8, 198u8,
						],
					)
				}
				#[doc = " Map of authorized aliasers of local origins. Each local location can authorize a list of"]
				#[doc = " other locations to alias into it. Each aliaser is only valid until its inner `expiry`"]
				#[doc = " block number."]
				pub fn authorized_aliases(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedLocation>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::AuthorizedAliasesEntry<
						(),
						runtime_types::pallet_xcm::pallet::MaxAuthorizedAliases,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AuthorizedAliases",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							196u8, 235u8, 1u8, 151u8, 33u8, 20u8, 11u8, 160u8, 66u8, 155u8, 246u8,
							230u8, 229u8, 13u8, 64u8, 50u8, 170u8, 206u8, 163u8, 3u8, 18u8, 196u8,
							198u8, 95u8, 145u8, 100u8, 65u8, 108u8, 129u8, 126u8, 50u8, 14u8,
						],
					)
				}
				#[doc = " Map of authorized aliasers of local origins. Each local location can authorize a list of"]
				#[doc = " other locations to alias into it. Each aliaser is only valid until its inner `expiry`"]
				#[doc = " block number."]
				pub fn authorized_aliases_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_xcm::AuthorizedAliasesEntry<
						(),
						runtime_types::pallet_xcm::pallet::MaxAuthorizedAliases,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"PolkadotXcm",
						"AuthorizedAliases",
						Vec::new(),
						[
							196u8, 235u8, 1u8, 151u8, 33u8, 20u8, 11u8, 160u8, 66u8, 155u8, 246u8,
							230u8, 229u8, 13u8, 64u8, 50u8, 170u8, 206u8, 163u8, 3u8, 18u8, 196u8,
							198u8, 95u8, 145u8, 100u8, 65u8, 108u8, 129u8, 126u8, 50u8, 14u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " This chain's Universal Location."]
				pub fn universal_location(
					&self,
				) -> ::subxt::constants::Address<runtime_types::staging_xcm::v5::junctions::Junctions>
				{
					::subxt::constants::Address::new_static(
						"PolkadotXcm",
						"UniversalLocation",
						[
							241u8, 114u8, 225u8, 116u8, 227u8, 77u8, 161u8, 134u8, 134u8, 121u8,
							72u8, 16u8, 209u8, 208u8, 114u8, 110u8, 163u8, 156u8, 92u8, 109u8,
							134u8, 194u8, 160u8, 228u8, 126u8, 244u8, 254u8, 171u8, 162u8, 58u8,
							216u8, 63u8,
						],
					)
				}
				#[doc = " The latest supported version that we advertise. Generally just set it to"]
				#[doc = " `pallet_xcm::CurrentXcmVersion`."]
				pub fn advertised_xcm_version(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"PolkadotXcm",
						"AdvertisedXcmVersion",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of local XCM locks that a single account may have."]
				pub fn max_lockers(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"PolkadotXcm",
						"MaxLockers",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of consumers a single remote lock may have."]
				pub fn max_remote_lock_consumers(
					&self,
				) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"PolkadotXcm",
						"MaxRemoteLockConsumers",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod cumulus_xcm {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::cumulus_pallet_xcm::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
			}
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Downward message is invalid XCM."]
			#[doc = "\\[ id \\]"]
			pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Downward message is unsupported version of XCM."]
			#[doc = "\\[ id \\]"]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Downward message executed with the given outcome."]
			#[doc = "\\[ id, outcome \\]"]
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 32usize],
				pub runtime_types::staging_xcm::v5::traits::Outcome,
			);
			impl ::subxt::events::StaticEvent for ExecutedDownward {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "ExecutedDownward";
			}
		}
	}
	pub mod message_queue {
		use super::root_mod;
		use super::runtime_types;
		#[doc = "The `Error` enum of this pallet."]
		pub type Error = runtime_types::pallet_message_queue::pallet::Error;
		#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
		pub type Call = runtime_types::pallet_message_queue::pallet::Call;
		pub mod calls {
			use super::root_mod;
			use super::runtime_types;
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReapPage {
					pub message_origin:
						runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					pub page_index: ::core::primitive::u32,
				}
				impl ::subxt::blocks::StaticExtrinsic for ReapPage {
					const PALLET: &'static str = "MessageQueue";
					const CALL: &'static str = "reap_page";
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExecuteOverweight {
					pub message_origin:
						runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					pub page: ::core::primitive::u32,
					pub index: ::core::primitive::u32,
					pub weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				}
				impl ::subxt::blocks::StaticExtrinsic for ExecuteOverweight {
					const PALLET: &'static str = "MessageQueue";
					const CALL: &'static str = "execute_overweight";
				}
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Remove a page which has no more messages remaining to be processed or is stale."]
				pub fn reap_page(
					&self,
					message_origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					page_index: ::core::primitive::u32,
				) -> ::subxt::tx::Payload<types::ReapPage> {
					::subxt::tx::Payload::new_static(
						"MessageQueue",
						"reap_page",
						types::ReapPage { message_origin, page_index },
						[
							116u8, 17u8, 120u8, 238u8, 117u8, 222u8, 10u8, 166u8, 132u8, 181u8,
							114u8, 150u8, 242u8, 202u8, 31u8, 143u8, 212u8, 65u8, 145u8, 249u8,
							27u8, 204u8, 137u8, 133u8, 220u8, 187u8, 137u8, 90u8, 112u8, 55u8,
							104u8, 163u8,
						],
					)
				}
				#[doc = "Execute an overweight message."]
				#[doc = ""]
				#[doc = "Temporary processing errors will be propagated whereas permanent errors are treated"]
				#[doc = "as success condition."]
				#[doc = ""]
				#[doc = "- `origin`: Must be `Signed`."]
				#[doc = "- `message_origin`: The origin from which the message to be executed arrived."]
				#[doc = "- `page`: The page in the queue in which the message to be executed is sitting."]
				#[doc = "- `index`: The index into the queue of the message to be executed."]
				#[doc = "- `weight_limit`: The maximum amount of weight allowed to be consumed in the execution"]
				#[doc = "  of the message."]
				#[doc = ""]
				#[doc = "Benchmark complexity considerations: O(index + weight_limit)."]
				pub fn execute_overweight(
					&self,
					message_origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					page: ::core::primitive::u32,
					index: ::core::primitive::u32,
					weight_limit: runtime_types::sp_weights::weight_v2::Weight,
				) -> ::subxt::tx::Payload<types::ExecuteOverweight> {
					::subxt::tx::Payload::new_static(
						"MessageQueue",
						"execute_overweight",
						types::ExecuteOverweight { message_origin, page, index, weight_limit },
						[
							177u8, 54u8, 82u8, 58u8, 94u8, 125u8, 241u8, 172u8, 52u8, 7u8, 236u8,
							80u8, 66u8, 99u8, 42u8, 199u8, 38u8, 195u8, 65u8, 118u8, 166u8, 246u8,
							239u8, 195u8, 144u8, 153u8, 155u8, 8u8, 224u8, 56u8, 106u8, 135u8,
						],
					)
				}
			}
		}
		#[doc = "The `Event` enum of this pallet"]
		pub type Event = runtime_types::pallet_message_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Message discarded due to an error in the `MessageProcessor` (usually a format error)."]
			pub struct ProcessingFailed {
				pub id: ::subxt::utils::H256,
				pub origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
				pub error: runtime_types::frame_support::traits::messages::ProcessMessageError,
			}
			impl ::subxt::events::StaticEvent for ProcessingFailed {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "ProcessingFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Message is processed."]
			pub struct Processed {
				pub id: ::subxt::utils::H256,
				pub origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
				pub weight_used: runtime_types::sp_weights::weight_v2::Weight,
				pub success: ::core::primitive::bool,
			}
			impl ::subxt::events::StaticEvent for Processed {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "Processed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "Message placed in overweight queue."]
			pub struct OverweightEnqueued {
				pub id: [::core::primitive::u8; 32usize],
				pub origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
				pub page_index: ::core::primitive::u32,
				pub message_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			#[doc = "This page was reaped."]
			pub struct PageReaped {
				pub origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for PageReaped {
				const PALLET: &'static str = "MessageQueue";
				const EVENT: &'static str = "PageReaped";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The index of the first and last (non-empty) pages."]
				pub fn book_state_for(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_message_queue::BookState<
						runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"MessageQueue",
						"BookStateFor",
						vec![::subxt::storage::address::make_static_storage_map_key(_0.borrow())],
						[
							33u8, 240u8, 235u8, 59u8, 150u8, 42u8, 91u8, 248u8, 235u8, 52u8, 170u8,
							52u8, 195u8, 129u8, 6u8, 174u8, 57u8, 242u8, 30u8, 220u8, 232u8, 4u8,
							246u8, 218u8, 162u8, 174u8, 102u8, 95u8, 210u8, 92u8, 133u8, 143u8,
						],
					)
				}
				#[doc = " The index of the first and last (non-empty) pages."]
				pub fn book_state_for_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_message_queue::BookState<
						runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"MessageQueue",
						"BookStateFor",
						Vec::new(),
						[
							33u8, 240u8, 235u8, 59u8, 150u8, 42u8, 91u8, 248u8, 235u8, 52u8, 170u8,
							52u8, 195u8, 129u8, 6u8, 174u8, 57u8, 242u8, 30u8, 220u8, 232u8, 4u8,
							246u8, 218u8, 162u8, 174u8, 102u8, 95u8, 210u8, 92u8, 133u8, 143u8,
						],
					)
				}
				#[doc = " The origin at which we should begin servicing."]
				pub fn service_head(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::Address::new_static(
						"MessageQueue",
						"ServiceHead",
						vec![],
						[
							104u8, 146u8, 240u8, 41u8, 171u8, 68u8, 20u8, 147u8, 212u8, 155u8,
							59u8, 39u8, 174u8, 186u8, 97u8, 250u8, 41u8, 247u8, 67u8, 190u8, 252u8,
							167u8, 234u8, 36u8, 124u8, 239u8, 163u8, 72u8, 223u8, 82u8, 82u8,
							171u8,
						],
					)
				}
				#[doc = " The map of page indices to pages."]
				pub fn pages(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
					>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_message_queue::Page<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"MessageQueue",
						"Pages",
						vec![
							::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
							::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
						],
						[
							45u8, 202u8, 18u8, 128u8, 31u8, 194u8, 175u8, 173u8, 99u8, 81u8, 161u8,
							44u8, 32u8, 183u8, 238u8, 181u8, 110u8, 240u8, 203u8, 12u8, 152u8,
							58u8, 239u8, 190u8, 144u8, 168u8, 210u8, 33u8, 121u8, 250u8, 137u8,
							142u8,
						],
					)
				}
				#[doc = " The map of page indices to pages."]
				pub fn pages_root(
					&self,
				) -> ::subxt::storage::address::Address<
					::subxt::storage::address::StaticStorageMapKey,
					runtime_types::pallet_message_queue::Page<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::Address::new_static(
						"MessageQueue",
						"Pages",
						Vec::new(),
						[
							45u8, 202u8, 18u8, 128u8, 31u8, 194u8, 175u8, 173u8, 99u8, 81u8, 161u8,
							44u8, 32u8, 183u8, 238u8, 181u8, 110u8, 240u8, 203u8, 12u8, 152u8,
							58u8, 239u8, 190u8, 144u8, 168u8, 210u8, 33u8, 121u8, 250u8, 137u8,
							142u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The size of the page; this implies the maximum message size which can be sent."]
				#[doc = ""]
				#[doc = " A good value depends on the expected message sizes, their weights, the weight that is"]
				#[doc = " available for processing them and the maximal needed message size. The maximal message"]
				#[doc = " size is slightly lower than this as defined by [`MaxMessageLenOf`]."]
				pub fn heap_size(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MessageQueue",
						"HeapSize",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of stale pages (i.e. of overweight messages) allowed before culling"]
				#[doc = " can happen. Once there are more stale pages than this, then historical pages may be"]
				#[doc = " dropped, even if they contain unprocessed overweight messages."]
				pub fn max_stale(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
					::subxt::constants::Address::new_static(
						"MessageQueue",
						"MaxStale",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The amount of weight (if any) which should be provided to the message queue for"]
				#[doc = " servicing enqueued items `on_initialize`."]
				#[doc = ""]
				#[doc = " This may be legitimately `None` in the case that you will call"]
				#[doc = " `ServiceQueues::service_queues` manually or set [`Self::IdleMaxServiceWeight`] to have"]
				#[doc = " it run in `on_idle`."]
				pub fn service_weight(
					&self,
				) -> ::subxt::constants::Address<
					::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				> {
					::subxt::constants::Address::new_static(
						"MessageQueue",
						"ServiceWeight",
						[
							204u8, 140u8, 63u8, 167u8, 49u8, 8u8, 148u8, 163u8, 190u8, 224u8, 15u8,
							103u8, 86u8, 153u8, 248u8, 117u8, 223u8, 117u8, 210u8, 80u8, 205u8,
							155u8, 40u8, 11u8, 59u8, 63u8, 129u8, 156u8, 17u8, 83u8, 177u8, 250u8,
						],
					)
				}
				#[doc = " The maximum amount of weight (if any) to be used from remaining weight `on_idle` which"]
				#[doc = " should be provided to the message queue for servicing enqueued items `on_idle`."]
				#[doc = " Useful for parachains to process messages at the same block they are received."]
				#[doc = ""]
				#[doc = " If `None`, it will not call `ServiceQueues::service_queues` in `on_idle`."]
				pub fn idle_max_service_weight(
					&self,
				) -> ::subxt::constants::Address<
					::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				> {
					::subxt::constants::Address::new_static(
						"MessageQueue",
						"IdleMaxServiceWeight",
						[
							204u8, 140u8, 63u8, 167u8, 49u8, 8u8, 148u8, 163u8, 190u8, 224u8, 15u8,
							103u8, 86u8, 153u8, 248u8, 117u8, 223u8, 117u8, 210u8, 80u8, 205u8,
							155u8, 40u8, 11u8, 59u8, 63u8, 129u8, 156u8, 17u8, 83u8, 177u8, 250u8,
						],
					)
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod bounded_collections {
			use super::runtime_types;
			pub mod bounded_btree_set {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedBTreeSet<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
			pub mod weak_bounded_vec {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
			}
		}
		pub mod cumulus_pallet_parachain_system {
			use super::runtime_types;
			pub mod block_weight {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum BlockWeightMode {
					#[codec(index = 0)]
					FullCore { context: ::core::primitive::u32 },
					#[codec(index = 1)]
					PotentialFullCore {
						context: ::core::primitive::u32,
						first_transaction_index: ::core::option::Option<::core::primitive::u32>,
						target_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					FractionOfCore {
						context: ::core::primitive::u32,
						first_transaction_index: ::core::option::Option<::core::primitive::u32>,
					},
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					# [codec (index = 0)] # [doc = "Set the current validation data."] # [doc = ""] # [doc = "This should be invoked exactly once per block. It will panic at the finalization"] # [doc = "phase if the call was not invoked."] # [doc = ""] # [doc = "The dispatch origin for this call must be `Inherent`"] # [doc = ""] # [doc = "As a side effect, this function upgrades the current validation function"] # [doc = "if the appropriate time has come."] set_validation_data { data : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: BasicParachainInherentData , inbound_messages_data : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: InboundMessagesData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to upgrade validation function while existing upgrade pending."]
					OverlappingUpgrades,
					#[codec(index = 1)]
					#[doc = "Polkadot currently prohibits this parachain from upgrading its validation function."]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					#[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
					#[doc = "willing to run."]
					TooBig,
					#[codec(index = 3)]
					#[doc = "The inherent which supplies the validation data did not run this block."]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					#[doc = "The inherent which supplies the host configuration did not run this block."]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					#[doc = "No validation function upgrade is currently scheduled."]
					NotScheduled,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The validation function has been scheduled to apply."]
					ValidationFunctionStored,
					#[codec(index = 1)]
					#[doc = "The validation function was applied as of the contained relay chain block number."]
					ValidationFunctionApplied { relay_chain_block_num: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "The relay-chain aborted the upgrade process."]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					#[doc = "Some downward messages have been received and will be processed."]
					DownwardMessagesReceived { count: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Downward messages were processed using the given weight."]
					DownwardMessagesProcessed {
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						dmq_head: ::subxt::utils::H256,
					},
					#[codec(index = 5)]
					#[doc = "An upward message was sent to the relay chain."]
					UpwardMessageSent {
						message_hash: ::core::option::Option<[::core::primitive::u8; 32usize]>,
					},
				}
			}
			pub mod parachain_inherent {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AbridgedInboundMessagesCollection<_0> {
					pub full_messages: ::std::vec::Vec<_0>,
					pub hashed_messages: ::std::vec::Vec<(
						runtime_types::polkadot_parachain_primitives::primitives::Id,
						runtime_types::cumulus_primitives_parachain_inherent::HashedMessage,
					)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BasicParachainInherentData {
					pub validation_data:
						runtime_types::polkadot_primitives::v9::PersistedValidationData<
							::subxt::utils::H256,
							::core::primitive::u32,
						>,
					pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
					pub relay_parent_descendants: ::std::vec::Vec<
						runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>,
					>,
					pub collator_peer_id: ::core::option::Option<
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct InboundMessageId {
					pub sent_at: ::core::primitive::u32,
					pub reverse_idx: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct InboundMessagesData { pub downward_messages : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: AbridgedInboundMessagesCollection < runtime_types :: polkadot_core_primitives :: InboundDownwardMessage < :: core :: primitive :: u32 > > , pub horizontal_messages : runtime_types :: cumulus_pallet_parachain_system :: parachain_inherent :: AbridgedInboundMessagesCollection < (runtime_types :: polkadot_parachain_primitives :: primitives :: Id , runtime_types :: polkadot_core_primitives :: InboundHrmpMessage < :: core :: primitive :: u32 > ,) > , }
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MessagingStateSnapshot { pub dmq_mqc_head : :: subxt :: utils :: H256 , pub relay_dispatch_queue_remaining_capacity : runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: RelayDispatchQueueRemainingCapacity , pub ingress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain_primitives :: primitives :: Id , runtime_types :: polkadot_primitives :: v9 :: AbridgedHrmpChannel ,) > , pub egress_channels : :: std :: vec :: Vec < (runtime_types :: polkadot_parachain_primitives :: primitives :: Id , runtime_types :: polkadot_primitives :: v9 :: AbridgedHrmpChannel ,) > , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RelayDispatchQueueRemainingCapacity {
					pub remaining_count: ::core::primitive::u32,
					pub remaining_size: ::core::primitive::u32,
				}
			}
			pub mod unincluded_segment {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Ancestor < _0 > { pub used_bandwidth : runtime_types :: cumulus_pallet_parachain_system :: unincluded_segment :: UsedBandwidth , pub para_head_hash : :: core :: option :: Option < _0 > , pub consumed_go_ahead_signal : :: core :: option :: Option < runtime_types :: polkadot_primitives :: v9 :: UpgradeGoAhead > , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HrmpChannelUpdate {
					pub msg_count: ::core::primitive::u32,
					pub total_bytes: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct SegmentTracker < _0 > { pub used_bandwidth : runtime_types :: cumulus_pallet_parachain_system :: unincluded_segment :: UsedBandwidth , pub hrmp_watermark : :: core :: option :: Option < :: core :: primitive :: u32 > , pub consumed_go_ahead_signal : :: core :: option :: Option < runtime_types :: polkadot_primitives :: v9 :: UpgradeGoAhead > , # [codec (skip)] pub __subxt_unused_type_params : :: core :: marker :: PhantomData < _0 > }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct UsedBandwidth { pub ump_msg_count : :: core :: primitive :: u32 , pub ump_total_bytes : :: core :: primitive :: u32 , pub hrmp_outgoing : :: subxt :: utils :: KeyedVec < runtime_types :: polkadot_parachain_primitives :: primitives :: Id , runtime_types :: cumulus_pallet_parachain_system :: unincluded_segment :: HrmpChannelUpdate > , }
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PoVMessages {
				pub relay_storage_root_or_hash: ::subxt::utils::H256,
				pub core_selector: ::core::primitive::u8,
				pub bundle_index: ::core::primitive::u8,
				pub ump_msg_count: ::core::primitive::u32,
				pub hrmp_outbound_count: ::core::primitive::u32,
				pub hrmp_outbound_recipients:
					::std::vec::Vec<runtime_types::polkadot_parachain_primitives::primitives::Id>,
			}
		}
		pub mod cumulus_pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					#[doc = "\\[ id \\]"]
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					#[doc = "\\[ id \\]"]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					#[doc = "\\[ id, outcome \\]"]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::staging_xcm::v5::traits::Outcome,
					),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 1)]
					#[doc = "Suspends all XCM executions for the XCMP queue, regardless of the sender's origin."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					suspend_xcm_execution,
					#[codec(index = 2)]
					#[doc = "Resumes all XCM executions for the XCMP queue."]
					#[doc = ""]
					#[doc = "Note that this function doesn't change the status of the in/out bound channels."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ControllerOrigin`."]
					resume_xcm_execution,
					#[codec(index = 3)]
					#[doc = "Overwrites the number of pages which must be in the queue for the other side to be"]
					#[doc = "told to suspend their sending."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.suspend_value`"]
					update_suspend_threshold { new: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Overwrites the number of pages which must be in the queue after which we drop any"]
					#[doc = "further messages from the channel."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.drop_threshold`"]
					update_drop_threshold { new: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "Overwrites the number of pages which the queue must be reduced to before it signals"]
					#[doc = "that message sending may recommence after it has been suspended."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `Root`."]
					#[doc = "- `new`: Desired value for `QueueConfigData.resume_threshold`"]
					update_resume_threshold { new: ::core::primitive::u32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Setting the queue config failed since one of its values was invalid."]
					BadQueueConfig,
					#[codec(index = 1)]
					#[doc = "The execution is already suspended."]
					AlreadySuspended,
					#[codec(index = 2)]
					#[doc = "The execution is already resumed."]
					AlreadyResumed,
					#[codec(index = 3)]
					#[doc = "There are too many active outbound channels."]
					TooManyActiveOutboundChannels,
					#[codec(index = 4)]
					#[doc = "The message is too big."]
					TooBig,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An HRMP message was sent to a sibling parachain."]
					XcmpMessageSent { message_hash: [::core::primitive::u8; 32usize] },
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OutboundChannelDetails {
				pub recipient: runtime_types::polkadot_parachain_primitives::primitives::Id,
				pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
				pub signals_exist: ::core::primitive::bool,
				pub first_index: ::core::primitive::u16,
				pub last_index: ::core::primitive::u16,
				pub flags: runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelFlags,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OutboundChannelFlags {
				pub bits: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum OutboundState {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
			}
		}
		pub mod cumulus_primitives_core {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum AggregateMessageOrigin {
				#[codec(index = 0)]
				Here,
				#[codec(index = 1)]
				Parent,
				#[codec(index = 2)]
				Sibling(runtime_types::polkadot_parachain_primitives::primitives::Id),
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct HashedMessage {
				pub sent_at: ::core::primitive::u32,
				pub msg_hash: ::subxt::utils::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct MessageQueueChain(pub ::subxt::utils::H256);
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod messages {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum ProcessMessageError {
						#[codec(index = 0)]
						BadFormat,
						#[codec(index = 1)]
						Corrupt,
						#[codec(index = 2)]
						Unsupported,
						#[codec(index = 3)]
						Overweight(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 4)]
						Yield,
						#[codec(index = 5)]
						StackLimitReached,
					}
				}
				pub mod storage {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct NoDrop<_0>(pub _0);
				}
				pub mod tokens {
					use super::runtime_types;
					pub mod fungible {
						use super::runtime_types;
						pub mod imbalance {
							use super::runtime_types;
							#[derive(
								:: subxt :: ext :: codec :: Decode,
								:: subxt :: ext :: codec :: Encode,
								:: subxt :: ext :: scale_decode :: DecodeAsType,
								:: subxt :: ext :: scale_encode :: EncodeAsType,
								Debug,
							)]
							# [codec (crate = :: subxt :: ext :: codec)]
							#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
							#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
							pub struct Imbalance<_0> {
								pub amount: _0,
							}
						}
					}
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
						#[derive(
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							:: subxt :: ext :: scale_decode :: DecodeAsType,
							:: subxt :: ext :: scale_encode :: EncodeAsType,
							Debug,
						)]
						# [codec (crate = :: subxt :: ext :: codec)]
						#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
						#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
						pub struct IdAmount<_0, _1> {
							pub id: _0,
							pub amount: _1,
						}
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
						::core::primitive::u32,
					>,
					pub max_header_size: ::core::option::Option<::core::primitive::u32>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BlockWeights {
					pub base_block: runtime_types::sp_weights::weight_v2::Weight,
					pub max_block: runtime_types::sp_weights::weight_v2::Weight,
					pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct WeightsPerClass {
					pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
					pub max_extrinsic:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub max_total:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
					pub reserved:
						::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Make some on-chain remark."]
					#[doc = ""]
					#[doc = "Can be executed by every `origin`."]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 1)]
					#[doc = "Set the number of pages in the WebAssembly environment's heap."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 2)]
					#[doc = "Set the new runtime code."]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 3)]
					#[doc = "Set the new runtime code without doing any checks of the given `code`."]
					#[doc = ""]
					#[doc = "Note that runtime upgrades will not run if this is called with a not-increasing spec"]
					#[doc = "version!"]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					#[doc = "Set some items of storage."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 5)]
					#[doc = "Kill some items from storage."]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 6)]
					#[doc = "Kill all storage items with a key that starts with the given prefix."]
					#[doc = ""]
					#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
					#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 7)]
					#[doc = "Make some on-chain remark and emit event."]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 9)]
					#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
					#[doc = "later."]
					#[doc = ""]
					#[doc = "This call requires Root origin."]
					authorize_upgrade { code_hash: ::subxt::utils::H256 },
					#[codec(index = 10)]
					#[doc = "Authorize an upgrade to a given `code_hash` for the runtime. The runtime can be supplied"]
					#[doc = "later."]
					#[doc = ""]
					#[doc = "WARNING: This authorizes an upgrade that will take place without any safety checks, for"]
					#[doc = "example that the spec name remains the same and that the version number increases. Not"]
					#[doc = "recommended for normal use. Use `authorize_upgrade` instead."]
					#[doc = ""]
					#[doc = "This call requires Root origin."]
					authorize_upgrade_without_checks { code_hash: ::subxt::utils::H256 },
					#[codec(index = 11)]
					#[doc = "Provide the preimage (runtime binary) `code` for an upgrade that has been authorized."]
					#[doc = ""]
					#[doc = "If the authorization required a version check, this call will ensure the spec name"]
					#[doc = "remains unchanged and that the spec version has increased."]
					#[doc = ""]
					#[doc = "Depending on the runtime's `OnSetCode` configuration, this function may directly apply"]
					#[doc = "the new `code` in the same block or attempt to schedule the upgrade."]
					#[doc = ""]
					#[doc = "All origins are allowed."]
					apply_authorized_upgrade { code: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the System pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
					#[codec(index = 6)]
					#[doc = "A multi-block migration is ongoing and prevents the current code from being replaced."]
					MultiBlockMigrationsOngoing,
					#[codec(index = 7)]
					#[doc = "No upgrade authorized."]
					NothingAuthorized,
					#[codec(index = 8)]
					#[doc = "The submitted code is not authorized."]
					Unauthorized,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_system::DispatchEventInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_system::DispatchEventInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated to the code with the given hash."]
					CodeUpdated { hash: ::subxt::utils::H256 },
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount { account: ::subxt::utils::AccountId32 },
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked { sender: ::subxt::utils::AccountId32, hash: ::subxt::utils::H256 },
					#[codec(index = 6)]
					#[doc = "An upgrade was authorized."]
					UpgradeAuthorized {
						code_hash: ::subxt::utils::H256,
						check_version: ::core::primitive::bool,
					},
					#[codec(index = 7)]
					#[doc = "An invalid authorized upgrade was rejected while trying to apply it."]
					RejectedInvalidAuthorizedUpgrade {
						code_hash: ::subxt::utils::H256,
						error: runtime_types::sp_runtime::DispatchError,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: ::core::primitive::u32,
				pub providers: ::core::primitive::u32,
				pub sufficients: ::core::primitive::u32,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct CodeUpgradeAuthorization {
				pub code_hash: ::subxt::utils::H256,
				pub check_version: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct DispatchEventInfo {
				pub weight: runtime_types::sp_weights::weight_v2::Weight,
				pub class: runtime_types::frame_support::dispatch::DispatchClass,
				pub pays_fee: runtime_types::frame_support::dispatch::Pays,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					transfer_allow_death {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
					#[doc = "may be specified."]
					force_transfer {
						source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
					#[doc = "kill the origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer_allow_death`] instead."]
					#[doc = ""]
					#[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
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
					#[doc = "  keep the sender account alive (true)."]
					transfer_all {
						dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Upgrade a specified account."]
					#[doc = ""]
					#[doc = "- `origin`: Must be `Signed`."]
					#[doc = "- `who`: The account to be upgraded."]
					#[doc = ""]
					#[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
					#[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
					#[doc = "possibility of churn)."]
					upgrade_accounts { who: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 8)]
					#[doc = "Set the regular balance of a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					force_set_balance {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Adjust the total issuance in a saturating way."]
					#[doc = ""]
					#[doc = "Can only be called by root and always needs a positive `delta`."]
					#[doc = ""]
					#[doc = "# Example"]
					force_adjust_total_issuance {
						direction: runtime_types::pallet_balances::types::AdjustmentDirection,
						#[codec(compact)]
						delta: ::core::primitive::u128,
					},
					#[codec(index = 10)]
					#[doc = "Burn the specified liquid free balance from the origin account."]
					#[doc = ""]
					#[doc = "If the origin's account ends up below the existential deposit as a result"]
					#[doc = "of the burn and `keep_alive` is false, the account will be reaped."]
					#[doc = ""]
					#[doc = "Unlike sending funds to a _burn_ address, which merely makes the funds inaccessible,"]
					#[doc = "this `burn` operation will reduce total issuance by the amount _burned_."]
					burn {
						#[codec(compact)]
						value: ::core::primitive::u128,
						keep_alive: ::core::primitive::bool,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value."]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal."]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value."]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit."]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account."]
					Expendability,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account."]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist."]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed `MaxReserves`."]
					TooManyReserves,
					#[codec(index = 8)]
					#[doc = "Number of holds exceed `VariantCountOf<T::RuntimeHoldReason>`."]
					TooManyHolds,
					#[codec(index = 9)]
					#[doc = "Number of freezes exceed `MaxFreezes`."]
					TooManyFreezes,
					#[codec(index = 10)]
					#[doc = "The issuance cannot be modified since it is already deactivated."]
					IssuanceDeactivated,
					#[codec(index = 11)]
					#[doc = "The delta cannot be zero."]
					DeltaZero,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::utils::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet { who: ::subxt::utils::AccountId32, free: ::core::primitive::u128 },
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::utils::AccountId32,
						to: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 10)]
					#[doc = "Some amount was minted into an account."]
					Minted { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 11)]
					#[doc = "Some credit was balanced and added to the TotalIssuance."]
					MintedCredit { amount: ::core::primitive::u128 },
					#[codec(index = 12)]
					#[doc = "Some amount was burned from an account."]
					Burned { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 13)]
					#[doc = "Some debt has been dropped from the Total Issuance."]
					BurnedDebt { amount: ::core::primitive::u128 },
					#[codec(index = 14)]
					#[doc = "Some amount was suspended from an account (it can be restored later)."]
					Suspended { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 15)]
					#[doc = "Some amount was restored into an account."]
					Restored { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 16)]
					#[doc = "An account was upgraded."]
					Upgraded { who: ::subxt::utils::AccountId32 },
					#[codec(index = 17)]
					#[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
					Issued { amount: ::core::primitive::u128 },
					#[codec(index = 18)]
					#[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
					Rescinded { amount: ::core::primitive::u128 },
					#[codec(index = 19)]
					#[doc = "Some balance was locked."]
					Locked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 20)]
					#[doc = "Some balance was unlocked."]
					Unlocked { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 21)]
					#[doc = "Some balance was frozen."]
					Frozen { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 22)]
					#[doc = "Some balance was thawed."]
					Thawed { who: ::subxt::utils::AccountId32, amount: ::core::primitive::u128 },
					#[codec(index = 23)]
					#[doc = "The `TotalIssuance` was forcefully changed."]
					TotalIssuanceForced {
						old: ::core::primitive::u128,
						new: ::core::primitive::u128,
					},
					#[codec(index = 24)]
					#[doc = "Some balance was placed on hold."]
					Held {
						reason: runtime_types::parachain_runtime::RuntimeHoldReason,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 25)]
					#[doc = "Held balance was burned from an account."]
					BurnedHeld {
						reason: runtime_types::parachain_runtime::RuntimeHoldReason,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 26)]
					#[doc = "A transfer of `amount` on hold from `source` to `dest` was initiated."]
					TransferOnHold {
						reason: runtime_types::parachain_runtime::RuntimeHoldReason,
						source: ::subxt::utils::AccountId32,
						dest: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 27)]
					#[doc = "The `transferred` balance is placed on hold at the `dest` account."]
					TransferAndHold {
						reason: runtime_types::parachain_runtime::RuntimeHoldReason,
						source: ::subxt::utils::AccountId32,
						dest: ::subxt::utils::AccountId32,
						transferred: ::core::primitive::u128,
					},
					#[codec(index = 28)]
					#[doc = "Some balance was released from hold."]
					Released {
						reason: runtime_types::parachain_runtime::RuntimeHoldReason,
						who: ::subxt::utils::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 29)]
					#[doc = "An unexpected/defensive event was triggered."]
					Unexpected(runtime_types::pallet_balances::pallet::UnexpectedKind),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UnexpectedKind {
					#[codec(index = 0)]
					BalanceUpdated,
					#[codec(index = 1)]
					FailedToMutateAccount,
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AccountData<_0> {
					pub free: _0,
					pub reserved: _0,
					pub frozen: _0,
					pub flags: runtime_types::pallet_balances::types::ExtraFlags,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum AdjustmentDirection {
					#[codec(index = 0)]
					Increase,
					#[codec(index = 1)]
					Decrease,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct BalanceLock<_0> {
					pub id: [::core::primitive::u8; 8usize],
					pub amount: _0,
					pub reasons: runtime_types::pallet_balances::types::Reasons,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ExtraFlags(pub ::core::primitive::u128);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Reasons {
					#[codec(index = 0)]
					Fee,
					#[codec(index = 1)]
					Misc,
					#[codec(index = 2)]
					All,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct ReserveData<_0, _1> {
					pub id: _0,
					pub amount: _1,
				}
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the list of invulnerable (fixed) collators. These collators must do some"]
					#[doc = "preparation, namely to have registered session keys."]
					#[doc = ""]
					#[doc = "The call will remove any accounts that have not registered keys from the set. That is,"]
					#[doc = "it is non-atomic; the caller accepts all `AccountId`s passed in `new` _individually_ as"]
					#[doc = "acceptable Invulnerables, and is not proposing a _set_ of new Invulnerables."]
					#[doc = ""]
					#[doc = "This call does not maintain mutual exclusivity of `Invulnerables` and `Candidates`. It"]
					#[doc = "is recommended to use a batch of `add_invulnerable` and `remove_invulnerable` instead. A"]
					#[doc = "`batch_all` can also be used to enforce atomicity. If any candidates are included in"]
					#[doc = "`new`, they should be removed with `remove_invulnerable_candidate` after execution."]
					#[doc = ""]
					#[doc = "Must be called by the `UpdateOrigin`."]
					set_invulnerables { new: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					#[doc = "Set the ideal number of non-invulnerable collators. If lowering this number, then the"]
					#[doc = "number of running collators could be higher than this figure. Aside from that edge case,"]
					#[doc = "there should be no other way to have more candidates than the desired number."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Set the candidacy bond amount."]
					#[doc = ""]
					#[doc = "If the candidacy bond is increased by this call, all current candidates which have a"]
					#[doc = "deposit lower than the new bond will be kicked from the list and get their deposits"]
					#[doc = "back."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec(index = 3)]
					#[doc = "Register this account as a collator candidate. The account must (a) already have"]
					#[doc = "registered session keys and (b) be able to reserve the `CandidacyBond`."]
					#[doc = ""]
					#[doc = "This call is not available to `Invulnerable` collators."]
					register_as_candidate,
					#[codec(index = 4)]
					#[doc = "Deregister `origin` as a collator candidate. Note that the collator can only leave on"]
					#[doc = "session change. The `CandidacyBond` will be unreserved immediately."]
					#[doc = ""]
					#[doc = "This call will fail if the total number of candidates would drop below"]
					#[doc = "`MinEligibleCollators`."]
					leave_intent,
					#[codec(index = 5)]
					#[doc = "Add a new account `who` to the list of `Invulnerables` collators. `who` must have"]
					#[doc = "registered session keys. If `who` is a candidate, they will be removed."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					add_invulnerable { who: ::subxt::utils::AccountId32 },
					#[codec(index = 6)]
					#[doc = "Remove an account `who` from the list of `Invulnerables` collators. `Invulnerables` must"]
					#[doc = "be sorted."]
					#[doc = ""]
					#[doc = "The origin for this call must be the `UpdateOrigin`."]
					remove_invulnerable { who: ::subxt::utils::AccountId32 },
					#[codec(index = 7)]
					#[doc = "Update the candidacy bond of collator candidate `origin` to a new amount `new_deposit`."]
					#[doc = ""]
					#[doc = "Setting a `new_deposit` that is lower than the current deposit while `origin` is"]
					#[doc = "occupying a top-`DesiredCandidates` slot is not allowed."]
					#[doc = ""]
					#[doc = "This call will fail if `origin` is not a collator candidate, the updated bond is lower"]
					#[doc = "than the minimum candidacy bond, and/or the amount cannot be reserved."]
					update_bond { new_deposit: ::core::primitive::u128 },
					#[codec(index = 8)]
					#[doc = "The caller `origin` replaces a candidate `target` in the collator candidate list by"]
					#[doc = "reserving `deposit`. The amount `deposit` reserved by the caller must be greater than"]
					#[doc = "the existing bond of the target it is trying to replace."]
					#[doc = ""]
					#[doc = "This call will fail if the caller is already a collator candidate or invulnerable, the"]
					#[doc = "caller does not have registered session keys, the target is not a collator candidate,"]
					#[doc = "and/or the `deposit` amount cannot be reserved."]
					take_candidate_slot {
						deposit: ::core::primitive::u128,
						target: ::subxt::utils::AccountId32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The pallet has too many candidates."]
					TooManyCandidates,
					#[codec(index = 1)]
					#[doc = "Leaving would result in too few candidates."]
					TooFewEligibleCollators,
					#[codec(index = 2)]
					#[doc = "Account is already a candidate."]
					AlreadyCandidate,
					#[codec(index = 3)]
					#[doc = "Account is not a candidate."]
					NotCandidate,
					#[codec(index = 4)]
					#[doc = "There are too many Invulnerables."]
					TooManyInvulnerables,
					#[codec(index = 5)]
					#[doc = "Account is already an Invulnerable."]
					AlreadyInvulnerable,
					#[codec(index = 6)]
					#[doc = "Account is not an Invulnerable."]
					NotInvulnerable,
					#[codec(index = 7)]
					#[doc = "Account has no associated validator ID."]
					NoAssociatedValidatorId,
					#[codec(index = 8)]
					#[doc = "Validator ID is not yet registered."]
					ValidatorNotRegistered,
					#[codec(index = 9)]
					#[doc = "Could not insert in the candidate list."]
					InsertToCandidateListFailed,
					#[codec(index = 10)]
					#[doc = "Could not remove from the candidate list."]
					RemoveFromCandidateListFailed,
					#[codec(index = 11)]
					#[doc = "New deposit amount would be below the minimum candidacy bond."]
					DepositTooLow,
					#[codec(index = 12)]
					#[doc = "Could not update the candidate list."]
					UpdateCandidateListFailed,
					#[codec(index = 13)]
					#[doc = "Deposit amount is too low to take the target's slot in the candidate list."]
					InsufficientBond,
					#[codec(index = 14)]
					#[doc = "The target account to be replaced in the candidate list is not a candidate."]
					TargetIsNotCandidate,
					#[codec(index = 15)]
					#[doc = "The updated deposit amount is equal to the amount already reserved."]
					IdenticalDeposit,
					#[codec(index = 16)]
					#[doc = "Cannot lower candidacy bond while occupying a future collator slot in the list."]
					InvalidUnreserve,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New Invulnerables were set."]
					NewInvulnerables { invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32> },
					#[codec(index = 1)]
					#[doc = "A new Invulnerable was added."]
					InvulnerableAdded { account_id: ::subxt::utils::AccountId32 },
					#[codec(index = 2)]
					#[doc = "An Invulnerable was removed."]
					InvulnerableRemoved { account_id: ::subxt::utils::AccountId32 },
					#[codec(index = 3)]
					#[doc = "The number of desired candidates was set."]
					NewDesiredCandidates { desired_candidates: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "The candidacy bond was set."]
					NewCandidacyBond { bond_amount: ::core::primitive::u128 },
					#[codec(index = 5)]
					#[doc = "A new candidate joined."]
					CandidateAdded {
						account_id: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Bond of a candidate updated."]
					CandidateBondUpdated {
						account_id: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 7)]
					#[doc = "A candidate was removed."]
					CandidateRemoved { account_id: ::subxt::utils::AccountId32 },
					#[codec(index = 8)]
					#[doc = "An account was replaced in the candidate list by another one."]
					CandidateReplaced {
						old: ::subxt::utils::AccountId32,
						new: ::subxt::utils::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "An account was unable to be added to the Invulnerables because they did not have keys"]
					#[doc = "registered. Other Invulnerables may have been set."]
					InvalidInvulnerableSkipped { account_id: ::subxt::utils::AccountId32 },
				}
			}
		}
		pub mod pallet_message_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Remove a page which has no more messages remaining to be processed or is stale."]
					reap_page {
						message_origin:
							runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						page_index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Execute an overweight message."]
					#[doc = ""]
					#[doc = "Temporary processing errors will be propagated whereas permanent errors are treated"]
					#[doc = "as success condition."]
					#[doc = ""]
					#[doc = "- `origin`: Must be `Signed`."]
					#[doc = "- `message_origin`: The origin from which the message to be executed arrived."]
					#[doc = "- `page`: The page in the queue in which the message to be executed is sitting."]
					#[doc = "- `index`: The index into the queue of the message to be executed."]
					#[doc = "- `weight_limit`: The maximum amount of weight allowed to be consumed in the execution"]
					#[doc = "  of the message."]
					#[doc = ""]
					#[doc = "Benchmark complexity considerations: O(index + weight_limit)."]
					execute_overweight {
						message_origin:
							runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						page: ::core::primitive::u32,
						index: ::core::primitive::u32,
						weight_limit: runtime_types::sp_weights::weight_v2::Weight,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Page is not reapable because it has items remaining to be processed and is not old"]
					#[doc = "enough."]
					NotReapable,
					#[codec(index = 1)]
					#[doc = "Page to be reaped does not exist."]
					NoPage,
					#[codec(index = 2)]
					#[doc = "The referenced message could not be found."]
					NoMessage,
					#[codec(index = 3)]
					#[doc = "The message was already processed and cannot be processed again."]
					AlreadyProcessed,
					#[codec(index = 4)]
					#[doc = "The message is queued for future execution."]
					Queued,
					#[codec(index = 5)]
					#[doc = "There is temporarily not enough weight to continue servicing messages."]
					InsufficientWeight,
					#[codec(index = 6)]
					#[doc = "This message is temporarily unprocessable."]
					#[doc = ""]
					#[doc = "Such errors are expected, but not guaranteed, to resolve themselves eventually through"]
					#[doc = "retrying."]
					TemporarilyUnprocessable,
					#[codec(index = 7)]
					#[doc = "The queue is paused and no message can be executed from it."]
					#[doc = ""]
					#[doc = "This can change at any time and may resolve in the future by re-trying."]
					QueuePaused,
					#[codec(index = 8)]
					#[doc = "Another call is in progress and needs to finish before this call can happen."]
					RecursiveDisallowed,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Message discarded due to an error in the `MessageProcessor` (usually a format error)."]
					ProcessingFailed {
						id: ::subxt::utils::H256,
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						error: runtime_types::frame_support::traits::messages::ProcessMessageError,
					},
					#[codec(index = 1)]
					#[doc = "Message is processed."]
					Processed {
						id: ::subxt::utils::H256,
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						weight_used: runtime_types::sp_weights::weight_v2::Weight,
						success: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					#[doc = "Message placed in overweight queue."]
					OverweightEnqueued {
						id: [::core::primitive::u8; 32usize],
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						page_index: ::core::primitive::u32,
						message_index: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "This page was reaped."]
					PageReaped {
						origin: runtime_types::cumulus_primitives_core::AggregateMessageOrigin,
						index: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct BookState<_0> {
				pub begin: ::core::primitive::u32,
				pub end: ::core::primitive::u32,
				pub count: ::core::primitive::u32,
				pub ready_neighbours:
					::core::option::Option<runtime_types::pallet_message_queue::Neighbours<_0>>,
				pub message_count: ::core::primitive::u64,
				pub size: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Neighbours<_0> {
				pub prev: _0,
				pub next: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Page<_0> {
				pub remaining: _0,
				pub remaining_size: _0,
				pub first_index: _0,
				pub first: _0,
				pub last: _0,
				pub heap: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					::core::primitive::u8,
				>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the session key(s) of the function caller to `keys`."]
					#[doc = ""]
					#[doc = "Allows an account to set its session key prior to becoming a validator."]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "- `origin`: The dispatch origin of this function must be signed."]
					#[doc = "- `keys`: The new session keys to set. These are the public keys of all sessions keys"]
					#[doc = "  setup in the runtime."]
					#[doc = "- `proof`: The proof that `origin` has access to the private keys of `keys`. See"]
					#[doc = "  [`impl_opaque_keys`](sp_runtime::impl_opaque_keys) for more information about the"]
					#[doc = "  proof format."]
					set_keys {
						keys: runtime_types::parachain_runtime::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Removes any session key(s) of the function caller."]
					#[doc = ""]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
					#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
					#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
					#[doc = "usually means being a stash account)."]
					purge_keys,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the session pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession { session_index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "The `NewSession` event in the current block also implies a new validator set to be"]
					#[doc = "queued."]
					NewQueued,
					#[codec(index = 2)]
					#[doc = "Validator has been disabled."]
					ValidatorDisabled { validator: ::subxt::utils::AccountId32 },
					#[codec(index = 3)]
					#[doc = "Validator has been re-enabled."]
					ValidatorReenabled { validator: ::subxt::utils::AccountId32 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum HoldReason {
					#[codec(index = 0)]
					Keys,
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					sudo { call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall> },
					#[codec(index = 1)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Sudo user to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
						weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 2)]
					#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
					#[doc = "key."]
					set_key { new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> },
					#[codec(index = 3)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
					#[doc = "a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					sudo_as {
						who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
						call: ::std::boxed::Box<runtime_types::parachain_runtime::RuntimeCall>,
					},
					#[codec(index = 4)]
					#[doc = "Permanently removes the sudo key."]
					#[doc = ""]
					#[doc = "**This cannot be un-done.**"]
					remove_key,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Error for the Sudo pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account."]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo call just took place."]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The sudo key has been updated."]
					KeyChanged {
						old: ::core::option::Option<::subxt::utils::AccountId32>,
						new: ::subxt::utils::AccountId32,
					},
					#[codec(index = 2)]
					#[doc = "The key was permanently removed."]
					KeyRemoved,
					#[codec(index = 3)]
					#[doc = "A [sudo_as](Pallet::sudo_as) call just took place."]
					SudoAsDone {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current time."]
					#[doc = ""]
					#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase, if this call hasn't been invoked by that time."]
					#[doc = ""]
					#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
					#[doc = "[`Config::MinimumPeriod`]."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _None_."]
					#[doc = ""]
					#[doc = "This dispatch class is _Mandatory_ to ensure it gets executed in the block. Be aware"]
					#[doc = "that changing the complexity of this call could result exhausting the resources in a"]
					#[doc = "block to execute any other calls."]
					#[doc = ""]
					#[doc = "## Complexity"]
					#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
					#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)` because of `DidUpdate::take` in"]
					#[doc = "  `on_finalize`)"]
					#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::utils::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod errors {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum ExecutionError {
					#[codec(index = 0)]
					Overflow,
					#[codec(index = 1)]
					Unimplemented,
					#[codec(index = 2)]
					UntrustedReserveLocation,
					#[codec(index = 3)]
					UntrustedTeleportLocation,
					#[codec(index = 4)]
					LocationFull,
					#[codec(index = 5)]
					LocationNotInvertible,
					#[codec(index = 6)]
					BadOrigin,
					#[codec(index = 7)]
					InvalidLocation,
					#[codec(index = 8)]
					AssetNotFound,
					#[codec(index = 9)]
					FailedToTransactAsset,
					#[codec(index = 10)]
					NotWithdrawable,
					#[codec(index = 11)]
					LocationCannotHold,
					#[codec(index = 12)]
					ExceedsMaxMessageSize,
					#[codec(index = 13)]
					DestinationUnsupported,
					#[codec(index = 14)]
					Transport,
					#[codec(index = 15)]
					Unroutable,
					#[codec(index = 16)]
					UnknownClaim,
					#[codec(index = 17)]
					FailedToDecode,
					#[codec(index = 18)]
					MaxWeightInvalid,
					#[codec(index = 19)]
					NotHoldingFees,
					#[codec(index = 20)]
					TooExpensive,
					#[codec(index = 21)]
					Trap,
					#[codec(index = 22)]
					ExpectationFalse,
					#[codec(index = 23)]
					PalletNotFound,
					#[codec(index = 24)]
					NameMismatch,
					#[codec(index = 25)]
					VersionIncompatible,
					#[codec(index = 26)]
					HoldingWouldOverflow,
					#[codec(index = 27)]
					ExportError,
					#[codec(index = 28)]
					ReanchorFailed,
					#[codec(index = 29)]
					NoDeal,
					#[codec(index = 30)]
					FeesNotMet,
					#[codec(index = 31)]
					LockError,
					#[codec(index = 32)]
					NoPermission,
					#[codec(index = 33)]
					Unanchored,
					#[codec(index = 34)]
					NotDepositable,
					#[codec(index = 35)]
					TooManyAssets,
					#[codec(index = 36)]
					UnhandledXcmVersion,
					#[codec(index = 37)]
					WeightLimitReached,
					#[codec(index = 38)]
					Barrier,
					#[codec(index = 39)]
					WeightNotComputable,
					#[codec(index = 40)]
					ExceedsStackLimit,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {
					# [codec (index = 0)] send { dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , message : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedXcm > , } , # [codec (index = 1)] # [doc = "Teleport some assets from the local chain to some destination chain."] # [doc = ""] # [doc = "**This function is deprecated: Use `limited_teleport_assets` instead.**"] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"] # [doc = "with all fees taken as needed from the asset."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` chain."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] teleport_assets { dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , } , # [codec (index = 2)] # [doc = "Transfer some assets from the local chain to the destination chain through their local,"] # [doc = "destination or remote reserve."] # [doc = ""] # [doc = "`assets` must have same reserve location and may not be teleportable to `dest`."] # [doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"] # [doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"] # [doc = "   assets to `beneficiary`."] # [doc = " - `assets` have destination reserve: burn local assets and forward a notification to"] # [doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"] # [doc = "   deposit them to `beneficiary`."] # [doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"] # [doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"] # [doc = "   to mint and deposit reserve-based assets to `beneficiary`."] # [doc = ""] # [doc = "**This function is deprecated: Use `limited_reserve_transfer_assets` instead.**"] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"] # [doc = "with all fees taken as needed from the asset."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] reserve_transfer_assets { dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , } , # [codec (index = 3)] # [doc = "Execute an XCM message from a local, signed, origin."] # [doc = ""] # [doc = "An event is deposited indicating whether `msg` could be executed completely or only"] # [doc = "partially."] # [doc = ""] # [doc = "No more than `max_weight` will be used in its attempted execution. If this is less than"] # [doc = "the maximum amount of weight that the message could take to be executed, then no"] # [doc = "execution attempt will be made."] execute { message : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedXcm2 > , max_weight : runtime_types :: sp_weights :: weight_v2 :: Weight , } , # [codec (index = 4)] # [doc = "Extoll that a particular destination can be communicated with through a particular"] # [doc = "version of XCM."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `location`: The destination that is being described."] # [doc = "- `xcm_version`: The latest version of XCM that `location` supports."] force_xcm_version { location : :: std :: boxed :: Box < runtime_types :: staging_xcm :: v5 :: location :: Location > , version : :: core :: primitive :: u32 , } , # [codec (index = 5)] # [doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"] # [doc = "version a destination can accept is unknown)."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."] force_default_xcm_version { maybe_xcm_version : :: core :: option :: Option < :: core :: primitive :: u32 > , } , # [codec (index = 6)] # [doc = "Ask a location to notify us regarding their XCM version and any changes to it."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `location`: The location to which we should subscribe for XCM version notifications."] force_subscribe_version_notify { location : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 7)] # [doc = "Require that a particular destination should no longer notify us regarding any XCM"] # [doc = "version changes."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `location`: The location to which we are currently subscribed for XCM version"] # [doc = "  notifications which we no longer desire."] force_unsubscribe_version_notify { location : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 8)] # [doc = "Transfer some assets from the local chain to the destination chain through their local,"] # [doc = "destination or remote reserve."] # [doc = ""] # [doc = "`assets` must have same reserve location and may not be teleportable to `dest`."] # [doc = " - `assets` have local reserve: transfer assets to sovereign account of destination"] # [doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"] # [doc = "   assets to `beneficiary`."] # [doc = " - `assets` have destination reserve: burn local assets and forward a notification to"] # [doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"] # [doc = "   deposit them to `beneficiary`."] # [doc = " - `assets` have remote reserve: burn local assets, forward XCM to reserve chain to move"] # [doc = "   reserves from this chain's SA to `dest` chain's SA, and forward another XCM to `dest`"] # [doc = "   to mint and deposit reserve-based assets to `beneficiary`."] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"] # [doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"] # [doc = "at risk."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] limited_reserve_transfer_assets { dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 9)] # [doc = "Teleport some assets from the local chain to some destination chain."] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"] # [doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"] # [doc = "at risk."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` chain."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] limited_teleport_assets { dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 10)] # [doc = "Set or unset the global suspension state of the XCM executor."] # [doc = ""] # [doc = "- `origin`: Must be an origin specified by AdminOrigin."] # [doc = "- `suspended`: `true` to suspend, `false` to resume."] force_suspension { suspended : :: core :: primitive :: bool , } , # [codec (index = 11)] # [doc = "Transfer some assets from the local chain to the destination chain through their local,"] # [doc = "destination or remote reserve, or through teleports."] # [doc = ""] # [doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"] # [doc = "index `fee_asset_item` (hence referred to as `fees`), up to enough to pay for"] # [doc = "`weight_limit` of weight. If more weight is needed than `weight_limit`, then the"] # [doc = "operation will fail and the sent assets may be at risk."] # [doc = ""] # [doc = "`assets` (excluding `fees`) must have same reserve location or otherwise be teleportable"] # [doc = "to `dest`, no limitations imposed on `fees`."] # [doc = " - for local reserve: transfer assets to sovereign account of destination chain and"] # [doc = "   forward a notification XCM to `dest` to mint and deposit reserve-based assets to"] # [doc = "   `beneficiary`."] # [doc = " - for destination reserve: burn local assets and forward a notification to `dest` chain"] # [doc = "   to withdraw the reserve assets from this chain's sovereign account and deposit them"] # [doc = "   to `beneficiary`."] # [doc = " - for remote reserve: burn local assets, forward XCM to reserve chain to move reserves"] # [doc = "   from this chain's SA to `dest` chain's SA, and forward another XCM to `dest` to mint"] # [doc = "   and deposit reserve-based assets to `beneficiary`."] # [doc = " - for teleports: burn local assets and forward XCM to `dest` chain to mint/teleport"] # [doc = "   assets and deposit them to `beneficiary`."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent,"] # [doc = "  Parachain(..))` to send from parachain to parachain, or `X1(Parachain(..))` to send"] # [doc = "  from relay to parachain."] # [doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will"] # [doc = "  generally be an `AccountId32` value."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"] # [doc = "  fees."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] transfer_assets { dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , beneficiary : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , fee_asset_item : :: core :: primitive :: u32 , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 12)] # [doc = "Claims assets trapped on this pallet because of leftover assets during XCM execution."] # [doc = ""] # [doc = "- `origin`: Anyone can call this extrinsic."] # [doc = "- `assets`: The exact assets that were trapped. Use the version to specify what version"] # [doc = "was the latest when they were trapped."] # [doc = "- `beneficiary`: The location/account where the claimed assets will be deposited."] claim_assets { assets : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , beneficiary : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 13)] # [doc = "Transfer assets from the local chain to the destination chain using explicit transfer"] # [doc = "types for assets and fees."] # [doc = ""] # [doc = "`assets` must have same reserve location or may be teleportable to `dest`. Caller must"] # [doc = "provide the `assets_transfer_type` to be used for `assets`:"] # [doc = " - `TransferType::LocalReserve`: transfer assets to sovereign account of destination"] # [doc = "   chain and forward a notification XCM to `dest` to mint and deposit reserve-based"] # [doc = "   assets to `beneficiary`."] # [doc = " - `TransferType::DestinationReserve`: burn local assets and forward a notification to"] # [doc = "   `dest` chain to withdraw the reserve assets from this chain's sovereign account and"] # [doc = "   deposit them to `beneficiary`."] # [doc = " - `TransferType::RemoteReserve(reserve)`: burn local assets, forward XCM to `reserve`"] # [doc = "   chain to move reserves from this chain's SA to `dest` chain's SA, and forward another"] # [doc = "   XCM to `dest` to mint and deposit reserve-based assets to `beneficiary`. Typically"] # [doc = "   the remote `reserve` is Asset Hub."] # [doc = " - `TransferType::Teleport`: burn local assets and forward XCM to `dest` chain to"] # [doc = "   mint/teleport assets and deposit them to `beneficiary`."] # [doc = ""] # [doc = "On the destination chain, as well as any intermediary hops, `BuyExecution` is used to"] # [doc = "buy execution using transferred `assets` identified by `remote_fees_id`."] # [doc = "Make sure enough of the specified `remote_fees_id` asset is included in the given list"] # [doc = "of `assets`. `remote_fees_id` should be enough to pay for `weight_limit`. If more weight"] # [doc = "is needed than `weight_limit`, then the operation will fail and the sent assets may be"] # [doc = "at risk."] # [doc = ""] # [doc = "`remote_fees_id` may use different transfer type than rest of `assets` and can be"] # [doc = "specified through `fees_transfer_type`."] # [doc = ""] # [doc = "The caller needs to specify what should happen to the transferred assets once they reach"] # [doc = "the `dest` chain. This is done through the `custom_xcm_on_dest` parameter, which"] # [doc = "contains the instructions to execute on `dest` as a final step."] # [doc = "  This is usually as simple as:"] # [doc = "  `Xcm(vec![DepositAsset { assets: Wild(AllCounted(assets.len())), beneficiary }])`,"] # [doc = "  but could be something more exotic like sending the `assets` even further."] # [doc = ""] # [doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."] # [doc = "- `dest`: Destination context for the assets. Will typically be `[Parent,"] # [doc = "  Parachain(..)]` to send from parachain to parachain, or `[Parachain(..)]` to send from"] # [doc = "  relay to parachain, or `(parents: 2, (GlobalConsensus(..), ..))` to send from"] # [doc = "  parachain across a bridge to another ecosystem destination."] # [doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the"] # [doc = "  fee on the `dest` (and possibly reserve) chains."] # [doc = "- `assets_transfer_type`: The XCM `TransferType` used to transfer the `assets`."] # [doc = "- `remote_fees_id`: One of the included `assets` to be used to pay fees."] # [doc = "- `fees_transfer_type`: The XCM `TransferType` used to transfer the `fees` assets."] # [doc = "- `custom_xcm_on_dest`: The XCM to be executed on `dest` chain as the last step of the"] # [doc = "  transfer, which also determines what happens to the assets on the destination chain."] # [doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."] transfer_assets_using_type_and_then { dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , assets : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssets > , assets_transfer_type : :: std :: boxed :: Box < runtime_types :: staging_xcm_executor :: traits :: asset_transfer :: TransferType > , remote_fees_id : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedAssetId > , fees_transfer_type : :: std :: boxed :: Box < runtime_types :: staging_xcm_executor :: traits :: asset_transfer :: TransferType > , custom_xcm_on_dest : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedXcm > , weight_limit : runtime_types :: xcm :: v3 :: WeightLimit , } , # [codec (index = 14)] # [doc = "Authorize another `aliaser` location to alias into the local `origin` making this call."] # [doc = "The `aliaser` is only authorized until the provided `expiry` block number."] # [doc = "The call can also be used for a previously authorized alias in order to update its"] # [doc = "`expiry` block number."] # [doc = ""] # [doc = "Usually useful to allow your local account to be aliased into from a remote location"] # [doc = "also under your control (like your account on another chain)."] # [doc = ""] # [doc = "WARNING: make sure the caller `origin` (you) trusts the `aliaser` location to act in"] # [doc = "their/your name. Once authorized using this call, the `aliaser` can freely impersonate"] # [doc = "`origin` in XCM programs executed on the local chain."] add_authorized_alias { aliaser : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , expires : :: core :: option :: Option < :: core :: primitive :: u64 > , } , # [codec (index = 15)] # [doc = "Remove a previously authorized `aliaser` from the list of locations that can alias into"] # [doc = "the local `origin` making this call."] remove_authorized_alias { aliaser : :: std :: boxed :: Box < runtime_types :: xcm :: VersionedLocation > , } , # [codec (index = 16)] # [doc = "Remove all previously authorized `aliaser`s that can alias into the local `origin`"] # [doc = "making this call."] remove_all_authorized_aliases , }
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Error` enum of this pallet."]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
					#[doc = "to it."]
					Unreachable,
					#[codec(index = 1)]
					#[doc = "There was some other issue (i.e. not to do with routing) in sending the message."]
					#[doc = "Perhaps a lack of space for buffering the message."]
					SendFailure,
					#[codec(index = 2)]
					#[doc = "The message execution fails the filter."]
					Filtered,
					#[codec(index = 3)]
					#[doc = "The message's weight could not be determined."]
					UnweighableMessage,
					#[codec(index = 4)]
					#[doc = "The destination `Location` provided cannot be inverted."]
					DestinationNotInvertible,
					#[codec(index = 5)]
					#[doc = "The assets to be sent are empty."]
					Empty,
					#[codec(index = 6)]
					#[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
					CannotReanchor,
					#[codec(index = 7)]
					#[doc = "Too many assets have been attempted for transfer."]
					TooManyAssets,
					#[codec(index = 8)]
					#[doc = "Origin is invalid for sending."]
					InvalidOrigin,
					#[codec(index = 9)]
					#[doc = "The version of the `Versioned` value used is not able to be interpreted."]
					BadVersion,
					#[codec(index = 10)]
					#[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
					#[doc = "desired version of XCM)."]
					BadLocation,
					#[codec(index = 11)]
					#[doc = "The referenced subscription could not be found."]
					NoSubscription,
					#[codec(index = 12)]
					#[doc = "The location is invalid since it already has a subscription from us."]
					AlreadySubscribed,
					#[codec(index = 13)]
					#[doc = "Could not check-out the assets for teleportation to the destination chain."]
					CannotCheckOutTeleport,
					#[codec(index = 14)]
					#[doc = "The owner does not own (all) of the asset that they wish to do the operation on."]
					LowBalance,
					#[codec(index = 15)]
					#[doc = "The asset owner has too many locks on the asset."]
					TooManyLocks,
					#[codec(index = 16)]
					#[doc = "The given account is not an identifiable sovereign account for any location."]
					AccountNotSovereign,
					#[codec(index = 17)]
					#[doc = "The operation required fees to be paid which the initiator could not meet."]
					FeesNotMet,
					#[codec(index = 18)]
					#[doc = "A remote lock with the corresponding data could not be found."]
					LockNotFound,
					#[codec(index = 19)]
					#[doc = "The unlock operation cannot succeed because there are still consumers of the lock."]
					InUse,
					#[codec(index = 21)]
					#[doc = "Invalid asset, reserve chain could not be determined for it."]
					InvalidAssetUnknownReserve,
					#[codec(index = 22)]
					#[doc = "Invalid asset, do not support remote asset reserves with different fees reserves."]
					InvalidAssetUnsupportedReserve,
					#[codec(index = 23)]
					#[doc = "Too many assets with different reserve locations have been attempted for transfer."]
					TooManyReserves,
					#[codec(index = 24)]
					#[doc = "Local XCM execution incomplete."]
					LocalExecutionIncomplete,
					#[codec(index = 25)]
					#[doc = "Too many locations authorized to alias origin."]
					TooManyAuthorizedAliases,
					#[codec(index = 26)]
					#[doc = "Expiry block number is in the past."]
					ExpiresInPast,
					#[codec(index = 27)]
					#[doc = "The alias to remove authorization for was not found."]
					AliasNotFound,
					#[codec(index = 28)]
					#[doc = "Local XCM execution incomplete with the actual XCM error and the index of the"]
					#[doc = "instruction that caused the error."]
					LocalExecutionIncompleteWithError {
						index: ::core::primitive::u8,
						error: runtime_types::pallet_xcm::errors::ExecutionError,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "The `Event` enum of this pallet"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Execution of an XCM message was attempted."]
					Attempted { outcome: runtime_types::staging_xcm::v5::traits::Outcome },
					#[codec(index = 1)]
					#[doc = "An XCM message was sent."]
					Sent {
						origin: runtime_types::staging_xcm::v5::location::Location,
						destination: runtime_types::staging_xcm::v5::location::Location,
						message: runtime_types::staging_xcm::v5::Xcm,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					#[doc = "An XCM message failed to send."]
					SendFailed {
						origin: runtime_types::staging_xcm::v5::location::Location,
						destination: runtime_types::staging_xcm::v5::location::Location,
						error: runtime_types::xcm::v3::traits::SendError,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 3)]
					#[doc = "An XCM message failed to process."]
					ProcessXcmError {
						origin: runtime_types::staging_xcm::v5::location::Location,
						error: runtime_types::xcm::v5::traits::Error,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 4)]
					#[doc = "Query response received which does not match a registered query. This may be because a"]
					#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
					#[doc = "because the query timed out."]
					UnexpectedResponse {
						origin: runtime_types::staging_xcm::v5::location::Location,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 5)]
					#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
					#[doc = "no registered notification call."]
					ResponseReady {
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v5::Response,
					},
					#[codec(index = 6)]
					#[doc = "Query response has been received and query is removed. The registered notification has"]
					#[doc = "been dispatched and executed successfully."]
					Notified {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 7)]
					#[doc = "Query response has been received and query is removed. The registered notification"]
					#[doc = "could not be dispatched because the dispatch weight is greater than the maximum weight"]
					#[doc = "originally budgeted by this runtime for the query result."]
					NotifyOverweight {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
						actual_weight: runtime_types::sp_weights::weight_v2::Weight,
						max_budgeted_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 8)]
					#[doc = "Query response has been received and query is removed. There was a general error with"]
					#[doc = "dispatching the notification call."]
					NotifyDispatchError {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 9)]
					#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
					#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
					#[doc = "is not `(origin, QueryId, Response)`."]
					NotifyDecodeFailed {
						query_id: ::core::primitive::u64,
						pallet_index: ::core::primitive::u8,
						call_index: ::core::primitive::u8,
					},
					#[codec(index = 10)]
					#[doc = "Expected query response has been received but the origin location of the response does"]
					#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
					#[doc = "be received and acted upon."]
					InvalidResponder {
						origin: runtime_types::staging_xcm::v5::location::Location,
						query_id: ::core::primitive::u64,
						expected_location: ::core::option::Option<
							runtime_types::staging_xcm::v5::location::Location,
						>,
					},
					#[codec(index = 11)]
					#[doc = "Expected query response has been received but the expected origin location placed in"]
					#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
					#[doc = ""]
					#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
					#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
					#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
					#[doc = "needed."]
					InvalidResponderVersion {
						origin: runtime_types::staging_xcm::v5::location::Location,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 12)]
					#[doc = "Received query response has been read and removed."]
					ResponseTaken { query_id: ::core::primitive::u64 },
					#[codec(index = 13)]
					#[doc = "Some assets have been placed in an asset trap."]
					AssetsTrapped {
						hash: ::subxt::utils::H256,
						origin: runtime_types::staging_xcm::v5::location::Location,
						assets: runtime_types::xcm::VersionedAssets,
					},
					#[codec(index = 14)]
					#[doc = "An XCM version change notification message has been attempted to be sent."]
					#[doc = ""]
					#[doc = "The cost of sending it (borne by the chain) is included."]
					VersionChangeNotified {
						destination: runtime_types::staging_xcm::v5::location::Location,
						result: ::core::primitive::u32,
						cost: runtime_types::staging_xcm::v5::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 15)]
					#[doc = "The supported version of a location has been changed. This might be through an"]
					#[doc = "automatic notification or a manual intervention."]
					SupportedVersionChanged {
						location: runtime_types::staging_xcm::v5::location::Location,
						version: ::core::primitive::u32,
					},
					#[codec(index = 16)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "sending the notification to it."]
					NotifyTargetSendFail {
						location: runtime_types::staging_xcm::v5::location::Location,
						query_id: ::core::primitive::u64,
						error: runtime_types::xcm::v5::traits::Error,
					},
					#[codec(index = 17)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "migrating the location to our new XCM format."]
					NotifyTargetMigrationFail {
						location: runtime_types::xcm::VersionedLocation,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 18)]
					#[doc = "Expected query response has been received but the expected querier location placed in"]
					#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
					#[doc = ""]
					#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
					#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
					#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
					#[doc = "needed."]
					InvalidQuerierVersion {
						origin: runtime_types::staging_xcm::v5::location::Location,
						query_id: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					#[doc = "Expected query response has been received but the querier location of the response does"]
					#[doc = "not match the expected. The query remains registered for a later, valid, response to"]
					#[doc = "be received and acted upon."]
					InvalidQuerier {
						origin: runtime_types::staging_xcm::v5::location::Location,
						query_id: ::core::primitive::u64,
						expected_querier: runtime_types::staging_xcm::v5::location::Location,
						maybe_actual_querier: ::core::option::Option<
							runtime_types::staging_xcm::v5::location::Location,
						>,
					},
					#[codec(index = 20)]
					#[doc = "A remote has requested XCM version change notification from us and we have honored it."]
					#[doc = "A version information message is sent to them and its cost is included."]
					VersionNotifyStarted {
						destination: runtime_types::staging_xcm::v5::location::Location,
						cost: runtime_types::staging_xcm::v5::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 21)]
					#[doc = "We have requested that a remote chain send us XCM version change notifications."]
					VersionNotifyRequested {
						destination: runtime_types::staging_xcm::v5::location::Location,
						cost: runtime_types::staging_xcm::v5::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 22)]
					#[doc = "We have requested that a remote chain stops sending us XCM version change"]
					#[doc = "notifications."]
					VersionNotifyUnrequested {
						destination: runtime_types::staging_xcm::v5::location::Location,
						cost: runtime_types::staging_xcm::v5::asset::Assets,
						message_id: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 23)]
					#[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
					FeesPaid {
						paying: runtime_types::staging_xcm::v5::location::Location,
						fees: runtime_types::staging_xcm::v5::asset::Assets,
					},
					#[codec(index = 24)]
					#[doc = "Some assets have been claimed from an asset trap"]
					AssetsClaimed {
						hash: ::subxt::utils::H256,
						origin: runtime_types::staging_xcm::v5::location::Location,
						assets: runtime_types::xcm::VersionedAssets,
					},
					#[codec(index = 25)]
					#[doc = "A XCM version migration finished."]
					VersionMigrationFinished { version: ::core::primitive::u32 },
					#[codec(index = 26)]
					#[doc = "An `aliaser` location was authorized by `target` to alias it, authorization valid until"]
					#[doc = "`expiry` block number."]
					AliasAuthorized {
						aliaser: runtime_types::staging_xcm::v5::location::Location,
						target: runtime_types::staging_xcm::v5::location::Location,
						expiry: ::core::option::Option<::core::primitive::u64>,
					},
					#[codec(index = 27)]
					#[doc = "`target` removed alias authorization for `aliaser`."]
					AliasAuthorizationRemoved {
						aliaser: runtime_types::staging_xcm::v5::location::Location,
						target: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 28)]
					#[doc = "`target` removed all alias authorizations."]
					AliasesAuthorizationsRemoved {
						target: runtime_types::staging_xcm::v5::location::Location,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum HoldReason {
					#[codec(index = 0)]
					AuthorizeAlias,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct MaxAuthorizedAliases;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum QueryStatus<_0> {
					#[codec(index = 0)]
					Pending {
						responder: runtime_types::xcm::VersionedLocation,
						maybe_match_querier:
							::core::option::Option<runtime_types::xcm::VersionedLocation>,
						maybe_notify:
							::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
						timeout: _0,
					},
					#[codec(index = 1)]
					VersionNotifier {
						origin: runtime_types::xcm::VersionedLocation,
						is_active: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					Ready { response: runtime_types::xcm::VersionedResponse, at: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct RemoteLockedFungibleRecord<_0> {
					pub amount: ::core::primitive::u128,
					pub owner: runtime_types::xcm::VersionedLocation,
					pub locker: runtime_types::xcm::VersionedLocation,
					pub consumers: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
						_0,
						::core::primitive::u128,
					)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum VersionMigrationStage {
					#[codec(index = 0)]
					MigrateSupportedVersion,
					#[codec(index = 1)]
					MigrateVersionNotifiers,
					#[codec(index = 2)]
					NotifyCurrentTargets(
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					),
					#[codec(index = 3)]
					MigrateAndNotifyOldTargets,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct AuthorizedAliasesEntry<_0, _1> {
				pub aliasers: runtime_types::bounded_collections::bounded_vec::BoundedVec<
					runtime_types::xcm_runtime_apis::authorized_aliases::OriginAliaser,
				>,
				pub ticket: _0,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
			}
		}
		pub mod parachain_runtime {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeCall {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 2)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 3)]
				ParachainInfo(runtime_types::staging_parachain_info::pallet::Call),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 15)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 33)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Call),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeError {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Error),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Error),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Error),
				#[codec(index = 15)]
				Sudo(runtime_types::pallet_sudo::pallet::Error),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Error),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Error),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Error),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Error),
				#[codec(index = 33)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Error),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeEvent {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 1)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 10)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 11)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
				#[codec(index = 15)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 30)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 32)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 33)]
				MessageQueue(runtime_types::pallet_message_queue::pallet::Event),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum RuntimeHoldReason {
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::HoldReason),
				#[codec(index = 31)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::HoldReason),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct SessionKeys {
				pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain_primitives {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Id(pub ::core::primitive::u32);
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v9 {
				use super::runtime_types;
				pub mod async_backing {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct AsyncBackingParams {
						pub max_candidate_depth: ::core::primitive::u32,
						pub allowed_ancestry_len: ::core::primitive::u32,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AbridgedHostConfiguration {
					pub max_code_size: ::core::primitive::u32,
					pub max_head_data_size: ::core::primitive::u32,
					pub max_upward_queue_count: ::core::primitive::u32,
					pub max_upward_queue_size: ::core::primitive::u32,
					pub max_upward_message_size: ::core::primitive::u32,
					pub max_upward_message_num_per_candidate: ::core::primitive::u32,
					pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
					pub validation_upgrade_cooldown: ::core::primitive::u32,
					pub validation_upgrade_delay: ::core::primitive::u32,
					pub async_backing_params:
						runtime_types::polkadot_primitives::v9::async_backing::AsyncBackingParams,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head:
						runtime_types::polkadot_parachain_primitives::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UpgradeGoAhead {
					#[codec(index = 0)]
					Abort,
					#[codec(index = 1)]
					GoAhead,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Perbill(pub ::core::primitive::u32);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Public(pub [::core::primitive::u8; 32usize]);
				}
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum DigestItem {
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod header {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Header<_0> {
						pub parent_hash: ::subxt::utils::H256,
						#[codec(compact)]
						pub number: _0,
						pub state_root: ::subxt::utils::H256,
						pub extrinsics_root: ::subxt::utils::H256,
						pub digest: runtime_types::sp_runtime::generic::digest::Digest,
					}
				}
			}
			pub mod proving_trie {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum TrieError {
					#[codec(index = 0)]
					InvalidStateRoot,
					#[codec(index = 1)]
					IncompleteDatabase,
					#[codec(index = 2)]
					ValueAtIncompleteKey,
					#[codec(index = 3)]
					DecoderError,
					#[codec(index = 4)]
					InvalidHash,
					#[codec(index = 5)]
					DuplicateKey,
					#[codec(index = 6)]
					ExtraneousNode,
					#[codec(index = 7)]
					ExtraneousValue,
					#[codec(index = 8)]
					ExtraneousHashReference,
					#[codec(index = 9)]
					InvalidChildReference,
					#[codec(index = 10)]
					ValueMismatch,
					#[codec(index = 11)]
					IncompleteProof,
					#[codec(index = 12)]
					RootMismatch,
					#[codec(index = 13)]
					DecodeError,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				TooManyConsumers,
				#[codec(index = 7)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 8)]
				Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
				#[codec(index = 10)]
				Exhausted,
				#[codec(index = 11)]
				Corruption,
				#[codec(index = 12)]
				Unavailable,
				#[codec(index = 13)]
				RootNotAllowed,
				#[codec(index = 14)]
				Trie(runtime_types::sp_runtime::proving_trie::TrieError),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519([::core::primitive::u8; 64usize]),
				#[codec(index = 1)]
				Sr25519([::core::primitive::u8; 64usize]),
				#[codec(index = 2)]
				Ecdsa([::core::primitive::u8; 65usize]),
				#[codec(index = 3)]
				Eth([::core::primitive::u8; 65usize]),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TokenError {
				#[codec(index = 0)]
				FundsUnavailable,
				#[codec(index = 1)]
				OnlyProvider,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
				#[codec(index = 7)]
				CannotCreateHold,
				#[codec(index = 8)]
				NotExpendable,
				#[codec(index = 9)]
				Blocked,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_staking {
			use super::runtime_types;
			pub mod offence {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OffenceSeverity(pub runtime_types::sp_arithmetic::per_things::Perbill);
			}
		}
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
				pub system_version: ::core::primitive::u8,
			}
		}
		pub mod sp_weights {
			use super::runtime_types;
			pub mod weight_v2 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Weight {
					#[codec(compact)]
					pub ref_time: ::core::primitive::u64,
					#[codec(compact)]
					pub proof_size: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub struct RuntimeDbWeight {
				pub read: ::core::primitive::u64,
				pub write: ::core::primitive::u64,
			}
		}
		pub mod staging_parachain_info {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				#[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
				pub enum Call {}
			}
		}
		pub mod staging_xcm {
			use super::runtime_types;
			pub mod v3 {
				use super::runtime_types;
				pub mod multilocation {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v3::junctions::Junctions,
					}
				}
			}
			pub mod v4 {
				use super::runtime_types;
				pub mod asset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Asset {
						pub id: runtime_types::staging_xcm::v4::asset::AssetId,
						pub fun: runtime_types::staging_xcm::v4::asset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::staging_xcm::v4::asset::Assets),
						#[codec(index = 1)]
						Wild(runtime_types::staging_xcm::v4::asset::WildAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct AssetId(pub runtime_types::staging_xcm::v4::location::Location);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Assets(
						pub ::std::vec::Vec<runtime_types::staging_xcm::v4::asset::Asset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::staging_xcm::v4::asset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::staging_xcm::v4::asset::AssetId,
							fun: runtime_types::staging_xcm::v4::asset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::staging_xcm::v4::asset::AssetId,
							fun: runtime_types::staging_xcm::v4::asset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
				}
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v4::junction::NetworkId,
							>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v4::junction::NetworkId,
							>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v4::junction::NetworkId,
							>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::staging_xcm::v4::junction::NetworkId),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
						#[codec(index = 10)]
						PolkadotBulletin,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1([runtime_types::staging_xcm::v4::junction::Junction; 1usize]),
						#[codec(index = 2)]
						X2([runtime_types::staging_xcm::v4::junction::Junction; 2usize]),
						#[codec(index = 3)]
						X3([runtime_types::staging_xcm::v4::junction::Junction; 3usize]),
						#[codec(index = 4)]
						X4([runtime_types::staging_xcm::v4::junction::Junction; 4usize]),
						#[codec(index = 5)]
						X5([runtime_types::staging_xcm::v4::junction::Junction; 5usize]),
						#[codec(index = 6)]
						X6([runtime_types::staging_xcm::v4::junction::Junction; 6usize]),
						#[codec(index = 7)]
						X7([runtime_types::staging_xcm::v4::junction::Junction; 7usize]),
						#[codec(index = 8)]
						X8([runtime_types::staging_xcm::v4::junction::Junction; 8usize]),
					}
				}
				pub mod location {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Location {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::staging_xcm::v4::junctions::Junctions,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v4::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::staging_xcm::v4::location::Location,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						beneficiary: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v4::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::staging_xcm::v4::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						beneficiary: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v4::asset::AssetFilter,
						want: runtime_types::staging_xcm::v4::asset::Assets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						reserve: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v4::asset::Asset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v4::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v4::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						ticket: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<runtime_types::staging_xcm::v4::location::Location>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::staging_xcm::v4::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::staging_xcm::v4::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::staging_xcm::v4::junction::NetworkId,
						destination: runtime_types::staging_xcm::v4::junctions::Junctions,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						unlocker: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						target: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						owner: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						locker: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v4::location::Location),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::staging_xcm::v4::location::Location,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v4::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::staging_xcm::v4::location::Location,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						beneficiary: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v4::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::staging_xcm::v4::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						beneficiary: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v4::asset::AssetFilter,
						want: runtime_types::staging_xcm::v4::asset::Assets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						reserve: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v4::location::Location,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
						assets: runtime_types::staging_xcm::v4::asset::AssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v4::asset::Asset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v4::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v4::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v4::asset::Assets,
						ticket: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<runtime_types::staging_xcm::v4::location::Location>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::staging_xcm::v4::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::staging_xcm::v4::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::staging_xcm::v4::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::staging_xcm::v4::junction::NetworkId,
						destination: runtime_types::staging_xcm::v4::junctions::Junctions,
						xcm: runtime_types::staging_xcm::v4::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						unlocker: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						target: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						owner: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::staging_xcm::v4::asset::Asset,
						locker: runtime_types::staging_xcm::v4::location::Location,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v4::location::Location),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::staging_xcm::v4::location::Location,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::staging_xcm::v4::location::Location,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::staging_xcm::v4::asset::Assets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v4::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::staging_xcm::v4::Instruction>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::staging_xcm::v4::Instruction2>);
			}
			pub mod v5 {
				use super::runtime_types;
				pub mod asset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Asset {
						pub id: runtime_types::staging_xcm::v5::asset::AssetId,
						pub fun: runtime_types::staging_xcm::v5::asset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::staging_xcm::v5::asset::Assets),
						#[codec(index = 1)]
						Wild(runtime_types::staging_xcm::v5::asset::WildAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct AssetId(pub runtime_types::staging_xcm::v5::location::Location);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetTransferFilter {
						#[codec(index = 0)]
						Teleport(runtime_types::staging_xcm::v5::asset::AssetFilter),
						#[codec(index = 1)]
						ReserveDeposit(runtime_types::staging_xcm::v5::asset::AssetFilter),
						#[codec(index = 2)]
						ReserveWithdraw(runtime_types::staging_xcm::v5::asset::AssetFilter),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Assets(
						pub ::std::vec::Vec<runtime_types::staging_xcm::v5::asset::Asset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::staging_xcm::v5::asset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::staging_xcm::v5::asset::AssetId,
							fun: runtime_types::staging_xcm::v5::asset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::staging_xcm::v5::asset::AssetId,
							fun: runtime_types::staging_xcm::v5::asset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
				}
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v5::junction::NetworkId,
							>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v5::junction::NetworkId,
							>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: ::core::option::Option<
								runtime_types::staging_xcm::v5::junction::NetworkId,
							>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::staging_xcm::v5::junction::NetworkId),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
						#[codec(index = 10)]
						PolkadotBulletin,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1([runtime_types::staging_xcm::v5::junction::Junction; 1usize]),
						#[codec(index = 2)]
						X2([runtime_types::staging_xcm::v5::junction::Junction; 2usize]),
						#[codec(index = 3)]
						X3([runtime_types::staging_xcm::v5::junction::Junction; 3usize]),
						#[codec(index = 4)]
						X4([runtime_types::staging_xcm::v5::junction::Junction; 4usize]),
						#[codec(index = 5)]
						X5([runtime_types::staging_xcm::v5::junction::Junction; 5usize]),
						#[codec(index = 6)]
						X6([runtime_types::staging_xcm::v5::junction::Junction; 6usize]),
						#[codec(index = 7)]
						X7([runtime_types::staging_xcm::v5::junction::Junction; 7usize]),
						#[codec(index = 8)]
						X8([runtime_types::staging_xcm::v5::junction::Junction; 8usize]),
					}
				}
				pub mod location {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct Location {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::staging_xcm::v5::junctions::Junctions,
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct InstructionError {
						pub index: ::core::primitive::u8,
						pub error: runtime_types::xcm::v5::traits::Error,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete { used: runtime_types::sp_weights::weight_v2::Weight },
						#[codec(index = 1)]
						Incomplete {
							used: runtime_types::sp_weights::weight_v2::Weight,
							error: runtime_types::staging_xcm::v5::traits::InstructionError,
						},
						#[codec(index = 2)]
						Error(runtime_types::staging_xcm::v5::traits::InstructionError),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Hint {
					#[codec(index = 0)]
					AssetClaimer { location: runtime_types::staging_xcm::v5::location::Location },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v5::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::staging_xcm::v5::location::Location,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v5::asset::Assets,
						beneficiary: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v5::asset::Assets,
						dest: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						fallback_max_weight:
							::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v5::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::staging_xcm::v5::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						beneficiary: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v5::asset::AssetFilter,
						want: runtime_types::staging_xcm::v5::asset::Assets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						reserve: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::staging_xcm::v5::QueryResponseInfo,
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v5::asset::Asset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v5::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v5::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v5::asset::Assets,
						ticket: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<runtime_types::staging_xcm::v5::location::Location>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v5::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::staging_xcm::v5::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::staging_xcm::v5::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::staging_xcm::v5::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::staging_xcm::v5::junction::NetworkId,
						destination: runtime_types::staging_xcm::v5::junctions::Junctions,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						unlocker: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						target: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						owner: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						locker: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v5::location::Location),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::staging_xcm::v5::location::Location,
						>,
					},
					#[codec(index = 48)]
					PayFees { asset: runtime_types::staging_xcm::v5::asset::Asset },
					#[codec(index = 49)]
					InitiateTransfer {
						destination: runtime_types::staging_xcm::v5::location::Location,
						remote_fees: ::core::option::Option<
							runtime_types::staging_xcm::v5::asset::AssetTransferFilter,
						>,
						preserve_origin: ::core::primitive::bool,
						assets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v5::asset::AssetTransferFilter,
						>,
						remote_xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 50)]
					ExecuteWithOrigin {
						descendant_origin: ::core::option::Option<
							runtime_types::staging_xcm::v5::junctions::Junctions,
						>,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 51)]
					SetHints {
						hints: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v5::Hint,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::staging_xcm::v5::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::staging_xcm::v5::location::Location,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::staging_xcm::v5::asset::Assets,
						beneficiary: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::staging_xcm::v5::asset::Assets,
						dest: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						fallback_max_weight:
							::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::staging_xcm::v5::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::staging_xcm::v5::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						beneficiary: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::staging_xcm::v5::asset::AssetFilter,
						want: runtime_types::staging_xcm::v5::asset::Assets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						reserve: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
						dest: runtime_types::staging_xcm::v5::location::Location,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::staging_xcm::v5::QueryResponseInfo,
						assets: runtime_types::staging_xcm::v5::asset::AssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::staging_xcm::v5::asset::Asset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::staging_xcm::v5::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::staging_xcm::v5::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::staging_xcm::v5::asset::Assets,
						ticket: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<runtime_types::staging_xcm::v5::location::Location>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v5::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::staging_xcm::v5::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::staging_xcm::v5::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::staging_xcm::v5::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::staging_xcm::v5::junction::NetworkId,
						destination: runtime_types::staging_xcm::v5::junctions::Junctions,
						xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						unlocker: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						target: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						owner: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::staging_xcm::v5::asset::Asset,
						locker: runtime_types::staging_xcm::v5::location::Location,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v5::location::Location),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::staging_xcm::v5::location::Location,
						>,
					},
					#[codec(index = 48)]
					PayFees { asset: runtime_types::staging_xcm::v5::asset::Asset },
					#[codec(index = 49)]
					InitiateTransfer {
						destination: runtime_types::staging_xcm::v5::location::Location,
						remote_fees: ::core::option::Option<
							runtime_types::staging_xcm::v5::asset::AssetTransferFilter,
						>,
						preserve_origin: ::core::primitive::bool,
						assets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v5::asset::AssetTransferFilter,
						>,
						remote_xcm: runtime_types::staging_xcm::v5::Xcm,
					},
					#[codec(index = 50)]
					ExecuteWithOrigin {
						descendant_origin: ::core::option::Option<
							runtime_types::staging_xcm::v5::junctions::Junctions,
						>,
						xcm: runtime_types::staging_xcm::v5::Xcm2,
					},
					#[codec(index = 51)]
					SetHints {
						hints: runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v5::Hint,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::staging_xcm::v5::location::Location,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::staging_xcm::v5::asset::Assets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v5::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::staging_xcm::v5::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::staging_xcm::v5::Instruction>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::staging_xcm::v5::Instruction2>);
			}
		}
		pub mod staging_xcm_executor {
			use super::runtime_types;
			pub mod traits {
				use super::runtime_types;
				pub mod asset_transfer {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum TransferType {
						#[codec(index = 0)]
						Teleport,
						#[codec(index = 1)]
						LocalReserve,
						#[codec(index = 2)]
						DestinationReserve,
						#[codec(index = 3)]
						RemoteReserve(runtime_types::xcm::VersionedLocation),
					}
				}
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct DoubleEncoded2 {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v3 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Moniker([::core::primitive::u8; 4usize]),
						#[codec(index = 2)]
						Index(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
						#[codec(index = 7)]
						Defense,
						#[codec(index = 8)]
						Administration,
						#[codec(index = 9)]
						Treasury,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network:
								::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey {
							length: ::core::primitive::u8,
							data: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v3::junction::BodyId,
							part: runtime_types::xcm::v3::junction::BodyPart,
						},
						#[codec(index = 9)]
						GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum NetworkId {
						#[codec(index = 0)]
						ByGenesis([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						ByFork {
							block_number: ::core::primitive::u64,
							block_hash: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
						#[codec(index = 4)]
						Westend,
						#[codec(index = 5)]
						Rococo,
						#[codec(index = 6)]
						Wococo,
						#[codec(index = 7)]
						Ethereum {
							#[codec(compact)]
							chain_id: ::core::primitive::u64,
						},
						#[codec(index = 8)]
						BitcoinCore,
						#[codec(index = 9)]
						BitcoinCash,
						#[codec(index = 10)]
						PolkadotBulletin,
					}
				}
				pub mod junctions {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v3::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
							runtime_types::xcm::v3::junction::Junction,
						),
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v3::multiasset::AssetId,
						pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v3::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
						},
						#[codec(index = 2)]
						AllCounted(#[codec(compact)] ::core::primitive::u32),
						#[codec(index = 3)]
						AllOfCounted {
							id: runtime_types::xcm::v3::multiasset::AssetId,
							fun: runtime_types::xcm::v3::multiasset::WildFungibility,
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
					}
				}
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						LocationFull,
						#[codec(index = 5)]
						LocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						ExpectationFalse,
						#[codec(index = 23)]
						PalletNotFound,
						#[codec(index = 24)]
						NameMismatch,
						#[codec(index = 25)]
						VersionIncompatible,
						#[codec(index = 26)]
						HoldingWouldOverflow,
						#[codec(index = 27)]
						ExportError,
						#[codec(index = 28)]
						ReanchorFailed,
						#[codec(index = 29)]
						NoDeal,
						#[codec(index = 30)]
						FeesNotMet,
						#[codec(index = 31)]
						LockError,
						#[codec(index = 32)]
						NoPermission,
						#[codec(index = 33)]
						Unanchored,
						#[codec(index = 34)]
						NotDepositable,
						#[codec(index = 35)]
						UnhandledXcmVersion,
						#[codec(index = 36)]
						WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 37)]
						Barrier,
						#[codec(index = 38)]
						WeightNotComputable,
						#[codec(index = 39)]
						ExceedsStackLimit,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum SendError {
						#[codec(index = 0)]
						NotApplicable,
						#[codec(index = 1)]
						Transport,
						#[codec(index = 2)]
						Unroutable,
						#[codec(index = 3)]
						DestinationUnsupported,
						#[codec(index = 4)]
						ExceedsMaxMessageSize,
						#[codec(index = 5)]
						MissingArgument,
						#[codec(index = 6)]
						Fees,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Instruction2 {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v3::Response,
						max_weight: runtime_types::sp_weights::weight_v2::Weight,
						querier: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_kind: runtime_types::xcm::v3::OriginKind,
						require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
						call: runtime_types::xcm::double_encoded::DoubleEncoded2,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
					#[codec(index = 12)]
					ReportError(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						beneficiary: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						want: runtime_types::xcm::v3::multiasset::MultiAssets,
						maximal: ::core::primitive::bool,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						reserve: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
						dest: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 18)]
					ReportHolding {
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
						assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v3::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v3::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v3::Xcm2),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v3::Xcm2),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v3::multiasset::MultiAssets,
						ticket: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(#[codec(compact)] ::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
					#[codec(index = 28)]
					BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 29)]
					ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 30)]
					ExpectOrigin(
						::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 31)]
					ExpectError(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 32)]
					ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
					#[codec(index = 33)]
					QueryPallet {
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						response_info: runtime_types::xcm::v3::QueryResponseInfo,
					},
					#[codec(index = 34)]
					ExpectPallet {
						#[codec(compact)]
						index: ::core::primitive::u32,
						name: ::std::vec::Vec<::core::primitive::u8>,
						module_name: ::std::vec::Vec<::core::primitive::u8>,
						#[codec(compact)]
						crate_major: ::core::primitive::u32,
						#[codec(compact)]
						min_crate_minor: ::core::primitive::u32,
					},
					#[codec(index = 35)]
					ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
					#[codec(index = 36)]
					ClearTransactStatus,
					#[codec(index = 37)]
					UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
					#[codec(index = 38)]
					ExportMessage {
						network: runtime_types::xcm::v3::junction::NetworkId,
						destination: runtime_types::xcm::v3::junctions::Junctions,
						xcm: runtime_types::xcm::v3::Xcm,
					},
					#[codec(index = 39)]
					LockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						unlocker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 40)]
					UnlockAsset {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						target: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 41)]
					NoteUnlockable {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						owner: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 42)]
					RequestUnlock {
						asset: runtime_types::xcm::v3::multiasset::MultiAsset,
						locker: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					},
					#[codec(index = 43)]
					SetFeesMode { jit_withdraw: ::core::primitive::bool },
					#[codec(index = 44)]
					SetTopic([::core::primitive::u8; 32usize]),
					#[codec(index = 45)]
					ClearTopic,
					#[codec(index = 46)]
					AliasOrigin(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
					#[codec(index = 47)]
					UnpaidExecution {
						weight_limit: runtime_types::xcm::v3::WeightLimit,
						check_origin: ::core::option::Option<
							runtime_types::staging_xcm::v3::multilocation::MultiLocation,
						>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum MaybeErrorCode {
					#[codec(index = 0)]
					Success,
					#[codec(index = 1)]
					Error(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
					#[codec(index = 2)]
					TruncatedError(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							::core::primitive::u8,
						>,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct PalletInfo {
					#[codec(compact)]
					pub index: ::core::primitive::u32,
					pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
						::core::primitive::u8,
					>,
					#[codec(compact)]
					pub major: ::core::primitive::u32,
					#[codec(compact)]
					pub minor: ::core::primitive::u32,
					#[codec(compact)]
					pub patch: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct QueryResponseInfo {
					pub destination: runtime_types::staging_xcm::v3::multilocation::MultiLocation,
					#[codec(compact)]
					pub query_id: ::core::primitive::u64,
					pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v3::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
					#[codec(index = 4)]
					PalletsInfo(
						runtime_types::bounded_collections::bounded_vec::BoundedVec<
							runtime_types::xcm::v3::PalletInfo,
						>,
					),
					#[codec(index = 5)]
					DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(runtime_types::sp_weights::weight_v2::Weight),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction>);
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction2>);
			}
			pub mod v5 {
				use super::runtime_types;
				pub mod traits {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						:: subxt :: ext :: scale_decode :: DecodeAsType,
						:: subxt :: ext :: scale_encode :: EncodeAsType,
						Debug,
					)]
					# [codec (crate = :: subxt :: ext :: codec)]
					#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
					#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						LocationFull,
						#[codec(index = 5)]
						LocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						MaxWeightInvalid,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						ExpectationFalse,
						#[codec(index = 23)]
						PalletNotFound,
						#[codec(index = 24)]
						NameMismatch,
						#[codec(index = 25)]
						VersionIncompatible,
						#[codec(index = 26)]
						HoldingWouldOverflow,
						#[codec(index = 27)]
						ExportError,
						#[codec(index = 28)]
						ReanchorFailed,
						#[codec(index = 29)]
						NoDeal,
						#[codec(index = 30)]
						FeesNotMet,
						#[codec(index = 31)]
						LockError,
						#[codec(index = 32)]
						NoPermission,
						#[codec(index = 33)]
						Unanchored,
						#[codec(index = 34)]
						NotDepositable,
						#[codec(index = 35)]
						TooManyAssets,
						#[codec(index = 36)]
						UnhandledXcmVersion,
						#[codec(index = 37)]
						WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
						#[codec(index = 38)]
						Barrier,
						#[codec(index = 39)]
						WeightNotComputable,
						#[codec(index = 40)]
						ExceedsStackLimit,
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedAssetId {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::AssetId),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::asset::AssetId),
				#[codec(index = 5)]
				V5(runtime_types::staging_xcm::v5::asset::AssetId),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedAssets {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::multiasset::MultiAssets),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::asset::Assets),
				#[codec(index = 5)]
				V5(runtime_types::staging_xcm::v5::asset::Assets),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedLocation {
				#[codec(index = 3)]
				V3(runtime_types::staging_xcm::v3::multilocation::MultiLocation),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::location::Location),
				#[codec(index = 5)]
				V5(runtime_types::staging_xcm::v5::location::Location),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedResponse {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Response),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::Response),
				#[codec(index = 5)]
				V5(runtime_types::staging_xcm::v5::Response),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedXcm {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::Xcm),
				#[codec(index = 5)]
				V5(runtime_types::staging_xcm::v5::Xcm),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				:: subxt :: ext :: scale_decode :: DecodeAsType,
				:: subxt :: ext :: scale_encode :: EncodeAsType,
				Debug,
			)]
			# [codec (crate = :: subxt :: ext :: codec)]
			#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
			#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
			pub enum VersionedXcm2 {
				#[codec(index = 3)]
				V3(runtime_types::xcm::v3::Xcm2),
				#[codec(index = 4)]
				V4(runtime_types::staging_xcm::v4::Xcm2),
				#[codec(index = 5)]
				V5(runtime_types::staging_xcm::v5::Xcm2),
			}
		}
		pub mod xcm_runtime_apis {
			use super::runtime_types;
			pub mod authorized_aliases {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					:: subxt :: ext :: scale_decode :: DecodeAsType,
					:: subxt :: ext :: scale_encode :: EncodeAsType,
					Debug,
				)]
				# [codec (crate = :: subxt :: ext :: codec)]
				#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
				#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
				pub struct OriginAliaser {
					pub location: runtime_types::xcm::VersionedLocation,
					pub expiry: ::core::option::Option<::core::primitive::u64>,
				}
			}
		}
	}
}
