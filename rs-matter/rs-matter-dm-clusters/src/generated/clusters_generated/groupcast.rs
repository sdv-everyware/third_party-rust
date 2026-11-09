# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const LISTENER = 1 ;
        const SENDER = 2 ;
        const PER_GROUP = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const LISTENER = 1 ;
        const SENDER = 2 ;
        const PER_GROUP = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupcastTestResultEnum {
    # [enumval (0 ) ] Success = 0 , # [enumval (1 ) ] GeneralError = 1 , # [enumval (2 ) ] MessageReplay = 2 , # [enumval (3 ) ] FailedAuth = 3 , # [enumval (4 ) ] NoAvailableKey = 4 , # [enumval (5 ) ] SendFailure = 5 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupcastTestingEnum {
    # [enumval (0 ) ] DisableTesting = 0 , # [enumval (1 ) ] EnableListenerTesting = 1 , # [enumval (2 ) ] EnableSenderTesting = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MulticastAddrPolicyEnum {
    # [enumval (0 ) ] IanaAddr = 0 , # [enumval (1 ) ] PerGroup = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MembershipStructTag {
    GroupId = 0 , Endpoints = 1 , KeySetId = 2 , HasAuxiliaryAcl = 3 , McastAddrPolicy = 4 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum JoinGroupRequestTag {
    GroupId = 0 , Endpoints = 1 , KeySetId = 2 , Key = 3 , UseAuxiliaryAcl = 4 , ReplaceEndpoints = 5 , McastAddrPolicy = 6 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum LeaveGroupRequestTag {
    GroupId = 0 , Endpoints = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum LeaveGroupResponseTag {
    GroupId = 0 , Endpoints = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum UpdateGroupKeyRequestTag {
    GroupId = 0 , KeySetId = 1 , Key = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ConfigureAuxiliaryACLRequestTag {
    GroupId = 0 , UseAuxiliaryAcl = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupcastTestingRequestTag {
    TestOperation = 0 , DurationSeconds = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MembershipStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MembershipStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn endpoints (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , crate :: im :: EndptId > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn key_set_id (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn has_auxiliary_acl (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn mcast_addr_policy (& self ) -> Result < MulticastAddrPolicyEnum , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MembershipStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MembershipStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MembershipStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MembershipStruct" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . endpoints () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "endpoints" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "endpoints" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) ? , 
        }
        match self . key_set_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "key_set_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "key_set_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "key_set_id" , e . code () ) ? , 
        }
        match self . has_auxiliary_acl () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "has_auxiliary_acl" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "has_auxiliary_acl" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "has_auxiliary_acl" , e . code () ) ? , 
        }
        match self . mcast_addr_policy () {
            Ok (value ) => write ! (f , "{}: {:?}," , "mcast_addr_policy" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mcast_addr_policy" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MembershipStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MembershipStruct" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . endpoints () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "endpoints" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "endpoints" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) , 
        }
        match self . key_set_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "key_set_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "key_set_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "key_set_id" , e . code () ) , 
        }
        match self . has_auxiliary_acl () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "has_auxiliary_acl" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "has_auxiliary_acl" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "has_auxiliary_acl" , e . code () ) , 
        }
        match self . mcast_addr_policy () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "mcast_addr_policy" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mcast_addr_policy" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct JoinGroupRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > JoinGroupRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn endpoints (& self ) -> Result < crate :: tlv :: TLVArray < '_ , crate :: im :: EndptId > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn key_set_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn key (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn use_auxiliary_acl (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn replace_endpoints (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn mcast_addr_policy (& self ) -> Result < Option < MulticastAddrPolicyEnum > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for JoinGroupRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for JoinGroupRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for JoinGroupRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "JoinGroupRequest" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . endpoints () {
            Ok (value ) => write ! (f , "{}: {:?}," , "endpoints" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) ? , 
        }
        match self . key_set_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "key_set_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "key_set_id" , e . code () ) ? , 
        }
        match self . key () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "key" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "key" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "key" , e . code () ) ? , 
        }
        match self . use_auxiliary_acl () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "use_auxiliary_acl" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "use_auxiliary_acl" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "use_auxiliary_acl" , e . code () ) ? , 
        }
        match self . replace_endpoints () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "replace_endpoints" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "replace_endpoints" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "replace_endpoints" , e . code () ) ? , 
        }
        match self . mcast_addr_policy () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "mcast_addr_policy" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "mcast_addr_policy" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "mcast_addr_policy" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for JoinGroupRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "JoinGroupRequest" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . endpoints () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "endpoints" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) , 
        }
        match self . key_set_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "key_set_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "key_set_id" , e . code () ) , 
        }
        match self . key () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "key" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "key" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "key" , e . code () ) , 
        }
        match self . use_auxiliary_acl () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "use_auxiliary_acl" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "use_auxiliary_acl" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "use_auxiliary_acl" , e . code () ) , 
        }
        match self . replace_endpoints () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "replace_endpoints" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "replace_endpoints" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "replace_endpoints" , e . code () ) , 
        }
        match self . mcast_addr_policy () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "mcast_addr_policy" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "mcast_addr_policy" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "mcast_addr_policy" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct LeaveGroupRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > LeaveGroupRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn endpoints (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , crate :: im :: EndptId > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for LeaveGroupRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for LeaveGroupRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for LeaveGroupRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "LeaveGroupRequest" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . endpoints () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "endpoints" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "endpoints" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for LeaveGroupRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "LeaveGroupRequest" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . endpoints () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "endpoints" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "endpoints" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct LeaveGroupResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > LeaveGroupResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn endpoints (& self ) -> Result < crate :: tlv :: TLVArray < '_ , crate :: im :: EndptId > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for LeaveGroupResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for LeaveGroupResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for LeaveGroupResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "LeaveGroupResponse" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . endpoints () {
            Ok (value ) => write ! (f , "{}: {:?}," , "endpoints" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for LeaveGroupResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "LeaveGroupResponse" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . endpoints () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "endpoints" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct UpdateGroupKeyRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > UpdateGroupKeyRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn key_set_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn key (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for UpdateGroupKeyRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for UpdateGroupKeyRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for UpdateGroupKeyRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "UpdateGroupKeyRequest" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . key_set_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "key_set_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "key_set_id" , e . code () ) ? , 
        }
        match self . key () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "key" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "key" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "key" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for UpdateGroupKeyRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "UpdateGroupKeyRequest" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . key_set_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "key_set_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "key_set_id" , e . code () ) , 
        }
        match self . key () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "key" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "key" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "key" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ConfigureAuxiliaryACLRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ConfigureAuxiliaryACLRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn use_auxiliary_acl (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ConfigureAuxiliaryACLRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ConfigureAuxiliaryACLRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ConfigureAuxiliaryACLRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ConfigureAuxiliaryACLRequest" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . use_auxiliary_acl () {
            Ok (value ) => write ! (f , "{}: {:?}," , "use_auxiliary_acl" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "use_auxiliary_acl" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ConfigureAuxiliaryACLRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ConfigureAuxiliaryACLRequest" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . use_auxiliary_acl () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "use_auxiliary_acl" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "use_auxiliary_acl" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GroupcastTestingRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GroupcastTestingRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn test_operation (& self ) -> Result < GroupcastTestingEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn duration_seconds (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GroupcastTestingRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GroupcastTestingRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GroupcastTestingRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GroupcastTestingRequest" ) ? ;
        match self . test_operation () {
            Ok (value ) => write ! (f , "{}: {:?}," , "test_operation" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "test_operation" , e . code () ) ? , 
        }
        match self . duration_seconds () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "duration_seconds" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "duration_seconds" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "duration_seconds" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GroupcastTestingRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GroupcastTestingRequest" ) ;
        match self . test_operation () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "test_operation" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "test_operation" , e . code () ) , 
        }
        match self . duration_seconds () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "duration_seconds" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "duration_seconds" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "duration_seconds" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct MembershipStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MembershipStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MembershipStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < MembershipStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MembershipStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MembershipStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoints (self ) -> Result < crate :: tlv :: OptionalBuilder < MembershipStructBuilder < P , 2usize > , crate :: tlv :: ToTLVArrayBuilder < MembershipStructBuilder < P , 2usize > , crate :: im :: EndptId > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MembershipStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > MembershipStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key_set_id (mut self , value : Option < u16 > ) -> Result < MembershipStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "keySetID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MembershipStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MembershipStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn has_auxiliary_acl (mut self , value : Option < bool > ) -> Result < MembershipStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "hasAuxiliaryACL" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MembershipStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MembershipStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mcast_addr_policy (mut self , value : MulticastAddrPolicyEnum ) -> Result < MembershipStructBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "mcastAddrPolicy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (MembershipStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MembershipStructBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < MembershipStructBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (MembershipStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MembershipStructBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MembershipStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MembershipStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MembershipStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MembershipStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MembershipStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MembershipStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MembershipStructArrayBuilder < P > (P ) ;
impl < P > MembershipStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MembershipStructBuilder < MembershipStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MembershipStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MembershipStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MembershipStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MembershipStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MembershipStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MembershipStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MembershipStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct JoinGroupRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > JoinGroupRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < JoinGroupRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (JoinGroupRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoints (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < JoinGroupRequestBuilder < P , 2usize > , crate :: im :: EndptId > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (JoinGroupRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key_set_id (mut self , value : u16 ) -> Result < JoinGroupRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "keySetID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (JoinGroupRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < JoinGroupRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "key" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (JoinGroupRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn use_auxiliary_acl (mut self , value : Option < bool > ) -> Result < JoinGroupRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "useAuxiliaryACL" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (JoinGroupRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn replace_endpoints (mut self , value : Option < bool > ) -> Result < JoinGroupRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "replaceEndpoints" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (JoinGroupRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn mcast_addr_policy (mut self , value : Option < MulticastAddrPolicyEnum > ) -> Result < JoinGroupRequestBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "mcastAddrPolicy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (JoinGroupRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > JoinGroupRequestBuilder < P , 7usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for JoinGroupRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "JoinGroupRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for JoinGroupRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "JoinGroupRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for JoinGroupRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for JoinGroupRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct JoinGroupRequestArrayBuilder < P > (P ) ;
impl < P > JoinGroupRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < JoinGroupRequestBuilder < JoinGroupRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (JoinGroupRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for JoinGroupRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "JoinGroupRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for JoinGroupRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "JoinGroupRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for JoinGroupRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for JoinGroupRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LeaveGroupRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > LeaveGroupRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > LeaveGroupRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < LeaveGroupRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (LeaveGroupRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > LeaveGroupRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoints (self ) -> Result < crate :: tlv :: OptionalBuilder < LeaveGroupRequestBuilder < P , 2usize > , crate :: tlv :: ToTLVArrayBuilder < LeaveGroupRequestBuilder < P , 2usize > , crate :: im :: EndptId > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LeaveGroupRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > LeaveGroupRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for LeaveGroupRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for LeaveGroupRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for LeaveGroupRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LeaveGroupRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LeaveGroupRequestArrayBuilder < P > (P ) ;
impl < P > LeaveGroupRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < LeaveGroupRequestBuilder < LeaveGroupRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LeaveGroupRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for LeaveGroupRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for LeaveGroupRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for LeaveGroupRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LeaveGroupRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LeaveGroupResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > LeaveGroupResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > LeaveGroupResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < LeaveGroupResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (LeaveGroupResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > LeaveGroupResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoints (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < LeaveGroupResponseBuilder < P , 2usize > , crate :: im :: EndptId > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LeaveGroupResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > LeaveGroupResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for LeaveGroupResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for LeaveGroupResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for LeaveGroupResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LeaveGroupResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LeaveGroupResponseArrayBuilder < P > (P ) ;
impl < P > LeaveGroupResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < LeaveGroupResponseBuilder < LeaveGroupResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LeaveGroupResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for LeaveGroupResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for LeaveGroupResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LeaveGroupResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for LeaveGroupResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LeaveGroupResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct UpdateGroupKeyRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > UpdateGroupKeyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > UpdateGroupKeyRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < UpdateGroupKeyRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (UpdateGroupKeyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > UpdateGroupKeyRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key_set_id (mut self , value : u16 ) -> Result < UpdateGroupKeyRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "keySetID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (UpdateGroupKeyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > UpdateGroupKeyRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < UpdateGroupKeyRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "key" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (UpdateGroupKeyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > UpdateGroupKeyRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for UpdateGroupKeyRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "UpdateGroupKeyRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for UpdateGroupKeyRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "UpdateGroupKeyRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for UpdateGroupKeyRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for UpdateGroupKeyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct UpdateGroupKeyRequestArrayBuilder < P > (P ) ;
impl < P > UpdateGroupKeyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < UpdateGroupKeyRequestBuilder < UpdateGroupKeyRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (UpdateGroupKeyRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for UpdateGroupKeyRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "UpdateGroupKeyRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for UpdateGroupKeyRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "UpdateGroupKeyRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for UpdateGroupKeyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for UpdateGroupKeyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ConfigureAuxiliaryACLRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ConfigureAuxiliaryACLRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ConfigureAuxiliaryACLRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < ConfigureAuxiliaryACLRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ConfigureAuxiliaryACLRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConfigureAuxiliaryACLRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn use_auxiliary_acl (mut self , value : bool ) -> Result < ConfigureAuxiliaryACLRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "useAuxiliaryACL" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ConfigureAuxiliaryACLRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ConfigureAuxiliaryACLRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ConfigureAuxiliaryACLRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConfigureAuxiliaryACLRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ConfigureAuxiliaryACLRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConfigureAuxiliaryACLRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ConfigureAuxiliaryACLRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConfigureAuxiliaryACLRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ConfigureAuxiliaryACLRequestArrayBuilder < P > (P ) ;
impl < P > ConfigureAuxiliaryACLRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ConfigureAuxiliaryACLRequestBuilder < ConfigureAuxiliaryACLRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ConfigureAuxiliaryACLRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ConfigureAuxiliaryACLRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ConfigureAuxiliaryACLRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ConfigureAuxiliaryACLRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ConfigureAuxiliaryACLRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ConfigureAuxiliaryACLRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ConfigureAuxiliaryACLRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupcastTestingRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > GroupcastTestingRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GroupcastTestingRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn test_operation (mut self , value : GroupcastTestingEnum ) -> Result < GroupcastTestingRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "testOperation" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (GroupcastTestingRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn duration_seconds (mut self , value : Option < u16 > ) -> Result < GroupcastTestingRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "durationSeconds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GroupcastTestingRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GroupcastTestingRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupcastTestingRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GroupcastTestingRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupcastTestingRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GroupcastTestingRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupcastTestingRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupcastTestingRequestArrayBuilder < P > (P ) ;
impl < P > GroupcastTestingRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GroupcastTestingRequestBuilder < GroupcastTestingRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GroupcastTestingRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GroupcastTestingRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupcastTestingRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GroupcastTestingRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupcastTestingRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GroupcastTestingRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupcastTestingRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupcastTestingTag {
    SourceIpAddress = 0 , DestinationIpAddress = 1 , GroupId = 2 , EndpointId = 3 , ClusterId = 4 , ElementId = 5 , AccessAllowed = 6 , GroupcastTestResult = 7 , FabricIndex = 254 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GroupcastTesting < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GroupcastTesting < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn source_ip_address (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn destination_ip_address (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn group_id (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn endpoint_id (& self ) -> Result < Option < crate :: im :: EndptId > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn cluster_id (& self ) -> Result < Option < crate :: im :: ClusterId > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn element_id (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn access_allowed (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    pub fn groupcast_test_result (& self ) -> Result < GroupcastTestResultEnum , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GroupcastTesting < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GroupcastTesting < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GroupcastTesting < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GroupcastTesting" ) ? ;
        match self . source_ip_address () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "source_ip_address" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "source_ip_address" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "source_ip_address" , e . code () ) ? , 
        }
        match self . destination_ip_address () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "destination_ip_address" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "destination_ip_address" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "destination_ip_address" , e . code () ) ? , 
        }
        match self . group_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "group_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "group_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . endpoint_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "endpoint_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "endpoint_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoint_id" , e . code () ) ? , 
        }
        match self . cluster_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "cluster_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "cluster_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cluster_id" , e . code () ) ? , 
        }
        match self . element_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "element_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "element_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "element_id" , e . code () ) ? , 
        }
        match self . access_allowed () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "access_allowed" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "access_allowed" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "access_allowed" , e . code () ) ? , 
        }
        match self . groupcast_test_result () {
            Ok (value ) => write ! (f , "{}: {:?}," , "groupcast_test_result" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "groupcast_test_result" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GroupcastTesting < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GroupcastTesting" ) ;
        match self . source_ip_address () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "source_ip_address" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "source_ip_address" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "source_ip_address" , e . code () ) , 
        }
        match self . destination_ip_address () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "destination_ip_address" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "destination_ip_address" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "destination_ip_address" , e . code () ) , 
        }
        match self . group_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "group_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "group_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . endpoint_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "endpoint_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "endpoint_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoint_id" , e . code () ) , 
        }
        match self . cluster_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "cluster_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "cluster_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cluster_id" , e . code () ) , 
        }
        match self . element_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "element_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "element_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "element_id" , e . code () ) , 
        }
        match self . access_allowed () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "access_allowed" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "access_allowed" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "access_allowed" , e . code () ) , 
        }
        match self . groupcast_test_result () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "groupcast_test_result" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "groupcast_test_result" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct GroupcastTestingBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > GroupcastTestingBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn source_ip_address (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < GroupcastTestingBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sourceIpAddress" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn destination_ip_address (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < GroupcastTestingBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "destinationIpAddress" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : Option < u16 > ) -> Result < GroupcastTestingBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoint_id (mut self , value : Option < crate :: im :: EndptId > ) -> Result < GroupcastTestingBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "endpointID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cluster_id (mut self , value : Option < crate :: im :: ClusterId > ) -> Result < GroupcastTestingBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "clusterID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn element_id (mut self , value : Option < u32 > ) -> Result < GroupcastTestingBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "elementID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn access_allowed (mut self , value : Option < bool > ) -> Result < GroupcastTestingBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "accessAllowed" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn groupcast_test_result (mut self , value : GroupcastTestResultEnum ) -> Result < GroupcastTestingBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupcastTestResult" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < GroupcastTestingBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (GroupcastTestingBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastTestingBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GroupcastTestingBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupcastTesting" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GroupcastTestingBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupcastTesting" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GroupcastTestingBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupcastTestingBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupcastTestingArrayBuilder < P > (P ) ;
impl < P > GroupcastTestingArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GroupcastTestingBuilder < GroupcastTestingArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GroupcastTestingArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GroupcastTestingArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupcastTesting[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GroupcastTestingArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupcastTesting[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GroupcastTestingArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupcastTestingArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl GroupcastTesting < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (GroupcastTestingBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 101 , EventId :: GroupcastTesting as _ , Self :: PRIORITY , | tw | {
            f (GroupcastTestingBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (GroupcastTestingBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: GroupcastTesting as _ , Self :: PRIORITY , | tw | {
            f (GroupcastTestingBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    Membership = 0 , MaxMembershipCount = 1 , MaxMcastAddrCount = 2 , UsedMcastAddrCount = 3 , FabricUnderTest = 4 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: Membership => write ! (f , "{}(0x{:02x})" , "Membership" , AttributeId :: Membership as u32 ) ? , AttributeId :: MaxMembershipCount => write ! (f , "{}(0x{:02x})" , "MaxMembershipCount" , AttributeId :: MaxMembershipCount as u32 ) ? , AttributeId :: MaxMcastAddrCount => write ! (f , "{}(0x{:02x})" , "MaxMcastAddrCount" , AttributeId :: MaxMcastAddrCount as u32 ) ? , AttributeId :: UsedMcastAddrCount => write ! (f , "{}(0x{:02x})" , "UsedMcastAddrCount" , AttributeId :: UsedMcastAddrCount as u32 ) ? , AttributeId :: FabricUnderTest => write ! (f , "{}(0x{:02x})" , "FabricUnderTest" , AttributeId :: FabricUnderTest as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: Membership => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Membership" , AttributeId :: Membership as u32 ) , AttributeId :: MaxMembershipCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxMembershipCount" , AttributeId :: MaxMembershipCount as u32 ) , AttributeId :: MaxMcastAddrCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxMcastAddrCount" , AttributeId :: MaxMcastAddrCount as u32 ) , AttributeId :: UsedMcastAddrCount => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UsedMcastAddrCount" , AttributeId :: UsedMcastAddrCount as u32 ) , AttributeId :: FabricUnderTest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FabricUnderTest" , AttributeId :: FabricUnderTest as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    JoinGroup = 0 , LeaveGroup = 1 , UpdateGroupKey = 3 , ConfigureAuxiliaryACL = 4 , GroupcastTesting = 5 
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
            CommandId :: JoinGroup => write ! (f , "{}(0x{:02x})" , "JoinGroup" , CommandId :: JoinGroup as u32 ) ? , CommandId :: LeaveGroup => write ! (f , "{}(0x{:02x})" , "LeaveGroup" , CommandId :: LeaveGroup as u32 ) ? , CommandId :: UpdateGroupKey => write ! (f , "{}(0x{:02x})" , "UpdateGroupKey" , CommandId :: UpdateGroupKey as u32 ) ? , CommandId :: ConfigureAuxiliaryACL => write ! (f , "{}(0x{:02x})" , "ConfigureAuxiliaryACL" , CommandId :: ConfigureAuxiliaryACL as u32 ) ? , CommandId :: GroupcastTesting => write ! (f , "{}(0x{:02x})" , "GroupcastTesting" , CommandId :: GroupcastTesting as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: JoinGroup => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "JoinGroup" , CommandId :: JoinGroup as u32 ) , CommandId :: LeaveGroup => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LeaveGroup" , CommandId :: LeaveGroup as u32 ) , CommandId :: UpdateGroupKey => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UpdateGroupKey" , CommandId :: UpdateGroupKey as u32 ) , CommandId :: ConfigureAuxiliaryACL => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ConfigureAuxiliaryACL" , CommandId :: ConfigureAuxiliaryACL as u32 ) , CommandId :: GroupcastTesting => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GroupcastTesting" , CommandId :: GroupcastTesting as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    LeaveGroupResponse = 2 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: LeaveGroupResponse => write ! (f , "{}(0x{:02x})" , "LeaveGroupResponse" , CommandResponseId :: LeaveGroupResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: LeaveGroupResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LeaveGroupResponse" , CommandResponseId :: LeaveGroupResponse as u32 ) , 
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
    GroupcastTesting = 0 
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
            EventId :: GroupcastTesting => write ! (f , "{}(0x{:02x})" , "GroupcastTesting" , EventId :: GroupcastTesting as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: GroupcastTesting => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GroupcastTesting" , EventId :: GroupcastTesting as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (101 , 1 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: Membership as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: MaxMembershipCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: MaxMcastAddrCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: UsedMcastAddrCount as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: FabricUnderTest as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: JoinGroup as _ , None , crate :: dm :: Access :: WM . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: LeaveGroup as _ , Some (CommandResponseId :: LeaveGroupResponse as _ ) , crate :: dm :: Access :: WM . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: UpdateGroupKey as _ , None , crate :: dm :: Access :: WM . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: ConfigureAuxiliaryACL as _ , None , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: GroupcastTesting as _ , None , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , ] , & [crate :: dm :: Event :: new (EventId :: GroupcastTesting as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: FAB_SENSITIVE ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn membership < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MembershipStructArrayBuilder < P > , MembershipStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn max_membership_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > ;
    fn max_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > ;
    fn used_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > ;
    fn fabric_under_test (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: im :: FabricIndex , crate :: error :: Error > ;
    fn handle_join_group (& self , ctx : impl crate :: dm :: InvokeContext , request : JoinGroupRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_leave_group < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : LeaveGroupRequest < '_ > , response : LeaveGroupResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_update_group_key (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateGroupKeyRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_configure_auxiliary_acl (& self , ctx : impl crate :: dm :: InvokeContext , request : ConfigureAuxiliaryACLRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_groupcast_testing (& self , ctx : impl crate :: dm :: InvokeContext , request : GroupcastTestingRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn membership < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MembershipStructArrayBuilder < P > , MembershipStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: membership (self , ctx , builder ) 
    }
    fn max_membership_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: max_membership_count (self , ctx ) 
    }
    fn max_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: max_mcast_addr_count (self , ctx ) 
    }
    fn used_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: used_mcast_addr_count (self , ctx ) 
    }
    fn fabric_under_test (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: im :: FabricIndex , crate :: error :: Error > {
        T :: fabric_under_test (self , ctx ) 
    }
    fn handle_join_group (& self , ctx : impl crate :: dm :: InvokeContext , request : JoinGroupRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_join_group (self , ctx , request ) 
    }
    fn handle_leave_group < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : LeaveGroupRequest < '_ > , response : LeaveGroupResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_leave_group (self , ctx , request , response ) 
    }
    fn handle_update_group_key (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateGroupKeyRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_update_group_key (self , ctx , request ) 
    }
    fn handle_configure_auxiliary_acl (& self , ctx : impl crate :: dm :: InvokeContext , request : ConfigureAuxiliaryACLRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_configure_auxiliary_acl (self , ctx , request ) 
    }
    fn handle_groupcast_testing (& self , ctx : impl crate :: dm :: InvokeContext , request : GroupcastTestingRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_groupcast_testing (self , ctx , request ) 
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
                    AttributeId :: Membership => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Membership , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. membership (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Membership , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Membership , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxMembershipCount => {
                        let attr_read_result = self . 0. max_membership_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxMembershipCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxMcastAddrCount => {
                        let attr_read_result = self . 0. max_mcast_addr_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxMcastAddrCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UsedMcastAddrCount => {
                        let attr_read_result = self . 0. used_mcast_addr_count (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UsedMcastAddrCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: FabricUnderTest => {
                        let attr_read_result = self . 0. fabric_under_test (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FabricUnderTest , false ) ) ) ) , & attr_read_result ) ;
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
            CommandId :: JoinGroup => {
                let cmd_data : JoinGroupRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_join_group (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: JoinGroup ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: LeaveGroup => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: LeaveGroup ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_leave_group (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: LeaveGroup ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: LeaveGroup ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: UpdateGroupKey => {
                let cmd_data : UpdateGroupKeyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_update_group_key (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateGroupKey ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ConfigureAuxiliaryACL => {
                let cmd_data : ConfigureAuxiliaryACLRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_configure_auxiliary_acl (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConfigureAuxiliaryACL ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: GroupcastTesting => {
                let cmd_data : GroupcastTestingRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_groupcast_testing (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GroupcastTesting ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 101u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Groupcast" , 101u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Groupcast" , 101u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `Groupcast` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct GroupcastCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > GroupcastCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn join_group (self , endpoint : crate :: dm :: EndptId , ) -> Result < JoinGroupRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 101 , 0 ) ? . data_builder () 
    }
    pub fn leave_group (self , endpoint : crate :: dm :: EndptId , ) -> Result < LeaveGroupRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 101 , 1 ) ? . data_builder () 
    }
    pub fn update_group_key (self , endpoint : crate :: dm :: EndptId , ) -> Result < UpdateGroupKeyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 101 , 3 ) ? . data_builder () 
    }
    pub fn configure_auxiliary_acl (self , endpoint : crate :: dm :: EndptId , ) -> Result < ConfigureAuxiliaryACLRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 101 , 4 ) ? . data_builder () 
    }
    pub fn groupcast_testing (self , endpoint : crate :: dm :: EndptId , ) -> Result < GroupcastTestingRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 101 , 5 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Groupcast` cluster's commands. `use` this trait to call `.groupcast_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`GroupcastCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GroupcastCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn groupcast_inv (self ) -> GroupcastCmdRequestsView < P > ;
    
}
impl < P > GroupcastCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn groupcast_inv (self ) -> GroupcastCmdRequestsView < P > {
        GroupcastCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `Groupcast` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupcastAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > GroupcastAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn membership (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_membership_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_mcast_addr_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn used_mcast_addr_count (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn fabric_under_test (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (101 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Groupcast` cluster's attribute reads. `use` this trait to call `.groupcast_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`GroupcastAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GroupcastAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn groupcast_read (self ) -> GroupcastAttrReadsView < P > ;
    
}
impl < P > GroupcastAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn groupcast_read (self ) -> GroupcastAttrReadsView < P > {
        GroupcastAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `Groupcast` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupcastAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > GroupcastAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Groupcast` cluster's attribute writes. `use` this trait to call `.groupcast_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`GroupcastAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GroupcastAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn groupcast_write (self ) -> GroupcastAttrWritesView < P > ;
    
}
impl < P > GroupcastAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn groupcast_write (self ) -> GroupcastAttrWritesView < P > {
        GroupcastAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `Groupcast` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct GroupcastCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > GroupcastCmdResponsesView < 'a , 'r > {
    pub fn join_group (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (101 , 0 ) 
    }
    pub fn leave_group (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < LeaveGroupResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < LeaveGroupResponse < 'a >> (101 , 1 ) 
    }
    pub fn update_group_key (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (101 , 3 ) 
    }
    pub fn configure_auxiliary_acl (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (101 , 4 ) 
    }
    pub fn groupcast_testing (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (101 , 5 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Groupcast`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.groupcast_inv_resp()` on an `InvokeResp`; the returned [`GroupcastCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait GroupcastCmdResponses < 'a > {
    fn groupcast_inv_resp (& self ) -> GroupcastCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > GroupcastCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn groupcast_inv_resp (& self ) -> GroupcastCmdResponsesView < 'a , '_ > {
        GroupcastCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `Groupcast` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupcastAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > GroupcastAttrResponsesView < 'a , 'r > {
    pub fn membership (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , MembershipStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , MembershipStruct < 'a > > > (101 , 0 ) 
    }
    pub fn max_membership_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (101 , 1 ) 
    }
    pub fn max_mcast_addr_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (101 , 2 ) 
    }
    pub fn used_mcast_addr_count (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (101 , 3 ) 
    }
    pub fn fabric_under_test (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: im :: FabricIndex , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: im :: FabricIndex > (101 , 4 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (101 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (101 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (101 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (101 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (101 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Groupcast`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.groupcast_read_resp()` on a `ReportDataResp`; the returned [`GroupcastAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait GroupcastAttrResponses < 'a > {
    fn groupcast_read_resp (& self ) -> GroupcastAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > GroupcastAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn groupcast_read_resp (& self ) -> GroupcastAttrResponsesView < 'a , '_ > {
        GroupcastAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `Groupcast` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupcastWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > GroupcastWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `Groupcast`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.groupcast_write_resp()` on a `WriteResp`; the returned [`GroupcastWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait GroupcastWriteResponses < 'a > {
    fn groupcast_write_resp (& self ) -> GroupcastWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > GroupcastWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn groupcast_write_resp (& self ) -> GroupcastWriteResponsesView < 'a , '_ > {
        GroupcastWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `LeaveGroupResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`LeaveGroupResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct LeaveGroupResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > LeaveGroupResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < LeaveGroupResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `Groupcast` cluster's client operations. Returned by [`GroupcastClient::groupcast`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct GroupcastClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > GroupcastClientView < T > {
    pub fn join_group < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (JoinGroupRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupcastCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . groupcast_inv () ;
            let req_builder = view . join_group (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn leave_group < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < LeaveGroupResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (LeaveGroupRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupcastCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . groupcast_inv () ;
            let req_builder = view . leave_group (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (LeaveGroupResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn update_group_key < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (UpdateGroupKeyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupcastCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . groupcast_inv () ;
            let req_builder = view . update_group_key (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn configure_auxiliary_acl < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (ConfigureAuxiliaryACLRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupcastCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . groupcast_inv () ;
            let req_builder = view . configure_auxiliary_acl (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn groupcast_testing < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (GroupcastTestingRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupcastCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . groupcast_inv () ;
            let req_builder = view . groupcast_testing (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn membership_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , MembershipStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . membership (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn max_membership_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . max_membership_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_mcast_addr_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . max_mcast_addr_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn used_mcast_addr_count_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . used_mcast_addr_count (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn fabric_under_test_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: im :: FabricIndex , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . fabric_under_test (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GroupcastAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . groupcast_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `Groupcast` cluster. `use` this trait to call `.groupcast()` on any [`crate::im::ImClient`], such as an exchange; the returned [`GroupcastClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `GroupcastClientView` keeps each cluster's API surface narrow: at the call site `exchange.groupcast().` shows only this cluster's methods in IDE completion." ] pub trait GroupcastClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn groupcast (self ) -> GroupcastClientView < Self > {
        GroupcastClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > GroupcastClient for T {
    
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
    fn membership < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MembershipStructArrayBuilder < P > , MembershipStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn max_membership_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn max_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn used_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn fabric_under_test (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: im :: FabricIndex , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_join_group (& self , ctx : impl crate :: dm :: InvokeContext , request : JoinGroupRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_leave_group < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : LeaveGroupRequest < '_ > , response : LeaveGroupResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_update_group_key (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateGroupKeyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_configure_auxiliary_acl (& self , ctx : impl crate :: dm :: InvokeContext , request : ConfigureAuxiliaryACLRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_groupcast_testing (& self , ctx : impl crate :: dm :: InvokeContext , request : GroupcastTestingRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn membership < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MembershipStructArrayBuilder < P > , MembershipStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: membership (self , ctx , builder ) 
    }
    fn max_membership_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: max_membership_count (self , ctx ) 
    }
    fn max_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: max_mcast_addr_count (self , ctx ) 
    }
    fn used_mcast_addr_count (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: used_mcast_addr_count (self , ctx ) 
    }
    fn fabric_under_test (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: im :: FabricIndex , crate :: error :: Error >> {
        T :: fabric_under_test (self , ctx ) 
    }
    fn handle_join_group (& self , ctx : impl crate :: dm :: InvokeContext , request : JoinGroupRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_join_group (self , ctx , request ) 
    }
    fn handle_leave_group < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : LeaveGroupRequest < '_ > , response : LeaveGroupResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_leave_group (self , ctx , request , response ) 
    }
    fn handle_update_group_key (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateGroupKeyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_update_group_key (self , ctx , request ) 
    }
    fn handle_configure_auxiliary_acl (& self , ctx : impl crate :: dm :: InvokeContext , request : ConfigureAuxiliaryACLRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_configure_auxiliary_acl (self , ctx , request ) 
    }
    fn handle_groupcast_testing (& self , ctx : impl crate :: dm :: InvokeContext , request : GroupcastTestingRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_groupcast_testing (self , ctx , request ) 
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
                    AttributeId :: Membership => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Membership , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. membership (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Membership , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Membership , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxMembershipCount => {
                        let attr_read_result = self . 0. max_membership_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxMembershipCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxMcastAddrCount => {
                        let attr_read_result = self . 0. max_mcast_addr_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxMcastAddrCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UsedMcastAddrCount => {
                        let attr_read_result = self . 0. used_mcast_addr_count (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UsedMcastAddrCount , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: FabricUnderTest => {
                        let attr_read_result = self . 0. fabric_under_test (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FabricUnderTest , false ) ) ) ) , & attr_read_result ) ;
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
            CommandId :: JoinGroup => {
                let cmd_data : JoinGroupRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_join_group (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: JoinGroup ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: LeaveGroup => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: LeaveGroup ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_leave_group (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: LeaveGroup ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: LeaveGroup ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: UpdateGroupKey => {
                let cmd_data : UpdateGroupKeyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_update_group_key (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateGroupKey ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ConfigureAuxiliaryACL => {
                let cmd_data : ConfigureAuxiliaryACLRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_configure_auxiliary_acl (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ConfigureAuxiliaryACL ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: GroupcastTesting => {
                let cmd_data : GroupcastTestingRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_groupcast_testing (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GroupcastTesting ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 101u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Groupcast" , 101u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Groupcast" , 101u32 , self . 0.2 ) 
    }
    
}
