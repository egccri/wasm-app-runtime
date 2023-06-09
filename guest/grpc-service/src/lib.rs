#[cfg(target_arch = "wasm32")]
use egccri_rust::invoker_grpc::__link_section;
use egccri_rust::invoker_grpc::exports;

struct GrpcService;

impl exports::gprc::Gprc for GrpcService {
    fn call(payload: String) -> Result<String, exports::gprc::Error> {
        if payload.eq("egccri") {
            Ok("success".to_string())
        } else {
            Err(404)
        }
    }
}

egccri_rust::invoker_grpc::export_invoker_grpc!(GrpcService);
