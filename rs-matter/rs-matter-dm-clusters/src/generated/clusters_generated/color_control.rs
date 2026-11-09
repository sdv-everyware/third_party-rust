# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct ColorCapabilitiesBitmap : u16 {
        const HUE_SATURATION = 1 ;
        const ENHANCED_HUE = 2 ;
        const COLOR_LOOP = 4 ;
        const XY = 8 ;
        const COLOR_TEMPERATURE = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct ColorCapabilitiesBitmap : u16 {
        const HUE_SATURATION = 1 ;
        const ENHANCED_HUE = 2 ;
        const COLOR_LOOP = 4 ;
        const XY = 8 ;
        const COLOR_TEMPERATURE = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (ColorCapabilitiesBitmap , u16 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const HUE_AND_SATURATION = 1 ;
        const ENHANCED_HUE = 2 ;
        const COLOR_LOOP = 4 ;
        const XY = 8 ;
        const COLOR_TEMPERATURE = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const HUE_AND_SATURATION = 1 ;
        const ENHANCED_HUE = 2 ;
        const COLOR_LOOP = 4 ;
        const XY = 8 ;
        const COLOR_TEMPERATURE = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct OptionsBitmap : u8 {
        const EXECUTE_IF_OFF = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct OptionsBitmap : u8 {
        const EXECUTE_IF_OFF = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (OptionsBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct UpdateFlagsBitmap : u8 {
        const UPDATE_ACTION = 1 ;
        const UPDATE_DIRECTION = 2 ;
        const UPDATE_TIME = 4 ;
        const UPDATE_START_HUE = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct UpdateFlagsBitmap : u8 {
        const UPDATE_ACTION = 1 ;
        const UPDATE_DIRECTION = 2 ;
        const UPDATE_TIME = 4 ;
        const UPDATE_START_HUE = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (UpdateFlagsBitmap , u8 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ColorLoopActionEnum {
    # [enumval (0 ) ] Deactivate = 0 , # [enumval (1 ) ] ActivateFromColorLoopStartEnhancedHue = 1 , # [enumval (2 ) ] ActivateFromEnhancedCurrentHue = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ColorLoopDirectionEnum {
    # [enumval (0 ) ] Decrement = 0 , # [enumval (1 ) ] Increment = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ColorModeEnum {
    # [enumval (0 ) ] CurrentHueAndCurrentSaturation = 0 , # [enumval (1 ) ] CurrentXAndCurrentY = 1 , # [enumval (2 ) ] ColorTemperatureMireds = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum DirectionEnum {
    # [enumval (0 ) ] Shortest = 0 , # [enumval (1 ) ] Longest = 1 , # [enumval (2 ) ] Up = 2 , # [enumval (3 ) ] Down = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum DriftCompensationEnum {
    # [enumval (0 ) ] None = 0 , # [enumval (1 ) ] OtherOrUnknown = 1 , # [enumval (2 ) ] TemperatureMonitoring = 2 , # [enumval (3 ) ] OpticalLuminanceMonitoringAndFeedback = 3 , # [enumval (4 ) ] OpticalColorMonitoringAndFeedback = 4 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum EnhancedColorModeEnum {
    # [enumval (0 ) ] CurrentHueAndCurrentSaturation = 0 , # [enumval (1 ) ] CurrentXAndCurrentY = 1 , # [enumval (2 ) ] ColorTemperatureMireds = 2 , # [enumval (3 ) ] EnhancedCurrentHueAndCurrentSaturation = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveModeEnum {
    # [enumval (0 ) ] Stop = 0 , # [enumval (1 ) ] Up = 1 , # [enumval (3 ) ] Down = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepModeEnum {
    # [enumval (1 ) ] Up = 1 , # [enumval (3 ) ] Down = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToHueRequestTag {
    Hue = 0 , Direction = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveHueRequestTag {
    MoveMode = 0 , Rate = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepHueRequestTag {
    StepMode = 0 , StepSize = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToSaturationRequestTag {
    Saturation = 0 , TransitionTime = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveSaturationRequestTag {
    MoveMode = 0 , Rate = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepSaturationRequestTag {
    StepMode = 0 , StepSize = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToHueAndSaturationRequestTag {
    Hue = 0 , Saturation = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToColorRequestTag {
    ColorX = 0 , ColorY = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveColorRequestTag {
    RateX = 0 , RateY = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepColorRequestTag {
    StepX = 0 , StepY = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveToColorTemperatureRequestTag {
    ColorTemperatureMireds = 0 , TransitionTime = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum EnhancedMoveToHueRequestTag {
    EnhancedHue = 0 , Direction = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum EnhancedMoveHueRequestTag {
    MoveMode = 0 , Rate = 1 , OptionsMask = 2 , OptionsOverride = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum EnhancedStepHueRequestTag {
    StepMode = 0 , StepSize = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum EnhancedMoveToHueAndSaturationRequestTag {
    EnhancedHue = 0 , Saturation = 1 , TransitionTime = 2 , OptionsMask = 3 , OptionsOverride = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ColorLoopSetRequestTag {
    UpdateFlags = 0 , Action = 1 , Direction = 2 , Time = 3 , StartHue = 4 , OptionsMask = 5 , OptionsOverride = 6 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StopMoveStepRequestTag {
    OptionsMask = 0 , OptionsOverride = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MoveColorTemperatureRequestTag {
    MoveMode = 0 , Rate = 1 , ColorTemperatureMinimumMireds = 2 , ColorTemperatureMaximumMireds = 3 , OptionsMask = 4 , OptionsOverride = 5 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepColorTemperatureRequestTag {
    StepMode = 0 , StepSize = 1 , TransitionTime = 2 , ColorTemperatureMinimumMireds = 3 , ColorTemperatureMaximumMireds = 4 , OptionsMask = 5 , OptionsOverride = 6 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToHueRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToHueRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn hue (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn direction (& self ) -> Result < DirectionEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToHueRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToHueRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToHueRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToHueRequest" ) ? ;
        match self . hue () {
            Ok (value ) => write ! (f , "{}: {:?}," , "hue" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "hue" , e . code () ) ? , 
        }
        match self . direction () {
            Ok (value ) => write ! (f , "{}: {:?}," , "direction" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToHueRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToHueRequest" ) ;
        match self . hue () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "hue" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "hue" , e . code () ) , 
        }
        match self . direction () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "direction" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveHueRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveHueRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn move_mode (& self ) -> Result < MoveModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rate (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveHueRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveHueRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveHueRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveHueRequest" ) ? ;
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveHueRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveHueRequest" ) ;
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StepHueRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StepHueRequest < 'a > {
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
    pub fn transition_time (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StepHueRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StepHueRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StepHueRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StepHueRequest" ) ? ;
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StepHueRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StepHueRequest" ) ;
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToSaturationRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToSaturationRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn saturation (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToSaturationRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToSaturationRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToSaturationRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToSaturationRequest" ) ? ;
        match self . saturation () {
            Ok (value ) => write ! (f , "{}: {:?}," , "saturation" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "saturation" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToSaturationRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToSaturationRequest" ) ;
        match self . saturation () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "saturation" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "saturation" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveSaturationRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveSaturationRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn move_mode (& self ) -> Result < MoveModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rate (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveSaturationRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveSaturationRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveSaturationRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveSaturationRequest" ) ? ;
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveSaturationRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveSaturationRequest" ) ;
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StepSaturationRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StepSaturationRequest < 'a > {
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
    pub fn transition_time (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StepSaturationRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StepSaturationRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StepSaturationRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StepSaturationRequest" ) ? ;
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StepSaturationRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StepSaturationRequest" ) ;
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToHueAndSaturationRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToHueAndSaturationRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn hue (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn saturation (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToHueAndSaturationRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToHueAndSaturationRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToHueAndSaturationRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToHueAndSaturationRequest" ) ? ;
        match self . hue () {
            Ok (value ) => write ! (f , "{}: {:?}," , "hue" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "hue" , e . code () ) ? , 
        }
        match self . saturation () {
            Ok (value ) => write ! (f , "{}: {:?}," , "saturation" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "saturation" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToHueAndSaturationRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToHueAndSaturationRequest" ) ;
        match self . hue () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "hue" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "hue" , e . code () ) , 
        }
        match self . saturation () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "saturation" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "saturation" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToColorRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToColorRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn color_x (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn color_y (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToColorRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToColorRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToColorRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToColorRequest" ) ? ;
        match self . color_x () {
            Ok (value ) => write ! (f , "{}: {:?}," , "color_x" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "color_x" , e . code () ) ? , 
        }
        match self . color_y () {
            Ok (value ) => write ! (f , "{}: {:?}," , "color_y" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "color_y" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToColorRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToColorRequest" ) ;
        match self . color_x () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "color_x" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "color_x" , e . code () ) , 
        }
        match self . color_y () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "color_y" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "color_y" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveColorRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveColorRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn rate_x (& self ) -> Result < i16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rate_y (& self ) -> Result < i16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveColorRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveColorRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveColorRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveColorRequest" ) ? ;
        match self . rate_x () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rate_x" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rate_x" , e . code () ) ? , 
        }
        match self . rate_y () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rate_y" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rate_y" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveColorRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveColorRequest" ) ;
        match self . rate_x () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rate_x" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rate_x" , e . code () ) , 
        }
        match self . rate_y () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rate_y" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rate_y" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StepColorRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StepColorRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn step_x (& self ) -> Result < i16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn step_y (& self ) -> Result < i16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StepColorRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StepColorRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StepColorRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StepColorRequest" ) ? ;
        match self . step_x () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_x" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_x" , e . code () ) ? , 
        }
        match self . step_y () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_y" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_y" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StepColorRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StepColorRequest" ) ;
        match self . step_x () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_x" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_x" , e . code () ) , 
        }
        match self . step_y () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_y" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_y" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveToColorTemperatureRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveToColorTemperatureRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn color_temperature_mireds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveToColorTemperatureRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveToColorTemperatureRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveToColorTemperatureRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveToColorTemperatureRequest" ) ? ;
        match self . color_temperature_mireds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "color_temperature_mireds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "color_temperature_mireds" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveToColorTemperatureRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveToColorTemperatureRequest" ) ;
        match self . color_temperature_mireds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "color_temperature_mireds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "color_temperature_mireds" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct EnhancedMoveToHueRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > EnhancedMoveToHueRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn enhanced_hue (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn direction (& self ) -> Result < DirectionEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for EnhancedMoveToHueRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for EnhancedMoveToHueRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for EnhancedMoveToHueRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "EnhancedMoveToHueRequest" ) ? ;
        match self . enhanced_hue () {
            Ok (value ) => write ! (f , "{}: {:?}," , "enhanced_hue" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "enhanced_hue" , e . code () ) ? , 
        }
        match self . direction () {
            Ok (value ) => write ! (f , "{}: {:?}," , "direction" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for EnhancedMoveToHueRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "EnhancedMoveToHueRequest" ) ;
        match self . enhanced_hue () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "enhanced_hue" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "enhanced_hue" , e . code () ) , 
        }
        match self . direction () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "direction" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct EnhancedMoveHueRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > EnhancedMoveHueRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn move_mode (& self ) -> Result < MoveModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for EnhancedMoveHueRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for EnhancedMoveHueRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for EnhancedMoveHueRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "EnhancedMoveHueRequest" ) ? ;
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for EnhancedMoveHueRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "EnhancedMoveHueRequest" ) ;
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct EnhancedStepHueRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > EnhancedStepHueRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn step_mode (& self ) -> Result < StepModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn step_size (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for EnhancedStepHueRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for EnhancedStepHueRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for EnhancedStepHueRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "EnhancedStepHueRequest" ) ? ;
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for EnhancedStepHueRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "EnhancedStepHueRequest" ) ;
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct EnhancedMoveToHueAndSaturationRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > EnhancedMoveToHueAndSaturationRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn enhanced_hue (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn saturation (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for EnhancedMoveToHueAndSaturationRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for EnhancedMoveToHueAndSaturationRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for EnhancedMoveToHueAndSaturationRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "EnhancedMoveToHueAndSaturationRequest" ) ? ;
        match self . enhanced_hue () {
            Ok (value ) => write ! (f , "{}: {:?}," , "enhanced_hue" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "enhanced_hue" , e . code () ) ? , 
        }
        match self . saturation () {
            Ok (value ) => write ! (f , "{}: {:?}," , "saturation" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "saturation" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for EnhancedMoveToHueAndSaturationRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "EnhancedMoveToHueAndSaturationRequest" ) ;
        match self . enhanced_hue () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "enhanced_hue" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "enhanced_hue" , e . code () ) , 
        }
        match self . saturation () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "saturation" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "saturation" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ColorLoopSetRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ColorLoopSetRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn update_flags (& self ) -> Result < UpdateFlagsBitmap , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn action (& self ) -> Result < ColorLoopActionEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn direction (& self ) -> Result < ColorLoopDirectionEnum , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn start_hue (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ColorLoopSetRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ColorLoopSetRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ColorLoopSetRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ColorLoopSetRequest" ) ? ;
        match self . update_flags () {
            Ok (value ) => write ! (f , "{}: {:?}," , "update_flags" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "update_flags" , e . code () ) ? , 
        }
        match self . action () {
            Ok (value ) => write ! (f , "{}: {:?}," , "action" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "action" , e . code () ) ? , 
        }
        match self . direction () {
            Ok (value ) => write ! (f , "{}: {:?}," , "direction" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) ? , 
        }
        match self . time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "time" , e . code () ) ? , 
        }
        match self . start_hue () {
            Ok (value ) => write ! (f , "{}: {:?}," , "start_hue" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "start_hue" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ColorLoopSetRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ColorLoopSetRequest" ) ;
        match self . update_flags () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "update_flags" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "update_flags" , e . code () ) , 
        }
        match self . action () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "action" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "action" , e . code () ) , 
        }
        match self . direction () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "direction" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) , 
        }
        match self . time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "time" , e . code () ) , 
        }
        match self . start_hue () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "start_hue" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "start_hue" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StopMoveStepRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StopMoveStepRequest < 'a > {
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
impl < 'a > crate :: tlv :: FromTLV < 'a > for StopMoveStepRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StopMoveStepRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StopMoveStepRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StopMoveStepRequest" ) ? ;
        match self . options_mask () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_mask" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) ? , 
        }
        match self . options_override () {
            Ok (value ) => write ! (f , "{}: {:?}," , "options_override" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StopMoveStepRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StopMoveStepRequest" ) ;
        match self . options_mask () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_mask" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_mask" , e . code () ) , 
        }
        match self . options_override () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "options_override" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "options_override" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MoveColorTemperatureRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MoveColorTemperatureRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn move_mode (& self ) -> Result < MoveModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn color_temperature_minimum_mireds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn color_temperature_maximum_mireds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MoveColorTemperatureRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MoveColorTemperatureRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MoveColorTemperatureRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MoveColorTemperatureRequest" ) ? ;
        match self . move_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "move_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "move_mode" , e . code () ) ? , 
        }
        match self . rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "rate" , e . code () ) ? , 
        }
        match self . color_temperature_minimum_mireds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "color_temperature_minimum_mireds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "color_temperature_minimum_mireds" , e . code () ) ? , 
        }
        match self . color_temperature_maximum_mireds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "color_temperature_maximum_mireds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "color_temperature_maximum_mireds" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MoveColorTemperatureRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MoveColorTemperatureRequest" ) ;
        match self . move_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "move_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "move_mode" , e . code () ) , 
        }
        match self . rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "rate" , e . code () ) , 
        }
        match self . color_temperature_minimum_mireds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "color_temperature_minimum_mireds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "color_temperature_minimum_mireds" , e . code () ) , 
        }
        match self . color_temperature_maximum_mireds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "color_temperature_maximum_mireds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "color_temperature_maximum_mireds" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StepColorTemperatureRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StepColorTemperatureRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn step_mode (& self ) -> Result < StepModeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn step_size (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transition_time (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn color_temperature_minimum_mireds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn color_temperature_maximum_mireds (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn options_mask (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn options_override (& self ) -> Result < OptionsBitmap , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StepColorTemperatureRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StepColorTemperatureRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StepColorTemperatureRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StepColorTemperatureRequest" ) ? ;
        match self . step_mode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_mode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_mode" , e . code () ) ? , 
        }
        match self . step_size () {
            Ok (value ) => write ! (f , "{}: {:?}," , "step_size" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "step_size" , e . code () ) ? , 
        }
        match self . transition_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transition_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) ? , 
        }
        match self . color_temperature_minimum_mireds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "color_temperature_minimum_mireds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "color_temperature_minimum_mireds" , e . code () ) ? , 
        }
        match self . color_temperature_maximum_mireds () {
            Ok (value ) => write ! (f , "{}: {:?}," , "color_temperature_maximum_mireds" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "color_temperature_maximum_mireds" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StepColorTemperatureRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StepColorTemperatureRequest" ) ;
        match self . step_mode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_mode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_mode" , e . code () ) , 
        }
        match self . step_size () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "step_size" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "step_size" , e . code () ) , 
        }
        match self . transition_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transition_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transition_time" , e . code () ) , 
        }
        match self . color_temperature_minimum_mireds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "color_temperature_minimum_mireds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "color_temperature_minimum_mireds" , e . code () ) , 
        }
        match self . color_temperature_maximum_mireds () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "color_temperature_maximum_mireds" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "color_temperature_maximum_mireds" , e . code () ) , 
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
pub struct MoveToHueRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToHueRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn hue (mut self , value : u8 ) -> Result < MoveToHueRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "hue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn direction (mut self , value : DirectionEnum ) -> Result < MoveToHueRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "direction" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < MoveToHueRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveToHueRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveToHueRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (MoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToHueRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToHueRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToHueRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToHueRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToHueRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToHueRequestArrayBuilder < P > (P ) ;
impl < P > MoveToHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToHueRequestBuilder < MoveToHueRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToHueRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToHueRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToHueRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToHueRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToHueRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveHueRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveHueRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_mode (mut self , value : MoveModeEnum ) -> Result < MoveHueRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "moveMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveHueRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate (mut self , value : u8 ) -> Result < MoveHueRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveHueRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveHueRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveHueRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveHueRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveHueRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveHueRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveHueRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveHueRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveHueRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveHueRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveHueRequestArrayBuilder < P > (P ) ;
impl < P > MoveHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveHueRequestBuilder < MoveHueRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveHueRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveHueRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveHueRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveHueRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveHueRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepHueRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StepHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StepHueRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_mode (mut self , value : StepModeEnum ) -> Result < StepHueRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepHueRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_size (mut self , value : u8 ) -> Result < StepHueRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepSize" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepHueRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u8 ) -> Result < StepHueRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepHueRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StepHueRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (StepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepHueRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StepHueRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (StepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepHueRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StepHueRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepHueRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StepHueRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepHueRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StepHueRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepHueRequestArrayBuilder < P > (P ) ;
impl < P > StepHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StepHueRequestBuilder < StepHueRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StepHueRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StepHueRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepHueRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StepHueRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepHueRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StepHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToSaturationRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToSaturationRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn saturation (mut self , value : u8 ) -> Result < MoveToSaturationRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "saturation" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToSaturationRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < MoveToSaturationRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveToSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToSaturationRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveToSaturationRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveToSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToSaturationRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveToSaturationRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveToSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToSaturationRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToSaturationRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToSaturationRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToSaturationRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToSaturationRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToSaturationRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToSaturationRequestArrayBuilder < P > (P ) ;
impl < P > MoveToSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToSaturationRequestBuilder < MoveToSaturationRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToSaturationRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToSaturationRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToSaturationRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToSaturationRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToSaturationRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveSaturationRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveSaturationRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_mode (mut self , value : MoveModeEnum ) -> Result < MoveSaturationRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "moveMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveSaturationRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate (mut self , value : u8 ) -> Result < MoveSaturationRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveSaturationRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveSaturationRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveSaturationRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveSaturationRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveSaturationRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveSaturationRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveSaturationRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveSaturationRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveSaturationRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveSaturationRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveSaturationRequestArrayBuilder < P > (P ) ;
impl < P > MoveSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveSaturationRequestBuilder < MoveSaturationRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveSaturationRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveSaturationRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveSaturationRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveSaturationRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveSaturationRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepSaturationRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StepSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StepSaturationRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_mode (mut self , value : StepModeEnum ) -> Result < StepSaturationRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StepSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepSaturationRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_size (mut self , value : u8 ) -> Result < StepSaturationRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepSize" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StepSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepSaturationRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u8 ) -> Result < StepSaturationRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StepSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepSaturationRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StepSaturationRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (StepSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepSaturationRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StepSaturationRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (StepSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepSaturationRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StepSaturationRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepSaturationRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StepSaturationRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepSaturationRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StepSaturationRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepSaturationRequestArrayBuilder < P > (P ) ;
impl < P > StepSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StepSaturationRequestBuilder < StepSaturationRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StepSaturationRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StepSaturationRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepSaturationRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StepSaturationRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepSaturationRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StepSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToHueAndSaturationRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToHueAndSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToHueAndSaturationRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn hue (mut self , value : u8 ) -> Result < MoveToHueAndSaturationRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "hue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueAndSaturationRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn saturation (mut self , value : u8 ) -> Result < MoveToHueAndSaturationRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "saturation" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueAndSaturationRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < MoveToHueAndSaturationRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueAndSaturationRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveToHueAndSaturationRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueAndSaturationRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveToHueAndSaturationRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (MoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToHueAndSaturationRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToHueAndSaturationRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToHueAndSaturationRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToHueAndSaturationRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToHueAndSaturationRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToHueAndSaturationRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToHueAndSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToHueAndSaturationRequestArrayBuilder < P > (P ) ;
impl < P > MoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToHueAndSaturationRequestBuilder < MoveToHueAndSaturationRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToHueAndSaturationRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToHueAndSaturationRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToHueAndSaturationRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToHueAndSaturationRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToColorRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToColorRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToColorRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn color_x (mut self , value : u16 ) -> Result < MoveToColorRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "colorX" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn color_y (mut self , value : u16 ) -> Result < MoveToColorRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "colorY" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveToColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < MoveToColorRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveToColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveToColorRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveToColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveToColorRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (MoveToColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToColorRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToColorRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToColorRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToColorRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToColorRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToColorRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToColorRequestArrayBuilder < P > (P ) ;
impl < P > MoveToColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToColorRequestBuilder < MoveToColorRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToColorRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToColorRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToColorRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToColorRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToColorRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveColorRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveColorRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveColorRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate_x (mut self , value : i16 ) -> Result < MoveColorRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rateX" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate_y (mut self , value : i16 ) -> Result < MoveColorRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rateY" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveColorRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveColorRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveColorRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveColorRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveColorRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveColorRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveColorRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveColorRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveColorRequestArrayBuilder < P > (P ) ;
impl < P > MoveColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveColorRequestBuilder < MoveColorRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveColorRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveColorRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveColorRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveColorRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveColorRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepColorRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StepColorRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StepColorRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_x (mut self , value : i16 ) -> Result < StepColorRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepX" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StepColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_y (mut self , value : i16 ) -> Result < StepColorRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepY" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StepColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < StepColorRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StepColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StepColorRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (StepColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StepColorRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (StepColorRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StepColorRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepColorRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StepColorRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepColorRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StepColorRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepColorRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepColorRequestArrayBuilder < P > (P ) ;
impl < P > StepColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StepColorRequestBuilder < StepColorRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StepColorRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StepColorRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepColorRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StepColorRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepColorRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StepColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepColorRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToColorTemperatureRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveToColorTemperatureRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveToColorTemperatureRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn color_temperature_mireds (mut self , value : u16 ) -> Result < MoveToColorTemperatureRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "colorTemperatureMireds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveToColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorTemperatureRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < MoveToColorTemperatureRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveToColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorTemperatureRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveToColorTemperatureRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveToColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorTemperatureRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveToColorTemperatureRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveToColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveToColorTemperatureRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveToColorTemperatureRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToColorTemperatureRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveToColorTemperatureRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToColorTemperatureRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveToColorTemperatureRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToColorTemperatureRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveToColorTemperatureRequestArrayBuilder < P > (P ) ;
impl < P > MoveToColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveToColorTemperatureRequestBuilder < MoveToColorTemperatureRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveToColorTemperatureRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveToColorTemperatureRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveToColorTemperatureRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveToColorTemperatureRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveToColorTemperatureRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveToColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveToColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedMoveToHueRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > EnhancedMoveToHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > EnhancedMoveToHueRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn enhanced_hue (mut self , value : u16 ) -> Result < EnhancedMoveToHueRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "enhancedHue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (EnhancedMoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn direction (mut self , value : DirectionEnum ) -> Result < EnhancedMoveToHueRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "direction" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (EnhancedMoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < EnhancedMoveToHueRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (EnhancedMoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < EnhancedMoveToHueRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (EnhancedMoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < EnhancedMoveToHueRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (EnhancedMoveToHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for EnhancedMoveToHueRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for EnhancedMoveToHueRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for EnhancedMoveToHueRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedMoveToHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedMoveToHueRequestArrayBuilder < P > (P ) ;
impl < P > EnhancedMoveToHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < EnhancedMoveToHueRequestBuilder < EnhancedMoveToHueRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (EnhancedMoveToHueRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for EnhancedMoveToHueRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for EnhancedMoveToHueRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for EnhancedMoveToHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedMoveToHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedMoveHueRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > EnhancedMoveHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > EnhancedMoveHueRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_mode (mut self , value : MoveModeEnum ) -> Result < EnhancedMoveHueRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "moveMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (EnhancedMoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveHueRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate (mut self , value : u16 ) -> Result < EnhancedMoveHueRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (EnhancedMoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveHueRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < EnhancedMoveHueRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (EnhancedMoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveHueRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < EnhancedMoveHueRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (EnhancedMoveHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveHueRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for EnhancedMoveHueRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveHueRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for EnhancedMoveHueRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveHueRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for EnhancedMoveHueRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedMoveHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedMoveHueRequestArrayBuilder < P > (P ) ;
impl < P > EnhancedMoveHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < EnhancedMoveHueRequestBuilder < EnhancedMoveHueRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (EnhancedMoveHueRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for EnhancedMoveHueRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveHueRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for EnhancedMoveHueRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveHueRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for EnhancedMoveHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedMoveHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedStepHueRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > EnhancedStepHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > EnhancedStepHueRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_mode (mut self , value : StepModeEnum ) -> Result < EnhancedStepHueRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (EnhancedStepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedStepHueRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_size (mut self , value : u16 ) -> Result < EnhancedStepHueRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepSize" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (EnhancedStepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedStepHueRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < EnhancedStepHueRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (EnhancedStepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedStepHueRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < EnhancedStepHueRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (EnhancedStepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedStepHueRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < EnhancedStepHueRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (EnhancedStepHueRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedStepHueRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for EnhancedStepHueRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedStepHueRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for EnhancedStepHueRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedStepHueRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for EnhancedStepHueRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedStepHueRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedStepHueRequestArrayBuilder < P > (P ) ;
impl < P > EnhancedStepHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < EnhancedStepHueRequestBuilder < EnhancedStepHueRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (EnhancedStepHueRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for EnhancedStepHueRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedStepHueRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for EnhancedStepHueRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedStepHueRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for EnhancedStepHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedStepHueRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedMoveToHueAndSaturationRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > EnhancedMoveToHueAndSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > EnhancedMoveToHueAndSaturationRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn enhanced_hue (mut self , value : u16 ) -> Result < EnhancedMoveToHueAndSaturationRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "enhancedHue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (EnhancedMoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueAndSaturationRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn saturation (mut self , value : u8 ) -> Result < EnhancedMoveToHueAndSaturationRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "saturation" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (EnhancedMoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueAndSaturationRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < EnhancedMoveToHueAndSaturationRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (EnhancedMoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueAndSaturationRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < EnhancedMoveToHueAndSaturationRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (EnhancedMoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueAndSaturationRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < EnhancedMoveToHueAndSaturationRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (EnhancedMoveToHueAndSaturationRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EnhancedMoveToHueAndSaturationRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for EnhancedMoveToHueAndSaturationRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueAndSaturationRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for EnhancedMoveToHueAndSaturationRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueAndSaturationRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for EnhancedMoveToHueAndSaturationRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedMoveToHueAndSaturationRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EnhancedMoveToHueAndSaturationRequestArrayBuilder < P > (P ) ;
impl < P > EnhancedMoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < EnhancedMoveToHueAndSaturationRequestBuilder < EnhancedMoveToHueAndSaturationRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (EnhancedMoveToHueAndSaturationRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for EnhancedMoveToHueAndSaturationRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueAndSaturationRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for EnhancedMoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EnhancedMoveToHueAndSaturationRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for EnhancedMoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EnhancedMoveToHueAndSaturationRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ColorLoopSetRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ColorLoopSetRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn update_flags (mut self , value : UpdateFlagsBitmap ) -> Result < ColorLoopSetRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "updateFlags" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ColorLoopSetRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn action (mut self , value : ColorLoopActionEnum ) -> Result < ColorLoopSetRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "action" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ColorLoopSetRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn direction (mut self , value : ColorLoopDirectionEnum ) -> Result < ColorLoopSetRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "direction" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ColorLoopSetRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn time (mut self , value : u16 ) -> Result < ColorLoopSetRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "time" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ColorLoopSetRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn start_hue (mut self , value : u16 ) -> Result < ColorLoopSetRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "startHue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (ColorLoopSetRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < ColorLoopSetRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (ColorLoopSetRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < ColorLoopSetRequestBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (ColorLoopSetRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ColorLoopSetRequestBuilder < P , 7usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ColorLoopSetRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ColorLoopSetRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ColorLoopSetRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ColorLoopSetRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ColorLoopSetRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ColorLoopSetRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ColorLoopSetRequestArrayBuilder < P > (P ) ;
impl < P > ColorLoopSetRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ColorLoopSetRequestBuilder < ColorLoopSetRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ColorLoopSetRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ColorLoopSetRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ColorLoopSetRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ColorLoopSetRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ColorLoopSetRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ColorLoopSetRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ColorLoopSetRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StopMoveStepRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StopMoveStepRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StopMoveStepRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StopMoveStepRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StopMoveStepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StopMoveStepRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StopMoveStepRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StopMoveStepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StopMoveStepRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StopMoveStepRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StopMoveStepRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StopMoveStepRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StopMoveStepRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StopMoveStepRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StopMoveStepRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StopMoveStepRequestArrayBuilder < P > (P ) ;
impl < P > StopMoveStepRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StopMoveStepRequestBuilder < StopMoveStepRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StopMoveStepRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StopMoveStepRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StopMoveStepRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StopMoveStepRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StopMoveStepRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StopMoveStepRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StopMoveStepRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveColorTemperatureRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MoveColorTemperatureRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MoveColorTemperatureRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_mode (mut self , value : MoveModeEnum ) -> Result < MoveColorTemperatureRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "moveMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MoveColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorTemperatureRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn rate (mut self , value : u16 ) -> Result < MoveColorTemperatureRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MoveColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorTemperatureRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn color_temperature_minimum_mireds (mut self , value : u16 ) -> Result < MoveColorTemperatureRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "colorTemperatureMinimumMireds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MoveColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorTemperatureRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn color_temperature_maximum_mireds (mut self , value : u16 ) -> Result < MoveColorTemperatureRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "colorTemperatureMaximumMireds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MoveColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorTemperatureRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < MoveColorTemperatureRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (MoveColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorTemperatureRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < MoveColorTemperatureRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (MoveColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > MoveColorTemperatureRequestBuilder < P , 6usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MoveColorTemperatureRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveColorTemperatureRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MoveColorTemperatureRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveColorTemperatureRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MoveColorTemperatureRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveColorTemperatureRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MoveColorTemperatureRequestArrayBuilder < P > (P ) ;
impl < P > MoveColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MoveColorTemperatureRequestBuilder < MoveColorTemperatureRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MoveColorTemperatureRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MoveColorTemperatureRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MoveColorTemperatureRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MoveColorTemperatureRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MoveColorTemperatureRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MoveColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MoveColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepColorTemperatureRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StepColorTemperatureRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_mode (mut self , value : StepModeEnum ) -> Result < StepColorTemperatureRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepMode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StepColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step_size (mut self , value : u16 ) -> Result < StepColorTemperatureRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "stepSize" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StepColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transition_time (mut self , value : u16 ) -> Result < StepColorTemperatureRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transitionTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StepColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn color_temperature_minimum_mireds (mut self , value : u16 ) -> Result < StepColorTemperatureRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "colorTemperatureMinimumMireds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (StepColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn color_temperature_maximum_mireds (mut self , value : u16 ) -> Result < StepColorTemperatureRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "colorTemperatureMaximumMireds" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (StepColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_mask (mut self , value : OptionsBitmap ) -> Result < StepColorTemperatureRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsMask" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (StepColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options_override (mut self , value : OptionsBitmap ) -> Result < StepColorTemperatureRequestBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "optionsOverride" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (StepColorTemperatureRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepColorTemperatureRequestBuilder < P , 7usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StepColorTemperatureRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepColorTemperatureRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StepColorTemperatureRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepColorTemperatureRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StepColorTemperatureRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepColorTemperatureRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StepColorTemperatureRequestArrayBuilder < P > (P ) ;
impl < P > StepColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StepColorTemperatureRequestBuilder < StepColorTemperatureRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StepColorTemperatureRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StepColorTemperatureRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StepColorTemperatureRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StepColorTemperatureRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StepColorTemperatureRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StepColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StepColorTemperatureRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    CurrentHue = 0 , CurrentSaturation = 1 , RemainingTime = 2 , CurrentX = 3 , CurrentY = 4 , DriftCompensation = 5 , CompensationText = 6 , ColorTemperatureMireds = 7 , ColorMode = 8 , Options = 15 , NumberOfPrimaries = 16 , Primary1X = 17 , Primary1Y = 18 , Primary1Intensity = 19 , Primary2X = 21 , Primary2Y = 22 , Primary2Intensity = 23 , Primary3X = 25 , Primary3Y = 26 , Primary3Intensity = 27 , Primary4X = 32 , Primary4Y = 33 , Primary4Intensity = 34 , Primary5X = 36 , Primary5Y = 37 , Primary5Intensity = 38 , Primary6X = 40 , Primary6Y = 41 , Primary6Intensity = 42 , WhitePointX = 48 , WhitePointY = 49 , ColorPointRX = 50 , ColorPointRY = 51 , ColorPointRIntensity = 52 , ColorPointGX = 54 , ColorPointGY = 55 , ColorPointGIntensity = 56 , ColorPointBX = 58 , ColorPointBY = 59 , ColorPointBIntensity = 60 , EnhancedCurrentHue = 16384 , EnhancedColorMode = 16385 , ColorLoopActive = 16386 , ColorLoopDirection = 16387 , ColorLoopTime = 16388 , ColorLoopStartEnhancedHue = 16389 , ColorLoopStoredEnhancedHue = 16390 , ColorCapabilities = 16394 , ColorTempPhysicalMinMireds = 16395 , ColorTempPhysicalMaxMireds = 16396 , CoupleColorTempToLevelMinMireds = 16397 , StartUpColorTemperatureMireds = 16400 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: CurrentHue => write ! (f , "{}(0x{:02x})" , "CurrentHue" , AttributeId :: CurrentHue as u32 ) ? , AttributeId :: CurrentSaturation => write ! (f , "{}(0x{:02x})" , "CurrentSaturation" , AttributeId :: CurrentSaturation as u32 ) ? , AttributeId :: RemainingTime => write ! (f , "{}(0x{:02x})" , "RemainingTime" , AttributeId :: RemainingTime as u32 ) ? , AttributeId :: CurrentX => write ! (f , "{}(0x{:02x})" , "CurrentX" , AttributeId :: CurrentX as u32 ) ? , AttributeId :: CurrentY => write ! (f , "{}(0x{:02x})" , "CurrentY" , AttributeId :: CurrentY as u32 ) ? , AttributeId :: DriftCompensation => write ! (f , "{}(0x{:02x})" , "DriftCompensation" , AttributeId :: DriftCompensation as u32 ) ? , AttributeId :: CompensationText => write ! (f , "{}(0x{:02x})" , "CompensationText" , AttributeId :: CompensationText as u32 ) ? , AttributeId :: ColorTemperatureMireds => write ! (f , "{}(0x{:02x})" , "ColorTemperatureMireds" , AttributeId :: ColorTemperatureMireds as u32 ) ? , AttributeId :: ColorMode => write ! (f , "{}(0x{:02x})" , "ColorMode" , AttributeId :: ColorMode as u32 ) ? , AttributeId :: Options => write ! (f , "{}(0x{:02x})" , "Options" , AttributeId :: Options as u32 ) ? , AttributeId :: NumberOfPrimaries => write ! (f , "{}(0x{:02x})" , "NumberOfPrimaries" , AttributeId :: NumberOfPrimaries as u32 ) ? , AttributeId :: Primary1X => write ! (f , "{}(0x{:02x})" , "Primary1X" , AttributeId :: Primary1X as u32 ) ? , AttributeId :: Primary1Y => write ! (f , "{}(0x{:02x})" , "Primary1Y" , AttributeId :: Primary1Y as u32 ) ? , AttributeId :: Primary1Intensity => write ! (f , "{}(0x{:02x})" , "Primary1Intensity" , AttributeId :: Primary1Intensity as u32 ) ? , AttributeId :: Primary2X => write ! (f , "{}(0x{:02x})" , "Primary2X" , AttributeId :: Primary2X as u32 ) ? , AttributeId :: Primary2Y => write ! (f , "{}(0x{:02x})" , "Primary2Y" , AttributeId :: Primary2Y as u32 ) ? , AttributeId :: Primary2Intensity => write ! (f , "{}(0x{:02x})" , "Primary2Intensity" , AttributeId :: Primary2Intensity as u32 ) ? , AttributeId :: Primary3X => write ! (f , "{}(0x{:02x})" , "Primary3X" , AttributeId :: Primary3X as u32 ) ? , AttributeId :: Primary3Y => write ! (f , "{}(0x{:02x})" , "Primary3Y" , AttributeId :: Primary3Y as u32 ) ? , AttributeId :: Primary3Intensity => write ! (f , "{}(0x{:02x})" , "Primary3Intensity" , AttributeId :: Primary3Intensity as u32 ) ? , AttributeId :: Primary4X => write ! (f , "{}(0x{:02x})" , "Primary4X" , AttributeId :: Primary4X as u32 ) ? , AttributeId :: Primary4Y => write ! (f , "{}(0x{:02x})" , "Primary4Y" , AttributeId :: Primary4Y as u32 ) ? , AttributeId :: Primary4Intensity => write ! (f , "{}(0x{:02x})" , "Primary4Intensity" , AttributeId :: Primary4Intensity as u32 ) ? , AttributeId :: Primary5X => write ! (f , "{}(0x{:02x})" , "Primary5X" , AttributeId :: Primary5X as u32 ) ? , AttributeId :: Primary5Y => write ! (f , "{}(0x{:02x})" , "Primary5Y" , AttributeId :: Primary5Y as u32 ) ? , AttributeId :: Primary5Intensity => write ! (f , "{}(0x{:02x})" , "Primary5Intensity" , AttributeId :: Primary5Intensity as u32 ) ? , AttributeId :: Primary6X => write ! (f , "{}(0x{:02x})" , "Primary6X" , AttributeId :: Primary6X as u32 ) ? , AttributeId :: Primary6Y => write ! (f , "{}(0x{:02x})" , "Primary6Y" , AttributeId :: Primary6Y as u32 ) ? , AttributeId :: Primary6Intensity => write ! (f , "{}(0x{:02x})" , "Primary6Intensity" , AttributeId :: Primary6Intensity as u32 ) ? , AttributeId :: WhitePointX => write ! (f , "{}(0x{:02x})" , "WhitePointX" , AttributeId :: WhitePointX as u32 ) ? , AttributeId :: WhitePointY => write ! (f , "{}(0x{:02x})" , "WhitePointY" , AttributeId :: WhitePointY as u32 ) ? , AttributeId :: ColorPointRX => write ! (f , "{}(0x{:02x})" , "ColorPointRX" , AttributeId :: ColorPointRX as u32 ) ? , AttributeId :: ColorPointRY => write ! (f , "{}(0x{:02x})" , "ColorPointRY" , AttributeId :: ColorPointRY as u32 ) ? , AttributeId :: ColorPointRIntensity => write ! (f , "{}(0x{:02x})" , "ColorPointRIntensity" , AttributeId :: ColorPointRIntensity as u32 ) ? , AttributeId :: ColorPointGX => write ! (f , "{}(0x{:02x})" , "ColorPointGX" , AttributeId :: ColorPointGX as u32 ) ? , AttributeId :: ColorPointGY => write ! (f , "{}(0x{:02x})" , "ColorPointGY" , AttributeId :: ColorPointGY as u32 ) ? , AttributeId :: ColorPointGIntensity => write ! (f , "{}(0x{:02x})" , "ColorPointGIntensity" , AttributeId :: ColorPointGIntensity as u32 ) ? , AttributeId :: ColorPointBX => write ! (f , "{}(0x{:02x})" , "ColorPointBX" , AttributeId :: ColorPointBX as u32 ) ? , AttributeId :: ColorPointBY => write ! (f , "{}(0x{:02x})" , "ColorPointBY" , AttributeId :: ColorPointBY as u32 ) ? , AttributeId :: ColorPointBIntensity => write ! (f , "{}(0x{:02x})" , "ColorPointBIntensity" , AttributeId :: ColorPointBIntensity as u32 ) ? , AttributeId :: EnhancedCurrentHue => write ! (f , "{}(0x{:02x})" , "EnhancedCurrentHue" , AttributeId :: EnhancedCurrentHue as u32 ) ? , AttributeId :: EnhancedColorMode => write ! (f , "{}(0x{:02x})" , "EnhancedColorMode" , AttributeId :: EnhancedColorMode as u32 ) ? , AttributeId :: ColorLoopActive => write ! (f , "{}(0x{:02x})" , "ColorLoopActive" , AttributeId :: ColorLoopActive as u32 ) ? , AttributeId :: ColorLoopDirection => write ! (f , "{}(0x{:02x})" , "ColorLoopDirection" , AttributeId :: ColorLoopDirection as u32 ) ? , AttributeId :: ColorLoopTime => write ! (f , "{}(0x{:02x})" , "ColorLoopTime" , AttributeId :: ColorLoopTime as u32 ) ? , AttributeId :: ColorLoopStartEnhancedHue => write ! (f , "{}(0x{:02x})" , "ColorLoopStartEnhancedHue" , AttributeId :: ColorLoopStartEnhancedHue as u32 ) ? , AttributeId :: ColorLoopStoredEnhancedHue => write ! (f , "{}(0x{:02x})" , "ColorLoopStoredEnhancedHue" , AttributeId :: ColorLoopStoredEnhancedHue as u32 ) ? , AttributeId :: ColorCapabilities => write ! (f , "{}(0x{:02x})" , "ColorCapabilities" , AttributeId :: ColorCapabilities as u32 ) ? , AttributeId :: ColorTempPhysicalMinMireds => write ! (f , "{}(0x{:02x})" , "ColorTempPhysicalMinMireds" , AttributeId :: ColorTempPhysicalMinMireds as u32 ) ? , AttributeId :: ColorTempPhysicalMaxMireds => write ! (f , "{}(0x{:02x})" , "ColorTempPhysicalMaxMireds" , AttributeId :: ColorTempPhysicalMaxMireds as u32 ) ? , AttributeId :: CoupleColorTempToLevelMinMireds => write ! (f , "{}(0x{:02x})" , "CoupleColorTempToLevelMinMireds" , AttributeId :: CoupleColorTempToLevelMinMireds as u32 ) ? , AttributeId :: StartUpColorTemperatureMireds => write ! (f , "{}(0x{:02x})" , "StartUpColorTemperatureMireds" , AttributeId :: StartUpColorTemperatureMireds as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: CurrentHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentHue" , AttributeId :: CurrentHue as u32 ) , AttributeId :: CurrentSaturation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentSaturation" , AttributeId :: CurrentSaturation as u32 ) , AttributeId :: RemainingTime => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RemainingTime" , AttributeId :: RemainingTime as u32 ) , AttributeId :: CurrentX => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentX" , AttributeId :: CurrentX as u32 ) , AttributeId :: CurrentY => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentY" , AttributeId :: CurrentY as u32 ) , AttributeId :: DriftCompensation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DriftCompensation" , AttributeId :: DriftCompensation as u32 ) , AttributeId :: CompensationText => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CompensationText" , AttributeId :: CompensationText as u32 ) , AttributeId :: ColorTemperatureMireds => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorTemperatureMireds" , AttributeId :: ColorTemperatureMireds as u32 ) , AttributeId :: ColorMode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorMode" , AttributeId :: ColorMode as u32 ) , AttributeId :: Options => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Options" , AttributeId :: Options as u32 ) , AttributeId :: NumberOfPrimaries => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfPrimaries" , AttributeId :: NumberOfPrimaries as u32 ) , AttributeId :: Primary1X => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary1X" , AttributeId :: Primary1X as u32 ) , AttributeId :: Primary1Y => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary1Y" , AttributeId :: Primary1Y as u32 ) , AttributeId :: Primary1Intensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary1Intensity" , AttributeId :: Primary1Intensity as u32 ) , AttributeId :: Primary2X => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary2X" , AttributeId :: Primary2X as u32 ) , AttributeId :: Primary2Y => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary2Y" , AttributeId :: Primary2Y as u32 ) , AttributeId :: Primary2Intensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary2Intensity" , AttributeId :: Primary2Intensity as u32 ) , AttributeId :: Primary3X => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary3X" , AttributeId :: Primary3X as u32 ) , AttributeId :: Primary3Y => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary3Y" , AttributeId :: Primary3Y as u32 ) , AttributeId :: Primary3Intensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary3Intensity" , AttributeId :: Primary3Intensity as u32 ) , AttributeId :: Primary4X => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary4X" , AttributeId :: Primary4X as u32 ) , AttributeId :: Primary4Y => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary4Y" , AttributeId :: Primary4Y as u32 ) , AttributeId :: Primary4Intensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary4Intensity" , AttributeId :: Primary4Intensity as u32 ) , AttributeId :: Primary5X => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary5X" , AttributeId :: Primary5X as u32 ) , AttributeId :: Primary5Y => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary5Y" , AttributeId :: Primary5Y as u32 ) , AttributeId :: Primary5Intensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary5Intensity" , AttributeId :: Primary5Intensity as u32 ) , AttributeId :: Primary6X => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary6X" , AttributeId :: Primary6X as u32 ) , AttributeId :: Primary6Y => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary6Y" , AttributeId :: Primary6Y as u32 ) , AttributeId :: Primary6Intensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Primary6Intensity" , AttributeId :: Primary6Intensity as u32 ) , AttributeId :: WhitePointX => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WhitePointX" , AttributeId :: WhitePointX as u32 ) , AttributeId :: WhitePointY => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WhitePointY" , AttributeId :: WhitePointY as u32 ) , AttributeId :: ColorPointRX => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointRX" , AttributeId :: ColorPointRX as u32 ) , AttributeId :: ColorPointRY => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointRY" , AttributeId :: ColorPointRY as u32 ) , AttributeId :: ColorPointRIntensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointRIntensity" , AttributeId :: ColorPointRIntensity as u32 ) , AttributeId :: ColorPointGX => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointGX" , AttributeId :: ColorPointGX as u32 ) , AttributeId :: ColorPointGY => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointGY" , AttributeId :: ColorPointGY as u32 ) , AttributeId :: ColorPointGIntensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointGIntensity" , AttributeId :: ColorPointGIntensity as u32 ) , AttributeId :: ColorPointBX => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointBX" , AttributeId :: ColorPointBX as u32 ) , AttributeId :: ColorPointBY => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointBY" , AttributeId :: ColorPointBY as u32 ) , AttributeId :: ColorPointBIntensity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorPointBIntensity" , AttributeId :: ColorPointBIntensity as u32 ) , AttributeId :: EnhancedCurrentHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EnhancedCurrentHue" , AttributeId :: EnhancedCurrentHue as u32 ) , AttributeId :: EnhancedColorMode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EnhancedColorMode" , AttributeId :: EnhancedColorMode as u32 ) , AttributeId :: ColorLoopActive => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorLoopActive" , AttributeId :: ColorLoopActive as u32 ) , AttributeId :: ColorLoopDirection => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorLoopDirection" , AttributeId :: ColorLoopDirection as u32 ) , AttributeId :: ColorLoopTime => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorLoopTime" , AttributeId :: ColorLoopTime as u32 ) , AttributeId :: ColorLoopStartEnhancedHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorLoopStartEnhancedHue" , AttributeId :: ColorLoopStartEnhancedHue as u32 ) , AttributeId :: ColorLoopStoredEnhancedHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorLoopStoredEnhancedHue" , AttributeId :: ColorLoopStoredEnhancedHue as u32 ) , AttributeId :: ColorCapabilities => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorCapabilities" , AttributeId :: ColorCapabilities as u32 ) , AttributeId :: ColorTempPhysicalMinMireds => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorTempPhysicalMinMireds" , AttributeId :: ColorTempPhysicalMinMireds as u32 ) , AttributeId :: ColorTempPhysicalMaxMireds => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorTempPhysicalMaxMireds" , AttributeId :: ColorTempPhysicalMaxMireds as u32 ) , AttributeId :: CoupleColorTempToLevelMinMireds => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CoupleColorTempToLevelMinMireds" , AttributeId :: CoupleColorTempToLevelMinMireds as u32 ) , AttributeId :: StartUpColorTemperatureMireds => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StartUpColorTemperatureMireds" , AttributeId :: StartUpColorTemperatureMireds as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    MoveToHue = 0 , MoveHue = 1 , StepHue = 2 , MoveToSaturation = 3 , MoveSaturation = 4 , StepSaturation = 5 , MoveToHueAndSaturation = 6 , MoveToColor = 7 , MoveColor = 8 , StepColor = 9 , MoveToColorTemperature = 10 , EnhancedMoveToHue = 64 , EnhancedMoveHue = 65 , EnhancedStepHue = 66 , EnhancedMoveToHueAndSaturation = 67 , ColorLoopSet = 68 , StopMoveStep = 71 , MoveColorTemperature = 75 , StepColorTemperature = 76 
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
            CommandId :: MoveToHue => write ! (f , "{}(0x{:02x})" , "MoveToHue" , CommandId :: MoveToHue as u32 ) ? , CommandId :: MoveHue => write ! (f , "{}(0x{:02x})" , "MoveHue" , CommandId :: MoveHue as u32 ) ? , CommandId :: StepHue => write ! (f , "{}(0x{:02x})" , "StepHue" , CommandId :: StepHue as u32 ) ? , CommandId :: MoveToSaturation => write ! (f , "{}(0x{:02x})" , "MoveToSaturation" , CommandId :: MoveToSaturation as u32 ) ? , CommandId :: MoveSaturation => write ! (f , "{}(0x{:02x})" , "MoveSaturation" , CommandId :: MoveSaturation as u32 ) ? , CommandId :: StepSaturation => write ! (f , "{}(0x{:02x})" , "StepSaturation" , CommandId :: StepSaturation as u32 ) ? , CommandId :: MoveToHueAndSaturation => write ! (f , "{}(0x{:02x})" , "MoveToHueAndSaturation" , CommandId :: MoveToHueAndSaturation as u32 ) ? , CommandId :: MoveToColor => write ! (f , "{}(0x{:02x})" , "MoveToColor" , CommandId :: MoveToColor as u32 ) ? , CommandId :: MoveColor => write ! (f , "{}(0x{:02x})" , "MoveColor" , CommandId :: MoveColor as u32 ) ? , CommandId :: StepColor => write ! (f , "{}(0x{:02x})" , "StepColor" , CommandId :: StepColor as u32 ) ? , CommandId :: MoveToColorTemperature => write ! (f , "{}(0x{:02x})" , "MoveToColorTemperature" , CommandId :: MoveToColorTemperature as u32 ) ? , CommandId :: EnhancedMoveToHue => write ! (f , "{}(0x{:02x})" , "EnhancedMoveToHue" , CommandId :: EnhancedMoveToHue as u32 ) ? , CommandId :: EnhancedMoveHue => write ! (f , "{}(0x{:02x})" , "EnhancedMoveHue" , CommandId :: EnhancedMoveHue as u32 ) ? , CommandId :: EnhancedStepHue => write ! (f , "{}(0x{:02x})" , "EnhancedStepHue" , CommandId :: EnhancedStepHue as u32 ) ? , CommandId :: EnhancedMoveToHueAndSaturation => write ! (f , "{}(0x{:02x})" , "EnhancedMoveToHueAndSaturation" , CommandId :: EnhancedMoveToHueAndSaturation as u32 ) ? , CommandId :: ColorLoopSet => write ! (f , "{}(0x{:02x})" , "ColorLoopSet" , CommandId :: ColorLoopSet as u32 ) ? , CommandId :: StopMoveStep => write ! (f , "{}(0x{:02x})" , "StopMoveStep" , CommandId :: StopMoveStep as u32 ) ? , CommandId :: MoveColorTemperature => write ! (f , "{}(0x{:02x})" , "MoveColorTemperature" , CommandId :: MoveColorTemperature as u32 ) ? , CommandId :: StepColorTemperature => write ! (f , "{}(0x{:02x})" , "StepColorTemperature" , CommandId :: StepColorTemperature as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: MoveToHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToHue" , CommandId :: MoveToHue as u32 ) , CommandId :: MoveHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveHue" , CommandId :: MoveHue as u32 ) , CommandId :: StepHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StepHue" , CommandId :: StepHue as u32 ) , CommandId :: MoveToSaturation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToSaturation" , CommandId :: MoveToSaturation as u32 ) , CommandId :: MoveSaturation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveSaturation" , CommandId :: MoveSaturation as u32 ) , CommandId :: StepSaturation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StepSaturation" , CommandId :: StepSaturation as u32 ) , CommandId :: MoveToHueAndSaturation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToHueAndSaturation" , CommandId :: MoveToHueAndSaturation as u32 ) , CommandId :: MoveToColor => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToColor" , CommandId :: MoveToColor as u32 ) , CommandId :: MoveColor => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveColor" , CommandId :: MoveColor as u32 ) , CommandId :: StepColor => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StepColor" , CommandId :: StepColor as u32 ) , CommandId :: MoveToColorTemperature => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveToColorTemperature" , CommandId :: MoveToColorTemperature as u32 ) , CommandId :: EnhancedMoveToHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EnhancedMoveToHue" , CommandId :: EnhancedMoveToHue as u32 ) , CommandId :: EnhancedMoveHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EnhancedMoveHue" , CommandId :: EnhancedMoveHue as u32 ) , CommandId :: EnhancedStepHue => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EnhancedStepHue" , CommandId :: EnhancedStepHue as u32 ) , CommandId :: EnhancedMoveToHueAndSaturation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EnhancedMoveToHueAndSaturation" , CommandId :: EnhancedMoveToHueAndSaturation as u32 ) , CommandId :: ColorLoopSet => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ColorLoopSet" , CommandId :: ColorLoopSet as u32 ) , CommandId :: StopMoveStep => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StopMoveStep" , CommandId :: StopMoveStep as u32 ) , CommandId :: MoveColorTemperature => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MoveColorTemperature" , CommandId :: MoveColorTemperature as u32 ) , CommandId :: StepColorTemperature => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StepColorTemperature" , CommandId :: StepColorTemperature as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (768 , 9 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: CurrentHue as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentSaturation as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RemainingTime as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentX as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentY as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: DriftCompensation as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: CompensationText as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorTemperatureMireds as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorMode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Options as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfPrimaries as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Primary1X as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary1Y as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary1Intensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary2X as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary2Y as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary2Intensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary3X as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary3Y as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary3Intensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary4X as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary4Y as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary4Intensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary5X as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary5Y as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary5Intensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary6X as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary6Y as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Primary6Intensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WhitePointX as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WhitePointY as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointRX as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointRY as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointRIntensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointGX as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointGY as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointGIntensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointBX as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointBY as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorPointBIntensity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: EnhancedCurrentHue as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: EnhancedColorMode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ColorLoopActive as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorLoopDirection as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorLoopTime as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorLoopStartEnhancedHue as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorLoopStoredEnhancedHue as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorCapabilities as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ColorTempPhysicalMinMireds as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ColorTempPhysicalMaxMireds as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: CoupleColorTempToLevelMinMireds as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: StartUpColorTemperatureMireds as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: MoveToHue as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveHue as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: StepHue as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveToSaturation as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveSaturation as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: StepSaturation as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveToHueAndSaturation as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveToColor as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveColor as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: StepColor as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveToColorTemperature as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: EnhancedMoveToHue as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: EnhancedMoveHue as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: EnhancedStepHue as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: EnhancedMoveToHueAndSaturation as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: ColorLoopSet as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: StopMoveStep as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: MoveColorTemperature as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: StepColorTemperature as _ , None , crate :: dm :: Access :: WO , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn current_saturation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn current_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn current_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn drift_compensation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < DriftCompensationEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn compensation_text < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ColorModeEnum , crate :: error :: Error > ;
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < OptionsBitmap , crate :: error :: Error > ;
    fn number_of_primaries (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > ;
    fn primary_1_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_1_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_1_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_2_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_2_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_2_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_3_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_3_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_3_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_4_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_4_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_4_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_5_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_5_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_5_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_6_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_6_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn primary_6_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn white_point_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn white_point_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_rx (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_ry (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_r_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_gx (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_gy (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_g_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_bx (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_by (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_point_b_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn enhanced_current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn enhanced_color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < EnhancedColorModeEnum , crate :: error :: Error > ;
    fn color_loop_active (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_loop_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_loop_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_loop_start_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_loop_stored_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_capabilities (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ColorCapabilitiesBitmap , crate :: error :: Error > ;
    fn color_temp_physical_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn color_temp_physical_max_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn couple_color_temp_to_level_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> Result < () , crate :: error :: Error > ;
    fn set_start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : StepHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_to_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_step_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : StepSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueAndSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_to_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_step_color (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_to_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorTemperatureRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_enhanced_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_enhanced_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_enhanced_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedStepHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_enhanced_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueAndSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_color_loop_set (& self , ctx : impl crate :: dm :: InvokeContext , request : ColorLoopSetRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_stop_move_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StopMoveStepRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_move_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorTemperatureRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_step_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorTemperatureRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: current_hue (self , ctx ) 
    }
    fn current_saturation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: current_saturation (self , ctx ) 
    }
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: remaining_time (self , ctx ) 
    }
    fn current_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: current_x (self , ctx ) 
    }
    fn current_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: current_y (self , ctx ) 
    }
    fn drift_compensation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < DriftCompensationEnum , crate :: error :: Error > {
        T :: drift_compensation (self , ctx ) 
    }
    fn compensation_text < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: compensation_text (self , ctx , builder ) 
    }
    fn color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_temperature_mireds (self , ctx ) 
    }
    fn color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ColorModeEnum , crate :: error :: Error > {
        T :: color_mode (self , ctx ) 
    }
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < OptionsBitmap , crate :: error :: Error > {
        T :: options (self , ctx ) 
    }
    fn number_of_primaries (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: number_of_primaries (self , ctx ) 
    }
    fn primary_1_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_1_x (self , ctx ) 
    }
    fn primary_1_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_1_y (self , ctx ) 
    }
    fn primary_1_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: primary_1_intensity (self , ctx ) 
    }
    fn primary_2_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_2_x (self , ctx ) 
    }
    fn primary_2_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_2_y (self , ctx ) 
    }
    fn primary_2_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: primary_2_intensity (self , ctx ) 
    }
    fn primary_3_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_3_x (self , ctx ) 
    }
    fn primary_3_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_3_y (self , ctx ) 
    }
    fn primary_3_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: primary_3_intensity (self , ctx ) 
    }
    fn primary_4_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_4_x (self , ctx ) 
    }
    fn primary_4_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_4_y (self , ctx ) 
    }
    fn primary_4_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: primary_4_intensity (self , ctx ) 
    }
    fn primary_5_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_5_x (self , ctx ) 
    }
    fn primary_5_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_5_y (self , ctx ) 
    }
    fn primary_5_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: primary_5_intensity (self , ctx ) 
    }
    fn primary_6_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_6_x (self , ctx ) 
    }
    fn primary_6_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: primary_6_y (self , ctx ) 
    }
    fn primary_6_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: primary_6_intensity (self , ctx ) 
    }
    fn white_point_x (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: white_point_x (self , ctx ) 
    }
    fn white_point_y (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: white_point_y (self , ctx ) 
    }
    fn color_point_rx (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_point_rx (self , ctx ) 
    }
    fn color_point_ry (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_point_ry (self , ctx ) 
    }
    fn color_point_r_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: color_point_r_intensity (self , ctx ) 
    }
    fn color_point_gx (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_point_gx (self , ctx ) 
    }
    fn color_point_gy (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_point_gy (self , ctx ) 
    }
    fn color_point_g_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: color_point_g_intensity (self , ctx ) 
    }
    fn color_point_bx (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_point_bx (self , ctx ) 
    }
    fn color_point_by (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_point_by (self , ctx ) 
    }
    fn color_point_b_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: color_point_b_intensity (self , ctx ) 
    }
    fn enhanced_current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: enhanced_current_hue (self , ctx ) 
    }
    fn enhanced_color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < EnhancedColorModeEnum , crate :: error :: Error > {
        T :: enhanced_color_mode (self , ctx ) 
    }
    fn color_loop_active (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: color_loop_active (self , ctx ) 
    }
    fn color_loop_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: color_loop_direction (self , ctx ) 
    }
    fn color_loop_time (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_loop_time (self , ctx ) 
    }
    fn color_loop_start_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_loop_start_enhanced_hue (self , ctx ) 
    }
    fn color_loop_stored_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_loop_stored_enhanced_hue (self , ctx ) 
    }
    fn color_capabilities (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ColorCapabilitiesBitmap , crate :: error :: Error > {
        T :: color_capabilities (self , ctx ) 
    }
    fn color_temp_physical_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_temp_physical_min_mireds (self , ctx ) 
    }
    fn color_temp_physical_max_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: color_temp_physical_max_mireds (self , ctx ) 
    }
    fn couple_color_temp_to_level_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: couple_color_temp_to_level_min_mireds (self , ctx ) 
    }
    fn start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: start_up_color_temperature_mireds (self , ctx ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_options (self , ctx , value ) 
    }
    fn set_start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> Result < () , crate :: error :: Error > {
        T :: set_start_up_color_temperature_mireds (self , ctx , value ) 
    }
    fn handle_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_hue (self , ctx , request ) 
    }
    fn handle_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_hue (self , ctx , request ) 
    }
    fn handle_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : StepHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_step_hue (self , ctx , request ) 
    }
    fn handle_move_to_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_saturation (self , ctx , request ) 
    }
    fn handle_move_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_saturation (self , ctx , request ) 
    }
    fn handle_step_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : StepSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_step_saturation (self , ctx , request ) 
    }
    fn handle_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueAndSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_hue_and_saturation (self , ctx , request ) 
    }
    fn handle_move_to_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_color (self , ctx , request ) 
    }
    fn handle_move_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_color (self , ctx , request ) 
    }
    fn handle_step_color (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_step_color (self , ctx , request ) 
    }
    fn handle_move_to_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorTemperatureRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_to_color_temperature (self , ctx , request ) 
    }
    fn handle_enhanced_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_enhanced_move_to_hue (self , ctx , request ) 
    }
    fn handle_enhanced_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_enhanced_move_hue (self , ctx , request ) 
    }
    fn handle_enhanced_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedStepHueRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_enhanced_step_hue (self , ctx , request ) 
    }
    fn handle_enhanced_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueAndSaturationRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_enhanced_move_to_hue_and_saturation (self , ctx , request ) 
    }
    fn handle_color_loop_set (& self , ctx : impl crate :: dm :: InvokeContext , request : ColorLoopSetRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_color_loop_set (self , ctx , request ) 
    }
    fn handle_stop_move_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StopMoveStepRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_stop_move_step (self , ctx , request ) 
    }
    fn handle_move_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorTemperatureRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_move_color_temperature (self , ctx , request ) 
    }
    fn handle_step_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorTemperatureRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_step_color_temperature (self , ctx , request ) 
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
                    AttributeId :: CurrentHue => {
                        let attr_read_result = self . 0. current_hue (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentSaturation => {
                        let attr_read_result = self . 0. current_saturation (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSaturation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RemainingTime => {
                        let attr_read_result = self . 0. remaining_time (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemainingTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentX => {
                        let attr_read_result = self . 0. current_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentY => {
                        let attr_read_result = self . 0. current_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DriftCompensation => {
                        let attr_read_result = self . 0. drift_compensation (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DriftCompensation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CompensationText => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CompensationText , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. compensation_text (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CompensationText , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CompensationText , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ColorTemperatureMireds => {
                        let attr_read_result = self . 0. color_temperature_mireds (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorTemperatureMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorMode => {
                        let attr_read_result = self . 0. color_mode (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Options => {
                        let attr_read_result = self . 0. options (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Options , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfPrimaries => {
                        let attr_read_result = self . 0. number_of_primaries (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfPrimaries , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary1X => {
                        let attr_read_result = self . 0. primary_1_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary1X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary1Y => {
                        let attr_read_result = self . 0. primary_1_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary1Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary1Intensity => {
                        let attr_read_result = self . 0. primary_1_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary1Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary2X => {
                        let attr_read_result = self . 0. primary_2_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary2X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary2Y => {
                        let attr_read_result = self . 0. primary_2_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary2Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary2Intensity => {
                        let attr_read_result = self . 0. primary_2_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary2Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary3X => {
                        let attr_read_result = self . 0. primary_3_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary3X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary3Y => {
                        let attr_read_result = self . 0. primary_3_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary3Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary3Intensity => {
                        let attr_read_result = self . 0. primary_3_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary3Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary4X => {
                        let attr_read_result = self . 0. primary_4_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary4X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary4Y => {
                        let attr_read_result = self . 0. primary_4_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary4Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary4Intensity => {
                        let attr_read_result = self . 0. primary_4_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary4Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary5X => {
                        let attr_read_result = self . 0. primary_5_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary5X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary5Y => {
                        let attr_read_result = self . 0. primary_5_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary5Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary5Intensity => {
                        let attr_read_result = self . 0. primary_5_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary5Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary6X => {
                        let attr_read_result = self . 0. primary_6_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary6X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary6Y => {
                        let attr_read_result = self . 0. primary_6_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary6Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary6Intensity => {
                        let attr_read_result = self . 0. primary_6_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary6Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WhitePointX => {
                        let attr_read_result = self . 0. white_point_x (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WhitePointX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WhitePointY => {
                        let attr_read_result = self . 0. white_point_y (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WhitePointY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointRX => {
                        let attr_read_result = self . 0. color_point_rx (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointRX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointRY => {
                        let attr_read_result = self . 0. color_point_ry (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointRY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointRIntensity => {
                        let attr_read_result = self . 0. color_point_r_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointRIntensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointGX => {
                        let attr_read_result = self . 0. color_point_gx (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointGX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointGY => {
                        let attr_read_result = self . 0. color_point_gy (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointGY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointGIntensity => {
                        let attr_read_result = self . 0. color_point_g_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointGIntensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointBX => {
                        let attr_read_result = self . 0. color_point_bx (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointBX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointBY => {
                        let attr_read_result = self . 0. color_point_by (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointBY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointBIntensity => {
                        let attr_read_result = self . 0. color_point_b_intensity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointBIntensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: EnhancedCurrentHue => {
                        let attr_read_result = self . 0. enhanced_current_hue (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EnhancedCurrentHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: EnhancedColorMode => {
                        let attr_read_result = self . 0. enhanced_color_mode (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EnhancedColorMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopActive => {
                        let attr_read_result = self . 0. color_loop_active (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopActive , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopDirection => {
                        let attr_read_result = self . 0. color_loop_direction (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopDirection , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopTime => {
                        let attr_read_result = self . 0. color_loop_time (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopStartEnhancedHue => {
                        let attr_read_result = self . 0. color_loop_start_enhanced_hue (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopStartEnhancedHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopStoredEnhancedHue => {
                        let attr_read_result = self . 0. color_loop_stored_enhanced_hue (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopStoredEnhancedHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorCapabilities => {
                        let attr_read_result = self . 0. color_capabilities (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorCapabilities , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorTempPhysicalMinMireds => {
                        let attr_read_result = self . 0. color_temp_physical_min_mireds (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorTempPhysicalMinMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorTempPhysicalMaxMireds => {
                        let attr_read_result = self . 0. color_temp_physical_max_mireds (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorTempPhysicalMaxMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CoupleColorTempToLevelMinMireds => {
                        let attr_read_result = self . 0. couple_color_temp_to_level_min_mireds (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CoupleColorTempToLevelMinMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StartUpColorTemperatureMireds => {
                        let attr_read_result = self . 0. start_up_color_temperature_mireds (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpColorTemperatureMireds , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: StartUpColorTemperatureMireds => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_start_up_color_temperature_mireds (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpColorTemperatureMireds , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: MoveToHue => {
                let cmd_data : MoveToHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_hue (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveHue => {
                let cmd_data : MoveHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_hue (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepHue => {
                let cmd_data : StepHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_hue (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToSaturation => {
                let cmd_data : MoveToSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_saturation (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveSaturation => {
                let cmd_data : MoveSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_saturation (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepSaturation => {
                let cmd_data : StepSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_saturation (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToHueAndSaturation => {
                let cmd_data : MoveToHueAndSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_hue_and_saturation (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToHueAndSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToColor => {
                let cmd_data : MoveToColorRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_color (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToColor ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveColor => {
                let cmd_data : MoveColorRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_color (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveColor ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepColor => {
                let cmd_data : StepColorRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_color (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepColor ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToColorTemperature => {
                let cmd_data : MoveToColorTemperatureRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_color_temperature (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToColorTemperature ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedMoveToHue => {
                let cmd_data : EnhancedMoveToHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_move_to_hue (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedMoveToHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedMoveHue => {
                let cmd_data : EnhancedMoveHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_move_hue (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedMoveHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedStepHue => {
                let cmd_data : EnhancedStepHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_step_hue (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedStepHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedMoveToHueAndSaturation => {
                let cmd_data : EnhancedMoveToHueAndSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_move_to_hue_and_saturation (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedMoveToHueAndSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ColorLoopSet => {
                let cmd_data : ColorLoopSetRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_color_loop_set (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ColorLoopSet ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StopMoveStep => {
                let cmd_data : StopMoveStepRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_stop_move_step (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StopMoveStep ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveColorTemperature => {
                let cmd_data : MoveColorTemperatureRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_color_temperature (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveColorTemperature ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepColorTemperature => {
                let cmd_data : StepColorTemperatureRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_color_temperature (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepColorTemperature ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 768u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ColorControl" , 768u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ColorControl" , 768u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `ColorControl` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct ColorControlCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > ColorControlCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn move_to_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 0 ) ? . data_builder () 
    }
    pub fn move_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 1 ) ? . data_builder () 
    }
    pub fn step_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < StepHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 2 ) ? . data_builder () 
    }
    pub fn move_to_saturation (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 3 ) ? . data_builder () 
    }
    pub fn move_saturation (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 4 ) ? . data_builder () 
    }
    pub fn step_saturation (self , endpoint : crate :: dm :: EndptId , ) -> Result < StepSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 5 ) ? . data_builder () 
    }
    pub fn move_to_hue_and_saturation (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToHueAndSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 6 ) ? . data_builder () 
    }
    pub fn move_to_color (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToColorRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 7 ) ? . data_builder () 
    }
    pub fn move_color (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveColorRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 8 ) ? . data_builder () 
    }
    pub fn step_color (self , endpoint : crate :: dm :: EndptId , ) -> Result < StepColorRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 9 ) ? . data_builder () 
    }
    pub fn move_to_color_temperature (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveToColorTemperatureRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 10 ) ? . data_builder () 
    }
    pub fn enhanced_move_to_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < EnhancedMoveToHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 64 ) ? . data_builder () 
    }
    pub fn enhanced_move_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < EnhancedMoveHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 65 ) ? . data_builder () 
    }
    pub fn enhanced_step_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < EnhancedStepHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 66 ) ? . data_builder () 
    }
    pub fn enhanced_move_to_hue_and_saturation (self , endpoint : crate :: dm :: EndptId , ) -> Result < EnhancedMoveToHueAndSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 67 ) ? . data_builder () 
    }
    pub fn color_loop_set (self , endpoint : crate :: dm :: EndptId , ) -> Result < ColorLoopSetRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 68 ) ? . data_builder () 
    }
    pub fn stop_move_step (self , endpoint : crate :: dm :: EndptId , ) -> Result < StopMoveStepRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 71 ) ? . data_builder () 
    }
    pub fn move_color_temperature (self , endpoint : crate :: dm :: EndptId , ) -> Result < MoveColorTemperatureRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 75 ) ? . data_builder () 
    }
    pub fn step_color_temperature (self , endpoint : crate :: dm :: EndptId , ) -> Result < StepColorTemperatureRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 768 , 76 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ColorControl` cluster's commands. `use` this trait to call `.color_control_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`ColorControlCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ColorControlCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn color_control_inv (self ) -> ColorControlCmdRequestsView < P > ;
    
}
impl < P > ColorControlCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn color_control_inv (self ) -> ColorControlCmdRequestsView < P > {
        ColorControlCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `ColorControl` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ColorControlAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > ColorControlAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_saturation (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn remaining_time (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn drift_compensation (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn compensation_text (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_temperature_mireds (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_mode (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn options (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (15 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_primaries (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_1_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (17 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_1_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (18 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_1_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (19 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_2_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (21 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_2_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (22 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_2_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (23 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_3_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (25 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_3_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (26 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_3_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (27 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_4_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (32 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_4_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (33 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_4_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (34 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_5_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (36 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_5_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (37 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_5_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (38 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_6_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (40 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_6_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (41 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn primary_6_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (42 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn white_point_x (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (48 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn white_point_y (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (49 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_rx (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (50 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_ry (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (51 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_r_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (52 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_gx (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (54 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_gy (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (55 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_g_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (56 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_bx (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (58 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_by (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (59 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_point_b_intensity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (60 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn enhanced_current_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16384 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn enhanced_color_mode (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16385 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_loop_active (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16386 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_loop_direction (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16387 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_loop_time (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16388 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_loop_start_enhanced_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16389 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_loop_stored_enhanced_hue (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16390 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_capabilities (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16394 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_temp_physical_min_mireds (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16395 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn color_temp_physical_max_mireds (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16396 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn couple_color_temp_to_level_min_mireds (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16397 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn start_up_color_temperature_mireds (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (16400 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (768 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ColorControl` cluster's attribute reads. `use` this trait to call `.color_control_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`ColorControlAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ColorControlAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn color_control_read (self ) -> ColorControlAttrReadsView < P > ;
    
}
impl < P > ColorControlAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn color_control_read (self ) -> ColorControlAttrReadsView < P > {
        ColorControlAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `ColorControl` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ColorControlAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > ColorControlAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn options (self , endpoint : crate :: dm :: EndptId , value : OptionsBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 768 , 15 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn start_up_color_temperature_mireds (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 768 , 16400 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ColorControl` cluster's attribute writes. `use` this trait to call `.color_control_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`ColorControlAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ColorControlAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn color_control_write (self ) -> ColorControlAttrWritesView < P > ;
    
}
impl < P > ColorControlAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn color_control_write (self ) -> ColorControlAttrWritesView < P > {
        ColorControlAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `ColorControl` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct ColorControlCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > ColorControlCmdResponsesView < 'a , 'r > {
    pub fn move_to_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 0 ) 
    }
    pub fn move_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 1 ) 
    }
    pub fn step_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 2 ) 
    }
    pub fn move_to_saturation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 3 ) 
    }
    pub fn move_saturation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 4 ) 
    }
    pub fn step_saturation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 5 ) 
    }
    pub fn move_to_hue_and_saturation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 6 ) 
    }
    pub fn move_to_color (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 7 ) 
    }
    pub fn move_color (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 8 ) 
    }
    pub fn step_color (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 9 ) 
    }
    pub fn move_to_color_temperature (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 10 ) 
    }
    pub fn enhanced_move_to_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 64 ) 
    }
    pub fn enhanced_move_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 65 ) 
    }
    pub fn enhanced_step_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 66 ) 
    }
    pub fn enhanced_move_to_hue_and_saturation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 67 ) 
    }
    pub fn color_loop_set (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 68 ) 
    }
    pub fn stop_move_step (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 71 ) 
    }
    pub fn move_color_temperature (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 75 ) 
    }
    pub fn step_color_temperature (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 76 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `ColorControl`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.color_control_inv_resp()` on an `InvokeResp`; the returned [`ColorControlCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait ColorControlCmdResponses < 'a > {
    fn color_control_inv_resp (& self ) -> ColorControlCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > ColorControlCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn color_control_inv_resp (& self ) -> ColorControlCmdResponsesView < 'a , '_ > {
        ColorControlCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `ColorControl` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ColorControlAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > ColorControlAttrResponsesView < 'a , 'r > {
    pub fn current_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (768 , 0 ) 
    }
    pub fn current_saturation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (768 , 1 ) 
    }
    pub fn remaining_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 2 ) 
    }
    pub fn current_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 3 ) 
    }
    pub fn current_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 4 ) 
    }
    pub fn drift_compensation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < DriftCompensationEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < DriftCompensationEnum > (768 , 5 ) 
    }
    pub fn compensation_text (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Utf8Str < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Utf8Str < 'a > > (768 , 6 ) 
    }
    pub fn color_temperature_mireds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 7 ) 
    }
    pub fn color_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ColorModeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ColorModeEnum > (768 , 8 ) 
    }
    pub fn options (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < OptionsBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < OptionsBitmap > (768 , 15 ) 
    }
    pub fn number_of_primaries (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 16 ) 
    }
    pub fn primary_1_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 17 ) 
    }
    pub fn primary_1_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 18 ) 
    }
    pub fn primary_1_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 19 ) 
    }
    pub fn primary_2_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 21 ) 
    }
    pub fn primary_2_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 22 ) 
    }
    pub fn primary_2_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 23 ) 
    }
    pub fn primary_3_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 25 ) 
    }
    pub fn primary_3_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 26 ) 
    }
    pub fn primary_3_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 27 ) 
    }
    pub fn primary_4_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 32 ) 
    }
    pub fn primary_4_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 33 ) 
    }
    pub fn primary_4_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 34 ) 
    }
    pub fn primary_5_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 36 ) 
    }
    pub fn primary_5_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 37 ) 
    }
    pub fn primary_5_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 38 ) 
    }
    pub fn primary_6_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 40 ) 
    }
    pub fn primary_6_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 41 ) 
    }
    pub fn primary_6_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 42 ) 
    }
    pub fn white_point_x (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 48 ) 
    }
    pub fn white_point_y (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 49 ) 
    }
    pub fn color_point_rx (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 50 ) 
    }
    pub fn color_point_ry (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 51 ) 
    }
    pub fn color_point_r_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 52 ) 
    }
    pub fn color_point_gx (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 54 ) 
    }
    pub fn color_point_gy (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 55 ) 
    }
    pub fn color_point_g_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 56 ) 
    }
    pub fn color_point_bx (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 58 ) 
    }
    pub fn color_point_by (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 59 ) 
    }
    pub fn color_point_b_intensity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (768 , 60 ) 
    }
    pub fn enhanced_current_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 16384 ) 
    }
    pub fn enhanced_color_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < EnhancedColorModeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < EnhancedColorModeEnum > (768 , 16385 ) 
    }
    pub fn color_loop_active (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (768 , 16386 ) 
    }
    pub fn color_loop_direction (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (768 , 16387 ) 
    }
    pub fn color_loop_time (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 16388 ) 
    }
    pub fn color_loop_start_enhanced_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 16389 ) 
    }
    pub fn color_loop_stored_enhanced_hue (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 16390 ) 
    }
    pub fn color_capabilities (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ColorCapabilitiesBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ColorCapabilitiesBitmap > (768 , 16394 ) 
    }
    pub fn color_temp_physical_min_mireds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 16395 ) 
    }
    pub fn color_temp_physical_max_mireds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 16396 ) 
    }
    pub fn couple_color_temp_to_level_min_mireds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 16397 ) 
    }
    pub fn start_up_color_temperature_mireds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (768 , 16400 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (768 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (768 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (768 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (768 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (768 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `ColorControl`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.color_control_read_resp()` on a `ReportDataResp`; the returned [`ColorControlAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait ColorControlAttrResponses < 'a > {
    fn color_control_read_resp (& self ) -> ColorControlAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > ColorControlAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn color_control_read_resp (& self ) -> ColorControlAttrResponsesView < 'a , '_ > {
        ColorControlAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `ColorControl` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ColorControlWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > ColorControlWriteResponsesView < 'a , 'r > {
    pub fn options (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 15 ) 
    }
    pub fn start_up_color_temperature_mireds (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (768 , 16400 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `ColorControl`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.color_control_write_resp()` on a `WriteResp`; the returned [`ColorControlWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait ColorControlWriteResponses < 'a > {
    fn color_control_write_resp (& self ) -> ColorControlWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > ColorControlWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn color_control_write_resp (& self ) -> ColorControlWriteResponsesView < 'a , '_ > {
        ColorControlWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `ColorControl` cluster's client operations. Returned by [`ColorControlClient::color_control`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct ColorControlClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > ColorControlClientView < T > {
    pub fn move_to_hue < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_to_hue (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_hue < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_hue (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn step_hue < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StepHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . step_hue (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_to_saturation < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_to_saturation (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_saturation < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_saturation (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn step_saturation < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StepSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . step_saturation (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_to_hue_and_saturation < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToHueAndSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_to_hue_and_saturation (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_to_color < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToColorRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_to_color (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_color < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveColorRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_color (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn step_color < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StepColorRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . step_color (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_to_color_temperature < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveToColorTemperatureRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_to_color_temperature (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn enhanced_move_to_hue < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (EnhancedMoveToHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . enhanced_move_to_hue (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn enhanced_move_hue < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (EnhancedMoveHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . enhanced_move_hue (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn enhanced_step_hue < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (EnhancedStepHueRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . enhanced_step_hue (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn enhanced_move_to_hue_and_saturation < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (EnhancedMoveToHueAndSaturationRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . enhanced_move_to_hue_and_saturation (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn color_loop_set < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (ColorLoopSetRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . color_loop_set (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn stop_move_step < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StopMoveStepRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . stop_move_step (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn move_color_temperature < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (MoveColorTemperatureRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . move_color_temperature (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn step_color_temperature < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StepColorTemperatureRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ColorControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . color_control_inv () ;
            let req_builder = view . step_color_temperature (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn current_hue_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . current_hue (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn current_saturation_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . current_saturation (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn remaining_time_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . remaining_time (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn current_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . current_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn current_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . current_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn drift_compensation_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < DriftCompensationEnum , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . drift_compensation (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn compensation_text_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . compensation_text (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn color_temperature_mireds_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_temperature_mireds (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_mode_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ColorModeEnum , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_mode (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn options_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < OptionsBitmap , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . options (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn number_of_primaries_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . number_of_primaries (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_1_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_1_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_1_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_1_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_1_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_1_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_2_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_2_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_2_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_2_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_2_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_2_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_3_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_3_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_3_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_3_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_3_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_3_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_4_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_4_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_4_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_4_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_4_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_4_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_5_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_5_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_5_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_5_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_5_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_5_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_6_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_6_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_6_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_6_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn primary_6_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . primary_6_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn white_point_x_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . white_point_x (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn white_point_y_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . white_point_y (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_rx_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_rx (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_ry_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_ry (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_r_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_r_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_gx_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_gx (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_gy_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_gy (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_g_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_g_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_bx_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_bx (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_by_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_by (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_point_b_intensity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_point_b_intensity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn enhanced_current_hue_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . enhanced_current_hue (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn enhanced_color_mode_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < EnhancedColorModeEnum , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . enhanced_color_mode (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_loop_active_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_loop_active (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_loop_direction_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_loop_direction (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_loop_time_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_loop_time (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_loop_start_enhanced_hue_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_loop_start_enhanced_hue (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_loop_stored_enhanced_hue_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_loop_stored_enhanced_hue (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_capabilities_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ColorCapabilitiesBitmap , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_capabilities (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_temp_physical_min_mireds_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_temp_physical_min_mireds (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn color_temp_physical_max_mireds_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . color_temp_physical_max_mireds (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn couple_color_temp_to_level_min_mireds_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . couple_color_temp_to_level_min_mireds (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn start_up_color_temperature_mireds_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . start_up_color_temperature_mireds (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ColorControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . color_control_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn options_write (self , endpoint : crate :: dm :: EndptId , value : OptionsBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ColorControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . color_control_write () . options (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn start_up_color_temperature_mireds_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u16 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: ColorControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . color_control_write () . start_up_color_temperature_mireds (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `ColorControl` cluster. `use` this trait to call `.color_control()` on any [`crate::im::ImClient`], such as an exchange; the returned [`ColorControlClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `ColorControlClientView` keeps each cluster's API surface narrow: at the call site `exchange.color_control().` shows only this cluster's methods in IDE completion." ] pub trait ColorControlClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn color_control (self ) -> ColorControlClientView < Self > {
        ColorControlClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > ColorControlClient for T {
    
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
    fn current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn current_saturation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn current_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn current_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn drift_compensation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < DriftCompensationEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn compensation_text < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ColorModeEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < OptionsBitmap , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn number_of_primaries (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn primary_1_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_1_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_1_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_2_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_2_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_2_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_3_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_3_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_3_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_4_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_4_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_4_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_5_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_5_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_5_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_6_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_6_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn primary_6_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn white_point_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn white_point_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_rx (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_ry (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_r_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_gx (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_gy (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_g_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_bx (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_by (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_point_b_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn enhanced_current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn enhanced_color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < EnhancedColorModeEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn color_loop_active (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_loop_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_loop_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_loop_start_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_loop_stored_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_capabilities (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ColorCapabilitiesBitmap , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn color_temp_physical_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn color_temp_physical_max_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn couple_color_temp_to_level_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : StepHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_to_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_step_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : StepSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueAndSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_to_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_step_color (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_to_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorTemperatureRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_enhanced_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_enhanced_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_enhanced_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedStepHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_enhanced_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueAndSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_color_loop_set (& self , ctx : impl crate :: dm :: InvokeContext , request : ColorLoopSetRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_stop_move_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StopMoveStepRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_move_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorTemperatureRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_step_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorTemperatureRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: current_hue (self , ctx ) 
    }
    fn current_saturation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: current_saturation (self , ctx ) 
    }
    fn remaining_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: remaining_time (self , ctx ) 
    }
    fn current_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: current_x (self , ctx ) 
    }
    fn current_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: current_y (self , ctx ) 
    }
    fn drift_compensation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < DriftCompensationEnum , crate :: error :: Error >> {
        T :: drift_compensation (self , ctx ) 
    }
    fn compensation_text < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: compensation_text (self , ctx , builder ) 
    }
    fn color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_temperature_mireds (self , ctx ) 
    }
    fn color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ColorModeEnum , crate :: error :: Error >> {
        T :: color_mode (self , ctx ) 
    }
    fn options (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < OptionsBitmap , crate :: error :: Error >> {
        T :: options (self , ctx ) 
    }
    fn number_of_primaries (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: number_of_primaries (self , ctx ) 
    }
    fn primary_1_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_1_x (self , ctx ) 
    }
    fn primary_1_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_1_y (self , ctx ) 
    }
    fn primary_1_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: primary_1_intensity (self , ctx ) 
    }
    fn primary_2_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_2_x (self , ctx ) 
    }
    fn primary_2_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_2_y (self , ctx ) 
    }
    fn primary_2_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: primary_2_intensity (self , ctx ) 
    }
    fn primary_3_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_3_x (self , ctx ) 
    }
    fn primary_3_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_3_y (self , ctx ) 
    }
    fn primary_3_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: primary_3_intensity (self , ctx ) 
    }
    fn primary_4_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_4_x (self , ctx ) 
    }
    fn primary_4_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_4_y (self , ctx ) 
    }
    fn primary_4_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: primary_4_intensity (self , ctx ) 
    }
    fn primary_5_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_5_x (self , ctx ) 
    }
    fn primary_5_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_5_y (self , ctx ) 
    }
    fn primary_5_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: primary_5_intensity (self , ctx ) 
    }
    fn primary_6_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_6_x (self , ctx ) 
    }
    fn primary_6_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: primary_6_y (self , ctx ) 
    }
    fn primary_6_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: primary_6_intensity (self , ctx ) 
    }
    fn white_point_x (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: white_point_x (self , ctx ) 
    }
    fn white_point_y (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: white_point_y (self , ctx ) 
    }
    fn color_point_rx (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_point_rx (self , ctx ) 
    }
    fn color_point_ry (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_point_ry (self , ctx ) 
    }
    fn color_point_r_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: color_point_r_intensity (self , ctx ) 
    }
    fn color_point_gx (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_point_gx (self , ctx ) 
    }
    fn color_point_gy (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_point_gy (self , ctx ) 
    }
    fn color_point_g_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: color_point_g_intensity (self , ctx ) 
    }
    fn color_point_bx (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_point_bx (self , ctx ) 
    }
    fn color_point_by (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_point_by (self , ctx ) 
    }
    fn color_point_b_intensity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: color_point_b_intensity (self , ctx ) 
    }
    fn enhanced_current_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: enhanced_current_hue (self , ctx ) 
    }
    fn enhanced_color_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < EnhancedColorModeEnum , crate :: error :: Error >> {
        T :: enhanced_color_mode (self , ctx ) 
    }
    fn color_loop_active (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: color_loop_active (self , ctx ) 
    }
    fn color_loop_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: color_loop_direction (self , ctx ) 
    }
    fn color_loop_time (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_loop_time (self , ctx ) 
    }
    fn color_loop_start_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_loop_start_enhanced_hue (self , ctx ) 
    }
    fn color_loop_stored_enhanced_hue (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_loop_stored_enhanced_hue (self , ctx ) 
    }
    fn color_capabilities (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ColorCapabilitiesBitmap , crate :: error :: Error >> {
        T :: color_capabilities (self , ctx ) 
    }
    fn color_temp_physical_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_temp_physical_min_mireds (self , ctx ) 
    }
    fn color_temp_physical_max_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: color_temp_physical_max_mireds (self , ctx ) 
    }
    fn couple_color_temp_to_level_min_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: couple_color_temp_to_level_min_mireds (self , ctx ) 
    }
    fn start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: start_up_color_temperature_mireds (self , ctx ) 
    }
    fn set_options (& self , ctx : impl crate :: dm :: WriteContext , value : OptionsBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_options (self , ctx , value ) 
    }
    fn set_start_up_color_temperature_mireds (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u16 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_start_up_color_temperature_mireds (self , ctx , value ) 
    }
    fn handle_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_hue (self , ctx , request ) 
    }
    fn handle_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_hue (self , ctx , request ) 
    }
    fn handle_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : StepHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_step_hue (self , ctx , request ) 
    }
    fn handle_move_to_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_saturation (self , ctx , request ) 
    }
    fn handle_move_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_saturation (self , ctx , request ) 
    }
    fn handle_step_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : StepSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_step_saturation (self , ctx , request ) 
    }
    fn handle_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToHueAndSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_hue_and_saturation (self , ctx , request ) 
    }
    fn handle_move_to_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_color (self , ctx , request ) 
    }
    fn handle_move_color (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_color (self , ctx , request ) 
    }
    fn handle_step_color (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_step_color (self , ctx , request ) 
    }
    fn handle_move_to_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveToColorTemperatureRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_to_color_temperature (self , ctx , request ) 
    }
    fn handle_enhanced_move_to_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_enhanced_move_to_hue (self , ctx , request ) 
    }
    fn handle_enhanced_move_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_enhanced_move_hue (self , ctx , request ) 
    }
    fn handle_enhanced_step_hue (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedStepHueRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_enhanced_step_hue (self , ctx , request ) 
    }
    fn handle_enhanced_move_to_hue_and_saturation (& self , ctx : impl crate :: dm :: InvokeContext , request : EnhancedMoveToHueAndSaturationRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_enhanced_move_to_hue_and_saturation (self , ctx , request ) 
    }
    fn handle_color_loop_set (& self , ctx : impl crate :: dm :: InvokeContext , request : ColorLoopSetRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_color_loop_set (self , ctx , request ) 
    }
    fn handle_stop_move_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StopMoveStepRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_stop_move_step (self , ctx , request ) 
    }
    fn handle_move_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : MoveColorTemperatureRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_move_color_temperature (self , ctx , request ) 
    }
    fn handle_step_color_temperature (& self , ctx : impl crate :: dm :: InvokeContext , request : StepColorTemperatureRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_step_color_temperature (self , ctx , request ) 
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
                    AttributeId :: CurrentHue => {
                        let attr_read_result = self . 0. current_hue (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentSaturation => {
                        let attr_read_result = self . 0. current_saturation (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSaturation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RemainingTime => {
                        let attr_read_result = self . 0. remaining_time (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RemainingTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentX => {
                        let attr_read_result = self . 0. current_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentY => {
                        let attr_read_result = self . 0. current_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: DriftCompensation => {
                        let attr_read_result = self . 0. drift_compensation (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DriftCompensation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CompensationText => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CompensationText , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. compensation_text (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CompensationText , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CompensationText , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: ColorTemperatureMireds => {
                        let attr_read_result = self . 0. color_temperature_mireds (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorTemperatureMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorMode => {
                        let attr_read_result = self . 0. color_mode (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Options => {
                        let attr_read_result = self . 0. options (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Options , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfPrimaries => {
                        let attr_read_result = self . 0. number_of_primaries (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfPrimaries , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary1X => {
                        let attr_read_result = self . 0. primary_1_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary1X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary1Y => {
                        let attr_read_result = self . 0. primary_1_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary1Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary1Intensity => {
                        let attr_read_result = self . 0. primary_1_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary1Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary2X => {
                        let attr_read_result = self . 0. primary_2_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary2X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary2Y => {
                        let attr_read_result = self . 0. primary_2_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary2Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary2Intensity => {
                        let attr_read_result = self . 0. primary_2_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary2Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary3X => {
                        let attr_read_result = self . 0. primary_3_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary3X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary3Y => {
                        let attr_read_result = self . 0. primary_3_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary3Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary3Intensity => {
                        let attr_read_result = self . 0. primary_3_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary3Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary4X => {
                        let attr_read_result = self . 0. primary_4_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary4X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary4Y => {
                        let attr_read_result = self . 0. primary_4_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary4Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary4Intensity => {
                        let attr_read_result = self . 0. primary_4_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary4Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary5X => {
                        let attr_read_result = self . 0. primary_5_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary5X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary5Y => {
                        let attr_read_result = self . 0. primary_5_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary5Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary5Intensity => {
                        let attr_read_result = self . 0. primary_5_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary5Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary6X => {
                        let attr_read_result = self . 0. primary_6_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary6X , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary6Y => {
                        let attr_read_result = self . 0. primary_6_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary6Y , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Primary6Intensity => {
                        let attr_read_result = self . 0. primary_6_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Primary6Intensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WhitePointX => {
                        let attr_read_result = self . 0. white_point_x (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WhitePointX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WhitePointY => {
                        let attr_read_result = self . 0. white_point_y (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WhitePointY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointRX => {
                        let attr_read_result = self . 0. color_point_rx (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointRX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointRY => {
                        let attr_read_result = self . 0. color_point_ry (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointRY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointRIntensity => {
                        let attr_read_result = self . 0. color_point_r_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointRIntensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointGX => {
                        let attr_read_result = self . 0. color_point_gx (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointGX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointGY => {
                        let attr_read_result = self . 0. color_point_gy (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointGY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointGIntensity => {
                        let attr_read_result = self . 0. color_point_g_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointGIntensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointBX => {
                        let attr_read_result = self . 0. color_point_bx (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointBX , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointBY => {
                        let attr_read_result = self . 0. color_point_by (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointBY , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorPointBIntensity => {
                        let attr_read_result = self . 0. color_point_b_intensity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorPointBIntensity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: EnhancedCurrentHue => {
                        let attr_read_result = self . 0. enhanced_current_hue (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EnhancedCurrentHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: EnhancedColorMode => {
                        let attr_read_result = self . 0. enhanced_color_mode (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EnhancedColorMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopActive => {
                        let attr_read_result = self . 0. color_loop_active (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopActive , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopDirection => {
                        let attr_read_result = self . 0. color_loop_direction (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopDirection , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopTime => {
                        let attr_read_result = self . 0. color_loop_time (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopTime , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopStartEnhancedHue => {
                        let attr_read_result = self . 0. color_loop_start_enhanced_hue (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopStartEnhancedHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorLoopStoredEnhancedHue => {
                        let attr_read_result = self . 0. color_loop_stored_enhanced_hue (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorLoopStoredEnhancedHue , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorCapabilities => {
                        let attr_read_result = self . 0. color_capabilities (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorCapabilities , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorTempPhysicalMinMireds => {
                        let attr_read_result = self . 0. color_temp_physical_min_mireds (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorTempPhysicalMinMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ColorTempPhysicalMaxMireds => {
                        let attr_read_result = self . 0. color_temp_physical_max_mireds (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ColorTempPhysicalMaxMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CoupleColorTempToLevelMinMireds => {
                        let attr_read_result = self . 0. couple_color_temp_to_level_min_mireds (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CoupleColorTempToLevelMinMireds , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StartUpColorTemperatureMireds => {
                        let attr_read_result = self . 0. start_up_color_temperature_mireds (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpColorTemperatureMireds , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: StartUpColorTemperatureMireds => {
                let attr_data : crate :: tlv :: Nullable < u16 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_start_up_color_temperature_mireds (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StartUpColorTemperatureMireds , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: MoveToHue => {
                let cmd_data : MoveToHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_hue (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveHue => {
                let cmd_data : MoveHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_hue (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepHue => {
                let cmd_data : StepHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_hue (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToSaturation => {
                let cmd_data : MoveToSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_saturation (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveSaturation => {
                let cmd_data : MoveSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_saturation (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepSaturation => {
                let cmd_data : StepSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_saturation (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToHueAndSaturation => {
                let cmd_data : MoveToHueAndSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_hue_and_saturation (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToHueAndSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToColor => {
                let cmd_data : MoveToColorRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_color (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToColor ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveColor => {
                let cmd_data : MoveColorRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_color (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveColor ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepColor => {
                let cmd_data : StepColorRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_color (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepColor ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveToColorTemperature => {
                let cmd_data : MoveToColorTemperatureRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_to_color_temperature (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveToColorTemperature ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedMoveToHue => {
                let cmd_data : EnhancedMoveToHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_move_to_hue (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedMoveToHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedMoveHue => {
                let cmd_data : EnhancedMoveHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_move_hue (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedMoveHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedStepHue => {
                let cmd_data : EnhancedStepHueRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_step_hue (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedStepHue ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EnhancedMoveToHueAndSaturation => {
                let cmd_data : EnhancedMoveToHueAndSaturationRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_enhanced_move_to_hue_and_saturation (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EnhancedMoveToHueAndSaturation ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ColorLoopSet => {
                let cmd_data : ColorLoopSetRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_color_loop_set (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ColorLoopSet ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StopMoveStep => {
                let cmd_data : StopMoveStepRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_stop_move_step (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StopMoveStep ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: MoveColorTemperature => {
                let cmd_data : MoveColorTemperatureRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_move_color_temperature (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: MoveColorTemperature ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: StepColorTemperature => {
                let cmd_data : StepColorTemperatureRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step_color_temperature (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: StepColorTemperature ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 768u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ColorControl" , 768u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ColorControl" , 768u32 , self . 0.2 ) 
    }
    
}
