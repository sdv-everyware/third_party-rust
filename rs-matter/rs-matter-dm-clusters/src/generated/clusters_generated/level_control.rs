# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const ON_OFF = 1 ;
        const LIGHTING = 2 ;
        const FREQUENCY = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const ON_OFF = 1 ;
        const LIGHTING = 2 ;
        const FREQUENCY = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct OptionsBitmap : u8 {
        const EXECUTE_IF_OFF = 1 ;
        const COUPLE_COLOR_TEMP_TO_LEVEL = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct OptionsBitmap : u8 {
        const EXECUTE_IF_OFF = 1 ;
        const COUPLE_COLOR_TEMP_TO_LEVEL = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (OptionsBitmap , u8 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveModeEnum {
    # [enumval (0 ) ] Up = 0 , # [enumval (1 ) ] Down = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepModeEnum {
    # [enumval (0 ) ] Up = 0 , # [enumval (1 ) ] Down = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToLevelRequestTag {
    Level = 0 , TransitionTime = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveRequestTag {
    MoveMode = 0 , Rate = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepRequestTag {
    StepMode = 0 , StepSize = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StopRequestTag {
    OptionsMask = 0 , OptionsOverride = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToLevelWithOnOffRequestTag {
    Level = 0 , TransitionTime = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveWithOnOffRequestTag {
    MoveMode = 0 , Rate = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepWithOnOffRequestTag {
    StepMode = 0 , StepSize = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StopWithOnOffRequestTag {
    OptionsMask = 0 , OptionsOverride = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToClosestFrequencyRequestTag {
    Frequency = 0 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToLevelRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToLevelRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn level (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transition_time (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToLevelRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToLevelRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToLevelRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToLevelRequest" ) ? ;
        match self . level () {
            Ok (value ) => write ! (f , "{}: {:?}," , "level" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "level" , e . code () ) ? , 
        }
        match self . transition_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transition_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) ? , 
        }
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToLevelRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToLevelRequest" ) ;
        match self . level () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "level" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "level" , e . code () ) , 
        }
        match self . transition_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transition_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) , 
        }
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn move_mode (& self ) -> Result < MoveModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rate (& self ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveRequest" ) ? ;
        match self . move_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "move_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "move_mode" , e . code () ) ? , 
        }
        match self . rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rate" , e . code () ) ? , 
        }
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveRequest" ) ;
        match self . move_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "move_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "move_mode" , e . code () ) , 
        }
        match self . rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rate" , e . code () ) , 
        }
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StepRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StepRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn step_mode (& self ) -> Result < StepModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn step_size (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StepRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StepRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StepRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StepRequest" ) ? ;
        match self . step_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_mode" , e . code () ) ? , 
        }
        match self . step_size () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_size" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_size" , e . code () ) ? , 
        }
        match self . transition_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transition_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) ? , 
        }
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StepRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StepRequest" ) ;
        match self . step_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_mode" , e . code () ) , 
        }
        match self . step_size () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_size" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_size" , e . code () ) , 
        }
        match self . transition_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transition_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) , 
        }
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StopRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StopRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StopRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StopRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StopRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StopRequest" ) ? ;
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StopRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StopRequest" ) ;
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToLevelWithOnOffRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToLevelWithOnOffRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn level (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transition_time (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToLevelWithOnOffRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToLevelWithOnOffRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToLevelWithOnOffRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToLevelWithOnOffRequest" ) ? ;
        match self . level () {
            Ok (value ) => write ! (f , "{}: {:?}," , "level" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "level" , e . code () ) ? , 
        }
        match self . transition_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transition_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) ? , 
        }
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToLevelWithOnOffRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToLevelWithOnOffRequest" ) ;
        match self . level () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "level" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "level" , e . code () ) , 
        }
        match self . transition_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transition_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) , 
        }
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveWithOnOffRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveWithOnOffRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn move_mode (& self ) -> Result < MoveModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rate (& self ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveWithOnOffRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveWithOnOffRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveWithOnOffRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveWithOnOffRequest" ) ? ;
        match self . move_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "move_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "move_mode" , e . code () ) ? , 
        }
        match self . rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rate" , e . code () ) ? , 
        }
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveWithOnOffRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveWithOnOffRequest" ) ;
        match self . move_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "move_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "move_mode" , e . code () ) , 
        }
        match self . rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rate" , e . code () ) , 
        }
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StepWithOnOffRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StepWithOnOffRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn step_mode (& self ) -> Result < StepModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn step_size (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StepWithOnOffRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StepWithOnOffRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StepWithOnOffRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StepWithOnOffRequest" ) ? ;
        match self . step_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_mode" , e . code () ) ? , 
        }
        match self . step_size () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_size" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_size" , e . code () ) ? , 
        }
        match self . transition_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transition_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) ? , 
        }
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StepWithOnOffRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StepWithOnOffRequest" ) ;
        match self . step_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_mode" , e . code () ) , 
        }
        match self . step_size () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_size" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_size" , e . code () ) , 
        }
        match self . transition_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transition_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) , 
        }
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StopWithOnOffRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StopWithOnOffRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StopWithOnOffRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StopWithOnOffRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StopWithOnOffRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StopWithOnOffRequest" ) ? ;
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StopWithOnOffRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StopWithOnOffRequest" ) ;
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToClosestFrequencyRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToClosestFrequencyRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn frequency (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToClosestFrequencyRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToClosestFrequencyRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToClosestFrequencyRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToClosestFrequencyRequest" ) ? ;
        match self . frequency () {
            Ok (value ) => write ! (f , "{}: {:?}," , "frequency" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "frequency" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToClosestFrequencyRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToClosestFrequencyRequest" ) ;
        match self . frequency () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "frequency" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "frequency" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct MoveToLevelRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToLevelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToLevelRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn level (mut self , value : u8 ) -> Result < MoveToLevelRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "level" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToLevelRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < MoveToLevelRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveToLevelRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveToLevelRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveToLevelRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveToLevelRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveToLevelRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToLevelRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToLevelRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToLevelRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToLevelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToLevelRequestArrayBuilder < P > (P ) ;
impl < P > MoveToLevelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToLevelRequestBuilder < MoveToLevelRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToLevelRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToLevelRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToLevelRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToLevelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToLevelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_mode (mut self , value : MoveModeEnum ) -> Result < MoveRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "moveMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate (mut self , value : crate :: tlv :: Nullable < u8 > ) -> Result < MoveRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveRequestArrayBuilder < P > (P ) ;
impl < P > MoveRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveRequestBuilder < MoveRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StepRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_mode (mut self , value : StepModeEnum ) -> Result < StepRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_size (mut self , value : u8 ) -> Result < StepRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepSize" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < StepRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StepRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StepRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StepRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StepRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StepRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepRequestArrayBuilder < P > (P ) ;
impl < P > StepRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StepRequestBuilder < StepRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StepRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StepRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StepRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StepRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StopRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StopRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StopRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StopRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StopRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StopRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StopRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StopRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StopRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StopRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StopRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StopRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StopRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StopRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StopRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StopRequestArrayBuilder < P > (P ) ;
impl < P > StopRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StopRequestBuilder < StopRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StopRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StopRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StopRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StopRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StopRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StopRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StopRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToLevelWithOnOffRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToLevelWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToLevelWithOnOffRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn level (mut self , value : u8 ) -> Result < MoveToLevelWithOnOffRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "level" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToLevelWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelWithOnOffRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < MoveToLevelWithOnOffRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveToLevelWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelWithOnOffRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveToLevelWithOnOffRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveToLevelWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelWithOnOffRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveToLevelWithOnOffRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveToLevelWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToLevelWithOnOffRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToLevelWithOnOffRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelWithOnOffRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToLevelWithOnOffRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelWithOnOffRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToLevelWithOnOffRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToLevelWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToLevelWithOnOffRequestArrayBuilder < P > (P ) ;
impl < P > MoveToLevelWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToLevelWithOnOffRequestBuilder < MoveToLevelWithOnOffRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToLevelWithOnOffRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToLevelWithOnOffRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelWithOnOffRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToLevelWithOnOffRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToLevelWithOnOffRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToLevelWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToLevelWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveWithOnOffRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveWithOnOffRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_mode (mut self , value : MoveModeEnum ) -> Result < MoveWithOnOffRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "moveMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveWithOnOffRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate (mut self , value : crate :: tlv :: Nullable < u8 > ) -> Result < MoveWithOnOffRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveWithOnOffRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveWithOnOffRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveWithOnOffRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveWithOnOffRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveWithOnOffRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveWithOnOffRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveWithOnOffRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveWithOnOffRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveWithOnOffRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveWithOnOffRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveWithOnOffRequestArrayBuilder < P > (P ) ;
impl < P > MoveWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveWithOnOffRequestBuilder < MoveWithOnOffRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveWithOnOffRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveWithOnOffRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveWithOnOffRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveWithOnOffRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveWithOnOffRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepWithOnOffRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StepWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StepWithOnOffRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_mode (mut self , value : StepModeEnum ) -> Result < StepWithOnOffRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StepWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepWithOnOffRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_size (mut self , value : u8 ) -> Result < StepWithOnOffRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepSize" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StepWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepWithOnOffRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < StepWithOnOffRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StepWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepWithOnOffRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StepWithOnOffRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (StepWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepWithOnOffRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StepWithOnOffRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (StepWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepWithOnOffRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StepWithOnOffRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepWithOnOffRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StepWithOnOffRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepWithOnOffRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StepWithOnOffRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepWithOnOffRequestArrayBuilder < P > (P ) ;
impl < P > StepWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StepWithOnOffRequestBuilder < StepWithOnOffRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StepWithOnOffRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StepWithOnOffRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepWithOnOffRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StepWithOnOffRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepWithOnOffRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StepWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StopWithOnOffRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StopWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StopWithOnOffRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StopWithOnOffRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StopWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StopWithOnOffRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StopWithOnOffRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StopWithOnOffRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StopWithOnOffRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StopWithOnOffRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StopWithOnOffRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StopWithOnOffRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StopWithOnOffRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StopWithOnOffRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StopWithOnOffRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StopWithOnOffRequestArrayBuilder < P > (P ) ;
impl < P > StopWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StopWithOnOffRequestBuilder < StopWithOnOffRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StopWithOnOffRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StopWithOnOffRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StopWithOnOffRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StopWithOnOffRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StopWithOnOffRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StopWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StopWithOnOffRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToClosestFrequencyRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToClosestFrequencyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToClosestFrequencyRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn frequency (mut self , value : u16 ) -> Result < MoveToClosestFrequencyRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "frequency" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToClosestFrequencyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToClosestFrequencyRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToClosestFrequencyRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToClosestFrequencyRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToClosestFrequencyRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToClosestFrequencyRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToClosestFrequencyRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToClosestFrequencyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToClosestFrequencyRequestArrayBuilder < P > (P ) ;
impl < P > MoveToClosestFrequencyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToClosestFrequencyRequestBuilder < MoveToClosestFrequencyRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToClosestFrequencyRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToClosestFrequencyRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToClosestFrequencyRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToClosestFrequencyRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToClosestFrequencyRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToClosestFrequencyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToClosestFrequencyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    CurrentLevel = 0 , RemainingTime = 1 , MinLevel = 2 , MaxLevel = 3 , CurrentFrequency = 4 , MinFrequency = 5 , MaxFrequency = 6 , Options = 15 , OnOffTransitionTime = 16 , OnLevel = 17 , OnTransitionTime = 18 , OffTransitionTime = 19 , DefaultMoveRate = 20 , StartUpCurrentLevel = 16384 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: CurrentLevel => write ! (f , "{}(0x{:02x})" , "CurrentLevel" , AttributeId :: CurrentLevel as u32 ) ? , AttributeId :: RemainingTime => write ! (f , "{}(0x{:02x})" , "RemainingTime" , AttributeId :: RemainingTime as u32 ) ? , AttributeId :: MinLevel => write ! (f , "{}(0x{:02x})" , "MinLevel" , AttributeId :: MinLevel as u32 ) ? , AttributeId :: MaxLevel => write ! (f , "{}(0x{:02x})" , "MaxLevel" , AttributeId :: MaxLevel as u32 ) ? , AttributeId :: CurrentFrequency => write ! (f , "{}(0x{:02x})" , "CurrentFrequency" , AttributeId :: CurrentFrequency as u32 ) ? , AttributeId :: MinFrequency => write ! (f , "{}(0x{:02x})" , "MinFrequency" , AttributeId :: MinFrequency as u32 ) ? , AttributeId :: MaxFrequency => write ! (f , "{}(0x{:02x})" , "MaxFrequency" , AttributeId :: MaxFrequency as u32 ) ? , AttributeId :: Options => write ! (f , "{}(0x{:02x})" , "Options" , AttributeId :: Options as u32 ) ? , AttributeId :: OnOffTransitionTime => write ! (f , "{}(0x{:02x})" , "OnOffTransitionTime" , AttributeId :: OnOffTransitionTime as u32 ) ? , AttributeId :: OnLevel => write ! (f , "{}(0x{:02x})" , "OnLevel" , AttributeId :: OnLevel as u32 ) ? , AttributeId :: OnTransitionTime => write ! (f , "{}(0x{:02x})" , "OnTransitionTime" , AttributeId :: OnTransitionTime as u32 ) ? , AttributeId :: OffTransitionTime => write ! (f , "{}(0x{:02x})" , "OffTransitionTime" , AttributeId :: OffTransitionTime as u32 ) ? , AttributeId :: DefaultMoveRate => write ! (f , "{}(0x{:02x})" , "DefaultMoveRate" , AttributeId :: DefaultMoveRate as u32 ) ? , AttributeId :: StartUpCurrentLevel => write ! (f , "{}(0x{:02x})" , "StartUpCurrentLevel" , AttributeId :: StartUpCurrentLevel as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: CurrentLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentLevel" , AttributeId :: CurrentLevel as u32 ) , AttributeId :: RemainingTime => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RemainingTime" , AttributeId :: RemainingTime as u32 ) , AttributeId :: MinLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MinLevel" , AttributeId :: MinLevel as u32 ) , AttributeId :: MaxLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxLevel" , AttributeId :: MaxLevel as u32 ) , AttributeId :: CurrentFrequency => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentFrequency" , AttributeId :: CurrentFrequency as u32 ) , AttributeId :: MinFrequency => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MinFrequency" , AttributeId :: MinFrequency as u32 ) , AttributeId :: MaxFrequency => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxFrequency" , AttributeId :: MaxFrequency as u32 ) , AttributeId :: Options => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Options" , AttributeId :: Options as u32 ) , AttributeId :: OnOffTransitionTime => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OnOffTransitionTime" , AttributeId :: OnOffTransitionTime as u32 ) , AttributeId :: OnLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OnLevel" , AttributeId :: OnLevel as u32 ) , AttributeId :: OnTransitionTime => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OnTransitionTime" , AttributeId :: OnTransitionTime as u32 ) , AttributeId :: OffTransitionTime => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "OffTransitionTime" , AttributeId :: OffTransitionTime as u32 ) , AttributeId :: DefaultMoveRate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DefaultMoveRate" , AttributeId :: DefaultMoveRate as u32 ) , AttributeId :: StartUpCurrentLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StartUpCurrentLevel" , AttributeId :: StartUpCurrentLevel as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    MoveToLevel = 0 , Move = 1 , Step = 2 , Stop = 3 , MoveToLevelWithOnOff = 4 , MoveWithOnOff = 5 , StepWithOnOff = 6 , StopWithOnOff = 7 , MoveToClosestFrequency = 8 
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
            CommandId :: MoveToLevel => write ! (f , "{}(0x{:02x})" , "MoveToLevel" , CommandId :: MoveToLevel as u32 ) ? , CommandId :: Move => write ! (f , "{}(0x{:02x})" , "Move" , CommandId :: Move as u32 ) ? , CommandId :: Step => write ! (f , "{}(0x{:02x})" , "Step" , CommandId :: Step as u32 ) ? , CommandId :: Stop => write ! (f , "{}(0x{:02x})" , "Stop" , CommandId :: Stop as u32 ) ? , CommandId :: MoveToLevelWithOnOff => write ! (f , "{}(0x{:02x})" , "MoveToLevelWithOnOff" , CommandId :: MoveToLevelWithOnOff as u32 ) ? , CommandId :: MoveWithOnOff => write ! (f , "{}(0x{:02x})" , "MoveWithOnOff" , CommandId :: MoveWithOnOff as u32 ) ? , CommandId :: StepWithOnOff => write ! (f , "{}(0x{:02x})" , "StepWithOnOff" , CommandId :: StepWithOnOff as u32 ) ? , CommandId :: StopWithOnOff => write ! (f , "{}(0x{:02x})" , "StopWithOnOff" , CommandId :: StopWithOnOff as u32 ) ? , CommandId :: MoveToClosestFrequency => write ! (f , "{}(0x{:02x})" , "MoveToClosestFrequency" , CommandId :: MoveToClosestFrequency as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: MoveToLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToLevel" , CommandId :: MoveToLevel as u32 ) , CommandId :: Move => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Move" , CommandId :: Move as u32 ) , CommandId :: Step => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Step" , CommandId :: Step as u32 ) , CommandId :: Stop => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Stop" , CommandId :: Stop as u32 ) , CommandId :: MoveToLevelWithOnOff => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToLevelWithOnOff" , CommandId :: MoveToLevelWithOnOff as u32 ) , CommandId :: MoveWithOnOff => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveWithOnOff" , CommandId :: MoveWithOnOff as u32 ) , CommandId :: StepWithOnOff => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StepWithOnOff" , CommandId :: StepWithOnOff as u32 ) , CommandId :: StopWithOnOff => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StopWithOnOff" , CommandId :: StopWithOnOff as u32 ) , CommandId :: MoveToClosestFrequency => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToClosestFrequency" , CommandId :: MoveToClosestFrequency as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (8 , 6 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: CurrentLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: RemainingTime as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MinLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MaxLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentFrequency as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MinFrequency as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MaxFrequency as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Options as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: OnOffTransitionTime as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: OnLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: OnTransitionTime as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: OffTransitionTime as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: DefaultMoveRate as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: StartUpCurrentLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: MoveToLevel as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: Move as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: Step as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: Stop as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveToLevelWithOnOff as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveWithOnOff as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: StepWithOnOff as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: StopWithOnOff as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveToClosestFrequency as _ , None , crate :: dm :: Access :: WO , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > ;
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn current_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn min_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < OptionsBitmap , crate :: error :: Error > ;
    fn on_off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn on_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > ;
    fn on_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn default_move_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn start_up_current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> Result < () , crate :: error :: Error > ;
    fn set_on_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_on_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > ;
    fn set_on_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_default_move_rate (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_start_up_current_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_move_to_level (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_stop (& self , ctx : impl crate :: dm :: InvokeContext , request : StopRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_to_level_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_step_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StepWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_stop_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StopWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_to_closest_frequency (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToClosestFrequencyRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: current_level (self , ctx ) 
    }
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: remaining_time (self , ctx ) 
    }
    fn min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: min_level (self , ctx ) 
    }
    fn max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: max_level (self , ctx ) 
    }
    fn current_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: current_frequency (self , ctx ) 
    }
    fn min_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: min_frequency (self , ctx ) 
    }
    fn max_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: max_frequency (self , ctx ) 
    }
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < OptionsBitmap , crate :: error :: Error > {
        T :: options (self , ctx ) 
    }
    fn on_off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: on_off_transition_time (self , ctx ) 
    }
    fn on_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: on_level (self , ctx ) 
    }
    fn on_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: on_transition_time (self , ctx ) 
    }
    fn off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: off_transition_time (self , ctx ) 
    }
    fn default_move_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: default_move_rate (self , ctx ) 
    }
    fn start_up_current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: start_up_current_level (self , ctx ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_options (self , ctx , value ) 
    }
    fn set_on_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        T :: set_on_off_transition_time (self , ctx , value ) 
    }
    fn set_on_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        T :: set_on_level (self , ctx , value ) 
    }
    fn set_on_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        T :: set_on_transition_time (self , ctx , value ) 
    }
    fn set_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        T :: set_off_transition_time (self , ctx , value ) 
    }
    fn set_default_move_rate (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        T :: set_default_move_rate (self , ctx , value ) 
    }
    fn set_start_up_current_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        T :: set_start_up_current_level (self , ctx , value ) 
    }
    fn handle_move_to_level (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_level (self , ctx , request ) 
    }
    fn handle_move (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move (self , ctx , request ) 
    }
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_step (self , ctx , request ) 
    }
    fn handle_stop (& self , ctx : impl crate :: dm :: InvokeContext , request : StopRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_stop (self , ctx , request ) 
    }
    fn handle_move_to_level_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_level_with_on_off (self , ctx , request ) 
    }
    fn handle_move_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_with_on_off (self , ctx , request ) 
    }
    fn handle_step_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StepWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_step_with_on_off (self , ctx , request ) 
    }
    fn handle_stop_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StopWithOnOffRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_stop_with_on_off (self , ctx , request ) 
    }
    fn handle_move_to_closest_frequency (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToClosestFrequencyRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_closest_frequency (self , ctx , request ) 
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
                    AttributeId :: CurrentLevel => {
                        let attr_read_result = self . 0. current_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RemainingTime => {
                        let attr_read_result = self . 0. remaining_time (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemainingTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinLevel => {
                        let attr_read_result = self . 0. min_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxLevel => {
                        let attr_read_result = self . 0. max_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentFrequency => {
                        let attr_read_result = self . 0. current_frequency (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinFrequency => {
                        let attr_read_result = self . 0. min_frequency (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxFrequency => {
                        let attr_read_result = self . 0. max_frequency (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Options => {
                        let attr_read_result = self . 0. options (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Options , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OnOffTransitionTime => {
                        let attr_read_result = self . 0. on_off_transition_time (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnOffTransitionTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OnLevel => {
                        let attr_read_result = self . 0. on_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OnTransitionTime => {
                        let attr_read_result = self . 0. on_transition_time (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnTransitionTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OffTransitionTime => {
                        let attr_read_result = self . 0. off_transition_time (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OffTransitionTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DefaultMoveRate => {
                        let attr_read_result = self . 0. default_move_rate (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultMoveRate , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StartUpCurrentLevel => {
                        let attr_read_result = self . 0. start_up_current_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpCurrentLevel , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: Options => {
                let attr_data : OptionsBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_options (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Options , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OnOffTransitionTime => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_on_off_transition_time (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnOffTransitionTime , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OnLevel => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_on_level (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OnTransitionTime => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_on_transition_time (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnTransitionTime , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OffTransitionTime => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_off_transition_time (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OffTransitionTime , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: DefaultMoveRate => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_default_move_rate (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultMoveRate , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: StartUpCurrentLevel => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_start_up_current_level (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpCurrentLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: MoveToLevel => {
                let cmd_data : MoveToLevelRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_level (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToLevel ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: Move => {
                let cmd_data : MoveRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Move ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: Step => {
                let cmd_data : StepRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Step ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: Stop => {
                let cmd_data : StopRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_stop (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Stop ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToLevelWithOnOff => {
                let cmd_data : MoveToLevelWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_level_with_on_off (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToLevelWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveWithOnOff => {
                let cmd_data : MoveWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_with_on_off (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepWithOnOff => {
                let cmd_data : StepWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_with_on_off (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StopWithOnOff => {
                let cmd_data : StopWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_stop_with_on_off (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StopWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToClosestFrequency => {
                let cmd_data : MoveToClosestFrequencyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_closest_frequency (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToClosestFrequency ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 8u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "LevelControl" , 8u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "LevelControl" , 8u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `LevelControl` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct LevelControlCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > LevelControlCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_to_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToLevelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 0 ) ? . data_builder () 
    }
    pub fn r#move (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 1 ) ? . data_builder () 
    }
    pub fn step (self , endpoint : crate :: dm :: EndptId , ) -> Result < StepRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 2 ) ? . data_builder () 
    }
    pub fn stop (self , endpoint : crate :: dm :: EndptId , ) -> Result < StopRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 3 ) ? . data_builder () 
    }
    pub fn move_to_level_with_on_off (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToLevelWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 4 ) ? . data_builder () 
    }
    pub fn move_with_on_off (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 5 ) ? . data_builder () 
    }
    pub fn step_with_on_off (self , endpoint : crate :: dm :: EndptId , ) -> Result < StepWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 6 ) ? . data_builder () 
    }
    pub fn stop_with_on_off (self , endpoint : crate :: dm :: EndptId , ) -> Result < StopWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 7 ) ? . data_builder () 
    }
    pub fn move_to_closest_frequency (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToClosestFrequencyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 8 , 8 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `LevelControl` cluster's commands. `use` this trait to call `.level_control_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`LevelControlCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait LevelControlCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn level_control_inv (self ) -> LevelControlCmdRequestsView < P > ;
    
}
impl < P > LevelControlCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn level_control_inv (self ) -> LevelControlCmdRequestsView < P > {
        LevelControlCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `LevelControl` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct LevelControlAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > LevelControlAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn remaining_time (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_frequency (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_frequency (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_frequency (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn options (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (15 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn on_off_transition_time (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (16 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn on_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (17 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn on_transition_time (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (18 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn off_transition_time (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (19 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn default_move_rate (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (20 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn start_up_current_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (16384 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (8 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `LevelControl` cluster's attribute reads. `use` this trait to call `.level_control_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`LevelControlAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait LevelControlAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn level_control_read (self ) -> LevelControlAttrReadsView < P > ;
    
}
impl < P > LevelControlAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn level_control_read (self ) -> LevelControlAttrReadsView < P > {
        LevelControlAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `LevelControl` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct LevelControlAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > LevelControlAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options (self , endpoint : crate :: dm :: EndptId , value : OptionsBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 8 , 15 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn on_off_transition_time (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 8 , 16 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn on_level (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 8 , 17 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn on_transition_time (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 8 , 18 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn off_transition_time (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 8 , 19 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn default_move_rate (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 8 , 20 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn start_up_current_level (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 8 , 16384 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `LevelControl` cluster's attribute writes. `use` this trait to call `.level_control_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`LevelControlAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait LevelControlAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn level_control_write (self ) -> LevelControlAttrWritesView < P > ;
    
}
impl < P > LevelControlAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn level_control_write (self ) -> LevelControlAttrWritesView < P > {
        LevelControlAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `LevelControl` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct LevelControlCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > LevelControlCmdResponsesView < 'a , 'r > {
    pub fn move_to_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 0 ) 
    }
    pub fn r#move (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 1 ) 
    }
    pub fn step (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 2 ) 
    }
    pub fn stop (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 3 ) 
    }
    pub fn move_to_level_with_on_off (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 4 ) 
    }
    pub fn move_with_on_off (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 5 ) 
    }
    pub fn step_with_on_off (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 6 ) 
    }
    pub fn stop_with_on_off (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 7 ) 
    }
    pub fn move_to_closest_frequency (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 8 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `LevelControl`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.level_control_inv_resp()` on an `InvokeResp`; the returned [`LevelControlCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait LevelControlCmdResponses < 'a > {
    fn level_control_inv_resp (& self ) -> LevelControlCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > LevelControlCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn level_control_inv_resp (& self ) -> LevelControlCmdResponsesView < 'a , '_ > {
        LevelControlCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `LevelControl` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct LevelControlAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > LevelControlAttrResponsesView < 'a , 'r > {
    pub fn current_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (8 , 0 ) 
    }
    pub fn remaining_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (8 , 1 ) 
    }
    pub fn min_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (8 , 2 ) 
    }
    pub fn max_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (8 , 3 ) 
    }
    pub fn current_frequency (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (8 , 4 ) 
    }
    pub fn min_frequency (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (8 , 5 ) 
    }
    pub fn max_frequency (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (8 , 6 ) 
    }
    pub fn options (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < OptionsBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < OptionsBitmap > (8 , 15 ) 
    }
    pub fn on_off_transition_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (8 , 16 ) 
    }
    pub fn on_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (8 , 17 ) 
    }
    pub fn on_transition_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (8 , 18 ) 
    }
    pub fn off_transition_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (8 , 19 ) 
    }
    pub fn default_move_rate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (8 , 20 ) 
    }
    pub fn start_up_current_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (8 , 16384 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (8 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (8 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (8 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (8 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (8 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `LevelControl`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.level_control_read_resp()` on a `ReportDataResp`; the returned [`LevelControlAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait LevelControlAttrResponses < 'a > {
    fn level_control_read_resp (& self ) -> LevelControlAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > LevelControlAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn level_control_read_resp (& self ) -> LevelControlAttrResponsesView < 'a , '_ > {
        LevelControlAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `LevelControl` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct LevelControlWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > LevelControlWriteResponsesView < 'a , 'r > {
    pub fn options (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 15 ) 
    }
    pub fn on_off_transition_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 16 ) 
    }
    pub fn on_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 17 ) 
    }
    pub fn on_transition_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 18 ) 
    }
    pub fn off_transition_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 19 ) 
    }
    pub fn default_move_rate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 20 ) 
    }
    pub fn start_up_current_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (8 , 16384 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `LevelControl`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.level_control_write_resp()` on a `WriteResp`; the returned [`LevelControlWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait LevelControlWriteResponses < 'a > {
    fn level_control_write_resp (& self ) -> LevelControlWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > LevelControlWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn level_control_write_resp (& self ) -> LevelControlWriteResponsesView < 'a , '_ > {
        LevelControlWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `LevelControl` cluster's client operations. Returned by [`LevelControlClient::level_control`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct LevelControlClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > LevelControlClientView < T > {
    pub fn move_to_level < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToLevelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . move_to_level (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn r#move < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . r#move (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn step < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StepRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . step (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn stop < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StopRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . stop (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_to_level_with_on_off < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToLevelWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . move_to_level_with_on_off (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_with_on_off < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . move_with_on_off (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn step_with_on_off < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StepWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . step_with_on_off (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn stop_with_on_off < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StopWithOnOffRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . stop_with_on_off (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_to_closest_frequency < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToClosestFrequencyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: LevelControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . level_control_inv () ;
            let req_builder = view . move_to_closest_frequency (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn current_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . current_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn remaining_time_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . remaining_time (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn min_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . min_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . max_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn current_frequency_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . current_frequency (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn min_frequency_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . min_frequency (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_frequency_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . max_frequency (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn options_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < OptionsBitmap , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . options (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn on_off_transition_time_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . on_off_transition_time (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn on_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . on_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn on_transition_time_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . on_transition_time (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn off_transition_time_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . off_transition_time (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn default_move_rate_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . default_move_rate (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn start_up_current_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . start_up_current_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: LevelControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . level_control_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn options_write (self , endpoint : crate :: dm :: EndptId , value : OptionsBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: LevelControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . level_control_write () . options (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn on_off_transition_time_write (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: LevelControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . level_control_write () . on_off_transition_time (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn on_level_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: LevelControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . level_control_write () . on_level (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn on_transition_time_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: LevelControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . level_control_write () . on_transition_time (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn off_transition_time_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: LevelControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . level_control_write () . off_transition_time (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn default_move_rate_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: LevelControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . level_control_write () . default_move_rate (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn start_up_current_level_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: LevelControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . level_control_write () . start_up_current_level (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `LevelControl` cluster. `use` this trait to call `.level_control()` on any [`crate::im::ImClient`], such as an exchange; the returned [`LevelControlClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `LevelControlClientView` keeps each cluster's API surface narrow: at the call site `exchange.level_control().` shows only this cluster's methods in IDE completion." ] pub trait LevelControlClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn level_control (self ) -> LevelControlClientView < Self > {
        LevelControlClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > LevelControlClient for T {
    
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
    fn current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn current_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn min_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < OptionsBitmap , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn on_off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn on_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn on_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn default_move_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn start_up_current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_on_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_on_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_on_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_default_move_rate (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_start_up_current_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_move_to_level (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_stop (& self , ctx : impl crate :: dm :: InvokeContext , request : StopRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_to_level_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_step_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StepWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_stop_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StopWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_to_closest_frequency (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToClosestFrequencyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: current_level (self , ctx ) 
    }
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: remaining_time (self , ctx ) 
    }
    fn min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: min_level (self , ctx ) 
    }
    fn max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: max_level (self , ctx ) 
    }
    fn current_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: current_frequency (self , ctx ) 
    }
    fn min_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: min_frequency (self , ctx ) 
    }
    fn max_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: max_frequency (self , ctx ) 
    }
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < OptionsBitmap , crate :: error :: Error >> {
        T :: options (self , ctx ) 
    }
    fn on_off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: on_off_transition_time (self , ctx ) 
    }
    fn on_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: on_level (self , ctx ) 
    }
    fn on_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: on_transition_time (self , ctx ) 
    }
    fn off_transition_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: off_transition_time (self , ctx ) 
    }
    fn default_move_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: default_move_rate (self , ctx ) 
    }
    fn start_up_current_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: start_up_current_level (self , ctx ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_options (self , ctx , value ) 
    }
    fn set_on_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_on_off_transition_time (self , ctx , value ) 
    }
    fn set_on_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_on_level (self , ctx , value ) 
    }
    fn set_on_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_on_transition_time (self , ctx , value ) 
    }
    fn set_off_transition_time (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_off_transition_time (self , ctx , value ) 
    }
    fn set_default_move_rate (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_default_move_rate (self , ctx , value ) 
    }
    fn set_start_up_current_level (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_start_up_current_level (self , ctx , value ) 
    }
    fn handle_move_to_level (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_level (self , ctx , request ) 
    }
    fn handle_move (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move (self , ctx , request ) 
    }
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_step (self , ctx , request ) 
    }
    fn handle_stop (& self , ctx : impl crate :: dm :: InvokeContext , request : StopRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_stop (self , ctx , request ) 
    }
    fn handle_move_to_level_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToLevelWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_level_with_on_off (self , ctx , request ) 
    }
    fn handle_move_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_with_on_off (self , ctx , request ) 
    }
    fn handle_step_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StepWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_step_with_on_off (self , ctx , request ) 
    }
    fn handle_stop_with_on_off (& self , ctx : impl crate :: dm :: InvokeContext , request : StopWithOnOffRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_stop_with_on_off (self , ctx , request ) 
    }
    fn handle_move_to_closest_frequency (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToClosestFrequencyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_closest_frequency (self , ctx , request ) 
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
                    AttributeId :: CurrentLevel => {
                        let attr_read_result = self . 0. current_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RemainingTime => {
                        let attr_read_result = self . 0. remaining_time (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemainingTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinLevel => {
                        let attr_read_result = self . 0. min_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxLevel => {
                        let attr_read_result = self . 0. max_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentFrequency => {
                        let attr_read_result = self . 0. current_frequency (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinFrequency => {
                        let attr_read_result = self . 0. min_frequency (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxFrequency => {
                        let attr_read_result = self . 0. max_frequency (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Options => {
                        let attr_read_result = self . 0. options (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Options , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OnOffTransitionTime => {
                        let attr_read_result = self . 0. on_off_transition_time (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnOffTransitionTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OnLevel => {
                        let attr_read_result = self . 0. on_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OnTransitionTime => {
                        let attr_read_result = self . 0. on_transition_time (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnTransitionTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: OffTransitionTime => {
                        let attr_read_result = self . 0. off_transition_time (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OffTransitionTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DefaultMoveRate => {
                        let attr_read_result = self . 0. default_move_rate (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultMoveRate , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StartUpCurrentLevel => {
                        let attr_read_result = self . 0. start_up_current_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpCurrentLevel , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: Options => {
                let attr_data : OptionsBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_options (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Options , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OnOffTransitionTime => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_on_off_transition_time (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnOffTransitionTime , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OnLevel => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_on_level (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OnTransitionTime => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_on_transition_time (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OnTransitionTime , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: OffTransitionTime => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_off_transition_time (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: OffTransitionTime , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: DefaultMoveRate => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_default_move_rate (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultMoveRate , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: StartUpCurrentLevel => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_start_up_current_level (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpCurrentLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: MoveToLevel => {
                let cmd_data : MoveToLevelRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_level (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToLevel ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: Move => {
                let cmd_data : MoveRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Move ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: Step => {
                let cmd_data : StepRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Step ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: Stop => {
                let cmd_data : StopRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_stop (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Stop ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToLevelWithOnOff => {
                let cmd_data : MoveToLevelWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_level_with_on_off (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToLevelWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveWithOnOff => {
                let cmd_data : MoveWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_with_on_off (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepWithOnOff => {
                let cmd_data : StepWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_with_on_off (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StopWithOnOff => {
                let cmd_data : StopWithOnOffRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_stop_with_on_off (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StopWithOnOff ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToClosestFrequency => {
                let cmd_data : MoveToClosestFrequencyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_closest_frequency (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToClosestFrequency ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 8u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "LevelControl" , 8u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "LevelControl" , 8u32 , self . 0.2 ) 
    }
    
}
