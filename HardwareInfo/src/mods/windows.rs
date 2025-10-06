// TODO: Windows sucks ass and i cant do this correctly

// use wmi::{COMLibrary, WMIConnection};

// pub fn get_windows_caption() -> Option<String> {
//   let com = COMLibrary::new().ok()?;
//   let wmi_con = WMIConnection::new(com.into()).ok()?;
//   let results: Vec<std::collections::HashMap<String, wmi::Variant>> =
//     wmi_con.raw_query("SELECT Caption FROM Win32_OperatingSystem").ok()?;
//   results.first()?.get("Caption")?.to_string().into()
// }
