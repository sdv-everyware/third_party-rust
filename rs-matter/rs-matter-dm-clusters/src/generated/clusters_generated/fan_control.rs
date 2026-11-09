# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const MULTI_SPEED = 1 ;
        const AUTO = 2 ;
        const ROCKING = 4 ;
        const WIND = 8 ;
        const STEP = 16 ;
        const AIRFLOW_DIRECTION = 32 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const MULTI_SPEED = 1 ;
        const AUTO = 2 ;
        const ROCKING = 4 ;
        const WIND = 8 ;
        const STEP = 16 ;
        const AIRFLOW_DIRECTION = 32 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct RockBitmap : u8 {
        const ROCK_LEFT_RIGHT = 1 ;
        const ROCK_UP_DOWN = 2 ;
        const ROCK_ROUND = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct RockBitmap : u8 {
        const ROCK_LEFT_RIGHT = 1 ;
        const ROCK_UP_DOWN = 2 ;
        const ROCK_ROUND = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (RockBitmap , u8 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct WindBitmap : u8 {
        const SLEEP_WIND = 1 ;
        const NATURAL_WIND = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct WindBitmap : u8 {
        const SLEEP_WIND = 1 ;
        const NATURAL_WIND = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (WindBitmap , u8 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AirflowDirectionEnum {
    # [enumval (0 ) ] Forward = 0 , # [enumval (1 ) ] Reverse = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum FanModeEnum {
    # [enumval (0 ) ] Off = 0 , # [enumval (1 ) ] Low = 1 , # [enumval (2 ) ] Medium = 2 , # [enumval (3 ) ] High = 3 , # [enumval (4 ) ] On = 4 , # [enumval (5 ) ] Auto = 5 , # [enumval (6 ) ] Smart = 6 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum FanModeSequenceEnum {
    # [enumval (0 ) ] OffLowMedHigh = 0 , # [enumval (1 ) ] OffLowHigh = 1 , # [enumval (2 ) ] OffLowMedHighAuto = 2 , # [enumval (3 ) ] OffLowHighAuto = 3 , # [enumval (4 ) ] OffHighAuto = 4 , # [enumval (5 ) ] OffHigh = 5 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepDirectionEnum {
    # [enumval (0 ) ] Increase = 0 , # [enumval (1 ) ] Decrease = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StepRequestTag {
    Direction = 0 , Wrap = 1 , LowestOff = 2 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StepRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StepRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn direction (& self ) -> Result < StepDirectionEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn wrap (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn lowest_off (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
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
        match self . direction () {
            Ok (value ) => write ! (f , "{}: {:?}," , "direction" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) ? , 
        }
        match self . wrap () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "wrap" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "wrap" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "wrap" , e . code () ) ? , 
        }
        match self . lowest_off () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "lowest_off" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "lowest_off" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "lowest_off" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StepRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StepRequest" ) ;
        match self . direction () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "direction" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "direction" , e . code () ) , 
        }
        match self . wrap () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "wrap" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "wrap" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "wrap" , e . code () ) , 
        }
        match self . lowest_off () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "lowest_off" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "lowest_off" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "lowest_off" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
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
    pub fn direction (mut self , value : StepDirectionEnum ) -> Result < StepRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "direction" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn wrap (mut self , value : Option < bool > ) -> Result < StepRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "wrap" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn lowest_off (mut self , value : Option < bool > ) -> Result < StepRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "lowestOff" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StepRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > StepRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
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
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    FanMode = 0 , FanModeSequence = 1 , PercentSetting = 2 , PercentCurrent = 3 , SpeedMax = 4 , SpeedSetting = 5 , SpeedCurrent = 6 , RockSupport = 7 , RockSetting = 8 , WindSupport = 9 , WindSetting = 10 , AirflowDirection = 11 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: FanMode => write ! (f , "{}(0x{:02x})" , "FanMode" , AttributeId :: FanMode as u32 ) ? , AttributeId :: FanModeSequence => write ! (f , "{}(0x{:02x})" , "FanModeSequence" , AttributeId :: FanModeSequence as u32 ) ? , AttributeId :: PercentSetting => write ! (f , "{}(0x{:02x})" , "PercentSetting" , AttributeId :: PercentSetting as u32 ) ? , AttributeId :: PercentCurrent => write ! (f , "{}(0x{:02x})" , "PercentCurrent" , AttributeId :: PercentCurrent as u32 ) ? , AttributeId :: SpeedMax => write ! (f , "{}(0x{:02x})" , "SpeedMax" , AttributeId :: SpeedMax as u32 ) ? , AttributeId :: SpeedSetting => write ! (f , "{}(0x{:02x})" , "SpeedSetting" , AttributeId :: SpeedSetting as u32 ) ? , AttributeId :: SpeedCurrent => write ! (f , "{}(0x{:02x})" , "SpeedCurrent" , AttributeId :: SpeedCurrent as u32 ) ? , AttributeId :: RockSupport => write ! (f , "{}(0x{:02x})" , "RockSupport" , AttributeId :: RockSupport as u32 ) ? , AttributeId :: RockSetting => write ! (f , "{}(0x{:02x})" , "RockSetting" , AttributeId :: RockSetting as u32 ) ? , AttributeId :: WindSupport => write ! (f , "{}(0x{:02x})" , "WindSupport" , AttributeId :: WindSupport as u32 ) ? , AttributeId :: WindSetting => write ! (f , "{}(0x{:02x})" , "WindSetting" , AttributeId :: WindSetting as u32 ) ? , AttributeId :: AirflowDirection => write ! (f , "{}(0x{:02x})" , "AirflowDirection" , AttributeId :: AirflowDirection as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: FanMode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FanMode" , AttributeId :: FanMode as u32 ) , AttributeId :: FanModeSequence => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FanModeSequence" , AttributeId :: FanModeSequence as u32 ) , AttributeId :: PercentSetting => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PercentSetting" , AttributeId :: PercentSetting as u32 ) , AttributeId :: PercentCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PercentCurrent" , AttributeId :: PercentCurrent as u32 ) , AttributeId :: SpeedMax => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeedMax" , AttributeId :: SpeedMax as u32 ) , AttributeId :: SpeedSetting => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeedSetting" , AttributeId :: SpeedSetting as u32 ) , AttributeId :: SpeedCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeedCurrent" , AttributeId :: SpeedCurrent as u32 ) , AttributeId :: RockSupport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RockSupport" , AttributeId :: RockSupport as u32 ) , AttributeId :: RockSetting => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RockSetting" , AttributeId :: RockSetting as u32 ) , AttributeId :: WindSupport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WindSupport" , AttributeId :: WindSupport as u32 ) , AttributeId :: WindSetting => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WindSetting" , AttributeId :: WindSetting as u32 ) , AttributeId :: AirflowDirection => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AirflowDirection" , AttributeId :: AirflowDirection as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    Step = 0 
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
            CommandId :: Step => write ! (f , "{}(0x{:02x})" , "Step" , CommandId :: Step as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: Step => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Step" , CommandId :: Step as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (514 , 5 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: FanMode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: FanModeSequence as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: PercentSetting as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: PercentCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: SpeedMax as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SpeedSetting as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SpeedCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RockSupport as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RockSetting as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WindSupport as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WindSetting as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: AirflowDirection as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: Step as _ , None , crate :: dm :: Access :: WO , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn fan_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < FanModeEnum , crate :: error :: Error > ;
    fn fan_mode_sequence (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < FanModeSequenceEnum , crate :: error :: Error > ;
    fn percent_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: Percent > , crate :: error :: Error > ;
    fn percent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: im :: Percent , crate :: error :: Error > ;
    fn speed_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn speed_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn speed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rock_support (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RockBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rock_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RockBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wind_support (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < WindBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wind_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < WindBitmap , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn airflow_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < AirflowDirectionEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_fan_mode (& self , ctx : impl crate :: dm :: WriteContext , value : FanModeEnum ) -> Result < () , crate :: error :: Error > ;
    fn set_percent_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < crate :: im :: Percent > ) -> Result < () , crate :: error :: Error > ;
    fn set_speed_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_rock_setting (& self , ctx : impl crate :: dm :: WriteContext , value : RockBitmap ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_wind_setting (& self , ctx : impl crate :: dm :: WriteContext , value : WindBitmap ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_airflow_direction (& self , ctx : impl crate :: dm :: WriteContext , value : AirflowDirectionEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn fan_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < FanModeEnum , crate :: error :: Error > {
        T :: fan_mode (self , ctx ) 
    }
    fn fan_mode_sequence (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < FanModeSequenceEnum , crate :: error :: Error > {
        T :: fan_mode_sequence (self , ctx ) 
    }
    fn percent_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: Percent > , crate :: error :: Error > {
        T :: percent_setting (self , ctx ) 
    }
    fn percent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: im :: Percent , crate :: error :: Error > {
        T :: percent_current (self , ctx ) 
    }
    fn speed_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: speed_max (self , ctx ) 
    }
    fn speed_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: speed_setting (self , ctx ) 
    }
    fn speed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: speed_current (self , ctx ) 
    }
    fn rock_support (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RockBitmap , crate :: error :: Error > {
        T :: rock_support (self , ctx ) 
    }
    fn rock_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < RockBitmap , crate :: error :: Error > {
        T :: rock_setting (self , ctx ) 
    }
    fn wind_support (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < WindBitmap , crate :: error :: Error > {
        T :: wind_support (self , ctx ) 
    }
    fn wind_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < WindBitmap , crate :: error :: Error > {
        T :: wind_setting (self , ctx ) 
    }
    fn airflow_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < AirflowDirectionEnum , crate :: error :: Error > {
        T :: airflow_direction (self , ctx ) 
    }
    fn set_fan_mode (& self , ctx : impl crate :: dm :: WriteContext , value : FanModeEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_fan_mode (self , ctx , value ) 
    }
    fn set_percent_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < crate :: im :: Percent > ) -> Result < () , crate :: error :: Error > {
        T :: set_percent_setting (self , ctx , value ) 
    }
    fn set_speed_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> Result < () , crate :: error :: Error > {
        T :: set_speed_setting (self , ctx , value ) 
    }
    fn set_rock_setting (& self , ctx : impl crate :: dm :: WriteContext , value : RockBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_rock_setting (self , ctx , value ) 
    }
    fn set_wind_setting (& self , ctx : impl crate :: dm :: WriteContext , value : WindBitmap ) -> Result < () , crate :: error :: Error > {
        T :: set_wind_setting (self , ctx , value ) 
    }
    fn set_airflow_direction (& self , ctx : impl crate :: dm :: WriteContext , value : AirflowDirectionEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_airflow_direction (self , ctx , value ) 
    }
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_step (self , ctx , request ) 
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
                    AttributeId :: FanMode => {
                        let attr_read_result = self . 0. fan_mode (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FanMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: FanModeSequence => {
                        let attr_read_result = self . 0. fan_mode_sequence (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FanModeSequence , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PercentSetting => {
                        let attr_read_result = self . 0. percent_setting (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PercentSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PercentCurrent => {
                        let attr_read_result = self . 0. percent_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PercentCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeedMax => {
                        let attr_read_result = self . 0. speed_max (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedMax , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeedSetting => {
                        let attr_read_result = self . 0. speed_setting (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeedCurrent => {
                        let attr_read_result = self . 0. speed_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RockSupport => {
                        let attr_read_result = self . 0. rock_support (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RockSupport , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RockSetting => {
                        let attr_read_result = self . 0. rock_setting (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RockSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WindSupport => {
                        let attr_read_result = self . 0. wind_support (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WindSupport , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WindSetting => {
                        let attr_read_result = self . 0. wind_setting (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WindSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AirflowDirection => {
                        let attr_read_result = self . 0. airflow_direction (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AirflowDirection , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: FanMode => {
                let attr_data : FanModeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_fan_mode (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FanMode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: PercentSetting => {
                let attr_data : crate :: tlv :: Nullable < crate :: im :: Percent > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_percent_setting (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PercentSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SpeedSetting => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_speed_setting (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: RockSetting => {
                let attr_data : RockBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_rock_setting (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RockSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: WindSetting => {
                let attr_data : WindBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_wind_setting (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WindSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: AirflowDirection => {
                let attr_data : AirflowDirectionEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_airflow_direction (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AirflowDirection , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: Step => {
                let cmd_data : StepRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Step ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 514u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "FanControl" , 514u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "FanControl" , 514u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `FanControl` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct FanControlCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > FanControlCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn step (self , endpoint : crate :: dm :: EndptId , ) -> Result < StepRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 514 , 0 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `FanControl` cluster's commands. `use` this trait to call `.fan_control_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`FanControlCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait FanControlCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn fan_control_inv (self ) -> FanControlCmdRequestsView < P > ;
    
}
impl < P > FanControlCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn fan_control_inv (self ) -> FanControlCmdRequestsView < P > {
        FanControlCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `FanControl` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct FanControlAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > FanControlAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fan_mode (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn fan_mode_sequence (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn percent_setting (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn percent_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speed_max (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speed_setting (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speed_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rock_support (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rock_setting (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wind_support (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wind_setting (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn airflow_direction (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (11 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (514 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `FanControl` cluster's attribute reads. `use` this trait to call `.fan_control_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`FanControlAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait FanControlAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn fan_control_read (self ) -> FanControlAttrReadsView < P > ;
    
}
impl < P > FanControlAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn fan_control_read (self ) -> FanControlAttrReadsView < P > {
        FanControlAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `FanControl` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct FanControlAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > FanControlAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fan_mode (self , endpoint : crate :: dm :: EndptId , value : FanModeEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 514 , 0 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn percent_setting (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < crate :: im :: Percent > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 514 , 2 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speed_setting (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 514 , 5 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rock_setting (self , endpoint : crate :: dm :: EndptId , value : RockBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 514 , 8 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wind_setting (self , endpoint : crate :: dm :: EndptId , value : WindBitmap , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 514 , 10 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn airflow_direction (self , endpoint : crate :: dm :: EndptId , value : AirflowDirectionEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 514 , 11 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `FanControl` cluster's attribute writes. `use` this trait to call `.fan_control_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`FanControlAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait FanControlAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn fan_control_write (self ) -> FanControlAttrWritesView < P > ;
    
}
impl < P > FanControlAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn fan_control_write (self ) -> FanControlAttrWritesView < P > {
        FanControlAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `FanControl` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct FanControlCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > FanControlCmdResponsesView < 'a , 'r > {
    pub fn step (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (514 , 0 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `FanControl`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.fan_control_inv_resp()` on an `InvokeResp`; the returned [`FanControlCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait FanControlCmdResponses < 'a > {
    fn fan_control_inv_resp (& self ) -> FanControlCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > FanControlCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn fan_control_inv_resp (& self ) -> FanControlCmdResponsesView < 'a , '_ > {
        FanControlCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `FanControl` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct FanControlAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > FanControlAttrResponsesView < 'a , 'r > {
    pub fn fan_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < FanModeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < FanModeEnum > (514 , 0 ) 
    }
    pub fn fan_mode_sequence (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < FanModeSequenceEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < FanModeSequenceEnum > (514 , 1 ) 
    }
    pub fn percent_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: Percent > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: Percent > > (514 , 2 ) 
    }
    pub fn percent_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: im :: Percent , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: im :: Percent > (514 , 3 ) 
    }
    pub fn speed_max (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (514 , 4 ) 
    }
    pub fn speed_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (514 , 5 ) 
    }
    pub fn speed_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (514 , 6 ) 
    }
    pub fn rock_support (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < RockBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < RockBitmap > (514 , 7 ) 
    }
    pub fn rock_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < RockBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < RockBitmap > (514 , 8 ) 
    }
    pub fn wind_support (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < WindBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < WindBitmap > (514 , 9 ) 
    }
    pub fn wind_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < WindBitmap , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < WindBitmap > (514 , 10 ) 
    }
    pub fn airflow_direction (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AirflowDirectionEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < AirflowDirectionEnum > (514 , 11 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (514 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (514 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (514 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (514 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (514 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `FanControl`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.fan_control_read_resp()` on a `ReportDataResp`; the returned [`FanControlAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait FanControlAttrResponses < 'a > {
    fn fan_control_read_resp (& self ) -> FanControlAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > FanControlAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn fan_control_read_resp (& self ) -> FanControlAttrResponsesView < 'a , '_ > {
        FanControlAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `FanControl` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct FanControlWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > FanControlWriteResponsesView < 'a , 'r > {
    pub fn fan_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (514 , 0 ) 
    }
    pub fn percent_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (514 , 2 ) 
    }
    pub fn speed_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (514 , 5 ) 
    }
    pub fn rock_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (514 , 8 ) 
    }
    pub fn wind_setting (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (514 , 10 ) 
    }
    pub fn airflow_direction (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (514 , 11 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `FanControl`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.fan_control_write_resp()` on a `WriteResp`; the returned [`FanControlWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait FanControlWriteResponses < 'a > {
    fn fan_control_write_resp (& self ) -> FanControlWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > FanControlWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn fan_control_write_resp (& self ) -> FanControlWriteResponsesView < 'a , '_ > {
        FanControlWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `FanControl` cluster's client operations. Returned by [`FanControlClient::fan_control`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct FanControlClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > FanControlClientView < T > {
    pub fn step < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (StepRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: FanControlCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . fan_control_inv () ;
            let req_builder = view . step (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn fan_mode_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < FanModeEnum , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . fan_mode (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn fan_mode_sequence_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < FanModeSequenceEnum , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . fan_mode_sequence (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn percent_setting_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: Percent > , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . percent_setting (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn percent_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: im :: Percent , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . percent_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn speed_max_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . speed_max (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn speed_setting_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . speed_setting (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn speed_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . speed_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rock_support_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < RockBitmap , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . rock_support (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rock_setting_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < RockBitmap , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . rock_setting (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wind_support_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < WindBitmap , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . wind_support (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wind_setting_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < WindBitmap , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . wind_setting (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn airflow_direction_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < AirflowDirectionEnum , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . airflow_direction (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: FanControlAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . fan_control_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn fan_mode_write (self , endpoint : crate :: dm :: EndptId , value : FanModeEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: FanControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . fan_control_write () . fan_mode (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn percent_setting_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < crate :: im :: Percent > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: FanControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . fan_control_write () . percent_setting (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn speed_setting_write (self , endpoint : crate :: dm :: EndptId , value : crate :: tlv :: Nullable < u8 > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: FanControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . fan_control_write () . speed_setting (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn rock_setting_write (self , endpoint : crate :: dm :: EndptId , value : RockBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: FanControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . fan_control_write () . rock_setting (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn wind_setting_write (self , endpoint : crate :: dm :: EndptId , value : WindBitmap , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: FanControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . fan_control_write () . wind_setting (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn airflow_direction_write (self , endpoint : crate :: dm :: EndptId , value : AirflowDirectionEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: FanControlAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . fan_control_write () . airflow_direction (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `FanControl` cluster. `use` this trait to call `.fan_control()` on any [`crate::im::ImClient`], such as an exchange; the returned [`FanControlClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `FanControlClientView` keeps each cluster's API surface narrow: at the call site `exchange.fan_control().` shows only this cluster's methods in IDE completion." ] pub trait FanControlClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn fan_control (self ) -> FanControlClientView < Self > {
        FanControlClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > FanControlClient for T {
    
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
    fn fan_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < FanModeEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn fan_mode_sequence (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < FanModeSequenceEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn percent_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: Percent > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn percent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: im :: Percent , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn speed_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn speed_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn speed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rock_support (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RockBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rock_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RockBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wind_support (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < WindBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wind_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < WindBitmap , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn airflow_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < AirflowDirectionEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_fan_mode (& self , ctx : impl crate :: dm :: WriteContext , value : FanModeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_percent_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < crate :: im :: Percent > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_speed_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_rock_setting (& self , ctx : impl crate :: dm :: WriteContext , value : RockBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_wind_setting (& self , ctx : impl crate :: dm :: WriteContext , value : WindBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_airflow_direction (& self , ctx : impl crate :: dm :: WriteContext , value : AirflowDirectionEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn fan_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < FanModeEnum , crate :: error :: Error >> {
        T :: fan_mode (self , ctx ) 
    }
    fn fan_mode_sequence (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < FanModeSequenceEnum , crate :: error :: Error >> {
        T :: fan_mode_sequence (self , ctx ) 
    }
    fn percent_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: Percent > , crate :: error :: Error >> {
        T :: percent_setting (self , ctx ) 
    }
    fn percent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: im :: Percent , crate :: error :: Error >> {
        T :: percent_current (self , ctx ) 
    }
    fn speed_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: speed_max (self , ctx ) 
    }
    fn speed_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: speed_setting (self , ctx ) 
    }
    fn speed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: speed_current (self , ctx ) 
    }
    fn rock_support (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RockBitmap , crate :: error :: Error >> {
        T :: rock_support (self , ctx ) 
    }
    fn rock_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < RockBitmap , crate :: error :: Error >> {
        T :: rock_setting (self , ctx ) 
    }
    fn wind_support (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < WindBitmap , crate :: error :: Error >> {
        T :: wind_support (self , ctx ) 
    }
    fn wind_setting (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < WindBitmap , crate :: error :: Error >> {
        T :: wind_setting (self , ctx ) 
    }
    fn airflow_direction (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < AirflowDirectionEnum , crate :: error :: Error >> {
        T :: airflow_direction (self , ctx ) 
    }
    fn set_fan_mode (& self , ctx : impl crate :: dm :: WriteContext , value : FanModeEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_fan_mode (self , ctx , value ) 
    }
    fn set_percent_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < crate :: im :: Percent > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_percent_setting (self , ctx , value ) 
    }
    fn set_speed_setting (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: tlv :: Nullable < u8 > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_speed_setting (self , ctx , value ) 
    }
    fn set_rock_setting (& self , ctx : impl crate :: dm :: WriteContext , value : RockBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_rock_setting (self , ctx , value ) 
    }
    fn set_wind_setting (& self , ctx : impl crate :: dm :: WriteContext , value : WindBitmap ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_wind_setting (self , ctx , value ) 
    }
    fn set_airflow_direction (& self , ctx : impl crate :: dm :: WriteContext , value : AirflowDirectionEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_airflow_direction (self , ctx , value ) 
    }
    fn handle_step (& self , ctx : impl crate :: dm :: InvokeContext , request : StepRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_step (self , ctx , request ) 
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
                    AttributeId :: FanMode => {
                        let attr_read_result = self . 0. fan_mode (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FanMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: FanModeSequence => {
                        let attr_read_result = self . 0. fan_mode_sequence (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FanModeSequence , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PercentSetting => {
                        let attr_read_result = self . 0. percent_setting (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PercentSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: PercentCurrent => {
                        let attr_read_result = self . 0. percent_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PercentCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeedMax => {
                        let attr_read_result = self . 0. speed_max (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedMax , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeedSetting => {
                        let attr_read_result = self . 0. speed_setting (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeedCurrent => {
                        let attr_read_result = self . 0. speed_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RockSupport => {
                        let attr_read_result = self . 0. rock_support (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RockSupport , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RockSetting => {
                        let attr_read_result = self . 0. rock_setting (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RockSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WindSupport => {
                        let attr_read_result = self . 0. wind_support (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WindSupport , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WindSetting => {
                        let attr_read_result = self . 0. wind_setting (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WindSetting , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: AirflowDirection => {
                        let attr_read_result = self . 0. airflow_direction (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AirflowDirection , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: FanMode => {
                let attr_data : FanModeEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_fan_mode (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: FanMode , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: PercentSetting => {
                let attr_data : crate :: tlv :: Nullable < crate :: im :: Percent > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_percent_setting (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PercentSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SpeedSetting => {
                let attr_data : crate :: tlv :: Nullable < u8 > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_speed_setting (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeedSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: RockSetting => {
                let attr_data : RockBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_rock_setting (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RockSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: WindSetting => {
                let attr_data : WindBitmap = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_wind_setting (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WindSetting , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: AirflowDirection => {
                let attr_data : AirflowDirectionEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_airflow_direction (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AirflowDirection , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: Step => {
                let cmd_data : StepRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_step (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: Step ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 514u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "FanControl" , 514u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "FanControl" , 514u32 , self . 0.2 ) 
    }
    
}
