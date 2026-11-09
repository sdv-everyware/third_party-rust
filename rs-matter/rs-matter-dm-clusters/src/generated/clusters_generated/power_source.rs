# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const WIRED = 1 ;
        const BATTERY = 2 ;
        const RECHARGEABLE = 4 ;
        const REPLACEABLE = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const WIRED = 1 ;
        const BATTERY = 2 ;
        const RECHARGEABLE = 4 ;
        const REPLACEABLE = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u16" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u16 ) ] pub enum BatApprovedChemistryEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] Alkaline = 1 , # [enumval (2 ) ] LithiumCarbonFluoride = 2 , # [enumval (3 ) ] LithiumChromiumOxide = 3 , # [enumval (4 ) ] LithiumCopperOxide = 4 , # [enumval (5 ) ] LithiumIronDisulfide = 5 , # [enumval (6 ) ] LithiumManganeseDioxide = 6 , # [enumval (7 ) ] LithiumThionylChloride = 7 , # [enumval (8 ) ] Magnesium = 8 , # [enumval (9 ) ] MercuryOxide = 9 , # [enumval (10 ) ] NickelOxyhydride = 10 , # [enumval (11 ) ] SilverOxide = 11 , # [enumval (12 ) ] ZincAir = 12 , # [enumval (13 ) ] ZincCarbon = 13 , # [enumval (14 ) ] ZincChloride = 14 , # [enumval (15 ) ] ZincManganeseDioxide = 15 , # [enumval (16 ) ] LeadAcid = 16 , # [enumval (17 ) ] LithiumCobaltOxide = 17 , # [enumval (18 ) ] LithiumIon = 18 , # [enumval (19 ) ] LithiumIonPolymer = 19 , # [enumval (20 ) ] LithiumIronPhosphate = 20 , # [enumval (21 ) ] LithiumSulfur = 21 , # [enumval (22 ) ] LithiumTitanate = 22 , # [enumval (23 ) ] NickelCadmium = 23 , # [enumval (24 ) ] NickelHydrogen = 24 , # [enumval (25 ) ] NickelIron = 25 , # [enumval (26 ) ] NickelMetalHydride = 26 , # [enumval (27 ) ] NickelZinc = 27 , # [enumval (28 ) ] SilverZinc = 28 , # [enumval (29 ) ] SodiumIon = 29 , # [enumval (30 ) ] SodiumSulfur = 30 , # [enumval (31 ) ] ZincBromide = 31 , # [enumval (32 ) ] ZincCerium = 32 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatChargeFaultEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] AmbientTooHot = 1 , # [enumval (2 ) ] AmbientTooCold = 2 , # [enumval (3 ) ] BatteryTooHot = 3 , # [enumval (4 ) ] BatteryTooCold = 4 , # [enumval (5 ) ] BatteryAbsent = 5 , # [enumval (6 ) ] BatteryOverVoltage = 6 , # [enumval (7 ) ] BatteryUnderVoltage = 7 , # [enumval (8 ) ] ChargerOverVoltage = 8 , # [enumval (9 ) ] ChargerUnderVoltage = 9 , # [enumval (10 ) ] SafetyTimeout = 10 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatChargeLevelEnum {
    # [enumval (0 ) ] OK = 0 , # [enumval (1 ) ] Warning = 1 , # [enumval (2 ) ] Critical = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatChargeStateEnum {
    # [enumval (0 ) ] Unknown = 0 , # [enumval (1 ) ] IsCharging = 1 , # [enumval (2 ) ] IsAtFullCharge = 2 , # [enumval (3 ) ] IsNotCharging = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u16" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u16 ) ] pub enum BatCommonDesignationEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] AAA = 1 , # [enumval (2 ) ] AA = 2 , # [enumval (3 ) ] C = 3 , # [enumval (4 ) ] D = 4 , # [enumval (5 ) ] V4v5 = 5 , # [enumval (6 ) ] V6v0 = 6 , # [enumval (7 ) ] V9v0 = 7 , # [enumval (8 ) ] V12AA = 8 , # [enumval (9 ) ] AAAA = 9 , # [enumval (10 ) ] A = 10 , # [enumval (11 ) ] B = 11 , # [enumval (12 ) ] F = 12 , # [enumval (13 ) ] N = 13 , # [enumval (14 ) ] No6 = 14 , # [enumval (15 ) ] SubC = 15 , # [enumval (16 ) ] A23 = 16 , # [enumval (17 ) ] A27 = 17 , # [enumval (18 ) ] BA5800 = 18 , # [enumval (19 ) ] Duplex = 19 , # [enumval (20 ) ] V4SR44 = 20 , # [enumval (21 ) ] V523 = 21 , # [enumval (22 ) ] V531 = 22 , # [enumval (23 ) ] V15v0 = 23 , # [enumval (24 ) ] V22v5 = 24 , # [enumval (25 ) ] V30v0 = 25 , # [enumval (26 ) ] V45v0 = 26 , # [enumval (27 ) ] V67v5 = 27 , # [enumval (28 ) ] J = 28 , # [enumval (29 ) ] CR123A = 29 , # [enumval (30 ) ] CR2 = 30 , # [enumval (31 ) ] V2CR5 = 31 , # [enumval (32 ) ] CRP2 = 32 , # [enumval (33 ) ] CRV3 = 33 , # [enumval (34 ) ] SR41 = 34 , # [enumval (35 ) ] SR43 = 35 , # [enumval (36 ) ] SR44 = 36 , # [enumval (37 ) ] SR45 = 37 , # [enumval (38 ) ] SR48 = 38 , # [enumval (39 ) ] SR54 = 39 , # [enumval (40 ) ] SR55 = 40 , # [enumval (41 ) ] SR57 = 41 , # [enumval (42 ) ] SR58 = 42 , # [enumval (43 ) ] SR59 = 43 , # [enumval (44 ) ] SR60 = 44 , # [enumval (45 ) ] SR63 = 45 , # [enumval (46 ) ] SR64 = 46 , # [enumval (47 ) ] SR65 = 47 , # [enumval (48 ) ] SR66 = 48 , # [enumval (49 ) ] SR67 = 49 , # [enumval (50 ) ] SR68 = 50 , # [enumval (51 ) ] SR69 = 51 , # [enumval (52 ) ] SR516 = 52 , # [enumval (53 ) ] SR731 = 53 , # [enumval (54 ) ] SR712 = 54 , # [enumval (55 ) ] LR932 = 55 , # [enumval (56 ) ] A5 = 56 , # [enumval (57 ) ] A10 = 57 , # [enumval (58 ) ] A13 = 58 , # [enumval (59 ) ] A312 = 59 , # [enumval (60 ) ] A675 = 60 , # [enumval (61 ) ] AC41E = 61 , # [enumval (62 ) ] V10180 = 62 , # [enumval (63 ) ] V10280 = 63 , # [enumval (64 ) ] V10440 = 64 , # [enumval (65 ) ] V14250 = 65 , # [enumval (66 ) ] V14430 = 66 , # [enumval (67 ) ] V14500 = 67 , # [enumval (68 ) ] V14650 = 68 , # [enumval (69 ) ] V15270 = 69 , # [enumval (70 ) ] V16340 = 70 , # [enumval (71 ) ] RCR123A = 71 , # [enumval (72 ) ] V17500 = 72 , # [enumval (73 ) ] V17670 = 73 , # [enumval (74 ) ] V18350 = 74 , # [enumval (75 ) ] V18500 = 75 , # [enumval (76 ) ] V18650 = 76 , # [enumval (77 ) ] V19670 = 77 , # [enumval (78 ) ] V25500 = 78 , # [enumval (79 ) ] V26650 = 79 , # [enumval (80 ) ] V32600 = 80 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatFaultEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] OverTemp = 1 , # [enumval (2 ) ] UnderTemp = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatReplaceabilityEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] NotReplaceable = 1 , # [enumval (2 ) ] UserReplaceable = 2 , # [enumval (3 ) ] FactoryReplaceable = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PowerSourceStatusEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] Active = 1 , # [enumval (2 ) ] Standby = 2 , # [enumval (3 ) ] Unavailable = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum WiredCurrentTypeEnum {
    # [enumval (0 ) ] AC = 0 , # [enumval (1 ) ] DC = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum WiredFaultEnum {
    # [enumval (0 ) ] Unspecified = 0 , # [enumval (1 ) ] OverVoltage = 1 , # [enumval (2 ) ] UnderVoltage = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatChargeFaultChangeTypeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatFaultChangeTypeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum WiredFaultChangeTypeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct BatChargeFaultChangeType < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > BatChargeFaultChangeType < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatChargeFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatChargeFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for BatChargeFaultChangeType < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for BatChargeFaultChangeType < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for BatChargeFaultChangeType < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "BatChargeFaultChangeType" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for BatChargeFaultChangeType < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "BatChargeFaultChangeType" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct BatFaultChangeType < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > BatFaultChangeType < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for BatFaultChangeType < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for BatFaultChangeType < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for BatFaultChangeType < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "BatFaultChangeType" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for BatFaultChangeType < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "BatFaultChangeType" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct WiredFaultChangeType < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > WiredFaultChangeType < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , WiredFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , WiredFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for WiredFaultChangeType < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for WiredFaultChangeType < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for WiredFaultChangeType < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "WiredFaultChangeType" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for WiredFaultChangeType < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "WiredFaultChangeType" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct BatChargeFaultChangeTypeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > BatChargeFaultChangeTypeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > BatChargeFaultChangeTypeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatChargeFaultChangeTypeBuilder < P , 1usize > , BatChargeFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatChargeFaultChangeTypeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > BatChargeFaultChangeTypeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatChargeFaultChangeTypeBuilder < P , 2usize > , BatChargeFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatChargeFaultChangeTypeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > BatChargeFaultChangeTypeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for BatChargeFaultChangeTypeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChangeType" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for BatChargeFaultChangeTypeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChangeType" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for BatChargeFaultChangeTypeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatChargeFaultChangeTypeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BatChargeFaultChangeTypeArrayBuilder < P > (P ) ;
impl < P > BatChargeFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < BatChargeFaultChangeTypeBuilder < BatChargeFaultChangeTypeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatChargeFaultChangeTypeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for BatChargeFaultChangeTypeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChangeType[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for BatChargeFaultChangeTypeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChangeType[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for BatChargeFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatChargeFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BatFaultChangeTypeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > BatFaultChangeTypeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > BatFaultChangeTypeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatFaultChangeTypeBuilder < P , 1usize > , BatFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatFaultChangeTypeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > BatFaultChangeTypeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatFaultChangeTypeBuilder < P , 2usize > , BatFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatFaultChangeTypeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > BatFaultChangeTypeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for BatFaultChangeTypeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatFaultChangeType" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for BatFaultChangeTypeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatFaultChangeType" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for BatFaultChangeTypeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatFaultChangeTypeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BatFaultChangeTypeArrayBuilder < P > (P ) ;
impl < P > BatFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < BatFaultChangeTypeBuilder < BatFaultChangeTypeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatFaultChangeTypeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for BatFaultChangeTypeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatFaultChangeType[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for BatFaultChangeTypeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatFaultChangeType[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for BatFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct WiredFaultChangeTypeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > WiredFaultChangeTypeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > WiredFaultChangeTypeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < WiredFaultChangeTypeBuilder < P , 1usize > , WiredFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WiredFaultChangeTypeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > WiredFaultChangeTypeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < WiredFaultChangeTypeBuilder < P , 2usize > , WiredFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WiredFaultChangeTypeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > WiredFaultChangeTypeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for WiredFaultChangeTypeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChangeType" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for WiredFaultChangeTypeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChangeType" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for WiredFaultChangeTypeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WiredFaultChangeTypeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct WiredFaultChangeTypeArrayBuilder < P > (P ) ;
impl < P > WiredFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < WiredFaultChangeTypeBuilder < WiredFaultChangeTypeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WiredFaultChangeTypeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for WiredFaultChangeTypeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChangeType[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for WiredFaultChangeTypeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChangeType[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for WiredFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WiredFaultChangeTypeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum WiredFaultChangeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatFaultChangeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum BatChargeFaultChangeTag {
    Current = 0 , Previous = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct WiredFaultChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > WiredFaultChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , WiredFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , WiredFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for WiredFaultChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for WiredFaultChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for WiredFaultChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "WiredFaultChange" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for WiredFaultChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "WiredFaultChange" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct BatFaultChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > BatFaultChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for BatFaultChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for BatFaultChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for BatFaultChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "BatFaultChange" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for BatFaultChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "BatFaultChange" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct BatChargeFaultChange < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > BatChargeFaultChange < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn current (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatChargeFaultEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn previous (& self ) -> Result < crate :: tlv :: TLVArray < '_ , BatChargeFaultEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for BatChargeFaultChange < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for BatChargeFaultChange < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for BatChargeFaultChange < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "BatChargeFaultChange" ) ? ;
        match self . current () {
            Ok (value ) => write ! (f , "{}: {:?}," , "current" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "current" , e . code () ) ? , 
        }
        match self . previous () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for BatChargeFaultChange < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "BatChargeFaultChange" ) ;
        match self . current () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "current" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "current" , e . code () ) , 
        }
        match self . previous () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct WiredFaultChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > WiredFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > WiredFaultChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < WiredFaultChangeBuilder < P , 1usize > , WiredFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WiredFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > WiredFaultChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < WiredFaultChangeBuilder < P , 2usize > , WiredFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WiredFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > WiredFaultChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for WiredFaultChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for WiredFaultChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for WiredFaultChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WiredFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct WiredFaultChangeArrayBuilder < P > (P ) ;
impl < P > WiredFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < WiredFaultChangeBuilder < WiredFaultChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (WiredFaultChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for WiredFaultChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for WiredFaultChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "WiredFaultChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for WiredFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for WiredFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BatFaultChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > BatFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > BatFaultChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatFaultChangeBuilder < P , 1usize > , BatFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > BatFaultChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatFaultChangeBuilder < P , 2usize > , BatFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > BatFaultChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for BatFaultChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatFaultChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for BatFaultChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatFaultChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for BatFaultChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BatFaultChangeArrayBuilder < P > (P ) ;
impl < P > BatFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < BatFaultChangeBuilder < BatFaultChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatFaultChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for BatFaultChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatFaultChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for BatFaultChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatFaultChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for BatFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BatChargeFaultChangeBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > BatChargeFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > BatChargeFaultChangeBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatChargeFaultChangeBuilder < P , 1usize > , BatChargeFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatChargeFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > BatChargeFaultChangeBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < BatChargeFaultChangeBuilder < P , 2usize > , BatChargeFaultEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatChargeFaultChangeBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > BatChargeFaultChangeBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for BatChargeFaultChangeBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChange" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for BatChargeFaultChangeBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChange" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for BatChargeFaultChangeBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatChargeFaultChangeBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct BatChargeFaultChangeArrayBuilder < P > (P ) ;
impl < P > BatChargeFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < BatChargeFaultChangeBuilder < BatChargeFaultChangeArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (BatChargeFaultChangeArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for BatChargeFaultChangeArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChange[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for BatChargeFaultChangeArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "BatChargeFaultChange[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for BatChargeFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for BatChargeFaultChangeArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl WiredFaultChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (WiredFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 47 , EventId :: WiredFaultChange as _ , Self :: PRIORITY , | tw | {
            f (WiredFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (WiredFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: WiredFaultChange as _ , Self :: PRIORITY , | tw | {
            f (WiredFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl BatFaultChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (BatFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 47 , EventId :: BatFaultChange as _ , Self :: PRIORITY , | tw | {
            f (BatFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (BatFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: BatFaultChange as _ , Self :: PRIORITY , | tw | {
            f (BatFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl BatChargeFaultChange < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (BatChargeFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 47 , EventId :: BatChargeFaultChange as _ , Self :: PRIORITY , | tw | {
            f (BatChargeFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (BatChargeFaultChangeBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: BatChargeFaultChange as _ , Self :: PRIORITY , | tw | {
            f (BatChargeFaultChangeBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    Status = 0 , Order = 1 , Description = 2 , WiredAssessedInputVoltage = 3 , WiredAssessedInputFrequency = 4 , WiredCurrentType = 5 , WiredAssessedCurrent = 6 , WiredNominalVoltage = 7 , WiredMaximumCurrent = 8 , WiredPresent = 9 , ActiveWiredFaults = 10 , BatVoltage = 11 , BatPercentRemaining = 12 , BatTimeRemaining = 13 , BatChargeLevel = 14 , BatReplacementNeeded = 15 , BatReplaceability = 16 , BatPresent = 17 , ActiveBatFaults = 18 , BatReplacementDescription = 19 , BatCommonDesignation = 20 , BatANSIDesignation = 21 , BatIECDesignation = 22 , BatApprovedChemistry = 23 , BatCapacity = 24 , BatQuantity = 25 , BatChargeState = 26 , BatTimeToFullCharge = 27 , BatFunctionalWhileCharging = 28 , BatChargingCurrent = 29 , ActiveBatChargeFaults = 30 , EndpointList = 31 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: Status => write ! (f , "{}(0x{:02x})" , "Status" , AttributeId :: Status as u32 ) ? , AttributeId :: Order => write ! (f , "{}(0x{:02x})" , "Order" , AttributeId :: Order as u32 ) ? , AttributeId :: Description => write ! (f , "{}(0x{:02x})" , "Description" , AttributeId :: Description as u32 ) ? , AttributeId :: WiredAssessedInputVoltage => write ! (f , "{}(0x{:02x})" , "WiredAssessedInputVoltage" , AttributeId :: WiredAssessedInputVoltage as u32 ) ? , AttributeId :: WiredAssessedInputFrequency => write ! (f , "{}(0x{:02x})" , "WiredAssessedInputFrequency" , AttributeId :: WiredAssessedInputFrequency as u32 ) ? , AttributeId :: WiredCurrentType => write ! (f , "{}(0x{:02x})" , "WiredCurrentType" , AttributeId :: WiredCurrentType as u32 ) ? , AttributeId :: WiredAssessedCurrent => write ! (f , "{}(0x{:02x})" , "WiredAssessedCurrent" , AttributeId :: WiredAssessedCurrent as u32 ) ? , AttributeId :: WiredNominalVoltage => write ! (f , "{}(0x{:02x})" , "WiredNominalVoltage" , AttributeId :: WiredNominalVoltage as u32 ) ? , AttributeId :: WiredMaximumCurrent => write ! (f , "{}(0x{:02x})" , "WiredMaximumCurrent" , AttributeId :: WiredMaximumCurrent as u32 ) ? , AttributeId :: WiredPresent => write ! (f , "{}(0x{:02x})" , "WiredPresent" , AttributeId :: WiredPresent as u32 ) ? , AttributeId :: ActiveWiredFaults => write ! (f , "{}(0x{:02x})" , "ActiveWiredFaults" , AttributeId :: ActiveWiredFaults as u32 ) ? , AttributeId :: BatVoltage => write ! (f , "{}(0x{:02x})" , "BatVoltage" , AttributeId :: BatVoltage as u32 ) ? , AttributeId :: BatPercentRemaining => write ! (f , "{}(0x{:02x})" , "BatPercentRemaining" , AttributeId :: BatPercentRemaining as u32 ) ? , AttributeId :: BatTimeRemaining => write ! (f , "{}(0x{:02x})" , "BatTimeRemaining" , AttributeId :: BatTimeRemaining as u32 ) ? , AttributeId :: BatChargeLevel => write ! (f , "{}(0x{:02x})" , "BatChargeLevel" , AttributeId :: BatChargeLevel as u32 ) ? , AttributeId :: BatReplacementNeeded => write ! (f , "{}(0x{:02x})" , "BatReplacementNeeded" , AttributeId :: BatReplacementNeeded as u32 ) ? , AttributeId :: BatReplaceability => write ! (f , "{}(0x{:02x})" , "BatReplaceability" , AttributeId :: BatReplaceability as u32 ) ? , AttributeId :: BatPresent => write ! (f , "{}(0x{:02x})" , "BatPresent" , AttributeId :: BatPresent as u32 ) ? , AttributeId :: ActiveBatFaults => write ! (f , "{}(0x{:02x})" , "ActiveBatFaults" , AttributeId :: ActiveBatFaults as u32 ) ? , AttributeId :: BatReplacementDescription => write ! (f , "{}(0x{:02x})" , "BatReplacementDescription" , AttributeId :: BatReplacementDescription as u32 ) ? , AttributeId :: BatCommonDesignation => write ! (f , "{}(0x{:02x})" , "BatCommonDesignation" , AttributeId :: BatCommonDesignation as u32 ) ? , AttributeId :: BatANSIDesignation => write ! (f , "{}(0x{:02x})" , "BatANSIDesignation" , AttributeId :: BatANSIDesignation as u32 ) ? , AttributeId :: BatIECDesignation => write ! (f , "{}(0x{:02x})" , "BatIECDesignation" , AttributeId :: BatIECDesignation as u32 ) ? , AttributeId :: BatApprovedChemistry => write ! (f , "{}(0x{:02x})" , "BatApprovedChemistry" , AttributeId :: BatApprovedChemistry as u32 ) ? , AttributeId :: BatCapacity => write ! (f , "{}(0x{:02x})" , "BatCapacity" , AttributeId :: BatCapacity as u32 ) ? , AttributeId :: BatQuantity => write ! (f , "{}(0x{:02x})" , "BatQuantity" , AttributeId :: BatQuantity as u32 ) ? , AttributeId :: BatChargeState => write ! (f , "{}(0x{:02x})" , "BatChargeState" , AttributeId :: BatChargeState as u32 ) ? , AttributeId :: BatTimeToFullCharge => write ! (f , "{}(0x{:02x})" , "BatTimeToFullCharge" , AttributeId :: BatTimeToFullCharge as u32 ) ? , AttributeId :: BatFunctionalWhileCharging => write ! (f , "{}(0x{:02x})" , "BatFunctionalWhileCharging" , AttributeId :: BatFunctionalWhileCharging as u32 ) ? , AttributeId :: BatChargingCurrent => write ! (f , "{}(0x{:02x})" , "BatChargingCurrent" , AttributeId :: BatChargingCurrent as u32 ) ? , AttributeId :: ActiveBatChargeFaults => write ! (f , "{}(0x{:02x})" , "ActiveBatChargeFaults" , AttributeId :: ActiveBatChargeFaults as u32 ) ? , AttributeId :: EndpointList => write ! (f , "{}(0x{:02x})" , "EndpointList" , AttributeId :: EndpointList as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: Status => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Status" , AttributeId :: Status as u32 ) , AttributeId :: Order => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Order" , AttributeId :: Order as u32 ) , AttributeId :: Description => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Description" , AttributeId :: Description as u32 ) , AttributeId :: WiredAssessedInputVoltage => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredAssessedInputVoltage" , AttributeId :: WiredAssessedInputVoltage as u32 ) , AttributeId :: WiredAssessedInputFrequency => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredAssessedInputFrequency" , AttributeId :: WiredAssessedInputFrequency as u32 ) , AttributeId :: WiredCurrentType => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredCurrentType" , AttributeId :: WiredCurrentType as u32 ) , AttributeId :: WiredAssessedCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredAssessedCurrent" , AttributeId :: WiredAssessedCurrent as u32 ) , AttributeId :: WiredNominalVoltage => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredNominalVoltage" , AttributeId :: WiredNominalVoltage as u32 ) , AttributeId :: WiredMaximumCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredMaximumCurrent" , AttributeId :: WiredMaximumCurrent as u32 ) , AttributeId :: WiredPresent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredPresent" , AttributeId :: WiredPresent as u32 ) , AttributeId :: ActiveWiredFaults => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveWiredFaults" , AttributeId :: ActiveWiredFaults as u32 ) , AttributeId :: BatVoltage => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatVoltage" , AttributeId :: BatVoltage as u32 ) , AttributeId :: BatPercentRemaining => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatPercentRemaining" , AttributeId :: BatPercentRemaining as u32 ) , AttributeId :: BatTimeRemaining => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatTimeRemaining" , AttributeId :: BatTimeRemaining as u32 ) , AttributeId :: BatChargeLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatChargeLevel" , AttributeId :: BatChargeLevel as u32 ) , AttributeId :: BatReplacementNeeded => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatReplacementNeeded" , AttributeId :: BatReplacementNeeded as u32 ) , AttributeId :: BatReplaceability => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatReplaceability" , AttributeId :: BatReplaceability as u32 ) , AttributeId :: BatPresent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatPresent" , AttributeId :: BatPresent as u32 ) , AttributeId :: ActiveBatFaults => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveBatFaults" , AttributeId :: ActiveBatFaults as u32 ) , AttributeId :: BatReplacementDescription => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatReplacementDescription" , AttributeId :: BatReplacementDescription as u32 ) , AttributeId :: BatCommonDesignation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatCommonDesignation" , AttributeId :: BatCommonDesignation as u32 ) , AttributeId :: BatANSIDesignation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatANSIDesignation" , AttributeId :: BatANSIDesignation as u32 ) , AttributeId :: BatIECDesignation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatIECDesignation" , AttributeId :: BatIECDesignation as u32 ) , AttributeId :: BatApprovedChemistry => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatApprovedChemistry" , AttributeId :: BatApprovedChemistry as u32 ) , AttributeId :: BatCapacity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatCapacity" , AttributeId :: BatCapacity as u32 ) , AttributeId :: BatQuantity => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatQuantity" , AttributeId :: BatQuantity as u32 ) , AttributeId :: BatChargeState => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatChargeState" , AttributeId :: BatChargeState as u32 ) , AttributeId :: BatTimeToFullCharge => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatTimeToFullCharge" , AttributeId :: BatTimeToFullCharge as u32 ) , AttributeId :: BatFunctionalWhileCharging => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatFunctionalWhileCharging" , AttributeId :: BatFunctionalWhileCharging as u32 ) , AttributeId :: BatChargingCurrent => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatChargingCurrent" , AttributeId :: BatChargingCurrent as u32 ) , AttributeId :: ActiveBatChargeFaults => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ActiveBatChargeFaults" , AttributeId :: ActiveBatChargeFaults as u32 ) , AttributeId :: EndpointList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EndpointList" , AttributeId :: EndpointList as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    WiredFaultChange = 0 , BatFaultChange = 1 , BatChargeFaultChange = 2 
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
            EventId :: WiredFaultChange => write ! (f , "{}(0x{:02x})" , "WiredFaultChange" , EventId :: WiredFaultChange as u32 ) ? , EventId :: BatFaultChange => write ! (f , "{}(0x{:02x})" , "BatFaultChange" , EventId :: BatFaultChange as u32 ) ? , EventId :: BatChargeFaultChange => write ! (f , "{}(0x{:02x})" , "BatChargeFaultChange" , EventId :: BatChargeFaultChange as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: WiredFaultChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "WiredFaultChange" , EventId :: WiredFaultChange as u32 ) , EventId :: BatFaultChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatFaultChange" , EventId :: BatFaultChange as u32 ) , EventId :: BatChargeFaultChange => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "BatChargeFaultChange" , EventId :: BatChargeFaultChange as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (47 , 3 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: Status as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Order as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: Description as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: WiredAssessedInputVoltage as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WiredAssessedInputFrequency as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WiredCurrentType as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WiredAssessedCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WiredNominalVoltage as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WiredMaximumCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: WiredPresent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveWiredFaults as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: BatVoltage as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatPercentRemaining as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatTimeRemaining as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatChargeLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatReplacementNeeded as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatReplaceability as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatPresent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveBatFaults as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: BatReplacementDescription as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatCommonDesignation as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatANSIDesignation as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatIECDesignation as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatApprovedChemistry as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatCapacity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatQuantity as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatChargeState as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatTimeToFullCharge as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatFunctionalWhileCharging as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: BatChargingCurrent as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ActiveBatChargeFaults as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: EndpointList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [] , & [crate :: dm :: Event :: new (EventId :: WiredFaultChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: BatFaultChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: BatChargeFaultChange as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn status (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < PowerSourceStatusEnum , crate :: error :: Error > ;
    fn order (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > ;
    fn wired_assessed_input_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wired_assessed_input_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wired_current_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < WiredCurrentTypeEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wired_assessed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wired_nominal_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wired_maximum_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn wired_present (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_wired_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiredFaultEnum > , crate :: tlv :: ToTLVBuilder < P , WiredFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_percent_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_time_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_charge_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatChargeLevelEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_replacement_needed (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_replaceability (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatReplaceabilityEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_present (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_bat_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_replacement_description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_common_designation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatCommonDesignationEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_ansi_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_iec_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_approved_chemistry (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatApprovedChemistryEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_quantity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_charge_state (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatChargeStateEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_time_to_full_charge (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_functional_while_charging (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn bat_charging_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn active_bat_charge_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatChargeFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatChargeFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn endpoint_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , crate :: im :: EndptId > , crate :: tlv :: ToTLVBuilder < P , crate :: im :: EndptId > > ) -> Result < P , crate :: error :: Error > ;
    
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
    fn status (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < PowerSourceStatusEnum , crate :: error :: Error > {
        T :: status (self , ctx ) 
    }
    fn order (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: order (self , ctx ) 
    }
    fn description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: description (self , ctx , builder ) 
    }
    fn wired_assessed_input_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: wired_assessed_input_voltage (self , ctx ) 
    }
    fn wired_assessed_input_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        T :: wired_assessed_input_frequency (self , ctx ) 
    }
    fn wired_current_type (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < WiredCurrentTypeEnum , crate :: error :: Error > {
        T :: wired_current_type (self , ctx ) 
    }
    fn wired_assessed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: wired_assessed_current (self , ctx ) 
    }
    fn wired_nominal_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: wired_nominal_voltage (self , ctx ) 
    }
    fn wired_maximum_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: wired_maximum_current (self , ctx ) 
    }
    fn wired_present (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: wired_present (self , ctx ) 
    }
    fn active_wired_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiredFaultEnum > , crate :: tlv :: ToTLVBuilder < P , WiredFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: active_wired_faults (self , ctx , builder ) 
    }
    fn bat_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: bat_voltage (self , ctx ) 
    }
    fn bat_percent_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: bat_percent_remaining (self , ctx ) 
    }
    fn bat_time_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: bat_time_remaining (self , ctx ) 
    }
    fn bat_charge_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatChargeLevelEnum , crate :: error :: Error > {
        T :: bat_charge_level (self , ctx ) 
    }
    fn bat_replacement_needed (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: bat_replacement_needed (self , ctx ) 
    }
    fn bat_replaceability (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatReplaceabilityEnum , crate :: error :: Error > {
        T :: bat_replaceability (self , ctx ) 
    }
    fn bat_present (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: bat_present (self , ctx ) 
    }
    fn active_bat_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: active_bat_faults (self , ctx , builder ) 
    }
    fn bat_replacement_description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: bat_replacement_description (self , ctx , builder ) 
    }
    fn bat_common_designation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatCommonDesignationEnum , crate :: error :: Error > {
        T :: bat_common_designation (self , ctx ) 
    }
    fn bat_ansi_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: bat_ansi_designation (self , ctx , builder ) 
    }
    fn bat_iec_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: bat_iec_designation (self , ctx , builder ) 
    }
    fn bat_approved_chemistry (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatApprovedChemistryEnum , crate :: error :: Error > {
        T :: bat_approved_chemistry (self , ctx ) 
    }
    fn bat_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: bat_capacity (self , ctx ) 
    }
    fn bat_quantity (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: bat_quantity (self , ctx ) 
    }
    fn bat_charge_state (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < BatChargeStateEnum , crate :: error :: Error > {
        T :: bat_charge_state (self , ctx ) 
    }
    fn bat_time_to_full_charge (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: bat_time_to_full_charge (self , ctx ) 
    }
    fn bat_functional_while_charging (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: bat_functional_while_charging (self , ctx ) 
    }
    fn bat_charging_current (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        T :: bat_charging_current (self , ctx ) 
    }
    fn active_bat_charge_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatChargeFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatChargeFaultEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: active_bat_charge_faults (self , ctx , builder ) 
    }
    fn endpoint_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , crate :: im :: EndptId > , crate :: tlv :: ToTLVBuilder < P , crate :: im :: EndptId > > ) -> Result < P , crate :: error :: Error > {
        T :: endpoint_list (self , ctx , builder ) 
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
                    AttributeId :: Status => {
                        let attr_read_result = self . 0. status (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Status , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Order => {
                        let attr_read_result = self . 0. order (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Order , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Description => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Description , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. description (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Description , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Description , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: WiredAssessedInputVoltage => {
                        let attr_read_result = self . 0. wired_assessed_input_voltage (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredAssessedInputVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredAssessedInputFrequency => {
                        let attr_read_result = self . 0. wired_assessed_input_frequency (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredAssessedInputFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredCurrentType => {
                        let attr_read_result = self . 0. wired_current_type (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredCurrentType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredAssessedCurrent => {
                        let attr_read_result = self . 0. wired_assessed_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredAssessedCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredNominalVoltage => {
                        let attr_read_result = self . 0. wired_nominal_voltage (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredNominalVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredMaximumCurrent => {
                        let attr_read_result = self . 0. wired_maximum_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredMaximumCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredPresent => {
                        let attr_read_result = self . 0. wired_present (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredPresent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveWiredFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveWiredFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_wired_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveWiredFaults , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveWiredFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatVoltage => {
                        let attr_read_result = self . 0. bat_voltage (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatPercentRemaining => {
                        let attr_read_result = self . 0. bat_percent_remaining (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatPercentRemaining , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatTimeRemaining => {
                        let attr_read_result = self . 0. bat_time_remaining (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatTimeRemaining , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatChargeLevel => {
                        let attr_read_result = self . 0. bat_charge_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatChargeLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatReplacementNeeded => {
                        let attr_read_result = self . 0. bat_replacement_needed (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementNeeded , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatReplaceability => {
                        let attr_read_result = self . 0. bat_replaceability (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplaceability , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatPresent => {
                        let attr_read_result = self . 0. bat_present (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatPresent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveBatFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_bat_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatFaults , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatReplacementDescription => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementDescription , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. bat_replacement_description (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementDescription , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementDescription , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatCommonDesignation => {
                        let attr_read_result = self . 0. bat_common_designation (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatCommonDesignation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatANSIDesignation => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatANSIDesignation , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. bat_ansi_designation (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatANSIDesignation , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatANSIDesignation , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatIECDesignation => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatIECDesignation , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. bat_iec_designation (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatIECDesignation , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatIECDesignation , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatApprovedChemistry => {
                        let attr_read_result = self . 0. bat_approved_chemistry (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatApprovedChemistry , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatCapacity => {
                        let attr_read_result = self . 0. bat_capacity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatCapacity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatQuantity => {
                        let attr_read_result = self . 0. bat_quantity (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatQuantity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatChargeState => {
                        let attr_read_result = self . 0. bat_charge_state (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatChargeState , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatTimeToFullCharge => {
                        let attr_read_result = self . 0. bat_time_to_full_charge (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatTimeToFullCharge , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatFunctionalWhileCharging => {
                        let attr_read_result = self . 0. bat_functional_while_charging (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatFunctionalWhileCharging , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatChargingCurrent => {
                        let attr_read_result = self . 0. bat_charging_current (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatChargingCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveBatChargeFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatChargeFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_bat_charge_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatChargeFaults , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatChargeFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: EndpointList => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EndpointList , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. endpoint_list (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EndpointList , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EndpointList , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
        crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
        return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 47u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PowerSource" , 47u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PowerSource" , 47u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `PowerSource` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct PowerSourceCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > PowerSourceCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `PowerSource` cluster's commands. `use` this trait to call `.power_source_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`PowerSourceCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait PowerSourceCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn power_source_inv (self ) -> PowerSourceCmdRequestsView < P > ;
    
}
impl < P > PowerSourceCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn power_source_inv (self ) -> PowerSourceCmdRequestsView < P > {
        PowerSourceCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `PowerSource` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PowerSourceAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > PowerSourceAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn status (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn order (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn description (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wired_assessed_input_voltage (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wired_assessed_input_frequency (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wired_current_type (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wired_assessed_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wired_nominal_voltage (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wired_maximum_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn wired_present (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_wired_faults (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_voltage (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (11 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_percent_remaining (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (12 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_time_remaining (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (13 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_charge_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (14 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_replacement_needed (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (15 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_replaceability (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (16 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_present (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (17 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_bat_faults (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (18 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_replacement_description (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (19 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_common_designation (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (20 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_ansi_designation (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (21 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_iec_designation (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (22 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_approved_chemistry (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (23 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_capacity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (24 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_quantity (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (25 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_charge_state (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (26 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_time_to_full_charge (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (27 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_functional_while_charging (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (28 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn bat_charging_current (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (29 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn active_bat_charge_faults (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (30 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn endpoint_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (31 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (47 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `PowerSource` cluster's attribute reads. `use` this trait to call `.power_source_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`PowerSourceAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait PowerSourceAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn power_source_read (self ) -> PowerSourceAttrReadsView < P > ;
    
}
impl < P > PowerSourceAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn power_source_read (self ) -> PowerSourceAttrReadsView < P > {
        PowerSourceAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `PowerSource` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PowerSourceAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > PowerSourceAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `PowerSource` cluster's attribute writes. `use` this trait to call `.power_source_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`PowerSourceAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait PowerSourceAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn power_source_write (self ) -> PowerSourceAttrWritesView < P > ;
    
}
impl < P > PowerSourceAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn power_source_write (self ) -> PowerSourceAttrWritesView < P > {
        PowerSourceAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `PowerSource` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct PowerSourceCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > PowerSourceCmdResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `PowerSource`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.power_source_inv_resp()` on an `InvokeResp`; the returned [`PowerSourceCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait PowerSourceCmdResponses < 'a > {
    fn power_source_inv_resp (& self ) -> PowerSourceCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > PowerSourceCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn power_source_inv_resp (& self ) -> PowerSourceCmdResponsesView < 'a , '_ > {
        PowerSourceCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `PowerSource` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PowerSourceAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > PowerSourceAttrResponsesView < 'a , 'r > {
    pub fn status (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < PowerSourceStatusEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < PowerSourceStatusEnum > (47 , 0 ) 
    }
    pub fn order (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (47 , 1 ) 
    }
    pub fn description (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Utf8Str < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Utf8Str < 'a > > (47 , 2 ) 
    }
    pub fn wired_assessed_input_voltage (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (47 , 3 ) 
    }
    pub fn wired_assessed_input_frequency (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u16 > > (47 , 4 ) 
    }
    pub fn wired_current_type (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < WiredCurrentTypeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < WiredCurrentTypeEnum > (47 , 5 ) 
    }
    pub fn wired_assessed_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (47 , 6 ) 
    }
    pub fn wired_nominal_voltage (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (47 , 7 ) 
    }
    pub fn wired_maximum_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (47 , 8 ) 
    }
    pub fn wired_present (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (47 , 9 ) 
    }
    pub fn active_wired_faults (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , WiredFaultEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , WiredFaultEnum > > (47 , 10 ) 
    }
    pub fn bat_voltage (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (47 , 11 ) 
    }
    pub fn bat_percent_remaining (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (47 , 12 ) 
    }
    pub fn bat_time_remaining (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (47 , 13 ) 
    }
    pub fn bat_charge_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < BatChargeLevelEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < BatChargeLevelEnum > (47 , 14 ) 
    }
    pub fn bat_replacement_needed (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (47 , 15 ) 
    }
    pub fn bat_replaceability (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < BatReplaceabilityEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < BatReplaceabilityEnum > (47 , 16 ) 
    }
    pub fn bat_present (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (47 , 17 ) 
    }
    pub fn active_bat_faults (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , BatFaultEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , BatFaultEnum > > (47 , 18 ) 
    }
    pub fn bat_replacement_description (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Utf8Str < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Utf8Str < 'a > > (47 , 19 ) 
    }
    pub fn bat_common_designation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < BatCommonDesignationEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < BatCommonDesignationEnum > (47 , 20 ) 
    }
    pub fn bat_ansi_designation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Utf8Str < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Utf8Str < 'a > > (47 , 21 ) 
    }
    pub fn bat_iec_designation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Utf8Str < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Utf8Str < 'a > > (47 , 22 ) 
    }
    pub fn bat_approved_chemistry (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < BatApprovedChemistryEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < BatApprovedChemistryEnum > (47 , 23 ) 
    }
    pub fn bat_capacity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (47 , 24 ) 
    }
    pub fn bat_quantity (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (47 , 25 ) 
    }
    pub fn bat_charge_state (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < BatChargeStateEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < BatChargeStateEnum > (47 , 26 ) 
    }
    pub fn bat_time_to_full_charge (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (47 , 27 ) 
    }
    pub fn bat_functional_while_charging (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (47 , 28 ) 
    }
    pub fn bat_charging_current (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u32 > > (47 , 29 ) 
    }
    pub fn active_bat_charge_faults (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , BatChargeFaultEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , BatChargeFaultEnum > > (47 , 30 ) 
    }
    pub fn endpoint_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: EndptId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: EndptId > > (47 , 31 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (47 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (47 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (47 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (47 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (47 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `PowerSource`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.power_source_read_resp()` on a `ReportDataResp`; the returned [`PowerSourceAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait PowerSourceAttrResponses < 'a > {
    fn power_source_read_resp (& self ) -> PowerSourceAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > PowerSourceAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn power_source_read_resp (& self ) -> PowerSourceAttrResponsesView < 'a , '_ > {
        PowerSourceAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `PowerSource` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PowerSourceWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > PowerSourceWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `PowerSource`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.power_source_write_resp()` on a `WriteResp`; the returned [`PowerSourceWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait PowerSourceWriteResponses < 'a > {
    fn power_source_write_resp (& self ) -> PowerSourceWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > PowerSourceWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn power_source_write_resp (& self ) -> PowerSourceWriteResponsesView < 'a , '_ > {
        PowerSourceWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `PowerSource` cluster's client operations. Returned by [`PowerSourceClient::power_source`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct PowerSourceClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > PowerSourceClientView < T > {
    pub fn status_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < PowerSourceStatusEnum , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . status (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn order_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . order (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn description_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . description (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn wired_assessed_input_voltage_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . wired_assessed_input_voltage (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wired_assessed_input_frequency_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . wired_assessed_input_frequency (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wired_current_type_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < WiredCurrentTypeEnum , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . wired_current_type (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wired_assessed_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . wired_assessed_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wired_nominal_voltage_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . wired_nominal_voltage (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wired_maximum_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . wired_maximum_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn wired_present_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . wired_present (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn active_wired_faults_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , WiredFaultEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . active_wired_faults (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn bat_voltage_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_voltage (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_percent_remaining_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_percent_remaining (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_time_remaining_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_time_remaining (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_charge_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < BatChargeLevelEnum , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_charge_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_replacement_needed_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_replacement_needed (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_replaceability_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < BatReplaceabilityEnum , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_replaceability (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_present_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_present (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn active_bat_faults_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , BatFaultEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . active_bat_faults (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn bat_replacement_description_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_replacement_description (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn bat_common_designation_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < BatCommonDesignationEnum , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_common_designation (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn bat_ansi_designation_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_ansi_designation (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn bat_iec_designation_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_iec_designation (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn bat_approved_chemistry_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < BatApprovedChemistryEnum , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_approved_chemistry (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_capacity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_capacity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_quantity_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_quantity (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_charge_state_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < BatChargeStateEnum , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_charge_state (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_time_to_full_charge_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_time_to_full_charge (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_functional_while_charging_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_functional_while_charging (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn bat_charging_current_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . bat_charging_current (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn active_bat_charge_faults_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , BatChargeFaultEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . active_bat_charge_faults (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn endpoint_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: EndptId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . endpoint_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: PowerSourceAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . power_source_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `PowerSource` cluster. `use` this trait to call `.power_source()` on any [`crate::im::ImClient`], such as an exchange; the returned [`PowerSourceClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `PowerSourceClientView` keeps each cluster's API surface narrow: at the call site `exchange.power_source().` shows only this cluster's methods in IDE completion." ] pub trait PowerSourceClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn power_source (self ) -> PowerSourceClientView < Self > {
        PowerSourceClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > PowerSourceClient for T {
    
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
    fn status (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < PowerSourceStatusEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn order (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn wired_assessed_input_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wired_assessed_input_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wired_current_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < WiredCurrentTypeEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wired_assessed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wired_nominal_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wired_maximum_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn wired_present (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_wired_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiredFaultEnum > , crate :: tlv :: ToTLVBuilder < P , WiredFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_percent_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_time_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_charge_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatChargeLevelEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_replacement_needed (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_replaceability (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatReplaceabilityEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_present (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_bat_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_replacement_description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_common_designation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatCommonDesignationEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_ansi_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_iec_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_approved_chemistry (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatApprovedChemistryEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_quantity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_charge_state (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatChargeStateEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_time_to_full_charge (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_functional_while_charging (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn bat_charging_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn active_bat_charge_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatChargeFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatChargeFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn endpoint_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , crate :: im :: EndptId > , crate :: tlv :: ToTLVBuilder < P , crate :: im :: EndptId > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn status (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < PowerSourceStatusEnum , crate :: error :: Error >> {
        T :: status (self , ctx ) 
    }
    fn order (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: order (self , ctx ) 
    }
    fn description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: description (self , ctx , builder ) 
    }
    fn wired_assessed_input_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: wired_assessed_input_voltage (self , ctx ) 
    }
    fn wired_assessed_input_frequency (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error >> {
        T :: wired_assessed_input_frequency (self , ctx ) 
    }
    fn wired_current_type (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < WiredCurrentTypeEnum , crate :: error :: Error >> {
        T :: wired_current_type (self , ctx ) 
    }
    fn wired_assessed_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: wired_assessed_current (self , ctx ) 
    }
    fn wired_nominal_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: wired_nominal_voltage (self , ctx ) 
    }
    fn wired_maximum_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: wired_maximum_current (self , ctx ) 
    }
    fn wired_present (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: wired_present (self , ctx ) 
    }
    fn active_wired_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , WiredFaultEnum > , crate :: tlv :: ToTLVBuilder < P , WiredFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_wired_faults (self , ctx , builder ) 
    }
    fn bat_voltage (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: bat_voltage (self , ctx ) 
    }
    fn bat_percent_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: bat_percent_remaining (self , ctx ) 
    }
    fn bat_time_remaining (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: bat_time_remaining (self , ctx ) 
    }
    fn bat_charge_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatChargeLevelEnum , crate :: error :: Error >> {
        T :: bat_charge_level (self , ctx ) 
    }
    fn bat_replacement_needed (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: bat_replacement_needed (self , ctx ) 
    }
    fn bat_replaceability (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatReplaceabilityEnum , crate :: error :: Error >> {
        T :: bat_replaceability (self , ctx ) 
    }
    fn bat_present (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: bat_present (self , ctx ) 
    }
    fn active_bat_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_bat_faults (self , ctx , builder ) 
    }
    fn bat_replacement_description < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: bat_replacement_description (self , ctx , builder ) 
    }
    fn bat_common_designation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatCommonDesignationEnum , crate :: error :: Error >> {
        T :: bat_common_designation (self , ctx ) 
    }
    fn bat_ansi_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: bat_ansi_designation (self , ctx , builder ) 
    }
    fn bat_iec_designation < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: Utf8StrBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: bat_iec_designation (self , ctx , builder ) 
    }
    fn bat_approved_chemistry (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatApprovedChemistryEnum , crate :: error :: Error >> {
        T :: bat_approved_chemistry (self , ctx ) 
    }
    fn bat_capacity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: bat_capacity (self , ctx ) 
    }
    fn bat_quantity (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: bat_quantity (self , ctx ) 
    }
    fn bat_charge_state (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < BatChargeStateEnum , crate :: error :: Error >> {
        T :: bat_charge_state (self , ctx ) 
    }
    fn bat_time_to_full_charge (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: bat_time_to_full_charge (self , ctx ) 
    }
    fn bat_functional_while_charging (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: bat_functional_while_charging (self , ctx ) 
    }
    fn bat_charging_current (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error >> {
        T :: bat_charging_current (self , ctx ) 
    }
    fn active_bat_charge_faults < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , BatChargeFaultEnum > , crate :: tlv :: ToTLVBuilder < P , BatChargeFaultEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: active_bat_charge_faults (self , ctx , builder ) 
    }
    fn endpoint_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , crate :: im :: EndptId > , crate :: tlv :: ToTLVBuilder < P , crate :: im :: EndptId > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: endpoint_list (self , ctx , builder ) 
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
                    AttributeId :: Status => {
                        let attr_read_result = self . 0. status (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Status , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Order => {
                        let attr_read_result = self . 0. order (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Order , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Description => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Description , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. description (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Description , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Description , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: WiredAssessedInputVoltage => {
                        let attr_read_result = self . 0. wired_assessed_input_voltage (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredAssessedInputVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredAssessedInputFrequency => {
                        let attr_read_result = self . 0. wired_assessed_input_frequency (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredAssessedInputFrequency , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredCurrentType => {
                        let attr_read_result = self . 0. wired_current_type (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredCurrentType , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredAssessedCurrent => {
                        let attr_read_result = self . 0. wired_assessed_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredAssessedCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredNominalVoltage => {
                        let attr_read_result = self . 0. wired_nominal_voltage (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredNominalVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredMaximumCurrent => {
                        let attr_read_result = self . 0. wired_maximum_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredMaximumCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: WiredPresent => {
                        let attr_read_result = self . 0. wired_present (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: WiredPresent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveWiredFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveWiredFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_wired_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveWiredFaults , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveWiredFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatVoltage => {
                        let attr_read_result = self . 0. bat_voltage (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatVoltage , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatPercentRemaining => {
                        let attr_read_result = self . 0. bat_percent_remaining (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatPercentRemaining , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatTimeRemaining => {
                        let attr_read_result = self . 0. bat_time_remaining (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatTimeRemaining , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatChargeLevel => {
                        let attr_read_result = self . 0. bat_charge_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatChargeLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatReplacementNeeded => {
                        let attr_read_result = self . 0. bat_replacement_needed (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementNeeded , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatReplaceability => {
                        let attr_read_result = self . 0. bat_replaceability (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplaceability , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatPresent => {
                        let attr_read_result = self . 0. bat_present (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatPresent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveBatFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_bat_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatFaults , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatReplacementDescription => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementDescription , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. bat_replacement_description (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementDescription , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatReplacementDescription , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatCommonDesignation => {
                        let attr_read_result = self . 0. bat_common_designation (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatCommonDesignation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatANSIDesignation => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatANSIDesignation , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. bat_ansi_designation (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatANSIDesignation , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatANSIDesignation , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatIECDesignation => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatIECDesignation , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. bat_iec_designation (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatIECDesignation , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatIECDesignation , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: BatApprovedChemistry => {
                        let attr_read_result = self . 0. bat_approved_chemistry (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatApprovedChemistry , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatCapacity => {
                        let attr_read_result = self . 0. bat_capacity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatCapacity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatQuantity => {
                        let attr_read_result = self . 0. bat_quantity (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatQuantity , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatChargeState => {
                        let attr_read_result = self . 0. bat_charge_state (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatChargeState , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatTimeToFullCharge => {
                        let attr_read_result = self . 0. bat_time_to_full_charge (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatTimeToFullCharge , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatFunctionalWhileCharging => {
                        let attr_read_result = self . 0. bat_functional_while_charging (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatFunctionalWhileCharging , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: BatChargingCurrent => {
                        let attr_read_result = self . 0. bat_charging_current (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: BatChargingCurrent , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ActiveBatChargeFaults => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatChargeFaults , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. active_bat_charge_faults (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatChargeFaults , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ActiveBatChargeFaults , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: EndpointList => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EndpointList , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. endpoint_list (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EndpointList , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: EndpointList , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
        crate :: dm :: log_cmd_not_supported (ctx . cmd () ) ;
        return Err (crate :: error :: ErrorCode :: CommandNotFound . into () ) ;
        Ok (() ) 
    }
    fn bump_dataver (& self , ctx : impl crate :: dm :: MatchContext ) {
        if ctx . cluster () . map (| c | c == 47u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PowerSource" , 47u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PowerSource" , 47u32 , self . 0.2 ) 
    }
    
}
