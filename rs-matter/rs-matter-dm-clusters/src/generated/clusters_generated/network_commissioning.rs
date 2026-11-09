# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const WI_FI_NETWORK_INTERFACE = 1 ;
        const THREAD_NETWORK_INTERFACE = 2 ;
        const ETHERNET_NETWORK_INTERFACE = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const WI_FI_NETWORK_INTERFACE = 1 ;
        const THREAD_NETWORK_INTERFACE = 2 ;
        const ETHERNET_NETWORK_INTERFACE = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ThreadCapabilitiesBitmap : u16 {
        const IS_BORDER_ROUTER_CAPABLE = 1 ;
        const IS_ROUTER_CAPABLE = 2 ;
        const IS_SLEEPY_END_DEVICE_CAPABLE = 4 ;
        const IS_FULL_THREAD_DEVICE = 8 ;
        const IS_SYNCHRONIZED_SLEEPY_END_DEVICE_CAPABLE = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ThreadCapabilitiesBitmap : u16 {
        const IS_BORDER_ROUTER_CAPABLE = 1 ;
        const IS_ROUTER_CAPABLE = 2 ;
        const IS_SLEEPY_END_DEVICE_CAPABLE = 4 ;
        const IS_FULL_THREAD_DEVICE = 8 ;
        const IS_SYNCHRONIZED_SLEEPY_END_DEVICE_CAPABLE = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ThreadCapabilitiesBitmap , u16 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct WiFiSecurityBitmap : u8 {
        const UNENCRYPTED = 1 ;
        const WEP = 2 ;
        const WPA_PERSONAL = 4 ;
        const WPA_2_PERSONAL = 8 ;
        const WPA_3_PERSONAL = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct WiFiSecurityBitmap : u8 {
        const UNENCRYPTED = 1 ;
        const WEP = 2 ;
        const WPA_PERSONAL = 4 ;
        const WPA_2_PERSONAL = 8 ;
        const WPA_3_PERSONAL = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (WiFiSecurityBitmap , u8 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkCommissioningStatusEnum {
    # [enumval (0 ) ] Success = 0 , # [enumval (1 ) ] OutOfRange = 1 , # [enumval (2 ) ] BoundsExceeded = 2 , # [enumval (3 ) ] NetworkIDNotFound = 3 , # [enumval (4 ) ] DuplicateNetworkID = 4 , # [enumval (5 ) ] NetworkNotFound = 5 , # [enumval (6 ) ] RegulatoryError = 6 , # [enumval (7 ) ] AuthFailure = 7 , # [enumval (8 ) ] UnsupportedSecurity = 8 , # [enumval (9 ) ] OtherConnectionFailure = 9 , # [enumval (10 ) ] IPV6Failed = 10 , # [enumval (11 ) ] IPBindFailed = 11 , # [enumval (12 ) ] UnknownError = 12 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum WiFiBandEnum {
    # [enumval (0 ) ] V2G4 = 0 , # [enumval (1 ) ] V3G65 = 1 , # [enumval (2 ) ] V5G = 2 , # [enumval (3 ) ] V6G = 3 , # [enumval (4 ) ] V60G = 4 , # [enumval (5 ) ] V1G = 5 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkInfoStructTag {
    NetworkId = 0 , Connected = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ThreadInterfaceScanResultStructTag {
    PanId = 0 , ExtendedPanId = 1 , NetworkName = 2 , Channel = 3 , Version = 4 , ExtendedAddress = 5 , Rssi = 6 , Lqi = 7 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum WiFiInterfaceScanResultStructTag {
    Security = 0 , Ssid = 1 , Bssid = 2 , Channel = 3 , WiFiBand = 4 , Rssi = 5 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ScanNetworksRequestTag {
    Ssid = 0 , Breadcrumb = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ScanNetworksResponseTag {
    NetworkingStatus = 0 , DebugText = 1 , WiFiScanResults = 2 , ThreadScanResults = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AddOrUpdateWiFiNetworkRequestTag {
    Ssid = 0 , Credentials = 1 , Breadcrumb = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AddOrUpdateThreadNetworkRequestTag {
    OperationalDataset = 0 , Breadcrumb = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RemoveNetworkRequestTag {
    NetworkId = 0 , Breadcrumb = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkConfigResponseTag {
    NetworkingStatus = 0 , DebugText = 1 , NetworkIndex = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ConnectNetworkRequestTag {
    NetworkId = 0 , Breadcrumb = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ConnectNetworkResponseTag {
    NetworkingStatus = 0 , DebugText = 1 , ErrorValue = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ReorderNetworkRequestTag {
    NetworkId = 0 , NetworkIndex = 1 , Breadcrumb = 2 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct NetworkInfoStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > NetworkInfoStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn network_id (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn connected (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for NetworkInfoStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for NetworkInfoStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for NetworkInfoStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "NetworkInfoStruct" ) ? ;
        match self . network_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "network_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) ? , 
        }
        match self . connected () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connected" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connected" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for NetworkInfoStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "NetworkInfoStruct" ) ;
        match self . network_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "network_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) , 
        }
        match self . connected () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connected" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connected" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ThreadInterfaceScanResultStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ThreadInterfaceScanResultStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn pan_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn extended_pan_id (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn network_name (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn channel (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn version (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn extended_address (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn rssi (& self ) -> Result < i8 , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn lqi (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ThreadInterfaceScanResultStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ThreadInterfaceScanResultStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ThreadInterfaceScanResultStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ThreadInterfaceScanResultStruct" ) ? ;
        match self . pan_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "pan_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "pan_id" , e . code () ) ? , 
        }
        match self . extended_pan_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "extended_pan_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "extended_pan_id" , e . code () ) ? , 
        }
        match self . network_name () {
            Ok (value ) => write ! (f , "{}: {:?}," , "network_name" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_name" , e . code () ) ? , 
        }
        match self . channel () {
            Ok (value ) => write ! (f , "{}: {:?}," , "channel" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel" , e . code () ) ? , 
        }
        match self . version () {
            Ok (value ) => write ! (f , "{}: {:?}," , "version" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "version" , e . code () ) ? , 
        }
        match self . extended_address () {
            Ok (value ) => write ! (f , "{}: {:?}," , "extended_address" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "extended_address" , e . code () ) ? , 
        }
        match self . rssi () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rssi" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rssi" , e . code () ) ? , 
        }
        match self . lqi () {
            Ok (value ) => write ! (f , "{}: {:?}," , "lqi" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "lqi" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ThreadInterfaceScanResultStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ThreadInterfaceScanResultStruct" ) ;
        match self . pan_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "pan_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "pan_id" , e . code () ) , 
        }
        match self . extended_pan_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "extended_pan_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "extended_pan_id" , e . code () ) , 
        }
        match self . network_name () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "network_name" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_name" , e . code () ) , 
        }
        match self . channel () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "channel" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel" , e . code () ) , 
        }
        match self . version () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "version" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "version" , e . code () ) , 
        }
        match self . extended_address () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "extended_address" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "extended_address" , e . code () ) , 
        }
        match self . rssi () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rssi" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rssi" , e . code () ) , 
        }
        match self . lqi () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "lqi" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "lqi" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct WiFiInterfaceScanResultStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > WiFiInterfaceScanResultStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn security (& self ) -> Result < WiFiSecurityBitmap , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn ssid (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn bssid (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn channel (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn wi_fi_band (& self ) -> Result < WiFiBandEnum , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn rssi (& self ) -> Result < i8 , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for WiFiInterfaceScanResultStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for WiFiInterfaceScanResultStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for WiFiInterfaceScanResultStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "WiFiInterfaceScanResultStruct" ) ? ;
        match self . security () {
            Ok (value ) => write ! (f , "{}: {:?}," , "security" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "security" , e . code () ) ? , 
        }
        match self . ssid () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ssid" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ssid" , e . code () ) ? , 
        }
        match self . bssid () {
            Ok (value ) => write ! (f , "{}: {:?}," , "bssid" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "bssid" , e . code () ) ? , 
        }
        match self . channel () {
            Ok (value ) => write ! (f , "{}: {:?}," , "channel" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel" , e . code () ) ? , 
        }
        match self . wi_fi_band () {
            Ok (value ) => write ! (f , "{}: {:?}," , "wi_fi_band" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "wi_fi_band" , e . code () ) ? , 
        }
        match self . rssi () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rssi" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rssi" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for WiFiInterfaceScanResultStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "WiFiInterfaceScanResultStruct" ) ;
        match self . security () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "security" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "security" , e . code () ) , 
        }
        match self . ssid () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ssid" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ssid" , e . code () ) , 
        }
        match self . bssid () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "bssid" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "bssid" , e . code () ) , 
        }
        match self . channel () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "channel" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel" , e . code () ) , 
        }
        match self . wi_fi_band () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "wi_fi_band" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "wi_fi_band" , e . code () ) , 
        }
        match self . rssi () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rssi" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rssi" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ScanNetworksRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ScanNetworksRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn ssid (& self ) -> Result < Option < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn breadcrumb (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ScanNetworksRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ScanNetworksRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ScanNetworksRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ScanNetworksRequest" ) ? ;
        match self . ssid () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "ssid" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "ssid" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ssid" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "breadcrumb" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ScanNetworksRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ScanNetworksRequest" ) ;
        match self . ssid () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "ssid" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "ssid" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ssid" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "breadcrumb" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ScanNetworksResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ScanNetworksResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn networking_status (& self ) -> Result < NetworkCommissioningStatusEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn debug_text (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn wi_fi_scan_results (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , WiFiInterfaceScanResultStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn thread_scan_results (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , ThreadInterfaceScanResultStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ScanNetworksResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ScanNetworksResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ScanNetworksResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ScanNetworksResponse" ) ? ;
        match self . networking_status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "networking_status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "networking_status" , e . code () ) ? , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "debug_text" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "debug_text" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) ? , 
        }
        match self . wi_fi_scan_results () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "wi_fi_scan_results" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "wi_fi_scan_results" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "wi_fi_scan_results" , e . code () ) ? , 
        }
        match self . thread_scan_results () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "thread_scan_results" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "thread_scan_results" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "thread_scan_results" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ScanNetworksResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ScanNetworksResponse" ) ;
        match self . networking_status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "networking_status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "networking_status" , e . code () ) , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "debug_text" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "debug_text" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) , 
        }
        match self . wi_fi_scan_results () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "wi_fi_scan_results" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "wi_fi_scan_results" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "wi_fi_scan_results" , e . code () ) , 
        }
        match self . thread_scan_results () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "thread_scan_results" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "thread_scan_results" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "thread_scan_results" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AddOrUpdateWiFiNetworkRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AddOrUpdateWiFiNetworkRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn ssid (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn credentials (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn breadcrumb (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AddOrUpdateWiFiNetworkRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AddOrUpdateWiFiNetworkRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AddOrUpdateWiFiNetworkRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AddOrUpdateWiFiNetworkRequest" ) ? ;
        match self . ssid () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ssid" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ssid" , e . code () ) ? , 
        }
        match self . credentials () {
            Ok (value ) => write ! (f , "{}: {:?}," , "credentials" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "credentials" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "breadcrumb" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AddOrUpdateWiFiNetworkRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AddOrUpdateWiFiNetworkRequest" ) ;
        match self . ssid () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ssid" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ssid" , e . code () ) , 
        }
        match self . credentials () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "credentials" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "credentials" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "breadcrumb" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AddOrUpdateThreadNetworkRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AddOrUpdateThreadNetworkRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn operational_dataset (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn breadcrumb (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AddOrUpdateThreadNetworkRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AddOrUpdateThreadNetworkRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AddOrUpdateThreadNetworkRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AddOrUpdateThreadNetworkRequest" ) ? ;
        match self . operational_dataset () {
            Ok (value ) => write ! (f , "{}: {:?}," , "operational_dataset" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "operational_dataset" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "breadcrumb" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AddOrUpdateThreadNetworkRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AddOrUpdateThreadNetworkRequest" ) ;
        match self . operational_dataset () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "operational_dataset" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "operational_dataset" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "breadcrumb" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RemoveNetworkRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RemoveNetworkRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn network_id (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn breadcrumb (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RemoveNetworkRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RemoveNetworkRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RemoveNetworkRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RemoveNetworkRequest" ) ? ;
        match self . network_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "network_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "breadcrumb" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RemoveNetworkRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RemoveNetworkRequest" ) ;
        match self . network_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "network_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "breadcrumb" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct NetworkConfigResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > NetworkConfigResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn networking_status (& self ) -> Result < NetworkCommissioningStatusEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn debug_text (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn network_index (& self ) -> Result < Option < u8 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for NetworkConfigResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for NetworkConfigResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for NetworkConfigResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "NetworkConfigResponse" ) ? ;
        match self . networking_status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "networking_status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "networking_status" , e . code () ) ? , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "debug_text" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "debug_text" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) ? , 
        }
        match self . network_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "network_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "network_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for NetworkConfigResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "NetworkConfigResponse" ) ;
        match self . networking_status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "networking_status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "networking_status" , e . code () ) , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "debug_text" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "debug_text" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) , 
        }
        match self . network_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "network_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "network_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ConnectNetworkRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ConnectNetworkRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn network_id (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn breadcrumb (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ConnectNetworkRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ConnectNetworkRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ConnectNetworkRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ConnectNetworkRequest" ) ? ;
        match self . network_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "network_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "breadcrumb" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ConnectNetworkRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ConnectNetworkRequest" ) ;
        match self . network_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "network_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "breadcrumb" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ConnectNetworkResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ConnectNetworkResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn networking_status (& self ) -> Result < NetworkCommissioningStatusEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn debug_text (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn error_value (& self ) -> Result < crate :: tlv :: Nullable < i32 > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ConnectNetworkResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ConnectNetworkResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ConnectNetworkResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ConnectNetworkResponse" ) ? ;
        match self . networking_status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "networking_status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "networking_status" , e . code () ) ? , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "debug_text" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "debug_text" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) ? , 
        }
        match self . error_value () {
            Ok (value ) => write ! (f , "{}: {:?}," , "error_value" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "error_value" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ConnectNetworkResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ConnectNetworkResponse" ) ;
        match self . networking_status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "networking_status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "networking_status" , e . code () ) , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "debug_text" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "debug_text" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) , 
        }
        match self . error_value () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "error_value" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "error_value" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ReorderNetworkRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ReorderNetworkRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn network_id (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn network_index (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn breadcrumb (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ReorderNetworkRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ReorderNetworkRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ReorderNetworkRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ReorderNetworkRequest" ) ? ;
        match self . network_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "network_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) ? , 
        }
        match self . network_index () {
            Ok (value ) => write ! (f , "{}: {:?}," , "network_index" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_index" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "breadcrumb" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ReorderNetworkRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ReorderNetworkRequest" ) ;
        match self . network_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "network_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_id" , e . code () ) , 
        }
        match self . network_index () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "network_index" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_index" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "breadcrumb" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "breadcrumb" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct NetworkInfoStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > NetworkInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > NetworkInfoStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_id (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < NetworkInfoStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (NetworkInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInfoStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connected (mut self , value : bool ) -> Result < NetworkInfoStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connected" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (NetworkInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInfoStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for NetworkInfoStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkInfoStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for NetworkInfoStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkInfoStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for NetworkInfoStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NetworkInfoStructArrayBuilder < P > (P ) ;
impl < P > NetworkInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < NetworkInfoStructBuilder < NetworkInfoStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkInfoStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for NetworkInfoStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkInfoStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for NetworkInfoStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkInfoStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for NetworkInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ThreadInterfaceScanResultStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ThreadInterfaceScanResultStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn pan_id (mut self , value : u16 ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "panId" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn extended_pan_id (mut self , value : u64 ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "extendedPanId" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_name (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkName" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel (mut self , value : u16 ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "channel" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn version (mut self , value : u8 ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "version" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn extended_address (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "extendedAddress" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rssi (mut self , value : i8 ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rssi" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn lqi (mut self , value : u8 ) -> Result < ThreadInterfaceScanResultStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "lqi" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (ThreadInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ThreadInterfaceScanResultStructBuilder < P , 8usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ThreadInterfaceScanResultStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ThreadInterfaceScanResultStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ThreadInterfaceScanResultStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ThreadInterfaceScanResultStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ThreadInterfaceScanResultStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ThreadInterfaceScanResultStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ThreadInterfaceScanResultStructArrayBuilder < P > (P ) ;
impl < P > ThreadInterfaceScanResultStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ThreadInterfaceScanResultStructBuilder < ThreadInterfaceScanResultStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ThreadInterfaceScanResultStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ThreadInterfaceScanResultStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ThreadInterfaceScanResultStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ThreadInterfaceScanResultStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ThreadInterfaceScanResultStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ThreadInterfaceScanResultStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ThreadInterfaceScanResultStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct WiFiInterfaceScanResultStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > WiFiInterfaceScanResultStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > WiFiInterfaceScanResultStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn security (mut self , value : WiFiSecurityBitmap ) -> Result < WiFiInterfaceScanResultStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "security" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (WiFiInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WiFiInterfaceScanResultStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ssid (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < WiFiInterfaceScanResultStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ssid" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (WiFiInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WiFiInterfaceScanResultStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn bssid (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < WiFiInterfaceScanResultStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "bssid" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (WiFiInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WiFiInterfaceScanResultStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel (mut self , value : u16 ) -> Result < WiFiInterfaceScanResultStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "channel" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (WiFiInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WiFiInterfaceScanResultStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn wi_fi_band (mut self , value : WiFiBandEnum ) -> Result < WiFiInterfaceScanResultStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "wiFiBand" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (WiFiInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WiFiInterfaceScanResultStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rssi (mut self , value : i8 ) -> Result < WiFiInterfaceScanResultStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rssi" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (WiFiInterfaceScanResultStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > WiFiInterfaceScanResultStructBuilder < P , 6usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for WiFiInterfaceScanResultStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WiFiInterfaceScanResultStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for WiFiInterfaceScanResultStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WiFiInterfaceScanResultStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for WiFiInterfaceScanResultStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WiFiInterfaceScanResultStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct WiFiInterfaceScanResultStructArrayBuilder < P > (P ) ;
impl < P > WiFiInterfaceScanResultStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < WiFiInterfaceScanResultStructBuilder < WiFiInterfaceScanResultStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WiFiInterfaceScanResultStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for WiFiInterfaceScanResultStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WiFiInterfaceScanResultStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for WiFiInterfaceScanResultStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WiFiInterfaceScanResultStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for WiFiInterfaceScanResultStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WiFiInterfaceScanResultStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScanNetworksRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ScanNetworksRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ScanNetworksRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ssid (mut self , value : Option < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > > ) -> Result < ScanNetworksRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ssid" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ScanNetworksRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScanNetworksRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : Option < u64 > ) -> Result < ScanNetworksRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ScanNetworksRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScanNetworksRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ScanNetworksRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ScanNetworksRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ScanNetworksRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScanNetworksRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScanNetworksRequestArrayBuilder < P > (P ) ;
impl < P > ScanNetworksRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ScanNetworksRequestBuilder < ScanNetworksRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScanNetworksRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ScanNetworksRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ScanNetworksRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ScanNetworksRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScanNetworksRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScanNetworksResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ScanNetworksResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ScanNetworksResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn networking_status (mut self , value : NetworkCommissioningStatusEnum ) -> Result < ScanNetworksResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkingStatus" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ScanNetworksResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScanNetworksResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn debug_text (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ScanNetworksResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "debugText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ScanNetworksResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ScanNetworksResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn wi_fi_scan_results (self ) -> Result < crate :: tlv :: OptionalBuilder < ScanNetworksResponseBuilder < P , 3usize > , WiFiInterfaceScanResultStructArrayBuilder < ScanNetworksResponseBuilder < P , 3usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScanNetworksResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > ScanNetworksResponseBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn thread_scan_results (self ) -> Result < crate :: tlv :: OptionalBuilder < ScanNetworksResponseBuilder < P , 4usize > , ThreadInterfaceScanResultStructArrayBuilder < ScanNetworksResponseBuilder < P , 4usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScanNetworksResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > ScanNetworksResponseBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ScanNetworksResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ScanNetworksResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ScanNetworksResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScanNetworksResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ScanNetworksResponseArrayBuilder < P > (P ) ;
impl < P > ScanNetworksResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ScanNetworksResponseBuilder < ScanNetworksResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ScanNetworksResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ScanNetworksResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ScanNetworksResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ScanNetworksResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ScanNetworksResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ScanNetworksResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddOrUpdateWiFiNetworkRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AddOrUpdateWiFiNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AddOrUpdateWiFiNetworkRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ssid (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AddOrUpdateWiFiNetworkRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ssid" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AddOrUpdateWiFiNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddOrUpdateWiFiNetworkRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn credentials (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AddOrUpdateWiFiNetworkRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "credentials" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AddOrUpdateWiFiNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddOrUpdateWiFiNetworkRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : Option < u64 > ) -> Result < AddOrUpdateWiFiNetworkRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AddOrUpdateWiFiNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddOrUpdateWiFiNetworkRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AddOrUpdateWiFiNetworkRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateWiFiNetworkRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AddOrUpdateWiFiNetworkRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateWiFiNetworkRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AddOrUpdateWiFiNetworkRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddOrUpdateWiFiNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddOrUpdateWiFiNetworkRequestArrayBuilder < P > (P ) ;
impl < P > AddOrUpdateWiFiNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AddOrUpdateWiFiNetworkRequestBuilder < AddOrUpdateWiFiNetworkRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AddOrUpdateWiFiNetworkRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AddOrUpdateWiFiNetworkRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateWiFiNetworkRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AddOrUpdateWiFiNetworkRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateWiFiNetworkRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AddOrUpdateWiFiNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddOrUpdateWiFiNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddOrUpdateThreadNetworkRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AddOrUpdateThreadNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AddOrUpdateThreadNetworkRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn operational_dataset (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AddOrUpdateThreadNetworkRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "operationalDataset" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AddOrUpdateThreadNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddOrUpdateThreadNetworkRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : Option < u64 > ) -> Result < AddOrUpdateThreadNetworkRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AddOrUpdateThreadNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddOrUpdateThreadNetworkRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AddOrUpdateThreadNetworkRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateThreadNetworkRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AddOrUpdateThreadNetworkRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateThreadNetworkRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AddOrUpdateThreadNetworkRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddOrUpdateThreadNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddOrUpdateThreadNetworkRequestArrayBuilder < P > (P ) ;
impl < P > AddOrUpdateThreadNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AddOrUpdateThreadNetworkRequestBuilder < AddOrUpdateThreadNetworkRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AddOrUpdateThreadNetworkRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AddOrUpdateThreadNetworkRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateThreadNetworkRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AddOrUpdateThreadNetworkRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddOrUpdateThreadNetworkRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AddOrUpdateThreadNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddOrUpdateThreadNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RemoveNetworkRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RemoveNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RemoveNetworkRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_id (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < RemoveNetworkRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RemoveNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > RemoveNetworkRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : Option < u64 > ) -> Result < RemoveNetworkRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (RemoveNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > RemoveNetworkRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RemoveNetworkRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RemoveNetworkRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RemoveNetworkRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RemoveNetworkRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RemoveNetworkRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RemoveNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RemoveNetworkRequestArrayBuilder < P > (P ) ;
impl < P > RemoveNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RemoveNetworkRequestBuilder < RemoveNetworkRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RemoveNetworkRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RemoveNetworkRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RemoveNetworkRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RemoveNetworkRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RemoveNetworkRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RemoveNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RemoveNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NetworkConfigResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > NetworkConfigResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > NetworkConfigResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn networking_status (mut self , value : NetworkCommissioningStatusEnum ) -> Result < NetworkConfigResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkingStatus" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (NetworkConfigResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkConfigResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn debug_text (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < NetworkConfigResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "debugText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (NetworkConfigResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkConfigResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_index (mut self , value : Option < u8 > ) -> Result < NetworkConfigResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (NetworkConfigResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkConfigResponseBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for NetworkConfigResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkConfigResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for NetworkConfigResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkConfigResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for NetworkConfigResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkConfigResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NetworkConfigResponseArrayBuilder < P > (P ) ;
impl < P > NetworkConfigResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < NetworkConfigResponseBuilder < NetworkConfigResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkConfigResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for NetworkConfigResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkConfigResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for NetworkConfigResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkConfigResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for NetworkConfigResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkConfigResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ConnectNetworkRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ConnectNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ConnectNetworkRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_id (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < ConnectNetworkRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ConnectNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConnectNetworkRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : Option < u64 > ) -> Result < ConnectNetworkRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ConnectNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConnectNetworkRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ConnectNetworkRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ConnectNetworkRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ConnectNetworkRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConnectNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ConnectNetworkRequestArrayBuilder < P > (P ) ;
impl < P > ConnectNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ConnectNetworkRequestBuilder < ConnectNetworkRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ConnectNetworkRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ConnectNetworkRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ConnectNetworkRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ConnectNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConnectNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ConnectNetworkResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ConnectNetworkResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ConnectNetworkResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn networking_status (mut self , value : NetworkCommissioningStatusEnum ) -> Result < ConnectNetworkResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkingStatus" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ConnectNetworkResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConnectNetworkResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn debug_text (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ConnectNetworkResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "debugText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ConnectNetworkResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConnectNetworkResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn error_value (mut self , value : crate :: tlv :: Nullable < i32 > ) -> Result < ConnectNetworkResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "errorValue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ConnectNetworkResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConnectNetworkResponseBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ConnectNetworkResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ConnectNetworkResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ConnectNetworkResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConnectNetworkResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ConnectNetworkResponseArrayBuilder < P > (P ) ;
impl < P > ConnectNetworkResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ConnectNetworkResponseBuilder < ConnectNetworkResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ConnectNetworkResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ConnectNetworkResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ConnectNetworkResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConnectNetworkResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ConnectNetworkResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConnectNetworkResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ReorderNetworkRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ReorderNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ReorderNetworkRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_id (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < ReorderNetworkRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ReorderNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ReorderNetworkRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_index (mut self , value : u8 ) -> Result < ReorderNetworkRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ReorderNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ReorderNetworkRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : Option < u64 > ) -> Result < ReorderNetworkRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ReorderNetworkRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ReorderNetworkRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ReorderNetworkRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ReorderNetworkRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ReorderNetworkRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ReorderNetworkRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ReorderNetworkRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ReorderNetworkRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ReorderNetworkRequestArrayBuilder < P > (P ) ;
impl < P > ReorderNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ReorderNetworkRequestBuilder < ReorderNetworkRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ReorderNetworkRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ReorderNetworkRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ReorderNetworkRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ReorderNetworkRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ReorderNetworkRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ReorderNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ReorderNetworkRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    MaxNetworks = 0 , Networks = 1 , ScanMaxTimeSeconds = 2 , ConnectMaxTimeSeconds = 3 , InterfaceEnabled = 4 , LastNetworkingStatus = 5 , LastNetworkID = 6 , LastConnectErrorValue = 7 , SupportedWiFiBands = 8 , SupportedThreadFeatures = 9 , ThreadVersion = 10 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
}
impl core :: convert :: TryFrom < crate :: dm :: AttrId > for AttributeId {
    type Error = crate :: error :: Error ;
    fn try_from (id : crate :: dm :: AttrId ) -> Result < Self , Self :: Error > {
        AttributeId :: from_repr (id ) . ok_or_else (|| crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    
}
impl core :: fmt :: Debug for MetadataDebug < (AttributeId , bool ) > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Attr::" ) ? ;
        match self . 0.0 {
            AttributeId :: MaxNetworks => write ! (f , "{}(0x{:02x})" , "MaxNetworks" , AttributeId :: MaxNetworks as u32 ) ? , AttributeId :: Networks => write ! (f , "{}(0x{:02x})" , "Networks" , AttributeId :: Networks as u32 ) ? , AttributeId :: ScanMaxTimeSeconds => write ! (f , "{}(0x{:02x})" , "ScanMaxTimeSeconds" , AttributeId :: ScanMaxTimeSeconds as u32 ) ? , AttributeId :: ConnectMaxTimeSeconds => write ! (f , "{}(0x{:02x})" , "ConnectMaxTimeSeconds" , AttributeId :: ConnectMaxTimeSeconds as u32 ) ? , AttributeId :: InterfaceEnabled => write ! (f , "{}(0x{:02x})" , "InterfaceEnabled" , AttributeId :: InterfaceEnabled as u32 ) ? , AttributeId :: LastNetworkingStatus => write ! (f , "{}(0x{:02x})" , "LastNetworkingStatus" , AttributeId :: LastNetworkingStatus as u32 ) ? , AttributeId :: LastNetworkID => write ! (f , "{}(0x{:02x})" , "LastNetworkID" , AttributeId :: LastNetworkID as u32 ) ? , AttributeId :: LastConnectErrorValue => write ! (f , "{}(0x{:02x})" , "LastConnectErrorValue" , AttributeId :: LastConnectErrorValue as u32 ) ? , AttributeId :: SupportedWiFiBands => write ! (f , "{}(0x{:02x})" , "SupportedWiFiBands" , AttributeId :: SupportedWiFiBands as u32 ) ? , AttributeId :: SupportedThreadFeatures => write ! (f , "{}(0x{:02x})" , "SupportedThreadFeatures" , AttributeId :: SupportedThreadFeatures as u32 ) ? , AttributeId :: ThreadVersion => write ! (f , "{}(0x{:02x})" , "ThreadVersion" , AttributeId :: ThreadVersion as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
        }
        if self . 0.1 {
            write ! (f , "::Write" ) 
        }
        else {
            write ! (f , "::Read" ) 
        }
        
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < (AttributeId , bool ) > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Attr::" ) ;
        match self . 0.0 {
            AttributeId :: MaxNetworks => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxNetworks" , AttributeId :: MaxNetworks as u32 ) , AttributeId :: Networks => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Networks" , AttributeId :: Networks as u32 ) , AttributeId :: ScanMaxTimeSeconds => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ScanMaxTimeSeconds" , AttributeId :: ScanMaxTimeSeconds as u32 ) , AttributeId :: ConnectMaxTimeSeconds => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ConnectMaxTimeSeconds" , AttributeId :: ConnectMaxTimeSeconds as u32 ) , AttributeId :: InterfaceEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "InterfaceEnabled" , AttributeId :: InterfaceEnabled as u32 ) , AttributeId :: LastNetworkingStatus => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LastNetworkingStatus" , AttributeId :: LastNetworkingStatus as u32 ) , AttributeId :: LastNetworkID => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LastNetworkID" , AttributeId :: LastNetworkID as u32 ) , AttributeId :: LastConnectErrorValue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LastConnectErrorValue" , AttributeId :: LastConnectErrorValue as u32 ) , AttributeId :: SupportedWiFiBands => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SupportedWiFiBands" , AttributeId :: SupportedWiFiBands as u32 ) , AttributeId :: SupportedThreadFeatures => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SupportedThreadFeatures" , AttributeId :: SupportedThreadFeatures as u32 ) , AttributeId :: ThreadVersion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ThreadVersion" , AttributeId :: ThreadVersion as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
        }
        if self . 0.1 {
            crate :: reexport :: defmt :: write ! (f , "::Write" ) 
        }
        else {
            crate :: reexport :: defmt :: write ! (f , "::Read" ) 
        }
        
    }
    
}
# [doc = "The command IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandId {
    ScanNetworks = 0 , AddOrUpdateWiFiNetwork = 2 , AddOrUpdateThreadNetwork = 3 , RemoveNetwork = 4 , ConnectNetwork = 6 , ReorderNetwork = 8 
}
impl core :: convert :: TryFrom < crate :: dm :: CmdId > for CommandId {
    type Error = crate :: error :: Error ;
    fn try_from (id : crate :: dm :: CmdId ) -> Result < Self , Self :: Error > {
        CommandId :: from_repr (id ) . ok_or_else (|| crate :: error :: ErrorCode :: CommandNotFound . into () ) 
    }
    
}
impl core :: fmt :: Debug for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandId :: ScanNetworks => write ! (f , "{}(0x{:02x})" , "ScanNetworks" , CommandId :: ScanNetworks as u32 ) ? , CommandId :: AddOrUpdateWiFiNetwork => write ! (f , "{}(0x{:02x})" , "AddOrUpdateWiFiNetwork" , CommandId :: AddOrUpdateWiFiNetwork as u32 ) ? , CommandId :: AddOrUpdateThreadNetwork => write ! (f , "{}(0x{:02x})" , "AddOrUpdateThreadNetwork" , CommandId :: AddOrUpdateThreadNetwork as u32 ) ? , CommandId :: RemoveNetwork => write ! (f , "{}(0x{:02x})" , "RemoveNetwork" , CommandId :: RemoveNetwork as u32 ) ? , CommandId :: ConnectNetwork => write ! (f , "{}(0x{:02x})" , "ConnectNetwork" , CommandId :: ConnectNetwork as u32 ) ? , CommandId :: ReorderNetwork => write ! (f , "{}(0x{:02x})" , "ReorderNetwork" , CommandId :: ReorderNetwork as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: ScanNetworks => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ScanNetworks" , CommandId :: ScanNetworks as u32 ) , CommandId :: AddOrUpdateWiFiNetwork => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AddOrUpdateWiFiNetwork" , CommandId :: AddOrUpdateWiFiNetwork as u32 ) , CommandId :: AddOrUpdateThreadNetwork => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AddOrUpdateThreadNetwork" , CommandId :: AddOrUpdateThreadNetwork as u32 ) , CommandId :: RemoveNetwork => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RemoveNetwork" , CommandId :: RemoveNetwork as u32 ) , CommandId :: ConnectNetwork => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ConnectNetwork" , CommandId :: ConnectNetwork as u32 ) , CommandId :: ReorderNetwork => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ReorderNetwork" , CommandId :: ReorderNetwork as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    ScanNetworksResponse = 1 , NetworkConfigResponse = 5 , ConnectNetworkResponse = 7 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: ScanNetworksResponse => write ! (f , "{}(0x{:02x})" , "ScanNetworksResponse" , CommandResponseId :: ScanNetworksResponse as u32 ) ? , CommandResponseId :: NetworkConfigResponse => write ! (f , "{}(0x{:02x})" , "NetworkConfigResponse" , CommandResponseId :: NetworkConfigResponse as u32 ) ? , CommandResponseId :: ConnectNetworkResponse => write ! (f , "{}(0x{:02x})" , "ConnectNetworkResponse" , CommandResponseId :: ConnectNetworkResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: ScanNetworksResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ScanNetworksResponse" , CommandResponseId :: ScanNetworksResponse as u32 ) , CommandResponseId :: NetworkConfigResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NetworkConfigResponse" , CommandResponseId :: NetworkConfigResponse as u32 ) , CommandResponseId :: ConnectNetworkResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ConnectNetworkResponse" , CommandResponseId :: ConnectNetworkResponse as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Response" ) 
    }
    
}
impl core :: convert :: TryFrom < crate :: dm :: CmdId > for CommandResponseId {
    type Error = crate :: error :: Error ;
    fn try_from (id : crate :: dm :: CmdId ) -> Result < Self , Self :: Error > {
        CommandResponseId :: from_repr (id ) . ok_or_else (|| crate :: error :: ErrorCode :: CommandNotFound . into () ) 
    }
    
}
# [doc = "The event IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] pub enum EventId {
    
}
impl core :: convert :: TryFrom < crate :: dm :: EventId > for EventId {
    type Error = crate :: error :: Error ;
    fn try_from (id : crate :: dm :: EventId ) -> Result < Self , Self :: Error > {
        Err (crate :: error :: ErrorCode :: EventNotFound . into () ) 
    }
    
}
impl core :: fmt :: Debug for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Event::" ) ? ;
        match self . 0 {
            
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (49 , 2 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: MaxNetworks as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Networks as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: ScanMaxTimeSeconds as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ConnectMaxTimeSeconds as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: InterfaceEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_ADMIN . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: LastNetworkingStatus as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: LastNetworkID as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: LastConnectErrorValue as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: SupportedWiFiBands as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: SupportedThreadFeatures as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ThreadVersion as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: ScanNetworks as _ , Some (CommandResponseId :: ScanNetworksResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: AddOrUpdateWiFiNetwork as _ , Some (CommandResponseId :: NetworkConfigResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: AddOrUpdateThreadNetwork as _ , Some (CommandResponseId :: NetworkConfigResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: RemoveNetwork as _ , Some (CommandResponseId :: NetworkConfigResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: ConnectNetwork as _ , Some (CommandResponseId :: ConnectNetworkResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: ReorderNetwork as _ , Some (CommandResponseId :: NetworkConfigResponse as _ ) , crate :: dm :: Access :: WA , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
# [doc = "A helper struct to generate the cluster debug info." ] struct MetadataDebug < T > (pub T ) ;
# [doc = "The handler trait for the cluster." ] pub trait ClusterHandler : crate :: utils :: sync :: MaybeSendSync {
    # [doc = "The cluster-metadata corresponding to this handler trait." ] const CLUSTER : crate :: dm :: Cluster < 'static > ;
    fn dataver (& self ) -> u32 ;
    fn dataver_changed (& self ) ;
    fn lifecycle (& self , _ctx : impl crate :: dm :: HandlerContext , _op : crate :: dm :: LifecycleOp ) -> Result < () , crate :: error :: Error > {
        Ok (() ) 
    }
    fn run (& self , _ctx : impl crate :: dm :: HandlerContext ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: pending :: < Result :: < () , crate :: error :: Error >> () 
    }
    fn max_networks (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInfoStructArrayBuilder < P > , NetworkInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn scan_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn connect_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn interface_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > ;
    fn last_networking_status (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < NetworkCommissioningStatusEnum > , crate :: error :: Error > ;
    fn last_network_id < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn last_connect_error_value (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i32 > , crate :: error :: Error > ;
    fn supported_wi_fi_bands < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiFiBandEnum > , crate :: tlv :: ToTLVBuilder < P , WiFiBandEnum > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn supported_thread_features (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ThreadCapabilitiesBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn thread_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_interface_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > ;
    fn handle_scan_networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ScanNetworksRequest < '_ > , response : ScanNetworksResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_add_or_update_wi_fi_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateWiFiNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_add_or_update_thread_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateThreadNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_remove_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_connect_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ConnectNetworkRequest < '_ > , response : ConnectNetworkResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_reorder_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ReorderNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
}
impl < T > ClusterHandler for & T where T : ClusterHandler {
    const CLUSTER : crate :: dm :: Cluster < 'static > = T :: CLUSTER ;
    fn dataver (& self ) -> u32 {
        T :: dataver (self ) 
    }
    fn dataver_changed (& self ) {
        T :: dataver_changed (self ) 
    }
    fn lifecycle (& self , ctx : impl crate :: dm :: HandlerContext , op : crate :: dm :: LifecycleOp ) -> Result < () , crate :: error :: Error > {
        T :: lifecycle (self , ctx , op ) 
    }
    fn run (& self , ctx : impl crate :: dm :: HandlerContext ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        (* * self ) . run (ctx ) 
    }
    fn max_networks (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: max_networks (self , ctx ) 
    }
    fn networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInfoStructArrayBuilder < P > , NetworkInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: networks (self , ctx , builder ) 
    }
    fn scan_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: scan_max_time_seconds (self , ctx ) 
    }
    fn connect_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: connect_max_time_seconds (self , ctx ) 
    }
    fn interface_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: interface_enabled (self , ctx ) 
    }
    fn last_networking_status (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < NetworkCommissioningStatusEnum > , crate :: error :: Error > {
        T :: last_networking_status (self , ctx ) 
    }
    fn last_network_id < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: last_network_id (self , ctx , builder ) 
    }
    fn last_connect_error_value (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i32 > , crate :: error :: Error > {
        T :: last_connect_error_value (self , ctx ) 
    }
    fn supported_wi_fi_bands < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiFiBandEnum > , crate :: tlv :: ToTLVBuilder < P , WiFiBandEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: supported_wi_fi_bands (self , ctx , builder ) 
    }
    fn supported_thread_features (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ThreadCapabilitiesBitmap , crate :: error :: Error > {
        T :: supported_thread_features (self , ctx ) 
    }
    fn thread_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: thread_version (self , ctx ) 
    }
    fn set_interface_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_interface_enabled (self , ctx , value ) 
    }
    fn handle_scan_networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ScanNetworksRequest < '_ > , response : ScanNetworksResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_scan_networks (self , ctx , request , response ) 
    }
    fn handle_add_or_update_wi_fi_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateWiFiNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_add_or_update_wi_fi_network (self , ctx , request , response ) 
    }
    fn handle_add_or_update_thread_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateThreadNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_add_or_update_thread_network (self , ctx , request , response ) 
    }
    fn handle_remove_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_remove_network (self , ctx , request , response ) 
    }
    fn handle_connect_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ConnectNetworkRequest < '_ > , response : ConnectNetworkResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_connect_network (self , ctx , request , response ) 
    }
    fn handle_reorder_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ReorderNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_reorder_network (self , ctx , request , response ) 
    }
    
}
# [doc = "The handler adaptor for the cluster-specific handler. This adaptor implements the generic `rs-matter` handler trait." ] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] pub struct HandlerAdaptor < T > (pub T ) ;
impl < T > crate :: dm :: Handler for HandlerAdaptor < T > where T : ClusterHandler , {
    # [allow (unreachable_code ) ] fn read (& self , ctx : impl crate :: dm :: ReadContext , reply : impl crate :: dm :: ReadReply , ) -> Result < () , crate :: error :: Error > {
        if let Some (mut writer ) = reply . with_dataver (self . 0 . dataver () ) ? {
            if ctx . attr () . is_system () {
                ctx . attr () . cluster (ctx . metadata () , | cluster | cluster . read (ctx . attr () , writer ) ) 
            }
            else {
                match AttributeId :: try_from (ctx . attr () . attr_id ) ? {
                    AttributeId :: MaxNetworks => {
                        let attr_read_result = self . 0. max_networks (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxNetworks , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Networks => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Networks , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. networks (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Networks , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Networks , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ScanMaxTimeSeconds => {
                        let attr_read_result = self . 0. scan_max_time_seconds (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScanMaxTimeSeconds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ConnectMaxTimeSeconds => {
                        let attr_read_result = self . 0. connect_max_time_seconds (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ConnectMaxTimeSeconds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: InterfaceEnabled => {
                        let attr_read_result = self . 0. interface_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: InterfaceEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LastNetworkingStatus => {
                        let attr_read_result = self . 0. last_networking_status (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkingStatus , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LastNetworkID => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkID , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. last_network_id (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkID , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkID , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: LastConnectErrorValue => {
                        let attr_read_result = self . 0. last_connect_error_value (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastConnectErrorValue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SupportedWiFiBands => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedWiFiBands , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. supported_wi_fi_bands (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedWiFiBands , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedWiFiBands , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SupportedThreadFeatures => {
                        let attr_read_result = self . 0. supported_thread_features (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedThreadFeatures , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThreadVersion => {
                        let attr_read_result = self . 0. thread_version (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThreadVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    # [allow (unreachable_code ) ] _ => {
                        crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
                    }
                    
                }
                
            }
            
        }
        else {
            Ok (() ) 
        }
        
    }
    # [allow (unreachable_code ) ] fn write (& self , ctx : impl crate :: dm :: WriteContext , ) -> Result < () , crate :: error :: Error > {
        ctx . attr () . check_dataver (self . 0 . dataver () ) ? ;
        if ctx . attr () . is_system () {
            return Err (crate :: error :: ErrorCode :: InvalidAction . into () ) 
        }
        match AttributeId :: try_from (ctx . attr () . attr_id ) ? {
            AttributeId :: InterfaceEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_interface_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: InterfaceEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            _ => {
                crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                return Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    # [allow (unreachable_code ) ] fn invoke (& self , ctx : impl crate :: dm :: InvokeContext , reply : impl crate :: dm :: InvokeReply , ) -> Result < () , crate :: error :: Error > {
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: ScanNetworks => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ScanNetworks ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_scan_networks (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ScanNetworks ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ScanNetworks ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddOrUpdateWiFiNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateWiFiNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_or_update_wi_fi_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateWiFiNetwork ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateWiFiNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddOrUpdateThreadNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateThreadNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_or_update_thread_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateThreadNetwork ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateThreadNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RemoveNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_remove_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveNetwork ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ConnectNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConnectNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (7u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_connect_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConnectNetwork ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConnectNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ReorderNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ReorderNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_reorder_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ReorderNetwork ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ReorderNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            _ => {
                crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
                return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 49u32 ) . unwrap_or (true ) {
            self . 0 . dataver_changed () ;
            
        }
        
    }
    fn lifecycle (& self , ctx : impl crate :: dm :: HandlerContext , op : crate :: dm :: LifecycleOp ) -> Result < () , crate :: error :: Error > {
        self . 0 . lifecycle (ctx , op ) 
    }
    fn run (& self , ctx : impl crate :: dm :: HandlerContext ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        self . 0 . run (ctx ) 
    }
    
}
impl < T , Q > core :: fmt :: Debug for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : core :: fmt :: Debug , {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "NetworkCommissioning" , 49u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "NetworkCommissioning" , 49u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `NetworkCommissioning` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct NetworkCommissioningCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > NetworkCommissioningCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn scan_networks (self , endpoint : crate :: dm :: EndptId , ) -> Result < ScanNetworksRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 49 , 0 ) ? . data_builder () 
    }
    pub fn add_or_update_wi_fi_network (self , endpoint : crate :: dm :: EndptId , ) -> Result < AddOrUpdateWiFiNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 49 , 2 ) ? . data_builder () 
    }
    pub fn add_or_update_thread_network (self , endpoint : crate :: dm :: EndptId , ) -> Result < AddOrUpdateThreadNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 49 , 3 ) ? . data_builder () 
    }
    pub fn remove_network (self , endpoint : crate :: dm :: EndptId , ) -> Result < RemoveNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 49 , 4 ) ? . data_builder () 
    }
    pub fn connect_network (self , endpoint : crate :: dm :: EndptId , ) -> Result < ConnectNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 49 , 6 ) ? . data_builder () 
    }
    pub fn reorder_network (self , endpoint : crate :: dm :: EndptId , ) -> Result < ReorderNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 49 , 8 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `NetworkCommissioning` cluster's commands. `use` this trait to call `.network_commissioning_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`NetworkCommissioningCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait NetworkCommissioningCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn network_commissioning_inv (self ) -> NetworkCommissioningCmdRequestsView < P > ;
    
}
impl < P > NetworkCommissioningCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn network_commissioning_inv (self ) -> NetworkCommissioningCmdRequestsView < P > {
        NetworkCommissioningCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `NetworkCommissioning` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct NetworkCommissioningAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > NetworkCommissioningAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_networks (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn networks (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn scan_max_time_seconds (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn connect_max_time_seconds (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn interface_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn last_networking_status (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn last_network_id (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn last_connect_error_value (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn supported_wi_fi_bands (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn supported_thread_features (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn thread_version (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (49 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `NetworkCommissioning` cluster's attribute reads. `use` this trait to call `.network_commissioning_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`NetworkCommissioningAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait NetworkCommissioningAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn network_commissioning_read (self ) -> NetworkCommissioningAttrReadsView < P > ;
    
}
impl < P > NetworkCommissioningAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn network_commissioning_read (self ) -> NetworkCommissioningAttrReadsView < P > {
        NetworkCommissioningAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `NetworkCommissioning` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct NetworkCommissioningAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > NetworkCommissioningAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn interface_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 49 , 4 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `NetworkCommissioning` cluster's attribute writes. `use` this trait to call `.network_commissioning_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`NetworkCommissioningAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait NetworkCommissioningAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn network_commissioning_write (self ) -> NetworkCommissioningAttrWritesView < P > ;
    
}
impl < P > NetworkCommissioningAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn network_commissioning_write (self ) -> NetworkCommissioningAttrWritesView < P > {
        NetworkCommissioningAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `NetworkCommissioning` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct NetworkCommissioningCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > NetworkCommissioningCmdResponsesView < 'a , 'r > {
    pub fn scan_networks (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ScanNetworksResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < ScanNetworksResponse < 'a >> (49 , 0 ) 
    }
    pub fn add_or_update_wi_fi_network (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NetworkConfigResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NetworkConfigResponse < 'a >> (49 , 2 ) 
    }
    pub fn add_or_update_thread_network (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NetworkConfigResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NetworkConfigResponse < 'a >> (49 , 3 ) 
    }
    pub fn remove_network (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NetworkConfigResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NetworkConfigResponse < 'a >> (49 , 4 ) 
    }
    pub fn connect_network (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ConnectNetworkResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < ConnectNetworkResponse < 'a >> (49 , 6 ) 
    }
    pub fn reorder_network (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NetworkConfigResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NetworkConfigResponse < 'a >> (49 , 8 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `NetworkCommissioning`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.network_commissioning_inv_resp()` on an `InvokeResp`; the returned [`NetworkCommissioningCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait NetworkCommissioningCmdResponses < 'a > {
    fn network_commissioning_inv_resp (& self ) -> NetworkCommissioningCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > NetworkCommissioningCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn network_commissioning_inv_resp (& self ) -> NetworkCommissioningCmdResponsesView < 'a , '_ > {
        NetworkCommissioningCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `NetworkCommissioning` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct NetworkCommissioningAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > NetworkCommissioningAttrResponsesView < 'a , 'r > {
    pub fn max_networks (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (49 , 0 ) 
    }
    pub fn networks (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , NetworkInfoStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , NetworkInfoStruct < 'a > > > (49 , 1 ) 
    }
    pub fn scan_max_time_seconds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (49 , 2 ) 
    }
    pub fn connect_max_time_seconds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (49 , 3 ) 
    }
    pub fn interface_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (49 , 4 ) 
    }
    pub fn last_networking_status (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < NetworkCommissioningStatusEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < NetworkCommissioningStatusEnum > > (49 , 5 ) 
    }
    pub fn last_network_id (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > > (49 , 6 ) 
    }
    pub fn last_connect_error_value (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < i32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < i32 > > (49 , 7 ) 
    }
    pub fn supported_wi_fi_bands (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , WiFiBandEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , WiFiBandEnum > > (49 , 8 ) 
    }
    pub fn supported_thread_features (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ThreadCapabilitiesBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ThreadCapabilitiesBitmap > (49 , 9 ) 
    }
    pub fn thread_version (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (49 , 10 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (49 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (49 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (49 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (49 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (49 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `NetworkCommissioning`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.network_commissioning_read_resp()` on a `ReportDataResp`; the returned [`NetworkCommissioningAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait NetworkCommissioningAttrResponses < 'a > {
    fn network_commissioning_read_resp (& self ) -> NetworkCommissioningAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > NetworkCommissioningAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn network_commissioning_read_resp (& self ) -> NetworkCommissioningAttrResponsesView < 'a , '_ > {
        NetworkCommissioningAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `NetworkCommissioning` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct NetworkCommissioningWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > NetworkCommissioningWriteResponsesView < 'a , 'r > {
    pub fn interface_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (49 , 4 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `NetworkCommissioning`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.network_commissioning_write_resp()` on a `WriteResp`; the returned [`NetworkCommissioningWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait NetworkCommissioningWriteResponses < 'a > {
    fn network_commissioning_write_resp (& self ) -> NetworkCommissioningWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > NetworkCommissioningWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn network_commissioning_write_resp (& self ) -> NetworkCommissioningWriteResponsesView < 'a , '_ > {
        NetworkCommissioningWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `ScanNetworksResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`ScanNetworksResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct ScanNetworksResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > ScanNetworksResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < ScanNetworksResponse < '_ > , crate :: error :: Error > {
        let invoke_resp = crate :: im :: ImInvokeChunk :: response (& self . chunk ) ? . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? ;
        let invoke_responses = invoke_resp . invoke_responses . as_ref () . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? ;
        let cmd_resp = invoke_responses . iter () . next () . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? . map_err (| _ | crate :: error :: ErrorCode :: InvalidData ) ? ;
        match cmd_resp {
            crate :: im :: CmdResp :: Cmd (data ) => {
                crate :: tlv :: FromTLV :: from_tlv (& data . data ) 
            }
            crate :: im :: CmdResp :: Status (s ) => {
                Err (s . status . status . to_error_code () . unwrap_or (crate :: error :: ErrorCode :: Failure ) . into () ) 
            }
            
        }
        
    }
    # [doc = r" Send the trailing `StatusResponse(Success)` and close" ] # [doc = r" the transaction. Call this once after the borrowed" ] # [doc = r" response is no longer needed. Forgetting it lets the" ] # [doc = r" peer time out and resend — functional but wasteful." ] pub fn complete (self ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        crate :: im :: complete_invoke (self . chunk ) 
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `NetworkConfigResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`NetworkConfigResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct NetworkConfigResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > NetworkConfigResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < NetworkConfigResponse < '_ > , crate :: error :: Error > {
        let invoke_resp = crate :: im :: ImInvokeChunk :: response (& self . chunk ) ? . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? ;
        let invoke_responses = invoke_resp . invoke_responses . as_ref () . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? ;
        let cmd_resp = invoke_responses . iter () . next () . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? . map_err (| _ | crate :: error :: ErrorCode :: InvalidData ) ? ;
        match cmd_resp {
            crate :: im :: CmdResp :: Cmd (data ) => {
                crate :: tlv :: FromTLV :: from_tlv (& data . data ) 
            }
            crate :: im :: CmdResp :: Status (s ) => {
                Err (s . status . status . to_error_code () . unwrap_or (crate :: error :: ErrorCode :: Failure ) . into () ) 
            }
            
        }
        
    }
    # [doc = r" Send the trailing `StatusResponse(Success)` and close" ] # [doc = r" the transaction. Call this once after the borrowed" ] # [doc = r" response is no longer needed. Forgetting it lets the" ] # [doc = r" peer time out and resend — functional but wasteful." ] pub fn complete (self ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        crate :: im :: complete_invoke (self . chunk ) 
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `ConnectNetworkResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`ConnectNetworkResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct ConnectNetworkResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > ConnectNetworkResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < ConnectNetworkResponse < '_ > , crate :: error :: Error > {
        let invoke_resp = crate :: im :: ImInvokeChunk :: response (& self . chunk ) ? . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? ;
        let invoke_responses = invoke_resp . invoke_responses . as_ref () . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? ;
        let cmd_resp = invoke_responses . iter () . next () . ok_or (crate :: error :: ErrorCode :: InvalidData ) ? . map_err (| _ | crate :: error :: ErrorCode :: InvalidData ) ? ;
        match cmd_resp {
            crate :: im :: CmdResp :: Cmd (data ) => {
                crate :: tlv :: FromTLV :: from_tlv (& data . data ) 
            }
            crate :: im :: CmdResp :: Status (s ) => {
                Err (s . status . status . to_error_code () . unwrap_or (crate :: error :: ErrorCode :: Failure ) . into () ) 
            }
            
        }
        
    }
    # [doc = r" Send the trailing `StatusResponse(Success)` and close" ] # [doc = r" the transaction. Call this once after the borrowed" ] # [doc = r" response is no longer needed. Forgetting it lets the" ] # [doc = r" peer time out and resend — functional but wasteful." ] pub fn complete (self ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        crate :: im :: complete_invoke (self . chunk ) 
    }
    
}
# [doc = "Per-transaction view onto the `NetworkCommissioning` cluster's client operations. Returned by [`NetworkCommissioningClient::network_commissioning`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct NetworkCommissioningClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > NetworkCommissioningClientView < T > {
    pub async fn scan_networks < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < ScanNetworksResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (ScanNetworksRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: NetworkCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . network_commissioning_inv () ;
            let req_builder = view . scan_networks (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (ScanNetworksResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn add_or_update_wi_fi_network < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NetworkConfigResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AddOrUpdateWiFiNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: NetworkCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . network_commissioning_inv () ;
            let req_builder = view . add_or_update_wi_fi_network (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NetworkConfigResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn add_or_update_thread_network < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NetworkConfigResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AddOrUpdateThreadNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: NetworkCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . network_commissioning_inv () ;
            let req_builder = view . add_or_update_thread_network (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NetworkConfigResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn remove_network < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NetworkConfigResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (RemoveNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: NetworkCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . network_commissioning_inv () ;
            let req_builder = view . remove_network (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NetworkConfigResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn connect_network < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < ConnectNetworkResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (ConnectNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: NetworkCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . network_commissioning_inv () ;
            let req_builder = view . connect_network (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (ConnectNetworkResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn reorder_network < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NetworkConfigResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (ReorderNetworkRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: NetworkCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . network_commissioning_inv () ;
            let req_builder = view . reorder_network (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NetworkConfigResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn max_networks_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . max_networks (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn networks_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , NetworkInfoStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . networks (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn scan_max_time_seconds_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . scan_max_time_seconds (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn connect_max_time_seconds_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . connect_max_time_seconds (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn interface_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . interface_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn last_networking_status_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < NetworkCommissioningStatusEnum > , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . last_networking_status (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn last_network_id_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . last_network_id (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn last_connect_error_value_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i32 > , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . last_connect_error_value (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn supported_wi_fi_bands_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , WiFiBandEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . supported_wi_fi_bands (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn supported_thread_features_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ThreadCapabilitiesBitmap , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . supported_thread_features (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn thread_version_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . thread_version (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn accepted_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn attribute_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: AttrId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . network_commissioning_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn interface_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: NetworkCommissioningAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . network_commissioning_write () . interface_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `NetworkCommissioning` cluster. `use` this trait to call `.network_commissioning()` on any [`crate::im::ImClient`], such as an exchange; the returned [`NetworkCommissioningClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `NetworkCommissioningClientView` keeps each cluster's API surface narrow: at the call site `exchange.network_commissioning().` shows only this cluster's methods in IDE completion." ] pub trait NetworkCommissioningClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn network_commissioning (self ) -> NetworkCommissioningClientView < Self > {
        NetworkCommissioningClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > NetworkCommissioningClient for T {
    
}
# [doc = "The handler trait for the cluster." ] pub trait ClusterAsyncHandler : crate :: utils :: sync :: MaybeSendSync {
    # [doc = "The cluster-metadata corresponding to this handler trait." ] const CLUSTER : crate :: dm :: Cluster < 'static > ;
    fn dataver (& self ) -> u32 ;
    fn dataver_changed (& self ) ;
    fn lifecycle (& self , _ctx : impl crate :: dm :: HandlerContext , _op : crate :: dm :: LifecycleOp ) -> Result < () , crate :: error :: Error > {
        Ok (() ) 
    }
    fn run (& self , _ctx : impl crate :: dm :: HandlerContext ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: pending :: < Result :: < () , crate :: error :: Error >> () 
    }
    fn max_networks (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInfoStructArrayBuilder < P > , NetworkInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn scan_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn connect_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn interface_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn last_networking_status (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < NetworkCommissioningStatusEnum > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn last_network_id < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn last_connect_error_value (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i32 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn supported_wi_fi_bands < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiFiBandEnum > , crate :: tlv :: ToTLVBuilder < P , WiFiBandEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn supported_thread_features (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ThreadCapabilitiesBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn thread_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_interface_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_scan_networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ScanNetworksRequest < '_ > , response : ScanNetworksResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_add_or_update_wi_fi_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateWiFiNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_add_or_update_thread_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateThreadNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_remove_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_connect_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ConnectNetworkRequest < '_ > , response : ConnectNetworkResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_reorder_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ReorderNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
}
impl < T > ClusterAsyncHandler for & T where T : ClusterAsyncHandler {
    const CLUSTER : crate :: dm :: Cluster < 'static > = T :: CLUSTER ;
    fn dataver (& self ) -> u32 {
        T :: dataver (self ) 
    }
    fn dataver_changed (& self ) {
        T :: dataver_changed (self ) 
    }
    fn lifecycle (& self , ctx : impl crate :: dm :: HandlerContext , op : crate :: dm :: LifecycleOp ) -> Result < () , crate :: error :: Error > {
        T :: lifecycle (self , ctx , op ) 
    }
    fn run (& self , ctx : impl crate :: dm :: HandlerContext ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        (* * self ) . run (ctx ) 
    }
    fn max_networks (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: max_networks (self , ctx ) 
    }
    fn networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInfoStructArrayBuilder < P > , NetworkInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: networks (self , ctx , builder ) 
    }
    fn scan_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: scan_max_time_seconds (self , ctx ) 
    }
    fn connect_max_time_seconds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: connect_max_time_seconds (self , ctx ) 
    }
    fn interface_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: interface_enabled (self , ctx ) 
    }
    fn last_networking_status (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < NetworkCommissioningStatusEnum > , crate :: error :: Error >> {
        T :: last_networking_status (self , ctx ) 
    }
    fn last_network_id < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: last_network_id (self , ctx , builder ) 
    }
    fn last_connect_error_value (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i32 > , crate :: error :: Error >> {
        T :: last_connect_error_value (self , ctx ) 
    }
    fn supported_wi_fi_bands < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiFiBandEnum > , crate :: tlv :: ToTLVBuilder < P , WiFiBandEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: supported_wi_fi_bands (self , ctx , builder ) 
    }
    fn supported_thread_features (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ThreadCapabilitiesBitmap , crate :: error :: Error >> {
        T :: supported_thread_features (self , ctx ) 
    }
    fn thread_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: thread_version (self , ctx ) 
    }
    fn set_interface_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_interface_enabled (self , ctx , value ) 
    }
    fn handle_scan_networks < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ScanNetworksRequest < '_ > , response : ScanNetworksResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_scan_networks (self , ctx , request , response ) 
    }
    fn handle_add_or_update_wi_fi_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateWiFiNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_add_or_update_wi_fi_network (self , ctx , request , response ) 
    }
    fn handle_add_or_update_thread_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddOrUpdateThreadNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_add_or_update_thread_network (self , ctx , request , response ) 
    }
    fn handle_remove_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_remove_network (self , ctx , request , response ) 
    }
    fn handle_connect_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ConnectNetworkRequest < '_ > , response : ConnectNetworkResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_connect_network (self , ctx , request , response ) 
    }
    fn handle_reorder_network < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ReorderNetworkRequest < '_ > , response : NetworkConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_reorder_network (self , ctx , request , response ) 
    }
    
}
# [doc = "The handler adaptor for the cluster-specific handler. This adaptor implements the generic `rs-matter` handler trait." ] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] pub struct HandlerAsyncAdaptor < T > (pub T ) ;
impl < T > crate :: dm :: AsyncHandler for HandlerAsyncAdaptor < T > where T : ClusterAsyncHandler , {
    # [allow (unreachable_code ) ] async fn read (& self , ctx : impl crate :: dm :: ReadContext , reply : impl crate :: dm :: ReadReply , ) -> Result < () , crate :: error :: Error > {
        if let Some (mut writer ) = reply . with_dataver (self . 0 . dataver () ) ? {
            if ctx . attr () . is_system () {
                ctx . attr () . cluster (ctx . metadata () , | cluster | cluster . read (ctx . attr () , writer ) ) 
            }
            else {
                match AttributeId :: try_from (ctx . attr () . attr_id ) ? {
                    AttributeId :: MaxNetworks => {
                        let attr_read_result = self . 0. max_networks (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxNetworks , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Networks => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Networks , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. networks (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Networks , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Networks , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ScanMaxTimeSeconds => {
                        let attr_read_result = self . 0. scan_max_time_seconds (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ScanMaxTimeSeconds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ConnectMaxTimeSeconds => {
                        let attr_read_result = self . 0. connect_max_time_seconds (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ConnectMaxTimeSeconds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: InterfaceEnabled => {
                        let attr_read_result = self . 0. interface_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: InterfaceEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LastNetworkingStatus => {
                        let attr_read_result = self . 0. last_networking_status (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkingStatus , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LastNetworkID => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkID , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. last_network_id (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkID , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastNetworkID , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: LastConnectErrorValue => {
                        let attr_read_result = self . 0. last_connect_error_value (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LastConnectErrorValue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SupportedWiFiBands => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedWiFiBands , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. supported_wi_fi_bands (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedWiFiBands , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedWiFiBands , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SupportedThreadFeatures => {
                        let attr_read_result = self . 0. supported_thread_features (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedThreadFeatures , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ThreadVersion => {
                        let attr_read_result = self . 0. thread_version (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ThreadVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    # [allow (unreachable_code ) ] _ => {
                        crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
                    }
                    
                }
                
            }
            
        }
        else {
            Ok (() ) 
        }
        
    }
    # [allow (unreachable_code ) ] async fn write (& self , ctx : impl crate :: dm :: WriteContext , ) -> Result < () , crate :: error :: Error > {
        ctx . attr () . check_dataver (self . 0 . dataver () ) ? ;
        if ctx . attr () . is_system () {
            return Err (crate :: error :: ErrorCode :: InvalidAction . into () ) 
        }
        match AttributeId :: try_from (ctx . attr () . attr_id ) ? {
            AttributeId :: InterfaceEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_interface_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: InterfaceEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            _ => {
                crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                return Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    # [allow (unreachable_code ) ] async fn invoke (& self , ctx : impl crate :: dm :: InvokeContext , reply : impl crate :: dm :: InvokeReply , ) -> Result < () , crate :: error :: Error > {
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: ScanNetworks => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ScanNetworks ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_scan_networks (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ScanNetworks ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ScanNetworks ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddOrUpdateWiFiNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateWiFiNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_or_update_wi_fi_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateWiFiNetwork ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateWiFiNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddOrUpdateThreadNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateThreadNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_or_update_thread_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateThreadNetwork ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddOrUpdateThreadNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RemoveNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_remove_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveNetwork ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ConnectNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConnectNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (7u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_connect_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConnectNetwork ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConnectNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ReorderNetwork => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ReorderNetwork ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_reorder_network (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ReorderNetwork ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ReorderNetwork ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            _ => {
                crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
                return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 49u32 ) . unwrap_or (true ) {
            self . 0 . dataver_changed () ;
            
        }
        
    }
    fn lifecycle (& self , ctx : impl crate :: dm :: HandlerContext , op : crate :: dm :: LifecycleOp ) -> Result < () , crate :: error :: Error > {
        self . 0 . lifecycle (ctx , op ) 
    }
    fn run (& self , ctx : impl crate :: dm :: HandlerContext ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        self . 0 . run (ctx ) 
    }
    
}
impl < T , Q > core :: fmt :: Debug for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : core :: fmt :: Debug , {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "NetworkCommissioning" , 49u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "NetworkCommissioning" , 49u32 , self . 0.2 ) 
    }
    
}
