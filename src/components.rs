/**********************************************
***** This file is generated, do not edit *****
***********************************************/

pub use vino_provider::prelude::*;

pub mod __multi__;
pub mod syslog5424; // syslog5424

type Result<T> = std::result::Result<T, WasmError>;

#[no_mangle]
pub(crate) extern "C" fn __guest_call(op_len: i32, req_len: i32) -> i32 {
    use std::slice;

    let buf: Vec<u8> = Vec::with_capacity(req_len as _);
    let req_ptr = buf.as_ptr();

    let opbuf: Vec<u8> = Vec::with_capacity(op_len as _);
    let op_ptr = opbuf.as_ptr();

    let (slice, op) = unsafe {
        wapc::__guest_request(op_ptr, req_ptr);
        (
            slice::from_raw_parts(req_ptr, req_len as _),
            slice::from_raw_parts(op_ptr, op_len as _),
        )
    };

    let op_str = ::std::str::from_utf8(op).unwrap();

    match Dispatcher::dispatch(op_str, slice) {
        Ok(response) => {
            unsafe { wapc::__guest_response(response.as_ptr(), response.len()) }
            1
        }
        Err(e) => {
            let errmsg = e.to_string();
            unsafe {
                wapc::__guest_error(errmsg.as_ptr(), errmsg.len() as _);
            }
            0
        }
    }
}

static ALL_COMPONENTS: &[&str] = &["syslog5424"];

pub struct Dispatcher {}
impl Dispatch for Dispatcher {
    fn dispatch(op: &str, payload: &[u8]) -> CallResult {
        let payload = IncomingPayload::from_buffer(payload)?;
        let result = match op {
            "syslog5424" => {
                crate::components::generated::syslog5424::Component::default().execute(&payload)
            }
            _ => Err(WasmError::ComponentNotFound(
                op.to_owned(),
                ALL_COMPONENTS.join(", "),
            )),
        }?;
        Ok(serialize(&result)?)
    }
}

pub mod types {
    // no additional types
}

pub mod generated {
    use super::*;

    // start namespace
    pub mod syslog5424 {
        use crate::components::syslog5424 as implementation;

        pub use vino_provider::prelude::*;

        use super::*;

        #[derive(Default)]
        pub struct Component {}

        impl WapcComponent for Component {
            fn execute(&self, payload: &IncomingPayload) -> JobResult {
                let outputs = get_outputs(payload.id());
                let inputs = populate_inputs(payload)?;
                implementation::job(inputs, outputs)
            }
        }

        fn populate_inputs(payload: &IncomingPayload) -> Result<Inputs> {
            Ok(Inputs {
                input: deserialize(payload.get("input")?)?,
            })
        }

        impl From<Inputs> for TransportMap {
            fn from(inputs: Inputs) -> TransportMap {
                let mut map = TransportMap::new();
                map.insert("input", MessageTransport::success(&inputs.input));
                map
            }
        }

        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub struct Inputs {
            #[serde(rename = "input")]
            pub input: serde_json::Value,
        }

        #[derive(Debug)]
        pub struct OutputPorts {
            pub severity: SeveritySender,
            pub facility: FacilitySender,
            pub version: VersionSender,
            pub timestamp: TimestampSender,
            pub timestamp_nanos: TimestampNanosSender,
            pub hostname: HostnameSender,
            pub appname: AppnameSender,
            pub procid: ProcidSender,
            pub procname: ProcnameSender,
            pub msgid: MsgidSender,
            pub sd: SdSender,
            pub msg: MsgSender,
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct SeveritySender {
            id: u32,
        }

        impl PortSender for SeveritySender {
            type PayloadType = String;
            fn get_name(&self) -> String {
                "severity".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct FacilitySender {
            id: u32,
        }

        impl PortSender for FacilitySender {
            type PayloadType = String;
            fn get_name(&self) -> String {
                "facility".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct VersionSender {
            id: u32,
        }

        impl PortSender for VersionSender {
            type PayloadType = i32;
            fn get_name(&self) -> String {
                "version".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct TimestampSender {
            id: u32,
        }

        impl PortSender for TimestampSender {
            type PayloadType = u32;
            fn get_name(&self) -> String {
                "timestamp".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct TimestampNanosSender {
            id: u32,
        }

        impl PortSender for TimestampNanosSender {
            type PayloadType = u32;
            fn get_name(&self) -> String {
                "timestamp_nanos".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct HostnameSender {
            id: u32,
        }

        impl PortSender for HostnameSender {
            type PayloadType = String;
            fn get_name(&self) -> String {
                "hostname".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct AppnameSender {
            id: u32,
        }

        impl PortSender for AppnameSender {
            type PayloadType = String;
            fn get_name(&self) -> String {
                "appname".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct ProcidSender {
            id: u32,
        }

        impl PortSender for ProcidSender {
            type PayloadType = u16;
            fn get_name(&self) -> String {
                "procid".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct ProcnameSender {
            id: u32,
        }

        impl PortSender for ProcnameSender {
            type PayloadType = String;
            fn get_name(&self) -> String {
                "procname".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct MsgidSender {
            id: u32,
        }

        impl PortSender for MsgidSender {
            type PayloadType = String;
            fn get_name(&self) -> String {
                "msgid".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct SdSender {
            id: u32,
        }

        impl PortSender for SdSender {
            type PayloadType = serde_json::Value;
            fn get_name(&self) -> String {
                "sd".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct MsgSender {
            id: u32,
        }

        impl PortSender for MsgSender {
            type PayloadType = String;
            fn get_name(&self) -> String {
                "msg".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }

        fn get_outputs(id: u32) -> OutputPorts {
            OutputPorts {
                severity: SeveritySender { id },
                facility: FacilitySender { id },
                version: VersionSender { id },
                timestamp: TimestampSender { id },
                timestamp_nanos: TimestampNanosSender { id },
                hostname: HostnameSender { id },
                appname: AppnameSender { id },
                procid: ProcidSender { id },
                procname: ProcnameSender { id },
                msgid: MsgidSender { id },
                sd: SdSender { id },
                msg: MsgSender { id },
            }
        }

        #[derive(Debug)]
        pub struct Outputs {
            packets: ProviderOutput,
        }

        impl Outputs {
            pub fn severity(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("severity")
                    .ok_or_else(|| ComponentError::new("No packets for port 'severity' found"))?;
                Ok(PortOutput::new("severity".to_owned(), packets))
            }
            pub fn facility(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("facility")
                    .ok_or_else(|| ComponentError::new("No packets for port 'facility' found"))?;
                Ok(PortOutput::new("facility".to_owned(), packets))
            }
            pub fn version(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("version")
                    .ok_or_else(|| ComponentError::new("No packets for port 'version' found"))?;
                Ok(PortOutput::new("version".to_owned(), packets))
            }
            pub fn timestamp(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("timestamp")
                    .ok_or_else(|| ComponentError::new("No packets for port 'timestamp' found"))?;
                Ok(PortOutput::new("timestamp".to_owned(), packets))
            }
            pub fn timestamp_nanos(&mut self) -> Result<PortOutput> {
                let packets = self.packets.take("timestamp_nanos").ok_or_else(|| {
                    ComponentError::new("No packets for port 'timestamp_nanos' found")
                })?;
                Ok(PortOutput::new("timestamp_nanos".to_owned(), packets))
            }
            pub fn hostname(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("hostname")
                    .ok_or_else(|| ComponentError::new("No packets for port 'hostname' found"))?;
                Ok(PortOutput::new("hostname".to_owned(), packets))
            }
            pub fn appname(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("appname")
                    .ok_or_else(|| ComponentError::new("No packets for port 'appname' found"))?;
                Ok(PortOutput::new("appname".to_owned(), packets))
            }
            pub fn procid(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("procid")
                    .ok_or_else(|| ComponentError::new("No packets for port 'procid' found"))?;
                Ok(PortOutput::new("procid".to_owned(), packets))
            }
            pub fn procname(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("procname")
                    .ok_or_else(|| ComponentError::new("No packets for port 'procname' found"))?;
                Ok(PortOutput::new("procname".to_owned(), packets))
            }
            pub fn msgid(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("msgid")
                    .ok_or_else(|| ComponentError::new("No packets for port 'msgid' found"))?;
                Ok(PortOutput::new("msgid".to_owned(), packets))
            }
            pub fn sd(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("sd")
                    .ok_or_else(|| ComponentError::new("No packets for port 'sd' found"))?;
                Ok(PortOutput::new("sd".to_owned(), packets))
            }
            pub fn msg(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("msg")
                    .ok_or_else(|| ComponentError::new("No packets for port 'msg' found"))?;
                Ok(PortOutput::new("msg".to_owned(), packets))
            }
        }

        impl From<ProviderOutput> for Outputs {
            fn from(packets: ProviderOutput) -> Self {
                Self { packets }
            }
        }
    }

    pub mod __multi__ {
        use super::Result;
        use crate::components::__multi__ as implementation;

        #[cfg(any(feature = "native"))]
        pub use vino_provider::native::prelude::*;
        #[cfg(any(feature = "wasm"))]
        pub use vino_provider::wasm::prelude::*;

        pub use vino_provider::prelude::*;
        #[derive(Default)]
        pub struct Component {}

        impl WapcComponent for Component {
            fn execute(&self, payload: &IncomingPayload) -> JobResult {
                let outputs = get_outputs(payload.id());
                let inputs = populate_inputs(payload)?;
                implementation::job(inputs, outputs)
            }
        }

        fn populate_inputs(payload: &IncomingPayload) -> Result<Vec<ComponentInputs>> {
            Ok(deserialize(payload.get("inputs")?)?)
        }

        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub enum ComponentInputs {
            Syslog5424(super::syslog5424::Inputs),
        }

        #[cfg(all(feature = "guest"))]
        #[allow(missing_debug_implementations)]
        pub enum ComponentOutputs {
            Syslog5424(super::syslog5424::Outputs),
        }

        #[derive(Debug)]
        pub struct OutputPorts {
            pub result: ResultSender,
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct ResultSender {
            id: u32,
        }

        impl PortSender for ResultSender {
            type PayloadType = bool;
            fn get_name(&self) -> String {
                "result".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }

        fn get_outputs(id: u32) -> OutputPorts {
            OutputPorts {
                result: ResultSender { id },
            }
        }

        #[derive(Debug)]
        pub struct Outputs {
            packets: ProviderOutput,
        }

        impl Outputs {
            pub fn result(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("result")
                    .ok_or_else(|| ComponentError::new("No packets for port 'result' found"))?;
                Ok(PortOutput::new("result".to_owned(), packets))
            }
        }

        impl From<ProviderOutput> for Outputs {
            fn from(packets: ProviderOutput) -> Self {
                Self { packets }
            }
        }
    }
}
