use std::{
    mem::{size_of, zeroed},
    net::TcpStream,
    os::windows::io::FromRawSocket,
    ptr,
};
use uuid::Uuid;
use windows::{
    core::{GUID, PWSTR},
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, NS_BTH, SOCKADDR_BTH},
        Networking::WinSock::{
            accept, bind, closesocket, getsockname, listen, WSACleanup, WSASetServiceW,
            WSASocketW, WSAStartup, CSADDR_INFO, RNRSERVICE_REGISTER, SOCKADDR,
            SOCKET_ADDRESS, SOCK_STREAM, WSADATA, WSAQUERYSETW, WSA_FLAG_OVERLAPPED,
        },
    },
};

const BT_PORT_ANY: u32 = u32::MAX;

pub fn run_rfcomm_listener(
    service_uuid: &str,
    on_ready: impl FnOnce(),
    mut on_stream: impl FnMut(TcpStream),
) -> std::io::Result<()> {
    let parsed_uuid = Uuid::parse_str(service_uuid)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;
    let mut service_guid = GUID::from_u128(parsed_uuid.as_u128());
    let mut wsa_data: WSADATA = unsafe { zeroed() };
    if unsafe { WSAStartup(0x0202, &mut wsa_data) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    let listener = match unsafe {
        WSASocketW(
            AF_BTH as i32,
            SOCK_STREAM.0,
            BTHPROTO_RFCOMM as i32,
            None,
            0,
            WSA_FLAG_OVERLAPPED,
        )
    } {
        Ok(socket) => socket,
        Err(err) => {
            unsafe { WSACleanup() };
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
        }
    };

    let result = (|| {
        let mut address = SOCKADDR_BTH {
            addressFamily: AF_BTH,
            btAddr: 0,
            serviceClassId: service_guid,
            port: BT_PORT_ANY,
        };
        if unsafe {
            bind(
                listener,
                &address as *const SOCKADDR_BTH as *const SOCKADDR,
                size_of::<SOCKADDR_BTH>() as i32,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error());
        }

        let mut address_len = size_of::<SOCKADDR_BTH>() as i32;
        if unsafe {
            getsockname(
                listener,
                &mut address as *mut SOCKADDR_BTH as *mut SOCKADDR,
                &mut address_len,
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error());
        }
        if unsafe { listen(listener, 1) } != 0 {
            return Err(std::io::Error::last_os_error());
        }

        let mut service_name: Vec<u16> = "WireWave\0".encode_utf16().collect();
        let mut cs_addr = CSADDR_INFO {
            LocalAddr: SOCKET_ADDRESS {
                lpSockaddr: &mut address as *mut SOCKADDR_BTH as *mut SOCKADDR,
                iSockaddrLength: size_of::<SOCKADDR_BTH>() as i32,
            },
            RemoteAddr: SOCKET_ADDRESS {
                lpSockaddr: ptr::null_mut(),
                iSockaddrLength: 0,
            },
            iSocketType: SOCK_STREAM.0,
            iProtocol: BTHPROTO_RFCOMM as i32,
        };
        let query = WSAQUERYSETW {
            dwSize: size_of::<WSAQUERYSETW>() as u32,
            lpszServiceInstanceName: PWSTR(service_name.as_mut_ptr()),
            lpServiceClassId: &mut service_guid,
            dwNameSpace: NS_BTH,
            dwNumberOfCsAddrs: 1,
            lpcsaBuffer: &mut cs_addr,
            ..Default::default()
        };
        if unsafe { WSASetServiceW(&query, RNRSERVICE_REGISTER, 0) } != 0 {
            return Err(std::io::Error::last_os_error());
        }

        on_ready();
        loop {
            let client = match unsafe { accept(listener, None, None) } {
                Ok(c) => c,
                Err(_) => continue,
            };
            let stream = unsafe { TcpStream::from_raw_socket(client.0 as _) };
            if stream.set_nonblocking(true).is_err() {
                continue;
            }
            on_stream(stream);
        }
    })();

    unsafe {
        closesocket(listener);
        WSACleanup();
    }
    result
}
