use std::collections::HashMap;

/// Hardware & System API Permission Types
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionType {
    Bluetooth,
    WebRTC,
    USBDevice,
    NetworkSocket,
    CameraMicrophone,
}

impl PermissionType {
    pub fn display_name(&self) -> &'static str {
        match self {
            PermissionType::Bluetooth => "Bluetooth Wireless Hardware",
            PermissionType::WebRTC => "WebRTC Real-time Audio/Video",
            PermissionType::USBDevice => "Direct USB Hardware Device",
            PermissionType::NetworkSocket => "Low-Level Network Socket Streaming",
            PermissionType::CameraMicrophone => "Camera & Microphone Access",
        }
    }
}

/// Runtime End-User Permission Guard System
/// Enforces explicit user consent before granting hardware access.
/// If end-user denies, the subsystem is safely disabled at runtime without crashing.
pub struct PermissionEngine {
    pub permissions: HashMap<PermissionType, bool>,
    pub pending_prompt: Option<PermissionType>,
}

impl PermissionEngine {
    pub fn new() -> Self {
        log::info!("Permission Engine initialized (Runtime User Consent Security Guard)");
        Self {
            permissions: HashMap::new(),
            pending_prompt: None,
        }
    }

    /// Check if permission is granted by the end-user
    #[allow(dead_code)]
    pub fn is_granted(&self, perm: PermissionType) -> bool {
        *self.permissions.get(&perm).unwrap_or(&false)
    }

    /// Request permission at runtime (triggers Native Dialog Prompt)
    pub fn request_permission(&mut self, perm: PermissionType) {
        if !self.permissions.contains_key(&perm) {
            self.pending_prompt = Some(perm);
        }
    }

    /// User responded to permission dialog prompt
    pub fn respond_permission(&mut self, perm: PermissionType, granted: bool) {
        self.permissions.insert(perm, granted);
        if self.pending_prompt == Some(perm) {
            self.pending_prompt = None;
        }
        log::info!("Permission for {:?} set to: {}", perm, granted);
    }
}
