pub use crate::components::generated::syslog5424::*;

pub(crate) fn job(input: Inputs, output: OutputPorts) -> JobResult {
    let message = input.message;
    let msg = message["msg"].to_string();
    let sd = message["sd"].to_string();
    let sd_struct = serde_json::from_str(&sd).unwrap();
    output.sd.done(&sd_struct);
    output.msg.done(&msg);
    Ok(())
}
