// This file is generated. Do not edit
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


// interface

pub trait FilSealVerifier {
    fn verify_seal(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifySealRequest) -> ::grpc::SingleResponse<super::response::FilVerifySealResponse>;
}

// client

pub struct FilSealVerifierClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_VerifySeal: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilVerifySealRequest, super::response::FilVerifySealResponse>>,
}

impl ::grpc::ClientStub for FilSealVerifierClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        FilSealVerifierClient {
            grpc_client: grpc_client,
            method_VerifySeal: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilSealVerifier/VerifySeal".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl FilSealVerifier for FilSealVerifierClient {
    fn verify_seal(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifySealRequest) -> ::grpc::SingleResponse<super::response::FilVerifySealResponse> {
        self.grpc_client.call_unary(o, p, self.method_VerifySeal.clone())
    }
}

// server

pub struct FilSealVerifierServer;


impl FilSealVerifierServer {
    pub fn new_service_def<H : FilSealVerifier + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/FilSealVerifier",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilSealVerifier/VerifySeal".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify_seal(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait FilPieceVerifier {
    fn verify_piece(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifyPieceRequest) -> ::grpc::SingleResponse<super::response::FilVerifyPieceResponse>;
}

// client

pub struct FilPieceVerifierClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_VerifyPiece: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilVerifyPieceRequest, super::response::FilVerifyPieceResponse>>,
}

impl ::grpc::ClientStub for FilPieceVerifierClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        FilPieceVerifierClient {
            grpc_client: grpc_client,
            method_VerifyPiece: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilPieceVerifier/VerifyPiece".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl FilPieceVerifier for FilPieceVerifierClient {
    fn verify_piece(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifyPieceRequest) -> ::grpc::SingleResponse<super::response::FilVerifyPieceResponse> {
        self.grpc_client.call_unary(o, p, self.method_VerifyPiece.clone())
    }
}

// server

pub struct FilPieceVerifierServer;


impl FilPieceVerifierServer {
    pub fn new_service_def<H : FilPieceVerifier + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/FilPieceVerifier",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilPieceVerifier/VerifyPiece".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify_piece(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait FilPoStVerifier {
    fn verify_po_st(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifyPoStRequest) -> ::grpc::SingleResponse<super::response::FilVerifyPoStResponse>;
}

// client

pub struct FilPoStVerifierClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_VerifyPoSt: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilVerifyPoStRequest, super::response::FilVerifyPoStResponse>>,
}

impl ::grpc::ClientStub for FilPoStVerifierClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        FilPoStVerifierClient {
            grpc_client: grpc_client,
            method_VerifyPoSt: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilPoStVerifier/VerifyPoSt".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl FilPoStVerifier for FilPoStVerifierClient {
    fn verify_po_st(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifyPoStRequest) -> ::grpc::SingleResponse<super::response::FilVerifyPoStResponse> {
        self.grpc_client.call_unary(o, p, self.method_VerifyPoSt.clone())
    }
}

// server

pub struct FilPoStVerifierServer;


impl FilPoStVerifierServer {
    pub fn new_service_def<H : FilPoStVerifier + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/FilPoStVerifier",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilPoStVerifier/VerifyPoSt".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify_po_st(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait FilBuilder {
    fn init(&self, o: ::grpc::RequestOptions, p: super::request::FilInitSectorBuilderRequest) -> ::grpc::SingleResponse<super::response::FilInitSectorBuilderResponse>;

    fn add_piece(&self, o: ::grpc::RequestOptions, p: super::request::FilAddPieceRequest) -> ::grpc::SingleResponse<super::response::FilAddPieceResponse>;

    fn add_piece_extend(&self, o: ::grpc::RequestOptions, p: super::request::FilAddPieceExtendRequest) -> ::grpc::SingleResponse<super::response::FilAddPieceResponse>;

    fn read_piece_from_sealed_sector(&self, o: ::grpc::RequestOptions, p: super::request::FilReadPieceFromSealedSectorRequest) -> ::grpc::SingleResponse<super::response::FilReadPieceFromSealedSectorResponse>;

    fn get_seal_status(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSealStatusRequest) -> ::grpc::SingleResponse<super::response::FilGetSealStatusResponse>;

    fn get_sealed_sectors(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSealedSectorsRequest) -> ::grpc::SingleResponse<super::response::FilGetSealedSectorsResponse>;

    fn get_staged_sectors(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::response::FilGetStagedSectorsResponse>;

    fn seal_all_staged_sectors(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::response::FilSealAllStagedSectorsResponse>;

    fn get_max_user_bytes_per_staged_sector(&self, o: ::grpc::RequestOptions, p: super::request::FilGetMaxUserBytesPerStagedSectorRequest) -> ::grpc::SingleResponse<super::response::FilGetMaxUserBytesPerStagedSectorResponse>;

    fn get_soon_seal(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSoonSealRequest) -> ::grpc::SingleResponse<super::request::FilGetSoonSealResponse>;

    fn seal_call_back(&self, o: ::grpc::RequestOptions, p: super::request::FilSealCallBackRequest) -> ::grpc::SingleResponse<super::response::FilSealCallBackResponse>;

    fn generate_piece_commitent(&self, o: ::grpc::RequestOptions, p: super::request::FilGeneratePieceCommitentRequest) -> ::grpc::SingleResponse<super::response::FilGeneratePieceCommitentResponse>;

    fn generate_po_st(&self, o: ::grpc::RequestOptions, p: super::request::FilGeneratePoStRequest) -> ::grpc::SingleResponse<super::response::FilGeneratePoStResponse>;
}

// client

pub struct FilBuilderClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Init: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilInitSectorBuilderRequest, super::response::FilInitSectorBuilderResponse>>,
    method_AddPiece: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilAddPieceRequest, super::response::FilAddPieceResponse>>,
    method_AddPieceExtend: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilAddPieceExtendRequest, super::response::FilAddPieceResponse>>,
    method_ReadPieceFromSealedSector: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilReadPieceFromSealedSectorRequest, super::response::FilReadPieceFromSealedSectorResponse>>,
    method_GetSealStatus: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilGetSealStatusRequest, super::response::FilGetSealStatusResponse>>,
    method_GetSealedSectors: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilGetSealedSectorsRequest, super::response::FilGetSealedSectorsResponse>>,
    method_GetStagedSectors: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::empty::Empty, super::response::FilGetStagedSectorsResponse>>,
    method_SealAllStagedSectors: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::empty::Empty, super::response::FilSealAllStagedSectorsResponse>>,
    method_GetMaxUserBytesPerStagedSector: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilGetMaxUserBytesPerStagedSectorRequest, super::response::FilGetMaxUserBytesPerStagedSectorResponse>>,
    method_GetSoonSeal: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilGetSoonSealRequest, super::request::FilGetSoonSealResponse>>,
    method_SealCallBack: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilSealCallBackRequest, super::response::FilSealCallBackResponse>>,
    method_GeneratePieceCommitent: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilGeneratePieceCommitentRequest, super::response::FilGeneratePieceCommitentResponse>>,
    method_GeneratePoSt: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilGeneratePoStRequest, super::response::FilGeneratePoStResponse>>,
}

impl ::grpc::ClientStub for FilBuilderClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        FilBuilderClient {
            grpc_client: grpc_client,
            method_Init: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/Init".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddPiece: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/AddPiece".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddPieceExtend: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/AddPieceExtend".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReadPieceFromSealedSector: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/ReadPieceFromSealedSector".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetSealStatus: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/GetSealStatus".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetSealedSectors: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/GetSealedSectors".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetStagedSectors: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/GetStagedSectors".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SealAllStagedSectors: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/SealAllStagedSectors".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetMaxUserBytesPerStagedSector: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/GetMaxUserBytesPerStagedSector".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetSoonSeal: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/GetSoonSeal".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SealCallBack: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/SealCallBack".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GeneratePieceCommitent: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/GeneratePieceCommitent".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GeneratePoSt: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilBuilder/GeneratePoSt".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl FilBuilder for FilBuilderClient {
    fn init(&self, o: ::grpc::RequestOptions, p: super::request::FilInitSectorBuilderRequest) -> ::grpc::SingleResponse<super::response::FilInitSectorBuilderResponse> {
        self.grpc_client.call_unary(o, p, self.method_Init.clone())
    }

    fn add_piece(&self, o: ::grpc::RequestOptions, p: super::request::FilAddPieceRequest) -> ::grpc::SingleResponse<super::response::FilAddPieceResponse> {
        self.grpc_client.call_unary(o, p, self.method_AddPiece.clone())
    }

    fn add_piece_extend(&self, o: ::grpc::RequestOptions, p: super::request::FilAddPieceExtendRequest) -> ::grpc::SingleResponse<super::response::FilAddPieceResponse> {
        self.grpc_client.call_unary(o, p, self.method_AddPieceExtend.clone())
    }

    fn read_piece_from_sealed_sector(&self, o: ::grpc::RequestOptions, p: super::request::FilReadPieceFromSealedSectorRequest) -> ::grpc::SingleResponse<super::response::FilReadPieceFromSealedSectorResponse> {
        self.grpc_client.call_unary(o, p, self.method_ReadPieceFromSealedSector.clone())
    }

    fn get_seal_status(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSealStatusRequest) -> ::grpc::SingleResponse<super::response::FilGetSealStatusResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetSealStatus.clone())
    }

    fn get_sealed_sectors(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSealedSectorsRequest) -> ::grpc::SingleResponse<super::response::FilGetSealedSectorsResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetSealedSectors.clone())
    }

    fn get_staged_sectors(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::response::FilGetStagedSectorsResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetStagedSectors.clone())
    }

    fn seal_all_staged_sectors(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::response::FilSealAllStagedSectorsResponse> {
        self.grpc_client.call_unary(o, p, self.method_SealAllStagedSectors.clone())
    }

    fn get_max_user_bytes_per_staged_sector(&self, o: ::grpc::RequestOptions, p: super::request::FilGetMaxUserBytesPerStagedSectorRequest) -> ::grpc::SingleResponse<super::response::FilGetMaxUserBytesPerStagedSectorResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetMaxUserBytesPerStagedSector.clone())
    }

    fn get_soon_seal(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSoonSealRequest) -> ::grpc::SingleResponse<super::request::FilGetSoonSealResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetSoonSeal.clone())
    }

    fn seal_call_back(&self, o: ::grpc::RequestOptions, p: super::request::FilSealCallBackRequest) -> ::grpc::SingleResponse<super::response::FilSealCallBackResponse> {
        self.grpc_client.call_unary(o, p, self.method_SealCallBack.clone())
    }

    fn generate_piece_commitent(&self, o: ::grpc::RequestOptions, p: super::request::FilGeneratePieceCommitentRequest) -> ::grpc::SingleResponse<super::response::FilGeneratePieceCommitentResponse> {
        self.grpc_client.call_unary(o, p, self.method_GeneratePieceCommitent.clone())
    }

    fn generate_po_st(&self, o: ::grpc::RequestOptions, p: super::request::FilGeneratePoStRequest) -> ::grpc::SingleResponse<super::response::FilGeneratePoStResponse> {
        self.grpc_client.call_unary(o, p, self.method_GeneratePoSt.clone())
    }
}

// server

pub struct FilBuilderServer;


impl FilBuilderServer {
    pub fn new_service_def<H : FilBuilder + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/FilBuilder",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/Init".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.init(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/AddPiece".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_piece(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/AddPieceExtend".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_piece_extend(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/ReadPieceFromSealedSector".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.read_piece_from_sealed_sector(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/GetSealStatus".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_seal_status(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/GetSealedSectors".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_sealed_sectors(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/GetStagedSectors".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_staged_sectors(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/SealAllStagedSectors".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.seal_all_staged_sectors(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/GetMaxUserBytesPerStagedSector".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_max_user_bytes_per_staged_sector(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/GetSoonSeal".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_soon_seal(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/SealCallBack".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.seal_call_back(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/GeneratePieceCommitent".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.generate_piece_commitent(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilBuilder/GeneratePoSt".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.generate_po_st(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait FilRemoteSeal {
    fn seal(&self, o: ::grpc::RequestOptions, p: super::request::FilSealRequest) -> ::grpc::SingleResponse<super::response::FilSealResponse>;
}

// client

pub struct FilRemoteSealClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Seal: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::request::FilSealRequest, super::response::FilSealResponse>>,
}

impl ::grpc::ClientStub for FilRemoteSealClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        FilRemoteSealClient {
            grpc_client: grpc_client,
            method_Seal: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/FilRemoteSeal/Seal".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl FilRemoteSeal for FilRemoteSealClient {
    fn seal(&self, o: ::grpc::RequestOptions, p: super::request::FilSealRequest) -> ::grpc::SingleResponse<super::response::FilSealResponse> {
        self.grpc_client.call_unary(o, p, self.method_Seal.clone())
    }
}

// server

pub struct FilRemoteSealServer;


impl FilRemoteSealServer {
    pub fn new_service_def<H : FilRemoteSeal + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/FilRemoteSeal",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/FilRemoteSeal/Seal".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.seal(o, p))
                    },
                ),
            ],
        )
    }
}
