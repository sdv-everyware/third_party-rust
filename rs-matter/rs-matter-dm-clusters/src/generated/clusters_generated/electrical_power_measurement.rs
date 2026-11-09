# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const DIRECT_CURRENT = 1 ;
        const ALTERNATING_CURRENT = 2 ;
        const POLYPHASE_POWER = 4 ;
        const HARMONICS = 8 ;
        const POWER_QUALITY = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const DIRECT_CURRENT = 1 ;
        const ALTERNATING_CURRENT = 2 ;
        const POLYPHASE_POWER = 4 ;
        const HARMONICS = 8 ;
        const POWER_QUALITY = 16 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PowerModeEnum {
    # [enumval (0 ) ] Unknown = 0 , # [enumval (1 ) ] DC = 1 , # [enumval (2 ) ] AC = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum HarmonicMeasurementStructTag {
    Order = 0 , Measurement = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MeasurementRangeStructTag {
    MeasurementType = 0 , Min = 1 , Max = 2 , StartTimestamp = 3 , EndTimestamp = 4 , MinTimestamp = 5 , MaxTimestamp = 6 , StartSystime = 7 , EndSystime = 8 , MinSystime = 9 , MaxSystime = 10 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct HarmonicMeasurementStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > HarmonicMeasurementStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn order (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn measurement (& self ) -> Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for HarmonicMeasurementStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for HarmonicMeasurementStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for HarmonicMeasurementStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "HarmonicMeasurementStruct" ) ? ;
        match self . order () {
            Ok (value ) => write ! (f , "{}: {:?}," , "order" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "order" , e . code () ) ? , 
        }
        match self . measurement () {
            Ok (value ) => write ! (f , "{}: {:?}," , "measurement" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "measurement" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for HarmonicMeasurementStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "HarmonicMeasurementStruct" ) ;
        match self . order () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "order" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "order" , e . code () ) , 
        }
        match self . measurement () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "measurement" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "measurement" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MeasurementRangeStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MeasurementRangeStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn measurement_type (& self ) -> Result < MeasurementTypeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn min (& self ) -> Result < i64 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn max (& self ) -> Result < i64 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn start_timestamp (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn end_timestamp (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn min_timestamp (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn max_timestamp (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    pub fn start_systime (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (7 ) 
    }
    pub fn end_systime (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (8 ) 
    }
    pub fn min_systime (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (9 ) 
    }
    pub fn max_systime (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (10 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MeasurementRangeStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MeasurementRangeStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MeasurementRangeStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MeasurementRangeStruct" ) ? ;
        match self . measurement_type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "measurement_type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "measurement_type" , e . code () ) ? , 
        }
        match self . min () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min" , e . code () ) ? , 
        }
        match self . max () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max" , e . code () ) ? , 
        }
        match self . start_timestamp () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "start_timestamp" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "start_timestamp" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "start_timestamp" , e . code () ) ? , 
        }
        match self . end_timestamp () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "end_timestamp" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "end_timestamp" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "end_timestamp" , e . code () ) ? , 
        }
        match self . min_timestamp () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "min_timestamp" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "min_timestamp" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_timestamp" , e . code () ) ? , 
        }
        match self . max_timestamp () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "max_timestamp" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "max_timestamp" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_timestamp" , e . code () ) ? , 
        }
        match self . start_systime () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "start_systime" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "start_systime" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "start_systime" , e . code () ) ? , 
        }
        match self . end_systime () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "end_systime" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "end_systime" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "end_systime" , e . code () ) ? , 
        }
        match self . min_systime () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "min_systime" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "min_systime" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_systime" , e . code () ) ? , 
        }
        match self . max_systime () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "max_systime" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "max_systime" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_systime" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MeasurementRangeStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MeasurementRangeStruct" ) ;
        match self . measurement_type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "measurement_type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "measurement_type" , e . code () ) , 
        }
        match self . min () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min" , e . code () ) , 
        }
        match self . max () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max" , e . code () ) , 
        }
        match self . start_timestamp () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "start_timestamp" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "start_timestamp" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "start_timestamp" , e . code () ) , 
        }
        match self . end_timestamp () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "end_timestamp" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "end_timestamp" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "end_timestamp" , e . code () ) , 
        }
        match self . min_timestamp () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "min_timestamp" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "min_timestamp" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_timestamp" , e . code () ) , 
        }
        match self . max_timestamp () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "max_timestamp" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "max_timestamp" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_timestamp" , e . code () ) , 
        }
        match self . start_systime () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "start_systime" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "start_systime" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "start_systime" , e . code () ) , 
        }
        match self . end_systime () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "end_systime" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "end_systime" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "end_systime" , e . code () ) , 
        }
        match self . min_systime () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "min_systime" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "min_systime" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_systime" , e . code () ) , 
        }
        match self . max_systime () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "max_systime" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "max_systime" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_systime" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct HarmonicMeasurementStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > HarmonicMeasurementStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > HarmonicMeasurementStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn order (mut self , value : u8 ) -> Result < HarmonicMeasurementStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "order" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (HarmonicMeasurementStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > HarmonicMeasurementStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn measurement (mut self , value : crate :: tlv :: Nullable < i64 > ) -> Result < HarmonicMeasurementStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "measurement" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (HarmonicMeasurementStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > HarmonicMeasurementStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for HarmonicMeasurementStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "HarmonicMeasurementStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for HarmonicMeasurementStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "HarmonicMeasurementStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for HarmonicMeasurementStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for HarmonicMeasurementStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct HarmonicMeasurementStructArrayBuilder < P > (P ) ;
impl < P > HarmonicMeasurementStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < HarmonicMeasurementStructBuilder < HarmonicMeasurementStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (HarmonicMeasurementStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for HarmonicMeasurementStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "HarmonicMeasurementStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for HarmonicMeasurementStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "HarmonicMeasurementStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for HarmonicMeasurementStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for HarmonicMeasurementStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MeasurementRangeStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MeasurementRangeStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn measurement_type (mut self , value : MeasurementTypeEnum ) -> Result < MeasurementRangeStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "measurementType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min (mut self , value : i64 ) -> Result < MeasurementRangeStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "min" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max (mut self , value : i64 ) -> Result < MeasurementRangeStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "max" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn start_timestamp (mut self , value : Option < u32 > ) -> Result < MeasurementRangeStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "startTimestamp" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn end_timestamp (mut self , value : Option < u32 > ) -> Result < MeasurementRangeStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "endTimestamp" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_timestamp (mut self , value : Option < u32 > ) -> Result < MeasurementRangeStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minTimestamp" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_timestamp (mut self , value : Option < u32 > ) -> Result < MeasurementRangeStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxTimestamp" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn start_systime (mut self , value : Option < u64 > ) -> Result < MeasurementRangeStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "startSystime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn end_systime (mut self , value : Option < u64 > ) -> Result < MeasurementRangeStructBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "endSystime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_systime (mut self , value : Option < u64 > ) -> Result < MeasurementRangeStructBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minSystime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_systime (mut self , value : Option < u64 > ) -> Result < MeasurementRangeStructBuilder < P , 11usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxSystime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 10 , & value ) ? ;
        Ok (MeasurementRangeStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > MeasurementRangeStructBuilder < P , 11usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MeasurementRangeStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MeasurementRangeStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MeasurementRangeStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MeasurementRangeStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MeasurementRangeStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MeasurementRangeStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MeasurementRangeStructArrayBuilder < P > (P ) ;
impl < P > MeasurementRangeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MeasurementRangeStructBuilder < MeasurementRangeStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MeasurementRangeStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MeasurementRangeStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MeasurementRangeStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MeasurementRangeStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MeasurementRangeStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MeasurementRangeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MeasurementRangeStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum MeasurementPeriodRangesTag {
    Ranges = 0 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct MeasurementPeriodRanges < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > MeasurementPeriodRanges < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn ranges (& self ) -> Result < crate :: tlv :: TLVArray < '_ , MeasurementRangeStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for MeasurementPeriodRanges < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for MeasurementPeriodRanges < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for MeasurementPeriodRanges < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "MeasurementPeriodRanges" ) ? ;
        match self . ranges () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ranges" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ranges" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MeasurementPeriodRanges < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "MeasurementPeriodRanges" ) ;
        match self . ranges () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ranges" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ranges" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct MeasurementPeriodRangesBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > MeasurementPeriodRangesBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > MeasurementPeriodRangesBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ranges (self ) -> Result < MeasurementRangeStructArrayBuilder < MeasurementPeriodRangesBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MeasurementPeriodRangesBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > MeasurementPeriodRangesBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for MeasurementPeriodRangesBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MeasurementPeriodRanges" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for MeasurementPeriodRangesBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MeasurementPeriodRanges" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for MeasurementPeriodRangesBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MeasurementPeriodRangesBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct MeasurementPeriodRangesArrayBuilder < P > (P ) ;
impl < P > MeasurementPeriodRangesArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < MeasurementPeriodRangesBuilder < MeasurementPeriodRangesArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (MeasurementPeriodRangesArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for MeasurementPeriodRangesArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "MeasurementPeriodRanges[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for MeasurementPeriodRangesArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "MeasurementPeriodRanges[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for MeasurementPeriodRangesArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for MeasurementPeriodRangesArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl MeasurementPeriodRanges < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (MeasurementPeriodRangesBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 144 , EventId :: MeasurementPeriodRanges as _ , Self :: PRIORITY , | tw | {
            f (MeasurementPeriodRangesBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (MeasurementPeriodRangesBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: MeasurementPeriodRanges as _ , Self :: PRIORITY , | tw | {
            f (MeasurementPeriodRangesBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    PowerMode = 0 , NumberOfMeasurementTypes = 1 , Accuracy = 2 , Ranges = 3 , Voltage = 4 , ActiveCurrent = 5 , ReactiveCurrent = 6 , ApparentCurrent = 7 , ActivePower = 8 , ReactivePower = 9 , ApparentPower = 10 , RMSVoltage = 11 , RMSCurrent = 12 , RMSPower = 13 , Frequency = 14 , HarmonicCurrents = 15 , HarmonicPhases = 16 , PowerFactor = 17 , NeutralCurrent = 18 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: PowerMode => write ! (f , "{}(0x{:02x})" , "PowerMode" , AttributeId :: PowerMode as u32 ) ? , AttributeId :: NumberOfMeasurementTypes => write ! (f , "{}(0x{:02x})" , "NumberOfMeasurementTypes" , AttributeId :: NumberOfMeasurementTypes as u32 ) ? , AttributeId :: Accuracy => write ! (f , "{}(0x{:02x})" , "Accuracy" , AttributeId :: Accuracy as u32 ) ? , AttributeId :: Ranges => write ! (f , "{}(0x{:02x})" , "Ranges" , AttributeId :: Ranges as u32 ) ? , AttributeId :: Voltage => write ! (f , "{}(0x{:02x})" , "Voltage" , AttributeId :: Voltage as u32 ) ? , AttributeId :: ActiveCurrent => write ! (f , "{}(0x{:02x})" , "ActiveCurrent" , AttributeId :: ActiveCurrent as u32 ) ? , AttributeId :: ReactiveCurrent => write ! (f , "{}(0x{:02x})" , "ReactiveCurrent" , AttributeId :: ReactiveCurrent as u32 ) ? , AttributeId :: ApparentCurrent => write ! (f , "{}(0x{:02x})" , "ApparentCurrent" , AttributeId :: ApparentCurrent as u32 ) ? , AttributeId :: ActivePower => write ! (f , "{}(0x{:02x})" , "ActivePower" , AttributeId :: ActivePower as u32 ) ? , AttributeId :: ReactivePower => write ! (f , "{}(0x{:02x})" , "ReactivePower" , AttributeId :: ReactivePower as u32 ) ? , AttributeId :: ApparentPower => write ! (f , "{}(0x{:02x})" , "ApparentPower" , AttributeId :: ApparentPower as u32 ) ? , AttributeId :: RMSVoltage => write ! (f , "{}(0x{:02x})" , "RMSVoltage" , AttributeId :: RMSVoltage as u32 ) ? , AttributeId :: RMSCurrent => write ! (f , "{}(0x{:02x})" , "RMSCurrent" , AttributeId :: RMSCurrent as u32 ) ? , AttributeId :: RMSPower => write ! (f , "{}(0x{:02x})" , "RMSPower" , AttributeId :: RMSPower as u32 ) ? , AttributeId :: Frequency => write ! (f , "{}(0x{:02x})" , "Frequency" , AttributeId :: Frequency as u32 ) ? , AttributeId :: HarmonicCurrents => write ! (f , "{}(0x{:02x})" , "HarmonicCurrents" , AttributeId :: HarmonicCurrents as u32 ) ? , AttributeId :: HarmonicPhases => write ! (f , "{}(0x{:02x})" , "HarmonicPhases" , AttributeId :: HarmonicPhases as u32 ) ? , AttributeId :: PowerFactor => write ! (f , "{}(0x{:02x})" , "PowerFactor" , AttributeId :: PowerFactor as u32 ) ? , AttributeId :: NeutralCurrent => write ! (f , "{}(0x{:02x})" , "NeutralCurrent" , AttributeId :: NeutralCurrent as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: PowerMode => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PowerMode" , AttributeId :: PowerMode as u32 ) , AttributeId :: NumberOfMeasurementTypes => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NumberOfMeasurementTypes" , AttributeId :: NumberOfMeasurementTypes as u32 ) , AttributeId :: Accuracy => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Accuracy" , AttributeId :: Accuracy as u32 ) , AttributeId :: Ranges => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Ranges" , AttributeId :: Ranges as u32 ) , AttributeId :: Voltage => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Voltage" , AttributeId :: Voltage as u32 ) , AttributeId :: ActiveCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveCurrent" , AttributeId :: ActiveCurrent as u32 ) , AttributeId :: ReactiveCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ReactiveCurrent" , AttributeId :: ReactiveCurrent as u32 ) , AttributeId :: ApparentCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ApparentCurrent" , AttributeId :: ApparentCurrent as u32 ) , AttributeId :: ActivePower => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActivePower" , AttributeId :: ActivePower as u32 ) , AttributeId :: ReactivePower => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ReactivePower" , AttributeId :: ReactivePower as u32 ) , AttributeId :: ApparentPower => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ApparentPower" , AttributeId :: ApparentPower as u32 ) , AttributeId :: RMSVoltage => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RMSVoltage" , AttributeId :: RMSVoltage as u32 ) , AttributeId :: RMSCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RMSCurrent" , AttributeId :: RMSCurrent as u32 ) , AttributeId :: RMSPower => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RMSPower" , AttributeId :: RMSPower as u32 ) , AttributeId :: Frequency => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Frequency" , AttributeId :: Frequency as u32 ) , AttributeId :: HarmonicCurrents => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "HarmonicCurrents" , AttributeId :: HarmonicCurrents as u32 ) , AttributeId :: HarmonicPhases => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "HarmonicPhases" , AttributeId :: HarmonicPhases as u32 ) , AttributeId :: PowerFactor => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PowerFactor" , AttributeId :: PowerFactor as u32 ) , AttributeId :: NeutralCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NeutralCurrent" , AttributeId :: NeutralCurrent as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    MeasurementPeriodRanges = 0 
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
            EventId :: MeasurementPeriodRanges => write ! (f , "{}(0x{:02x})" , "MeasurementPeriodRanges" , EventId :: MeasurementPeriodRanges as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: MeasurementPeriodRanges => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MeasurementPeriodRanges" , EventId :: MeasurementPeriodRanges as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
use super :: globals :: * ;
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (144 , 3 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: PowerMode as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: NumberOfMeasurementTypes as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Accuracy as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: Ranges as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: Voltage as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ReactiveCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ApparentCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActivePower as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ReactivePower as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ApparentPower as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RMSVoltage as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RMSCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RMSPower as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Frequency as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: HarmonicCurrents as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: HarmonicPhases as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: PowerFactor as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NeutralCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [] , & [crate :: dm :: Event :: new (EventId :: MeasurementPeriodRanges as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn power_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < PowerModeEnum , crate :: error :: Error > ;
    fn number_of_measurement_types (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn accuracy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementAccuracyStructArrayBuilder < P > , MeasurementAccuracyStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn ranges < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementRangeStructArrayBuilder < P > , MeasurementRangeStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn reactive_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn apparent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error > ;
    fn reactive_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVAR > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn apparent_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVA > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rms_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rms_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rms_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn harmonic_currents < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn harmonic_phases < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn power_factor (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn neutral_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
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
    fn power_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < PowerModeEnum , crate :: error :: Error > {
        T :: power_mode (self , ctx ) 
    }
    fn number_of_measurement_types (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: number_of_measurement_types (self , ctx ) 
    }
    fn accuracy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementAccuracyStructArrayBuilder < P > , MeasurementAccuracyStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: accuracy (self , ctx , builder ) 
    }
    fn ranges < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementRangeStructArrayBuilder < P > , MeasurementRangeStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: ranges (self , ctx , builder ) 
    }
    fn voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error > {
        T :: voltage (self , ctx ) 
    }
    fn active_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        T :: active_current (self , ctx ) 
    }
    fn reactive_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        T :: reactive_current (self , ctx ) 
    }
    fn apparent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        T :: apparent_current (self , ctx ) 
    }
    fn active_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error > {
        T :: active_power (self , ctx ) 
    }
    fn reactive_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVAR > , crate :: error :: Error > {
        T :: reactive_power (self , ctx ) 
    }
    fn apparent_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVA > , crate :: error :: Error > {
        T :: apparent_power (self , ctx ) 
    }
    fn rms_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error > {
        T :: rms_voltage (self , ctx ) 
    }
    fn rms_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        T :: rms_current (self , ctx ) 
    }
    fn rms_power (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error > {
        T :: rms_power (self , ctx ) 
    }
    fn frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > {
        T :: frequency (self , ctx ) 
    }
    fn harmonic_currents < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: harmonic_currents (self , ctx , builder ) 
    }
    fn harmonic_phases < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: harmonic_phases (self , ctx , builder ) 
    }
    fn power_factor (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > {
        T :: power_factor (self , ctx ) 
    }
    fn neutral_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > {
        T :: neutral_current (self , ctx ) 
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
                    AttributeId :: PowerMode => {
                        let attr_read_result = self . 0. power_mode (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PowerMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfMeasurementTypes => {
                        let attr_read_result = self . 0. number_of_measurement_types (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfMeasurementTypes , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Accuracy => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Accuracy , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. accuracy (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Accuracy , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Accuracy , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Ranges => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Ranges , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. ranges (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Ranges , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Ranges , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Voltage => {
                        let attr_read_result = self . 0. voltage (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Voltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveCurrent => {
                        let attr_read_result = self . 0. active_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ReactiveCurrent => {
                        let attr_read_result = self . 0. reactive_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ReactiveCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ApparentCurrent => {
                        let attr_read_result = self . 0. apparent_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ApparentCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActivePower => {
                        let attr_read_result = self . 0. active_power (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ReactivePower => {
                        let attr_read_result = self . 0. reactive_power (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ReactivePower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ApparentPower => {
                        let attr_read_result = self . 0. apparent_power (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ApparentPower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RMSVoltage => {
                        let attr_read_result = self . 0. rms_voltage (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RMSVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RMSCurrent => {
                        let attr_read_result = self . 0. rms_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RMSCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RMSPower => {
                        let attr_read_result = self . 0. rms_power (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RMSPower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Frequency => {
                        let attr_read_result = self . 0. frequency (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Frequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HarmonicCurrents => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicCurrents , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. harmonic_currents (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicCurrents , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicCurrents , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: HarmonicPhases => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicPhases , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. harmonic_phases (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicPhases , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicPhases , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: PowerFactor => {
                        let attr_read_result = self . 0. power_factor (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PowerFactor , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NeutralCurrent => {
                        let attr_read_result = self . 0. neutral_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeutralCurrent , false ) ) ) ) , & attr_read_result ) ;
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
        if ctx . cluster () . map (| c | c == 144u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ElectricalPowerMeasurement" , 144u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ElectricalPowerMeasurement" , 144u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `ElectricalPowerMeasurement` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct ElectricalPowerMeasurementCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > ElectricalPowerMeasurementCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ElectricalPowerMeasurement` cluster's commands. `use` this trait to call `.electrical_power_measurement_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`ElectricalPowerMeasurementCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ElectricalPowerMeasurementCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn electrical_power_measurement_inv (self ) -> ElectricalPowerMeasurementCmdRequestsView < P > ;
    
}
impl < P > ElectricalPowerMeasurementCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn electrical_power_measurement_inv (self ) -> ElectricalPowerMeasurementCmdRequestsView < P > {
        ElectricalPowerMeasurementCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `ElectricalPowerMeasurement` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ElectricalPowerMeasurementAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > ElectricalPowerMeasurementAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn power_mode (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn number_of_measurement_types (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accuracy (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn ranges (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn voltage (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn reactive_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn apparent_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_power (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn reactive_power (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn apparent_power (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rms_voltage (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (11 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rms_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (12 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rms_power (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (13 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn frequency (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (14 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn harmonic_currents (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (15 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn harmonic_phases (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (16 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn power_factor (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (17 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn neutral_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (18 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (144 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ElectricalPowerMeasurement` cluster's attribute reads. `use` this trait to call `.electrical_power_measurement_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`ElectricalPowerMeasurementAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ElectricalPowerMeasurementAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn electrical_power_measurement_read (self ) -> ElectricalPowerMeasurementAttrReadsView < P > ;
    
}
impl < P > ElectricalPowerMeasurementAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn electrical_power_measurement_read (self ) -> ElectricalPowerMeasurementAttrReadsView < P > {
        ElectricalPowerMeasurementAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `ElectricalPowerMeasurement` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ElectricalPowerMeasurementAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > ElectricalPowerMeasurementAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `ElectricalPowerMeasurement` cluster's attribute writes. `use` this trait to call `.electrical_power_measurement_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`ElectricalPowerMeasurementAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ElectricalPowerMeasurementAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn electrical_power_measurement_write (self ) -> ElectricalPowerMeasurementAttrWritesView < P > ;
    
}
impl < P > ElectricalPowerMeasurementAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn electrical_power_measurement_write (self ) -> ElectricalPowerMeasurementAttrWritesView < P > {
        ElectricalPowerMeasurementAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `ElectricalPowerMeasurement` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct ElectricalPowerMeasurementCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > ElectricalPowerMeasurementCmdResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `ElectricalPowerMeasurement`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.electrical_power_measurement_inv_resp()` on an `InvokeResp`; the returned [`ElectricalPowerMeasurementCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait ElectricalPowerMeasurementCmdResponses < 'a > {
    fn electrical_power_measurement_inv_resp (& self ) -> ElectricalPowerMeasurementCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > ElectricalPowerMeasurementCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn electrical_power_measurement_inv_resp (& self ) -> ElectricalPowerMeasurementCmdResponsesView < 'a , '_ > {
        ElectricalPowerMeasurementCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `ElectricalPowerMeasurement` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ElectricalPowerMeasurementAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > ElectricalPowerMeasurementAttrResponsesView < 'a , 'r > {
    pub fn power_mode (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < PowerModeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < PowerModeEnum > (144 , 0 ) 
    }
    pub fn number_of_measurement_types (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (144 , 1 ) 
    }
    pub fn accuracy (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , MeasurementAccuracyStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , MeasurementAccuracyStruct < 'a > > > (144 , 2 ) 
    }
    pub fn ranges (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , MeasurementRangeStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , MeasurementRangeStruct < 'a > > > (144 , 3 ) 
    }
    pub fn voltage (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > > (144 , 4 ) 
    }
    pub fn active_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > > (144 , 5 ) 
    }
    pub fn reactive_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > > (144 , 6 ) 
    }
    pub fn apparent_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > > (144 , 7 ) 
    }
    pub fn active_power (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > > (144 , 8 ) 
    }
    pub fn reactive_power (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVAR > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: PowerMilliVAR > > (144 , 9 ) 
    }
    pub fn apparent_power (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVA > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: PowerMilliVA > > (144 , 10 ) 
    }
    pub fn rms_voltage (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > > (144 , 11 ) 
    }
    pub fn rms_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > > (144 , 12 ) 
    }
    pub fn rms_power (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > > (144 , 13 ) 
    }
    pub fn frequency (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < i64 > > (144 , 14 ) 
    }
    pub fn harmonic_currents (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: TLVArray < 'a , HarmonicMeasurementStruct < 'a > > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: TLVArray < 'a , HarmonicMeasurementStruct < 'a > > > > (144 , 15 ) 
    }
    pub fn harmonic_phases (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: tlv :: TLVArray < 'a , HarmonicMeasurementStruct < 'a > > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: tlv :: TLVArray < 'a , HarmonicMeasurementStruct < 'a > > > > (144 , 16 ) 
    }
    pub fn power_factor (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < i64 > > (144 , 17 ) 
    }
    pub fn neutral_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > > (144 , 18 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (144 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (144 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (144 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (144 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (144 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `ElectricalPowerMeasurement`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.electrical_power_measurement_read_resp()` on a `ReportDataResp`; the returned [`ElectricalPowerMeasurementAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait ElectricalPowerMeasurementAttrResponses < 'a > {
    fn electrical_power_measurement_read_resp (& self ) -> ElectricalPowerMeasurementAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > ElectricalPowerMeasurementAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn electrical_power_measurement_read_resp (& self ) -> ElectricalPowerMeasurementAttrResponsesView < 'a , '_ > {
        ElectricalPowerMeasurementAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `ElectricalPowerMeasurement` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ElectricalPowerMeasurementWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > ElectricalPowerMeasurementWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `ElectricalPowerMeasurement`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.electrical_power_measurement_write_resp()` on a `WriteResp`; the returned [`ElectricalPowerMeasurementWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait ElectricalPowerMeasurementWriteResponses < 'a > {
    fn electrical_power_measurement_write_resp (& self ) -> ElectricalPowerMeasurementWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > ElectricalPowerMeasurementWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn electrical_power_measurement_write_resp (& self ) -> ElectricalPowerMeasurementWriteResponsesView < 'a , '_ > {
        ElectricalPowerMeasurementWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `ElectricalPowerMeasurement` cluster's client operations. Returned by [`ElectricalPowerMeasurementClient::electrical_power_measurement`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct ElectricalPowerMeasurementClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > ElectricalPowerMeasurementClientView < T > {
    pub fn power_mode_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < PowerModeEnum , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . power_mode (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn number_of_measurement_types_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . number_of_measurement_types (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn accuracy_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , MeasurementAccuracyStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . accuracy (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn ranges_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , MeasurementRangeStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . ranges (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn voltage_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . voltage (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn active_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . active_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn reactive_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . reactive_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn apparent_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . apparent_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn active_power_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . active_power (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn reactive_power_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVAR > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . reactive_power (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn apparent_power_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVA > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . apparent_power (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rms_voltage_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . rms_voltage (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rms_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . rms_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn rms_power_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . rms_power (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn frequency_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . frequency (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn harmonic_currents_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: TLVArray < '_ , HarmonicMeasurementStruct < '_ > > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . harmonic_currents (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn harmonic_phases_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < crate :: tlv :: TLVArray < '_ , HarmonicMeasurementStruct < '_ > > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . harmonic_phases (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn power_factor_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . power_factor (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn neutral_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . neutral_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ElectricalPowerMeasurementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . electrical_power_measurement_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `ElectricalPowerMeasurement` cluster. `use` this trait to call `.electrical_power_measurement()` on any [`crate::im::ImClient`], such as an exchange; the returned [`ElectricalPowerMeasurementClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `ElectricalPowerMeasurementClientView` keeps each cluster's API surface narrow: at the call site `exchange.electrical_power_measurement().` shows only this cluster's methods in IDE completion." ] pub trait ElectricalPowerMeasurementClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn electrical_power_measurement (self ) -> ElectricalPowerMeasurementClientView < Self > {
        ElectricalPowerMeasurementClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > ElectricalPowerMeasurementClient for T {
    
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
    fn power_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < PowerModeEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn number_of_measurement_types (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn accuracy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementAccuracyStructArrayBuilder < P > , MeasurementAccuracyStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn ranges < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementRangeStructArrayBuilder < P > , MeasurementRangeStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn reactive_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn apparent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn reactive_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVAR > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn apparent_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVA > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rms_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rms_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rms_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn harmonic_currents < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn harmonic_phases < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn power_factor (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn neutral_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
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
    fn power_mode (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < PowerModeEnum , crate :: error :: Error >> {
        T :: power_mode (self , ctx ) 
    }
    fn number_of_measurement_types (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: number_of_measurement_types (self , ctx ) 
    }
    fn accuracy < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementAccuracyStructArrayBuilder < P > , MeasurementAccuracyStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: accuracy (self , ctx , builder ) 
    }
    fn ranges < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < MeasurementRangeStructArrayBuilder < P > , MeasurementRangeStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: ranges (self , ctx , builder ) 
    }
    fn voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error >> {
        T :: voltage (self , ctx ) 
    }
    fn active_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        T :: active_current (self , ctx ) 
    }
    fn reactive_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        T :: reactive_current (self , ctx ) 
    }
    fn apparent_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        T :: apparent_current (self , ctx ) 
    }
    fn active_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error >> {
        T :: active_power (self , ctx ) 
    }
    fn reactive_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVAR > , crate :: error :: Error >> {
        T :: reactive_power (self , ctx ) 
    }
    fn apparent_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliVA > , crate :: error :: Error >> {
        T :: apparent_power (self , ctx ) 
    }
    fn rms_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: VoltageMilliV > , crate :: error :: Error >> {
        T :: rms_voltage (self , ctx ) 
    }
    fn rms_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        T :: rms_current (self , ctx ) 
    }
    fn rms_power (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: PowerMilliW > , crate :: error :: Error >> {
        T :: rms_power (self , ctx ) 
    }
    fn frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error >> {
        T :: frequency (self , ctx ) 
    }
    fn harmonic_currents < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: harmonic_currents (self , ctx , builder ) 
    }
    fn harmonic_phases < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: NullableBuilder < P , HarmonicMeasurementStructArrayBuilder < P > > , HarmonicMeasurementStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: harmonic_phases (self , ctx , builder ) 
    }
    fn power_factor (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error >> {
        T :: power_factor (self , ctx ) 
    }
    fn neutral_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < crate :: im :: AmperageMilliA > , crate :: error :: Error >> {
        T :: neutral_current (self , ctx ) 
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
                    AttributeId :: PowerMode => {
                        let attr_read_result = self . 0. power_mode (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PowerMode , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NumberOfMeasurementTypes => {
                        let attr_read_result = self . 0. number_of_measurement_types (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NumberOfMeasurementTypes , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Accuracy => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Accuracy , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. accuracy (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Accuracy , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Accuracy , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Ranges => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Ranges , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. ranges (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Ranges , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Ranges , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Voltage => {
                        let attr_read_result = self . 0. voltage (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Voltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveCurrent => {
                        let attr_read_result = self . 0. active_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ReactiveCurrent => {
                        let attr_read_result = self . 0. reactive_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ReactiveCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ApparentCurrent => {
                        let attr_read_result = self . 0. apparent_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ApparentCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActivePower => {
                        let attr_read_result = self . 0. active_power (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActivePower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ReactivePower => {
                        let attr_read_result = self . 0. reactive_power (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ReactivePower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ApparentPower => {
                        let attr_read_result = self . 0. apparent_power (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ApparentPower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RMSVoltage => {
                        let attr_read_result = self . 0. rms_voltage (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RMSVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RMSCurrent => {
                        let attr_read_result = self . 0. rms_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RMSCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: RMSPower => {
                        let attr_read_result = self . 0. rms_power (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RMSPower , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Frequency => {
                        let attr_read_result = self . 0. frequency (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Frequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HarmonicCurrents => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicCurrents , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. harmonic_currents (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicCurrents , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicCurrents , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: HarmonicPhases => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicPhases , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. harmonic_phases (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicPhases , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HarmonicPhases , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: PowerFactor => {
                        let attr_read_result = self . 0. power_factor (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: PowerFactor , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NeutralCurrent => {
                        let attr_read_result = self . 0. neutral_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NeutralCurrent , false ) ) ) ) , & attr_read_result ) ;
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
        if ctx . cluster () . map (| c | c == 144u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ElectricalPowerMeasurement" , 144u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "ElectricalPowerMeasurement" , 144u32 , self . 0.2 ) 
    }
    
}
