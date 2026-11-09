# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const CHANNEL_LIST = 1 ;
        const LINEUP_INFO = 2 ;
        const ELECTRONIC_GUIDE = 4 ;
        const RECORD_PROGRAM = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const CHANNEL_LIST = 1 ;
        const LINEUP_INFO = 2 ;
        const ELECTRONIC_GUIDE = 4 ;
        const RECORD_PROGRAM = 8 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct RecordingFlagBitmap : u32 {
        const SCHEDULED = 1 ;
        const RECORD_SERIES = 2 ;
        const RECORDED = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct RecordingFlagBitmap : u32 {
        const SCHEDULED = 1 ;
        const RECORD_SERIES = 2 ;
        const RECORDED = 4 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (RecordingFlagBitmap , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ChannelTypeEnum {
    # [enumval (0 ) ] Satellite = 0 , # [enumval (1 ) ] Cable = 1 , # [enumval (2 ) ] Terrestrial = 2 , # [enumval (3 ) ] OTT = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum LineupInfoTypeEnum {
    # [enumval (0 ) ] MSO = 0 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StatusEnum {
    # [enumval (0 ) ] Success = 0 , # [enumval (1 ) ] MultipleMatches = 1 , # [enumval (2 ) ] NoMatches = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProgramCastStructTag {
    Name = 0 , Role = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProgramCategoryStructTag {
    Category = 0 , SubCategory = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SeriesInfoStructTag {
    Season = 0 , Episode = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ChannelInfoStructTag {
    MajorNumber = 0 , MinorNumber = 1 , Name = 2 , CallSign = 3 , AffiliateCallSign = 4 , Identifier = 5 , Type = 6 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProgramStructTag {
    Identifier = 0 , Channel = 1 , StartTime = 2 , EndTime = 3 , Title = 4 , Subtitle = 5 , Description = 6 , AudioLanguages = 7 , Ratings = 8 , ThumbnailUrl = 9 , PosterArtUrl = 10 , DvbiUrl = 11 , ReleaseDate = 12 , ParentalGuidanceText = 13 , RecordingFlag = 14 , SeriesInfo = 15 , CategoryList = 16 , CastList = 17 , ExternalIdList = 18 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PageTokenStructTag {
    Limit = 0 , After = 1 , Before = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ChannelPagingStructTag {
    PreviousToken = 0 , NextToken = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AdditionalInfoStructTag {
    Name = 0 , Value = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum LineupInfoStructTag {
    OperatorName = 0 , LineupName = 1 , PostalCode = 2 , LineupInfoType = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ChangeChannelRequestTag {
    Match = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ChangeChannelResponseTag {
    Status = 0 , Data = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ChangeChannelByNumberRequestTag {
    MajorNumber = 0 , MinorNumber = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SkipChannelRequestTag {
    Count = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum GetProgramGuideRequestTag {
    StartTime = 0 , EndTime = 1 , ChannelList = 2 , PageToken = 3 , RecordingFlag = 4 , ExternalIdList = 5 , Data = 6 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProgramGuideResponseTag {
    Paging = 0 , ProgramList = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RecordProgramRequestTag {
    ProgramIdentifier = 0 , ShouldRecordSeries = 1 , ExternalIdList = 2 , Data = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CancelRecordProgramRequestTag {
    ProgramIdentifier = 0 , ShouldRecordSeries = 1 , ExternalIdList = 2 , Data = 3 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProgramCastStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProgramCastStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn name (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn role (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProgramCastStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProgramCastStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProgramCastStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProgramCastStruct" ) ? ;
        match self . name () {
            Ok (value ) => write ! (f , "{}: {:?}," , "name" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "name" , e . code () ) ? , 
        }
        match self . role () {
            Ok (value ) => write ! (f , "{}: {:?}," , "role" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "role" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProgramCastStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProgramCastStruct" ) ;
        match self . name () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "name" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "name" , e . code () ) , 
        }
        match self . role () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "role" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "role" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProgramCategoryStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProgramCategoryStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn category (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn sub_category (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProgramCategoryStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProgramCategoryStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProgramCategoryStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProgramCategoryStruct" ) ? ;
        match self . category () {
            Ok (value ) => write ! (f , "{}: {:?}," , "category" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "category" , e . code () ) ? , 
        }
        match self . sub_category () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "sub_category" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "sub_category" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sub_category" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProgramCategoryStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProgramCategoryStruct" ) ;
        match self . category () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "category" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "category" , e . code () ) , 
        }
        match self . sub_category () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "sub_category" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "sub_category" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sub_category" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SeriesInfoStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SeriesInfoStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn season (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn episode (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SeriesInfoStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SeriesInfoStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SeriesInfoStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SeriesInfoStruct" ) ? ;
        match self . season () {
            Ok (value ) => write ! (f , "{}: {:?}," , "season" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "season" , e . code () ) ? , 
        }
        match self . episode () {
            Ok (value ) => write ! (f , "{}: {:?}," , "episode" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "episode" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SeriesInfoStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SeriesInfoStruct" ) ;
        match self . season () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "season" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "season" , e . code () ) , 
        }
        match self . episode () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "episode" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "episode" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ChannelInfoStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ChannelInfoStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn major_number (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn minor_number (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn name (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn call_sign (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn affiliate_call_sign (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn identifier (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn r#type (& self ) -> Result < Option < ChannelTypeEnum > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ChannelInfoStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ChannelInfoStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ChannelInfoStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ChannelInfoStruct" ) ? ;
        match self . major_number () {
            Ok (value ) => write ! (f , "{}: {:?}," , "major_number" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "major_number" , e . code () ) ? , 
        }
        match self . minor_number () {
            Ok (value ) => write ! (f , "{}: {:?}," , "minor_number" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "minor_number" , e . code () ) ? , 
        }
        match self . name () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "name" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "name" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "name" , e . code () ) ? , 
        }
        match self . call_sign () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "call_sign" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "call_sign" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "call_sign" , e . code () ) ? , 
        }
        match self . affiliate_call_sign () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "affiliate_call_sign" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "affiliate_call_sign" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "affiliate_call_sign" , e . code () ) ? , 
        }
        match self . identifier () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "identifier" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "identifier" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "identifier" , e . code () ) ? , 
        }
        match self . r#type () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "type" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "type" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "type" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ChannelInfoStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ChannelInfoStruct" ) ;
        match self . major_number () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "major_number" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "major_number" , e . code () ) , 
        }
        match self . minor_number () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "minor_number" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "minor_number" , e . code () ) , 
        }
        match self . name () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "name" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "name" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "name" , e . code () ) , 
        }
        match self . call_sign () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "call_sign" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "call_sign" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "call_sign" , e . code () ) , 
        }
        match self . affiliate_call_sign () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "affiliate_call_sign" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "affiliate_call_sign" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "affiliate_call_sign" , e . code () ) , 
        }
        match self . identifier () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "identifier" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "identifier" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "identifier" , e . code () ) , 
        }
        match self . r#type () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "type" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "type" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "type" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProgramStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProgramStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn identifier (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn channel (& self ) -> Result < ChannelInfoStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn start_time (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn end_time (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn title (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn subtitle (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn description (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    pub fn audio_languages (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , crate :: tlv :: Utf8Str < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (7 ) 
    }
    pub fn ratings (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , crate :: tlv :: Utf8Str < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (8 ) 
    }
    pub fn thumbnail_url (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (9 ) 
    }
    pub fn poster_art_url (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (10 ) 
    }
    pub fn dvbi_url (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (11 ) 
    }
    pub fn release_date (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (12 ) 
    }
    pub fn parental_guidance_text (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (13 ) 
    }
    pub fn recording_flag (& self ) -> Result < Option < RecordingFlagBitmap > , crate :: error :: Error > {
        self . 0 . read_opt (14 ) 
    }
    pub fn series_info (& self ) -> Result < Option < crate :: tlv :: Nullable < SeriesInfoStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (15 ) 
    }
    pub fn category_list (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , ProgramCategoryStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (16 ) 
    }
    pub fn cast_list (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , ProgramCastStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (17 ) 
    }
    pub fn external_id_list (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , ProgramCastStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (18 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProgramStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProgramStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProgramStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProgramStruct" ) ? ;
        match self . identifier () {
            Ok (value ) => write ! (f , "{}: {:?}," , "identifier" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "identifier" , e . code () ) ? , 
        }
        match self . channel () {
            Ok (value ) => write ! (f , "{}: {:?}," , "channel" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel" , e . code () ) ? , 
        }
        match self . start_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "start_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "start_time" , e . code () ) ? , 
        }
        match self . end_time () {
            Ok (value ) => write ! (f , "{}: {:?}," , "end_time" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "end_time" , e . code () ) ? , 
        }
        match self . title () {
            Ok (value ) => write ! (f , "{}: {:?}," , "title" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "title" , e . code () ) ? , 
        }
        match self . subtitle () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "subtitle" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "subtitle" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "subtitle" , e . code () ) ? , 
        }
        match self . description () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "description" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "description" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "description" , e . code () ) ? , 
        }
        match self . audio_languages () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "audio_languages" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "audio_languages" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_languages" , e . code () ) ? , 
        }
        match self . ratings () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "ratings" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "ratings" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ratings" , e . code () ) ? , 
        }
        match self . thumbnail_url () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "thumbnail_url" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "thumbnail_url" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "thumbnail_url" , e . code () ) ? , 
        }
        match self . poster_art_url () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "poster_art_url" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "poster_art_url" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "poster_art_url" , e . code () ) ? , 
        }
        match self . dvbi_url () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "dvbi_url" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "dvbi_url" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "dvbi_url" , e . code () ) ? , 
        }
        match self . release_date () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "release_date" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "release_date" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "release_date" , e . code () ) ? , 
        }
        match self . parental_guidance_text () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "parental_guidance_text" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "parental_guidance_text" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "parental_guidance_text" , e . code () ) ? , 
        }
        match self . recording_flag () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "recording_flag" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "recording_flag" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "recording_flag" , e . code () ) ? , 
        }
        match self . series_info () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "series_info" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "series_info" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "series_info" , e . code () ) ? , 
        }
        match self . category_list () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "category_list" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "category_list" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "category_list" , e . code () ) ? , 
        }
        match self . cast_list () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "cast_list" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "cast_list" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cast_list" , e . code () ) ? , 
        }
        match self . external_id_list () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "external_id_list" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "external_id_list" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProgramStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProgramStruct" ) ;
        match self . identifier () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "identifier" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "identifier" , e . code () ) , 
        }
        match self . channel () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "channel" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel" , e . code () ) , 
        }
        match self . start_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "start_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "start_time" , e . code () ) , 
        }
        match self . end_time () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "end_time" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "end_time" , e . code () ) , 
        }
        match self . title () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "title" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "title" , e . code () ) , 
        }
        match self . subtitle () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "subtitle" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "subtitle" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "subtitle" , e . code () ) , 
        }
        match self . description () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "description" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "description" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "description" , e . code () ) , 
        }
        match self . audio_languages () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "audio_languages" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "audio_languages" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_languages" , e . code () ) , 
        }
        match self . ratings () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "ratings" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "ratings" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ratings" , e . code () ) , 
        }
        match self . thumbnail_url () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "thumbnail_url" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "thumbnail_url" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "thumbnail_url" , e . code () ) , 
        }
        match self . poster_art_url () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "poster_art_url" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "poster_art_url" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "poster_art_url" , e . code () ) , 
        }
        match self . dvbi_url () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "dvbi_url" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "dvbi_url" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "dvbi_url" , e . code () ) , 
        }
        match self . release_date () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "release_date" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "release_date" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "release_date" , e . code () ) , 
        }
        match self . parental_guidance_text () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "parental_guidance_text" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "parental_guidance_text" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "parental_guidance_text" , e . code () ) , 
        }
        match self . recording_flag () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "recording_flag" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "recording_flag" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "recording_flag" , e . code () ) , 
        }
        match self . series_info () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "series_info" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "series_info" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "series_info" , e . code () ) , 
        }
        match self . category_list () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "category_list" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "category_list" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "category_list" , e . code () ) , 
        }
        match self . cast_list () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "cast_list" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "cast_list" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cast_list" , e . code () ) , 
        }
        match self . external_id_list () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "external_id_list" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "external_id_list" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct PageTokenStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > PageTokenStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn limit (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn after (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn before (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for PageTokenStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for PageTokenStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for PageTokenStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "PageTokenStruct" ) ? ;
        match self . limit () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "limit" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "limit" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "limit" , e . code () ) ? , 
        }
        match self . after () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "after" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "after" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "after" , e . code () ) ? , 
        }
        match self . before () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "before" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "before" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "before" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for PageTokenStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "PageTokenStruct" ) ;
        match self . limit () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "limit" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "limit" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "limit" , e . code () ) , 
        }
        match self . after () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "after" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "after" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "after" , e . code () ) , 
        }
        match self . before () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "before" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "before" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "before" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ChannelPagingStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ChannelPagingStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_token (& self ) -> Result < Option < crate :: tlv :: Nullable < PageTokenStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn next_token (& self ) -> Result < Option < crate :: tlv :: Nullable < PageTokenStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ChannelPagingStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ChannelPagingStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ChannelPagingStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ChannelPagingStruct" ) ? ;
        match self . previous_token () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "previous_token" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "previous_token" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_token" , e . code () ) ? , 
        }
        match self . next_token () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "next_token" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "next_token" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "next_token" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ChannelPagingStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ChannelPagingStruct" ) ;
        match self . previous_token () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "previous_token" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "previous_token" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_token" , e . code () ) , 
        }
        match self . next_token () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "next_token" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "next_token" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "next_token" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AdditionalInfoStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AdditionalInfoStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn name (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn value (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AdditionalInfoStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AdditionalInfoStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AdditionalInfoStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AdditionalInfoStruct" ) ? ;
        match self . name () {
            Ok (value ) => write ! (f , "{}: {:?}," , "name" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "name" , e . code () ) ? , 
        }
        match self . value () {
            Ok (value ) => write ! (f , "{}: {:?}," , "value" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "value" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AdditionalInfoStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AdditionalInfoStruct" ) ;
        match self . name () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "name" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "name" , e . code () ) , 
        }
        match self . value () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "value" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "value" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct LineupInfoStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > LineupInfoStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn operator_name (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn lineup_name (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn postal_code (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn lineup_info_type (& self ) -> Result < LineupInfoTypeEnum , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for LineupInfoStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for LineupInfoStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for LineupInfoStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "LineupInfoStruct" ) ? ;
        match self . operator_name () {
            Ok (value ) => write ! (f , "{}: {:?}," , "operator_name" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "operator_name" , e . code () ) ? , 
        }
        match self . lineup_name () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "lineup_name" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "lineup_name" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "lineup_name" , e . code () ) ? , 
        }
        match self . postal_code () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "postal_code" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "postal_code" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "postal_code" , e . code () ) ? , 
        }
        match self . lineup_info_type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "lineup_info_type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "lineup_info_type" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for LineupInfoStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "LineupInfoStruct" ) ;
        match self . operator_name () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "operator_name" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "operator_name" , e . code () ) , 
        }
        match self . lineup_name () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "lineup_name" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "lineup_name" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "lineup_name" , e . code () ) , 
        }
        match self . postal_code () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "postal_code" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "postal_code" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "postal_code" , e . code () ) , 
        }
        match self . lineup_info_type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "lineup_info_type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "lineup_info_type" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ChangeChannelRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ChangeChannelRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn r#match (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ChangeChannelRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ChangeChannelRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ChangeChannelRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ChangeChannelRequest" ) ? ;
        match self . r#match () {
            Ok (value ) => write ! (f , "{}: {:?}," , "match" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "match" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ChangeChannelRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ChangeChannelRequest" ) ;
        match self . r#match () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "match" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "match" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ChangeChannelResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ChangeChannelResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn status (& self ) -> Result < StatusEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn data (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ChangeChannelResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ChangeChannelResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ChangeChannelResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ChangeChannelResponse" ) ? ;
        match self . status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "status" , e . code () ) ? , 
        }
        match self . data () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "data" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "data" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "data" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ChangeChannelResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ChangeChannelResponse" ) ;
        match self . status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "status" , e . code () ) , 
        }
        match self . data () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "data" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "data" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "data" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ChangeChannelByNumberRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ChangeChannelByNumberRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn major_number (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn minor_number (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ChangeChannelByNumberRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ChangeChannelByNumberRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ChangeChannelByNumberRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ChangeChannelByNumberRequest" ) ? ;
        match self . major_number () {
            Ok (value ) => write ! (f , "{}: {:?}," , "major_number" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "major_number" , e . code () ) ? , 
        }
        match self . minor_number () {
            Ok (value ) => write ! (f , "{}: {:?}," , "minor_number" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "minor_number" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ChangeChannelByNumberRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ChangeChannelByNumberRequest" ) ;
        match self . major_number () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "major_number" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "major_number" , e . code () ) , 
        }
        match self . minor_number () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "minor_number" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "minor_number" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SkipChannelRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SkipChannelRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn count (& self ) -> Result < i16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SkipChannelRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SkipChannelRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SkipChannelRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SkipChannelRequest" ) ? ;
        match self . count () {
            Ok (value ) => write ! (f , "{}: {:?}," , "count" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "count" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SkipChannelRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SkipChannelRequest" ) ;
        match self . count () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "count" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "count" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct GetProgramGuideRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > GetProgramGuideRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn start_time (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn end_time (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn channel_list (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , ChannelInfoStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn page_token (& self ) -> Result < Option < PageTokenStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn recording_flag (& self ) -> Result < Option < RecordingFlagBitmap > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn external_id_list (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , AdditionalInfoStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn data (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for GetProgramGuideRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for GetProgramGuideRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for GetProgramGuideRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "GetProgramGuideRequest" ) ? ;
        match self . start_time () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "start_time" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "start_time" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "start_time" , e . code () ) ? , 
        }
        match self . end_time () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "end_time" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "end_time" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "end_time" , e . code () ) ? , 
        }
        match self . channel_list () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "channel_list" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "channel_list" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel_list" , e . code () ) ? , 
        }
        match self . page_token () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "page_token" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "page_token" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "page_token" , e . code () ) ? , 
        }
        match self . recording_flag () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "recording_flag" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "recording_flag" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "recording_flag" , e . code () ) ? , 
        }
        match self . external_id_list () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "external_id_list" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "external_id_list" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) ? , 
        }
        match self . data () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "data" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "data" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "data" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for GetProgramGuideRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "GetProgramGuideRequest" ) ;
        match self . start_time () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "start_time" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "start_time" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "start_time" , e . code () ) , 
        }
        match self . end_time () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "end_time" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "end_time" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "end_time" , e . code () ) , 
        }
        match self . channel_list () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "channel_list" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "channel_list" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel_list" , e . code () ) , 
        }
        match self . page_token () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "page_token" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "page_token" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "page_token" , e . code () ) , 
        }
        match self . recording_flag () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "recording_flag" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "recording_flag" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "recording_flag" , e . code () ) , 
        }
        match self . external_id_list () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "external_id_list" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "external_id_list" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) , 
        }
        match self . data () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "data" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "data" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "data" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProgramGuideResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProgramGuideResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn paging (& self ) -> Result < ChannelPagingStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn program_list (& self ) -> Result < crate :: tlv :: TLVArray < '_ , ProgramStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProgramGuideResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProgramGuideResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProgramGuideResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProgramGuideResponse" ) ? ;
        match self . paging () {
            Ok (value ) => write ! (f , "{}: {:?}," , "paging" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "paging" , e . code () ) ? , 
        }
        match self . program_list () {
            Ok (value ) => write ! (f , "{}: {:?}," , "program_list" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "program_list" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProgramGuideResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProgramGuideResponse" ) ;
        match self . paging () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "paging" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "paging" , e . code () ) , 
        }
        match self . program_list () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "program_list" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "program_list" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RecordProgramRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RecordProgramRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn program_identifier (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn should_record_series (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn external_id_list (& self ) -> Result < crate :: tlv :: TLVArray < '_ , AdditionalInfoStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn data (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RecordProgramRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RecordProgramRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RecordProgramRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RecordProgramRequest" ) ? ;
        match self . program_identifier () {
            Ok (value ) => write ! (f , "{}: {:?}," , "program_identifier" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "program_identifier" , e . code () ) ? , 
        }
        match self . should_record_series () {
            Ok (value ) => write ! (f , "{}: {:?}," , "should_record_series" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "should_record_series" , e . code () ) ? , 
        }
        match self . external_id_list () {
            Ok (value ) => write ! (f , "{}: {:?}," , "external_id_list" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) ? , 
        }
        match self . data () {
            Ok (value ) => write ! (f , "{}: {:?}," , "data" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "data" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RecordProgramRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RecordProgramRequest" ) ;
        match self . program_identifier () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "program_identifier" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "program_identifier" , e . code () ) , 
        }
        match self . should_record_series () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "should_record_series" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "should_record_series" , e . code () ) , 
        }
        match self . external_id_list () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "external_id_list" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) , 
        }
        match self . data () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "data" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "data" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CancelRecordProgramRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CancelRecordProgramRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn program_identifier (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn should_record_series (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn external_id_list (& self ) -> Result < crate :: tlv :: TLVArray < '_ , AdditionalInfoStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn data (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CancelRecordProgramRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CancelRecordProgramRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CancelRecordProgramRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CancelRecordProgramRequest" ) ? ;
        match self . program_identifier () {
            Ok (value ) => write ! (f , "{}: {:?}," , "program_identifier" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "program_identifier" , e . code () ) ? , 
        }
        match self . should_record_series () {
            Ok (value ) => write ! (f , "{}: {:?}," , "should_record_series" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "should_record_series" , e . code () ) ? , 
        }
        match self . external_id_list () {
            Ok (value ) => write ! (f , "{}: {:?}," , "external_id_list" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) ? , 
        }
        match self . data () {
            Ok (value ) => write ! (f , "{}: {:?}," , "data" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "data" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CancelRecordProgramRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CancelRecordProgramRequest" ) ;
        match self . program_identifier () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "program_identifier" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "program_identifier" , e . code () ) , 
        }
        match self . should_record_series () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "should_record_series" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "should_record_series" , e . code () ) , 
        }
        match self . external_id_list () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "external_id_list" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "external_id_list" , e . code () ) , 
        }
        match self . data () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "data" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "data" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct ProgramCastStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProgramCastStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProgramCastStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn name (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ProgramCastStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "name" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ProgramCastStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramCastStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn role (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ProgramCastStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "role" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ProgramCastStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramCastStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProgramCastStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramCastStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProgramCastStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramCastStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProgramCastStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramCastStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProgramCastStructArrayBuilder < P > (P ) ;
impl < P > ProgramCastStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProgramCastStructBuilder < ProgramCastStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramCastStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProgramCastStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramCastStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProgramCastStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramCastStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProgramCastStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramCastStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProgramCategoryStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProgramCategoryStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProgramCategoryStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn category (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ProgramCategoryStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "category" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ProgramCategoryStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramCategoryStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sub_category (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramCategoryStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "subCategory" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ProgramCategoryStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramCategoryStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProgramCategoryStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramCategoryStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProgramCategoryStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramCategoryStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProgramCategoryStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramCategoryStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProgramCategoryStructArrayBuilder < P > (P ) ;
impl < P > ProgramCategoryStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProgramCategoryStructBuilder < ProgramCategoryStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramCategoryStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProgramCategoryStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramCategoryStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProgramCategoryStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramCategoryStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProgramCategoryStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramCategoryStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SeriesInfoStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SeriesInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SeriesInfoStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn season (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < SeriesInfoStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "season" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SeriesInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SeriesInfoStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn episode (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < SeriesInfoStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "episode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SeriesInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SeriesInfoStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SeriesInfoStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SeriesInfoStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SeriesInfoStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SeriesInfoStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SeriesInfoStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SeriesInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SeriesInfoStructArrayBuilder < P > (P ) ;
impl < P > SeriesInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SeriesInfoStructBuilder < SeriesInfoStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SeriesInfoStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SeriesInfoStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SeriesInfoStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SeriesInfoStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SeriesInfoStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SeriesInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SeriesInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChannelInfoStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ChannelInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn major_number (mut self , value : u16 ) -> Result < ChannelInfoStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "majorNumber" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ChannelInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn minor_number (mut self , value : u16 ) -> Result < ChannelInfoStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minorNumber" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ChannelInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn name (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ChannelInfoStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "name" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ChannelInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn call_sign (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ChannelInfoStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "callSign" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ChannelInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn affiliate_call_sign (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ChannelInfoStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "affiliateCallSign" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (ChannelInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn identifier (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ChannelInfoStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "identifier" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (ChannelInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn r#type (mut self , value : Option < ChannelTypeEnum > ) -> Result < ChannelInfoStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "type" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (ChannelInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChannelInfoStructBuilder < P , 7usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ChannelInfoStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChannelInfoStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ChannelInfoStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChannelInfoStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ChannelInfoStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChannelInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChannelInfoStructArrayBuilder < P > (P ) ;
impl < P > ChannelInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ChannelInfoStructBuilder < ChannelInfoStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ChannelInfoStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ChannelInfoStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChannelInfoStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ChannelInfoStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChannelInfoStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ChannelInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChannelInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProgramStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProgramStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn identifier (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ProgramStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "identifier" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel (self ) -> Result < ChannelInfoStructBuilder < ProgramStructBuilder < P , 2usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn start_time (mut self , value : u32 ) -> Result < ProgramStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "startTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn end_time (mut self , value : u32 ) -> Result < ProgramStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "endTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn title (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ProgramStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "title" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn subtitle (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "subtitle" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn description (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "description" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_languages (self ) -> Result < crate :: tlv :: OptionalBuilder < ProgramStructBuilder < P , 8usize > , crate :: tlv :: Utf8StrArrayBuilder < ProgramStructBuilder < P , 8usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (7 ) , ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ratings (self ) -> Result < crate :: tlv :: OptionalBuilder < ProgramStructBuilder < P , 9usize > , crate :: tlv :: Utf8StrArrayBuilder < ProgramStructBuilder < P , 9usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (8 ) , ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn thumbnail_url (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramStructBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "thumbnailUrl" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn poster_art_url (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramStructBuilder < P , 11usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "posterArtUrl" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 10 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 11 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn dvbi_url (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramStructBuilder < P , 12usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "dvbiUrl" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 11 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 12 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn release_date (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramStructBuilder < P , 13usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "releaseDate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 12 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 13 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn parental_guidance_text (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProgramStructBuilder < P , 14usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "parentalGuidanceText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 13 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 14 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn recording_flag (mut self , value : Option < RecordingFlagBitmap > ) -> Result < ProgramStructBuilder < P , 15usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "recordingFlag" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 14 , & value ) ? ;
        Ok (ProgramStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 15 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn series_info (self ) -> Result < crate :: tlv :: OptionalBuilder < ProgramStructBuilder < P , 16usize > , crate :: tlv :: NullableBuilder < ProgramStructBuilder < P , 16usize > , SeriesInfoStructBuilder < ProgramStructBuilder < P , 16usize > > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (15 ) , ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 16 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn category_list (self ) -> Result < crate :: tlv :: OptionalBuilder < ProgramStructBuilder < P , 17usize > , ProgramCategoryStructArrayBuilder < ProgramStructBuilder < P , 17usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (16 ) , ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 17 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cast_list (self ) -> Result < crate :: tlv :: OptionalBuilder < ProgramStructBuilder < P , 18usize > , ProgramCastStructArrayBuilder < ProgramStructBuilder < P , 18usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (17 ) , ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 18 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn external_id_list (self ) -> Result < crate :: tlv :: OptionalBuilder < ProgramStructBuilder < P , 19usize > , ProgramCastStructArrayBuilder < ProgramStructBuilder < P , 19usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (18 ) , ) 
    }
    
}
impl < P > ProgramStructBuilder < P , 19usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProgramStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProgramStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProgramStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProgramStructArrayBuilder < P > (P ) ;
impl < P > ProgramStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProgramStructBuilder < ProgramStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProgramStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProgramStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProgramStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PageTokenStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > PageTokenStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > PageTokenStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn limit (mut self , value : Option < u16 > ) -> Result < PageTokenStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "limit" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (PageTokenStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PageTokenStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn after (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < PageTokenStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "after" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (PageTokenStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PageTokenStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn before (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < PageTokenStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "before" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (PageTokenStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > PageTokenStructBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for PageTokenStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PageTokenStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for PageTokenStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PageTokenStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for PageTokenStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PageTokenStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PageTokenStructArrayBuilder < P > (P ) ;
impl < P > PageTokenStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < PageTokenStructBuilder < PageTokenStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (PageTokenStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for PageTokenStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PageTokenStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for PageTokenStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PageTokenStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for PageTokenStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PageTokenStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChannelPagingStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ChannelPagingStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ChannelPagingStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_token (self ) -> Result < crate :: tlv :: OptionalBuilder < ChannelPagingStructBuilder < P , 1usize > , crate :: tlv :: NullableBuilder < ChannelPagingStructBuilder < P , 1usize > , PageTokenStructBuilder < ChannelPagingStructBuilder < P , 1usize > > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ChannelPagingStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > ChannelPagingStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn next_token (self ) -> Result < crate :: tlv :: OptionalBuilder < ChannelPagingStructBuilder < P , 2usize > , crate :: tlv :: NullableBuilder < ChannelPagingStructBuilder < P , 2usize > , PageTokenStructBuilder < ChannelPagingStructBuilder < P , 2usize > > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ChannelPagingStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > ChannelPagingStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ChannelPagingStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChannelPagingStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ChannelPagingStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChannelPagingStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ChannelPagingStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChannelPagingStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChannelPagingStructArrayBuilder < P > (P ) ;
impl < P > ChannelPagingStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ChannelPagingStructBuilder < ChannelPagingStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ChannelPagingStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ChannelPagingStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChannelPagingStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ChannelPagingStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChannelPagingStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ChannelPagingStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChannelPagingStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AdditionalInfoStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AdditionalInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AdditionalInfoStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn name (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < AdditionalInfoStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "name" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AdditionalInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AdditionalInfoStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn value (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < AdditionalInfoStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "value" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AdditionalInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AdditionalInfoStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AdditionalInfoStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AdditionalInfoStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AdditionalInfoStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AdditionalInfoStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AdditionalInfoStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AdditionalInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AdditionalInfoStructArrayBuilder < P > (P ) ;
impl < P > AdditionalInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AdditionalInfoStructBuilder < AdditionalInfoStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AdditionalInfoStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AdditionalInfoStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AdditionalInfoStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AdditionalInfoStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AdditionalInfoStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AdditionalInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AdditionalInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LineupInfoStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > LineupInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > LineupInfoStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn operator_name (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < LineupInfoStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "operatorName" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (LineupInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > LineupInfoStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn lineup_name (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < LineupInfoStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "lineupName" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (LineupInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > LineupInfoStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn postal_code (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < LineupInfoStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "postalCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (LineupInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > LineupInfoStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn lineup_info_type (mut self , value : LineupInfoTypeEnum ) -> Result < LineupInfoStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "lineupInfoType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (LineupInfoStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > LineupInfoStructBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for LineupInfoStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LineupInfoStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for LineupInfoStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LineupInfoStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for LineupInfoStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LineupInfoStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct LineupInfoStructArrayBuilder < P > (P ) ;
impl < P > LineupInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < LineupInfoStructBuilder < LineupInfoStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (LineupInfoStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for LineupInfoStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "LineupInfoStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for LineupInfoStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "LineupInfoStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for LineupInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for LineupInfoStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChangeChannelRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ChangeChannelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ChangeChannelRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn r#match (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ChangeChannelRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "match" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ChangeChannelRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChangeChannelRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ChangeChannelRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ChangeChannelRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ChangeChannelRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChangeChannelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChangeChannelRequestArrayBuilder < P > (P ) ;
impl < P > ChangeChannelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ChangeChannelRequestBuilder < ChangeChannelRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ChangeChannelRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ChangeChannelRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ChangeChannelRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ChangeChannelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChangeChannelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChangeChannelResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ChangeChannelResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ChangeChannelResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn status (mut self , value : StatusEnum ) -> Result < ChangeChannelResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "status" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ChangeChannelResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChangeChannelResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn data (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ChangeChannelResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "data" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ChangeChannelResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChangeChannelResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ChangeChannelResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ChangeChannelResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ChangeChannelResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChangeChannelResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChangeChannelResponseArrayBuilder < P > (P ) ;
impl < P > ChangeChannelResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ChangeChannelResponseBuilder < ChangeChannelResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ChangeChannelResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ChangeChannelResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ChangeChannelResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ChangeChannelResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChangeChannelResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChangeChannelByNumberRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ChangeChannelByNumberRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ChangeChannelByNumberRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn major_number (mut self , value : u16 ) -> Result < ChangeChannelByNumberRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "majorNumber" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ChangeChannelByNumberRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChangeChannelByNumberRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn minor_number (mut self , value : u16 ) -> Result < ChangeChannelByNumberRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minorNumber" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ChangeChannelByNumberRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ChangeChannelByNumberRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ChangeChannelByNumberRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelByNumberRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ChangeChannelByNumberRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelByNumberRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ChangeChannelByNumberRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChangeChannelByNumberRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ChangeChannelByNumberRequestArrayBuilder < P > (P ) ;
impl < P > ChangeChannelByNumberRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ChangeChannelByNumberRequestBuilder < ChangeChannelByNumberRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ChangeChannelByNumberRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ChangeChannelByNumberRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelByNumberRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ChangeChannelByNumberRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ChangeChannelByNumberRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ChangeChannelByNumberRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ChangeChannelByNumberRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SkipChannelRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SkipChannelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SkipChannelRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn count (mut self , value : i16 ) -> Result < SkipChannelRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "count" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SkipChannelRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SkipChannelRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SkipChannelRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SkipChannelRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SkipChannelRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SkipChannelRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SkipChannelRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SkipChannelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SkipChannelRequestArrayBuilder < P > (P ) ;
impl < P > SkipChannelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SkipChannelRequestBuilder < SkipChannelRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SkipChannelRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SkipChannelRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SkipChannelRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SkipChannelRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SkipChannelRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SkipChannelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SkipChannelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GetProgramGuideRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > GetProgramGuideRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn start_time (mut self , value : Option < u32 > ) -> Result < GetProgramGuideRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "startTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (GetProgramGuideRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn end_time (mut self , value : Option < u32 > ) -> Result < GetProgramGuideRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "endTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (GetProgramGuideRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel_list (self ) -> Result < crate :: tlv :: OptionalBuilder < GetProgramGuideRequestBuilder < P , 3usize > , ChannelInfoStructArrayBuilder < GetProgramGuideRequestBuilder < P , 3usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GetProgramGuideRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn page_token (self ) -> Result < crate :: tlv :: OptionalBuilder < GetProgramGuideRequestBuilder < P , 4usize > , PageTokenStructBuilder < GetProgramGuideRequestBuilder < P , 4usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GetProgramGuideRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn recording_flag (mut self , value : Option < RecordingFlagBitmap > ) -> Result < GetProgramGuideRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "recordingFlag" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (GetProgramGuideRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn external_id_list (self ) -> Result < crate :: tlv :: OptionalBuilder < GetProgramGuideRequestBuilder < P , 6usize > , AdditionalInfoStructArrayBuilder < GetProgramGuideRequestBuilder < P , 6usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GetProgramGuideRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (5 ) , ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn data (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < GetProgramGuideRequestBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "data" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (GetProgramGuideRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > GetProgramGuideRequestBuilder < P , 7usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for GetProgramGuideRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GetProgramGuideRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for GetProgramGuideRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GetProgramGuideRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for GetProgramGuideRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GetProgramGuideRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct GetProgramGuideRequestArrayBuilder < P > (P ) ;
impl < P > GetProgramGuideRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < GetProgramGuideRequestBuilder < GetProgramGuideRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (GetProgramGuideRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for GetProgramGuideRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "GetProgramGuideRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for GetProgramGuideRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "GetProgramGuideRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for GetProgramGuideRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for GetProgramGuideRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProgramGuideResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProgramGuideResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProgramGuideResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn paging (self ) -> Result < ChannelPagingStructBuilder < ProgramGuideResponseBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramGuideResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > ProgramGuideResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn program_list (self ) -> Result < ProgramStructArrayBuilder < ProgramGuideResponseBuilder < P , 2usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramGuideResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > ProgramGuideResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProgramGuideResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramGuideResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProgramGuideResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramGuideResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProgramGuideResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramGuideResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProgramGuideResponseArrayBuilder < P > (P ) ;
impl < P > ProgramGuideResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProgramGuideResponseBuilder < ProgramGuideResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProgramGuideResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProgramGuideResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProgramGuideResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProgramGuideResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProgramGuideResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProgramGuideResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProgramGuideResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RecordProgramRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RecordProgramRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RecordProgramRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn program_identifier (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < RecordProgramRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "programIdentifier" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RecordProgramRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > RecordProgramRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn should_record_series (mut self , value : bool ) -> Result < RecordProgramRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "shouldRecordSeries" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (RecordProgramRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > RecordProgramRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn external_id_list (self ) -> Result < AdditionalInfoStructArrayBuilder < RecordProgramRequestBuilder < P , 3usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RecordProgramRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > RecordProgramRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn data (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < RecordProgramRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "data" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (RecordProgramRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > RecordProgramRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RecordProgramRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RecordProgramRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RecordProgramRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RecordProgramRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RecordProgramRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RecordProgramRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RecordProgramRequestArrayBuilder < P > (P ) ;
impl < P > RecordProgramRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RecordProgramRequestBuilder < RecordProgramRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RecordProgramRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RecordProgramRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RecordProgramRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RecordProgramRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RecordProgramRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RecordProgramRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RecordProgramRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CancelRecordProgramRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CancelRecordProgramRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CancelRecordProgramRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn program_identifier (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < CancelRecordProgramRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "programIdentifier" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CancelRecordProgramRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > CancelRecordProgramRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn should_record_series (mut self , value : bool ) -> Result < CancelRecordProgramRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "shouldRecordSeries" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (CancelRecordProgramRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > CancelRecordProgramRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn external_id_list (self ) -> Result < AdditionalInfoStructArrayBuilder < CancelRecordProgramRequestBuilder < P , 3usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CancelRecordProgramRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > CancelRecordProgramRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn data (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < CancelRecordProgramRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "data" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (CancelRecordProgramRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > CancelRecordProgramRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CancelRecordProgramRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CancelRecordProgramRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CancelRecordProgramRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CancelRecordProgramRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CancelRecordProgramRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CancelRecordProgramRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CancelRecordProgramRequestArrayBuilder < P > (P ) ;
impl < P > CancelRecordProgramRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CancelRecordProgramRequestBuilder < CancelRecordProgramRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CancelRecordProgramRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CancelRecordProgramRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CancelRecordProgramRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CancelRecordProgramRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CancelRecordProgramRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CancelRecordProgramRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CancelRecordProgramRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    ChannelList = 0 , Lineup = 1 , CurrentChannel = 2 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: ChannelList => write ! (f , "{}(0x{:02x})" , "ChannelList" , AttributeId :: ChannelList as u32 ) ? , AttributeId :: Lineup => write ! (f , "{}(0x{:02x})" , "Lineup" , AttributeId :: Lineup as u32 ) ? , AttributeId :: CurrentChannel => write ! (f , "{}(0x{:02x})" , "CurrentChannel" , AttributeId :: CurrentChannel as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: ChannelList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ChannelList" , AttributeId :: ChannelList as u32 ) , AttributeId :: Lineup => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Lineup" , AttributeId :: Lineup as u32 ) , AttributeId :: CurrentChannel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentChannel" , AttributeId :: CurrentChannel as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    ChangeChannel = 0 , ChangeChannelByNumber = 2 , SkipChannel = 3 , GetProgramGuide = 4 , RecordProgram = 6 , CancelRecordProgram = 7 
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
            CommandId :: ChangeChannel => write ! (f , "{}(0x{:02x})" , "ChangeChannel" , CommandId :: ChangeChannel as u32 ) ? , CommandId :: ChangeChannelByNumber => write ! (f , "{}(0x{:02x})" , "ChangeChannelByNumber" , CommandId :: ChangeChannelByNumber as u32 ) ? , CommandId :: SkipChannel => write ! (f , "{}(0x{:02x})" , "SkipChannel" , CommandId :: SkipChannel as u32 ) ? , CommandId :: GetProgramGuide => write ! (f , "{}(0x{:02x})" , "GetProgramGuide" , CommandId :: GetProgramGuide as u32 ) ? , CommandId :: RecordProgram => write ! (f , "{}(0x{:02x})" , "RecordProgram" , CommandId :: RecordProgram as u32 ) ? , CommandId :: CancelRecordProgram => write ! (f , "{}(0x{:02x})" , "CancelRecordProgram" , CommandId :: CancelRecordProgram as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: ChangeChannel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ChangeChannel" , CommandId :: ChangeChannel as u32 ) , CommandId :: ChangeChannelByNumber => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ChangeChannelByNumber" , CommandId :: ChangeChannelByNumber as u32 ) , CommandId :: SkipChannel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SkipChannel" , CommandId :: SkipChannel as u32 ) , CommandId :: GetProgramGuide => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GetProgramGuide" , CommandId :: GetProgramGuide as u32 ) , CommandId :: RecordProgram => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RecordProgram" , CommandId :: RecordProgram as u32 ) , CommandId :: CancelRecordProgram => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CancelRecordProgram" , CommandId :: CancelRecordProgram as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    ChangeChannelResponse = 1 , ProgramGuideResponse = 5 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: ChangeChannelResponse => write ! (f , "{}(0x{:02x})" , "ChangeChannelResponse" , CommandResponseId :: ChangeChannelResponse as u32 ) ? , CommandResponseId :: ProgramGuideResponse => write ! (f , "{}(0x{:02x})" , "ProgramGuideResponse" , CommandResponseId :: ProgramGuideResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: ChangeChannelResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ChangeChannelResponse" , CommandResponseId :: ChangeChannelResponse as u32 ) , CommandResponseId :: ProgramGuideResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ProgramGuideResponse" , CommandResponseId :: ProgramGuideResponse as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (1284 , 2 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: ChannelList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: Lineup as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentChannel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: ChangeChannel as _ , Some (CommandResponseId :: ChangeChannelResponse as _ ) , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: ChangeChannelByNumber as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: SkipChannel as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: GetProgramGuide as _ , Some (CommandResponseId :: ProgramGuideResponse as _ ) , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: RecordProgram as _ , None , crate :: dm :: Access :: WO , ) , crate :: dm :: Command :: new (CommandId :: CancelRecordProgram as _ , None , crate :: dm :: Access :: WO , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn channel_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ChannelInfoStructArrayBuilder < P > , ChannelInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn lineup < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , LineupInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn current_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ChannelInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_change_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelRequest < '_ > , response : ChangeChannelResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_change_channel_by_number (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelByNumberRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_skip_channel (& self , ctx : impl crate :: dm :: InvokeContext , request : SkipChannelRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_get_program_guide < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetProgramGuideRequest < '_ > , response : ProgramGuideResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : RecordProgramRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_cancel_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : CancelRecordProgramRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn channel_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ChannelInfoStructArrayBuilder < P > , ChannelInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: channel_list (self , ctx , builder ) 
    }
    fn lineup < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , LineupInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: lineup (self , ctx , builder ) 
    }
    fn current_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ChannelInfoStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: current_channel (self , ctx , builder ) 
    }
    fn handle_change_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelRequest < '_ > , response : ChangeChannelResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_change_channel (self , ctx , request , response ) 
    }
    fn handle_change_channel_by_number (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelByNumberRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_change_channel_by_number (self , ctx , request ) 
    }
    fn handle_skip_channel (& self , ctx : impl crate :: dm :: InvokeContext , request : SkipChannelRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_skip_channel (self , ctx , request ) 
    }
    fn handle_get_program_guide < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetProgramGuideRequest < '_ > , response : ProgramGuideResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_get_program_guide (self , ctx , request , response ) 
    }
    fn handle_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : RecordProgramRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_record_program (self , ctx , request ) 
    }
    fn handle_cancel_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : CancelRecordProgramRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_cancel_record_program (self , ctx , request ) 
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
                    AttributeId :: ChannelList => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelList , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. channel_list (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelList , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelList , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Lineup => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Lineup , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. lineup (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Lineup , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Lineup , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentChannel => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentChannel , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_channel (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentChannel , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentChannel , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: ChangeChannel => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannel ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_change_channel (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannel ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannel ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ChangeChannelByNumber => {
                let cmd_data : ChangeChannelByNumberRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_change_channel_by_number (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannelByNumber ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SkipChannel => {
                let cmd_data : SkipChannelRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_skip_channel (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SkipChannel ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: GetProgramGuide => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetProgramGuide ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_get_program_guide (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetProgramGuide ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetProgramGuide ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RecordProgram => {
                let cmd_data : RecordProgramRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_record_program (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RecordProgram ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: CancelRecordProgram => {
                let cmd_data : CancelRecordProgramRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_cancel_record_program (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CancelRecordProgram ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 1284u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Channel" , 1284u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Channel" , 1284u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `Channel` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct ChannelCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > ChannelCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn change_channel (self , endpoint : crate :: dm :: EndptId , ) -> Result < ChangeChannelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1284 , 0 ) ? . data_builder () 
    }
    pub fn change_channel_by_number (self , endpoint : crate :: dm :: EndptId , ) -> Result < ChangeChannelByNumberRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1284 , 2 ) ? . data_builder () 
    }
    pub fn skip_channel (self , endpoint : crate :: dm :: EndptId , ) -> Result < SkipChannelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1284 , 3 ) ? . data_builder () 
    }
    pub fn get_program_guide (self , endpoint : crate :: dm :: EndptId , ) -> Result < GetProgramGuideRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1284 , 4 ) ? . data_builder () 
    }
    pub fn record_program (self , endpoint : crate :: dm :: EndptId , ) -> Result < RecordProgramRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1284 , 6 ) ? . data_builder () 
    }
    pub fn cancel_record_program (self , endpoint : crate :: dm :: EndptId , ) -> Result < CancelRecordProgramRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1284 , 7 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Channel` cluster's commands. `use` this trait to call `.channel_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`ChannelCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ChannelCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn channel_inv (self ) -> ChannelCmdRequestsView < P > ;
    
}
impl < P > ChannelCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn channel_inv (self ) -> ChannelCmdRequestsView < P > {
        ChannelCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `Channel` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ChannelAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > ChannelAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn lineup (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_channel (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1284 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Channel` cluster's attribute reads. `use` this trait to call `.channel_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`ChannelAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ChannelAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn channel_read (self ) -> ChannelAttrReadsView < P > ;
    
}
impl < P > ChannelAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn channel_read (self ) -> ChannelAttrReadsView < P > {
        ChannelAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `Channel` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ChannelAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > ChannelAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `Channel` cluster's attribute writes. `use` this trait to call `.channel_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`ChannelAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait ChannelAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn channel_write (self ) -> ChannelAttrWritesView < P > ;
    
}
impl < P > ChannelAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn channel_write (self ) -> ChannelAttrWritesView < P > {
        ChannelAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `Channel` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct ChannelCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > ChannelCmdResponsesView < 'a , 'r > {
    pub fn change_channel (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ChangeChannelResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < ChangeChannelResponse < 'a >> (1284 , 0 ) 
    }
    pub fn change_channel_by_number (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1284 , 2 ) 
    }
    pub fn skip_channel (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1284 , 3 ) 
    }
    pub fn get_program_guide (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ProgramGuideResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < ProgramGuideResponse < 'a >> (1284 , 4 ) 
    }
    pub fn record_program (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1284 , 6 ) 
    }
    pub fn cancel_record_program (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1284 , 7 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Channel`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.channel_inv_resp()` on an `InvokeResp`; the returned [`ChannelCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait ChannelCmdResponses < 'a > {
    fn channel_inv_resp (& self ) -> ChannelCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > ChannelCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn channel_inv_resp (& self ) -> ChannelCmdResponsesView < 'a , '_ > {
        ChannelCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `Channel` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ChannelAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > ChannelAttrResponsesView < 'a , 'r > {
    pub fn channel_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , ChannelInfoStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , ChannelInfoStruct < 'a > > > (1284 , 0 ) 
    }
    pub fn lineup (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < LineupInfoStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < LineupInfoStruct < 'a > > > (1284 , 1 ) 
    }
    pub fn current_channel (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < ChannelInfoStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < ChannelInfoStruct < 'a > > > (1284 , 2 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1284 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1284 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (1284 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (1284 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (1284 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `Channel`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.channel_read_resp()` on a `ReportDataResp`; the returned [`ChannelAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait ChannelAttrResponses < 'a > {
    fn channel_read_resp (& self ) -> ChannelAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > ChannelAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn channel_read_resp (& self ) -> ChannelAttrResponsesView < 'a , '_ > {
        ChannelAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `Channel` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct ChannelWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > ChannelWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `Channel`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.channel_write_resp()` on a `WriteResp`; the returned [`ChannelWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait ChannelWriteResponses < 'a > {
    fn channel_write_resp (& self ) -> ChannelWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > ChannelWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn channel_write_resp (& self ) -> ChannelWriteResponsesView < 'a , '_ > {
        ChannelWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `ChangeChannelResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`ChangeChannelResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct ChangeChannelResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > ChangeChannelResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < ChangeChannelResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `ProgramGuideResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`ProgramGuideResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct ProgramGuideResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > ProgramGuideResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < ProgramGuideResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `Channel` cluster's client operations. Returned by [`ChannelClient::channel`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct ChannelClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > ChannelClientView < T > {
    pub async fn change_channel < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < ChangeChannelResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (ChangeChannelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ChannelCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . channel_inv () ;
            let req_builder = view . change_channel (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (ChangeChannelResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn change_channel_by_number < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (ChangeChannelByNumberRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ChannelCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . channel_inv () ;
            let req_builder = view . change_channel_by_number (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn skip_channel < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SkipChannelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ChannelCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . channel_inv () ;
            let req_builder = view . skip_channel (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn get_program_guide < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < ProgramGuideResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (GetProgramGuideRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ChannelCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . channel_inv () ;
            let req_builder = view . get_program_guide (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (ProgramGuideResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn record_program < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (RecordProgramRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ChannelCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . channel_inv () ;
            let req_builder = view . record_program (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn cancel_record_program < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (CancelRecordProgramRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: ChannelCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . channel_inv () ;
            let req_builder = view . cancel_record_program (endpoint ) ? ;
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
                         attribute surfaces as `Err` to `f`." ] pub fn channel_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , ChannelInfoStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . channel_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn lineup_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < LineupInfoStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . lineup (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn current_channel_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: Nullable < ChannelInfoStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . current_channel (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: ChannelAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . channel_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `Channel` cluster. `use` this trait to call `.channel()` on any [`crate::im::ImClient`], such as an exchange; the returned [`ChannelClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `ChannelClientView` keeps each cluster's API surface narrow: at the call site `exchange.channel().` shows only this cluster's methods in IDE completion." ] pub trait ChannelClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn channel (self ) -> ChannelClientView < Self > {
        ChannelClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > ChannelClient for T {
    
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
    fn channel_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ChannelInfoStructArrayBuilder < P > , ChannelInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn lineup < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , LineupInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn current_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ChannelInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_change_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelRequest < '_ > , response : ChangeChannelResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_change_channel_by_number (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelByNumberRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_skip_channel (& self , ctx : impl crate :: dm :: InvokeContext , request : SkipChannelRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_get_program_guide < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetProgramGuideRequest < '_ > , response : ProgramGuideResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : RecordProgramRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_cancel_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : CancelRecordProgramRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn channel_list < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ChannelInfoStructArrayBuilder < P > , ChannelInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: channel_list (self , ctx , builder ) 
    }
    fn lineup < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , LineupInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: lineup (self , ctx , builder ) 
    }
    fn current_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: tlv :: NullableBuilder < P , ChannelInfoStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: current_channel (self , ctx , builder ) 
    }
    fn handle_change_channel < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelRequest < '_ > , response : ChangeChannelResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_change_channel (self , ctx , request , response ) 
    }
    fn handle_change_channel_by_number (& self , ctx : impl crate :: dm :: InvokeContext , request : ChangeChannelByNumberRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_change_channel_by_number (self , ctx , request ) 
    }
    fn handle_skip_channel (& self , ctx : impl crate :: dm :: InvokeContext , request : SkipChannelRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_skip_channel (self , ctx , request ) 
    }
    fn handle_get_program_guide < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : GetProgramGuideRequest < '_ > , response : ProgramGuideResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_get_program_guide (self , ctx , request , response ) 
    }
    fn handle_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : RecordProgramRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_record_program (self , ctx , request ) 
    }
    fn handle_cancel_record_program (& self , ctx : impl crate :: dm :: InvokeContext , request : CancelRecordProgramRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_cancel_record_program (self , ctx , request ) 
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
                    AttributeId :: ChannelList => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelList , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. channel_list (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelList , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ChannelList , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Lineup => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Lineup , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. lineup (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Lineup , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Lineup , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentChannel => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentChannel , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_channel (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentChannel , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentChannel , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: ChangeChannel => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannel ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_change_channel (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannel ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannel ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ChangeChannelByNumber => {
                let cmd_data : ChangeChannelByNumberRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_change_channel_by_number (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ChangeChannelByNumber ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SkipChannel => {
                let cmd_data : SkipChannelRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_skip_channel (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SkipChannel ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: GetProgramGuide => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetProgramGuide ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_get_program_guide (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetProgramGuide ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: GetProgramGuide ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RecordProgram => {
                let cmd_data : RecordProgramRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_record_program (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RecordProgram ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: CancelRecordProgram => {
                let cmd_data : CancelRecordProgramRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_cancel_record_program (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CancelRecordProgram ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 1284u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Channel" , 1284u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "Channel" , 1284u32 , self . 0.2 ) 
    }
    
}
