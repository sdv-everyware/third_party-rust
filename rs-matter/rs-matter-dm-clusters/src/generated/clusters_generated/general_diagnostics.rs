# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const DATA_MODEL_TEST = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const DATA_MODEL_TEST = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BootReasonEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] PowerOnReboot = 1 , # [enumval (2 ) ] BrownOutReset = 2 , # [enumval (3 ) ] SoftwareWatchdogReset = 3 , # [enumval (4 ) ] HardwareWatchdogReset = 4 , # [enumval (5 ) ] SoftwareUpdateCompleted = 5 , # [enumval (6 ) ] SoftwareReset = 6 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum HardwareFaultEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] Radio = 1 , # [enumval (2 ) ] Sensor = 2 , # [enumval (3 ) ] ResettableOverTemp = 3 , # [enumval (4 ) ] NonResettableOverTemp = 4 , # [enumval (5 ) ] PowerSource = 5 , # [enumval (6 ) ] VisualDisplayFault = 6 , # [enumval (7 ) ] AudioOutputFault = 7 , # [enumval (8 ) ] UserInterfaceFault = 8 , # [enumval (9 ) ] NonVolatileMemoryError = 9 , # [enumval (10 ) ] TamperDetected = 10 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum InterfaceTypeEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] WiFi = 1 , # [enumval (2 ) ] Ethernet = 2 , # [enumval (3 ) ] Cellular = 3 , # [enumval (4 ) ] Thread = 4 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkFaultEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] HardwareFailure = 1 , # [enumval (2 ) ] NetworkJammed = 2 , # [enumval (3 ) ] ConnectionFailed = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RadioFaultEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] WiFiFault = 1 , # [enumval (2 ) ] CellularFault = 2 , # [enumval (3 ) ] ThreadFault = 3 , # [enumval (4 ) ] NFCFault = 4 , # [enumval (5 ) ] BLEFault = 5 , # [enumval (6 ) ] EthernetFault = 6 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum DeviceLoadStructTag {
    CurrentSubscriptions = 0 , CurrentSubscriptionsForFabric = 1 , TotalSubscriptionsEstablished = 2 , TotalInteractionModelMessagesSent = 3 , TotalInteractionModelMessagesReceived = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkInterfaceTag {
    Name = 0 , IsOperational = 1 , OffPremiseServicesReachableIPv4 = 2 , OffPremiseServicesReachableIPv6 = 3 , HardwareAddress = 4 , IPv4Addresses = 5 , IPv6Addresses = 6 , Type = 7 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TestEventTriggerRequestTag {
    EnableKey = 0 , EventTrigger = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TimeSnapshotResponseTag {
    SystemTimeMs = 0 , PosixTimeMs = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PayloadTestRequestRequestTag {
    EnableKey = 0 , Value = 1 , Count = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PayloadTestResponseTag {
    Payload = 0 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct DeviceLoadStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > DeviceLoadStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current_subscriptions (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn current_subscriptions_for_fabric (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn total_subscriptions_established (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn total_interaction_model_messages_sent (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn total_interaction_model_messages_received (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for DeviceLoadStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for DeviceLoadStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for DeviceLoadStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "DeviceLoadStruct" ) ? ;
        match self . current_subscriptions () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_subscriptions" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_subscriptions" , e . code () ) ? , 
        }
        match self . current_subscriptions_for_fabric () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_subscriptions_for_fabric" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_subscriptions_for_fabric" , e . code () ) ? , 
        }
        match self . total_subscriptions_established () {
            Ok (value ) => write ! (f , "{}: {:?}," , "total_subscriptions_established" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "total_subscriptions_established" , e . code () ) ? , 
        }
        match self . total_interaction_model_messages_sent () {
            Ok (value ) => write ! (f , "{}: {:?}," , "total_interaction_model_messages_sent" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "total_interaction_model_messages_sent" , e . code () ) ? , 
        }
        match self . total_interaction_model_messages_received () {
            Ok (value ) => write ! (f , "{}: {:?}," , "total_interaction_model_messages_received" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "total_interaction_model_messages_received" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for DeviceLoadStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "DeviceLoadStruct" ) ;
        match self . current_subscriptions () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_subscriptions" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_subscriptions" , e . code () ) , 
        }
        match self . current_subscriptions_for_fabric () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_subscriptions_for_fabric" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_subscriptions_for_fabric" , e . code () ) , 
        }
        match self . total_subscriptions_established () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "total_subscriptions_established" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "total_subscriptions_established" , e . code () ) , 
        }
        match self . total_interaction_model_messages_sent () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "total_interaction_model_messages_sent" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "total_interaction_model_messages_sent" , e . code () ) , 
        }
        match self . total_interaction_model_messages_received () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "total_interaction_model_messages_received" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "total_interaction_model_messages_received" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct NetworkInterface < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > NetworkInterface < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn name (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn is_operational (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn off_premise_services_reachable_i_pv_4 (& self ) -> Result < crate :: tlv :: Nullable < bool > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn off_premise_services_reachable_i_pv_6 (& self ) -> Result < crate :: tlv :: Nullable < bool > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn hardware_address (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn i_pv_4_addresses (& self ) -> Result < crate :: tlv :: TLVArray < '_ , crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn i_pv_6_addresses (& self ) -> Result < crate :: tlv :: TLVArray < '_ , crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn r#type (& self ) -> Result < InterfaceTypeEnum , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for NetworkInterface < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for NetworkInterface < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for NetworkInterface < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "NetworkInterface" ) ? ;
        match self . name () {
            Ok (value ) => write ! (f , "{}: {:?}," , "name" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "name" , e . code () ) ? , 
        }
        match self . is_operational () {
            Ok (value ) => write ! (f , "{}: {:?}," , "is_operational" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "is_operational" , e . code () ) ? , 
        }
        match self . off_premise_services_reachable_i_pv_4 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "off_premise_services_reachable_i_pv_4" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "off_premise_services_reachable_i_pv_4" , e . code () ) ? , 
        }
        match self . off_premise_services_reachable_i_pv_6 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "off_premise_services_reachable_i_pv_6" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "off_premise_services_reachable_i_pv_6" , e . code () ) ? , 
        }
        match self . hardware_address () {
            Ok (value ) => write ! (f , "{}: {:?}," , "hardware_address" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "hardware_address" , e . code () ) ? , 
        }
        match self . i_pv_4_addresses () {
            Ok (value ) => write ! (f , "{}: {:?}," , "i_pv_4_addresses" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "i_pv_4_addresses" , e . code () ) ? , 
        }
        match self . i_pv_6_addresses () {
            Ok (value ) => write ! (f , "{}: {:?}," , "i_pv_6_addresses" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "i_pv_6_addresses" , e . code () ) ? , 
        }
        match self . r#type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "type" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for NetworkInterface < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "NetworkInterface" ) ;
        match self . name () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "name" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "name" , e . code () ) , 
        }
        match self . is_operational () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "is_operational" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "is_operational" , e . code () ) , 
        }
        match self . off_premise_services_reachable_i_pv_4 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "off_premise_services_reachable_i_pv_4" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "off_premise_services_reachable_i_pv_4" , e . code () ) , 
        }
        match self . off_premise_services_reachable_i_pv_6 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "off_premise_services_reachable_i_pv_6" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "off_premise_services_reachable_i_pv_6" , e . code () ) , 
        }
        match self . hardware_address () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "hardware_address" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "hardware_address" , e . code () ) , 
        }
        match self . i_pv_4_addresses () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "i_pv_4_addresses" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "i_pv_4_addresses" , e . code () ) , 
        }
        match self . i_pv_6_addresses () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "i_pv_6_addresses" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "i_pv_6_addresses" , e . code () ) , 
        }
        match self . r#type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "type" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct TestEventTriggerRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > TestEventTriggerRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn enable_key (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn event_trigger (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for TestEventTriggerRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for TestEventTriggerRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for TestEventTriggerRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "TestEventTriggerRequest" ) ? ;
        match self . enable_key () {
            Ok (value ) => write ! (f , "{}: {:?}," , "enable_key" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "enable_key" , e . code () ) ? , 
        }
        match self . event_trigger () {
            Ok (value ) => write ! (f , "{}: {:?}," , "event_trigger" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "event_trigger" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for TestEventTriggerRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "TestEventTriggerRequest" ) ;
        match self . enable_key () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "enable_key" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "enable_key" , e . code () ) , 
        }
        match self . event_trigger () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "event_trigger" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "event_trigger" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct TimeSnapshotResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > TimeSnapshotResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn system_time_ms (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn posix_time_ms (& self ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for TimeSnapshotResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for TimeSnapshotResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for TimeSnapshotResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "TimeSnapshotResponse" ) ? ;
        match self . system_time_ms () {
            Ok (value ) => write ! (f , "{}: {:?}," , "system_time_ms" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "system_time_ms" , e . code () ) ? , 
        }
        match self . posix_time_ms () {
            Ok (value ) => write ! (f , "{}: {:?}," , "posix_time_ms" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "posix_time_ms" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for TimeSnapshotResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "TimeSnapshotResponse" ) ;
        match self . system_time_ms () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "system_time_ms" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "system_time_ms" , e . code () ) , 
        }
        match self . posix_time_ms () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "posix_time_ms" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "posix_time_ms" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct PayloadTestRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > PayloadTestRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn enable_key (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn value (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn count (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for PayloadTestRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for PayloadTestRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for PayloadTestRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "PayloadTestRequestRequest" ) ? ;
        match self . enable_key () {
            Ok (value ) => write ! (f , "{}: {:?}," , "enable_key" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "enable_key" , e . code () ) ? , 
        }
        match self . value () {
            Ok (value ) => write ! (f , "{}: {:?}," , "value" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "value" , e . code () ) ? , 
        }
        match self . count () {
            Ok (value ) => write ! (f , "{}: {:?}," , "count" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "count" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for PayloadTestRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "PayloadTestRequestRequest" ) ;
        match self . enable_key () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "enable_key" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "enable_key" , e . code () ) , 
        }
        match self . value () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "value" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "value" , e . code () ) , 
        }
        match self . count () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "count" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "count" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct PayloadTestResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > PayloadTestResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn payload (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for PayloadTestResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for PayloadTestResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for PayloadTestResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "PayloadTestResponse" ) ? ;
        match self . payload () {
            Ok (value ) => write ! (f , "{}: {:?}," , "payload" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "payload" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for PayloadTestResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "PayloadTestResponse" ) ;
        match self . payload () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "payload" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "payload" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct DeviceLoadStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > DeviceLoadStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > DeviceLoadStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_subscriptions (mut self , value : u16 ) -> Result < DeviceLoadStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentSubscriptions" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (DeviceLoadStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > DeviceLoadStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_subscriptions_for_fabric (mut self , value : u16 ) -> Result < DeviceLoadStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentSubscriptionsForFabric" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (DeviceLoadStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > DeviceLoadStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn total_subscriptions_established (mut self , value : u32 ) -> Result < DeviceLoadStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "totalSubscriptionsEstablished" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (DeviceLoadStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > DeviceLoadStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn total_interaction_model_messages_sent (mut self , value : u32 ) -> Result < DeviceLoadStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "totalInteractionModelMessagesSent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (DeviceLoadStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > DeviceLoadStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn total_interaction_model_messages_received (mut self , value : u32 ) -> Result < DeviceLoadStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "totalInteractionModelMessagesReceived" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (DeviceLoadStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > DeviceLoadStructBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for DeviceLoadStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "DeviceLoadStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for DeviceLoadStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "DeviceLoadStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for DeviceLoadStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for DeviceLoadStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct DeviceLoadStructArrayBuilder < P > (P ) ;
impl < P > DeviceLoadStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < DeviceLoadStructBuilder < DeviceLoadStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (DeviceLoadStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for DeviceLoadStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "DeviceLoadStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for DeviceLoadStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "DeviceLoadStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for DeviceLoadStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for DeviceLoadStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NetworkInterfaceBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > NetworkInterfaceBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn name (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < NetworkInterfaceBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "name" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (NetworkInterfaceBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn is_operational (mut self , value : bool ) -> Result < NetworkInterfaceBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "isOperational" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (NetworkInterfaceBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn off_premise_services_reachable_i_pv_4 (mut self , value : crate :: tlv :: Nullable < bool > ) -> Result < NetworkInterfaceBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "offPremiseServicesReachableIPv4" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (NetworkInterfaceBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn off_premise_services_reachable_i_pv_6 (mut self , value : crate :: tlv :: Nullable < bool > ) -> Result < NetworkInterfaceBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "offPremiseServicesReachableIPv6" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (NetworkInterfaceBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn hardware_address (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < NetworkInterfaceBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "hardwareAddress" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (NetworkInterfaceBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn i_pv_4_addresses (self ) -> Result < crate :: tlv :: OctetsArrayBuilder < NetworkInterfaceBuilder < P , 6usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkInterfaceBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (5 ) , ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn i_pv_6_addresses (self ) -> Result < crate :: tlv :: OctetsArrayBuilder < NetworkInterfaceBuilder < P , 7usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkInterfaceBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (6 ) , ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn r#type (mut self , value : InterfaceTypeEnum ) -> Result < NetworkInterfaceBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "type" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (NetworkInterfaceBuilder (self . 0 ) ) 
    }
    
}
impl < P > NetworkInterfaceBuilder < P , 8usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for NetworkInterfaceBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkInterface" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for NetworkInterfaceBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkInterface" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for NetworkInterfaceBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkInterfaceBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NetworkInterfaceArrayBuilder < P > (P ) ;
impl < P > NetworkInterfaceArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < NetworkInterfaceBuilder < NetworkInterfaceArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkInterfaceArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for NetworkInterfaceArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkInterface[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for NetworkInterfaceArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkInterface[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for NetworkInterfaceArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkInterfaceArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TestEventTriggerRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > TestEventTriggerRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > TestEventTriggerRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn enable_key (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < TestEventTriggerRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "enableKey" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (TestEventTriggerRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > TestEventTriggerRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn event_trigger (mut self , value : u64 ) -> Result < TestEventTriggerRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "eventTrigger" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (TestEventTriggerRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > TestEventTriggerRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for TestEventTriggerRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TestEventTriggerRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for TestEventTriggerRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TestEventTriggerRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for TestEventTriggerRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TestEventTriggerRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TestEventTriggerRequestArrayBuilder < P > (P ) ;
impl < P > TestEventTriggerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < TestEventTriggerRequestBuilder < TestEventTriggerRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TestEventTriggerRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for TestEventTriggerRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TestEventTriggerRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for TestEventTriggerRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TestEventTriggerRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for TestEventTriggerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TestEventTriggerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TimeSnapshotResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > TimeSnapshotResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > TimeSnapshotResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn system_time_ms (mut self , value : u64 ) -> Result < TimeSnapshotResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "systemTimeMs" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (TimeSnapshotResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > TimeSnapshotResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn posix_time_ms (mut self , value : crate :: tlv :: Nullable < u64 > ) -> Result < TimeSnapshotResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "posixTimeMs" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (TimeSnapshotResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > TimeSnapshotResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for TimeSnapshotResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TimeSnapshotResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for TimeSnapshotResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TimeSnapshotResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for TimeSnapshotResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TimeSnapshotResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TimeSnapshotResponseArrayBuilder < P > (P ) ;
impl < P > TimeSnapshotResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < TimeSnapshotResponseBuilder < TimeSnapshotResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TimeSnapshotResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for TimeSnapshotResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TimeSnapshotResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for TimeSnapshotResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TimeSnapshotResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for TimeSnapshotResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TimeSnapshotResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PayloadTestRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > PayloadTestRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > PayloadTestRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn enable_key (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < PayloadTestRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "enableKey" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (PayloadTestRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > PayloadTestRequestRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn value (mut self , value : u8 ) -> Result < PayloadTestRequestRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "value" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (PayloadTestRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > PayloadTestRequestRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn count (mut self , value : u16 ) -> Result < PayloadTestRequestRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "count" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (PayloadTestRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > PayloadTestRequestRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for PayloadTestRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PayloadTestRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for PayloadTestRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PayloadTestRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for PayloadTestRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PayloadTestRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PayloadTestRequestRequestArrayBuilder < P > (P ) ;
impl < P > PayloadTestRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < PayloadTestRequestRequestBuilder < PayloadTestRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (PayloadTestRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for PayloadTestRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PayloadTestRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for PayloadTestRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PayloadTestRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for PayloadTestRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PayloadTestRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PayloadTestResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > PayloadTestResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > PayloadTestResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn payload (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < PayloadTestResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "payload" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (PayloadTestResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > PayloadTestResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for PayloadTestResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PayloadTestResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for PayloadTestResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PayloadTestResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for PayloadTestResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PayloadTestResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PayloadTestResponseArrayBuilder < P > (P ) ;
impl < P > PayloadTestResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < PayloadTestResponseBuilder < PayloadTestResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (PayloadTestResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for PayloadTestResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PayloadTestResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for PayloadTestResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PayloadTestResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for PayloadTestResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PayloadTestResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum HardwareFaultChangeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RadioFaultChangeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkFaultChangeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BootReasonTag {
    BootReason = 0 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct HardwareFaultChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > HardwareFaultChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , HardwareFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , HardwareFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for HardwareFaultChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for HardwareFaultChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for HardwareFaultChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "HardwareFaultChange" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for HardwareFaultChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "HardwareFaultChange" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RadioFaultChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RadioFaultChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , RadioFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , RadioFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RadioFaultChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RadioFaultChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RadioFaultChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RadioFaultChange" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RadioFaultChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RadioFaultChange" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct NetworkFaultChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > NetworkFaultChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , NetworkFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , NetworkFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for NetworkFaultChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for NetworkFaultChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for NetworkFaultChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "NetworkFaultChange" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for NetworkFaultChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "NetworkFaultChange" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct BootReason < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > BootReason < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn boot_reason (& self ) -> Result < BootReasonEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for BootReason < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for BootReason < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for BootReason < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "BootReason" ) ? ;
        match self . boot_reason () {
            Ok (value ) => write ! (f , "{}: {:?}," , "boot_reason" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "boot_reason" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for BootReason < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "BootReason" ) ;
        match self . boot_reason () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "boot_reason" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "boot_reason" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct HardwareFaultChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > HardwareFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > HardwareFaultChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < HardwareFaultChangeBuilder < P , 1usize > , HardwareFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (HardwareFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > HardwareFaultChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < HardwareFaultChangeBuilder < P , 2usize > , HardwareFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (HardwareFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > HardwareFaultChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for HardwareFaultChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "HardwareFaultChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for HardwareFaultChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "HardwareFaultChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for HardwareFaultChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for HardwareFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct HardwareFaultChangeArrayBuilder < P > (P ) ;
impl < P > HardwareFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < HardwareFaultChangeBuilder < HardwareFaultChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (HardwareFaultChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for HardwareFaultChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "HardwareFaultChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for HardwareFaultChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "HardwareFaultChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for HardwareFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for HardwareFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RadioFaultChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RadioFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RadioFaultChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < RadioFaultChangeBuilder < P , 1usize > , RadioFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RadioFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > RadioFaultChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < RadioFaultChangeBuilder < P , 2usize > , RadioFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RadioFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > RadioFaultChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RadioFaultChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RadioFaultChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RadioFaultChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RadioFaultChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RadioFaultChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RadioFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RadioFaultChangeArrayBuilder < P > (P ) ;
impl < P > RadioFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RadioFaultChangeBuilder < RadioFaultChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RadioFaultChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RadioFaultChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RadioFaultChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RadioFaultChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RadioFaultChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RadioFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RadioFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NetworkFaultChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > NetworkFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > NetworkFaultChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < NetworkFaultChangeBuilder < P , 1usize > , NetworkFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > NetworkFaultChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < NetworkFaultChangeBuilder < P , 2usize > , NetworkFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > NetworkFaultChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for NetworkFaultChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkFaultChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for NetworkFaultChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkFaultChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for NetworkFaultChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NetworkFaultChangeArrayBuilder < P > (P ) ;
impl < P > NetworkFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < NetworkFaultChangeBuilder < NetworkFaultChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NetworkFaultChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for NetworkFaultChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NetworkFaultChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for NetworkFaultChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NetworkFaultChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for NetworkFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NetworkFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BootReasonBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > BootReasonBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > BootReasonBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn boot_reason (mut self , value : BootReasonEnum ) -> Result < BootReasonBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "bootReason" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (BootReasonBuilder (self . 0 ) ) 
    }
    
}
impl < P > BootReasonBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for BootReasonBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BootReason" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for BootReasonBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BootReason" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for BootReasonBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BootReasonBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BootReasonArrayBuilder < P > (P ) ;
impl < P > BootReasonArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < BootReasonBuilder < BootReasonArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BootReasonArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for BootReasonArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BootReason[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for BootReasonArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BootReason[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for BootReasonArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BootReasonArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl HardwareFaultChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Critical ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (HardwareFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 51 , EventId :: HardwareFaultChange as _ , Self :: PRIORITY , | tw | {
            f (HardwareFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (HardwareFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: HardwareFaultChange as _ , Self :: PRIORITY , | tw | {
            f (HardwareFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl RadioFaultChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Critical ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (RadioFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 51 , EventId :: RadioFaultChange as _ , Self :: PRIORITY , | tw | {
            f (RadioFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (RadioFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: RadioFaultChange as _ , Self :: PRIORITY , | tw | {
            f (RadioFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl NetworkFaultChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Critical ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (NetworkFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 51 , EventId :: NetworkFaultChange as _ , Self :: PRIORITY , | tw | {
            f (NetworkFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (NetworkFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: NetworkFaultChange as _ , Self :: PRIORITY , | tw | {
            f (NetworkFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl BootReason < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Critical ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (BootReasonBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 51 , EventId :: BootReason as _ , Self :: PRIORITY , | tw | {
            f (BootReasonBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (BootReasonBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: BootReason as _ , Self :: PRIORITY , | tw | {
            f (BootReasonBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    NetworkInterfaces = 0 , RebootCount = 1 , UpTime = 2 , TotalOperationalHours = 3 , BootReason = 4 , ActiveHardwareFaults = 5 , ActiveRadioFaults = 6 , ActiveNetworkFaults = 7 , TestEventTriggersEnabled = 8 , DeviceLoadStatus = 10 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: NetworkInterfaces => write ! (f , "{}(0x{:02x})" , "NetworkInterfaces" , AttributeId :: NetworkInterfaces as u32 ) ? , AttributeId :: RebootCount => write ! (f , "{}(0x{:02x})" , "RebootCount" , AttributeId :: RebootCount as u32 ) ? , AttributeId :: UpTime => write ! (f , "{}(0x{:02x})" , "UpTime" , AttributeId :: UpTime as u32 ) ? , AttributeId :: TotalOperationalHours => write ! (f , "{}(0x{:02x})" , "TotalOperationalHours" , AttributeId :: TotalOperationalHours as u32 ) ? , AttributeId :: BootReason => write ! (f , "{}(0x{:02x})" , "BootReason" , AttributeId :: BootReason as u32 ) ? , AttributeId :: ActiveHardwareFaults => write ! (f , "{}(0x{:02x})" , "ActiveHardwareFaults" , AttributeId :: ActiveHardwareFaults as u32 ) ? , AttributeId :: ActiveRadioFaults => write ! (f , "{}(0x{:02x})" , "ActiveRadioFaults" , AttributeId :: ActiveRadioFaults as u32 ) ? , AttributeId :: ActiveNetworkFaults => write ! (f , "{}(0x{:02x})" , "ActiveNetworkFaults" , AttributeId :: ActiveNetworkFaults as u32 ) ? , AttributeId :: TestEventTriggersEnabled => write ! (f , "{}(0x{:02x})" , "TestEventTriggersEnabled" , AttributeId :: TestEventTriggersEnabled as u32 ) ? , AttributeId :: DeviceLoadStatus => write ! (f , "{}(0x{:02x})" , "DeviceLoadStatus" , AttributeId :: DeviceLoadStatus as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: NetworkInterfaces => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NetworkInterfaces" , AttributeId :: NetworkInterfaces as u32 ) , AttributeId :: RebootCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RebootCount" , AttributeId :: RebootCount as u32 ) , AttributeId :: UpTime => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UpTime" , AttributeId :: UpTime as u32 ) , AttributeId :: TotalOperationalHours => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TotalOperationalHours" , AttributeId :: TotalOperationalHours as u32 ) , AttributeId :: BootReason => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BootReason" , AttributeId :: BootReason as u32 ) , AttributeId :: ActiveHardwareFaults => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveHardwareFaults" , AttributeId :: ActiveHardwareFaults as u32 ) , AttributeId :: ActiveRadioFaults => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveRadioFaults" , AttributeId :: ActiveRadioFaults as u32 ) , AttributeId :: ActiveNetworkFaults => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveNetworkFaults" , AttributeId :: ActiveNetworkFaults as u32 ) , AttributeId :: TestEventTriggersEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TestEventTriggersEnabled" , AttributeId :: TestEventTriggersEnabled as u32 ) , AttributeId :: DeviceLoadStatus => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DeviceLoadStatus" , AttributeId :: DeviceLoadStatus as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    TestEventTrigger = 0 , TimeSnapshot = 1 , PayloadTestRequest = 3 
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
            CommandId :: TestEventTrigger => write ! (f , "{}(0x{:02x})" , "TestEventTrigger" , CommandId :: TestEventTrigger as u32 ) ? , CommandId :: TimeSnapshot => write ! (f , "{}(0x{:02x})" , "TimeSnapshot" , CommandId :: TimeSnapshot as u32 ) ? , CommandId :: PayloadTestRequest => write ! (f , "{}(0x{:02x})" , "PayloadTestRequest" , CommandId :: PayloadTestRequest as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: TestEventTrigger => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TestEventTrigger" , CommandId :: TestEventTrigger as u32 ) , CommandId :: TimeSnapshot => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TimeSnapshot" , CommandId :: TimeSnapshot as u32 ) , CommandId :: PayloadTestRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PayloadTestRequest" , CommandId :: PayloadTestRequest as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    TimeSnapshotResponse = 2 , PayloadTestResponse = 4 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: TimeSnapshotResponse => write ! (f , "{}(0x{:02x})" , "TimeSnapshotResponse" , CommandResponseId :: TimeSnapshotResponse as u32 ) ? , CommandResponseId :: PayloadTestResponse => write ! (f , "{}(0x{:02x})" , "PayloadTestResponse" , CommandResponseId :: PayloadTestResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: TimeSnapshotResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TimeSnapshotResponse" , CommandResponseId :: TimeSnapshotResponse as u32 ) , CommandResponseId :: PayloadTestResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PayloadTestResponse" , CommandResponseId :: PayloadTestResponse as u32 ) , 
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
# [doc = "The event IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum EventId {
    HardwareFaultChange = 0 , RadioFaultChange = 1 , NetworkFaultChange = 2 , BootReason = 3 
}
impl core :: convert :: TryFrom < crate :: dm :: EventId > for EventId {
    type Error = crate :: error :: Error ;
    fn try_from (id : crate :: dm :: EventId ) -> Result < Self , Self :: Error > {
        EventId :: from_repr (id ) . ok_or_else (|| crate :: error :: ErrorCode :: EventNotFound . into () ) 
    }
    
}
impl core :: fmt :: Debug for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Event::" ) ? ;
        match self . 0 {
            EventId :: HardwareFaultChange => write ! (f , "{}(0x{:02x})" , "HardwareFaultChange" , EventId :: HardwareFaultChange as u32 ) ? , EventId :: RadioFaultChange => write ! (f , "{}(0x{:02x})" , "RadioFaultChange" , EventId :: RadioFaultChange as u32 ) ? , EventId :: NetworkFaultChange => write ! (f , "{}(0x{:02x})" , "NetworkFaultChange" , EventId :: NetworkFaultChange as u32 ) ? , EventId :: BootReason => write ! (f , "{}(0x{:02x})" , "BootReason" , EventId :: BootReason as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: HardwareFaultChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "HardwareFaultChange" , EventId :: HardwareFaultChange as u32 ) , EventId :: RadioFaultChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RadioFaultChange" , EventId :: RadioFaultChange as u32 ) , EventId :: NetworkFaultChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NetworkFaultChange" , EventId :: NetworkFaultChange as u32 ) , EventId :: BootReason => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BootReason" , EventId :: BootReason as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (51 , 3 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: NetworkInterfaces as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: RebootCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: UpTime as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TotalOperationalHours as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BootReason as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveHardwareFaults as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveRadioFaults as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveNetworkFaults as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: TestEventTriggersEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: DeviceLoadStatus as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: TestEventTrigger as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: TimeSnapshot as _ , Some (CommandResponseId :: TimeSnapshotResponse as _ ) , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: PayloadTestRequest as _ , Some (CommandResponseId :: PayloadTestResponse as _ ) , crate :: dm :: Access :: WM , ) , ] , & [crate :: dm :: Event :: new (EventId :: HardwareFaultChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: RadioFaultChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: NetworkFaultChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: BootReason as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn network_interfaces < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInterfaceArrayBuilder < P > , NetworkInterfaceBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn reboot_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > ;
    fn up_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u64 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn total_operational_hours (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn boot_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BootReasonEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_hardware_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , HardwareFaultEnum > , crate :: tlv :: ToTLVBuilder < P , HardwareFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_radio_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , RadioFaultEnum > , crate :: tlv :: ToTLVBuilder < P , RadioFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_network_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn test_event_triggers_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > ;
    fn device_load_status < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : DeviceLoadStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_test_event_trigger (& self , ctx : impl crate :: dm :: InvokeContext , request : TestEventTriggerRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_time_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : TimeSnapshotResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_payload_test_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : PayloadTestRequestRequest < '_ > , response : PayloadTestResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
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
    fn network_interfaces < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInterfaceArrayBuilder < P > , NetworkInterfaceBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: network_interfaces (self , ctx , builder ) 
    }
    fn reboot_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: reboot_count (self , ctx ) 
    }
    fn up_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u64 , crate :: error :: Error > {
        T :: up_time (self , ctx ) 
    }
    fn total_operational_hours (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: total_operational_hours (self , ctx ) 
    }
    fn boot_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BootReasonEnum , crate :: error :: Error > {
        T :: boot_reason (self , ctx ) 
    }
    fn active_hardware_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , HardwareFaultEnum > , crate :: tlv :: ToTLVBuilder < P , HardwareFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: active_hardware_faults (self , ctx , builder ) 
    }
    fn active_radio_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , RadioFaultEnum > , crate :: tlv :: ToTLVBuilder < P , RadioFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: active_radio_faults (self , ctx , builder ) 
    }
    fn active_network_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: active_network_faults (self , ctx , builder ) 
    }
    fn test_event_triggers_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: test_event_triggers_enabled (self , ctx ) 
    }
    fn device_load_status < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : DeviceLoadStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: device_load_status (self , ctx , builder ) 
    }
    fn handle_test_event_trigger (& self , ctx : impl crate :: dm :: InvokeContext , request : TestEventTriggerRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_test_event_trigger (self , ctx , request ) 
    }
    fn handle_time_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : TimeSnapshotResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_time_snapshot (self , ctx , response ) 
    }
    fn handle_payload_test_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : PayloadTestRequestRequest < '_ > , response : PayloadTestResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_payload_test_request (self , ctx , request , response ) 
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
                    AttributeId :: NetworkInterfaces => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkInterfaces , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. network_interfaces (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkInterfaces , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkInterfaces , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RebootCount => {
                        let attr_read_result = self . 0. reboot_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RebootCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UpTime => {
                        let attr_read_result = self . 0. up_time (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TotalOperationalHours => {
                        let attr_read_result = self . 0. total_operational_hours (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TotalOperationalHours , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BootReason => {
                        let attr_read_result = self . 0. boot_reason (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BootReason , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveHardwareFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveHardwareFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_hardware_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveHardwareFaults , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveHardwareFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveRadioFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveRadioFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_radio_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveRadioFaults , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveRadioFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveNetworkFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_network_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaults , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: TestEventTriggersEnabled => {
                        let attr_read_result = self . 0. test_event_triggers_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TestEventTriggersEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DeviceLoadStatus => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DeviceLoadStatus , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. device_load_status (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DeviceLoadStatus , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DeviceLoadStatus , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
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
            _ => {
                crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                return Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    # [allow (unreachable_code ) ] fn invoke (& self , ctx : impl crate :: dm :: InvokeContext , reply : impl crate :: dm :: InvokeReply , ) -> Result < () , crate :: error :: Error > {
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: TestEventTrigger => {
                let cmd_data : TestEventTriggerRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_test_event_trigger (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TestEventTrigger ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: TimeSnapshot => {
                crate :: dm :: log_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TimeSnapshot ) ) ) ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_time_snapshot (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TimeSnapshot ) ) ) , tw ) , tag , ) ? , ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TimeSnapshot ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: PayloadTestRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: PayloadTestRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (4u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_payload_test_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: PayloadTestRequest ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: PayloadTestRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 51u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralDiagnostics" , 51u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralDiagnostics" , 51u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `GeneralDiagnostics` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct GeneralDiagnosticsCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > GeneralDiagnosticsCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn test_event_trigger (self , endpoint : crate :: dm :: EndptId , ) -> Result < TestEventTriggerRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 51 , 0 ) ? . data_builder () 
    }
    pub fn time_snapshot (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        let array = self . array . push () ? . path (endpoint , 51 , 1 ) ? . data (| w | {
            w . start_struct (& crate :: tlv :: TLVTag :: Context (crate :: im :: CmdDataTag :: Data as u8 , ) ) ? ;
            w . end_container () 
        }
        ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn payload_test_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < PayloadTestRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 51 , 3 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GeneralDiagnostics` cluster's commands. `use` this trait to call `.general_diagnostics_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`GeneralDiagnosticsCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GeneralDiagnosticsCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn general_diagnostics_inv (self ) -> GeneralDiagnosticsCmdRequestsView < P > ;
    
}
impl < P > GeneralDiagnosticsCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn general_diagnostics_inv (self ) -> GeneralDiagnosticsCmdRequestsView < P > {
        GeneralDiagnosticsCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `GeneralDiagnostics` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralDiagnosticsAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > GeneralDiagnosticsAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_interfaces (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn reboot_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn up_time (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn total_operational_hours (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn boot_reason (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_hardware_faults (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_radio_faults (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_network_faults (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn test_event_triggers_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn device_load_status (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (51 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GeneralDiagnostics` cluster's attribute reads. `use` this trait to call `.general_diagnostics_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`GeneralDiagnosticsAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GeneralDiagnosticsAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn general_diagnostics_read (self ) -> GeneralDiagnosticsAttrReadsView < P > ;
    
}
impl < P > GeneralDiagnosticsAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn general_diagnostics_read (self ) -> GeneralDiagnosticsAttrReadsView < P > {
        GeneralDiagnosticsAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `GeneralDiagnostics` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralDiagnosticsAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > GeneralDiagnosticsAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GeneralDiagnostics` cluster's attribute writes. `use` this trait to call `.general_diagnostics_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`GeneralDiagnosticsAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GeneralDiagnosticsAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn general_diagnostics_write (self ) -> GeneralDiagnosticsAttrWritesView < P > ;
    
}
impl < P > GeneralDiagnosticsAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn general_diagnostics_write (self ) -> GeneralDiagnosticsAttrWritesView < P > {
        GeneralDiagnosticsAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `GeneralDiagnostics` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct GeneralDiagnosticsCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > GeneralDiagnosticsCmdResponsesView < 'a , 'r > {
    pub fn test_event_trigger (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (51 , 0 ) 
    }
    pub fn time_snapshot (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < TimeSnapshotResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < TimeSnapshotResponse < 'a >> (51 , 1 ) 
    }
    pub fn payload_test_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < PayloadTestResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < PayloadTestResponse < 'a >> (51 , 3 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GeneralDiagnostics`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.general_diagnostics_inv_resp()` on an `InvokeResp`; the returned [`GeneralDiagnosticsCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait GeneralDiagnosticsCmdResponses < 'a > {
    fn general_diagnostics_inv_resp (& self ) -> GeneralDiagnosticsCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > GeneralDiagnosticsCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn general_diagnostics_inv_resp (& self ) -> GeneralDiagnosticsCmdResponsesView < 'a , '_ > {
        GeneralDiagnosticsCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `GeneralDiagnostics` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralDiagnosticsAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > GeneralDiagnosticsAttrResponsesView < 'a , 'r > {
    pub fn network_interfaces (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , NetworkInterface < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , NetworkInterface < 'a > > > (51 , 0 ) 
    }
    pub fn reboot_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (51 , 1 ) 
    }
    pub fn up_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u64 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u64 > (51 , 2 ) 
    }
    pub fn total_operational_hours (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (51 , 3 ) 
    }
    pub fn boot_reason (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < BootReasonEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < BootReasonEnum > (51 , 4 ) 
    }
    pub fn active_hardware_faults (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , HardwareFaultEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , HardwareFaultEnum > > (51 , 5 ) 
    }
    pub fn active_radio_faults (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , RadioFaultEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , RadioFaultEnum > > (51 , 6 ) 
    }
    pub fn active_network_faults (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , NetworkFaultEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , NetworkFaultEnum > > (51 , 7 ) 
    }
    pub fn test_event_triggers_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (51 , 8 ) 
    }
    pub fn device_load_status (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < DeviceLoadStruct < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < DeviceLoadStruct < 'a > > (51 , 10 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (51 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (51 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (51 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (51 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (51 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GeneralDiagnostics`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.general_diagnostics_read_resp()` on a `ReportDataResp`; the returned [`GeneralDiagnosticsAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait GeneralDiagnosticsAttrResponses < 'a > {
    fn general_diagnostics_read_resp (& self ) -> GeneralDiagnosticsAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > GeneralDiagnosticsAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn general_diagnostics_read_resp (& self ) -> GeneralDiagnosticsAttrResponsesView < 'a , '_ > {
        GeneralDiagnosticsAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `GeneralDiagnostics` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralDiagnosticsWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > GeneralDiagnosticsWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `GeneralDiagnostics`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.general_diagnostics_write_resp()` on a `WriteResp`; the returned [`GeneralDiagnosticsWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait GeneralDiagnosticsWriteResponses < 'a > {
    fn general_diagnostics_write_resp (& self ) -> GeneralDiagnosticsWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > GeneralDiagnosticsWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn general_diagnostics_write_resp (& self ) -> GeneralDiagnosticsWriteResponsesView < 'a , '_ > {
        GeneralDiagnosticsWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `TimeSnapshotResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`TimeSnapshotResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct TimeSnapshotResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > TimeSnapshotResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < TimeSnapshotResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `PayloadTestResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`PayloadTestResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct PayloadTestResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > PayloadTestResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < PayloadTestResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `GeneralDiagnostics` cluster's client operations. Returned by [`GeneralDiagnosticsClient::general_diagnostics`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct GeneralDiagnosticsClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > GeneralDiagnosticsClientView < T > {
    pub fn test_event_trigger < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (TestEventTriggerRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GeneralDiagnosticsCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . general_diagnostics_inv () ;
            let req_builder = view . test_event_trigger (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn time_snapshot (self , endpoint : crate :: dm :: EndptId , ) -> Result < TimeSnapshotResponseHandle < T :: InvokeChunk > , crate :: error :: Error > {
        use self :: GeneralDiagnosticsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            msg . invoke_requests () ? . general_diagnostics_inv () . time_snapshot (endpoint ) ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (TimeSnapshotResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn payload_test_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < PayloadTestResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (PayloadTestRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GeneralDiagnosticsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . general_diagnostics_inv () ;
            let req_builder = view . payload_test_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (PayloadTestResponseHandle {
            chunk 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn network_interfaces_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , NetworkInterface < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . network_interfaces (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn reboot_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . reboot_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn up_time_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . up_time (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn total_operational_hours_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . total_operational_hours (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn boot_reason_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < BootReasonEnum , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . boot_reason (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn active_hardware_faults_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , HardwareFaultEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . active_hardware_faults (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn active_radio_faults_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , RadioFaultEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . active_radio_faults (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn active_network_faults_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , NetworkFaultEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . active_network_faults (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn test_event_triggers_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . test_event_triggers_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn device_load_status_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < DeviceLoadStruct < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . device_load_status (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GeneralDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_diagnostics_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `GeneralDiagnostics` cluster. `use` this trait to call `.general_diagnostics()` on any [`crate::im::ImClient`], such as an exchange; the returned [`GeneralDiagnosticsClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `GeneralDiagnosticsClientView` keeps each cluster's API surface narrow: at the call site `exchange.general_diagnostics().` shows only this cluster's methods in IDE completion." ] pub trait GeneralDiagnosticsClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn general_diagnostics (self ) -> GeneralDiagnosticsClientView < Self > {
        GeneralDiagnosticsClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > GeneralDiagnosticsClient for T {
    
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
    fn network_interfaces < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInterfaceArrayBuilder < P > , NetworkInterfaceBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn reboot_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn up_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn total_operational_hours (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn boot_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BootReasonEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_hardware_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , HardwareFaultEnum > , crate :: tlv :: ToTLVBuilder < P , HardwareFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_radio_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , RadioFaultEnum > , crate :: tlv :: ToTLVBuilder < P , RadioFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_network_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn test_event_triggers_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn device_load_status < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : DeviceLoadStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_test_event_trigger (& self , ctx : impl crate :: dm :: InvokeContext , request : TestEventTriggerRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_time_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : TimeSnapshotResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_payload_test_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : PayloadTestRequestRequest < '_ > , response : PayloadTestResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn network_interfaces < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NetworkInterfaceArrayBuilder < P > , NetworkInterfaceBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: network_interfaces (self , ctx , builder ) 
    }
    fn reboot_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: reboot_count (self , ctx ) 
    }
    fn up_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> {
        T :: up_time (self , ctx ) 
    }
    fn total_operational_hours (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: total_operational_hours (self , ctx ) 
    }
    fn boot_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BootReasonEnum , crate :: error :: Error >> {
        T :: boot_reason (self , ctx ) 
    }
    fn active_hardware_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , HardwareFaultEnum > , crate :: tlv :: ToTLVBuilder < P , HardwareFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_hardware_faults (self , ctx , builder ) 
    }
    fn active_radio_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , RadioFaultEnum > , crate :: tlv :: ToTLVBuilder < P , RadioFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_radio_faults (self , ctx , builder ) 
    }
    fn active_network_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_network_faults (self , ctx , builder ) 
    }
    fn test_event_triggers_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: test_event_triggers_enabled (self , ctx ) 
    }
    fn device_load_status < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : DeviceLoadStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: device_load_status (self , ctx , builder ) 
    }
    fn handle_test_event_trigger (& self , ctx : impl crate :: dm :: InvokeContext , request : TestEventTriggerRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_test_event_trigger (self , ctx , request ) 
    }
    fn handle_time_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : TimeSnapshotResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_time_snapshot (self , ctx , response ) 
    }
    fn handle_payload_test_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : PayloadTestRequestRequest < '_ > , response : PayloadTestResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_payload_test_request (self , ctx , request , response ) 
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
                    AttributeId :: NetworkInterfaces => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkInterfaces , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. network_interfaces (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkInterfaces , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkInterfaces , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RebootCount => {
                        let attr_read_result = self . 0. reboot_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RebootCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UpTime => {
                        let attr_read_result = self . 0. up_time (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TotalOperationalHours => {
                        let attr_read_result = self . 0. total_operational_hours (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TotalOperationalHours , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BootReason => {
                        let attr_read_result = self . 0. boot_reason (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BootReason , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveHardwareFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveHardwareFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_hardware_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveHardwareFaults , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveHardwareFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveRadioFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveRadioFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_radio_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveRadioFaults , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveRadioFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveNetworkFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_network_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaults , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: TestEventTriggersEnabled => {
                        let attr_read_result = self . 0. test_event_triggers_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TestEventTriggersEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DeviceLoadStatus => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DeviceLoadStatus , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. device_load_status (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DeviceLoadStatus , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DeviceLoadStatus , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
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
            _ => {
                crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                return Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    # [allow (unreachable_code ) ] async fn invoke (& self , ctx : impl crate :: dm :: InvokeContext , reply : impl crate :: dm :: InvokeReply , ) -> Result < () , crate :: error :: Error > {
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: TestEventTrigger => {
                let cmd_data : TestEventTriggerRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_test_event_trigger (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TestEventTrigger ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: TimeSnapshot => {
                crate :: dm :: log_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TimeSnapshot ) ) ) ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_time_snapshot (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TimeSnapshot ) ) ) , tw ) , tag , ) ? , ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: TimeSnapshot ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: PayloadTestRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: PayloadTestRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (4u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_payload_test_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: PayloadTestRequest ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: PayloadTestRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 51u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralDiagnostics" , 51u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralDiagnostics" , 51u32 , self . 0.2 ) 
    }
    
}
