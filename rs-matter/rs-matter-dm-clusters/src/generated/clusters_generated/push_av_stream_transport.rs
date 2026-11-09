# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const PER_ZONE_SENSITIVITY = 1 ;
        const METADATA = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const PER_ZONE_SENSITIVITY = 1 ;
        const METADATA = 2 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CMAFInterfaceEnum {
    # [enumval (0 ) ] Interface1 = 0 , # [enumval (1 ) ] Interface2DASH = 1 , # [enumval (2 ) ] Interface2HLS = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ContainerFormatEnum {
    # [enumval (0 ) ] CMAF = 0 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum IngestMethodsEnum {
    # [enumval (0 ) ] CMAFIngest = 0 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StatusCodeEnum {
    # [enumval (2 ) ] InvalidTLSEndpoint = 2 , # [enumval (3 ) ] InvalidStream = 3 , # [enumval (4 ) ] InvalidURL = 4 , # [enumval (5 ) ] InvalidZone = 5 , # [enumval (6 ) ] InvalidCombination = 6 , # [enumval (7 ) ] InvalidTriggerType = 7 , # [enumval (8 ) ] InvalidTransportStatus = 8 , # [enumval (9 ) ] InvalidOptions = 9 , # [enumval (10 ) ] InvalidStreamUsage = 10 , # [enumval (11 ) ] InvalidTime = 11 , # [enumval (12 ) ] InvalidPreRollLength = 12 , # [enumval (13 ) ] DuplicateStreamValues = 13 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TransportStatusEnum {
    # [enumval (0 ) ] Active = 0 , # [enumval (1 ) ] Inactive = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TransportTriggerTypeEnum {
    # [enumval (0 ) ] Command = 0 , # [enumval (1 ) ] Motion = 1 , # [enumval (2 ) ] Continuous = 2 , # [enumval (3 ) ] AmbientContext = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TriggerActivationReasonEnum {
    # [enumval (0 ) ] UserInitiated = 0 , # [enumval (1 ) ] Automation = 1 , # [enumval (2 ) ] Emergency = 2 , # [enumval (3 ) ] DoorbellPressed = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TransportMotionTriggerTimeControlStructTag {
    InitialDuration = 0 , AugmentationDuration = 1 , MaxDuration = 2 , BlindDuration = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TransportZoneOptionsStructTag {
    Zone = 0 , Sensitivity = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AudioStreamStructTag {
    AudioStreamName = 0 , AudioStreamId = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoStreamStructTag {
    VideoStreamName = 0 , VideoStreamId = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TransportTriggerOptionsStructTag {
    TriggerType = 0 , MotionZones = 1 , MotionSensitivity = 2 , MotionTimeControl = 3 , MaxPreRollLen = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CMAFContainerOptionsStructTag {
    CmafInterface = 0 , SegmentDuration = 1 , ChunkDuration = 2 , SessionGroup = 3 , TrackName = 4 , MetadataEnabled = 7 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ContainerOptionsStructTag {
    ContainerType = 0 , CmafContainerOptions = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TransportOptionsStructTag {
    StreamUsage = 0 , VideoStreamId = 1 , AudioStreamId = 2 , TlsEndpointId = 3 , Url = 4 , TriggerOptions = 5 , IngestMethod = 6 , ContainerOptions = 7 , ExpiryTime = 8 , VideoStreams = 9 , AudioStreams = 10 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TransportConfigurationStructTag {
    ConnectionId = 0 , TransportStatus = 1 , TransportOptions = 2 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SupportedFormatStructTag {
    ContainerFormat = 0 , IngestMethod = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AllocatePushTransportRequestTag {
    TransportOptions = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AllocatePushTransportResponseTag {
    TransportConfiguration = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum DeallocatePushTransportRequestTag {
    ConnectionId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ModifyPushTransportRequestTag {
    ConnectionId = 0 , TransportOptions = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetTransportStatusRequestTag {
    ConnectionId = 0 , TransportStatus = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ManuallyTriggerTransportRequestTag {
    ConnectionId = 0 , ActivationReason = 1 , TimeControl = 2 , UserDefined = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum FindTransportRequestTag {
    ConnectionId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum FindTransportResponseTag {
    TransportConfigurations = 0 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct TransportMotionTriggerTimeControlStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > TransportMotionTriggerTimeControlStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn initial_duration (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn augmentation_duration (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn max_duration (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn blind_duration (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for TransportMotionTriggerTimeControlStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for TransportMotionTriggerTimeControlStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for TransportMotionTriggerTimeControlStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "TransportMotionTriggerTimeControlStruct" ) ? ;
        match self . initial_duration () {
            Ok (value ) => write ! (f , "{}: {:?}," , "initial_duration" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "initial_duration" , e . code () ) ? , 
        }
        match self . augmentation_duration () {
            Ok (value ) => write ! (f , "{}: {:?}," , "augmentation_duration" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "augmentation_duration" , e . code () ) ? , 
        }
        match self . max_duration () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_duration" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_duration" , e . code () ) ? , 
        }
        match self . blind_duration () {
            Ok (value ) => write ! (f , "{}: {:?}," , "blind_duration" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "blind_duration" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for TransportMotionTriggerTimeControlStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "TransportMotionTriggerTimeControlStruct" ) ;
        match self . initial_duration () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "initial_duration" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "initial_duration" , e . code () ) , 
        }
        match self . augmentation_duration () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "augmentation_duration" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "augmentation_duration" , e . code () ) , 
        }
        match self . max_duration () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_duration" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_duration" , e . code () ) , 
        }
        match self . blind_duration () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "blind_duration" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "blind_duration" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct TransportZoneOptionsStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > TransportZoneOptionsStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn zone (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn sensitivity (& self ) -> Result < Option < u8 > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for TransportZoneOptionsStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for TransportZoneOptionsStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for TransportZoneOptionsStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "TransportZoneOptionsStruct" ) ? ;
        match self . zone () {
            Ok (value ) => write ! (f , "{}: {:?}," , "zone" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "zone" , e . code () ) ? , 
        }
        match self . sensitivity () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "sensitivity" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "sensitivity" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sensitivity" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for TransportZoneOptionsStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "TransportZoneOptionsStruct" ) ;
        match self . zone () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "zone" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "zone" , e . code () ) , 
        }
        match self . sensitivity () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "sensitivity" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "sensitivity" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sensitivity" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AudioStreamStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AudioStreamStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn audio_stream_name (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn audio_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AudioStreamStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AudioStreamStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AudioStreamStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AudioStreamStruct" ) ? ;
        match self . audio_stream_name () {
            Ok (value ) => write ! (f , "{}: {:?}," , "audio_stream_name" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_name" , e . code () ) ? , 
        }
        match self . audio_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "audio_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AudioStreamStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AudioStreamStruct" ) ;
        match self . audio_stream_name () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "audio_stream_name" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_name" , e . code () ) , 
        }
        match self . audio_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "audio_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VideoStreamStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VideoStreamStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn video_stream_name (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn video_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VideoStreamStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VideoStreamStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VideoStreamStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VideoStreamStruct" ) ? ;
        match self . video_stream_name () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_stream_name" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_name" , e . code () ) ? , 
        }
        match self . video_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoStreamStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoStreamStruct" ) ;
        match self . video_stream_name () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_stream_name" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_name" , e . code () ) , 
        }
        match self . video_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct TransportTriggerOptionsStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > TransportTriggerOptionsStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn trigger_type (& self ) -> Result < TransportTriggerTypeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn motion_zones (& self ) -> Result < Option < crate :: tlv :: Nullable < crate :: tlv :: TLVArray < '_ , TransportZoneOptionsStruct < '_ > > > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn motion_sensitivity (& self ) -> Result < Option < crate :: tlv :: Nullable < u8 > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn motion_time_control (& self ) -> Result < Option < TransportMotionTriggerTimeControlStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn max_pre_roll_len (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for TransportTriggerOptionsStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for TransportTriggerOptionsStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for TransportTriggerOptionsStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "TransportTriggerOptionsStruct" ) ? ;
        match self . trigger_type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "trigger_type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "trigger_type" , e . code () ) ? , 
        }
        match self . motion_zones () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "motion_zones" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "motion_zones" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "motion_zones" , e . code () ) ? , 
        }
        match self . motion_sensitivity () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "motion_sensitivity" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "motion_sensitivity" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "motion_sensitivity" , e . code () ) ? , 
        }
        match self . motion_time_control () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "motion_time_control" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "motion_time_control" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "motion_time_control" , e . code () ) ? , 
        }
        match self . max_pre_roll_len () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "max_pre_roll_len" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "max_pre_roll_len" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_pre_roll_len" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for TransportTriggerOptionsStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "TransportTriggerOptionsStruct" ) ;
        match self . trigger_type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "trigger_type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "trigger_type" , e . code () ) , 
        }
        match self . motion_zones () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "motion_zones" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "motion_zones" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "motion_zones" , e . code () ) , 
        }
        match self . motion_sensitivity () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "motion_sensitivity" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "motion_sensitivity" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "motion_sensitivity" , e . code () ) , 
        }
        match self . motion_time_control () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "motion_time_control" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "motion_time_control" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "motion_time_control" , e . code () ) , 
        }
        match self . max_pre_roll_len () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "max_pre_roll_len" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "max_pre_roll_len" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_pre_roll_len" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CMAFContainerOptionsStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CMAFContainerOptionsStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn cmaf_interface (& self ) -> Result < CMAFInterfaceEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn segment_duration (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn chunk_duration (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn session_group (& self ) -> Result < Option < u8 > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn track_name (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn metadata_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (7 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CMAFContainerOptionsStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CMAFContainerOptionsStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CMAFContainerOptionsStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CMAFContainerOptionsStruct" ) ? ;
        match self . cmaf_interface () {
            Ok (value ) => write ! (f , "{}: {:?}," , "cmaf_interface" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cmaf_interface" , e . code () ) ? , 
        }
        match self . segment_duration () {
            Ok (value ) => write ! (f , "{}: {:?}," , "segment_duration" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "segment_duration" , e . code () ) ? , 
        }
        match self . chunk_duration () {
            Ok (value ) => write ! (f , "{}: {:?}," , "chunk_duration" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "chunk_duration" , e . code () ) ? , 
        }
        match self . session_group () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "session_group" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "session_group" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "session_group" , e . code () ) ? , 
        }
        match self . track_name () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "track_name" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "track_name" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "track_name" , e . code () ) ? , 
        }
        match self . metadata_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "metadata_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "metadata_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "metadata_enabled" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CMAFContainerOptionsStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CMAFContainerOptionsStruct" ) ;
        match self . cmaf_interface () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "cmaf_interface" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cmaf_interface" , e . code () ) , 
        }
        match self . segment_duration () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "segment_duration" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "segment_duration" , e . code () ) , 
        }
        match self . chunk_duration () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "chunk_duration" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "chunk_duration" , e . code () ) , 
        }
        match self . session_group () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "session_group" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "session_group" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "session_group" , e . code () ) , 
        }
        match self . track_name () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "track_name" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "track_name" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "track_name" , e . code () ) , 
        }
        match self . metadata_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "metadata_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "metadata_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "metadata_enabled" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ContainerOptionsStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ContainerOptionsStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn container_type (& self ) -> Result < ContainerFormatEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn cmaf_container_options (& self ) -> Result < Option < CMAFContainerOptionsStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ContainerOptionsStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ContainerOptionsStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ContainerOptionsStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ContainerOptionsStruct" ) ? ;
        match self . container_type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "container_type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "container_type" , e . code () ) ? , 
        }
        match self . cmaf_container_options () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "cmaf_container_options" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "cmaf_container_options" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cmaf_container_options" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ContainerOptionsStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ContainerOptionsStruct" ) ;
        match self . container_type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "container_type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "container_type" , e . code () ) , 
        }
        match self . cmaf_container_options () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "cmaf_container_options" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "cmaf_container_options" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cmaf_container_options" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct TransportOptionsStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > TransportOptionsStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn stream_usage (& self ) -> Result < StreamUsageEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn video_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn audio_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn tls_endpoint_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn url (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn trigger_options (& self ) -> Result < TransportTriggerOptionsStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn ingest_method (& self ) -> Result < IngestMethodsEnum , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn container_options (& self ) -> Result < ContainerOptionsStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn expiry_time (& self ) -> Result < Option < u32 > , crate :: error :: Error > {
        self . 0 . read_opt (8 ) 
    }
    pub fn video_streams (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , VideoStreamStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (9 ) 
    }
    pub fn audio_streams (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , AudioStreamStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (10 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for TransportOptionsStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for TransportOptionsStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for TransportOptionsStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "TransportOptionsStruct" ) ? ;
        match self . stream_usage () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_usage" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) ? , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "video_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "audio_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        match self . tls_endpoint_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "tls_endpoint_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "tls_endpoint_id" , e . code () ) ? , 
        }
        match self . url () {
            Ok (value ) => write ! (f , "{}: {:?}," , "url" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "url" , e . code () ) ? , 
        }
        match self . trigger_options () {
            Ok (value ) => write ! (f , "{}: {:?}," , "trigger_options" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "trigger_options" , e . code () ) ? , 
        }
        match self . ingest_method () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ingest_method" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ingest_method" , e . code () ) ? , 
        }
        match self . container_options () {
            Ok (value ) => write ! (f , "{}: {:?}," , "container_options" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "container_options" , e . code () ) ? , 
        }
        match self . expiry_time () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "expiry_time" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "expiry_time" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "expiry_time" , e . code () ) ? , 
        }
        match self . video_streams () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "video_streams" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "video_streams" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_streams" , e . code () ) ? , 
        }
        match self . audio_streams () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "audio_streams" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "audio_streams" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_streams" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for TransportOptionsStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "TransportOptionsStruct" ) ;
        match self . stream_usage () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_usage" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "video_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "audio_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        match self . tls_endpoint_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "tls_endpoint_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "tls_endpoint_id" , e . code () ) , 
        }
        match self . url () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "url" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "url" , e . code () ) , 
        }
        match self . trigger_options () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "trigger_options" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "trigger_options" , e . code () ) , 
        }
        match self . ingest_method () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ingest_method" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ingest_method" , e . code () ) , 
        }
        match self . container_options () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "container_options" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "container_options" , e . code () ) , 
        }
        match self . expiry_time () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "expiry_time" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "expiry_time" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "expiry_time" , e . code () ) , 
        }
        match self . video_streams () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "video_streams" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "video_streams" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_streams" , e . code () ) , 
        }
        match self . audio_streams () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "audio_streams" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "audio_streams" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_streams" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct TransportConfigurationStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > TransportConfigurationStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transport_status (& self ) -> Result < TransportStatusEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn transport_options (& self ) -> Result < Option < TransportOptionsStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for TransportConfigurationStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for TransportConfigurationStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for TransportConfigurationStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "TransportConfigurationStruct" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        match self . transport_status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transport_status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transport_status" , e . code () ) ? , 
        }
        match self . transport_options () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "transport_options" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "transport_options" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transport_options" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for TransportConfigurationStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "TransportConfigurationStruct" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        match self . transport_status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transport_status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transport_status" , e . code () ) , 
        }
        match self . transport_options () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "transport_options" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "transport_options" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transport_options" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SupportedFormatStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SupportedFormatStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn container_format (& self ) -> Result < ContainerFormatEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn ingest_method (& self ) -> Result < IngestMethodsEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SupportedFormatStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SupportedFormatStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SupportedFormatStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SupportedFormatStruct" ) ? ;
        match self . container_format () {
            Ok (value ) => write ! (f , "{}: {:?}," , "container_format" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "container_format" , e . code () ) ? , 
        }
        match self . ingest_method () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ingest_method" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ingest_method" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SupportedFormatStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SupportedFormatStruct" ) ;
        match self . container_format () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "container_format" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "container_format" , e . code () ) , 
        }
        match self . ingest_method () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ingest_method" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ingest_method" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AllocatePushTransportRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AllocatePushTransportRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn transport_options (& self ) -> Result < TransportOptionsStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AllocatePushTransportRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AllocatePushTransportRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AllocatePushTransportRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AllocatePushTransportRequest" ) ? ;
        match self . transport_options () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transport_options" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transport_options" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AllocatePushTransportRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AllocatePushTransportRequest" ) ;
        match self . transport_options () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transport_options" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transport_options" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AllocatePushTransportResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AllocatePushTransportResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn transport_configuration (& self ) -> Result < TransportConfigurationStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AllocatePushTransportResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AllocatePushTransportResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AllocatePushTransportResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AllocatePushTransportResponse" ) ? ;
        match self . transport_configuration () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transport_configuration" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transport_configuration" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AllocatePushTransportResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AllocatePushTransportResponse" ) ;
        match self . transport_configuration () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transport_configuration" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transport_configuration" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct DeallocatePushTransportRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > DeallocatePushTransportRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for DeallocatePushTransportRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for DeallocatePushTransportRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for DeallocatePushTransportRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "DeallocatePushTransportRequest" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for DeallocatePushTransportRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "DeallocatePushTransportRequest" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ModifyPushTransportRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ModifyPushTransportRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transport_options (& self ) -> Result < TransportOptionsStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ModifyPushTransportRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ModifyPushTransportRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ModifyPushTransportRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ModifyPushTransportRequest" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        match self . transport_options () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transport_options" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transport_options" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ModifyPushTransportRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ModifyPushTransportRequest" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        match self . transport_options () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transport_options" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transport_options" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetTransportStatusRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetTransportStatusRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn transport_status (& self ) -> Result < TransportStatusEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetTransportStatusRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetTransportStatusRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetTransportStatusRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetTransportStatusRequest" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        match self . transport_status () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transport_status" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transport_status" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetTransportStatusRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetTransportStatusRequest" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        match self . transport_status () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transport_status" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transport_status" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ManuallyTriggerTransportRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ManuallyTriggerTransportRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn activation_reason (& self ) -> Result < TriggerActivationReasonEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn time_control (& self ) -> Result < Option < TransportMotionTriggerTimeControlStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn user_defined (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ManuallyTriggerTransportRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ManuallyTriggerTransportRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ManuallyTriggerTransportRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ManuallyTriggerTransportRequest" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        match self . activation_reason () {
            Ok (value ) => write ! (f , "{}: {:?}," , "activation_reason" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "activation_reason" , e . code () ) ? , 
        }
        match self . time_control () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "time_control" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "time_control" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "time_control" , e . code () ) ? , 
        }
        match self . user_defined () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "user_defined" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "user_defined" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "user_defined" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ManuallyTriggerTransportRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ManuallyTriggerTransportRequest" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        match self . activation_reason () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "activation_reason" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "activation_reason" , e . code () ) , 
        }
        match self . time_control () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "time_control" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "time_control" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "time_control" , e . code () ) , 
        }
        match self . user_defined () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "user_defined" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "user_defined" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "user_defined" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct FindTransportRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > FindTransportRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for FindTransportRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for FindTransportRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for FindTransportRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "FindTransportRequest" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for FindTransportRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "FindTransportRequest" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct FindTransportResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > FindTransportResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn transport_configurations (& self ) -> Result < crate :: tlv :: TLVArray < '_ , TransportConfigurationStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for FindTransportResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for FindTransportResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for FindTransportResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "FindTransportResponse" ) ? ;
        match self . transport_configurations () {
            Ok (value ) => write ! (f , "{}: {:?}," , "transport_configurations" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "transport_configurations" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for FindTransportResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "FindTransportResponse" ) ;
        match self . transport_configurations () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "transport_configurations" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "transport_configurations" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct TransportMotionTriggerTimeControlStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > TransportMotionTriggerTimeControlStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > TransportMotionTriggerTimeControlStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn initial_duration (mut self , value : u16 ) -> Result < TransportMotionTriggerTimeControlStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "initialDuration" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (TransportMotionTriggerTimeControlStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportMotionTriggerTimeControlStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn augmentation_duration (mut self , value : u16 ) -> Result < TransportMotionTriggerTimeControlStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "augmentationDuration" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (TransportMotionTriggerTimeControlStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportMotionTriggerTimeControlStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_duration (mut self , value : u32 ) -> Result < TransportMotionTriggerTimeControlStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxDuration" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (TransportMotionTriggerTimeControlStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportMotionTriggerTimeControlStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn blind_duration (mut self , value : u16 ) -> Result < TransportMotionTriggerTimeControlStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "blindDuration" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (TransportMotionTriggerTimeControlStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportMotionTriggerTimeControlStructBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for TransportMotionTriggerTimeControlStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportMotionTriggerTimeControlStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for TransportMotionTriggerTimeControlStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportMotionTriggerTimeControlStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for TransportMotionTriggerTimeControlStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportMotionTriggerTimeControlStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportMotionTriggerTimeControlStructArrayBuilder < P > (P ) ;
impl < P > TransportMotionTriggerTimeControlStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < TransportMotionTriggerTimeControlStructBuilder < TransportMotionTriggerTimeControlStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportMotionTriggerTimeControlStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for TransportMotionTriggerTimeControlStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportMotionTriggerTimeControlStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for TransportMotionTriggerTimeControlStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportMotionTriggerTimeControlStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for TransportMotionTriggerTimeControlStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportMotionTriggerTimeControlStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportZoneOptionsStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > TransportZoneOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > TransportZoneOptionsStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn zone (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < TransportZoneOptionsStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "zone" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (TransportZoneOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportZoneOptionsStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sensitivity (mut self , value : Option < u8 > ) -> Result < TransportZoneOptionsStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sensitivity" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (TransportZoneOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportZoneOptionsStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for TransportZoneOptionsStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportZoneOptionsStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for TransportZoneOptionsStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportZoneOptionsStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for TransportZoneOptionsStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportZoneOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportZoneOptionsStructArrayBuilder < P > (P ) ;
impl < P > TransportZoneOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < TransportZoneOptionsStructBuilder < TransportZoneOptionsStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportZoneOptionsStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for TransportZoneOptionsStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportZoneOptionsStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for TransportZoneOptionsStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportZoneOptionsStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for TransportZoneOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportZoneOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AudioStreamStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_name (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < AudioStreamStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamName" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : u16 ) -> Result < AudioStreamStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AudioStreamStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AudioStreamStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AudioStreamStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamStructArrayBuilder < P > (P ) ;
impl < P > AudioStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AudioStreamStructBuilder < AudioStreamStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioStreamStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AudioStreamStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AudioStreamStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AudioStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VideoStreamStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_name (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < VideoStreamStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamName" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : u16 ) -> Result < VideoStreamStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VideoStreamStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VideoStreamStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VideoStreamStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamStructArrayBuilder < P > (P ) ;
impl < P > VideoStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VideoStreamStructBuilder < VideoStreamStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VideoStreamStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VideoStreamStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VideoStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportTriggerOptionsStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > TransportTriggerOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > TransportTriggerOptionsStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn trigger_type (mut self , value : TransportTriggerTypeEnum ) -> Result < TransportTriggerOptionsStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "triggerType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (TransportTriggerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportTriggerOptionsStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn motion_zones (self ) -> Result < crate :: tlv :: OptionalBuilder < TransportTriggerOptionsStructBuilder < P , 2usize > , crate :: tlv :: NullableBuilder < TransportTriggerOptionsStructBuilder < P , 2usize > , TransportZoneOptionsStructArrayBuilder < TransportTriggerOptionsStructBuilder < P , 2usize > > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportTriggerOptionsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > TransportTriggerOptionsStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn motion_sensitivity (mut self , value : Option < crate :: tlv :: Nullable < u8 > > ) -> Result < TransportTriggerOptionsStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "motionSensitivity" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (TransportTriggerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportTriggerOptionsStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn motion_time_control (self ) -> Result < crate :: tlv :: OptionalBuilder < TransportTriggerOptionsStructBuilder < P , 4usize > , TransportMotionTriggerTimeControlStructBuilder < TransportTriggerOptionsStructBuilder < P , 4usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportTriggerOptionsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > TransportTriggerOptionsStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_pre_roll_len (mut self , value : Option < u16 > ) -> Result < TransportTriggerOptionsStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxPreRollLen" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (TransportTriggerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportTriggerOptionsStructBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for TransportTriggerOptionsStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportTriggerOptionsStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for TransportTriggerOptionsStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportTriggerOptionsStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for TransportTriggerOptionsStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportTriggerOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportTriggerOptionsStructArrayBuilder < P > (P ) ;
impl < P > TransportTriggerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < TransportTriggerOptionsStructBuilder < TransportTriggerOptionsStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportTriggerOptionsStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for TransportTriggerOptionsStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportTriggerOptionsStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for TransportTriggerOptionsStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportTriggerOptionsStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for TransportTriggerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportTriggerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CMAFContainerOptionsStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CMAFContainerOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CMAFContainerOptionsStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cmaf_interface (mut self , value : CMAFInterfaceEnum ) -> Result < CMAFContainerOptionsStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "CMAFInterface" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CMAFContainerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > CMAFContainerOptionsStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn segment_duration (mut self , value : u16 ) -> Result < CMAFContainerOptionsStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "segmentDuration" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (CMAFContainerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > CMAFContainerOptionsStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn chunk_duration (mut self , value : u16 ) -> Result < CMAFContainerOptionsStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "chunkDuration" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (CMAFContainerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > CMAFContainerOptionsStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn session_group (mut self , value : Option < u8 > ) -> Result < CMAFContainerOptionsStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sessionGroup" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (CMAFContainerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > CMAFContainerOptionsStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn track_name (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < CMAFContainerOptionsStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "trackName" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (CMAFContainerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > CMAFContainerOptionsStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn metadata_enabled (mut self , value : Option < bool > ) -> Result < CMAFContainerOptionsStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "metadataEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (CMAFContainerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > CMAFContainerOptionsStructBuilder < P , 8usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CMAFContainerOptionsStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CMAFContainerOptionsStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CMAFContainerOptionsStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CMAFContainerOptionsStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CMAFContainerOptionsStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CMAFContainerOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CMAFContainerOptionsStructArrayBuilder < P > (P ) ;
impl < P > CMAFContainerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CMAFContainerOptionsStructBuilder < CMAFContainerOptionsStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CMAFContainerOptionsStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CMAFContainerOptionsStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CMAFContainerOptionsStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CMAFContainerOptionsStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CMAFContainerOptionsStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CMAFContainerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CMAFContainerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ContainerOptionsStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ContainerOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ContainerOptionsStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn container_type (mut self , value : ContainerFormatEnum ) -> Result < ContainerOptionsStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "containerType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ContainerOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > ContainerOptionsStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cmaf_container_options (self ) -> Result < crate :: tlv :: OptionalBuilder < ContainerOptionsStructBuilder < P , 2usize > , CMAFContainerOptionsStructBuilder < ContainerOptionsStructBuilder < P , 2usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ContainerOptionsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > ContainerOptionsStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ContainerOptionsStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ContainerOptionsStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ContainerOptionsStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ContainerOptionsStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ContainerOptionsStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ContainerOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ContainerOptionsStructArrayBuilder < P > (P ) ;
impl < P > ContainerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ContainerOptionsStructBuilder < ContainerOptionsStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ContainerOptionsStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ContainerOptionsStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ContainerOptionsStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ContainerOptionsStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ContainerOptionsStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ContainerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ContainerOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportOptionsStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > TransportOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_usage (mut self , value : StreamUsageEnum ) -> Result < TransportOptionsStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "streamUsage" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (TransportOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < TransportOptionsStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (TransportOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < TransportOptionsStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (TransportOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn tls_endpoint_id (mut self , value : u16 ) -> Result < TransportOptionsStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "TLSEndpointID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (TransportOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn url (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < TransportOptionsStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "url" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (TransportOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn trigger_options (self ) -> Result < TransportTriggerOptionsStructBuilder < TransportOptionsStructBuilder < P , 6usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportOptionsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (5 ) , ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ingest_method (mut self , value : IngestMethodsEnum ) -> Result < TransportOptionsStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ingestMethod" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (TransportOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn container_options (self ) -> Result < ContainerOptionsStructBuilder < TransportOptionsStructBuilder < P , 8usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportOptionsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (7 ) , ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn expiry_time (mut self , value : Option < u32 > ) -> Result < TransportOptionsStructBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "expiryTime" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (TransportOptionsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_streams (self ) -> Result < crate :: tlv :: OptionalBuilder < TransportOptionsStructBuilder < P , 10usize > , VideoStreamStructArrayBuilder < TransportOptionsStructBuilder < P , 10usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportOptionsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (9 ) , ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_streams (self ) -> Result < crate :: tlv :: OptionalBuilder < TransportOptionsStructBuilder < P , 11usize > , AudioStreamStructArrayBuilder < TransportOptionsStructBuilder < P , 11usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportOptionsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (10 ) , ) 
    }
    
}
impl < P > TransportOptionsStructBuilder < P , 11usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for TransportOptionsStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportOptionsStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for TransportOptionsStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportOptionsStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for TransportOptionsStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportOptionsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportOptionsStructArrayBuilder < P > (P ) ;
impl < P > TransportOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < TransportOptionsStructBuilder < TransportOptionsStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportOptionsStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for TransportOptionsStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportOptionsStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for TransportOptionsStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportOptionsStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for TransportOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportOptionsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportConfigurationStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > TransportConfigurationStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > TransportConfigurationStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : u16 ) -> Result < TransportConfigurationStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (TransportConfigurationStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportConfigurationStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transport_status (mut self , value : TransportStatusEnum ) -> Result < TransportConfigurationStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transportStatus" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (TransportConfigurationStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportConfigurationStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transport_options (self ) -> Result < crate :: tlv :: OptionalBuilder < TransportConfigurationStructBuilder < P , 254usize > , TransportOptionsStructBuilder < TransportConfigurationStructBuilder < P , 254usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportConfigurationStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > TransportConfigurationStructBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < TransportConfigurationStructBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (TransportConfigurationStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > TransportConfigurationStructBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for TransportConfigurationStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportConfigurationStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for TransportConfigurationStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportConfigurationStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for TransportConfigurationStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportConfigurationStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct TransportConfigurationStructArrayBuilder < P > (P ) ;
impl < P > TransportConfigurationStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < TransportConfigurationStructBuilder < TransportConfigurationStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (TransportConfigurationStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for TransportConfigurationStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "TransportConfigurationStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for TransportConfigurationStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "TransportConfigurationStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for TransportConfigurationStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for TransportConfigurationStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SupportedFormatStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SupportedFormatStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SupportedFormatStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn container_format (mut self , value : ContainerFormatEnum ) -> Result < SupportedFormatStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "containerFormat" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SupportedFormatStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SupportedFormatStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ingest_method (mut self , value : IngestMethodsEnum ) -> Result < SupportedFormatStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ingestMethod" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SupportedFormatStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SupportedFormatStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SupportedFormatStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SupportedFormatStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SupportedFormatStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SupportedFormatStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SupportedFormatStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SupportedFormatStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SupportedFormatStructArrayBuilder < P > (P ) ;
impl < P > SupportedFormatStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SupportedFormatStructBuilder < SupportedFormatStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SupportedFormatStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SupportedFormatStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SupportedFormatStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SupportedFormatStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SupportedFormatStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SupportedFormatStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SupportedFormatStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AllocatePushTransportRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AllocatePushTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AllocatePushTransportRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transport_options (self ) -> Result < TransportOptionsStructBuilder < AllocatePushTransportRequestBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AllocatePushTransportRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > AllocatePushTransportRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AllocatePushTransportRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AllocatePushTransportRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AllocatePushTransportRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AllocatePushTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AllocatePushTransportRequestArrayBuilder < P > (P ) ;
impl < P > AllocatePushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AllocatePushTransportRequestBuilder < AllocatePushTransportRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AllocatePushTransportRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AllocatePushTransportRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AllocatePushTransportRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AllocatePushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AllocatePushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AllocatePushTransportResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AllocatePushTransportResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AllocatePushTransportResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transport_configuration (self ) -> Result < TransportConfigurationStructBuilder < AllocatePushTransportResponseBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AllocatePushTransportResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > AllocatePushTransportResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AllocatePushTransportResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AllocatePushTransportResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AllocatePushTransportResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AllocatePushTransportResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AllocatePushTransportResponseArrayBuilder < P > (P ) ;
impl < P > AllocatePushTransportResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AllocatePushTransportResponseBuilder < AllocatePushTransportResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AllocatePushTransportResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AllocatePushTransportResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AllocatePushTransportResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AllocatePushTransportResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AllocatePushTransportResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AllocatePushTransportResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct DeallocatePushTransportRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > DeallocatePushTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > DeallocatePushTransportRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : u16 ) -> Result < DeallocatePushTransportRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (DeallocatePushTransportRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > DeallocatePushTransportRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for DeallocatePushTransportRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "DeallocatePushTransportRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for DeallocatePushTransportRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "DeallocatePushTransportRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for DeallocatePushTransportRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for DeallocatePushTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct DeallocatePushTransportRequestArrayBuilder < P > (P ) ;
impl < P > DeallocatePushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < DeallocatePushTransportRequestBuilder < DeallocatePushTransportRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (DeallocatePushTransportRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for DeallocatePushTransportRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "DeallocatePushTransportRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for DeallocatePushTransportRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "DeallocatePushTransportRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for DeallocatePushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for DeallocatePushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ModifyPushTransportRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ModifyPushTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ModifyPushTransportRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : u16 ) -> Result < ModifyPushTransportRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ModifyPushTransportRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ModifyPushTransportRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transport_options (self ) -> Result < TransportOptionsStructBuilder < ModifyPushTransportRequestBuilder < P , 2usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ModifyPushTransportRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > ModifyPushTransportRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ModifyPushTransportRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ModifyPushTransportRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ModifyPushTransportRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ModifyPushTransportRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ModifyPushTransportRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ModifyPushTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ModifyPushTransportRequestArrayBuilder < P > (P ) ;
impl < P > ModifyPushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ModifyPushTransportRequestBuilder < ModifyPushTransportRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ModifyPushTransportRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ModifyPushTransportRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ModifyPushTransportRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ModifyPushTransportRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ModifyPushTransportRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ModifyPushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ModifyPushTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetTransportStatusRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetTransportStatusRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetTransportStatusRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < SetTransportStatusRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetTransportStatusRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetTransportStatusRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transport_status (mut self , value : TransportStatusEnum ) -> Result < SetTransportStatusRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "transportStatus" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetTransportStatusRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetTransportStatusRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetTransportStatusRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetTransportStatusRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetTransportStatusRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetTransportStatusRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetTransportStatusRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetTransportStatusRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetTransportStatusRequestArrayBuilder < P > (P ) ;
impl < P > SetTransportStatusRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetTransportStatusRequestBuilder < SetTransportStatusRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetTransportStatusRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetTransportStatusRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetTransportStatusRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetTransportStatusRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetTransportStatusRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetTransportStatusRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetTransportStatusRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ManuallyTriggerTransportRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ManuallyTriggerTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ManuallyTriggerTransportRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : u16 ) -> Result < ManuallyTriggerTransportRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ManuallyTriggerTransportRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ManuallyTriggerTransportRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn activation_reason (mut self , value : TriggerActivationReasonEnum ) -> Result < ManuallyTriggerTransportRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "activationReason" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ManuallyTriggerTransportRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ManuallyTriggerTransportRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn time_control (self ) -> Result < crate :: tlv :: OptionalBuilder < ManuallyTriggerTransportRequestBuilder < P , 3usize > , TransportMotionTriggerTimeControlStructBuilder < ManuallyTriggerTransportRequestBuilder < P , 3usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ManuallyTriggerTransportRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > ManuallyTriggerTransportRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn user_defined (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < ManuallyTriggerTransportRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "userDefined" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ManuallyTriggerTransportRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ManuallyTriggerTransportRequestBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ManuallyTriggerTransportRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ManuallyTriggerTransportRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ManuallyTriggerTransportRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ManuallyTriggerTransportRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ManuallyTriggerTransportRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ManuallyTriggerTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ManuallyTriggerTransportRequestArrayBuilder < P > (P ) ;
impl < P > ManuallyTriggerTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ManuallyTriggerTransportRequestBuilder < ManuallyTriggerTransportRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ManuallyTriggerTransportRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ManuallyTriggerTransportRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ManuallyTriggerTransportRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ManuallyTriggerTransportRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ManuallyTriggerTransportRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ManuallyTriggerTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ManuallyTriggerTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct FindTransportRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > FindTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > FindTransportRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < FindTransportRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (FindTransportRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > FindTransportRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for FindTransportRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "FindTransportRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for FindTransportRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "FindTransportRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for FindTransportRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for FindTransportRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct FindTransportRequestArrayBuilder < P > (P ) ;
impl < P > FindTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < FindTransportRequestBuilder < FindTransportRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (FindTransportRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for FindTransportRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "FindTransportRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for FindTransportRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "FindTransportRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for FindTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for FindTransportRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct FindTransportResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > FindTransportResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > FindTransportResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn transport_configurations (self ) -> Result < TransportConfigurationStructArrayBuilder < FindTransportResponseBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (FindTransportResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > FindTransportResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for FindTransportResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "FindTransportResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for FindTransportResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "FindTransportResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for FindTransportResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for FindTransportResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct FindTransportResponseArrayBuilder < P > (P ) ;
impl < P > FindTransportResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < FindTransportResponseBuilder < FindTransportResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (FindTransportResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for FindTransportResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "FindTransportResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for FindTransportResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "FindTransportResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for FindTransportResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for FindTransportResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PushTransportBeginTag {
    ConnectionId = 0 , TriggerType = 1 , ActivationReason = 2 , ContainerType = 3 , CmafSessionNumber = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum PushTransportEndTag {
    ConnectionId = 0 , ContainerType = 1 , CmafSessionNumber = 2 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct PushTransportBegin < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > PushTransportBegin < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn trigger_type (& self ) -> Result < TransportTriggerTypeEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn activation_reason (& self ) -> Result < Option < TriggerActivationReasonEnum > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn container_type (& self ) -> Result < Option < ContainerFormatEnum > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn cmaf_session_number (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for PushTransportBegin < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for PushTransportBegin < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for PushTransportBegin < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "PushTransportBegin" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        match self . trigger_type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "trigger_type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "trigger_type" , e . code () ) ? , 
        }
        match self . activation_reason () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "activation_reason" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "activation_reason" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "activation_reason" , e . code () ) ? , 
        }
        match self . container_type () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "container_type" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "container_type" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "container_type" , e . code () ) ? , 
        }
        match self . cmaf_session_number () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "cmaf_session_number" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "cmaf_session_number" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cmaf_session_number" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for PushTransportBegin < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "PushTransportBegin" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        match self . trigger_type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "trigger_type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "trigger_type" , e . code () ) , 
        }
        match self . activation_reason () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "activation_reason" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "activation_reason" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "activation_reason" , e . code () ) , 
        }
        match self . container_type () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "container_type" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "container_type" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "container_type" , e . code () ) , 
        }
        match self . cmaf_session_number () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "cmaf_session_number" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "cmaf_session_number" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cmaf_session_number" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct PushTransportEnd < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > PushTransportEnd < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn connection_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn container_type (& self ) -> Result < Option < ContainerFormatEnum > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn cmaf_session_number (& self ) -> Result < Option < u64 > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for PushTransportEnd < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for PushTransportEnd < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for PushTransportEnd < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "PushTransportEnd" ) ? ;
        match self . connection_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "connection_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) ? , 
        }
        match self . container_type () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "container_type" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "container_type" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "container_type" , e . code () ) ? , 
        }
        match self . cmaf_session_number () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "cmaf_session_number" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "cmaf_session_number" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cmaf_session_number" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for PushTransportEnd < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "PushTransportEnd" ) ;
        match self . connection_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "connection_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "connection_id" , e . code () ) , 
        }
        match self . container_type () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "container_type" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "container_type" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "container_type" , e . code () ) , 
        }
        match self . cmaf_session_number () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "cmaf_session_number" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "cmaf_session_number" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cmaf_session_number" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct PushTransportBeginBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > PushTransportBeginBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > PushTransportBeginBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : u16 ) -> Result < PushTransportBeginBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (PushTransportBeginBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportBeginBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn trigger_type (mut self , value : TransportTriggerTypeEnum ) -> Result < PushTransportBeginBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "triggerType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (PushTransportBeginBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportBeginBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn activation_reason (mut self , value : Option < TriggerActivationReasonEnum > ) -> Result < PushTransportBeginBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "activationReason" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (PushTransportBeginBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportBeginBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn container_type (mut self , value : Option < ContainerFormatEnum > ) -> Result < PushTransportBeginBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "containerType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (PushTransportBeginBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportBeginBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cmaf_session_number (mut self , value : Option < u64 > ) -> Result < PushTransportBeginBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "CMAFSessionNumber" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (PushTransportBeginBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportBeginBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for PushTransportBeginBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PushTransportBegin" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for PushTransportBeginBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PushTransportBegin" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for PushTransportBeginBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PushTransportBeginBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PushTransportBeginArrayBuilder < P > (P ) ;
impl < P > PushTransportBeginArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < PushTransportBeginBuilder < PushTransportBeginArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (PushTransportBeginArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for PushTransportBeginArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PushTransportBegin[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for PushTransportBeginArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PushTransportBegin[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for PushTransportBeginArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PushTransportBeginArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PushTransportEndBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > PushTransportEndBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > PushTransportEndBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn connection_id (mut self , value : u16 ) -> Result < PushTransportEndBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "connectionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (PushTransportEndBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportEndBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn container_type (mut self , value : Option < ContainerFormatEnum > ) -> Result < PushTransportEndBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "containerType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (PushTransportEndBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportEndBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cmaf_session_number (mut self , value : Option < u64 > ) -> Result < PushTransportEndBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "CMAFSessionNumber" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (PushTransportEndBuilder (self . 0 ) ) 
    }
    
}
impl < P > PushTransportEndBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for PushTransportEndBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PushTransportEnd" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for PushTransportEndBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PushTransportEnd" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for PushTransportEndBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PushTransportEndBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct PushTransportEndArrayBuilder < P > (P ) ;
impl < P > PushTransportEndArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < PushTransportEndBuilder < PushTransportEndArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (PushTransportEndArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for PushTransportEndArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "PushTransportEnd[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for PushTransportEndArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "PushTransportEnd[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for PushTransportEndArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for PushTransportEndArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl PushTransportBegin < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (PushTransportBeginBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 1365 , EventId :: PushTransportBegin as _ , Self :: PRIORITY , | tw | {
            f (PushTransportBeginBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (PushTransportBeginBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: PushTransportBegin as _ , Self :: PRIORITY , | tw | {
            f (PushTransportBeginBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl PushTransportEnd < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (PushTransportEndBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 1365 , EventId :: PushTransportEnd as _ , Self :: PRIORITY , | tw | {
            f (PushTransportEndBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (PushTransportEndBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: PushTransportEnd as _ , Self :: PRIORITY , | tw | {
            f (PushTransportEndBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    SupportedFormats = 0 , CurrentConnections = 1 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: SupportedFormats => write ! (f , "{}(0x{:02x})" , "SupportedFormats" , AttributeId :: SupportedFormats as u32 ) ? , AttributeId :: CurrentConnections => write ! (f , "{}(0x{:02x})" , "CurrentConnections" , AttributeId :: CurrentConnections as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: SupportedFormats => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SupportedFormats" , AttributeId :: SupportedFormats as u32 ) , AttributeId :: CurrentConnections => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentConnections" , AttributeId :: CurrentConnections as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    AllocatePushTransport = 0 , DeallocatePushTransport = 2 , ModifyPushTransport = 3 , SetTransportStatus = 4 , ManuallyTriggerTransport = 5 , FindTransport = 6 
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
            CommandId :: AllocatePushTransport => write ! (f , "{}(0x{:02x})" , "AllocatePushTransport" , CommandId :: AllocatePushTransport as u32 ) ? , CommandId :: DeallocatePushTransport => write ! (f , "{}(0x{:02x})" , "DeallocatePushTransport" , CommandId :: DeallocatePushTransport as u32 ) ? , CommandId :: ModifyPushTransport => write ! (f , "{}(0x{:02x})" , "ModifyPushTransport" , CommandId :: ModifyPushTransport as u32 ) ? , CommandId :: SetTransportStatus => write ! (f , "{}(0x{:02x})" , "SetTransportStatus" , CommandId :: SetTransportStatus as u32 ) ? , CommandId :: ManuallyTriggerTransport => write ! (f , "{}(0x{:02x})" , "ManuallyTriggerTransport" , CommandId :: ManuallyTriggerTransport as u32 ) ? , CommandId :: FindTransport => write ! (f , "{}(0x{:02x})" , "FindTransport" , CommandId :: FindTransport as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: AllocatePushTransport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AllocatePushTransport" , CommandId :: AllocatePushTransport as u32 ) , CommandId :: DeallocatePushTransport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DeallocatePushTransport" , CommandId :: DeallocatePushTransport as u32 ) , CommandId :: ModifyPushTransport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ModifyPushTransport" , CommandId :: ModifyPushTransport as u32 ) , CommandId :: SetTransportStatus => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetTransportStatus" , CommandId :: SetTransportStatus as u32 ) , CommandId :: ManuallyTriggerTransport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ManuallyTriggerTransport" , CommandId :: ManuallyTriggerTransport as u32 ) , CommandId :: FindTransport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FindTransport" , CommandId :: FindTransport as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    AllocatePushTransportResponse = 1 , FindTransportResponse = 7 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: AllocatePushTransportResponse => write ! (f , "{}(0x{:02x})" , "AllocatePushTransportResponse" , CommandResponseId :: AllocatePushTransportResponse as u32 ) ? , CommandResponseId :: FindTransportResponse => write ! (f , "{}(0x{:02x})" , "FindTransportResponse" , CommandResponseId :: FindTransportResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: AllocatePushTransportResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AllocatePushTransportResponse" , CommandResponseId :: AllocatePushTransportResponse as u32 ) , CommandResponseId :: FindTransportResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FindTransportResponse" , CommandResponseId :: FindTransportResponse as u32 ) , 
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
    PushTransportBegin = 0 , PushTransportEnd = 1 
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
            EventId :: PushTransportBegin => write ! (f , "{}(0x{:02x})" , "PushTransportBegin" , EventId :: PushTransportBegin as u32 ) ? , EventId :: PushTransportEnd => write ! (f , "{}(0x{:02x})" , "PushTransportEnd" , EventId :: PushTransportEnd as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: PushTransportBegin => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PushTransportBegin" , EventId :: PushTransportBegin as u32 ) , EventId :: PushTransportEnd => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "PushTransportEnd" , EventId :: PushTransportEnd as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
use super :: globals :: * ;
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (1365 , 3 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: SupportedFormats as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentConnections as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: AllocatePushTransport as _ , Some (CommandResponseId :: AllocatePushTransportResponse as _ ) , crate :: dm :: Access :: WM . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: DeallocatePushTransport as _ , None , crate :: dm :: Access :: WM . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: ModifyPushTransport as _ , None , crate :: dm :: Access :: WM . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: SetTransportStatus as _ , None , crate :: dm :: Access :: WM . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: ManuallyTriggerTransport as _ , None , crate :: dm :: Access :: WO . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: FindTransport as _ , Some (CommandResponseId :: FindTransportResponse as _ ) , crate :: dm :: Access :: WO . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , ] , & [crate :: dm :: Event :: new (EventId :: PushTransportBegin as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: PushTransportEnd as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn supported_formats < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SupportedFormatStructArrayBuilder < P > , SupportedFormatStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn current_connections < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < TransportConfigurationStructArrayBuilder < P > , TransportConfigurationStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn handle_allocate_push_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AllocatePushTransportRequest < '_ > , response : AllocatePushTransportResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_deallocate_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : DeallocatePushTransportRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_modify_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ModifyPushTransportRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_set_transport_status (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTransportStatusRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_manually_trigger_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ManuallyTriggerTransportRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_find_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : FindTransportRequest < '_ > , response : FindTransportResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
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
    fn supported_formats < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SupportedFormatStructArrayBuilder < P > , SupportedFormatStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: supported_formats (self , ctx , builder ) 
    }
    fn current_connections < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < TransportConfigurationStructArrayBuilder < P > , TransportConfigurationStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: current_connections (self , ctx , builder ) 
    }
    fn handle_allocate_push_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AllocatePushTransportRequest < '_ > , response : AllocatePushTransportResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_allocate_push_transport (self , ctx , request , response ) 
    }
    fn handle_deallocate_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : DeallocatePushTransportRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_deallocate_push_transport (self , ctx , request ) 
    }
    fn handle_modify_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ModifyPushTransportRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_modify_push_transport (self , ctx , request ) 
    }
    fn handle_set_transport_status (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTransportStatusRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_set_transport_status (self , ctx , request ) 
    }
    fn handle_manually_trigger_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ManuallyTriggerTransportRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_manually_trigger_transport (self , ctx , request ) 
    }
    fn handle_find_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : FindTransportRequest < '_ > , response : FindTransportResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_find_transport (self , ctx , request , response ) 
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
                    AttributeId :: SupportedFormats => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFormats , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. supported_formats (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFormats , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFormats , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentConnections => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentConnections , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_connections (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentConnections , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentConnections , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
            CommandId :: AllocatePushTransport => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AllocatePushTransport ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_allocate_push_transport (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AllocatePushTransport ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AllocatePushTransport ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: DeallocatePushTransport => {
                let cmd_data : DeallocatePushTransportRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_deallocate_push_transport (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: DeallocatePushTransport ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ModifyPushTransport => {
                let cmd_data : ModifyPushTransportRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_modify_push_transport (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ModifyPushTransport ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetTransportStatus => {
                let cmd_data : SetTransportStatusRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_transport_status (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTransportStatus ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ManuallyTriggerTransport => {
                let cmd_data : ManuallyTriggerTransportRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_manually_trigger_transport (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ManuallyTriggerTransport ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: FindTransport => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: FindTransport ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (7u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_find_transport (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: FindTransport ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: FindTransport ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 1365u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PushAvStreamTransport" , 1365u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PushAvStreamTransport" , 1365u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `PushAvStreamTransport` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct PushAvStreamTransportCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > PushAvStreamTransportCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn allocate_push_transport (self , endpoint : crate :: dm :: EndptId , ) -> Result < AllocatePushTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1365 , 0 ) ? . data_builder () 
    }
    pub fn deallocate_push_transport (self , endpoint : crate :: dm :: EndptId , ) -> Result < DeallocatePushTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1365 , 2 ) ? . data_builder () 
    }
    pub fn modify_push_transport (self , endpoint : crate :: dm :: EndptId , ) -> Result < ModifyPushTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1365 , 3 ) ? . data_builder () 
    }
    pub fn set_transport_status (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetTransportStatusRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1365 , 4 ) ? . data_builder () 
    }
    pub fn manually_trigger_transport (self , endpoint : crate :: dm :: EndptId , ) -> Result < ManuallyTriggerTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1365 , 5 ) ? . data_builder () 
    }
    pub fn find_transport (self , endpoint : crate :: dm :: EndptId , ) -> Result < FindTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1365 , 6 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `PushAvStreamTransport` cluster's commands. `use` this trait to call `.push_av_stream_transport_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`PushAvStreamTransportCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait PushAvStreamTransportCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn push_av_stream_transport_inv (self ) -> PushAvStreamTransportCmdRequestsView < P > ;
    
}
impl < P > PushAvStreamTransportCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn push_av_stream_transport_inv (self ) -> PushAvStreamTransportCmdRequestsView < P > {
        PushAvStreamTransportCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `PushAvStreamTransport` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PushAvStreamTransportAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > PushAvStreamTransportAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn supported_formats (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1365 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_connections (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1365 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1365 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1365 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1365 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1365 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1365 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `PushAvStreamTransport` cluster's attribute reads. `use` this trait to call `.push_av_stream_transport_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`PushAvStreamTransportAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait PushAvStreamTransportAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn push_av_stream_transport_read (self ) -> PushAvStreamTransportAttrReadsView < P > ;
    
}
impl < P > PushAvStreamTransportAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn push_av_stream_transport_read (self ) -> PushAvStreamTransportAttrReadsView < P > {
        PushAvStreamTransportAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `PushAvStreamTransport` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PushAvStreamTransportAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > PushAvStreamTransportAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `PushAvStreamTransport` cluster's attribute writes. `use` this trait to call `.push_av_stream_transport_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`PushAvStreamTransportAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait PushAvStreamTransportAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn push_av_stream_transport_write (self ) -> PushAvStreamTransportAttrWritesView < P > ;
    
}
impl < P > PushAvStreamTransportAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn push_av_stream_transport_write (self ) -> PushAvStreamTransportAttrWritesView < P > {
        PushAvStreamTransportAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `PushAvStreamTransport` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct PushAvStreamTransportCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > PushAvStreamTransportCmdResponsesView < 'a , 'r > {
    pub fn allocate_push_transport (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AllocatePushTransportResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < AllocatePushTransportResponse < 'a >> (1365 , 0 ) 
    }
    pub fn deallocate_push_transport (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1365 , 2 ) 
    }
    pub fn modify_push_transport (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1365 , 3 ) 
    }
    pub fn set_transport_status (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1365 , 4 ) 
    }
    pub fn manually_trigger_transport (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1365 , 5 ) 
    }
    pub fn find_transport (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < FindTransportResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < FindTransportResponse < 'a >> (1365 , 6 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `PushAvStreamTransport`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.push_av_stream_transport_inv_resp()` on an `InvokeResp`; the returned [`PushAvStreamTransportCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait PushAvStreamTransportCmdResponses < 'a > {
    fn push_av_stream_transport_inv_resp (& self ) -> PushAvStreamTransportCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > PushAvStreamTransportCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn push_av_stream_transport_inv_resp (& self ) -> PushAvStreamTransportCmdResponsesView < 'a , '_ > {
        PushAvStreamTransportCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `PushAvStreamTransport` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PushAvStreamTransportAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > PushAvStreamTransportAttrResponsesView < 'a , 'r > {
    pub fn supported_formats (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , SupportedFormatStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , SupportedFormatStruct < 'a > > > (1365 , 0 ) 
    }
    pub fn current_connections (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , TransportConfigurationStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , TransportConfigurationStruct < 'a > > > (1365 , 1 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1365 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1365 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (1365 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (1365 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (1365 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `PushAvStreamTransport`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.push_av_stream_transport_read_resp()` on a `ReportDataResp`; the returned [`PushAvStreamTransportAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait PushAvStreamTransportAttrResponses < 'a > {
    fn push_av_stream_transport_read_resp (& self ) -> PushAvStreamTransportAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > PushAvStreamTransportAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn push_av_stream_transport_read_resp (& self ) -> PushAvStreamTransportAttrResponsesView < 'a , '_ > {
        PushAvStreamTransportAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `PushAvStreamTransport` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct PushAvStreamTransportWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > PushAvStreamTransportWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `PushAvStreamTransport`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.push_av_stream_transport_write_resp()` on a `WriteResp`; the returned [`PushAvStreamTransportWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait PushAvStreamTransportWriteResponses < 'a > {
    fn push_av_stream_transport_write_resp (& self ) -> PushAvStreamTransportWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > PushAvStreamTransportWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn push_av_stream_transport_write_resp (& self ) -> PushAvStreamTransportWriteResponsesView < 'a , '_ > {
        PushAvStreamTransportWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `AllocatePushTransportResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`AllocatePushTransportResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct AllocatePushTransportResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > AllocatePushTransportResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < AllocatePushTransportResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `FindTransportResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`FindTransportResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct FindTransportResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > FindTransportResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < FindTransportResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `PushAvStreamTransport` cluster's client operations. Returned by [`PushAvStreamTransportClient::push_av_stream_transport`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct PushAvStreamTransportClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > PushAvStreamTransportClientView < T > {
    pub async fn allocate_push_transport < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < AllocatePushTransportResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AllocatePushTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: PushAvStreamTransportCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . push_av_stream_transport_inv () ;
            let req_builder = view . allocate_push_transport (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (AllocatePushTransportResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn deallocate_push_transport < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (DeallocatePushTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: PushAvStreamTransportCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . push_av_stream_transport_inv () ;
            let req_builder = view . deallocate_push_transport (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn modify_push_transport < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (ModifyPushTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: PushAvStreamTransportCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . push_av_stream_transport_inv () ;
            let req_builder = view . modify_push_transport (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn set_transport_status < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SetTransportStatusRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: PushAvStreamTransportCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . push_av_stream_transport_inv () ;
            let req_builder = view . set_transport_status (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn manually_trigger_transport < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (ManuallyTriggerTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: PushAvStreamTransportCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . push_av_stream_transport_inv () ;
            let req_builder = view . manually_trigger_transport (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn find_transport < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < FindTransportResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (FindTransportRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: PushAvStreamTransportCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . push_av_stream_transport_inv () ;
            let req_builder = view . find_transport (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (FindTransportResponseHandle {
            chunk 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn supported_formats_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , SupportedFormatStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PushAvStreamTransportAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . push_av_stream_transport_read () . supported_formats (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn current_connections_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , TransportConfigurationStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: PushAvStreamTransportAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . push_av_stream_transport_read () . current_connections (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: PushAvStreamTransportAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . push_av_stream_transport_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: PushAvStreamTransportAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . push_av_stream_transport_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: PushAvStreamTransportAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . push_av_stream_transport_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: PushAvStreamTransportAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . push_av_stream_transport_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: PushAvStreamTransportAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . push_av_stream_transport_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `PushAvStreamTransport` cluster. `use` this trait to call `.push_av_stream_transport()` on any [`crate::im::ImClient`], such as an exchange; the returned [`PushAvStreamTransportClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `PushAvStreamTransportClientView` keeps each cluster's API surface narrow: at the call site `exchange.push_av_stream_transport().` shows only this cluster's methods in IDE completion." ] pub trait PushAvStreamTransportClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn push_av_stream_transport (self ) -> PushAvStreamTransportClientView < Self > {
        PushAvStreamTransportClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > PushAvStreamTransportClient for T {
    
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
    fn supported_formats < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SupportedFormatStructArrayBuilder < P > , SupportedFormatStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn current_connections < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < TransportConfigurationStructArrayBuilder < P > , TransportConfigurationStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_allocate_push_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AllocatePushTransportRequest < '_ > , response : AllocatePushTransportResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_deallocate_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : DeallocatePushTransportRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_modify_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ModifyPushTransportRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_transport_status (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTransportStatusRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_manually_trigger_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ManuallyTriggerTransportRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_find_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : FindTransportRequest < '_ > , response : FindTransportResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn supported_formats < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SupportedFormatStructArrayBuilder < P > , SupportedFormatStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: supported_formats (self , ctx , builder ) 
    }
    fn current_connections < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < TransportConfigurationStructArrayBuilder < P > , TransportConfigurationStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: current_connections (self , ctx , builder ) 
    }
    fn handle_allocate_push_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AllocatePushTransportRequest < '_ > , response : AllocatePushTransportResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_allocate_push_transport (self , ctx , request , response ) 
    }
    fn handle_deallocate_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : DeallocatePushTransportRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_deallocate_push_transport (self , ctx , request ) 
    }
    fn handle_modify_push_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ModifyPushTransportRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_modify_push_transport (self , ctx , request ) 
    }
    fn handle_set_transport_status (& self , ctx : impl crate :: dm :: InvokeContext , request : SetTransportStatusRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_set_transport_status (self , ctx , request ) 
    }
    fn handle_manually_trigger_transport (& self , ctx : impl crate :: dm :: InvokeContext , request : ManuallyTriggerTransportRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_manually_trigger_transport (self , ctx , request ) 
    }
    fn handle_find_transport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : FindTransportRequest < '_ > , response : FindTransportResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_find_transport (self , ctx , request , response ) 
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
                    AttributeId :: SupportedFormats => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFormats , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. supported_formats (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFormats , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFormats , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentConnections => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentConnections , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_connections (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentConnections , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentConnections , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
            CommandId :: AllocatePushTransport => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AllocatePushTransport ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_allocate_push_transport (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AllocatePushTransport ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AllocatePushTransport ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: DeallocatePushTransport => {
                let cmd_data : DeallocatePushTransportRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_deallocate_push_transport (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: DeallocatePushTransport ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ModifyPushTransport => {
                let cmd_data : ModifyPushTransportRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_modify_push_transport (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ModifyPushTransport ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetTransportStatus => {
                let cmd_data : SetTransportStatusRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_transport_status (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetTransportStatus ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ManuallyTriggerTransport => {
                let cmd_data : ManuallyTriggerTransportRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_manually_trigger_transport (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ManuallyTriggerTransport ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: FindTransport => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: FindTransport ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (7u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_find_transport (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: FindTransport ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: FindTransport ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 1365u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PushAvStreamTransport" , 1365u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "PushAvStreamTransport" , 1365u32 , self . 0.2 ) 
    }
    
}
