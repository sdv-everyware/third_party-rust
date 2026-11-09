# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const TERMS_AND_CONDITIONS = 1 ;
        const NETWORK_RECOVERY = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const TERMS_AND_CONDITIONS = 1 ;
        const NETWORK_RECOVERY = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CommissioningErrorEnum {
    # [enumval (0 ) ] OK = 0 , # [enumval (1 ) ] ValueOutsideRange = 1 , # [enumval (2 ) ] InvalidAuthentication = 2 , # [enumval (3 ) ] NoFailSafe = 3 , # [enumval (4 ) ] BusyWithOtherAdmin = 4 , # [enumval (5 ) ] RequiredTCNotAccepted = 5 , # [enumval (6 ) ] TCAcknowledgementsNotReceived = 6 , # [enumval (7 ) ] TCMinVersionNotMet = 7 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NetworkRecoveryReasonEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] Auth = 1 , # [enumval (2 ) ] Visibility = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RegulatoryLocationTypeEnum {
    # [enumval (0 ) ] Indoor = 0 , # [enumval (1 ) ] Outdoor = 1 , # [enumval (2 ) ] IndoorOutdoor = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BasicCommissioningInfoTag {
    FailSafeExpiryLengthSeconds = 0 , MaxCumulativeFailsafeSeconds = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ArmFailSafeRequestTag {
    ExpiryLengthSeconds = 0 , Breadcrumb = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ArmFailSafeResponseTag {
    ErrorCode = 0 , DebugText = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetRegulatoryConfigRequestTag {
    NewRegulatoryConfig = 0 , CountryCode = 1 , Breadcrumb = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetRegulatoryConfigResponseTag {
    ErrorCode = 0 , DebugText = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CommissioningCompleteResponseTag {
    ErrorCode = 0 , DebugText = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetTCAcknowledgementsRequestTag {
    TcVersion = 0 , TcUserResponse = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetTCAcknowledgementsResponseTag {
    ErrorCode = 0 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct BasicCommissioningInfo < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > BasicCommissioningInfo < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn fail_safe_expiry_length_seconds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn max_cumulative_failsafe_seconds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for BasicCommissioningInfo < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for BasicCommissioningInfo < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for BasicCommissioningInfo < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "BasicCommissioningInfo" ) ? ;
        match self . fail_safe_expiry_length_seconds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "fail_safe_expiry_length_seconds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fail_safe_expiry_length_seconds" , e . code () ) ? , 
        }
        match self . max_cumulative_failsafe_seconds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_cumulative_failsafe_seconds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_cumulative_failsafe_seconds" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for BasicCommissioningInfo < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "BasicCommissioningInfo" ) ;
        match self . fail_safe_expiry_length_seconds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "fail_safe_expiry_length_seconds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fail_safe_expiry_length_seconds" , e . code () ) , 
        }
        match self . max_cumulative_failsafe_seconds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_cumulative_failsafe_seconds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_cumulative_failsafe_seconds" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ArmFailSafeRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ArmFailSafeRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn expiry_length_seconds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn breadcrumb (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ArmFailSafeRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ArmFailSafeRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ArmFailSafeRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ArmFailSafeRequest" ) ? ;
        match self . expiry_length_seconds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "expiry_length_seconds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "expiry_length_seconds" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (value ) => write ! (f , "{}: {:?}," , "breadcrumb" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ArmFailSafeRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ArmFailSafeRequest" ) ;
        match self . expiry_length_seconds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "expiry_length_seconds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "expiry_length_seconds" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "breadcrumb" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ArmFailSafeResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ArmFailSafeResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn error_code (& self ) -> Result < CommissioningErrorEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn debug_text (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ArmFailSafeResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ArmFailSafeResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ArmFailSafeResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ArmFailSafeResponse" ) ? ;
        match self . error_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "error_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) ? , 
        }
        match self . debug_text () {
            Ok (value ) => write ! (f , "{}: {:?}," , "debug_text" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ArmFailSafeResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ArmFailSafeResponse" ) ;
        match self . error_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "error_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) , 
        }
        match self . debug_text () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "debug_text" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetRegulatoryConfigRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetRegulatoryConfigRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn new_regulatory_config (& self ) -> Result < RegulatoryLocationTypeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn country_code (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn breadcrumb (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetRegulatoryConfigRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetRegulatoryConfigRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetRegulatoryConfigRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetRegulatoryConfigRequest" ) ? ;
        match self . new_regulatory_config () {
            Ok (value ) => write ! (f , "{}: {:?}," , "new_regulatory_config" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "new_regulatory_config" , e . code () ) ? , 
        }
        match self . country_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "country_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "country_code" , e . code () ) ? , 
        }
        match self . breadcrumb () {
            Ok (value ) => write ! (f , "{}: {:?}," , "breadcrumb" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetRegulatoryConfigRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetRegulatoryConfigRequest" ) ;
        match self . new_regulatory_config () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "new_regulatory_config" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "new_regulatory_config" , e . code () ) , 
        }
        match self . country_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "country_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "country_code" , e . code () ) , 
        }
        match self . breadcrumb () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "breadcrumb" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "breadcrumb" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetRegulatoryConfigResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetRegulatoryConfigResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn error_code (& self ) -> Result < CommissioningErrorEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn debug_text (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetRegulatoryConfigResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetRegulatoryConfigResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetRegulatoryConfigResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetRegulatoryConfigResponse" ) ? ;
        match self . error_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "error_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) ? , 
        }
        match self . debug_text () {
            Ok (value ) => write ! (f , "{}: {:?}," , "debug_text" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetRegulatoryConfigResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetRegulatoryConfigResponse" ) ;
        match self . error_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "error_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) , 
        }
        match self . debug_text () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "debug_text" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CommissioningCompleteResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CommissioningCompleteResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn error_code (& self ) -> Result < CommissioningErrorEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn debug_text (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CommissioningCompleteResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CommissioningCompleteResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CommissioningCompleteResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CommissioningCompleteResponse" ) ? ;
        match self . error_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "error_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) ? , 
        }
        match self . debug_text () {
            Ok (value ) => write ! (f , "{}: {:?}," , "debug_text" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CommissioningCompleteResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CommissioningCompleteResponse" ) ;
        match self . error_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "error_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) , 
        }
        match self . debug_text () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "debug_text" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetTCAcknowledgementsRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetTCAcknowledgementsRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn tc_version (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn tc_user_response (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetTCAcknowledgementsRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetTCAcknowledgementsRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetTCAcknowledgementsRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetTCAcknowledgementsRequest" ) ? ;
        match self . tc_version () {
            Ok (value ) => write ! (f , "{}: {:?}," , "tc_version" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "tc_version" , e . code () ) ? , 
        }
        match self . tc_user_response () {
            Ok (value ) => write ! (f , "{}: {:?}," , "tc_user_response" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "tc_user_response" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetTCAcknowledgementsRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetTCAcknowledgementsRequest" ) ;
        match self . tc_version () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "tc_version" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "tc_version" , e . code () ) , 
        }
        match self . tc_user_response () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "tc_user_response" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "tc_user_response" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetTCAcknowledgementsResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetTCAcknowledgementsResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn error_code (& self ) -> Result < CommissioningErrorEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetTCAcknowledgementsResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetTCAcknowledgementsResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetTCAcknowledgementsResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetTCAcknowledgementsResponse" ) ? ;
        match self . error_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "error_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetTCAcknowledgementsResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetTCAcknowledgementsResponse" ) ;
        match self . error_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "error_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "error_code" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct BasicCommissioningInfoBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > BasicCommissioningInfoBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > BasicCommissioningInfoBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fail_safe_expiry_length_seconds (mut self , value : u16 ) -> Result < BasicCommissioningInfoBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "failSafeExpiryLengthSeconds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (BasicCommissioningInfoBuilder (self . 0 ) ) 
    }
    
}
impl < P > BasicCommissioningInfoBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_cumulative_failsafe_seconds (mut self , value : u16 ) -> Result < BasicCommissioningInfoBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxCumulativeFailsafeSeconds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (BasicCommissioningInfoBuilder (self . 0 ) ) 
    }
    
}
impl < P > BasicCommissioningInfoBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for BasicCommissioningInfoBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BasicCommissioningInfo" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for BasicCommissioningInfoBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BasicCommissioningInfo" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for BasicCommissioningInfoBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BasicCommissioningInfoBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BasicCommissioningInfoArrayBuilder < P > (P ) ;
impl < P > BasicCommissioningInfoArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < BasicCommissioningInfoBuilder < BasicCommissioningInfoArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BasicCommissioningInfoArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for BasicCommissioningInfoArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BasicCommissioningInfo[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for BasicCommissioningInfoArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BasicCommissioningInfo[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for BasicCommissioningInfoArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BasicCommissioningInfoArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ArmFailSafeRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ArmFailSafeRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ArmFailSafeRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn expiry_length_seconds (mut self , value : u16 ) -> Result < ArmFailSafeRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "expiryLengthSeconds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ArmFailSafeRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ArmFailSafeRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : u64 ) -> Result < ArmFailSafeRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ArmFailSafeRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ArmFailSafeRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ArmFailSafeRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ArmFailSafeRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ArmFailSafeRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ArmFailSafeRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ArmFailSafeRequestArrayBuilder < P > (P ) ;
impl < P > ArmFailSafeRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ArmFailSafeRequestBuilder < ArmFailSafeRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ArmFailSafeRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ArmFailSafeRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ArmFailSafeRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ArmFailSafeRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ArmFailSafeRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ArmFailSafeResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ArmFailSafeResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ArmFailSafeResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn error_code (mut self , value : CommissioningErrorEnum ) -> Result < ArmFailSafeResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "errorCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ArmFailSafeResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ArmFailSafeResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn debug_text (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ArmFailSafeResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "debugText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ArmFailSafeResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ArmFailSafeResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ArmFailSafeResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ArmFailSafeResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ArmFailSafeResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ArmFailSafeResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ArmFailSafeResponseArrayBuilder < P > (P ) ;
impl < P > ArmFailSafeResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ArmFailSafeResponseBuilder < ArmFailSafeResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ArmFailSafeResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ArmFailSafeResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ArmFailSafeResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ArmFailSafeResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ArmFailSafeResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ArmFailSafeResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetRegulatoryConfigRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetRegulatoryConfigRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetRegulatoryConfigRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn new_regulatory_config (mut self , value : RegulatoryLocationTypeEnum ) -> Result < SetRegulatoryConfigRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "newRegulatoryConfig" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetRegulatoryConfigRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetRegulatoryConfigRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn country_code (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < SetRegulatoryConfigRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "countryCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetRegulatoryConfigRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetRegulatoryConfigRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (mut self , value : u64 ) -> Result < SetRegulatoryConfigRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "breadcrumb" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SetRegulatoryConfigRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetRegulatoryConfigRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetRegulatoryConfigRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetRegulatoryConfigRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetRegulatoryConfigRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetRegulatoryConfigRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetRegulatoryConfigRequestArrayBuilder < P > (P ) ;
impl < P > SetRegulatoryConfigRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetRegulatoryConfigRequestBuilder < SetRegulatoryConfigRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetRegulatoryConfigRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetRegulatoryConfigRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetRegulatoryConfigRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetRegulatoryConfigRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetRegulatoryConfigRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetRegulatoryConfigResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetRegulatoryConfigResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetRegulatoryConfigResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn error_code (mut self , value : CommissioningErrorEnum ) -> Result < SetRegulatoryConfigResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "errorCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetRegulatoryConfigResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetRegulatoryConfigResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn debug_text (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < SetRegulatoryConfigResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "debugText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetRegulatoryConfigResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetRegulatoryConfigResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetRegulatoryConfigResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetRegulatoryConfigResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetRegulatoryConfigResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetRegulatoryConfigResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetRegulatoryConfigResponseArrayBuilder < P > (P ) ;
impl < P > SetRegulatoryConfigResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetRegulatoryConfigResponseBuilder < SetRegulatoryConfigResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetRegulatoryConfigResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetRegulatoryConfigResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetRegulatoryConfigResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetRegulatoryConfigResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetRegulatoryConfigResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetRegulatoryConfigResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CommissioningCompleteResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CommissioningCompleteResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CommissioningCompleteResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn error_code (mut self , value : CommissioningErrorEnum ) -> Result < CommissioningCompleteResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "errorCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CommissioningCompleteResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > CommissioningCompleteResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn debug_text (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < CommissioningCompleteResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "debugText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (CommissioningCompleteResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > CommissioningCompleteResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CommissioningCompleteResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CommissioningCompleteResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CommissioningCompleteResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CommissioningCompleteResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CommissioningCompleteResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CommissioningCompleteResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CommissioningCompleteResponseArrayBuilder < P > (P ) ;
impl < P > CommissioningCompleteResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CommissioningCompleteResponseBuilder < CommissioningCompleteResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CommissioningCompleteResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CommissioningCompleteResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CommissioningCompleteResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CommissioningCompleteResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CommissioningCompleteResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CommissioningCompleteResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CommissioningCompleteResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetTCAcknowledgementsRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetTCAcknowledgementsRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetTCAcknowledgementsRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn tc_version (mut self , value : u16 ) -> Result < SetTCAcknowledgementsRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "TCVersion" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetTCAcknowledgementsRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetTCAcknowledgementsRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn tc_user_response (mut self , value : u16 ) -> Result < SetTCAcknowledgementsRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "TCUserResponse" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetTCAcknowledgementsRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetTCAcknowledgementsRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetTCAcknowledgementsRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetTCAcknowledgementsRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetTCAcknowledgementsRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetTCAcknowledgementsRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetTCAcknowledgementsRequestArrayBuilder < P > (P ) ;
impl < P > SetTCAcknowledgementsRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetTCAcknowledgementsRequestBuilder < SetTCAcknowledgementsRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetTCAcknowledgementsRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetTCAcknowledgementsRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetTCAcknowledgementsRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetTCAcknowledgementsRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetTCAcknowledgementsRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetTCAcknowledgementsResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetTCAcknowledgementsResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetTCAcknowledgementsResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn error_code (mut self , value : CommissioningErrorEnum ) -> Result < SetTCAcknowledgementsResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "errorCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetTCAcknowledgementsResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetTCAcknowledgementsResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetTCAcknowledgementsResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetTCAcknowledgementsResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetTCAcknowledgementsResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetTCAcknowledgementsResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetTCAcknowledgementsResponseArrayBuilder < P > (P ) ;
impl < P > SetTCAcknowledgementsResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetTCAcknowledgementsResponseBuilder < SetTCAcknowledgementsResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetTCAcknowledgementsResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetTCAcknowledgementsResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetTCAcknowledgementsResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetTCAcknowledgementsResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetTCAcknowledgementsResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetTCAcknowledgementsResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    Breadcrumb = 0 , BasicCommissioningInfo = 1 , RegulatoryConfig = 2 , LocationCapability = 3 , SupportsConcurrentConnection = 4 , TCAcceptedVersion = 5 , TCMinRequiredVersion = 6 , TCAcknowledgements = 7 , TCAcknowledgementsRequired = 8 , TCUpdateDeadline = 9 , RecoveryIdentifier = 10 , NetworkRecoveryReason = 11 , IsCommissioningWithoutPower = 12 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: Breadcrumb => write ! (f , "{}(0x{:02x})" , "Breadcrumb" , AttributeId :: Breadcrumb as u32 ) ? , AttributeId :: BasicCommissioningInfo => write ! (f , "{}(0x{:02x})" , "BasicCommissioningInfo" , AttributeId :: BasicCommissioningInfo as u32 ) ? , AttributeId :: RegulatoryConfig => write ! (f , "{}(0x{:02x})" , "RegulatoryConfig" , AttributeId :: RegulatoryConfig as u32 ) ? , AttributeId :: LocationCapability => write ! (f , "{}(0x{:02x})" , "LocationCapability" , AttributeId :: LocationCapability as u32 ) ? , AttributeId :: SupportsConcurrentConnection => write ! (f , "{}(0x{:02x})" , "SupportsConcurrentConnection" , AttributeId :: SupportsConcurrentConnection as u32 ) ? , AttributeId :: TCAcceptedVersion => write ! (f , "{}(0x{:02x})" , "TCAcceptedVersion" , AttributeId :: TCAcceptedVersion as u32 ) ? , AttributeId :: TCMinRequiredVersion => write ! (f , "{}(0x{:02x})" , "TCMinRequiredVersion" , AttributeId :: TCMinRequiredVersion as u32 ) ? , AttributeId :: TCAcknowledgements => write ! (f , "{}(0x{:02x})" , "TCAcknowledgements" , AttributeId :: TCAcknowledgements as u32 ) ? , AttributeId :: TCAcknowledgementsRequired => write ! (f , "{}(0x{:02x})" , "TCAcknowledgementsRequired" , AttributeId :: TCAcknowledgementsRequired as u32 ) ? , AttributeId :: TCUpdateDeadline => write ! (f , "{}(0x{:02x})" , "TCUpdateDeadline" , AttributeId :: TCUpdateDeadline as u32 ) ? , AttributeId :: RecoveryIdentifier => write ! (f , "{}(0x{:02x})" , "RecoveryIdentifier" , AttributeId :: RecoveryIdentifier as u32 ) ? , AttributeId :: NetworkRecoveryReason => write ! (f , "{}(0x{:02x})" , "NetworkRecoveryReason" , AttributeId :: NetworkRecoveryReason as u32 ) ? , AttributeId :: IsCommissioningWithoutPower => write ! (f , "{}(0x{:02x})" , "IsCommissioningWithoutPower" , AttributeId :: IsCommissioningWithoutPower as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: Breadcrumb => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Breadcrumb" , AttributeId :: Breadcrumb as u32 ) , AttributeId :: BasicCommissioningInfo => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BasicCommissioningInfo" , AttributeId :: BasicCommissioningInfo as u32 ) , AttributeId :: RegulatoryConfig => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RegulatoryConfig" , AttributeId :: RegulatoryConfig as u32 ) , AttributeId :: LocationCapability => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LocationCapability" , AttributeId :: LocationCapability as u32 ) , AttributeId :: SupportsConcurrentConnection => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SupportsConcurrentConnection" , AttributeId :: SupportsConcurrentConnection as u32 ) , AttributeId :: TCAcceptedVersion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TCAcceptedVersion" , AttributeId :: TCAcceptedVersion as u32 ) , AttributeId :: TCMinRequiredVersion => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TCMinRequiredVersion" , AttributeId :: TCMinRequiredVersion as u32 ) , AttributeId :: TCAcknowledgements => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TCAcknowledgements" , AttributeId :: TCAcknowledgements as u32 ) , AttributeId :: TCAcknowledgementsRequired => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TCAcknowledgementsRequired" , AttributeId :: TCAcknowledgementsRequired as u32 ) , AttributeId :: TCUpdateDeadline => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TCUpdateDeadline" , AttributeId :: TCUpdateDeadline as u32 ) , AttributeId :: RecoveryIdentifier => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RecoveryIdentifier" , AttributeId :: RecoveryIdentifier as u32 ) , AttributeId :: NetworkRecoveryReason => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NetworkRecoveryReason" , AttributeId :: NetworkRecoveryReason as u32 ) , AttributeId :: IsCommissioningWithoutPower => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "IsCommissioningWithoutPower" , AttributeId :: IsCommissioningWithoutPower as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    ArmFailSafe = 0 , SetRegulatoryConfig = 2 , CommissioningComplete = 4 , SetTCAcknowledgements = 6 
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
            CommandId :: ArmFailSafe => write ! (f , "{}(0x{:02x})" , "ArmFailSafe" , CommandId :: ArmFailSafe as u32 ) ? , CommandId :: SetRegulatoryConfig => write ! (f , "{}(0x{:02x})" , "SetRegulatoryConfig" , CommandId :: SetRegulatoryConfig as u32 ) ? , CommandId :: CommissioningComplete => write ! (f , "{}(0x{:02x})" , "CommissioningComplete" , CommandId :: CommissioningComplete as u32 ) ? , CommandId :: SetTCAcknowledgements => write ! (f , "{}(0x{:02x})" , "SetTCAcknowledgements" , CommandId :: SetTCAcknowledgements as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: ArmFailSafe => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ArmFailSafe" , CommandId :: ArmFailSafe as u32 ) , CommandId :: SetRegulatoryConfig => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetRegulatoryConfig" , CommandId :: SetRegulatoryConfig as u32 ) , CommandId :: CommissioningComplete => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CommissioningComplete" , CommandId :: CommissioningComplete as u32 ) , CommandId :: SetTCAcknowledgements => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetTCAcknowledgements" , CommandId :: SetTCAcknowledgements as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    ArmFailSafeResponse = 1 , SetRegulatoryConfigResponse = 3 , CommissioningCompleteResponse = 5 , SetTCAcknowledgementsResponse = 7 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: ArmFailSafeResponse => write ! (f , "{}(0x{:02x})" , "ArmFailSafeResponse" , CommandResponseId :: ArmFailSafeResponse as u32 ) ? , CommandResponseId :: SetRegulatoryConfigResponse => write ! (f , "{}(0x{:02x})" , "SetRegulatoryConfigResponse" , CommandResponseId :: SetRegulatoryConfigResponse as u32 ) ? , CommandResponseId :: CommissioningCompleteResponse => write ! (f , "{}(0x{:02x})" , "CommissioningCompleteResponse" , CommandResponseId :: CommissioningCompleteResponse as u32 ) ? , CommandResponseId :: SetTCAcknowledgementsResponse => write ! (f , "{}(0x{:02x})" , "SetTCAcknowledgementsResponse" , CommandResponseId :: SetTCAcknowledgementsResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: ArmFailSafeResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ArmFailSafeResponse" , CommandResponseId :: ArmFailSafeResponse as u32 ) , CommandResponseId :: SetRegulatoryConfigResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetRegulatoryConfigResponse" , CommandResponseId :: SetRegulatoryConfigResponse as u32 ) , CommandResponseId :: CommissioningCompleteResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CommissioningCompleteResponse" , CommandResponseId :: CommissioningCompleteResponse as u32 ) , CommandResponseId :: SetTCAcknowledgementsResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetTCAcknowledgementsResponse" , CommandResponseId :: SetTCAcknowledgementsResponse as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (48 , 2 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: Breadcrumb as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_ADMIN . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: BasicCommissioningInfo as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: RegulatoryConfig as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: LocationCapability as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: SupportsConcurrentConnection as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: TCAcceptedVersion as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TCMinRequiredVersion as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TCAcknowledgements as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TCAcknowledgementsRequired as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TCUpdateDeadline as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RecoveryIdentifier as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NetworkRecoveryReason as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: IsCommissioningWithoutPower as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: ArmFailSafe as _ , Some (CommandResponseId :: ArmFailSafeResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: SetRegulatoryConfig as _ , Some (CommandResponseId :: SetRegulatoryConfigResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: CommissioningComplete as _ , Some (CommandResponseId :: CommissioningCompleteResponse as _ ) , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: SetTCAcknowledgements as _ , Some (CommandResponseId :: SetTCAcknowledgementsResponse as _ ) , crate :: dm :: Access :: WA , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn breadcrumb (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u64 , crate :: error :: Error > ;
    fn basic_commissioning_info < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : BasicCommissioningInfoBuilder < P > ) -> Result < P , crate :: error :: Error > ;
    fn regulatory_config (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RegulatoryLocationTypeEnum , crate :: error :: Error > ;
    fn location_capability (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RegulatoryLocationTypeEnum , crate :: error :: Error > ;
    fn supports_concurrent_connection (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > ;
    fn tc_accepted_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tc_min_required_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tc_acknowledgements (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tc_acknowledgements_required (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn tc_update_deadline (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn recovery_identifier < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: OctetsBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn network_recovery_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < NetworkRecoveryReasonEnum > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn is_commissioning_without_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_breadcrumb (& self , ctx : impl crate :: dm :: WriteContext , value : u64 ) -> Result < () , crate :: error :: Error > ;
    fn handle_arm_fail_safe < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ArmFailSafeRequest < '_ > , response : ArmFailSafeResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_set_regulatory_config < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetRegulatoryConfigRequest < '_ > , response : SetRegulatoryConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_commissioning_complete < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : CommissioningCompleteResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_set_tc_acknowledgements < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTCAcknowledgementsRequest < '_ > , response : SetTCAcknowledgementsResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
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
    fn breadcrumb (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u64 , crate :: error :: Error > {
        T :: breadcrumb (self , ctx ) 
    }
    fn basic_commissioning_info < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : BasicCommissioningInfoBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: basic_commissioning_info (self , ctx , builder ) 
    }
    fn regulatory_config (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RegulatoryLocationTypeEnum , crate :: error :: Error > {
        T :: regulatory_config (self , ctx ) 
    }
    fn location_capability (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RegulatoryLocationTypeEnum , crate :: error :: Error > {
        T :: location_capability (self , ctx ) 
    }
    fn supports_concurrent_connection (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: supports_concurrent_connection (self , ctx ) 
    }
    fn tc_accepted_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: tc_accepted_version (self , ctx ) 
    }
    fn tc_min_required_version (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: tc_min_required_version (self , ctx ) 
    }
    fn tc_acknowledgements (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: tc_acknowledgements (self , ctx ) 
    }
    fn tc_acknowledgements_required (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: tc_acknowledgements_required (self , ctx ) 
    }
    fn tc_update_deadline (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: tc_update_deadline (self , ctx ) 
    }
    fn recovery_identifier < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: OctetsBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: recovery_identifier (self , ctx , builder ) 
    }
    fn network_recovery_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < NetworkRecoveryReasonEnum > , crate :: error :: Error > {
        T :: network_recovery_reason (self , ctx ) 
    }
    fn is_commissioning_without_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: is_commissioning_without_power (self , ctx ) 
    }
    fn set_breadcrumb (& self , ctx : impl crate :: dm :: WriteContext , value : u64 ) -> Result < () , crate :: error :: Error > {
        T :: set_breadcrumb (self , ctx , value ) 
    }
    fn handle_arm_fail_safe < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ArmFailSafeRequest < '_ > , response : ArmFailSafeResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_arm_fail_safe (self , ctx , request , response ) 
    }
    fn handle_set_regulatory_config < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetRegulatoryConfigRequest < '_ > , response : SetRegulatoryConfigResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_set_regulatory_config (self , ctx , request , response ) 
    }
    fn handle_commissioning_complete < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : CommissioningCompleteResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_commissioning_complete (self , ctx , response ) 
    }
    fn handle_set_tc_acknowledgements < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTCAcknowledgementsRequest < '_ > , response : SetTCAcknowledgementsResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_set_tc_acknowledgements (self , ctx , request , response ) 
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
                    AttributeId :: Breadcrumb => {
                        let attr_read_result = self . 0. breadcrumb (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Breadcrumb , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BasicCommissioningInfo => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BasicCommissioningInfo , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. basic_commissioning_info (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BasicCommissioningInfo , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BasicCommissioningInfo , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RegulatoryConfig => {
                        let attr_read_result = self . 0. regulatory_config (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RegulatoryConfig , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocationCapability => {
                        let attr_read_result = self . 0. location_capability (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocationCapability , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SupportsConcurrentConnection => {
                        let attr_read_result = self . 0. supports_concurrent_connection (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportsConcurrentConnection , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCAcceptedVersion => {
                        let attr_read_result = self . 0. tc_accepted_version (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCAcceptedVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCMinRequiredVersion => {
                        let attr_read_result = self . 0. tc_min_required_version (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCMinRequiredVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCAcknowledgements => {
                        let attr_read_result = self . 0. tc_acknowledgements (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCAcknowledgements , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCAcknowledgementsRequired => {
                        let attr_read_result = self . 0. tc_acknowledgements_required (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCAcknowledgementsRequired , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCUpdateDeadline => {
                        let attr_read_result = self . 0. tc_update_deadline (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCUpdateDeadline , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RecoveryIdentifier => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RecoveryIdentifier , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. recovery_identifier (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RecoveryIdentifier , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RecoveryIdentifier , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: NetworkRecoveryReason => {
                        let attr_read_result = self . 0. network_recovery_reason (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkRecoveryReason , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: IsCommissioningWithoutPower => {
                        let attr_read_result = self . 0. is_commissioning_without_power (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: IsCommissioningWithoutPower , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: Breadcrumb => {
                let attr_data : u64 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_breadcrumb (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Breadcrumb , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: ArmFailSafe => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ArmFailSafe ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_arm_fail_safe (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ArmFailSafe ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ArmFailSafe ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: SetRegulatoryConfig => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetRegulatoryConfig ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (3u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_set_regulatory_config (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetRegulatoryConfig ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetRegulatoryConfig ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: CommissioningComplete => {
                crate :: dm :: log_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CommissioningComplete ) ) ) ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_commissioning_complete (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CommissioningComplete ) ) ) , tw ) , tag , ) ? , ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CommissioningComplete ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: SetTCAcknowledgements => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTCAcknowledgements ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (7u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_set_tc_acknowledgements (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTCAcknowledgements ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTCAcknowledgements ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 48u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralCommissioning" , 48u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralCommissioning" , 48u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `GeneralCommissioning` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct GeneralCommissioningCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > GeneralCommissioningCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn arm_fail_safe (self , endpoint : crate :: dm :: EndptId , ) -> Result < ArmFailSafeRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 48 , 0 ) ? . data_builder () 
    }
    pub fn set_regulatory_config (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetRegulatoryConfigRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 48 , 2 ) ? . data_builder () 
    }
    pub fn commissioning_complete (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        let array = self . array . push () ? . path (endpoint , 48 , 4 ) ? . data (| w | {
            w . start_struct (& crate :: tlv :: TLVTag :: Context (crate :: im :: CmdDataTag :: Data as u8 , ) ) ? ;
            w . end_container () 
        }
        ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn set_tc_acknowledgements (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetTCAcknowledgementsRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 48 , 6 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GeneralCommissioning` cluster's commands. `use` this trait to call `.general_commissioning_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`GeneralCommissioningCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GeneralCommissioningCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn general_commissioning_inv (self ) -> GeneralCommissioningCmdRequestsView < P > ;
    
}
impl < P > GeneralCommissioningCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn general_commissioning_inv (self ) -> GeneralCommissioningCmdRequestsView < P > {
        GeneralCommissioningCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `GeneralCommissioning` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralCommissioningAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > GeneralCommissioningAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn basic_commissioning_info (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn regulatory_config (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn location_capability (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn supports_concurrent_connection (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tc_accepted_version (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tc_min_required_version (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tc_acknowledgements (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tc_acknowledgements_required (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn tc_update_deadline (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn recovery_identifier (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn network_recovery_reason (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (11 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn is_commissioning_without_power (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (12 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (48 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GeneralCommissioning` cluster's attribute reads. `use` this trait to call `.general_commissioning_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`GeneralCommissioningAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GeneralCommissioningAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn general_commissioning_read (self ) -> GeneralCommissioningAttrReadsView < P > ;
    
}
impl < P > GeneralCommissioningAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn general_commissioning_read (self ) -> GeneralCommissioningAttrReadsView < P > {
        GeneralCommissioningAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `GeneralCommissioning` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralCommissioningAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > GeneralCommissioningAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn breadcrumb (self , endpoint : crate :: dm :: EndptId , value : u64 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 48 , 0 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `GeneralCommissioning` cluster's attribute writes. `use` this trait to call `.general_commissioning_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`GeneralCommissioningAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait GeneralCommissioningAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn general_commissioning_write (self ) -> GeneralCommissioningAttrWritesView < P > ;
    
}
impl < P > GeneralCommissioningAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn general_commissioning_write (self ) -> GeneralCommissioningAttrWritesView < P > {
        GeneralCommissioningAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `GeneralCommissioning` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct GeneralCommissioningCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > GeneralCommissioningCmdResponsesView < 'a , 'r > {
    pub fn arm_fail_safe (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ArmFailSafeResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < ArmFailSafeResponse < 'a >> (48 , 0 ) 
    }
    pub fn set_regulatory_config (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < SetRegulatoryConfigResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < SetRegulatoryConfigResponse < 'a >> (48 , 2 ) 
    }
    pub fn commissioning_complete (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < CommissioningCompleteResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < CommissioningCompleteResponse < 'a >> (48 , 4 ) 
    }
    pub fn set_tc_acknowledgements (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < SetTCAcknowledgementsResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < SetTCAcknowledgementsResponse < 'a >> (48 , 6 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GeneralCommissioning`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.general_commissioning_inv_resp()` on an `InvokeResp`; the returned [`GeneralCommissioningCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait GeneralCommissioningCmdResponses < 'a > {
    fn general_commissioning_inv_resp (& self ) -> GeneralCommissioningCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > GeneralCommissioningCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn general_commissioning_inv_resp (& self ) -> GeneralCommissioningCmdResponsesView < 'a , '_ > {
        GeneralCommissioningCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `GeneralCommissioning` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralCommissioningAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > GeneralCommissioningAttrResponsesView < 'a , 'r > {
    pub fn breadcrumb (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u64 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u64 > (48 , 0 ) 
    }
    pub fn basic_commissioning_info (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < BasicCommissioningInfo < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < BasicCommissioningInfo < 'a > > (48 , 1 ) 
    }
    pub fn regulatory_config (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < RegulatoryLocationTypeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < RegulatoryLocationTypeEnum > (48 , 2 ) 
    }
    pub fn location_capability (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < RegulatoryLocationTypeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < RegulatoryLocationTypeEnum > (48 , 3 ) 
    }
    pub fn supports_concurrent_connection (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (48 , 4 ) 
    }
    pub fn tc_accepted_version (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (48 , 5 ) 
    }
    pub fn tc_min_required_version (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (48 , 6 ) 
    }
    pub fn tc_acknowledgements (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (48 , 7 ) 
    }
    pub fn tc_acknowledgements_required (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (48 , 8 ) 
    }
    pub fn tc_update_deadline (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (48 , 9 ) 
    }
    pub fn recovery_identifier (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: OctetStr < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: OctetStr < 'a > > (48 , 10 ) 
    }
    pub fn network_recovery_reason (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < NetworkRecoveryReasonEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < NetworkRecoveryReasonEnum > > (48 , 11 ) 
    }
    pub fn is_commissioning_without_power (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (48 , 12 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (48 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (48 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (48 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (48 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (48 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GeneralCommissioning`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.general_commissioning_read_resp()` on a `ReportDataResp`; the returned [`GeneralCommissioningAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait GeneralCommissioningAttrResponses < 'a > {
    fn general_commissioning_read_resp (& self ) -> GeneralCommissioningAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > GeneralCommissioningAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn general_commissioning_read_resp (& self ) -> GeneralCommissioningAttrResponsesView < 'a , '_ > {
        GeneralCommissioningAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `GeneralCommissioning` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct GeneralCommissioningWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > GeneralCommissioningWriteResponsesView < 'a , 'r > {
    pub fn breadcrumb (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (48 , 0 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `GeneralCommissioning`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.general_commissioning_write_resp()` on a `WriteResp`; the returned [`GeneralCommissioningWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait GeneralCommissioningWriteResponses < 'a > {
    fn general_commissioning_write_resp (& self ) -> GeneralCommissioningWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > GeneralCommissioningWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn general_commissioning_write_resp (& self ) -> GeneralCommissioningWriteResponsesView < 'a , '_ > {
        GeneralCommissioningWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `ArmFailSafeResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`ArmFailSafeResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct ArmFailSafeResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > ArmFailSafeResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < ArmFailSafeResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `SetRegulatoryConfigResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`SetRegulatoryConfigResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct SetRegulatoryConfigResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > SetRegulatoryConfigResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < SetRegulatoryConfigResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `CommissioningCompleteResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`CommissioningCompleteResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct CommissioningCompleteResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > CommissioningCompleteResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < CommissioningCompleteResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `SetTCAcknowledgementsResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`SetTCAcknowledgementsResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct SetTCAcknowledgementsResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > SetTCAcknowledgementsResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < SetTCAcknowledgementsResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `GeneralCommissioning` cluster's client operations. Returned by [`GeneralCommissioningClient::general_commissioning`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct GeneralCommissioningClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > GeneralCommissioningClientView < T > {
    pub async fn arm_fail_safe < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < ArmFailSafeResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (ArmFailSafeRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GeneralCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . general_commissioning_inv () ;
            let req_builder = view . arm_fail_safe (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (ArmFailSafeResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn set_regulatory_config < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < SetRegulatoryConfigResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (SetRegulatoryConfigRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GeneralCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . general_commissioning_inv () ;
            let req_builder = view . set_regulatory_config (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (SetRegulatoryConfigResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn commissioning_complete (self , endpoint : crate :: dm :: EndptId , ) -> Result < CommissioningCompleteResponseHandle < T :: InvokeChunk > , crate :: error :: Error > {
        use self :: GeneralCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            msg . invoke_requests () ? . general_commissioning_inv () . commissioning_complete (endpoint ) ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (CommissioningCompleteResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn set_tc_acknowledgements < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < SetTCAcknowledgementsResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (SetTCAcknowledgementsRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: GeneralCommissioningCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . general_commissioning_inv () ;
            let req_builder = view . set_tc_acknowledgements (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (SetTCAcknowledgementsResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn breadcrumb_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . breadcrumb (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn basic_commissioning_info_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < BasicCommissioningInfo < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . basic_commissioning_info (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn regulatory_config_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < RegulatoryLocationTypeEnum , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . regulatory_config (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn location_capability_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < RegulatoryLocationTypeEnum , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . location_capability (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn supports_concurrent_connection_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . supports_concurrent_connection (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tc_accepted_version_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . tc_accepted_version (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tc_min_required_version_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . tc_min_required_version (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tc_acknowledgements_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . tc_acknowledgements (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tc_acknowledgements_required_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . tc_acknowledgements_required (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn tc_update_deadline_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . tc_update_deadline (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn recovery_identifier_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . recovery_identifier (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn network_recovery_reason_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < NetworkRecoveryReasonEnum > , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . network_recovery_reason (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn is_commissioning_without_power_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . is_commissioning_without_power (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . general_commissioning_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn breadcrumb_write (self , endpoint : crate :: dm :: EndptId , value : u64 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: GeneralCommissioningAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . general_commissioning_write () . breadcrumb (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `GeneralCommissioning` cluster. `use` this trait to call `.general_commissioning()` on any [`crate::im::ImClient`], such as an exchange; the returned [`GeneralCommissioningClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `GeneralCommissioningClientView` keeps each cluster's API surface narrow: at the call site `exchange.general_commissioning().` shows only this cluster's methods in IDE completion." ] pub trait GeneralCommissioningClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn general_commissioning (self ) -> GeneralCommissioningClientView < Self > {
        GeneralCommissioningClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > GeneralCommissioningClient for T {
    
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
    fn breadcrumb (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn basic_commissioning_info < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : BasicCommissioningInfoBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn regulatory_config (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RegulatoryLocationTypeEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn location_capability (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RegulatoryLocationTypeEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn supports_concurrent_connection (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn tc_accepted_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tc_min_required_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tc_acknowledgements (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tc_acknowledgements_required (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn tc_update_deadline (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn recovery_identifier < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: OctetsBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn network_recovery_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < NetworkRecoveryReasonEnum > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn is_commissioning_without_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_breadcrumb (& self , ctx : impl crate :: dm :: WriteContext , value : u64 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_arm_fail_safe < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ArmFailSafeRequest < '_ > , response : ArmFailSafeResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_regulatory_config < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetRegulatoryConfigRequest < '_ > , response : SetRegulatoryConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_commissioning_complete < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : CommissioningCompleteResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_tc_acknowledgements < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTCAcknowledgementsRequest < '_ > , response : SetTCAcknowledgementsResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn breadcrumb (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u64 , crate :: error :: Error >> {
        T :: breadcrumb (self , ctx ) 
    }
    fn basic_commissioning_info < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : BasicCommissioningInfoBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: basic_commissioning_info (self , ctx , builder ) 
    }
    fn regulatory_config (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RegulatoryLocationTypeEnum , crate :: error :: Error >> {
        T :: regulatory_config (self , ctx ) 
    }
    fn location_capability (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RegulatoryLocationTypeEnum , crate :: error :: Error >> {
        T :: location_capability (self , ctx ) 
    }
    fn supports_concurrent_connection (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: supports_concurrent_connection (self , ctx ) 
    }
    fn tc_accepted_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: tc_accepted_version (self , ctx ) 
    }
    fn tc_min_required_version (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: tc_min_required_version (self , ctx ) 
    }
    fn tc_acknowledgements (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: tc_acknowledgements (self , ctx ) 
    }
    fn tc_acknowledgements_required (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: tc_acknowledgements_required (self , ctx ) 
    }
    fn tc_update_deadline (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: tc_update_deadline (self , ctx ) 
    }
    fn recovery_identifier < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: OctetsBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: recovery_identifier (self , ctx , builder ) 
    }
    fn network_recovery_reason (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < NetworkRecoveryReasonEnum > , crate :: error :: Error >> {
        T :: network_recovery_reason (self , ctx ) 
    }
    fn is_commissioning_without_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: is_commissioning_without_power (self , ctx ) 
    }
    fn set_breadcrumb (& self , ctx : impl crate :: dm :: WriteContext , value : u64 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_breadcrumb (self , ctx , value ) 
    }
    fn handle_arm_fail_safe < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ArmFailSafeRequest < '_ > , response : ArmFailSafeResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_arm_fail_safe (self , ctx , request , response ) 
    }
    fn handle_set_regulatory_config < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetRegulatoryConfigRequest < '_ > , response : SetRegulatoryConfigResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_set_regulatory_config (self , ctx , request , response ) 
    }
    fn handle_commissioning_complete < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , response : CommissioningCompleteResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_commissioning_complete (self , ctx , response ) 
    }
    fn handle_set_tc_acknowledgements < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTCAcknowledgementsRequest < '_ > , response : SetTCAcknowledgementsResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_set_tc_acknowledgements (self , ctx , request , response ) 
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
                    AttributeId :: Breadcrumb => {
                        let attr_read_result = self . 0. breadcrumb (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Breadcrumb , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BasicCommissioningInfo => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BasicCommissioningInfo , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. basic_commissioning_info (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BasicCommissioningInfo , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BasicCommissioningInfo , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RegulatoryConfig => {
                        let attr_read_result = self . 0. regulatory_config (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RegulatoryConfig , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocationCapability => {
                        let attr_read_result = self . 0. location_capability (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocationCapability , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SupportsConcurrentConnection => {
                        let attr_read_result = self . 0. supports_concurrent_connection (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportsConcurrentConnection , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCAcceptedVersion => {
                        let attr_read_result = self . 0. tc_accepted_version (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCAcceptedVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCMinRequiredVersion => {
                        let attr_read_result = self . 0. tc_min_required_version (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCMinRequiredVersion , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCAcknowledgements => {
                        let attr_read_result = self . 0. tc_acknowledgements (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCAcknowledgements , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCAcknowledgementsRequired => {
                        let attr_read_result = self . 0. tc_acknowledgements_required (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCAcknowledgementsRequired , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TCUpdateDeadline => {
                        let attr_read_result = self . 0. tc_update_deadline (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TCUpdateDeadline , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RecoveryIdentifier => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RecoveryIdentifier , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. recovery_identifier (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RecoveryIdentifier , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RecoveryIdentifier , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: NetworkRecoveryReason => {
                        let attr_read_result = self . 0. network_recovery_reason (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NetworkRecoveryReason , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: IsCommissioningWithoutPower => {
                        let attr_read_result = self . 0. is_commissioning_without_power (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: IsCommissioningWithoutPower , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: Breadcrumb => {
                let attr_data : u64 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_breadcrumb (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Breadcrumb , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: ArmFailSafe => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ArmFailSafe ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_arm_fail_safe (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ArmFailSafe ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ArmFailSafe ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: SetRegulatoryConfig => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetRegulatoryConfig ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (3u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_set_regulatory_config (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetRegulatoryConfig ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetRegulatoryConfig ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: CommissioningComplete => {
                crate :: dm :: log_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CommissioningComplete ) ) ) ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_commissioning_complete (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CommissioningComplete ) ) ) , tw ) , tag , ) ? , ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CommissioningComplete ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: SetTCAcknowledgements => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTCAcknowledgements ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (7u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_set_tc_acknowledgements (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTCAcknowledgements ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTCAcknowledgements ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 48u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralCommissioning" , 48u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "GeneralCommissioning" , 48u32 , self . 0.2 ) 
    }
    
}
