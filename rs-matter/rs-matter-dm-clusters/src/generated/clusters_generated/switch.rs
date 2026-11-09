# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const LATCHING_SWITCH = 1 ;
        const MOMENTARY_SWITCH = 2 ;
        const MOMENTARY_SWITCH_RELEASE = 4 ;
        const MOMENTARY_SWITCH_LONG_PRESS = 8 ;
        const MOMENTARY_SWITCH_MULTI_PRESS = 16 ;
        const ACTION_SWITCH = 32 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const LATCHING_SWITCH = 1 ;
        const MOMENTARY_SWITCH = 2 ;
        const MOMENTARY_SWITCH_RELEASE = 4 ;
        const MOMENTARY_SWITCH_LONG_PRESS = 8 ;
        const MOMENTARY_SWITCH_MULTI_PRESS = 16 ;
        const ACTION_SWITCH = 32 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SwitchLatchedTag {
    NewPosition = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum InitialPressTag {
    NewPosition = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum LongPressTag {
    NewPosition = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ShortReleaseTag {
    PreviousPosition = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum LongReleaseTag {
    PreviousPosition = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MultiPressOngoingTag {
    NewPosition = 0 , CurrentNumberOfPressesCounted = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MultiPressCompleteTag {
    PreviousPosition = 0 , TotalNumberOfPressesCounted = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SwitchLatched < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SwitchLatched < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn new_position (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SwitchLatched < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SwitchLatched < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SwitchLatched < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SwitchLatched" ) ? ;
        match self . new_position () {
            Ok (value ) => write ! (f , "{}: {:?}," , "new_position" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SwitchLatched < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SwitchLatched" ) ;
        match self . new_position () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "new_position" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct InitialPress < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > InitialPress < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn new_position (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for InitialPress < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for InitialPress < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for InitialPress < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "InitialPress" ) ? ;
        match self . new_position () {
            Ok (value ) => write ! (f , "{}: {:?}," , "new_position" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for InitialPress < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "InitialPress" ) ;
        match self . new_position () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "new_position" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct LongPress < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > LongPress < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn new_position (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for LongPress < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for LongPress < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for LongPress < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "LongPress" ) ? ;
        match self . new_position () {
            Ok (value ) => write ! (f , "{}: {:?}," , "new_position" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for LongPress < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "LongPress" ) ;
        match self . new_position () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "new_position" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ShortRelease < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ShortRelease < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_position (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ShortRelease < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ShortRelease < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ShortRelease < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ShortRelease" ) ? ;
        match self . previous_position () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous_position" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_position" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ShortRelease < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ShortRelease" ) ;
        match self . previous_position () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous_position" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_position" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct LongRelease < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > LongRelease < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_position (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for LongRelease < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for LongRelease < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for LongRelease < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "LongRelease" ) ? ;
        match self . previous_position () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous_position" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_position" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for LongRelease < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "LongRelease" ) ;
        match self . previous_position () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous_position" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_position" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MultiPressOngoing < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MultiPressOngoing < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn new_position (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn current_number_of_presses_counted (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MultiPressOngoing < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MultiPressOngoing < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MultiPressOngoing < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MultiPressOngoing" ) ? ;
        match self . new_position () {
            Ok (value ) => write ! (f , "{}: {:?}," , "new_position" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) ? , 
        }
        match self . current_number_of_presses_counted () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current_number_of_presses_counted" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current_number_of_presses_counted" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MultiPressOngoing < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MultiPressOngoing" ) ;
        match self . new_position () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "new_position" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "new_position" , e . code () ) , 
        }
        match self . current_number_of_presses_counted () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current_number_of_presses_counted" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current_number_of_presses_counted" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MultiPressComplete < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MultiPressComplete < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_position (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn total_number_of_presses_counted (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MultiPressComplete < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MultiPressComplete < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MultiPressComplete < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MultiPressComplete" ) ? ;
        match self . previous_position () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous_position" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_position" , e . code () ) ? , 
        }
        match self . total_number_of_presses_counted () {
            Ok (value ) => write ! (f , "{}: {:?}," , "total_number_of_presses_counted" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "total_number_of_presses_counted" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MultiPressComplete < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MultiPressComplete" ) ;
        match self . previous_position () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous_position" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_position" , e . code () ) , 
        }
        match self . total_number_of_presses_counted () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "total_number_of_presses_counted" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "total_number_of_presses_counted" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct SwitchLatchedBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SwitchLatchedBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SwitchLatchedBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn new_position (mut self , value : u8 ) -> Result < SwitchLatchedBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "newPosition" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SwitchLatchedBuilder (self . 0 ) ) 
    }
    
}
impl < P > SwitchLatchedBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SwitchLatchedBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SwitchLatched" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SwitchLatchedBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SwitchLatched" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SwitchLatchedBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SwitchLatchedBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SwitchLatchedArrayBuilder < P > (P ) ;
impl < P > SwitchLatchedArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SwitchLatchedBuilder < SwitchLatchedArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SwitchLatchedArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SwitchLatchedArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SwitchLatched[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SwitchLatchedArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SwitchLatched[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SwitchLatchedArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SwitchLatchedArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct InitialPressBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > InitialPressBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > InitialPressBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn new_position (mut self , value : u8 ) -> Result < InitialPressBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "newPosition" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (InitialPressBuilder (self . 0 ) ) 
    }
    
}
impl < P > InitialPressBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for InitialPressBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "InitialPress" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for InitialPressBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "InitialPress" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for InitialPressBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for InitialPressBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct InitialPressArrayBuilder < P > (P ) ;
impl < P > InitialPressArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < InitialPressBuilder < InitialPressArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (InitialPressArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for InitialPressArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "InitialPress[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for InitialPressArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "InitialPress[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for InitialPressArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for InitialPressArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LongPressBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > LongPressBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > LongPressBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn new_position (mut self , value : u8 ) -> Result < LongPressBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "newPosition" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (LongPressBuilder (self . 0 ) ) 
    }
    
}
impl < P > LongPressBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for LongPressBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LongPress" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for LongPressBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LongPress" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for LongPressBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LongPressBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LongPressArrayBuilder < P > (P ) ;
impl < P > LongPressArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < LongPressBuilder < LongPressArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LongPressArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for LongPressArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LongPress[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for LongPressArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LongPress[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for LongPressArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LongPressArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ShortReleaseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ShortReleaseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ShortReleaseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_position (mut self , value : u8 ) -> Result < ShortReleaseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousPosition" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ShortReleaseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ShortReleaseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ShortReleaseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ShortRelease" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ShortReleaseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ShortRelease" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ShortReleaseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ShortReleaseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ShortReleaseArrayBuilder < P > (P ) ;
impl < P > ShortReleaseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ShortReleaseBuilder < ShortReleaseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ShortReleaseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ShortReleaseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ShortRelease[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ShortReleaseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ShortRelease[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ShortReleaseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ShortReleaseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LongReleaseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > LongReleaseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > LongReleaseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_position (mut self , value : u8 ) -> Result < LongReleaseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousPosition" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (LongReleaseBuilder (self . 0 ) ) 
    }
    
}
impl < P > LongReleaseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for LongReleaseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LongRelease" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for LongReleaseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LongRelease" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for LongReleaseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LongReleaseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LongReleaseArrayBuilder < P > (P ) ;
impl < P > LongReleaseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < LongReleaseBuilder < LongReleaseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LongReleaseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for LongReleaseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LongRelease[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for LongReleaseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LongRelease[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for LongReleaseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LongReleaseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MultiPressOngoingBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MultiPressOngoingBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MultiPressOngoingBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn new_position (mut self , value : u8 ) -> Result < MultiPressOngoingBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "newPosition" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MultiPressOngoingBuilder (self . 0 ) ) 
    }
    
}
impl < P > MultiPressOngoingBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_number_of_presses_counted (mut self , value : u8 ) -> Result < MultiPressOngoingBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "currentNumberOfPressesCounted" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MultiPressOngoingBuilder (self . 0 ) ) 
    }
    
}
impl < P > MultiPressOngoingBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MultiPressOngoingBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MultiPressOngoing" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MultiPressOngoingBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MultiPressOngoing" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MultiPressOngoingBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MultiPressOngoingBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MultiPressOngoingArrayBuilder < P > (P ) ;
impl < P > MultiPressOngoingArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MultiPressOngoingBuilder < MultiPressOngoingArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MultiPressOngoingArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MultiPressOngoingArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MultiPressOngoing[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MultiPressOngoingArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MultiPressOngoing[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MultiPressOngoingArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MultiPressOngoingArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MultiPressCompleteBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MultiPressCompleteBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MultiPressCompleteBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_position (mut self , value : u8 ) -> Result < MultiPressCompleteBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousPosition" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MultiPressCompleteBuilder (self . 0 ) ) 
    }
    
}
impl < P > MultiPressCompleteBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn total_number_of_presses_counted (mut self , value : u8 ) -> Result < MultiPressCompleteBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "totalNumberOfPressesCounted" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MultiPressCompleteBuilder (self . 0 ) ) 
    }
    
}
impl < P > MultiPressCompleteBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MultiPressCompleteBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MultiPressComplete" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MultiPressCompleteBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MultiPressComplete" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MultiPressCompleteBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MultiPressCompleteBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MultiPressCompleteArrayBuilder < P > (P ) ;
impl < P > MultiPressCompleteArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MultiPressCompleteBuilder < MultiPressCompleteArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MultiPressCompleteArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MultiPressCompleteArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MultiPressComplete[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MultiPressCompleteArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MultiPressComplete[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MultiPressCompleteArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MultiPressCompleteArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl SwitchLatched < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (SwitchLatchedBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 59 , EventId :: SwitchLatched as _ , Self :: PRIORITY , | tw | {
            f (SwitchLatchedBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (SwitchLatchedBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: SwitchLatched as _ , Self :: PRIORITY , | tw | {
            f (SwitchLatchedBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl InitialPress < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (InitialPressBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 59 , EventId :: InitialPress as _ , Self :: PRIORITY , | tw | {
            f (InitialPressBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (InitialPressBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: InitialPress as _ , Self :: PRIORITY , | tw | {
            f (InitialPressBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl LongPress < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (LongPressBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 59 , EventId :: LongPress as _ , Self :: PRIORITY , | tw | {
            f (LongPressBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (LongPressBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: LongPress as _ , Self :: PRIORITY , | tw | {
            f (LongPressBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl ShortRelease < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ShortReleaseBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 59 , EventId :: ShortRelease as _ , Self :: PRIORITY , | tw | {
            f (ShortReleaseBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (ShortReleaseBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: ShortRelease as _ , Self :: PRIORITY , | tw | {
            f (ShortReleaseBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl LongRelease < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (LongReleaseBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 59 , EventId :: LongRelease as _ , Self :: PRIORITY , | tw | {
            f (LongReleaseBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (LongReleaseBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: LongRelease as _ , Self :: PRIORITY , | tw | {
            f (LongReleaseBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl MultiPressOngoing < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (MultiPressOngoingBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 59 , EventId :: MultiPressOngoing as _ , Self :: PRIORITY , | tw | {
            f (MultiPressOngoingBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (MultiPressOngoingBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: MultiPressOngoing as _ , Self :: PRIORITY , | tw | {
            f (MultiPressOngoingBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl MultiPressComplete < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (MultiPressCompleteBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 59 , EventId :: MultiPressComplete as _ , Self :: PRIORITY , | tw | {
            f (MultiPressCompleteBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (MultiPressCompleteBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: MultiPressComplete as _ , Self :: PRIORITY , | tw | {
            f (MultiPressCompleteBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    NumberOfPositions = 0 , CurrentPosition = 1 , MultiPressMax = 2 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: NumberOfPositions => write ! (f , "{}(0x{:02x})" , "NumberOfPositions" , AttributeId :: NumberOfPositions as u32 ) ? , AttributeId :: CurrentPosition => write ! (f , "{}(0x{:02x})" , "CurrentPosition" , AttributeId :: CurrentPosition as u32 ) ? , AttributeId :: MultiPressMax => write ! (f , "{}(0x{:02x})" , "MultiPressMax" , AttributeId :: MultiPressMax as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: NumberOfPositions => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfPositions" , AttributeId :: NumberOfPositions as u32 ) , AttributeId :: CurrentPosition => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentPosition" , AttributeId :: CurrentPosition as u32 ) , AttributeId :: MultiPressMax => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MultiPressMax" , AttributeId :: MultiPressMax as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
        }
        if self . 0.1 {
            crate :: reexport :: defmt :: write ! (f , "::Write" ) 
        }
        else {
            crate :: reexport :: defmt :: write ! (f , "::Read" ) 
        }
        
    }
    
}
# [doc = "The command IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] pub enum CommandId {
    
}
impl core :: convert :: TryFrom < crate :: dm :: CmdId > for CommandId {
    type Error = crate :: error :: Error ;
    fn try_from (id : crate :: dm :: CmdId ) -> Result < Self , Self :: Error > {
        Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) 
    }
    
}
impl core :: fmt :: Debug for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            
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
    SwitchLatched = 0 , InitialPress = 1 , LongPress = 2 , ShortRelease = 3 , LongRelease = 4 , MultiPressOngoing = 5 , MultiPressComplete = 6 
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
            EventId :: SwitchLatched => write ! (f , "{}(0x{:02x})" , "SwitchLatched" , EventId :: SwitchLatched as u32 ) ? , EventId :: InitialPress => write ! (f , "{}(0x{:02x})" , "InitialPress" , EventId :: InitialPress as u32 ) ? , EventId :: LongPress => write ! (f , "{}(0x{:02x})" , "LongPress" , EventId :: LongPress as u32 ) ? , EventId :: ShortRelease => write ! (f , "{}(0x{:02x})" , "ShortRelease" , EventId :: ShortRelease as u32 ) ? , EventId :: LongRelease => write ! (f , "{}(0x{:02x})" , "LongRelease" , EventId :: LongRelease as u32 ) ? , EventId :: MultiPressOngoing => write ! (f , "{}(0x{:02x})" , "MultiPressOngoing" , EventId :: MultiPressOngoing as u32 ) ? , EventId :: MultiPressComplete => write ! (f , "{}(0x{:02x})" , "MultiPressComplete" , EventId :: MultiPressComplete as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: SwitchLatched => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SwitchLatched" , EventId :: SwitchLatched as u32 ) , EventId :: InitialPress => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "InitialPress" , EventId :: InitialPress as u32 ) , EventId :: LongPress => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LongPress" , EventId :: LongPress as u32 ) , EventId :: ShortRelease => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ShortRelease" , EventId :: ShortRelease as u32 ) , EventId :: LongRelease => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LongRelease" , EventId :: LongRelease as u32 ) , EventId :: MultiPressOngoing => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MultiPressOngoing" , EventId :: MultiPressOngoing as u32 ) , EventId :: MultiPressComplete => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MultiPressComplete" , EventId :: MultiPressComplete as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (59 , 2 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: NumberOfPositions as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentPosition as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: MultiPressMax as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [] , & [crate :: dm :: Event :: new (EventId :: SwitchLatched as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: InitialPress as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: LongPress as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: ShortRelease as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: LongRelease as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: MultiPressOngoing as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: MultiPressComplete as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn number_of_positions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn current_position (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn multi_press_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    
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
    fn number_of_positions (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: number_of_positions (self , ctx ) 
    }
    fn current_position (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: current_position (self , ctx ) 
    }
    fn multi_press_max (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: multi_press_max (self , ctx ) 
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
                    AttributeId :: NumberOfPositions => {
                        let attr_read_result = self . 0. number_of_positions (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfPositions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentPosition => {
                        let attr_read_result = self . 0. current_position (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentPosition , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MultiPressMax => {
                        let attr_read_result = self . 0. multi_press_max (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MultiPressMax , false ) ) ) ) , & attr_read_result ) ;
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
        crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
        return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 59u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Switch" , 59u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Switch" , 59u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `Switch` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct SwitchCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > SwitchCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Switch` cluster's commands. `use` this trait to call `.switch_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`SwitchCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait SwitchCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn switch_inv (self ) -> SwitchCmdRequestsView < P > ;
    
}
impl < P > SwitchCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn switch_inv (self ) -> SwitchCmdRequestsView < P > {
        SwitchCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `Switch` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct SwitchAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > SwitchAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn number_of_positions (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_position (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn multi_press_max (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (59 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Switch` cluster's attribute reads. `use` this trait to call `.switch_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`SwitchAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait SwitchAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn switch_read (self ) -> SwitchAttrReadsView < P > ;
    
}
impl < P > SwitchAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn switch_read (self ) -> SwitchAttrReadsView < P > {
        SwitchAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `Switch` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct SwitchAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > SwitchAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Switch` cluster's attribute writes. `use` this trait to call `.switch_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`SwitchAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait SwitchAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn switch_write (self ) -> SwitchAttrWritesView < P > ;
    
}
impl < P > SwitchAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn switch_write (self ) -> SwitchAttrWritesView < P > {
        SwitchAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `Switch` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct SwitchCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > SwitchCmdResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `Switch`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.switch_inv_resp()` on an `InvokeResp`; the returned [`SwitchCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait SwitchCmdResponses < 'a > {
    fn switch_inv_resp (& self ) -> SwitchCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > SwitchCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn switch_inv_resp (& self ) -> SwitchCmdResponsesView < 'a , '_ > {
        SwitchCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `Switch` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct SwitchAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > SwitchAttrResponsesView < 'a , 'r > {
    pub fn number_of_positions (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (59 , 0 ) 
    }
    pub fn current_position (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (59 , 1 ) 
    }
    pub fn multi_press_max (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (59 , 2 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (59 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (59 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (59 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (59 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (59 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Switch`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.switch_read_resp()` on a `ReportDataResp`; the returned [`SwitchAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait SwitchAttrResponses < 'a > {
    fn switch_read_resp (& self ) -> SwitchAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > SwitchAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn switch_read_resp (& self ) -> SwitchAttrResponsesView < 'a , '_ > {
        SwitchAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `Switch` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct SwitchWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > SwitchWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `Switch`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.switch_write_resp()` on a `WriteResp`; the returned [`SwitchWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait SwitchWriteResponses < 'a > {
    fn switch_write_resp (& self ) -> SwitchWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > SwitchWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn switch_write_resp (& self ) -> SwitchWriteResponsesView < 'a , '_ > {
        SwitchWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `Switch` cluster's client operations. Returned by [`SwitchClient::switch`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct SwitchClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > SwitchClientView < T > {
    pub fn number_of_positions_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . number_of_positions (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn current_position_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . current_position (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn multi_press_max_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . multi_press_max (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: SwitchAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . switch_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `Switch` cluster. `use` this trait to call `.switch()` on any [`crate::im::ImClient`], such as an exchange; the returned [`SwitchClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `SwitchClientView` keeps each cluster's API surface narrow: at the call site `exchange.switch().` shows only this cluster's methods in IDE completion." ] pub trait SwitchClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn switch (self ) -> SwitchClientView < Self > {
        SwitchClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > SwitchClient for T {
    
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
    fn number_of_positions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn current_position (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn multi_press_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    
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
    fn number_of_positions (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: number_of_positions (self , ctx ) 
    }
    fn current_position (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: current_position (self , ctx ) 
    }
    fn multi_press_max (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: multi_press_max (self , ctx ) 
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
                    AttributeId :: NumberOfPositions => {
                        let attr_read_result = self . 0. number_of_positions (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfPositions , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentPosition => {
                        let attr_read_result = self . 0. current_position (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentPosition , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MultiPressMax => {
                        let attr_read_result = self . 0. multi_press_max (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MultiPressMax , false ) ) ) ) , & attr_read_result ) ;
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
        crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
        return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 59u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Switch" , 59u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Switch" , 59u32 , self . 0.2 ) 
    }
    
}
