# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const CACHE_AND_SYNC = 1 ;
        const GROUPCAST = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const CACHE_AND_SYNC = 1 ;
        const GROUPCAST = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupKeyMulticastPolicyEnum {
    # [enumval (0 ) ] PerGroupID = 0 , # [enumval (1 ) ] AllNodes = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupKeySecurityPolicyEnum {
    # [enumval (0 ) ] TrustFirst = 0 , # [enumval (1 ) ] CacheAndSync = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupInfoMapStructTag {
    GroupId = 1 , Endpoints = 2 , GroupName = 3 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupKeyMapStructTag {
    GroupId = 1 , GroupKeySetId = 2 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupKeySetStructTag {
    GroupKeySetId = 0 , GroupKeySecurityPolicy = 1 , EpochKey0 = 2 , EpochStartTime0 = 3 , EpochKey1 = 4 , EpochStartTime1 = 5 , EpochKey2 = 6 , EpochStartTime2 = 7 , GroupKeyMulticastPolicy = 8 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GroupcastAdoptionStructTag {
    GroupcastAdopted = 0 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum KeySetWriteRequestTag {
    GroupKeySet = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum KeySetReadRequestTag {
    GroupKeySetId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum KeySetReadResponseTag {
    GroupKeySet = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum KeySetRemoveRequestTag {
    GroupKeySetId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum KeySetReadAllIndicesResponseTag {
    GroupKeySetIDs = 0 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GroupInfoMapStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GroupInfoMapStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn endpoints (& self ) -> Result < crate :: tlv :: TLVArray < '_ , crate :: im :: EndptId > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn group_name (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GroupInfoMapStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GroupInfoMapStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GroupInfoMapStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GroupInfoMapStruct" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . endpoints () {
            Ok (value ) => write ! (f , "{}: {:?}," , "endpoints" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) ? , 
        }
        match self . group_name () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "group_name" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "group_name" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_name" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GroupInfoMapStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GroupInfoMapStruct" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . endpoints () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "endpoints" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoints" , e . code () ) , 
        }
        match self . group_name () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "group_name" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "group_name" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_name" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GroupKeyMapStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GroupKeyMapStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn group_key_set_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GroupKeyMapStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GroupKeyMapStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GroupKeyMapStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GroupKeyMapStruct" ) ? ;
        match self . group_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) ? , 
        }
        match self . group_key_set_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_set_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GroupKeyMapStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GroupKeyMapStruct" ) ;
        match self . group_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_id" , e . code () ) , 
        }
        match self . group_key_set_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_set_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GroupKeySetStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GroupKeySetStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_key_set_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn group_key_security_policy (& self ) -> Result < GroupKeySecurityPolicyEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn epoch_key_0 (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn epoch_start_time_0 (& self ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn epoch_key_1 (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn epoch_start_time_1 (& self ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn epoch_key_2 (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn epoch_start_time_2 (& self ) -> Result < crate :: tlv :: Nullable < u64 > , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn group_key_multicast_policy (& self ) -> Result < GroupKeyMulticastPolicyEnum , crate :: error :: Error > {
        self . 0 . read (8 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GroupKeySetStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GroupKeySetStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GroupKeySetStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GroupKeySetStruct" ) ? ;
        match self . group_key_set_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_set_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) ? , 
        }
        match self . group_key_security_policy () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_security_policy" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_security_policy" , e . code () ) ? , 
        }
        match self . epoch_key_0 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "epoch_key_0" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "epoch_key_0" , e . code () ) ? , 
        }
        match self . epoch_start_time_0 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "epoch_start_time_0" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "epoch_start_time_0" , e . code () ) ? , 
        }
        match self . epoch_key_1 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "epoch_key_1" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "epoch_key_1" , e . code () ) ? , 
        }
        match self . epoch_start_time_1 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "epoch_start_time_1" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "epoch_start_time_1" , e . code () ) ? , 
        }
        match self . epoch_key_2 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "epoch_key_2" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "epoch_key_2" , e . code () ) ? , 
        }
        match self . epoch_start_time_2 () {
            Ok (value ) => write ! (f , "{}: {:?}," , "epoch_start_time_2" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "epoch_start_time_2" , e . code () ) ? , 
        }
        match self . group_key_multicast_policy () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_multicast_policy" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_multicast_policy" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GroupKeySetStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GroupKeySetStruct" ) ;
        match self . group_key_set_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_set_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) , 
        }
        match self . group_key_security_policy () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_security_policy" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_security_policy" , e . code () ) , 
        }
        match self . epoch_key_0 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "epoch_key_0" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "epoch_key_0" , e . code () ) , 
        }
        match self . epoch_start_time_0 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "epoch_start_time_0" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "epoch_start_time_0" , e . code () ) , 
        }
        match self . epoch_key_1 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "epoch_key_1" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "epoch_key_1" , e . code () ) , 
        }
        match self . epoch_start_time_1 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "epoch_start_time_1" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "epoch_start_time_1" , e . code () ) , 
        }
        match self . epoch_key_2 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "epoch_key_2" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "epoch_key_2" , e . code () ) , 
        }
        match self . epoch_start_time_2 () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "epoch_start_time_2" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "epoch_start_time_2" , e . code () ) , 
        }
        match self . group_key_multicast_policy () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_multicast_policy" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_multicast_policy" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GroupcastAdoptionStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GroupcastAdoptionStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn groupcast_adopted (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GroupcastAdoptionStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GroupcastAdoptionStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GroupcastAdoptionStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GroupcastAdoptionStruct" ) ? ;
        match self . groupcast_adopted () {
            Ok (value ) => write ! (f , "{}: {:?}," , "groupcast_adopted" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "groupcast_adopted" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GroupcastAdoptionStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GroupcastAdoptionStruct" ) ;
        match self . groupcast_adopted () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "groupcast_adopted" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "groupcast_adopted" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct KeySetWriteRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > KeySetWriteRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_key_set (& self ) -> Result < GroupKeySetStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for KeySetWriteRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for KeySetWriteRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for KeySetWriteRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "KeySetWriteRequest" ) ? ;
        match self . group_key_set () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_set" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_set" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for KeySetWriteRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "KeySetWriteRequest" ) ;
        match self . group_key_set () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_set" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_set" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct KeySetReadRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > KeySetReadRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_key_set_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for KeySetReadRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for KeySetReadRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for KeySetReadRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "KeySetReadRequest" ) ? ;
        match self . group_key_set_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_set_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for KeySetReadRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "KeySetReadRequest" ) ;
        match self . group_key_set_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_set_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct KeySetReadResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > KeySetReadResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_key_set (& self ) -> Result < GroupKeySetStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for KeySetReadResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for KeySetReadResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for KeySetReadResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "KeySetReadResponse" ) ? ;
        match self . group_key_set () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_set" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_set" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for KeySetReadResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "KeySetReadResponse" ) ;
        match self . group_key_set () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_set" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_set" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct KeySetRemoveRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > KeySetRemoveRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_key_set_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for KeySetRemoveRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for KeySetRemoveRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for KeySetRemoveRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "KeySetRemoveRequest" ) ? ;
        match self . group_key_set_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_set_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for KeySetRemoveRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "KeySetRemoveRequest" ) ;
        match self . group_key_set_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_set_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_set_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct KeySetReadAllIndicesResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > KeySetReadAllIndicesResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn group_key_set_i_ds (& self ) -> Result < crate :: tlv :: TLVArray < '_ , u16 > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for KeySetReadAllIndicesResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for KeySetReadAllIndicesResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for KeySetReadAllIndicesResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "KeySetReadAllIndicesResponse" ) ? ;
        match self . group_key_set_i_ds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "group_key_set_i_ds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "group_key_set_i_ds" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for KeySetReadAllIndicesResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "KeySetReadAllIndicesResponse" ) ;
        match self . group_key_set_i_ds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "group_key_set_i_ds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "group_key_set_i_ds" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct GroupInfoMapStructBuilder < P , const F : usize = 1usize > (P ) ;
impl < P > GroupInfoMapStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GroupInfoMapStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < GroupInfoMapStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupId" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GroupInfoMapStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupInfoMapStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoints (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < GroupInfoMapStructBuilder < P , 3usize > , crate :: im :: EndptId > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GroupInfoMapStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > GroupInfoMapStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_name (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < GroupInfoMapStructBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupName" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (GroupInfoMapStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupInfoMapStructBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < GroupInfoMapStructBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (GroupInfoMapStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupInfoMapStructBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GroupInfoMapStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupInfoMapStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GroupInfoMapStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupInfoMapStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GroupInfoMapStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupInfoMapStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupInfoMapStructArrayBuilder < P > (P ) ;
impl < P > GroupInfoMapStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GroupInfoMapStructBuilder < GroupInfoMapStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GroupInfoMapStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GroupInfoMapStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupInfoMapStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GroupInfoMapStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupInfoMapStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GroupInfoMapStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupInfoMapStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupKeyMapStructBuilder < P , const F : usize = 1usize > (P ) ;
impl < P > GroupKeyMapStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GroupKeyMapStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_id (mut self , value : u16 ) -> Result < GroupKeyMapStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupId" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GroupKeyMapStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeyMapStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_set_id (mut self , value : u16 ) -> Result < GroupKeyMapStructBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupKeySetID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (GroupKeyMapStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeyMapStructBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < GroupKeyMapStructBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (GroupKeyMapStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeyMapStructBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GroupKeyMapStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupKeyMapStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GroupKeyMapStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupKeyMapStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GroupKeyMapStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupKeyMapStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupKeyMapStructArrayBuilder < P > (P ) ;
impl < P > GroupKeyMapStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GroupKeyMapStructBuilder < GroupKeyMapStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GroupKeyMapStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GroupKeyMapStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupKeyMapStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GroupKeyMapStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupKeyMapStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GroupKeyMapStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupKeyMapStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupKeySetStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > GroupKeySetStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_set_id (mut self , value : u16 ) -> Result < GroupKeySetStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupKeySetID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_security_policy (mut self , value : GroupKeySecurityPolicyEnum ) -> Result < GroupKeySetStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupKeySecurityPolicy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn epoch_key_0 (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < GroupKeySetStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "epochKey0" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn epoch_start_time_0 (mut self , value : crate :: tlv :: Nullable < u64 > ) -> Result < GroupKeySetStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "epochStartTime0" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn epoch_key_1 (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < GroupKeySetStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "epochKey1" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn epoch_start_time_1 (mut self , value : crate :: tlv :: Nullable < u64 > ) -> Result < GroupKeySetStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "epochStartTime1" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn epoch_key_2 (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < GroupKeySetStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "epochKey2" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn epoch_start_time_2 (mut self , value : crate :: tlv :: Nullable < u64 > ) -> Result < GroupKeySetStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "epochStartTime2" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_multicast_policy (mut self , value : GroupKeyMulticastPolicyEnum ) -> Result < GroupKeySetStructBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupKeyMulticastPolicy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (GroupKeySetStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupKeySetStructBuilder < P , 9usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GroupKeySetStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupKeySetStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GroupKeySetStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupKeySetStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GroupKeySetStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupKeySetStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupKeySetStructArrayBuilder < P > (P ) ;
impl < P > GroupKeySetStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GroupKeySetStructBuilder < GroupKeySetStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GroupKeySetStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GroupKeySetStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupKeySetStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GroupKeySetStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupKeySetStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GroupKeySetStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupKeySetStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupcastAdoptionStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > GroupcastAdoptionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GroupcastAdoptionStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn groupcast_adopted (mut self , value : bool ) -> Result < GroupcastAdoptionStructBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupcastAdopted" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (GroupcastAdoptionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastAdoptionStructBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < GroupcastAdoptionStructBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (GroupcastAdoptionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > GroupcastAdoptionStructBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GroupcastAdoptionStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupcastAdoptionStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GroupcastAdoptionStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupcastAdoptionStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GroupcastAdoptionStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupcastAdoptionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GroupcastAdoptionStructArrayBuilder < P > (P ) ;
impl < P > GroupcastAdoptionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GroupcastAdoptionStructBuilder < GroupcastAdoptionStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GroupcastAdoptionStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GroupcastAdoptionStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GroupcastAdoptionStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GroupcastAdoptionStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GroupcastAdoptionStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GroupcastAdoptionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GroupcastAdoptionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetWriteRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > KeySetWriteRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > KeySetWriteRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_set (self ) -> Result < GroupKeySetStructBuilder < KeySetWriteRequestBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetWriteRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > KeySetWriteRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for KeySetWriteRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetWriteRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for KeySetWriteRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetWriteRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for KeySetWriteRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetWriteRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetWriteRequestArrayBuilder < P > (P ) ;
impl < P > KeySetWriteRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < KeySetWriteRequestBuilder < KeySetWriteRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetWriteRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for KeySetWriteRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetWriteRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for KeySetWriteRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetWriteRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for KeySetWriteRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetWriteRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetReadRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > KeySetReadRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > KeySetReadRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_set_id (mut self , value : u16 ) -> Result < KeySetReadRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupKeySetID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (KeySetReadRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > KeySetReadRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for KeySetReadRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetReadRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for KeySetReadRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetReadRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for KeySetReadRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetReadRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetReadRequestArrayBuilder < P > (P ) ;
impl < P > KeySetReadRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < KeySetReadRequestBuilder < KeySetReadRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetReadRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for KeySetReadRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetReadRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for KeySetReadRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetReadRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for KeySetReadRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetReadRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetReadResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > KeySetReadResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > KeySetReadResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_set (self ) -> Result < GroupKeySetStructBuilder < KeySetReadResponseBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetReadResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > KeySetReadResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for KeySetReadResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetReadResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for KeySetReadResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetReadResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for KeySetReadResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetReadResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetReadResponseArrayBuilder < P > (P ) ;
impl < P > KeySetReadResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < KeySetReadResponseBuilder < KeySetReadResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetReadResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for KeySetReadResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetReadResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for KeySetReadResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetReadResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for KeySetReadResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetReadResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetRemoveRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > KeySetRemoveRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > KeySetRemoveRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_set_id (mut self , value : u16 ) -> Result < KeySetRemoveRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "groupKeySetID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (KeySetRemoveRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > KeySetRemoveRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for KeySetRemoveRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetRemoveRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for KeySetRemoveRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetRemoveRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for KeySetRemoveRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetRemoveRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetRemoveRequestArrayBuilder < P > (P ) ;
impl < P > KeySetRemoveRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < KeySetRemoveRequestBuilder < KeySetRemoveRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetRemoveRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for KeySetRemoveRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetRemoveRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for KeySetRemoveRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetRemoveRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for KeySetRemoveRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetRemoveRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetReadAllIndicesResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > KeySetReadAllIndicesResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > KeySetReadAllIndicesResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_set_i_ds (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < KeySetReadAllIndicesResponseBuilder < P , 1usize > , u16 > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetReadAllIndicesResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > KeySetReadAllIndicesResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for KeySetReadAllIndicesResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetReadAllIndicesResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for KeySetReadAllIndicesResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetReadAllIndicesResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for KeySetReadAllIndicesResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetReadAllIndicesResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct KeySetReadAllIndicesResponseArrayBuilder < P > (P ) ;
impl < P > KeySetReadAllIndicesResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < KeySetReadAllIndicesResponseBuilder < KeySetReadAllIndicesResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (KeySetReadAllIndicesResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for KeySetReadAllIndicesResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "KeySetReadAllIndicesResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for KeySetReadAllIndicesResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "KeySetReadAllIndicesResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for KeySetReadAllIndicesResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for KeySetReadAllIndicesResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    GroupKeyMap = 0 , GroupTable = 1 , MaxGroupsPerFabric = 2 , MaxGroupKeysPerFabric = 3 , GroupcastAdoption = 4 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: GroupKeyMap => write ! (f , "{}(0x{:02x})" , "GroupKeyMap" , AttributeId :: GroupKeyMap as u32 ) ? , AttributeId :: GroupTable => write ! (f , "{}(0x{:02x})" , "GroupTable" , AttributeId :: GroupTable as u32 ) ? , AttributeId :: MaxGroupsPerFabric => write ! (f , "{}(0x{:02x})" , "MaxGroupsPerFabric" , AttributeId :: MaxGroupsPerFabric as u32 ) ? , AttributeId :: MaxGroupKeysPerFabric => write ! (f , "{}(0x{:02x})" , "MaxGroupKeysPerFabric" , AttributeId :: MaxGroupKeysPerFabric as u32 ) ? , AttributeId :: GroupcastAdoption => write ! (f , "{}(0x{:02x})" , "GroupcastAdoption" , AttributeId :: GroupcastAdoption as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: GroupKeyMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GroupKeyMap" , AttributeId :: GroupKeyMap as u32 ) , AttributeId :: GroupTable => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GroupTable" , AttributeId :: GroupTable as u32 ) , AttributeId :: MaxGroupsPerFabric => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxGroupsPerFabric" , AttributeId :: MaxGroupsPerFabric as u32 ) , AttributeId :: MaxGroupKeysPerFabric => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxGroupKeysPerFabric" , AttributeId :: MaxGroupKeysPerFabric as u32 ) , AttributeId :: GroupcastAdoption => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GroupcastAdoption" , AttributeId :: GroupcastAdoption as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    KeySetWrite = 0 , KeySetRead = 1 , KeySetRemove = 3 , KeySetReadAllIndices = 4 
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
            CommandId :: KeySetWrite => write ! (f , "{}(0x{:02x})" , "KeySetWrite" , CommandId :: KeySetWrite as u32 ) ? , CommandId :: KeySetRead => write ! (f , "{}(0x{:02x})" , "KeySetRead" , CommandId :: KeySetRead as u32 ) ? , CommandId :: KeySetRemove => write ! (f , "{}(0x{:02x})" , "KeySetRemove" , CommandId :: KeySetRemove as u32 ) ? , CommandId :: KeySetReadAllIndices => write ! (f , "{}(0x{:02x})" , "KeySetReadAllIndices" , CommandId :: KeySetReadAllIndices as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: KeySetWrite => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "KeySetWrite" , CommandId :: KeySetWrite as u32 ) , CommandId :: KeySetRead => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "KeySetRead" , CommandId :: KeySetRead as u32 ) , CommandId :: KeySetRemove => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "KeySetRemove" , CommandId :: KeySetRemove as u32 ) , CommandId :: KeySetReadAllIndices => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "KeySetReadAllIndices" , CommandId :: KeySetReadAllIndices as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    KeySetReadResponse = 2 , KeySetReadAllIndicesResponse = 5 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: KeySetReadResponse => write ! (f , "{}(0x{:02x})" , "KeySetReadResponse" , CommandResponseId :: KeySetReadResponse as u32 ) ? , CommandResponseId :: KeySetReadAllIndicesResponse => write ! (f , "{}(0x{:02x})" , "KeySetReadAllIndicesResponse" , CommandResponseId :: KeySetReadAllIndicesResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: KeySetReadResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "KeySetReadResponse" , CommandResponseId :: KeySetReadResponse as u32 ) , CommandResponseId :: KeySetReadAllIndicesResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "KeySetReadAllIndicesResponse" , CommandResponseId :: KeySetReadAllIndicesResponse as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (63 , 3 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: GroupKeyMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: GroupTable as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: MaxGroupsPerFabric as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: MaxGroupKeysPerFabric as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: GroupcastAdoption as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: KeySetWrite as _ , None , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: KeySetRead as _ , Some (CommandResponseId :: KeySetReadResponse as _ ) , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: KeySetRemove as _ , None , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: KeySetReadAllIndices as _ , Some (CommandResponseId :: KeySetReadAllIndicesResponse as _ ) , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn group_key_map < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupKeyMapStructArrayBuilder < P > , GroupKeyMapStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn group_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupInfoMapStructArrayBuilder < P > , GroupInfoMapStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn max_groups_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > ;
    fn max_group_keys_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > ;
    fn groupcast_adoption < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupcastAdoptionStructArrayBuilder < P > , GroupcastAdoptionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_group_key_map (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupKeyMapStruct < '_ > > , GroupKeyMapStruct < '_ > > ) -> Result < () , crate :: error :: Error > ;
    fn set_groupcast_adoption (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupcastAdoptionStruct < '_ > > , GroupcastAdoptionStruct < '_ > > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_key_set_write (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetWriteRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_key_set_read < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetReadRequest < '_ > , response : KeySetReadResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_key_set_remove (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetRemoveRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_key_set_read_all_indices < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : KeySetReadAllIndicesResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
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
    fn group_key_map < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupKeyMapStructArrayBuilder < P > , GroupKeyMapStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: group_key_map (self , ctx , builder ) 
    }
    fn group_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupInfoMapStructArrayBuilder < P > , GroupInfoMapStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: group_table (self , ctx , builder ) 
    }
    fn max_groups_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: max_groups_per_fabric (self , ctx ) 
    }
    fn max_group_keys_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: max_group_keys_per_fabric (self , ctx ) 
    }
    fn groupcast_adoption < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupcastAdoptionStructArrayBuilder < P > , GroupcastAdoptionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: groupcast_adoption (self , ctx , builder ) 
    }
    fn set_group_key_map (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupKeyMapStruct < '_ > > , GroupKeyMapStruct < '_ > > ) -> Result < () , crate :: error :: Error > {
        T :: set_group_key_map (self , ctx , value ) 
    }
    fn set_groupcast_adoption (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupcastAdoptionStruct < '_ > > , GroupcastAdoptionStruct < '_ > > ) -> Result < () , crate :: error :: Error > {
        T :: set_groupcast_adoption (self , ctx , value ) 
    }
    fn handle_key_set_write (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetWriteRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_key_set_write (self , ctx , request ) 
    }
    fn handle_key_set_read < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetReadRequest < '_ > , response : KeySetReadResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_key_set_read (self , ctx , request , response ) 
    }
    fn handle_key_set_remove (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetRemoveRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_key_set_remove (self , ctx , request ) 
    }
    fn handle_key_set_read_all_indices < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : KeySetReadAllIndicesResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_key_set_read_all_indices (self , ctx , response ) 
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
                    AttributeId :: GroupKeyMap => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. group_key_map (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: GroupTable => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupTable , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. group_table (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupTable , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupTable , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxGroupsPerFabric => {
                        let attr_read_result = self . 0. max_groups_per_fabric (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxGroupsPerFabric , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxGroupKeysPerFabric => {
                        let attr_read_result = self . 0. max_group_keys_per_fabric (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxGroupKeysPerFabric , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: GroupcastAdoption => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. groupcast_adoption (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
            AttributeId :: GroupKeyMap => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_group_key_map (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: GroupcastAdoption => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_groupcast_adoption (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: KeySetWrite => {
                let cmd_data : KeySetWriteRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_key_set_write (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetWrite ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: KeySetRead => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRead ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_key_set_read (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRead ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRead ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: KeySetRemove => {
                let cmd_data : KeySetRemoveRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_key_set_remove (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRemove ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: KeySetReadAllIndices => {
                crate :: dm :: log_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetReadAllIndices ) ) ) ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_key_set_read_all_indices (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetReadAllIndices ) ) ) , tw ) , tag , ) ? , ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetReadAllIndices ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 63u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GroupKeyManagement" , 63u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GroupKeyManagement" , 63u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `GroupKeyManagement` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct GroupKeyManagementCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > GroupKeyManagementCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key_set_write (self , endpoint : crate :: dm :: EndptId , ) -> Result < KeySetWriteRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 63 , 0 ) ? . data_builder () 
    }
    pub fn key_set_read (self , endpoint : crate :: dm :: EndptId , ) -> Result < KeySetReadRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 63 , 1 ) ? . data_builder () 
    }
    pub fn key_set_remove (self , endpoint : crate :: dm :: EndptId , ) -> Result < KeySetRemoveRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 63 , 3 ) ? . data_builder () 
    }
    pub fn key_set_read_all_indices (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        let array = self . array . push () ? . path (endpoint , 63 , 4 ) ? . data (| w | {
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
# [doc = "IM-client extension trait for the `GroupKeyManagement` cluster's commands. `use` this trait to call `.group_key_management_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`GroupKeyManagementCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GroupKeyManagementCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn group_key_management_inv (self ) -> GroupKeyManagementCmdRequestsView < P > ;
    
}
impl < P > GroupKeyManagementCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn group_key_management_inv (self ) -> GroupKeyManagementCmdRequestsView < P > {
        GroupKeyManagementCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `GroupKeyManagement` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupKeyManagementAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > GroupKeyManagementAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn group_table (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_groups_per_fabric (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_group_keys_per_fabric (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn groupcast_adoption (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (63 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GroupKeyManagement` cluster's attribute reads. `use` this trait to call `.group_key_management_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`GroupKeyManagementAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GroupKeyManagementAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn group_key_management_read (self ) -> GroupKeyManagementAttrReadsView < P > ;
    
}
impl < P > GroupKeyManagementAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn group_key_management_read (self ) -> GroupKeyManagementAttrReadsView < P > {
        GroupKeyManagementAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `GroupKeyManagement` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupKeyManagementAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > GroupKeyManagementAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn group_key_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < GroupKeyMapStructArrayBuilder < crate :: im :: AttrDataBuilder < crate :: im :: AttrDataArrayBuilder < P > , 3 > > , crate :: error :: Error > {
        self . array . push () ? . path (endpoint , 63 , 0 ) ? . data_builder () 
    }
    pub fn groupcast_adoption (self , endpoint : crate :: dm :: EndptId , ) -> Result < GroupcastAdoptionStructArrayBuilder < crate :: im :: AttrDataBuilder < crate :: im :: AttrDataArrayBuilder < P > , 3 > > , crate :: error :: Error > {
        self . array . push () ? . path (endpoint , 63 , 4 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GroupKeyManagement` cluster's attribute writes. `use` this trait to call `.group_key_management_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`GroupKeyManagementAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GroupKeyManagementAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn group_key_management_write (self ) -> GroupKeyManagementAttrWritesView < P > ;
    
}
impl < P > GroupKeyManagementAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn group_key_management_write (self ) -> GroupKeyManagementAttrWritesView < P > {
        GroupKeyManagementAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `GroupKeyManagement` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct GroupKeyManagementCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > GroupKeyManagementCmdResponsesView < 'a , 'r > {
    pub fn key_set_write (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (63 , 0 ) 
    }
    pub fn key_set_read (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < KeySetReadResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < KeySetReadResponse < 'a >> (63 , 1 ) 
    }
    pub fn key_set_remove (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (63 , 3 ) 
    }
    pub fn key_set_read_all_indices (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < KeySetReadAllIndicesResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < KeySetReadAllIndicesResponse < 'a >> (63 , 4 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GroupKeyManagement`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.group_key_management_inv_resp()` on an `InvokeResp`; the returned [`GroupKeyManagementCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait GroupKeyManagementCmdResponses < 'a > {
    fn group_key_management_inv_resp (& self ) -> GroupKeyManagementCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > GroupKeyManagementCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn group_key_management_inv_resp (& self ) -> GroupKeyManagementCmdResponsesView < 'a , '_ > {
        GroupKeyManagementCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `GroupKeyManagement` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupKeyManagementAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > GroupKeyManagementAttrResponsesView < 'a , 'r > {
    pub fn group_key_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , GroupKeyMapStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , GroupKeyMapStruct < 'a > > > (63 , 0 ) 
    }
    pub fn group_table (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , GroupInfoMapStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , GroupInfoMapStruct < 'a > > > (63 , 1 ) 
    }
    pub fn max_groups_per_fabric (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (63 , 2 ) 
    }
    pub fn max_group_keys_per_fabric (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (63 , 3 ) 
    }
    pub fn groupcast_adoption (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , GroupcastAdoptionStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , GroupcastAdoptionStruct < 'a > > > (63 , 4 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (63 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (63 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (63 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (63 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (63 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GroupKeyManagement`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.group_key_management_read_resp()` on a `ReportDataResp`; the returned [`GroupKeyManagementAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait GroupKeyManagementAttrResponses < 'a > {
    fn group_key_management_read_resp (& self ) -> GroupKeyManagementAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > GroupKeyManagementAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn group_key_management_read_resp (& self ) -> GroupKeyManagementAttrResponsesView < 'a , '_ > {
        GroupKeyManagementAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `GroupKeyManagement` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GroupKeyManagementWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > GroupKeyManagementWriteResponsesView < 'a , 'r > {
    pub fn group_key_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (63 , 0 ) 
    }
    pub fn groupcast_adoption (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (63 , 4 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GroupKeyManagement`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.group_key_management_write_resp()` on a `WriteResp`; the returned [`GroupKeyManagementWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait GroupKeyManagementWriteResponses < 'a > {
    fn group_key_management_write_resp (& self ) -> GroupKeyManagementWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > GroupKeyManagementWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn group_key_management_write_resp (& self ) -> GroupKeyManagementWriteResponsesView < 'a , '_ > {
        GroupKeyManagementWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `KeySetReadResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`KeySetReadResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct KeySetReadResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > KeySetReadResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < KeySetReadResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `KeySetReadAllIndicesResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`KeySetReadAllIndicesResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct KeySetReadAllIndicesResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > KeySetReadAllIndicesResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < KeySetReadAllIndicesResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `GroupKeyManagement` cluster's client operations. Returned by [`GroupKeyManagementClient::group_key_management`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct GroupKeyManagementClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > GroupKeyManagementClientView < T > {
    pub fn key_set_write < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (KeySetWriteRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupKeyManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . group_key_management_inv () ;
            let req_builder = view . key_set_write (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn key_set_read < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < KeySetReadResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (KeySetReadRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupKeyManagementCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . group_key_management_inv () ;
            let req_builder = view . key_set_read (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (KeySetReadResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn key_set_remove < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (KeySetRemoveRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GroupKeyManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . group_key_management_inv () ;
            let req_builder = view . key_set_remove (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn key_set_read_all_indices (self , endpoint : crate :: dm :: EndptId , ) -> Result < KeySetReadAllIndicesResponseHandle < T :: InvokeChunk > , crate :: error :: Error > {
        use self :: GroupKeyManagementCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            msg . invoke_requests () ? . group_key_management_inv () . key_set_read_all_indices (endpoint ) ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (KeySetReadAllIndicesResponseHandle {
            chunk 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn group_key_map_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , GroupKeyMapStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . group_key_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn group_table_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , GroupInfoMapStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . group_table (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn max_groups_per_fabric_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . max_groups_per_fabric (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_group_keys_per_fabric_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . max_group_keys_per_fabric (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn groupcast_adoption_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , GroupcastAdoptionStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . groupcast_adoption (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GroupKeyManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . group_key_management_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `GroupKeyManagement` cluster. `use` this trait to call `.group_key_management()` on any [`crate::im::ImClient`], such as an exchange; the returned [`GroupKeyManagementClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `GroupKeyManagementClientView` keeps each cluster's API surface narrow: at the call site `exchange.group_key_management().` shows only this cluster's methods in IDE completion." ] pub trait GroupKeyManagementClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn group_key_management (self ) -> GroupKeyManagementClientView < Self > {
        GroupKeyManagementClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > GroupKeyManagementClient for T {
    
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
    fn group_key_map < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupKeyMapStructArrayBuilder < P > , GroupKeyMapStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn group_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupInfoMapStructArrayBuilder < P > , GroupInfoMapStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn max_groups_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn max_group_keys_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn groupcast_adoption < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupcastAdoptionStructArrayBuilder < P > , GroupcastAdoptionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_group_key_map (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupKeyMapStruct < '_ > > , GroupKeyMapStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_groupcast_adoption (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupcastAdoptionStruct < '_ > > , GroupcastAdoptionStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_key_set_write (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetWriteRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_key_set_read < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetReadRequest < '_ > , response : KeySetReadResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_key_set_remove (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetRemoveRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_key_set_read_all_indices < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : KeySetReadAllIndicesResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn group_key_map < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupKeyMapStructArrayBuilder < P > , GroupKeyMapStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: group_key_map (self , ctx , builder ) 
    }
    fn group_table < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupInfoMapStructArrayBuilder < P > , GroupInfoMapStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: group_table (self , ctx , builder ) 
    }
    fn max_groups_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: max_groups_per_fabric (self , ctx ) 
    }
    fn max_group_keys_per_fabric (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: max_group_keys_per_fabric (self , ctx ) 
    }
    fn groupcast_adoption < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < GroupcastAdoptionStructArrayBuilder < P > , GroupcastAdoptionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: groupcast_adoption (self , ctx , builder ) 
    }
    fn set_group_key_map (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupKeyMapStruct < '_ > > , GroupKeyMapStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_group_key_map (self , ctx , value ) 
    }
    fn set_groupcast_adoption (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , GroupcastAdoptionStruct < '_ > > , GroupcastAdoptionStruct < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_groupcast_adoption (self , ctx , value ) 
    }
    fn handle_key_set_write (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetWriteRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_key_set_write (self , ctx , request ) 
    }
    fn handle_key_set_read < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetReadRequest < '_ > , response : KeySetReadResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_key_set_read (self , ctx , request , response ) 
    }
    fn handle_key_set_remove (& self , ctx : impl crate :: dm :: InvokeContext , request : KeySetRemoveRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_key_set_remove (self , ctx , request ) 
    }
    fn handle_key_set_read_all_indices < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : KeySetReadAllIndicesResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_key_set_read_all_indices (self , ctx , response ) 
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
                    AttributeId :: GroupKeyMap => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. group_key_map (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: GroupTable => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupTable , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. group_table (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupTable , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupTable , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxGroupsPerFabric => {
                        let attr_read_result = self . 0. max_groups_per_fabric (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxGroupsPerFabric , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxGroupKeysPerFabric => {
                        let attr_read_result = self . 0. max_group_keys_per_fabric (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxGroupKeysPerFabric , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: GroupcastAdoption => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. groupcast_adoption (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
            AttributeId :: GroupKeyMap => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_group_key_map (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupKeyMap , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: GroupcastAdoption => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_groupcast_adoption (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: GroupcastAdoption , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: KeySetWrite => {
                let cmd_data : KeySetWriteRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_key_set_write (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetWrite ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: KeySetRead => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRead ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (2u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_key_set_read (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRead ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRead ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: KeySetRemove => {
                let cmd_data : KeySetRemoveRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_key_set_remove (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetRemove ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: KeySetReadAllIndices => {
                crate :: dm :: log_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetReadAllIndices ) ) ) ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_key_set_read_all_indices (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetReadAllIndices ) ) ) , tw ) , tag , ) ? , ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: KeySetReadAllIndices ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 63u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GroupKeyManagement" , 63u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GroupKeyManagement" , 63u32 , self . 0.2 ) 
    }
    
}
