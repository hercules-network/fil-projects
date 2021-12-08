// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `service.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rservice.proto\x1a\rrequest.proto\x1a\x0eresponse.proto\x1a\x0bempty.\
    proto2N\n\x0fFilSealVerifier\x12;\n\nVerifySeal\x12\x15.FilVerifySealReq\
    uest\x1a\x16.FilVerifySealResponse2R\n\x10FilPieceVerifier\x12>\n\x0bVer\
    ifyPiece\x12\x16.FilVerifyPieceRequest\x1a\x17.FilVerifyPieceResponse2N\
    \n\x0fFilPoStVerifier\x12;\n\nVerifyPoSt\x12\x15.FilVerifyPoStRequest\
    \x1a\x16.FilVerifyPoStResponse2\xe6\x07\n\nFilBuilder\x12C\n\x04Init\x12\
    \x1c.FilInitSectorBuilderRequest\x1a\x1d.FilInitSectorBuilderResponse\
    \x125\n\x08AddPiece\x12\x13.FilAddPieceRequest\x1a\x14.FilAddPieceRespon\
    se\x12A\n\x0eAddPieceExtend\x12\x19.FilAddPieceExtendRequest\x1a\x14.Fil\
    AddPieceResponse\x12h\n\x19ReadPieceFromSealedSector\x12$.FilReadPieceFr\
    omSealedSectorRequest\x1a%.FilReadPieceFromSealedSectorResponse\x12D\n\r\
    GetSealStatus\x12\x18.FilGetSealStatusRequest\x1a\x19.FilGetSealStatusRe\
    sponse\x12M\n\x10GetSealedSectors\x12\x1b.FilGetSealedSectorsRequest\x1a\
    \x1c.FilGetSealedSectorsResponse\x128\n\x10GetStagedSectors\x12\x06.Empt\
    y\x1a\x1c.FilGetStagedSectorsResponse\x12@\n\x14SealAllStagedSectors\x12\
    \x06.Empty\x1a\x20.FilSealAllStagedSectorsResponse\x12w\n\x1eGetMaxUserB\
    ytesPerStagedSector\x12).FilGetMaxUserBytesPerStagedSectorRequest\x1a*.F\
    ilGetMaxUserBytesPerStagedSectorResponse\x12>\n\x0bGetSoonSeal\x12\x16.F\
    ilGetSoonSealRequest\x1a\x17.FilGetSoonSealResponse\x12A\n\x0cSealCallBa\
    ck\x12\x17.FilSealCallBackRequest\x1a\x18.FilSealCallBackResponse\x12_\n\
    \x16GeneratePieceCommitent\x12!.FilGeneratePieceCommitentRequest\x1a\".F\
    ilGeneratePieceCommitentResponse\x12A\n\x0cGeneratePoSt\x12\x17.FilGener\
    atePoStRequest\x1a\x18.FilGeneratePoStResponse2:\n\rFilRemoteSeal\x12)\n\
    \x04Seal\x12\x0f.FilSealRequest\x1a\x10.FilSealResponseb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}