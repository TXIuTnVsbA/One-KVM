---
*** Begin Patch
*** Update File: src/web/handlers/devices.rs
@@
 pub async fn list_atx_devices() -> Json<AtxDevices> {
     Json(discover_devices())
 }
@@
 #[cfg(unix)]
 pub async fn list_network_interfaces() -> Result<Json<Vec<crate::otg::bridge::NetworkInterfaceInfo>>>
 {
     crate::otg::bridge::list_network_interfaces().map(Json)
 }
+
+#[cfg(not(unix))]
+pub async fn list_network_interfaces() -> Json<Vec<serde_json::Value>> {
+    // On non-Unix platforms report an empty interface list so frontend doesn't error.
+    Json(vec![])
+}
*** End Patch
