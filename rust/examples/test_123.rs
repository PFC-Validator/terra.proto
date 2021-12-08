use protobuf::Message;
use terra_rust_proto::cosmos::tx::tx::Tx;
use terra_rust_proto::protos::wasm::tx::MsgExecuteContract;

fn main() {
    let msg_1=  "CsQBCsEBCiYvdGVycmEud2FzbS52MWJldGExLk1zZ0V4ZWN1dGVDb250cmFjdBKWAQosdGVycmExNjRrcnZhaHFlY3p0MDU4djdkODZxcnB3d2EybWcyZDZjaDB2dzUSLHRlcnJhMW44ZXRucWVqdHo2eGg3dG4zdGx2YXJkNDBqNnV6ZWVhZDlybHJsGjh7InN1Ym1pdCI6eyJyb3VuZF9pZCI6MjI4MDMyLCJzdWJtaXNzaW9uIjoiNjkxNjgzNTgxMCJ9fRJpClIKRgofL2Nvc21vcy5jcnlwdG8uc2VjcDI1NmsxLlB1YktleRIjCiEDMzHIFQn58OaE+yp7nPkuyOV+1uENOUR7yyah64ADQxkSBAoCCAEY2ZNmEhMKDQoFdWx1bmESBDQ1MDAQ4KcSGkBObdKuGK4//3Rpy1n3VzOQbRykkfUZPy2JIzhFfF1+EDGliR8AWrCmfN4DqIrgYe7Qqew/mIQq2Y8+6pSl7dS0";
    let b64_msg = base64::decode(msg_1).unwrap();
    let tx: Tx = Tx::parse_from_bytes(b64_msg.as_slice()).unwrap();
    if tx.has_body() {
        let body = tx.get_body();
        //   println!("{:?}", body);
        body.messages.iter().for_each(|msg| {
            let f: MsgExecuteContract =
                MsgExecuteContract::parse_from_bytes(msg.get_value()).unwrap();
            println!("{:?}", f);
        });
    }
}
