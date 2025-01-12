use crate::constants::WSA_VERSION_2_2;
use crate::functions::{WSACleanup, WSAStartup};
use crate::structs::WsaData;

/*
    Convenience functions for (de)initialization of WinSock
*/
pub struct WinSock;
impl WinSock {
    pub fn startup() {
        let mut wsa_data: WsaData = WsaData::default();
        unsafe{ WSAStartup(WSA_VERSION_2_2, &mut wsa_data as *mut WsaData); }
    }

    pub fn cleanup() {
        unsafe { WSACleanup(); }
    }
}