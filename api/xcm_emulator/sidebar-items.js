initSidebarItems({"constant":[["DOWNWARD_MESSAGES","Downward messages, each message is: `(to_para_id, [(relay_block_number, msg)])`"],["HORIZONTAL_MESSAGES","Horizontal messages, each message is: `(to_para_id, [(from_para_id, relay_block_number, msg)])`"],["UPWARD_MESSAGES","Upward messages, each message is: `(from_para_id, msg)"],["XCM_VERSION","This module’s XCM version."]],"derive":[["Encode","Derive `parity_scale_codec::Encode` and `parity_scale_codec::EncodeLike` for struct and enum."]],"enum":[["AssetId","Classification of an asset being concrete or abstract."],["AssetInstance","A general identifier for an instance of a non-fungible asset class."],["BodyId","An identifier of a pluralistic body."],["BodyPart","A part of a pluralistic body."],["Fungibility","Classification of whether an asset is fungible or not, along with a mandatory amount or instance."],["Instruction","Cross-Consensus Message: A message from one consensus system to another."],["Junction","A single item in a path to describe the relative location of a consensus system."],["Junctions","Non-parent junctions that can be constructed, up to the length of 8. This specific `Junctions` implementation uses a Rust `enum` in order to make pattern matching easier."],["MultiAssetFilter","`MultiAsset` collection, either `MultiAssets` or a single wildcard."],["NetworkId","A global identifier of an account-bearing consensus system."],["OriginKind","Basically just the XCM (more general) version of `ParachainDispatchOrigin`."],["Outcome","Outcome of an XCM execution."],["Response","Response data to a query."],["SendError","Error result value when attempting to send an XCM message."],["VersionedXcm","A single XCM message, together with its version code."],["WeightLimit","An optional weight limit."],["WildFungibility","Classification of whether an asset is fungible or not."],["WildMultiAsset","A wildcard representing a set of assets."],["XcmError",""]],"macro":[["__impl_ext_for_parachain",""],["__impl_ext_for_relay_chain",""],["decl_test_network",""],["decl_test_parachain",""],["decl_test_relay_chain",""]],"mod":[["dmp",""],["opaque",""]],"struct":[["Ancestor","A unit struct which can be converted into a `MultiLocation` of the inner `parents` value."],["AncestorThen","A unit struct which can be converted into a `MultiLocation` of the inner `parents` value and the inner interior."],["MultiAsset",""],["MultiAssets","A `Vec` of `MultiAsset`s. There may be no duplicate fungible items in here and when decoding, they must be sorted."],["MultiLocation","A relative path between state-bearing consensus systems."],["ParaId","Unique identifier of a parachain."],["ParachainInherentData","The inherent data that is passed by the collator to the parachain runtime."],["Parent","A unit struct which can be converted into a `MultiLocation` of `parents` value 1."],["ParentThen","A tuple struct which can be converted into a `MultiLocation` of `parents` value 1 with the inner interior."],["PersistedValidationData","The validation data provides information about how to create the inputs for validation of a candidate. This information is derived from the chain state and will vary from para to para, although some fields may be the same for every para."],["PhantomData","Zero-sized type used to mark things that “act like” they own a `T`."],["RefCell","A mutable memory location with dynamically checked borrow rules"],["RelayStateSproofBuilder","Builds a sproof (portmanteau of ‘spoof’ and ‘proof’) of the relay chain state."],["VecDeque","A double-ended queue implemented with a growable ring buffer."],["Xcm",""],["XcmExecutor","The XCM executor."],["XcmSink","A specific implementation of a `UmpSink` where messages are in the XCM format and will be forwarded to the XCM Executor."]],"trait":[["DmpMessageHandler","Something that should be called when a downward message is received."],["Encode","Trait that allows zero-copy write of value-references to slices in LE format."],["ExecuteXcm","Type of XCM message executor."],["Get","A trait for querying a single value from a type."],["Hooks","The pallet hooks trait. Implementing this lets you express some logic to execute."],["SendXcm","Utility for sending an XCM message."],["TestExt",""],["UmpSink","All upward messages coming from parachains will be funneled into an implementation of this trait."],["XcmWeightInfo","The info needed to weight an XCM."],["XcmpMessageHandler","Something that should be called for each batch of messages received over XCMP."]],"type":[["InteriorMultiLocation","A relative location which is constrained to be an interior location of the context."],["MessageId","Simple type used to identify messages for the purpose of reporting events. Secure if and only if the message content is unique."],["QueryId","An identifier for a query."],["RelayBlockNumber","The block number type used by Polkadot. 32-bits will allow for 136 years of blocks assuming 1 block per second."],["SendResult","Result value when attempting to send an XCM message."],["TestExternalities","Type alias for Externalities implementation used in tests."],["Weight","Numeric range of a transaction weight."],["XcmResult",""]]});