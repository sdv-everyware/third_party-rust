# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const PACKET_COUNTS = 1 ;
        const ERROR_COUNTS = 2 ;
        const MLE_COUNTS = 4 ;
        const MAC_COUNTS = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const PACKET_COUNTS = 1 ;
        const ERROR_COUNTS = 2 ;
        const MLE_COUNTS = 4 ;
        const MAC_COUNTS = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ConnectionStatusEnum {
    # [enumval (0 ) ] Connected = 0 , # [enumval (1 ) ] NotConnected = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkFaultEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] LinkDown = 1 , # [enumval (2 ) ] HardwareFailure = 2 , # [enumval (3 ) ] NetworkJammed = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RoutingRoleEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] Unassigned = 1 , # [enumval (2 ) ] SleepyEndDevice = 2 , # [enumval (3 ) ] EndDevice = 3 , # [enumval (4 ) ] REED = 4 , # [enumval (5 ) ] Router = 5 , # [enumval (6 ) ] Leader = 6 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NeighborTableStructTag {
    ExtAddress = 0 , Age = 1 , Rloc16 = 2 , LinkFrameCounter = 3 , MleFrameCounter = 4 , Lqi = 5 , AverageRssi = 6 , LastRssi = 7 , FrameErrorRate = 8 , MessageErrorRate = 9 , RxOnWhenIdle = 10 , FullThreadDevice = 11 , FullNetworkData = 12 , IsChild = 13 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum OperationalDatasetComponentsTag {
    ActiveTimestampPresent = 0 , PendingTimestampPresent = 1 , MasterKeyPresent = 2 , NetworkNamePresent = 3 , ExtendedPanIdPresent = 4 , MeshLocalPrefixPresent = 5 , DelayPresent = 6 , PanIdPresent = 7 , ChannelPresent = 8 , PskcPresent = 9 , SecurityPolicyPresent = 10 , ChannelMaskPresent = 11 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RouteTableStructTag {
    ExtAddress = 0 , Rloc16 = 1 , RouterId = 2 , NextHop = 3 , PathCost = 4 , LqiIn = 5 , LqiOut = 6 , Age = 7 , Allocated = 8 , LinkEstablished = 9 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SecurityPolicyTag {
    RotationTime = 0 , Flags = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct NeighborTableStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > NeighborTableStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn ext_address (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn age (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn rloc_16 (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn link_frame_counter (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn mle_frame_counter (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn lqi (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn average_rssi (& self ) -> Result < crate :: tlv :: Nullable < i8 > , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn last_rssi (& self ) -> Result < crate :: tlv :: Nullable < i8 > , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn frame_error_rate (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (8 ) 
    }
    pub fn message_error_rate (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (9 ) 
    }
    pub fn rx_on_when_idle (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (10 ) 
    }
    pub fn full_thread_device (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (11 ) 
    }
    pub fn full_network_data (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (12 ) 
    }
    pub fn is_child (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (13 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for NeighborTableStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for NeighborTableStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for NeighborTableStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "NeighborTableStruct" ) ? ;
        match self . ext_address () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ext_address" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ext_address" , e . code () ) ? , 
        }
        match self . age () {
            Ok (value ) => write ! (f , "{}: {:?}," , "age" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "age" , e . code () ) ? , 
        }
        match self . rloc_16 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rloc_16" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rloc_16" , e . code () ) ? , 
        }
        match self . link_frame_counter () {
            Ok (value ) => write ! (f , "{}: {:?}," , "link_frame_counter" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "link_frame_counter" , e . code () ) ? , 
        }
        match self . mle_frame_counter () {
            Ok (value ) => write ! (f , "{}: {:?}," , "mle_frame_counter" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mle_frame_counter" , e . code () ) ? , 
        }
        match self . lqi () {
            Ok (value ) => write ! (f , "{}: {:?}," , "lqi" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "lqi" , e . code () ) ? , 
        }
        match self . average_rssi () {
            Ok (value ) => write ! (f , "{}: {:?}," , "average_rssi" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "average_rssi" , e . code () ) ? , 
        }
        match self . last_rssi () {
            Ok (value ) => write ! (f , "{}: {:?}," , "last_rssi" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "last_rssi" , e . code () ) ? , 
        }
        match self . frame_error_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "frame_error_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "frame_error_rate" , e . code () ) ? , 
        }
        match self . message_error_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "message_error_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "message_error_rate" , e . code () ) ? , 
        }
        match self . rx_on_when_idle () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rx_on_when_idle" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rx_on_when_idle" , e . code () ) ? , 
        }
        match self . full_thread_device () {
            Ok (value ) => write ! (f , "{}: {:?}," , "full_thread_device" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "full_thread_device" , e . code () ) ? , 
        }
        match self . full_network_data () {
            Ok (value ) => write ! (f , "{}: {:?}," , "full_network_data" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "full_network_data" , e . code () ) ? , 
        }
        match self . is_child () {
            Ok (value ) => write ! (f , "{}: {:?}," , "is_child" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "is_child" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for NeighborTableStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "NeighborTableStruct" ) ;
        match self . ext_address () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ext_address" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ext_address" , e . code () ) , 
        }
        match self . age () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "age" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "age" , e . code () ) , 
        }
        match self . rloc_16 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rloc_16" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rloc_16" , e . code () ) , 
        }
        match self . link_frame_counter () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "link_frame_counter" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "link_frame_counter" , e . code () ) , 
        }
        match self . mle_frame_counter () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "mle_frame_counter" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mle_frame_counter" , e . code () ) , 
        }
        match self . lqi () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "lqi" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "lqi" , e . code () ) , 
        }
        match self . average_rssi () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "average_rssi" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "average_rssi" , e . code () ) , 
        }
        match self . last_rssi () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "last_rssi" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "last_rssi" , e . code () ) , 
        }
        match self . frame_error_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "frame_error_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "frame_error_rate" , e . code () ) , 
        }
        match self . message_error_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "message_error_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "message_error_rate" , e . code () ) , 
        }
        match self . rx_on_when_idle () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rx_on_when_idle" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rx_on_when_idle" , e . code () ) , 
        }
        match self . full_thread_device () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "full_thread_device" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "full_thread_device" , e . code () ) , 
        }
        match self . full_network_data () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "full_network_data" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "full_network_data" , e . code () ) , 
        }
        match self . is_child () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "is_child" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "is_child" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct OperationalDatasetComponents < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > OperationalDatasetComponents < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn active_timestamp_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn pending_timestamp_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn master_key_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn network_name_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn extended_pan_id_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn mesh_local_prefix_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn delay_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn pan_id_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn channel_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (8 ) 
    }
    pub fn pskc_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (9 ) 
    }
    pub fn security_policy_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (10 ) 
    }
    pub fn channel_mask_present (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (11 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for OperationalDatasetComponents < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for OperationalDatasetComponents < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for OperationalDatasetComponents < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "OperationalDatasetComponents" ) ? ;
        match self . active_timestamp_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "active_timestamp_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "active_timestamp_present" , e . code () ) ? , 
        }
        match self . pending_timestamp_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "pending_timestamp_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "pending_timestamp_present" , e . code () ) ? , 
        }
        match self . master_key_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "master_key_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "master_key_present" , e . code () ) ? , 
        }
        match self . network_name_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "network_name_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "network_name_present" , e . code () ) ? , 
        }
        match self . extended_pan_id_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "extended_pan_id_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "extended_pan_id_present" , e . code () ) ? , 
        }
        match self . mesh_local_prefix_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "mesh_local_prefix_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mesh_local_prefix_present" , e . code () ) ? , 
        }
        match self . delay_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "delay_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "delay_present" , e . code () ) ? , 
        }
        match self . pan_id_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "pan_id_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "pan_id_present" , e . code () ) ? , 
        }
        match self . channel_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "channel_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel_present" , e . code () ) ? , 
        }
        match self . pskc_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "pskc_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "pskc_present" , e . code () ) ? , 
        }
        match self . security_policy_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "security_policy_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "security_policy_present" , e . code () ) ? , 
        }
        match self . channel_mask_present () {
            Ok (value ) => write ! (f , "{}: {:?}," , "channel_mask_present" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel_mask_present" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for OperationalDatasetComponents < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "OperationalDatasetComponents" ) ;
        match self . active_timestamp_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "active_timestamp_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "active_timestamp_present" , e . code () ) , 
        }
        match self . pending_timestamp_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "pending_timestamp_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "pending_timestamp_present" , e . code () ) , 
        }
        match self . master_key_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "master_key_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "master_key_present" , e . code () ) , 
        }
        match self . network_name_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "network_name_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "network_name_present" , e . code () ) , 
        }
        match self . extended_pan_id_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "extended_pan_id_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "extended_pan_id_present" , e . code () ) , 
        }
        match self . mesh_local_prefix_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "mesh_local_prefix_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mesh_local_prefix_present" , e . code () ) , 
        }
        match self . delay_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "delay_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "delay_present" , e . code () ) , 
        }
        match self . pan_id_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "pan_id_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "pan_id_present" , e . code () ) , 
        }
        match self . channel_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "channel_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel_present" , e . code () ) , 
        }
        match self . pskc_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "pskc_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "pskc_present" , e . code () ) , 
        }
        match self . security_policy_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "security_policy_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "security_policy_present" , e . code () ) , 
        }
        match self . channel_mask_present () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "channel_mask_present" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel_mask_present" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RouteTableStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RouteTableStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn ext_address (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rloc_16 (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn router_id (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn next_hop (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn path_cost (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn lqi_in (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn lqi_out (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn age (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn allocated (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (8 ) 
    }
    pub fn link_established (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (9 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RouteTableStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RouteTableStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RouteTableStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RouteTableStruct" ) ? ;
        match self . ext_address () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ext_address" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ext_address" , e . code () ) ? , 
        }
        match self . rloc_16 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rloc_16" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rloc_16" , e . code () ) ? , 
        }
        match self . router_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "router_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "router_id" , e . code () ) ? , 
        }
        match self . next_hop () {
            Ok (value ) => write ! (f , "{}: {:?}," , "next_hop" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "next_hop" , e . code () ) ? , 
        }
        match self . path_cost () {
            Ok (value ) => write ! (f , "{}: {:?}," , "path_cost" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "path_cost" , e . code () ) ? , 
        }
        match self . lqi_in () {
            Ok (value ) => write ! (f , "{}: {:?}," , "lqi_in" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "lqi_in" , e . code () ) ? , 
        }
        match self . lqi_out () {
            Ok (value ) => write ! (f , "{}: {:?}," , "lqi_out" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "lqi_out" , e . code () ) ? , 
        }
        match self . age () {
            Ok (value ) => write ! (f , "{}: {:?}," , "age" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "age" , e . code () ) ? , 
        }
        match self . allocated () {
            Ok (value ) => write ! (f , "{}: {:?}," , "allocated" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "allocated" , e . code () ) ? , 
        }
        match self . link_established () {
            Ok (value ) => write ! (f , "{}: {:?}," , "link_established" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "link_established" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RouteTableStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RouteTableStruct" ) ;
        match self . ext_address () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ext_address" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ext_address" , e . code () ) , 
        }
        match self . rloc_16 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rloc_16" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rloc_16" , e . code () ) , 
        }
        match self . router_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "router_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "router_id" , e . code () ) , 
        }
        match self . next_hop () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "next_hop" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "next_hop" , e . code () ) , 
        }
        match self . path_cost () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "path_cost" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "path_cost" , e . code () ) , 
        }
        match self . lqi_in () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "lqi_in" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "lqi_in" , e . code () ) , 
        }
        match self . lqi_out () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "lqi_out" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "lqi_out" , e . code () ) , 
        }
        match self . age () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "age" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "age" , e . code () ) , 
        }
        match self . allocated () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "allocated" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "allocated" , e . code () ) , 
        }
        match self . link_established () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "link_established" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "link_established" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SecurityPolicy < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SecurityPolicy < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn rotation_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn flags (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SecurityPolicy < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SecurityPolicy < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SecurityPolicy < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SecurityPolicy" ) ? ;
        match self . rotation_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rotation_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rotation_time" , e . code () ) ? , 
        }
        match self . flags () {
            Ok (value ) => write ! (f , "{}: {:?}," , "flags" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "flags" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SecurityPolicy < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SecurityPolicy" ) ;
        match self . rotation_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rotation_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rotation_time" , e . code () ) , 
        }
        match self . flags () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "flags" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "flags" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct NeighborTableStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > NeighborTableStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ext_address (mut self , value : u64 ) -> Result < NeighborTableStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "extAddress" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn age (mut self , value : u32 ) -> Result < NeighborTableStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "age" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rloc_16 (mut self , value : u16 ) -> Result < NeighborTableStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rloc16" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn link_frame_counter (mut self , value : u32 ) -> Result < NeighborTableStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "linkFrameCounter" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mle_frame_counter (mut self , value : u32 ) -> Result < NeighborTableStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "mleFrameCounter" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn lqi (mut self , value : u8 ) -> Result < NeighborTableStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "lqi" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn average_rssi (mut self , value : crate :: tlv :: Nullable < i8 > ) -> Result < NeighborTableStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "averageRssi" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn last_rssi (mut self , value : crate :: tlv :: Nullable < i8 > ) -> Result < NeighborTableStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "lastRssi" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn frame_error_rate (mut self , value : u8 ) -> Result < NeighborTableStructBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "frameErrorRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn message_error_rate (mut self , value : u8 ) -> Result < NeighborTableStructBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "messageErrorRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rx_on_when_idle (mut self , value : bool ) -> Result < NeighborTableStructBuilder < P , 11usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rxOnWhenIdle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 10 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 11 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn full_thread_device (mut self , value : bool ) -> Result < NeighborTableStructBuilder < P , 12usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fullThreadDevice" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 11 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 12 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn full_network_data (mut self , value : bool ) -> Result < NeighborTableStructBuilder < P , 13usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fullNetworkData" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 12 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 13 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn is_child (mut self , value : bool ) -> Result < NeighborTableStructBuilder < P , 14usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "isChild" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 13 , & value ) ? ;
        Ok (NeighborTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NeighborTableStructBuilder < P , 14usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for NeighborTableStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NeighborTableStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for NeighborTableStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NeighborTableStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for NeighborTableStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NeighborTableStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NeighborTableStructArrayBuilder < P > (P ) ;
impl < P > NeighborTableStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < NeighborTableStructBuilder < NeighborTableStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NeighborTableStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for NeighborTableStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NeighborTableStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for NeighborTableStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NeighborTableStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for NeighborTableStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NeighborTableStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct OperationalDatasetComponentsBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > OperationalDatasetComponentsBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn active_timestamp_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "activeTimestampPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn pending_timestamp_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "pendingTimestampPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn master_key_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "masterKeyPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn network_name_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "networkNamePresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn extended_pan_id_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "extendedPanIdPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mesh_local_prefix_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "meshLocalPrefixPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn delay_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "delayPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn pan_id_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "panIdPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "channelPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn pskc_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "pskcPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn security_policy_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 11usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "securityPolicyPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 10 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 11 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel_mask_present (mut self , value : bool ) -> Result < OperationalDatasetComponentsBuilder < P , 12usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "channelMaskPresent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 11 , & value ) ? ;
        Ok (OperationalDatasetComponentsBuilder (self . 0 ) ) 
    }
    
}
impl < P > OperationalDatasetComponentsBuilder < P , 12usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for OperationalDatasetComponentsBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "OperationalDatasetComponents" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for OperationalDatasetComponentsBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "OperationalDatasetComponents" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for OperationalDatasetComponentsBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for OperationalDatasetComponentsBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct OperationalDatasetComponentsArrayBuilder < P > (P ) ;
impl < P > OperationalDatasetComponentsArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < OperationalDatasetComponentsBuilder < OperationalDatasetComponentsArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (OperationalDatasetComponentsArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for OperationalDatasetComponentsArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "OperationalDatasetComponents[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for OperationalDatasetComponentsArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "OperationalDatasetComponents[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for OperationalDatasetComponentsArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for OperationalDatasetComponentsArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RouteTableStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RouteTableStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ext_address (mut self , value : u64 ) -> Result < RouteTableStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "extAddress" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rloc_16 (mut self , value : u16 ) -> Result < RouteTableStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rloc16" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn router_id (mut self , value : u8 ) -> Result < RouteTableStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "routerId" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn next_hop (mut self , value : u8 ) -> Result < RouteTableStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "nextHop" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn path_cost (mut self , value : u8 ) -> Result < RouteTableStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "pathCost" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn lqi_in (mut self , value : u8 ) -> Result < RouteTableStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "LQIIn" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn lqi_out (mut self , value : u8 ) -> Result < RouteTableStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "LQIOut" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn age (mut self , value : u8 ) -> Result < RouteTableStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "age" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn allocated (mut self , value : bool ) -> Result < RouteTableStructBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "allocated" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn link_established (mut self , value : bool ) -> Result < RouteTableStructBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "linkEstablished" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (RouteTableStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RouteTableStructBuilder < P , 10usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RouteTableStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RouteTableStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RouteTableStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RouteTableStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RouteTableStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RouteTableStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RouteTableStructArrayBuilder < P > (P ) ;
impl < P > RouteTableStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RouteTableStructBuilder < RouteTableStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RouteTableStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RouteTableStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RouteTableStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RouteTableStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RouteTableStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RouteTableStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RouteTableStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SecurityPolicyBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SecurityPolicyBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SecurityPolicyBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rotation_time (mut self , value : u16 ) -> Result < SecurityPolicyBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rotationTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SecurityPolicyBuilder (self . 0 ) ) 
    }
    
}
impl < P > SecurityPolicyBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn flags (mut self , value : u16 ) -> Result < SecurityPolicyBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "flags" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SecurityPolicyBuilder (self . 0 ) ) 
    }
    
}
impl < P > SecurityPolicyBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SecurityPolicyBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SecurityPolicy" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SecurityPolicyBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SecurityPolicy" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SecurityPolicyBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SecurityPolicyBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SecurityPolicyArrayBuilder < P > (P ) ;
impl < P > SecurityPolicyArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SecurityPolicyBuilder < SecurityPolicyArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SecurityPolicyArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SecurityPolicyArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SecurityPolicy[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SecurityPolicyArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SecurityPolicy[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SecurityPolicyArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SecurityPolicyArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ConnectionStatusTag {
    ConnectionStatus = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkFaultChangeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ConnectionStatus < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ConnectionStatus < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_status (& self ) -> Result < ConnectionStatusEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ConnectionStatus < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ConnectionStatus < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ConnectionStatus < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ConnectionStatus" ) ? ;
        match self . connection_status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_status" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ConnectionStatus < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ConnectionStatus" ) ;
        match self . connection_status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_status" , e . code () ) , 
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
pub struct ConnectionStatusBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ConnectionStatusBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ConnectionStatusBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_status (mut self , value : ConnectionStatusEnum ) -> Result < ConnectionStatusBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionStatus" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ConnectionStatusBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConnectionStatusBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ConnectionStatusBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConnectionStatus" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ConnectionStatusBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConnectionStatus" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ConnectionStatusBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConnectionStatusBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ConnectionStatusArrayBuilder < P > (P ) ;
impl < P > ConnectionStatusArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ConnectionStatusBuilder < ConnectionStatusArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ConnectionStatusArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ConnectionStatusArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConnectionStatus[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ConnectionStatusArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConnectionStatus[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ConnectionStatusArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConnectionStatusArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
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
impl ConnectionStatus < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ConnectionStatusBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 53 , EventId :: ConnectionStatus as _ , Self :: PRIORITY , | tw | {
            f (ConnectionStatusBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ConnectionStatusBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: ConnectionStatus as _ , Self :: PRIORITY , | tw | {
            f (ConnectionStatusBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl NetworkFaultChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (NetworkFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 53 , EventId :: NetworkFaultChange as _ , Self :: PRIORITY , | tw | {
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
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    Channel = 0 , RoutingRole = 1 , NetworkName = 2 , PanId = 3 , ExtendedPanId = 4 , MeshLocalPrefix = 5 , OverrunCount = 6 , NeighborTable = 7 , RouteTable = 8 , PartitionId = 9 , Weighting = 10 , DataVersion = 11 , StableDataVersion = 12 , LeaderRouterId = 13 , DetachedRoleCount = 14 , ChildRoleCount = 15 , RouterRoleCount = 16 , LeaderRoleCount = 17 , AttachAttemptCount = 18 , PartitionIdChangeCount = 19 , BetterPartitionAttachAttemptCount = 20 , ParentChangeCount = 21 , TxTotalCount = 22 , TxUnicastCount = 23 , TxBroadcastCount = 24 , TxAckRequestedCount = 25 , TxAckedCount = 26 , TxNoAckRequestedCount = 27 , TxDataCount = 28 , TxDataPollCount = 29 , TxBeaconCount = 30 , TxBeaconRequestCount = 31 , TxOtherCount = 32 , TxRetryCount = 33 , TxDirectMaxRetryExpiryCount = 34 , TxIndirectMaxRetryExpiryCount = 35 , TxErrCcaCount = 36 , TxErrAbortCount = 37 , TxErrBusyChannelCount = 38 , RxTotalCount = 39 , RxUnicastCount = 40 , RxBroadcastCount = 41 , RxDataCount = 42 , RxDataPollCount = 43 , RxBeaconCount = 44 , RxBeaconRequestCount = 45 , RxOtherCount = 46 , RxAddressFilteredCount = 47 , RxDestAddrFilteredCount = 48 , RxDuplicatedCount = 49 , RxErrNoFrameCount = 50 , RxErrUnknownNeighborCount = 51 , RxErrInvalidSrcAddrCount = 52 , RxErrSecCount = 53 , RxErrFcsCount = 54 , RxErrOtherCount = 55 , ActiveTimestamp = 56 , PendingTimestamp = 57 , Delay = 58 , SecurityPolicy = 59 , ChannelPage0Mask = 60 , OperationalDatasetComponents = 61 , ActiveNetworkFaultsList = 62 , ExtAddress = 63 , Rloc16 = 64 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: Channel => write ! (f , "{}(0x{:02x})" , "Channel" , AttributeId :: Channel as u32 ) ? , AttributeId :: RoutingRole => write ! (f , "{}(0x{:02x})" , "RoutingRole" , AttributeId :: RoutingRole as u32 ) ? , AttributeId :: NetworkName => write ! (f , "{}(0x{:02x})" , "NetworkName" , AttributeId :: NetworkName as u32 ) ? , AttributeId :: PanId => write ! (f , "{}(0x{:02x})" , "PanId" , AttributeId :: PanId as u32 ) ? , AttributeId :: ExtendedPanId => write ! (f , "{}(0x{:02x})" , "ExtendedPanId" , AttributeId :: ExtendedPanId as u32 ) ? , AttributeId :: MeshLocalPrefix => write ! (f , "{}(0x{:02x})" , "MeshLocalPrefix" , AttributeId :: MeshLocalPrefix as u32 ) ? , AttributeId :: OverrunCount => write ! (f , "{}(0x{:02x})" , "OverrunCount" , AttributeId :: OverrunCount as u32 ) ? , AttributeId :: NeighborTable => write ! (f , "{}(0x{:02x})" , "NeighborTable" , AttributeId :: NeighborTable as u32 ) ? , AttributeId :: RouteTable => write ! (f , "{}(0x{:02x})" , "RouteTable" , AttributeId :: RouteTable as u32 ) ? , AttributeId :: PartitionId => write ! (f , "{}(0x{:02x})" , "PartitionId" , AttributeId :: PartitionId as u32 ) ? , AttributeId :: Weighting => write ! (f , "{}(0x{:02x})" , "Weighting" , AttributeId :: Weighting as u32 ) ? , AttributeId :: DataVersion => write ! (f , "{}(0x{:02x})" , "DataVersion" , AttributeId :: DataVersion as u32 ) ? , AttributeId :: StableDataVersion => write ! (f , "{}(0x{:02x})" , "StableDataVersion" , AttributeId :: StableDataVersion as u32 ) ? , AttributeId :: LeaderRouterId => write ! (f , "{}(0x{:02x})" , "LeaderRouterId" , AttributeId :: LeaderRouterId as u32 ) ? , AttributeId :: DetachedRoleCount => write ! (f , "{}(0x{:02x})" , "DetachedRoleCount" , AttributeId :: DetachedRoleCount as u32 ) ? , AttributeId :: ChildRoleCount => write ! (f , "{}(0x{:02x})" , "ChildRoleCount" , AttributeId :: ChildRoleCount as u32 ) ? , AttributeId :: RouterRoleCount => write ! (f , "{}(0x{:02x})" , "RouterRoleCount" , AttributeId :: RouterRoleCount as u32 ) ? , AttributeId :: LeaderRoleCount => write ! (f , "{}(0x{:02x})" , "LeaderRoleCount" , AttributeId :: LeaderRoleCount as u32 ) ? , AttributeId :: AttachAttemptCount => write ! (f , "{}(0x{:02x})" , "AttachAttemptCount" , AttributeId :: AttachAttemptCount as u32 ) ? , AttributeId :: PartitionIdChangeCount => write ! (f , "{}(0x{:02x})" , "PartitionIdChangeCount" , AttributeId :: PartitionIdChangeCount as u32 ) ? , AttributeId :: BetterPartitionAttachAttemptCount => write ! (f , "{}(0x{:02x})" , "BetterPartitionAttachAttemptCount" , AttributeId :: BetterPartitionAttachAttemptCount as u32 ) ? , AttributeId :: ParentChangeCount => write ! (f , "{}(0x{:02x})" , "ParentChangeCount" , AttributeId :: ParentChangeCount as u32 ) ? , AttributeId :: TxTotalCount => write ! (f , "{}(0x{:02x})" , "TxTotalCount" , AttributeId :: TxTotalCount as u32 ) ? , AttributeId :: TxUnicastCount => write ! (f , "{}(0x{:02x})" , "TxUnicastCount" , AttributeId :: TxUnicastCount as u32 ) ? , AttributeId :: TxBroadcastCount => write ! (f , "{}(0x{:02x})" , "TxBroadcastCount" , AttributeId :: TxBroadcastCount as u32 ) ? , AttributeId :: TxAckRequestedCount => write ! (f , "{}(0x{:02x})" , "TxAckRequestedCount" , AttributeId :: TxAckRequestedCount as u32 ) ? , AttributeId :: TxAckedCount => write ! (f , "{}(0x{:02x})" , "TxAckedCount" , AttributeId :: TxAckedCount as u32 ) ? , AttributeId :: TxNoAckRequestedCount => write ! (f , "{}(0x{:02x})" , "TxNoAckRequestedCount" , AttributeId :: TxNoAckRequestedCount as u32 ) ? , AttributeId :: TxDataCount => write ! (f , "{}(0x{:02x})" , "TxDataCount" , AttributeId :: TxDataCount as u32 ) ? , AttributeId :: TxDataPollCount => write ! (f , "{}(0x{:02x})" , "TxDataPollCount" , AttributeId :: TxDataPollCount as u32 ) ? , AttributeId :: TxBeaconCount => write ! (f , "{}(0x{:02x})" , "TxBeaconCount" , AttributeId :: TxBeaconCount as u32 ) ? , AttributeId :: TxBeaconRequestCount => write ! (f , "{}(0x{:02x})" , "TxBeaconRequestCount" , AttributeId :: TxBeaconRequestCount as u32 ) ? , AttributeId :: TxOtherCount => write ! (f , "{}(0x{:02x})" , "TxOtherCount" , AttributeId :: TxOtherCount as u32 ) ? , AttributeId :: TxRetryCount => write ! (f , "{}(0x{:02x})" , "TxRetryCount" , AttributeId :: TxRetryCount as u32 ) ? , AttributeId :: TxDirectMaxRetryExpiryCount => write ! (f , "{}(0x{:02x})" , "TxDirectMaxRetryExpiryCount" , AttributeId :: TxDirectMaxRetryExpiryCount as u32 ) ? , AttributeId :: TxIndirectMaxRetryExpiryCount => write ! (f , "{}(0x{:02x})" , "TxIndirectMaxRetryExpiryCount" , AttributeId :: TxIndirectMaxRetryExpiryCount as u32 ) ? , AttributeId :: TxErrCcaCount => write ! (f , "{}(0x{:02x})" , "TxErrCcaCount" , AttributeId :: TxErrCcaCount as u32 ) ? , AttributeId :: TxErrAbortCount => write ! (f , "{}(0x{:02x})" , "TxErrAbortCount" , AttributeId :: TxErrAbortCount as u32 ) ? , AttributeId :: TxErrBusyChannelCount => write ! (f , "{}(0x{:02x})" , "TxErrBusyChannelCount" , AttributeId :: TxErrBusyChannelCount as u32 ) ? , AttributeId :: RxTotalCount => write ! (f , "{}(0x{:02x})" , "RxTotalCount" , AttributeId :: RxTotalCount as u32 ) ? , AttributeId :: RxUnicastCount => write ! (f , "{}(0x{:02x})" , "RxUnicastCount" , AttributeId :: RxUnicastCount as u32 ) ? , AttributeId :: RxBroadcastCount => write ! (f , "{}(0x{:02x})" , "RxBroadcastCount" , AttributeId :: RxBroadcastCount as u32 ) ? , AttributeId :: RxDataCount => write ! (f , "{}(0x{:02x})" , "RxDataCount" , AttributeId :: RxDataCount as u32 ) ? , AttributeId :: RxDataPollCount => write ! (f , "{}(0x{:02x})" , "RxDataPollCount" , AttributeId :: RxDataPollCount as u32 ) ? , AttributeId :: RxBeaconCount => write ! (f , "{}(0x{:02x})" , "RxBeaconCount" , AttributeId :: RxBeaconCount as u32 ) ? , AttributeId :: RxBeaconRequestCount => write ! (f , "{}(0x{:02x})" , "RxBeaconRequestCount" , AttributeId :: RxBeaconRequestCount as u32 ) ? , AttributeId :: RxOtherCount => write ! (f , "{}(0x{:02x})" , "RxOtherCount" , AttributeId :: RxOtherCount as u32 ) ? , AttributeId :: RxAddressFilteredCount => write ! (f , "{}(0x{:02x})" , "RxAddressFilteredCount" , AttributeId :: RxAddressFilteredCount as u32 ) ? , AttributeId :: RxDestAddrFilteredCount => write ! (f , "{}(0x{:02x})" , "RxDestAddrFilteredCount" , AttributeId :: RxDestAddrFilteredCount as u32 ) ? , AttributeId :: RxDuplicatedCount => write ! (f , "{}(0x{:02x})" , "RxDuplicatedCount" , AttributeId :: RxDuplicatedCount as u32 ) ? , AttributeId :: RxErrNoFrameCount => write ! (f , "{}(0x{:02x})" , "RxErrNoFrameCount" , AttributeId :: RxErrNoFrameCount as u32 ) ? , AttributeId :: RxErrUnknownNeighborCount => write ! (f , "{}(0x{:02x})" , "RxErrUnknownNeighborCount" , AttributeId :: RxErrUnknownNeighborCount as u32 ) ? , AttributeId :: RxErrInvalidSrcAddrCount => write ! (f , "{}(0x{:02x})" , "RxErrInvalidSrcAddrCount" , AttributeId :: RxErrInvalidSrcAddrCount as u32 ) ? , AttributeId :: RxErrSecCount => write ! (f , "{}(0x{:02x})" , "RxErrSecCount" , AttributeId :: RxErrSecCount as u32 ) ? , AttributeId :: RxErrFcsCount => write ! (f , "{}(0x{:02x})" , "RxErrFcsCount" , AttributeId :: RxErrFcsCount as u32 ) ? , AttributeId :: RxErrOtherCount => write ! (f , "{}(0x{:02x})" , "RxErrOtherCount" , AttributeId :: RxErrOtherCount as u32 ) ? , AttributeId :: ActiveTimestamp => write ! (f , "{}(0x{:02x})" , "ActiveTimestamp" , AttributeId :: ActiveTimestamp as u32 ) ? , AttributeId :: PendingTimestamp => write ! (f , "{}(0x{:02x})" , "PendingTimestamp" , AttributeId :: PendingTimestamp as u32 ) ? , AttributeId :: Delay => write ! (f , "{}(0x{:02x})" , "Delay" , AttributeId :: Delay as u32 ) ? , AttributeId :: SecurityPolicy => write ! (f , "{}(0x{:02x})" , "SecurityPolicy" , AttributeId :: SecurityPolicy as u32 ) ? , AttributeId :: ChannelPage0Mask => write ! (f , "{}(0x{:02x})" , "ChannelPage0Mask" , AttributeId :: ChannelPage0Mask as u32 ) ? , AttributeId :: OperationalDatasetComponents => write ! (f , "{}(0x{:02x})" , "OperationalDatasetComponents" , AttributeId :: OperationalDatasetComponents as u32 ) ? , AttributeId :: ActiveNetworkFaultsList => write ! (f , "{}(0x{:02x})" , "ActiveNetworkFaultsList" , AttributeId :: ActiveNetworkFaultsList as u32 ) ? , AttributeId :: ExtAddress => write ! (f , "{}(0x{:02x})" , "ExtAddress" , AttributeId :: ExtAddress as u32 ) ? , AttributeId :: Rloc16 => write ! (f , "{}(0x{:02x})" , "Rloc16" , AttributeId :: Rloc16 as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: Channel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Channel" , AttributeId :: Channel as u32 ) , AttributeId :: RoutingRole => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RoutingRole" , AttributeId :: RoutingRole as u32 ) , AttributeId :: NetworkName => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NetworkName" , AttributeId :: NetworkName as u32 ) , AttributeId :: PanId => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PanId" , AttributeId :: PanId as u32 ) , AttributeId :: ExtendedPanId => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ExtendedPanId" , AttributeId :: ExtendedPanId as u32 ) , AttributeId :: MeshLocalPrefix => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MeshLocalPrefix" , AttributeId :: MeshLocalPrefix as u32 ) , AttributeId :: OverrunCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OverrunCount" , AttributeId :: OverrunCount as u32 ) , AttributeId :: NeighborTable => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NeighborTable" , AttributeId :: NeighborTable as u32 ) , AttributeId :: RouteTable => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RouteTable" , AttributeId :: RouteTable as u32 ) , AttributeId :: PartitionId => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PartitionId" , AttributeId :: PartitionId as u32 ) , AttributeId :: Weighting => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Weighting" , AttributeId :: Weighting as u32 ) , AttributeId :: DataVersion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DataVersion" , AttributeId :: DataVersion as u32 ) , AttributeId :: StableDataVersion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StableDataVersion" , AttributeId :: StableDataVersion as u32 ) , AttributeId :: LeaderRouterId => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LeaderRouterId" , AttributeId :: LeaderRouterId as u32 ) , AttributeId :: DetachedRoleCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DetachedRoleCount" , AttributeId :: DetachedRoleCount as u32 ) , AttributeId :: ChildRoleCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ChildRoleCount" , AttributeId :: ChildRoleCount as u32 ) , AttributeId :: RouterRoleCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RouterRoleCount" , AttributeId :: RouterRoleCount as u32 ) , AttributeId :: LeaderRoleCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LeaderRoleCount" , AttributeId :: LeaderRoleCount as u32 ) , AttributeId :: AttachAttemptCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttachAttemptCount" , AttributeId :: AttachAttemptCount as u32 ) , AttributeId :: PartitionIdChangeCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PartitionIdChangeCount" , AttributeId :: PartitionIdChangeCount as u32 ) , AttributeId :: BetterPartitionAttachAttemptCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BetterPartitionAttachAttemptCount" , AttributeId :: BetterPartitionAttachAttemptCount as u32 ) , AttributeId :: ParentChangeCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ParentChangeCount" , AttributeId :: ParentChangeCount as u32 ) , AttributeId :: TxTotalCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxTotalCount" , AttributeId :: TxTotalCount as u32 ) , AttributeId :: TxUnicastCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxUnicastCount" , AttributeId :: TxUnicastCount as u32 ) , AttributeId :: TxBroadcastCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxBroadcastCount" , AttributeId :: TxBroadcastCount as u32 ) , AttributeId :: TxAckRequestedCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxAckRequestedCount" , AttributeId :: TxAckRequestedCount as u32 ) , AttributeId :: TxAckedCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxAckedCount" , AttributeId :: TxAckedCount as u32 ) , AttributeId :: TxNoAckRequestedCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxNoAckRequestedCount" , AttributeId :: TxNoAckRequestedCount as u32 ) , AttributeId :: TxDataCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxDataCount" , AttributeId :: TxDataCount as u32 ) , AttributeId :: TxDataPollCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxDataPollCount" , AttributeId :: TxDataPollCount as u32 ) , AttributeId :: TxBeaconCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxBeaconCount" , AttributeId :: TxBeaconCount as u32 ) , AttributeId :: TxBeaconRequestCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxBeaconRequestCount" , AttributeId :: TxBeaconRequestCount as u32 ) , AttributeId :: TxOtherCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxOtherCount" , AttributeId :: TxOtherCount as u32 ) , AttributeId :: TxRetryCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxRetryCount" , AttributeId :: TxRetryCount as u32 ) , AttributeId :: TxDirectMaxRetryExpiryCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxDirectMaxRetryExpiryCount" , AttributeId :: TxDirectMaxRetryExpiryCount as u32 ) , AttributeId :: TxIndirectMaxRetryExpiryCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxIndirectMaxRetryExpiryCount" , AttributeId :: TxIndirectMaxRetryExpiryCount as u32 ) , AttributeId :: TxErrCcaCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxErrCcaCount" , AttributeId :: TxErrCcaCount as u32 ) , AttributeId :: TxErrAbortCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxErrAbortCount" , AttributeId :: TxErrAbortCount as u32 ) , AttributeId :: TxErrBusyChannelCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TxErrBusyChannelCount" , AttributeId :: TxErrBusyChannelCount as u32 ) , AttributeId :: RxTotalCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxTotalCount" , AttributeId :: RxTotalCount as u32 ) , AttributeId :: RxUnicastCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxUnicastCount" , AttributeId :: RxUnicastCount as u32 ) , AttributeId :: RxBroadcastCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxBroadcastCount" , AttributeId :: RxBroadcastCount as u32 ) , AttributeId :: RxDataCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxDataCount" , AttributeId :: RxDataCount as u32 ) , AttributeId :: RxDataPollCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxDataPollCount" , AttributeId :: RxDataPollCount as u32 ) , AttributeId :: RxBeaconCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxBeaconCount" , AttributeId :: RxBeaconCount as u32 ) , AttributeId :: RxBeaconRequestCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxBeaconRequestCount" , AttributeId :: RxBeaconRequestCount as u32 ) , AttributeId :: RxOtherCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxOtherCount" , AttributeId :: RxOtherCount as u32 ) , AttributeId :: RxAddressFilteredCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxAddressFilteredCount" , AttributeId :: RxAddressFilteredCount as u32 ) , AttributeId :: RxDestAddrFilteredCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxDestAddrFilteredCount" , AttributeId :: RxDestAddrFilteredCount as u32 ) , AttributeId :: RxDuplicatedCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxDuplicatedCount" , AttributeId :: RxDuplicatedCount as u32 ) , AttributeId :: RxErrNoFrameCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxErrNoFrameCount" , AttributeId :: RxErrNoFrameCount as u32 ) , AttributeId :: RxErrUnknownNeighborCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxErrUnknownNeighborCount" , AttributeId :: RxErrUnknownNeighborCount as u32 ) , AttributeId :: RxErrInvalidSrcAddrCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxErrInvalidSrcAddrCount" , AttributeId :: RxErrInvalidSrcAddrCount as u32 ) , AttributeId :: RxErrSecCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxErrSecCount" , AttributeId :: RxErrSecCount as u32 ) , AttributeId :: RxErrFcsCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxErrFcsCount" , AttributeId :: RxErrFcsCount as u32 ) , AttributeId :: RxErrOtherCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RxErrOtherCount" , AttributeId :: RxErrOtherCount as u32 ) , AttributeId :: ActiveTimestamp => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveTimestamp" , AttributeId :: ActiveTimestamp as u32 ) , AttributeId :: PendingTimestamp => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PendingTimestamp" , AttributeId :: PendingTimestamp as u32 ) , AttributeId :: Delay => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Delay" , AttributeId :: Delay as u32 ) , AttributeId :: SecurityPolicy => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SecurityPolicy" , AttributeId :: SecurityPolicy as u32 ) , AttributeId :: ChannelPage0Mask => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ChannelPage0Mask" , AttributeId :: ChannelPage0Mask as u32 ) , AttributeId :: OperationalDatasetComponents => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OperationalDatasetComponents" , AttributeId :: OperationalDatasetComponents as u32 ) , AttributeId :: ActiveNetworkFaultsList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveNetworkFaultsList" , AttributeId :: ActiveNetworkFaultsList as u32 ) , AttributeId :: ExtAddress => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ExtAddress" , AttributeId :: ExtAddress as u32 ) , AttributeId :: Rloc16 => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Rloc16" , AttributeId :: Rloc16 as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    ResetCounts = 0 
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
            CommandId :: ResetCounts => write ! (f , "{}(0x{:02x})" , "ResetCounts" , CommandId :: ResetCounts as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: ResetCounts => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ResetCounts" , CommandId :: ResetCounts as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] pub enum CommandResponseId {
    
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            
        }
        crate :: reexport :: defmt :: write ! (f , "::Response" ) 
    }
    
}
impl core :: convert :: TryFrom < crate :: dm :: CmdId > for CommandResponseId {
    type Error = crate :: error :: Error ;
    fn try_from (id : crate :: dm :: CmdId ) -> Result < Self , Self :: Error > {
        Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) 
    }
    
}
# [doc = "The event IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum EventId {
    ConnectionStatus = 0 , NetworkFaultChange = 1 
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
            EventId :: ConnectionStatus => write ! (f , "{}(0x{:02x})" , "ConnectionStatus" , EventId :: ConnectionStatus as u32 ) ? , EventId :: NetworkFaultChange => write ! (f , "{}(0x{:02x})" , "NetworkFaultChange" , EventId :: NetworkFaultChange as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: ConnectionStatus => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ConnectionStatus" , EventId :: ConnectionStatus as u32 ) , EventId :: NetworkFaultChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NetworkFaultChange" , EventId :: NetworkFaultChange as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (53 , 3 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: Channel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: RoutingRole as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: NetworkName as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: PanId as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ExtendedPanId as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: MeshLocalPrefix as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: OverrunCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NeighborTable as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: RouteTable as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: PartitionId as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Weighting as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: DataVersion as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: StableDataVersion as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: LeaderRouterId as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: DetachedRoleCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ChildRoleCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RouterRoleCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: LeaderRoleCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: AttachAttemptCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: PartitionIdChangeCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BetterPartitionAttachAttemptCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ParentChangeCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxTotalCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxUnicastCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxBroadcastCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxAckRequestedCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxAckedCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxNoAckRequestedCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxDataCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxDataPollCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxBeaconCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxBeaconRequestCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxOtherCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxRetryCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxDirectMaxRetryExpiryCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxIndirectMaxRetryExpiryCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxErrCcaCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxErrAbortCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TxErrBusyChannelCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxTotalCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxUnicastCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxBroadcastCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxDataCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxDataPollCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxBeaconCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxBeaconRequestCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxOtherCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxAddressFilteredCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxDestAddrFilteredCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxDuplicatedCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxErrNoFrameCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxErrUnknownNeighborCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxErrInvalidSrcAddrCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxErrSecCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxErrFcsCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RxErrOtherCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveTimestamp as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: PendingTimestamp as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Delay as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SecurityPolicy as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ChannelPage0Mask as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: OperationalDatasetComponents as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveNetworkFaultsList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: ExtAddress as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Rloc16 as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: ResetCounts as _ , None , crate :: dm :: Access :: WM , ) , ] , & [crate :: dm :: Event :: new (EventId :: ConnectionStatus as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: NetworkFaultChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn channel (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > ;
    fn routing_role (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < RoutingRoleEnum > , crate :: error :: Error > ;
    fn network_name < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: Utf8StrBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > ;
    fn extended_pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > ;
    fn mesh_local_prefix < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn overrun_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u64 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn neighbor_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NeighborTableStructArrayBuilder < P > , NeighborTableStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn route_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RouteTableStructArrayBuilder < P > , RouteTableStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn partition_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > ;
    fn weighting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > ;
    fn data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > ;
    fn stable_data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > ;
    fn leader_router_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > ;
    fn detached_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn child_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn router_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn leader_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn partition_id_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn better_partition_attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn parent_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_acked_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_no_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_retry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_direct_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_indirect_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_err_cca_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_err_abort_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tx_err_busy_channel_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_address_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_dest_addr_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_duplicated_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_err_no_frame_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_err_unknown_neighbor_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_err_invalid_src_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_err_sec_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_err_fcs_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rx_err_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn pending_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn delay (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn security_policy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , SecurityPolicyBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn channel_page_0_mask < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn operational_dataset_components < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , OperationalDatasetComponentsBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn active_network_faults_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> Result < P , crate :: error :: Error > ;
    fn ext_address (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > ;
    fn rloc_16 (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > ;
    fn handle_reset_counts (& self , ctx : impl crate :: dm :: InvokeContext , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn channel (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: channel (self , ctx ) 
    }
    fn routing_role (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < RoutingRoleEnum > , crate :: error :: Error > {
        T :: routing_role (self , ctx ) 
    }
    fn network_name < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: Utf8StrBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: network_name (self , ctx , builder ) 
    }
    fn pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: pan_id (self , ctx ) 
    }
    fn extended_pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        T :: extended_pan_id (self , ctx ) 
    }
    fn mesh_local_prefix < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: mesh_local_prefix (self , ctx , builder ) 
    }
    fn overrun_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u64 , crate :: error :: Error > {
        T :: overrun_count (self , ctx ) 
    }
    fn neighbor_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NeighborTableStructArrayBuilder < P > , NeighborTableStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: neighbor_table (self , ctx , builder ) 
    }
    fn route_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RouteTableStructArrayBuilder < P > , RouteTableStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: route_table (self , ctx , builder ) 
    }
    fn partition_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: partition_id (self , ctx ) 
    }
    fn weighting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: weighting (self , ctx ) 
    }
    fn data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: data_version (self , ctx ) 
    }
    fn stable_data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: stable_data_version (self , ctx ) 
    }
    fn leader_router_id (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: leader_router_id (self , ctx ) 
    }
    fn detached_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: detached_role_count (self , ctx ) 
    }
    fn child_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: child_role_count (self , ctx ) 
    }
    fn router_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: router_role_count (self , ctx ) 
    }
    fn leader_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: leader_role_count (self , ctx ) 
    }
    fn attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: attach_attempt_count (self , ctx ) 
    }
    fn partition_id_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: partition_id_change_count (self , ctx ) 
    }
    fn better_partition_attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: better_partition_attach_attempt_count (self , ctx ) 
    }
    fn parent_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: parent_change_count (self , ctx ) 
    }
    fn tx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_total_count (self , ctx ) 
    }
    fn tx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_unicast_count (self , ctx ) 
    }
    fn tx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_broadcast_count (self , ctx ) 
    }
    fn tx_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_ack_requested_count (self , ctx ) 
    }
    fn tx_acked_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_acked_count (self , ctx ) 
    }
    fn tx_no_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_no_ack_requested_count (self , ctx ) 
    }
    fn tx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_data_count (self , ctx ) 
    }
    fn tx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_data_poll_count (self , ctx ) 
    }
    fn tx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_beacon_count (self , ctx ) 
    }
    fn tx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_beacon_request_count (self , ctx ) 
    }
    fn tx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_other_count (self , ctx ) 
    }
    fn tx_retry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_retry_count (self , ctx ) 
    }
    fn tx_direct_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_direct_max_retry_expiry_count (self , ctx ) 
    }
    fn tx_indirect_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_indirect_max_retry_expiry_count (self , ctx ) 
    }
    fn tx_err_cca_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_err_cca_count (self , ctx ) 
    }
    fn tx_err_abort_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_err_abort_count (self , ctx ) 
    }
    fn tx_err_busy_channel_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: tx_err_busy_channel_count (self , ctx ) 
    }
    fn rx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_total_count (self , ctx ) 
    }
    fn rx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_unicast_count (self , ctx ) 
    }
    fn rx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_broadcast_count (self , ctx ) 
    }
    fn rx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_data_count (self , ctx ) 
    }
    fn rx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_data_poll_count (self , ctx ) 
    }
    fn rx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_beacon_count (self , ctx ) 
    }
    fn rx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_beacon_request_count (self , ctx ) 
    }
    fn rx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_other_count (self , ctx ) 
    }
    fn rx_address_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_address_filtered_count (self , ctx ) 
    }
    fn rx_dest_addr_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_dest_addr_filtered_count (self , ctx ) 
    }
    fn rx_duplicated_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_duplicated_count (self , ctx ) 
    }
    fn rx_err_no_frame_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_err_no_frame_count (self , ctx ) 
    }
    fn rx_err_unknown_neighbor_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_err_unknown_neighbor_count (self , ctx ) 
    }
    fn rx_err_invalid_src_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_err_invalid_src_addr_count (self , ctx ) 
    }
    fn rx_err_sec_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_err_sec_count (self , ctx ) 
    }
    fn rx_err_fcs_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_err_fcs_count (self , ctx ) 
    }
    fn rx_err_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: rx_err_other_count (self , ctx ) 
    }
    fn active_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        T :: active_timestamp (self , ctx ) 
    }
    fn pending_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        T :: pending_timestamp (self , ctx ) 
    }
    fn delay (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: delay (self , ctx ) 
    }
    fn security_policy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , SecurityPolicyBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: security_policy (self , ctx , builder ) 
    }
    fn channel_page_0_mask < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: channel_page_0_mask (self , ctx , builder ) 
    }
    fn operational_dataset_components < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , OperationalDatasetComponentsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: operational_dataset_components (self , ctx , builder ) 
    }
    fn active_network_faults_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: active_network_faults_list (self , ctx , builder ) 
    }
    fn ext_address (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        T :: ext_address (self , ctx ) 
    }
    fn rloc_16 (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: rloc_16 (self , ctx ) 
    }
    fn handle_reset_counts (& self , ctx : impl crate :: dm :: InvokeContext , ) -> Result < () , crate :: error :: Error > {
        T :: handle_reset_counts (self , ctx ) 
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
                    AttributeId :: Channel => {
                        let attr_read_result = self . 0. channel (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Channel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RoutingRole => {
                        let attr_read_result = self . 0. routing_role (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RoutingRole , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NetworkName => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkName , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. network_name (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkName , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkName , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: PanId => {
                        let attr_read_result = self . 0. pan_id (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PanId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ExtendedPanId => {
                        let attr_read_result = self . 0. extended_pan_id (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ExtendedPanId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MeshLocalPrefix => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MeshLocalPrefix , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. mesh_local_prefix (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MeshLocalPrefix , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MeshLocalPrefix , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: OverrunCount => {
                        let attr_read_result = self . 0. overrun_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OverrunCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NeighborTable => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeighborTable , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. neighbor_table (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeighborTable , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeighborTable , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RouteTable => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouteTable , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. route_table (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouteTable , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouteTable , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: PartitionId => {
                        let attr_read_result = self . 0. partition_id (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PartitionId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Weighting => {
                        let attr_read_result = self . 0. weighting (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Weighting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DataVersion => {
                        let attr_read_result = self . 0. data_version (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DataVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StableDataVersion => {
                        let attr_read_result = self . 0. stable_data_version (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StableDataVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LeaderRouterId => {
                        let attr_read_result = self . 0. leader_router_id (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LeaderRouterId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DetachedRoleCount => {
                        let attr_read_result = self . 0. detached_role_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DetachedRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ChildRoleCount => {
                        let attr_read_result = self . 0. child_role_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChildRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RouterRoleCount => {
                        let attr_read_result = self . 0. router_role_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouterRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LeaderRoleCount => {
                        let attr_read_result = self . 0. leader_role_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LeaderRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AttachAttemptCount => {
                        let attr_read_result = self . 0. attach_attempt_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AttachAttemptCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PartitionIdChangeCount => {
                        let attr_read_result = self . 0. partition_id_change_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PartitionIdChangeCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BetterPartitionAttachAttemptCount => {
                        let attr_read_result = self . 0. better_partition_attach_attempt_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BetterPartitionAttachAttemptCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ParentChangeCount => {
                        let attr_read_result = self . 0. parent_change_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ParentChangeCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxTotalCount => {
                        let attr_read_result = self . 0. tx_total_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxTotalCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxUnicastCount => {
                        let attr_read_result = self . 0. tx_unicast_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxUnicastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxBroadcastCount => {
                        let attr_read_result = self . 0. tx_broadcast_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxBroadcastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxAckRequestedCount => {
                        let attr_read_result = self . 0. tx_ack_requested_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxAckRequestedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxAckedCount => {
                        let attr_read_result = self . 0. tx_acked_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxAckedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxNoAckRequestedCount => {
                        let attr_read_result = self . 0. tx_no_ack_requested_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxNoAckRequestedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxDataCount => {
                        let attr_read_result = self . 0. tx_data_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxDataCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxDataPollCount => {
                        let attr_read_result = self . 0. tx_data_poll_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxDataPollCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxBeaconCount => {
                        let attr_read_result = self . 0. tx_beacon_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxBeaconCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxBeaconRequestCount => {
                        let attr_read_result = self . 0. tx_beacon_request_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxBeaconRequestCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxOtherCount => {
                        let attr_read_result = self . 0. tx_other_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxOtherCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxRetryCount => {
                        let attr_read_result = self . 0. tx_retry_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxRetryCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxDirectMaxRetryExpiryCount => {
                        let attr_read_result = self . 0. tx_direct_max_retry_expiry_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxDirectMaxRetryExpiryCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxIndirectMaxRetryExpiryCount => {
                        let attr_read_result = self . 0. tx_indirect_max_retry_expiry_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxIndirectMaxRetryExpiryCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxErrCcaCount => {
                        let attr_read_result = self . 0. tx_err_cca_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxErrCcaCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxErrAbortCount => {
                        let attr_read_result = self . 0. tx_err_abort_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxErrAbortCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxErrBusyChannelCount => {
                        let attr_read_result = self . 0. tx_err_busy_channel_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxErrBusyChannelCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxTotalCount => {
                        let attr_read_result = self . 0. rx_total_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxTotalCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxUnicastCount => {
                        let attr_read_result = self . 0. rx_unicast_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxUnicastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxBroadcastCount => {
                        let attr_read_result = self . 0. rx_broadcast_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxBroadcastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDataCount => {
                        let attr_read_result = self . 0. rx_data_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDataCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDataPollCount => {
                        let attr_read_result = self . 0. rx_data_poll_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDataPollCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxBeaconCount => {
                        let attr_read_result = self . 0. rx_beacon_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxBeaconCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxBeaconRequestCount => {
                        let attr_read_result = self . 0. rx_beacon_request_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxBeaconRequestCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxOtherCount => {
                        let attr_read_result = self . 0. rx_other_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxOtherCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxAddressFilteredCount => {
                        let attr_read_result = self . 0. rx_address_filtered_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxAddressFilteredCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDestAddrFilteredCount => {
                        let attr_read_result = self . 0. rx_dest_addr_filtered_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDestAddrFilteredCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDuplicatedCount => {
                        let attr_read_result = self . 0. rx_duplicated_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDuplicatedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrNoFrameCount => {
                        let attr_read_result = self . 0. rx_err_no_frame_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrNoFrameCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrUnknownNeighborCount => {
                        let attr_read_result = self . 0. rx_err_unknown_neighbor_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrUnknownNeighborCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrInvalidSrcAddrCount => {
                        let attr_read_result = self . 0. rx_err_invalid_src_addr_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrInvalidSrcAddrCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrSecCount => {
                        let attr_read_result = self . 0. rx_err_sec_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrSecCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrFcsCount => {
                        let attr_read_result = self . 0. rx_err_fcs_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrFcsCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrOtherCount => {
                        let attr_read_result = self . 0. rx_err_other_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrOtherCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveTimestamp => {
                        let attr_read_result = self . 0. active_timestamp (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PendingTimestamp => {
                        let attr_read_result = self . 0. pending_timestamp (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PendingTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Delay => {
                        let attr_read_result = self . 0. delay (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Delay , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SecurityPolicy => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SecurityPolicy , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. security_policy (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SecurityPolicy , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SecurityPolicy , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ChannelPage0Mask => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelPage0Mask , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. channel_page_0_mask (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelPage0Mask , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelPage0Mask , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: OperationalDatasetComponents => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OperationalDatasetComponents , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. operational_dataset_components (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OperationalDatasetComponents , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OperationalDatasetComponents , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveNetworkFaultsList => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaultsList , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_network_faults_list (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaultsList , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaultsList , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ExtAddress => {
                        let attr_read_result = self . 0. ext_address (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ExtAddress , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Rloc16 => {
                        let attr_read_result = self . 0. rloc_16 (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Rloc16 , false ) ) ) ) , & attr_read_result ) ;
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
            _ => {
                crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                return Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    # [allow (unreachable_code ) ] fn invoke (& self , ctx : impl crate :: dm :: InvokeContext , reply : impl crate :: dm :: InvokeReply , ) -> Result < () , crate :: error :: Error > {
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: ResetCounts => {
                let cmd_invoke_result = self . 0. handle_reset_counts (& ctx ) ;
                crate :: dm :: log_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ResetCounts ) ) ) , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            _ => {
                crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
                return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 53u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ThreadNetworkDiagnostics" , 53u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ThreadNetworkDiagnostics" , 53u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `ThreadNetworkDiagnostics` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct ThreadNetworkDiagnosticsCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > ThreadNetworkDiagnosticsCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn reset_counts (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        let array = self . array . push () ? . path (endpoint , 53 , 0 ) ? . data (| w | {
            w . start_struct (& crate :: tlv :: TLVTag :: Context (crate :: im :: CmdDataTag :: Data as u8 , ) ) ? ;
            w . end_container () 
        }
        ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ThreadNetworkDiagnostics` cluster's commands. `use` this trait to call `.thread_network_diagnostics_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`ThreadNetworkDiagnosticsCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ThreadNetworkDiagnosticsCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn thread_network_diagnostics_inv (self ) -> ThreadNetworkDiagnosticsCmdRequestsView < P > ;
    
}
impl < P > ThreadNetworkDiagnosticsCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn thread_network_diagnostics_inv (self ) -> ThreadNetworkDiagnosticsCmdRequestsView < P > {
        ThreadNetworkDiagnosticsCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `ThreadNetworkDiagnostics` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThreadNetworkDiagnosticsAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > ThreadNetworkDiagnosticsAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn routing_role (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn network_name (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn pan_id (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn extended_pan_id (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn mesh_local_prefix (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn overrun_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn neighbor_table (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn route_table (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn partition_id (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn weighting (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn data_version (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (11 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn stable_data_version (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (12 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn leader_router_id (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (13 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn detached_role_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (14 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn child_role_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (15 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn router_role_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (16 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn leader_role_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (17 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attach_attempt_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (18 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn partition_id_change_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (19 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn better_partition_attach_attempt_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (20 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn parent_change_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (21 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_total_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (22 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_unicast_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (23 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_broadcast_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (24 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_ack_requested_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (25 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_acked_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (26 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_no_ack_requested_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (27 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_data_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (28 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_data_poll_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (29 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_beacon_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (30 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_beacon_request_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (31 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_other_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (32 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_retry_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (33 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_direct_max_retry_expiry_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (34 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_indirect_max_retry_expiry_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (35 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_err_cca_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (36 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_err_abort_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (37 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tx_err_busy_channel_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (38 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_total_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (39 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_unicast_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (40 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_broadcast_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (41 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_data_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (42 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_data_poll_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (43 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_beacon_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (44 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_beacon_request_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (45 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_other_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (46 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_address_filtered_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (47 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_dest_addr_filtered_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (48 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_duplicated_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (49 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_err_no_frame_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (50 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_err_unknown_neighbor_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (51 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_err_invalid_src_addr_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (52 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_err_sec_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (53 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_err_fcs_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (54 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rx_err_other_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (55 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_timestamp (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (56 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn pending_timestamp (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (57 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn delay (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (58 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn security_policy (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (59 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn channel_page_0_mask (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (60 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn operational_dataset_components (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (61 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_network_faults_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (62 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ext_address (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (63 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rloc_16 (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (64 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (53 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ThreadNetworkDiagnostics` cluster's attribute reads. `use` this trait to call `.thread_network_diagnostics_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`ThreadNetworkDiagnosticsAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ThreadNetworkDiagnosticsAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn thread_network_diagnostics_read (self ) -> ThreadNetworkDiagnosticsAttrReadsView < P > ;
    
}
impl < P > ThreadNetworkDiagnosticsAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn thread_network_diagnostics_read (self ) -> ThreadNetworkDiagnosticsAttrReadsView < P > {
        ThreadNetworkDiagnosticsAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `ThreadNetworkDiagnostics` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThreadNetworkDiagnosticsAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > ThreadNetworkDiagnosticsAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ThreadNetworkDiagnostics` cluster's attribute writes. `use` this trait to call `.thread_network_diagnostics_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`ThreadNetworkDiagnosticsAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ThreadNetworkDiagnosticsAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn thread_network_diagnostics_write (self ) -> ThreadNetworkDiagnosticsAttrWritesView < P > ;
    
}
impl < P > ThreadNetworkDiagnosticsAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn thread_network_diagnostics_write (self ) -> ThreadNetworkDiagnosticsAttrWritesView < P > {
        ThreadNetworkDiagnosticsAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `ThreadNetworkDiagnostics` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct ThreadNetworkDiagnosticsCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > ThreadNetworkDiagnosticsCmdResponsesView < 'a , 'r > {
    pub fn reset_counts (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (53 , 0 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `ThreadNetworkDiagnostics`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.thread_network_diagnostics_inv_resp()` on an `InvokeResp`; the returned [`ThreadNetworkDiagnosticsCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait ThreadNetworkDiagnosticsCmdResponses < 'a > {
    fn thread_network_diagnostics_inv_resp (& self ) -> ThreadNetworkDiagnosticsCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > ThreadNetworkDiagnosticsCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn thread_network_diagnostics_inv_resp (& self ) -> ThreadNetworkDiagnosticsCmdResponsesView < 'a , '_ > {
        ThreadNetworkDiagnosticsCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `ThreadNetworkDiagnostics` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThreadNetworkDiagnosticsAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > ThreadNetworkDiagnosticsAttrResponsesView < 'a , 'r > {
    pub fn channel (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (53 , 0 ) 
    }
    pub fn routing_role (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < RoutingRoleEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < RoutingRoleEnum > > (53 , 1 ) 
    }
    pub fn network_name (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: Utf8Str < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: Utf8Str < 'a > > > (53 , 2 ) 
    }
    pub fn pan_id (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (53 , 3 ) 
    }
    pub fn extended_pan_id (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u64 > > (53 , 4 ) 
    }
    pub fn mesh_local_prefix (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > > (53 , 5 ) 
    }
    pub fn overrun_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u64 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u64 > (53 , 6 ) 
    }
    pub fn neighbor_table (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , NeighborTableStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , NeighborTableStruct < 'a > > > (53 , 7 ) 
    }
    pub fn route_table (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , RouteTableStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , RouteTableStruct < 'a > > > (53 , 8 ) 
    }
    pub fn partition_id (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (53 , 9 ) 
    }
    pub fn weighting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (53 , 10 ) 
    }
    pub fn data_version (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (53 , 11 ) 
    }
    pub fn stable_data_version (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (53 , 12 ) 
    }
    pub fn leader_router_id (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (53 , 13 ) 
    }
    pub fn detached_role_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 14 ) 
    }
    pub fn child_role_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 15 ) 
    }
    pub fn router_role_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 16 ) 
    }
    pub fn leader_role_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 17 ) 
    }
    pub fn attach_attempt_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 18 ) 
    }
    pub fn partition_id_change_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 19 ) 
    }
    pub fn better_partition_attach_attempt_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 20 ) 
    }
    pub fn parent_change_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 21 ) 
    }
    pub fn tx_total_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 22 ) 
    }
    pub fn tx_unicast_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 23 ) 
    }
    pub fn tx_broadcast_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 24 ) 
    }
    pub fn tx_ack_requested_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 25 ) 
    }
    pub fn tx_acked_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 26 ) 
    }
    pub fn tx_no_ack_requested_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 27 ) 
    }
    pub fn tx_data_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 28 ) 
    }
    pub fn tx_data_poll_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 29 ) 
    }
    pub fn tx_beacon_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 30 ) 
    }
    pub fn tx_beacon_request_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 31 ) 
    }
    pub fn tx_other_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 32 ) 
    }
    pub fn tx_retry_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 33 ) 
    }
    pub fn tx_direct_max_retry_expiry_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 34 ) 
    }
    pub fn tx_indirect_max_retry_expiry_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 35 ) 
    }
    pub fn tx_err_cca_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 36 ) 
    }
    pub fn tx_err_abort_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 37 ) 
    }
    pub fn tx_err_busy_channel_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 38 ) 
    }
    pub fn rx_total_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 39 ) 
    }
    pub fn rx_unicast_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 40 ) 
    }
    pub fn rx_broadcast_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 41 ) 
    }
    pub fn rx_data_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 42 ) 
    }
    pub fn rx_data_poll_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 43 ) 
    }
    pub fn rx_beacon_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 44 ) 
    }
    pub fn rx_beacon_request_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 45 ) 
    }
    pub fn rx_other_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 46 ) 
    }
    pub fn rx_address_filtered_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 47 ) 
    }
    pub fn rx_dest_addr_filtered_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 48 ) 
    }
    pub fn rx_duplicated_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 49 ) 
    }
    pub fn rx_err_no_frame_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 50 ) 
    }
    pub fn rx_err_unknown_neighbor_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 51 ) 
    }
    pub fn rx_err_invalid_src_addr_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 52 ) 
    }
    pub fn rx_err_sec_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 53 ) 
    }
    pub fn rx_err_fcs_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 54 ) 
    }
    pub fn rx_err_other_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 55 ) 
    }
    pub fn active_timestamp (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u64 > > (53 , 56 ) 
    }
    pub fn pending_timestamp (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u64 > > (53 , 57 ) 
    }
    pub fn delay (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (53 , 58 ) 
    }
    pub fn security_policy (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < SecurityPolicy < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < SecurityPolicy < 'a > > > (53 , 59 ) 
    }
    pub fn channel_page_0_mask (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < 'a > > > (53 , 60 ) 
    }
    pub fn operational_dataset_components (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < OperationalDatasetComponents < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < OperationalDatasetComponents < 'a > > > (53 , 61 ) 
    }
    pub fn active_network_faults_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , NetworkFaultEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , NetworkFaultEnum > > (53 , 62 ) 
    }
    pub fn ext_address (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u64 > > (53 , 63 ) 
    }
    pub fn rloc_16 (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (53 , 64 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (53 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (53 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (53 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (53 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (53 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `ThreadNetworkDiagnostics`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.thread_network_diagnostics_read_resp()` on a `ReportDataResp`; the returned [`ThreadNetworkDiagnosticsAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait ThreadNetworkDiagnosticsAttrResponses < 'a > {
    fn thread_network_diagnostics_read_resp (& self ) -> ThreadNetworkDiagnosticsAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > ThreadNetworkDiagnosticsAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn thread_network_diagnostics_read_resp (& self ) -> ThreadNetworkDiagnosticsAttrResponsesView < 'a , '_ > {
        ThreadNetworkDiagnosticsAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `ThreadNetworkDiagnostics` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ThreadNetworkDiagnosticsWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > ThreadNetworkDiagnosticsWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `ThreadNetworkDiagnostics`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.thread_network_diagnostics_write_resp()` on a `WriteResp`; the returned [`ThreadNetworkDiagnosticsWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait ThreadNetworkDiagnosticsWriteResponses < 'a > {
    fn thread_network_diagnostics_write_resp (& self ) -> ThreadNetworkDiagnosticsWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > ThreadNetworkDiagnosticsWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn thread_network_diagnostics_write_resp (& self ) -> ThreadNetworkDiagnosticsWriteResponsesView < 'a , '_ > {
        ThreadNetworkDiagnosticsWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `ThreadNetworkDiagnostics` cluster's client operations. Returned by [`ThreadNetworkDiagnosticsClient::thread_network_diagnostics`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct ThreadNetworkDiagnosticsClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > ThreadNetworkDiagnosticsClientView < T > {
    pub fn reset_counts (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            msg . invoke_requests () ? . thread_network_diagnostics_inv () . reset_counts (endpoint ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn channel_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . channel (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn routing_role_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < RoutingRoleEnum > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . routing_role (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn network_name_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . network_name (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn pan_id_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . pan_id (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn extended_pan_id_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . extended_pan_id (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn mesh_local_prefix_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . mesh_local_prefix (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn overrun_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . overrun_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn neighbor_table_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , NeighborTableStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . neighbor_table (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn route_table_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , RouteTableStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . route_table (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn partition_id_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . partition_id (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn weighting_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . weighting (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn data_version_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . data_version (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn stable_data_version_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . stable_data_version (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn leader_router_id_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . leader_router_id (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn detached_role_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . detached_role_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn child_role_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . child_role_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn router_role_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . router_role_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn leader_role_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . leader_role_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn attach_attempt_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . attach_attempt_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn partition_id_change_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . partition_id_change_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn better_partition_attach_attempt_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . better_partition_attach_attempt_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn parent_change_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . parent_change_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_total_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_total_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_unicast_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_unicast_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_broadcast_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_broadcast_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_ack_requested_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_ack_requested_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_acked_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_acked_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_no_ack_requested_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_no_ack_requested_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_data_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_data_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_data_poll_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_data_poll_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_beacon_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_beacon_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_beacon_request_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_beacon_request_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_other_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_other_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_retry_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_retry_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_direct_max_retry_expiry_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_direct_max_retry_expiry_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_indirect_max_retry_expiry_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_indirect_max_retry_expiry_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_err_cca_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_err_cca_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_err_abort_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_err_abort_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tx_err_busy_channel_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . tx_err_busy_channel_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_total_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_total_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_unicast_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_unicast_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_broadcast_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_broadcast_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_data_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_data_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_data_poll_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_data_poll_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_beacon_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_beacon_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_beacon_request_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_beacon_request_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_other_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_other_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_address_filtered_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_address_filtered_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_dest_addr_filtered_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_dest_addr_filtered_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_duplicated_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_duplicated_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_err_no_frame_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_err_no_frame_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_err_unknown_neighbor_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_err_unknown_neighbor_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_err_invalid_src_addr_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_err_invalid_src_addr_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_err_sec_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_err_sec_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_err_fcs_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_err_fcs_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rx_err_other_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rx_err_other_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn active_timestamp_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . active_timestamp (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn pending_timestamp_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . pending_timestamp (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn delay_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . delay (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn security_policy_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < SecurityPolicy < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . security_policy (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn channel_page_0_mask_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . channel_page_0_mask (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn operational_dataset_components_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < OperationalDatasetComponents < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . operational_dataset_components (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn active_network_faults_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , NetworkFaultEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . active_network_faults_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn ext_address_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . ext_address (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rloc_16_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . rloc_16 (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ThreadNetworkDiagnosticsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . thread_network_diagnostics_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `ThreadNetworkDiagnostics` cluster. `use` this trait to call `.thread_network_diagnostics()` on any [`crate::im::ImClient`], such as an exchange; the returned [`ThreadNetworkDiagnosticsClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `ThreadNetworkDiagnosticsClientView` keeps each cluster's API surface narrow: at the call site `exchange.thread_network_diagnostics().` shows only this cluster's methods in IDE completion." ] pub trait ThreadNetworkDiagnosticsClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn thread_network_diagnostics (self ) -> ThreadNetworkDiagnosticsClientView < Self > {
        ThreadNetworkDiagnosticsClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > ThreadNetworkDiagnosticsClient for T {
    
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
    fn channel (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn routing_role (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < RoutingRoleEnum > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn network_name < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: Utf8StrBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn extended_pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn mesh_local_prefix < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn overrun_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn neighbor_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NeighborTableStructArrayBuilder < P > , NeighborTableStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn route_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RouteTableStructArrayBuilder < P > , RouteTableStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn partition_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn weighting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn stable_data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn leader_router_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn detached_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn child_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn router_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn leader_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn partition_id_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn better_partition_attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn parent_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_acked_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_no_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_retry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_direct_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_indirect_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_err_cca_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_err_abort_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tx_err_busy_channel_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_address_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_dest_addr_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_duplicated_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_err_no_frame_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_err_unknown_neighbor_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_err_invalid_src_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_err_sec_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_err_fcs_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rx_err_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn pending_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn delay (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn security_policy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , SecurityPolicyBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn channel_page_0_mask < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn operational_dataset_components < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , OperationalDatasetComponentsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn active_network_faults_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn ext_address (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn rloc_16 (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_reset_counts (& self , ctx : impl crate :: dm :: InvokeContext , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn channel (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: channel (self , ctx ) 
    }
    fn routing_role (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < RoutingRoleEnum > , crate :: error :: Error >> {
        T :: routing_role (self , ctx ) 
    }
    fn network_name < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: Utf8StrBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: network_name (self , ctx , builder ) 
    }
    fn pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: pan_id (self , ctx ) 
    }
    fn extended_pan_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        T :: extended_pan_id (self , ctx ) 
    }
    fn mesh_local_prefix < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: mesh_local_prefix (self , ctx , builder ) 
    }
    fn overrun_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> {
        T :: overrun_count (self , ctx ) 
    }
    fn neighbor_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NeighborTableStructArrayBuilder < P > , NeighborTableStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: neighbor_table (self , ctx , builder ) 
    }
    fn route_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RouteTableStructArrayBuilder < P > , RouteTableStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: route_table (self , ctx , builder ) 
    }
    fn partition_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: partition_id (self , ctx ) 
    }
    fn weighting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: weighting (self , ctx ) 
    }
    fn data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: data_version (self , ctx ) 
    }
    fn stable_data_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: stable_data_version (self , ctx ) 
    }
    fn leader_router_id (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: leader_router_id (self , ctx ) 
    }
    fn detached_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: detached_role_count (self , ctx ) 
    }
    fn child_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: child_role_count (self , ctx ) 
    }
    fn router_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: router_role_count (self , ctx ) 
    }
    fn leader_role_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: leader_role_count (self , ctx ) 
    }
    fn attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: attach_attempt_count (self , ctx ) 
    }
    fn partition_id_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: partition_id_change_count (self , ctx ) 
    }
    fn better_partition_attach_attempt_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: better_partition_attach_attempt_count (self , ctx ) 
    }
    fn parent_change_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: parent_change_count (self , ctx ) 
    }
    fn tx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_total_count (self , ctx ) 
    }
    fn tx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_unicast_count (self , ctx ) 
    }
    fn tx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_broadcast_count (self , ctx ) 
    }
    fn tx_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_ack_requested_count (self , ctx ) 
    }
    fn tx_acked_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_acked_count (self , ctx ) 
    }
    fn tx_no_ack_requested_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_no_ack_requested_count (self , ctx ) 
    }
    fn tx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_data_count (self , ctx ) 
    }
    fn tx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_data_poll_count (self , ctx ) 
    }
    fn tx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_beacon_count (self , ctx ) 
    }
    fn tx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_beacon_request_count (self , ctx ) 
    }
    fn tx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_other_count (self , ctx ) 
    }
    fn tx_retry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_retry_count (self , ctx ) 
    }
    fn tx_direct_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_direct_max_retry_expiry_count (self , ctx ) 
    }
    fn tx_indirect_max_retry_expiry_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_indirect_max_retry_expiry_count (self , ctx ) 
    }
    fn tx_err_cca_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_err_cca_count (self , ctx ) 
    }
    fn tx_err_abort_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_err_abort_count (self , ctx ) 
    }
    fn tx_err_busy_channel_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: tx_err_busy_channel_count (self , ctx ) 
    }
    fn rx_total_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_total_count (self , ctx ) 
    }
    fn rx_unicast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_unicast_count (self , ctx ) 
    }
    fn rx_broadcast_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_broadcast_count (self , ctx ) 
    }
    fn rx_data_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_data_count (self , ctx ) 
    }
    fn rx_data_poll_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_data_poll_count (self , ctx ) 
    }
    fn rx_beacon_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_beacon_count (self , ctx ) 
    }
    fn rx_beacon_request_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_beacon_request_count (self , ctx ) 
    }
    fn rx_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_other_count (self , ctx ) 
    }
    fn rx_address_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_address_filtered_count (self , ctx ) 
    }
    fn rx_dest_addr_filtered_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_dest_addr_filtered_count (self , ctx ) 
    }
    fn rx_duplicated_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_duplicated_count (self , ctx ) 
    }
    fn rx_err_no_frame_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_err_no_frame_count (self , ctx ) 
    }
    fn rx_err_unknown_neighbor_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_err_unknown_neighbor_count (self , ctx ) 
    }
    fn rx_err_invalid_src_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_err_invalid_src_addr_count (self , ctx ) 
    }
    fn rx_err_sec_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_err_sec_count (self , ctx ) 
    }
    fn rx_err_fcs_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_err_fcs_count (self , ctx ) 
    }
    fn rx_err_other_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: rx_err_other_count (self , ctx ) 
    }
    fn active_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        T :: active_timestamp (self , ctx ) 
    }
    fn pending_timestamp (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        T :: pending_timestamp (self , ctx ) 
    }
    fn delay (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: delay (self , ctx ) 
    }
    fn security_policy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , SecurityPolicyBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: security_policy (self , ctx , builder ) 
    }
    fn channel_page_0_mask < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: channel_page_0_mask (self , ctx , builder ) 
    }
    fn operational_dataset_components < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , OperationalDatasetComponentsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: operational_dataset_components (self , ctx , builder ) 
    }
    fn active_network_faults_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , NetworkFaultEnum > , crate :: tlv :: ToTLVBuilder < P , NetworkFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_network_faults_list (self , ctx , builder ) 
    }
    fn ext_address (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error >> {
        T :: ext_address (self , ctx ) 
    }
    fn rloc_16 (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: rloc_16 (self , ctx ) 
    }
    fn handle_reset_counts (& self , ctx : impl crate :: dm :: InvokeContext , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_reset_counts (self , ctx ) 
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
                    AttributeId :: Channel => {
                        let attr_read_result = self . 0. channel (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Channel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RoutingRole => {
                        let attr_read_result = self . 0. routing_role (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RoutingRole , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NetworkName => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkName , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. network_name (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkName , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkName , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: PanId => {
                        let attr_read_result = self . 0. pan_id (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PanId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ExtendedPanId => {
                        let attr_read_result = self . 0. extended_pan_id (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ExtendedPanId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MeshLocalPrefix => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MeshLocalPrefix , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. mesh_local_prefix (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MeshLocalPrefix , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MeshLocalPrefix , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: OverrunCount => {
                        let attr_read_result = self . 0. overrun_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OverrunCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NeighborTable => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeighborTable , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. neighbor_table (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeighborTable , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeighborTable , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RouteTable => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouteTable , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. route_table (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouteTable , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouteTable , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: PartitionId => {
                        let attr_read_result = self . 0. partition_id (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PartitionId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Weighting => {
                        let attr_read_result = self . 0. weighting (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Weighting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DataVersion => {
                        let attr_read_result = self . 0. data_version (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DataVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StableDataVersion => {
                        let attr_read_result = self . 0. stable_data_version (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StableDataVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LeaderRouterId => {
                        let attr_read_result = self . 0. leader_router_id (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LeaderRouterId , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DetachedRoleCount => {
                        let attr_read_result = self . 0. detached_role_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DetachedRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ChildRoleCount => {
                        let attr_read_result = self . 0. child_role_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChildRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RouterRoleCount => {
                        let attr_read_result = self . 0. router_role_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RouterRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LeaderRoleCount => {
                        let attr_read_result = self . 0. leader_role_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LeaderRoleCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AttachAttemptCount => {
                        let attr_read_result = self . 0. attach_attempt_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AttachAttemptCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PartitionIdChangeCount => {
                        let attr_read_result = self . 0. partition_id_change_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PartitionIdChangeCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BetterPartitionAttachAttemptCount => {
                        let attr_read_result = self . 0. better_partition_attach_attempt_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BetterPartitionAttachAttemptCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ParentChangeCount => {
                        let attr_read_result = self . 0. parent_change_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ParentChangeCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxTotalCount => {
                        let attr_read_result = self . 0. tx_total_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxTotalCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxUnicastCount => {
                        let attr_read_result = self . 0. tx_unicast_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxUnicastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxBroadcastCount => {
                        let attr_read_result = self . 0. tx_broadcast_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxBroadcastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxAckRequestedCount => {
                        let attr_read_result = self . 0. tx_ack_requested_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxAckRequestedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxAckedCount => {
                        let attr_read_result = self . 0. tx_acked_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxAckedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxNoAckRequestedCount => {
                        let attr_read_result = self . 0. tx_no_ack_requested_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxNoAckRequestedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxDataCount => {
                        let attr_read_result = self . 0. tx_data_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxDataCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxDataPollCount => {
                        let attr_read_result = self . 0. tx_data_poll_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxDataPollCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxBeaconCount => {
                        let attr_read_result = self . 0. tx_beacon_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxBeaconCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxBeaconRequestCount => {
                        let attr_read_result = self . 0. tx_beacon_request_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxBeaconRequestCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxOtherCount => {
                        let attr_read_result = self . 0. tx_other_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxOtherCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxRetryCount => {
                        let attr_read_result = self . 0. tx_retry_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxRetryCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxDirectMaxRetryExpiryCount => {
                        let attr_read_result = self . 0. tx_direct_max_retry_expiry_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxDirectMaxRetryExpiryCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxIndirectMaxRetryExpiryCount => {
                        let attr_read_result = self . 0. tx_indirect_max_retry_expiry_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxIndirectMaxRetryExpiryCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxErrCcaCount => {
                        let attr_read_result = self . 0. tx_err_cca_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxErrCcaCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxErrAbortCount => {
                        let attr_read_result = self . 0. tx_err_abort_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxErrAbortCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TxErrBusyChannelCount => {
                        let attr_read_result = self . 0. tx_err_busy_channel_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TxErrBusyChannelCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxTotalCount => {
                        let attr_read_result = self . 0. rx_total_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxTotalCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxUnicastCount => {
                        let attr_read_result = self . 0. rx_unicast_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxUnicastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxBroadcastCount => {
                        let attr_read_result = self . 0. rx_broadcast_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxBroadcastCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDataCount => {
                        let attr_read_result = self . 0. rx_data_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDataCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDataPollCount => {
                        let attr_read_result = self . 0. rx_data_poll_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDataPollCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxBeaconCount => {
                        let attr_read_result = self . 0. rx_beacon_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxBeaconCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxBeaconRequestCount => {
                        let attr_read_result = self . 0. rx_beacon_request_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxBeaconRequestCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxOtherCount => {
                        let attr_read_result = self . 0. rx_other_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxOtherCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxAddressFilteredCount => {
                        let attr_read_result = self . 0. rx_address_filtered_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxAddressFilteredCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDestAddrFilteredCount => {
                        let attr_read_result = self . 0. rx_dest_addr_filtered_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDestAddrFilteredCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxDuplicatedCount => {
                        let attr_read_result = self . 0. rx_duplicated_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxDuplicatedCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrNoFrameCount => {
                        let attr_read_result = self . 0. rx_err_no_frame_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrNoFrameCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrUnknownNeighborCount => {
                        let attr_read_result = self . 0. rx_err_unknown_neighbor_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrUnknownNeighborCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrInvalidSrcAddrCount => {
                        let attr_read_result = self . 0. rx_err_invalid_src_addr_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrInvalidSrcAddrCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrSecCount => {
                        let attr_read_result = self . 0. rx_err_sec_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrSecCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrFcsCount => {
                        let attr_read_result = self . 0. rx_err_fcs_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrFcsCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RxErrOtherCount => {
                        let attr_read_result = self . 0. rx_err_other_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RxErrOtherCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveTimestamp => {
                        let attr_read_result = self . 0. active_timestamp (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PendingTimestamp => {
                        let attr_read_result = self . 0. pending_timestamp (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PendingTimestamp , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Delay => {
                        let attr_read_result = self . 0. delay (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Delay , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SecurityPolicy => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SecurityPolicy , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. security_policy (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SecurityPolicy , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SecurityPolicy , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ChannelPage0Mask => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelPage0Mask , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. channel_page_0_mask (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelPage0Mask , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelPage0Mask , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: OperationalDatasetComponents => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OperationalDatasetComponents , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. operational_dataset_components (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OperationalDatasetComponents , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OperationalDatasetComponents , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ActiveNetworkFaultsList => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaultsList , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_network_faults_list (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaultsList , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveNetworkFaultsList , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ExtAddress => {
                        let attr_read_result = self . 0. ext_address (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ExtAddress , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Rloc16 => {
                        let attr_read_result = self . 0. rloc_16 (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Rloc16 , false ) ) ) ) , & attr_read_result ) ;
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
            _ => {
                crate :: dm :: log_attr_not_supported (ctx . attr () ) ;
                return Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    # [allow (unreachable_code ) ] async fn invoke (& self , ctx : impl crate :: dm :: InvokeContext , reply : impl crate :: dm :: InvokeReply , ) -> Result < () , crate :: error :: Error > {
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: ResetCounts => {
                let cmd_invoke_result = self . 0. handle_reset_counts (& ctx ) . await ;
                crate :: dm :: log_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ResetCounts ) ) ) , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            _ => {
                crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
                return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
                
            }
            
        }
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 53u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ThreadNetworkDiagnostics" , 53u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ThreadNetworkDiagnostics" , 53u32 , self . 0.2 ) 
    }
    
}
