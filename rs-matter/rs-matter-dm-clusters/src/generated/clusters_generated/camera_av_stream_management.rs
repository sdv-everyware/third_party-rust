# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const AUDIO = 1 ;
        const VIDEO = 2 ;
        const SNAPSHOT = 4 ;
        const PRIVACY = 8 ;
        const SPEAKER = 16 ;
        const IMAGE_CONTROL = 32 ;
        const WATERMARK = 64 ;
        const ON_SCREEN_DISPLAY = 128 ;
        const LOCAL_STORAGE = 256 ;
        const HIGH_DYNAMIC_RANGE = 512 ;
        const NIGHT_VISION = 1024 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const AUDIO = 1 ;
        const VIDEO = 2 ;
        const SNAPSHOT = 4 ;
        const PRIVACY = 8 ;
        const SPEAKER = 16 ;
        const IMAGE_CONTROL = 32 ;
        const WATERMARK = 64 ;
        const ON_SCREEN_DISPLAY = 128 ;
        const LOCAL_STORAGE = 256 ;
        const HIGH_DYNAMIC_RANGE = 512 ;
        const NIGHT_VISION = 1024 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AudioCodecEnum {
    # [enumval (0 ) ] OPUS = 0 , # [enumval (1 ) ] AACLC = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ImageCodecEnum {
    # [enumval (0 ) ] JPEG = 0 , # [enumval (1 ) ] HEIC = 1 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TriStateAutoEnum {
    # [enumval (0 ) ] Off = 0 , # [enumval (1 ) ] On = 1 , # [enumval (2 ) ] Auto = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum TwoWayTalkSupportTypeEnum {
    # [enumval (0 ) ] NotSupported = 0 , # [enumval (1 ) ] HalfDuplex = 1 , # [enumval (2 ) ] FullDuplex = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoCodecEnum {
    # [enumval (0 ) ] H264 = 0 , # [enumval (1 ) ] HEVC = 1 , # [enumval (2 ) ] VVC = 2 , # [enumval (3 ) ] AV1 = 3 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoResolutionStructTag {
    Width = 0 , Height = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoStreamStructTag {
    VideoStreamId = 0 , StreamUsage = 1 , VideoCodec = 2 , MinFrameRate = 3 , MaxFrameRate = 4 , MinResolution = 5 , MaxResolution = 6 , MinBitRate = 7 , MaxBitRate = 8 , KeyFrameInterval = 9 , WatermarkEnabled = 10 , OsdEnabled = 11 , ReferenceCount = 12 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SnapshotStreamStructTag {
    SnapshotStreamId = 0 , ImageCodec = 1 , FrameRate = 2 , MinResolution = 3 , MaxResolution = 4 , Quality = 5 , ReferenceCount = 6 , EncodedPixels = 7 , HardwareEncoder = 8 , WatermarkEnabled = 9 , OsdEnabled = 10 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SnapshotCapabilitiesStructTag {
    Resolution = 0 , MaxFrameRate = 1 , ImageCodec = 2 , RequiresEncodedPixels = 3 , RequiresHardwareEncoder = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RateDistortionTradeOffPointsStructTag {
    Codec = 0 , Resolution = 1 , MinBitRate = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AudioCapabilitiesStructTag {
    MaxNumberOfChannels = 0 , SupportedCodecs = 1 , SupportedSampleRates = 2 , SupportedBitDepths = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AudioStreamStructTag {
    AudioStreamId = 0 , StreamUsage = 1 , AudioCodec = 2 , ChannelCount = 3 , SampleRate = 4 , BitRate = 5 , BitDepth = 6 , ReferenceCount = 7 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoSensorParamsStructTag {
    SensorWidth = 0 , SensorHeight = 1 , MaxFps = 2 , MaxHdrfps = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AudioStreamAllocateRequestTag {
    StreamUsage = 0 , AudioCodec = 1 , ChannelCount = 2 , SampleRate = 3 , BitRate = 4 , BitDepth = 5 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AudioStreamAllocateResponseTag {
    AudioStreamId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AudioStreamDeallocateRequestTag {
    AudioStreamId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoStreamAllocateRequestTag {
    StreamUsage = 0 , VideoCodec = 1 , MinFrameRate = 2 , MaxFrameRate = 3 , MinResolution = 4 , MaxResolution = 5 , MinBitRate = 6 , MaxBitRate = 7 , KeyFrameInterval = 8 , WatermarkEnabled = 9 , OsdEnabled = 10 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoStreamAllocateResponseTag {
    VideoStreamId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoStreamModifyRequestTag {
    VideoStreamId = 0 , WatermarkEnabled = 1 , OsdEnabled = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VideoStreamDeallocateRequestTag {
    VideoStreamId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SnapshotStreamAllocateRequestTag {
    ImageCodec = 0 , MaxFrameRate = 1 , MinResolution = 2 , MaxResolution = 3 , Quality = 4 , WatermarkEnabled = 5 , OsdEnabled = 6 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SnapshotStreamAllocateResponseTag {
    SnapshotStreamId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SnapshotStreamModifyRequestTag {
    SnapshotStreamId = 0 , WatermarkEnabled = 1 , OsdEnabled = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SnapshotStreamDeallocateRequestTag {
    SnapshotStreamId = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetStreamPrioritiesRequestTag {
    StreamPriorities = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CaptureSnapshotRequestTag {
    SnapshotStreamId = 0 , RequestedResolution = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CaptureSnapshotResponseTag {
    Data = 0 , ImageCodec = 1 , Resolution = 2 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VideoResolutionStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VideoResolutionStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn width (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn height (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VideoResolutionStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VideoResolutionStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VideoResolutionStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VideoResolutionStruct" ) ? ;
        match self . width () {
            Ok (value ) => write ! (f , "{}: {:?}," , "width" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "width" , e . code () ) ? , 
        }
        match self . height () {
            Ok (value ) => write ! (f , "{}: {:?}," , "height" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "height" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoResolutionStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoResolutionStruct" ) ;
        match self . width () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "width" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "width" , e . code () ) , 
        }
        match self . height () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "height" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "height" , e . code () ) , 
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
    pub fn video_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn stream_usage (& self ) -> Result < StreamUsageEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn video_codec (& self ) -> Result < VideoCodecEnum , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn min_frame_rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn max_frame_rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn min_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn max_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn min_bit_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn max_bit_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (8 ) 
    }
    pub fn key_frame_interval (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (9 ) 
    }
    pub fn watermark_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (10 ) 
    }
    pub fn osd_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (11 ) 
    }
    pub fn reference_count (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (12 ) 
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
        match self . video_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        match self . stream_usage () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_usage" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) ? , 
        }
        match self . video_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_codec" , e . code () ) ? , 
        }
        match self . min_frame_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_frame_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_frame_rate" , e . code () ) ? , 
        }
        match self . max_frame_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_frame_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) ? , 
        }
        match self . min_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) ? , 
        }
        match self . max_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) ? , 
        }
        match self . min_bit_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_bit_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_bit_rate" , e . code () ) ? , 
        }
        match self . max_bit_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_bit_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_bit_rate" , e . code () ) ? , 
        }
        match self . key_frame_interval () {
            Ok (value ) => write ! (f , "{}: {:?}," , "key_frame_interval" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "key_frame_interval" , e . code () ) ? , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "watermark_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) ? , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "osd_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) ? , 
        }
        match self . reference_count () {
            Ok (value ) => write ! (f , "{}: {:?}," , "reference_count" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "reference_count" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoStreamStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoStreamStruct" ) ;
        match self . video_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        match self . stream_usage () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_usage" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) , 
        }
        match self . video_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_codec" , e . code () ) , 
        }
        match self . min_frame_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_frame_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_frame_rate" , e . code () ) , 
        }
        match self . max_frame_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_frame_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) , 
        }
        match self . min_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) , 
        }
        match self . max_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) , 
        }
        match self . min_bit_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_bit_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_bit_rate" , e . code () ) , 
        }
        match self . max_bit_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_bit_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_bit_rate" , e . code () ) , 
        }
        match self . key_frame_interval () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "key_frame_interval" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "key_frame_interval" , e . code () ) , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "watermark_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "osd_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) , 
        }
        match self . reference_count () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "reference_count" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "reference_count" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SnapshotStreamStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SnapshotStreamStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn snapshot_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn image_codec (& self ) -> Result < ImageCodecEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn frame_rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn min_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn max_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn quality (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn reference_count (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn encoded_pixels (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn hardware_encoder (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (8 ) 
    }
    pub fn watermark_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (9 ) 
    }
    pub fn osd_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (10 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SnapshotStreamStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SnapshotStreamStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SnapshotStreamStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SnapshotStreamStruct" ) ? ;
        match self . snapshot_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) ? , 
        }
        match self . image_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "image_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) ? , 
        }
        match self . frame_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "frame_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "frame_rate" , e . code () ) ? , 
        }
        match self . min_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) ? , 
        }
        match self . max_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) ? , 
        }
        match self . quality () {
            Ok (value ) => write ! (f , "{}: {:?}," , "quality" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "quality" , e . code () ) ? , 
        }
        match self . reference_count () {
            Ok (value ) => write ! (f , "{}: {:?}," , "reference_count" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "reference_count" , e . code () ) ? , 
        }
        match self . encoded_pixels () {
            Ok (value ) => write ! (f , "{}: {:?}," , "encoded_pixels" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "encoded_pixels" , e . code () ) ? , 
        }
        match self . hardware_encoder () {
            Ok (value ) => write ! (f , "{}: {:?}," , "hardware_encoder" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "hardware_encoder" , e . code () ) ? , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "watermark_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) ? , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "osd_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SnapshotStreamStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SnapshotStreamStruct" ) ;
        match self . snapshot_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) , 
        }
        match self . image_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "image_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) , 
        }
        match self . frame_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "frame_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "frame_rate" , e . code () ) , 
        }
        match self . min_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) , 
        }
        match self . max_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) , 
        }
        match self . quality () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "quality" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "quality" , e . code () ) , 
        }
        match self . reference_count () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "reference_count" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "reference_count" , e . code () ) , 
        }
        match self . encoded_pixels () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "encoded_pixels" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "encoded_pixels" , e . code () ) , 
        }
        match self . hardware_encoder () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "hardware_encoder" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "hardware_encoder" , e . code () ) , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "watermark_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "osd_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SnapshotCapabilitiesStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SnapshotCapabilitiesStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn max_frame_rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn image_codec (& self ) -> Result < ImageCodecEnum , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn requires_encoded_pixels (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn requires_hardware_encoder (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SnapshotCapabilitiesStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SnapshotCapabilitiesStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SnapshotCapabilitiesStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SnapshotCapabilitiesStruct" ) ? ;
        match self . resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "resolution" , e . code () ) ? , 
        }
        match self . max_frame_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_frame_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) ? , 
        }
        match self . image_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "image_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) ? , 
        }
        match self . requires_encoded_pixels () {
            Ok (value ) => write ! (f , "{}: {:?}," , "requires_encoded_pixels" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "requires_encoded_pixels" , e . code () ) ? , 
        }
        match self . requires_hardware_encoder () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "requires_hardware_encoder" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "requires_hardware_encoder" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "requires_hardware_encoder" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SnapshotCapabilitiesStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SnapshotCapabilitiesStruct" ) ;
        match self . resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "resolution" , e . code () ) , 
        }
        match self . max_frame_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_frame_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) , 
        }
        match self . image_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "image_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) , 
        }
        match self . requires_encoded_pixels () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "requires_encoded_pixels" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "requires_encoded_pixels" , e . code () ) , 
        }
        match self . requires_hardware_encoder () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "requires_hardware_encoder" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "requires_hardware_encoder" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "requires_hardware_encoder" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RateDistortionTradeOffPointsStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RateDistortionTradeOffPointsStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn codec (& self ) -> Result < VideoCodecEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn min_bit_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RateDistortionTradeOffPointsStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RateDistortionTradeOffPointsStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RateDistortionTradeOffPointsStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RateDistortionTradeOffPointsStruct" ) ? ;
        match self . codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "codec" , e . code () ) ? , 
        }
        match self . resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "resolution" , e . code () ) ? , 
        }
        match self . min_bit_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_bit_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_bit_rate" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RateDistortionTradeOffPointsStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RateDistortionTradeOffPointsStruct" ) ;
        match self . codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "codec" , e . code () ) , 
        }
        match self . resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "resolution" , e . code () ) , 
        }
        match self . min_bit_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_bit_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_bit_rate" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AudioCapabilitiesStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AudioCapabilitiesStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn max_number_of_channels (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn supported_codecs (& self ) -> Result < crate :: tlv :: TLVArray < '_ , AudioCodecEnum > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn supported_sample_rates (& self ) -> Result < crate :: tlv :: TLVArray < '_ , u32 > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn supported_bit_depths (& self ) -> Result < crate :: tlv :: TLVArray < '_ , u8 > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AudioCapabilitiesStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AudioCapabilitiesStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AudioCapabilitiesStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AudioCapabilitiesStruct" ) ? ;
        match self . max_number_of_channels () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_number_of_channels" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_number_of_channels" , e . code () ) ? , 
        }
        match self . supported_codecs () {
            Ok (value ) => write ! (f , "{}: {:?}," , "supported_codecs" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "supported_codecs" , e . code () ) ? , 
        }
        match self . supported_sample_rates () {
            Ok (value ) => write ! (f , "{}: {:?}," , "supported_sample_rates" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "supported_sample_rates" , e . code () ) ? , 
        }
        match self . supported_bit_depths () {
            Ok (value ) => write ! (f , "{}: {:?}," , "supported_bit_depths" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "supported_bit_depths" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AudioCapabilitiesStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AudioCapabilitiesStruct" ) ;
        match self . max_number_of_channels () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_number_of_channels" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_number_of_channels" , e . code () ) , 
        }
        match self . supported_codecs () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "supported_codecs" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "supported_codecs" , e . code () ) , 
        }
        match self . supported_sample_rates () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "supported_sample_rates" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "supported_sample_rates" , e . code () ) , 
        }
        match self . supported_bit_depths () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "supported_bit_depths" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "supported_bit_depths" , e . code () ) , 
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
    pub fn audio_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn stream_usage (& self ) -> Result < StreamUsageEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn audio_codec (& self ) -> Result < AudioCodecEnum , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn channel_count (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn sample_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn bit_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn bit_depth (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn reference_count (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (7 ) 
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
        match self . audio_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "audio_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        match self . stream_usage () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_usage" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) ? , 
        }
        match self . audio_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "audio_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_codec" , e . code () ) ? , 
        }
        match self . channel_count () {
            Ok (value ) => write ! (f , "{}: {:?}," , "channel_count" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel_count" , e . code () ) ? , 
        }
        match self . sample_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "sample_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sample_rate" , e . code () ) ? , 
        }
        match self . bit_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "bit_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "bit_rate" , e . code () ) ? , 
        }
        match self . bit_depth () {
            Ok (value ) => write ! (f , "{}: {:?}," , "bit_depth" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "bit_depth" , e . code () ) ? , 
        }
        match self . reference_count () {
            Ok (value ) => write ! (f , "{}: {:?}," , "reference_count" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "reference_count" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AudioStreamStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AudioStreamStruct" ) ;
        match self . audio_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "audio_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        match self . stream_usage () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_usage" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) , 
        }
        match self . audio_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "audio_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_codec" , e . code () ) , 
        }
        match self . channel_count () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "channel_count" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel_count" , e . code () ) , 
        }
        match self . sample_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "sample_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sample_rate" , e . code () ) , 
        }
        match self . bit_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "bit_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "bit_rate" , e . code () ) , 
        }
        match self . bit_depth () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "bit_depth" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "bit_depth" , e . code () ) , 
        }
        match self . reference_count () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "reference_count" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "reference_count" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VideoSensorParamsStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VideoSensorParamsStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn sensor_width (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn sensor_height (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn max_fps (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn max_hdrfps (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VideoSensorParamsStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VideoSensorParamsStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VideoSensorParamsStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VideoSensorParamsStruct" ) ? ;
        match self . sensor_width () {
            Ok (value ) => write ! (f , "{}: {:?}," , "sensor_width" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sensor_width" , e . code () ) ? , 
        }
        match self . sensor_height () {
            Ok (value ) => write ! (f , "{}: {:?}," , "sensor_height" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sensor_height" , e . code () ) ? , 
        }
        match self . max_fps () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_fps" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_fps" , e . code () ) ? , 
        }
        match self . max_hdrfps () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "max_hdrfps" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "max_hdrfps" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_hdrfps" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoSensorParamsStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoSensorParamsStruct" ) ;
        match self . sensor_width () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "sensor_width" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sensor_width" , e . code () ) , 
        }
        match self . sensor_height () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "sensor_height" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sensor_height" , e . code () ) , 
        }
        match self . max_fps () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_fps" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_fps" , e . code () ) , 
        }
        match self . max_hdrfps () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "max_hdrfps" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "max_hdrfps" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_hdrfps" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AudioStreamAllocateRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AudioStreamAllocateRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn stream_usage (& self ) -> Result < StreamUsageEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn audio_codec (& self ) -> Result < AudioCodecEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn channel_count (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn sample_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn bit_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn bit_depth (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AudioStreamAllocateRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AudioStreamAllocateRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AudioStreamAllocateRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AudioStreamAllocateRequest" ) ? ;
        match self . stream_usage () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_usage" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) ? , 
        }
        match self . audio_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "audio_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_codec" , e . code () ) ? , 
        }
        match self . channel_count () {
            Ok (value ) => write ! (f , "{}: {:?}," , "channel_count" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "channel_count" , e . code () ) ? , 
        }
        match self . sample_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "sample_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sample_rate" , e . code () ) ? , 
        }
        match self . bit_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "bit_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "bit_rate" , e . code () ) ? , 
        }
        match self . bit_depth () {
            Ok (value ) => write ! (f , "{}: {:?}," , "bit_depth" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "bit_depth" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AudioStreamAllocateRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AudioStreamAllocateRequest" ) ;
        match self . stream_usage () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_usage" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) , 
        }
        match self . audio_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "audio_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_codec" , e . code () ) , 
        }
        match self . channel_count () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "channel_count" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "channel_count" , e . code () ) , 
        }
        match self . sample_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "sample_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sample_rate" , e . code () ) , 
        }
        match self . bit_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "bit_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "bit_rate" , e . code () ) , 
        }
        match self . bit_depth () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "bit_depth" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "bit_depth" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AudioStreamAllocateResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AudioStreamAllocateResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn audio_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AudioStreamAllocateResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AudioStreamAllocateResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AudioStreamAllocateResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AudioStreamAllocateResponse" ) ? ;
        match self . audio_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "audio_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AudioStreamAllocateResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AudioStreamAllocateResponse" ) ;
        match self . audio_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "audio_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AudioStreamDeallocateRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AudioStreamDeallocateRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn audio_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AudioStreamDeallocateRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AudioStreamDeallocateRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AudioStreamDeallocateRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AudioStreamDeallocateRequest" ) ? ;
        match self . audio_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "audio_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AudioStreamDeallocateRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AudioStreamDeallocateRequest" ) ;
        match self . audio_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "audio_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VideoStreamAllocateRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VideoStreamAllocateRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn stream_usage (& self ) -> Result < StreamUsageEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn video_codec (& self ) -> Result < VideoCodecEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn min_frame_rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn max_frame_rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn min_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn max_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn min_bit_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (6 ) 
    }
    pub fn max_bit_rate (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (7 ) 
    }
    pub fn key_frame_interval (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (8 ) 
    }
    pub fn watermark_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (9 ) 
    }
    pub fn osd_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (10 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VideoStreamAllocateRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VideoStreamAllocateRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VideoStreamAllocateRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VideoStreamAllocateRequest" ) ? ;
        match self . stream_usage () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_usage" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) ? , 
        }
        match self . video_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_codec" , e . code () ) ? , 
        }
        match self . min_frame_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_frame_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_frame_rate" , e . code () ) ? , 
        }
        match self . max_frame_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_frame_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) ? , 
        }
        match self . min_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) ? , 
        }
        match self . max_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) ? , 
        }
        match self . min_bit_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_bit_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_bit_rate" , e . code () ) ? , 
        }
        match self . max_bit_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_bit_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_bit_rate" , e . code () ) ? , 
        }
        match self . key_frame_interval () {
            Ok (value ) => write ! (f , "{}: {:?}," , "key_frame_interval" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "key_frame_interval" , e . code () ) ? , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "watermark_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) ? , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "osd_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoStreamAllocateRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoStreamAllocateRequest" ) ;
        match self . stream_usage () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_usage" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) , 
        }
        match self . video_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_codec" , e . code () ) , 
        }
        match self . min_frame_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_frame_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_frame_rate" , e . code () ) , 
        }
        match self . max_frame_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_frame_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) , 
        }
        match self . min_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) , 
        }
        match self . max_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) , 
        }
        match self . min_bit_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_bit_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_bit_rate" , e . code () ) , 
        }
        match self . max_bit_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_bit_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_bit_rate" , e . code () ) , 
        }
        match self . key_frame_interval () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "key_frame_interval" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "key_frame_interval" , e . code () ) , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "watermark_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "osd_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VideoStreamAllocateResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VideoStreamAllocateResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn video_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VideoStreamAllocateResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VideoStreamAllocateResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VideoStreamAllocateResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VideoStreamAllocateResponse" ) ? ;
        match self . video_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoStreamAllocateResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoStreamAllocateResponse" ) ;
        match self . video_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VideoStreamModifyRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VideoStreamModifyRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn video_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn watermark_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn osd_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VideoStreamModifyRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VideoStreamModifyRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VideoStreamModifyRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VideoStreamModifyRequest" ) ? ;
        match self . video_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "watermark_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) ? , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "osd_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoStreamModifyRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoStreamModifyRequest" ) ;
        match self . video_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "watermark_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "osd_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VideoStreamDeallocateRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VideoStreamDeallocateRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn video_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VideoStreamDeallocateRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VideoStreamDeallocateRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VideoStreamDeallocateRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VideoStreamDeallocateRequest" ) ? ;
        match self . video_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "video_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VideoStreamDeallocateRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VideoStreamDeallocateRequest" ) ;
        match self . video_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "video_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SnapshotStreamAllocateRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SnapshotStreamAllocateRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn image_codec (& self ) -> Result < ImageCodecEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn max_frame_rate (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn min_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn max_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn quality (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn watermark_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn osd_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SnapshotStreamAllocateRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SnapshotStreamAllocateRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SnapshotStreamAllocateRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SnapshotStreamAllocateRequest" ) ? ;
        match self . image_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "image_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) ? , 
        }
        match self . max_frame_rate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_frame_rate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) ? , 
        }
        match self . min_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "min_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) ? , 
        }
        match self . max_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "max_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) ? , 
        }
        match self . quality () {
            Ok (value ) => write ! (f , "{}: {:?}," , "quality" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "quality" , e . code () ) ? , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "watermark_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) ? , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "osd_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SnapshotStreamAllocateRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SnapshotStreamAllocateRequest" ) ;
        match self . image_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "image_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) , 
        }
        match self . max_frame_rate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_frame_rate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_frame_rate" , e . code () ) , 
        }
        match self . min_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "min_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "min_resolution" , e . code () ) , 
        }
        match self . max_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "max_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "max_resolution" , e . code () ) , 
        }
        match self . quality () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "quality" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "quality" , e . code () ) , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "watermark_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "osd_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SnapshotStreamAllocateResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SnapshotStreamAllocateResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn snapshot_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SnapshotStreamAllocateResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SnapshotStreamAllocateResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SnapshotStreamAllocateResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SnapshotStreamAllocateResponse" ) ? ;
        match self . snapshot_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SnapshotStreamAllocateResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SnapshotStreamAllocateResponse" ) ;
        match self . snapshot_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SnapshotStreamModifyRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SnapshotStreamModifyRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn snapshot_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn watermark_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn osd_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SnapshotStreamModifyRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SnapshotStreamModifyRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SnapshotStreamModifyRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SnapshotStreamModifyRequest" ) ? ;
        match self . snapshot_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) ? , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "watermark_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) ? , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "osd_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SnapshotStreamModifyRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SnapshotStreamModifyRequest" ) ;
        match self . snapshot_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) , 
        }
        match self . watermark_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "watermark_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "watermark_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "watermark_enabled" , e . code () ) , 
        }
        match self . osd_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "osd_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "osd_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "osd_enabled" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SnapshotStreamDeallocateRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SnapshotStreamDeallocateRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn snapshot_stream_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SnapshotStreamDeallocateRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SnapshotStreamDeallocateRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SnapshotStreamDeallocateRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SnapshotStreamDeallocateRequest" ) ? ;
        match self . snapshot_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SnapshotStreamDeallocateRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SnapshotStreamDeallocateRequest" ) ;
        match self . snapshot_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetStreamPrioritiesRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetStreamPrioritiesRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn stream_priorities (& self ) -> Result < crate :: tlv :: TLVArray < '_ , StreamUsageEnum > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetStreamPrioritiesRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetStreamPrioritiesRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetStreamPrioritiesRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetStreamPrioritiesRequest" ) ? ;
        match self . stream_priorities () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_priorities" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_priorities" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetStreamPrioritiesRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetStreamPrioritiesRequest" ) ;
        match self . stream_priorities () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_priorities" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_priorities" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CaptureSnapshotRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CaptureSnapshotRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn snapshot_stream_id (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn requested_resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CaptureSnapshotRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CaptureSnapshotRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CaptureSnapshotRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CaptureSnapshotRequest" ) ? ;
        match self . snapshot_stream_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) ? , 
        }
        match self . requested_resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "requested_resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "requested_resolution" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CaptureSnapshotRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CaptureSnapshotRequest" ) ;
        match self . snapshot_stream_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "snapshot_stream_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "snapshot_stream_id" , e . code () ) , 
        }
        match self . requested_resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "requested_resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "requested_resolution" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CaptureSnapshotResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CaptureSnapshotResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn data (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn image_codec (& self ) -> Result < ImageCodecEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn resolution (& self ) -> Result < VideoResolutionStruct < '_ > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CaptureSnapshotResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CaptureSnapshotResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CaptureSnapshotResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CaptureSnapshotResponse" ) ? ;
        match self . data () {
            Ok (value ) => write ! (f , "{}: {:?}," , "data" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "data" , e . code () ) ? , 
        }
        match self . image_codec () {
            Ok (value ) => write ! (f , "{}: {:?}," , "image_codec" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) ? , 
        }
        match self . resolution () {
            Ok (value ) => write ! (f , "{}: {:?}," , "resolution" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "resolution" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CaptureSnapshotResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CaptureSnapshotResponse" ) ;
        match self . data () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "data" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "data" , e . code () ) , 
        }
        match self . image_codec () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "image_codec" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "image_codec" , e . code () ) , 
        }
        match self . resolution () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "resolution" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "resolution" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct VideoResolutionStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VideoResolutionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VideoResolutionStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn width (mut self , value : u16 ) -> Result < VideoResolutionStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "width" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoResolutionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoResolutionStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn height (mut self , value : u16 ) -> Result < VideoResolutionStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "height" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (VideoResolutionStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoResolutionStructBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VideoResolutionStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoResolutionStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VideoResolutionStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoResolutionStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VideoResolutionStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoResolutionStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoResolutionStructArrayBuilder < P > (P ) ;
impl < P > VideoResolutionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VideoResolutionStructBuilder < VideoResolutionStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoResolutionStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VideoResolutionStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoResolutionStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VideoResolutionStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoResolutionStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VideoResolutionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoResolutionStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
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
    pub fn video_stream_id (mut self , value : u16 ) -> Result < VideoStreamStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_usage (mut self , value : StreamUsageEnum ) -> Result < VideoStreamStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "streamUsage" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_codec (mut self , value : VideoCodecEnum ) -> Result < VideoStreamStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_frame_rate (mut self , value : u16 ) -> Result < VideoStreamStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minFrameRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_frame_rate (mut self , value : u16 ) -> Result < VideoStreamStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxFrameRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_resolution (self ) -> Result < VideoResolutionStructBuilder < VideoStreamStructBuilder < P , 6usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (5 ) , ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_resolution (self ) -> Result < VideoResolutionStructBuilder < VideoStreamStructBuilder < P , 7usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (6 ) , ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_bit_rate (mut self , value : u32 ) -> Result < VideoStreamStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minBitRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_bit_rate (mut self , value : u32 ) -> Result < VideoStreamStructBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxBitRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key_frame_interval (mut self , value : u16 ) -> Result < VideoStreamStructBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "keyFrameInterval" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn watermark_enabled (mut self , value : Option < bool > ) -> Result < VideoStreamStructBuilder < P , 11usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "watermarkEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 10 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 11 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn osd_enabled (mut self , value : Option < bool > ) -> Result < VideoStreamStructBuilder < P , 12usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "OSDEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 11 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 12 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn reference_count (mut self , value : u8 ) -> Result < VideoStreamStructBuilder < P , 13usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "referenceCount" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 12 , & value ) ? ;
        Ok (VideoStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamStructBuilder < P , 13usize > where P : crate :: tlv :: TLVBuilderParent , {
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
pub struct SnapshotStreamStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SnapshotStreamStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn snapshot_stream_id (mut self , value : u16 ) -> Result < SnapshotStreamStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "snapshotStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn image_codec (mut self , value : ImageCodecEnum ) -> Result < SnapshotStreamStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "imageCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn frame_rate (mut self , value : u16 ) -> Result < SnapshotStreamStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "frameRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_resolution (self ) -> Result < VideoResolutionStructBuilder < SnapshotStreamStructBuilder < P , 4usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_resolution (self ) -> Result < VideoResolutionStructBuilder < SnapshotStreamStructBuilder < P , 5usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (4 ) , ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn quality (mut self , value : u8 ) -> Result < SnapshotStreamStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "quality" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn reference_count (mut self , value : u8 ) -> Result < SnapshotStreamStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "referenceCount" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn encoded_pixels (mut self , value : bool ) -> Result < SnapshotStreamStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "encodedPixels" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn hardware_encoder (mut self , value : bool ) -> Result < SnapshotStreamStructBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "hardwareEncoder" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn watermark_enabled (mut self , value : Option < bool > ) -> Result < SnapshotStreamStructBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "watermarkEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn osd_enabled (mut self , value : Option < bool > ) -> Result < SnapshotStreamStructBuilder < P , 11usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "OSDEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 10 , & value ) ? ;
        Ok (SnapshotStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamStructBuilder < P , 11usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SnapshotStreamStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SnapshotStreamStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SnapshotStreamStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamStructArrayBuilder < P > (P ) ;
impl < P > SnapshotStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SnapshotStreamStructBuilder < SnapshotStreamStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SnapshotStreamStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SnapshotStreamStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SnapshotStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotCapabilitiesStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SnapshotCapabilitiesStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SnapshotCapabilitiesStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn resolution (self ) -> Result < VideoResolutionStructBuilder < SnapshotCapabilitiesStructBuilder < P , 1usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotCapabilitiesStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > SnapshotCapabilitiesStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_frame_rate (mut self , value : u16 ) -> Result < SnapshotCapabilitiesStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxFrameRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SnapshotCapabilitiesStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotCapabilitiesStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn image_codec (mut self , value : ImageCodecEnum ) -> Result < SnapshotCapabilitiesStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "imageCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SnapshotCapabilitiesStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotCapabilitiesStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn requires_encoded_pixels (mut self , value : bool ) -> Result < SnapshotCapabilitiesStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "requiresEncodedPixels" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (SnapshotCapabilitiesStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotCapabilitiesStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn requires_hardware_encoder (mut self , value : Option < bool > ) -> Result < SnapshotCapabilitiesStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "requiresHardwareEncoder" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (SnapshotCapabilitiesStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotCapabilitiesStructBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SnapshotCapabilitiesStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotCapabilitiesStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SnapshotCapabilitiesStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotCapabilitiesStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SnapshotCapabilitiesStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotCapabilitiesStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotCapabilitiesStructArrayBuilder < P > (P ) ;
impl < P > SnapshotCapabilitiesStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SnapshotCapabilitiesStructBuilder < SnapshotCapabilitiesStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotCapabilitiesStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SnapshotCapabilitiesStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotCapabilitiesStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SnapshotCapabilitiesStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotCapabilitiesStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SnapshotCapabilitiesStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotCapabilitiesStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RateDistortionTradeOffPointsStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RateDistortionTradeOffPointsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RateDistortionTradeOffPointsStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn codec (mut self , value : VideoCodecEnum ) -> Result < RateDistortionTradeOffPointsStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "codec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RateDistortionTradeOffPointsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RateDistortionTradeOffPointsStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn resolution (self ) -> Result < VideoResolutionStructBuilder < RateDistortionTradeOffPointsStructBuilder < P , 2usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RateDistortionTradeOffPointsStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > RateDistortionTradeOffPointsStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_bit_rate (mut self , value : u32 ) -> Result < RateDistortionTradeOffPointsStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minBitRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (RateDistortionTradeOffPointsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > RateDistortionTradeOffPointsStructBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RateDistortionTradeOffPointsStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RateDistortionTradeOffPointsStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RateDistortionTradeOffPointsStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RateDistortionTradeOffPointsStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RateDistortionTradeOffPointsStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RateDistortionTradeOffPointsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RateDistortionTradeOffPointsStructArrayBuilder < P > (P ) ;
impl < P > RateDistortionTradeOffPointsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RateDistortionTradeOffPointsStructBuilder < RateDistortionTradeOffPointsStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RateDistortionTradeOffPointsStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RateDistortionTradeOffPointsStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RateDistortionTradeOffPointsStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RateDistortionTradeOffPointsStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RateDistortionTradeOffPointsStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RateDistortionTradeOffPointsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RateDistortionTradeOffPointsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioCapabilitiesStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AudioCapabilitiesStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AudioCapabilitiesStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_number_of_channels (mut self , value : u8 ) -> Result < AudioCapabilitiesStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxNumberOfChannels" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AudioCapabilitiesStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioCapabilitiesStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn supported_codecs (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < AudioCapabilitiesStructBuilder < P , 2usize > , AudioCodecEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioCapabilitiesStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > AudioCapabilitiesStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn supported_sample_rates (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < AudioCapabilitiesStructBuilder < P , 3usize > , u32 > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioCapabilitiesStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > AudioCapabilitiesStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn supported_bit_depths (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < AudioCapabilitiesStructBuilder < P , 4usize > , u8 > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioCapabilitiesStructBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > AudioCapabilitiesStructBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AudioCapabilitiesStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioCapabilitiesStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AudioCapabilitiesStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioCapabilitiesStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AudioCapabilitiesStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioCapabilitiesStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioCapabilitiesStructArrayBuilder < P > (P ) ;
impl < P > AudioCapabilitiesStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AudioCapabilitiesStructBuilder < AudioCapabilitiesStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioCapabilitiesStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AudioCapabilitiesStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioCapabilitiesStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AudioCapabilitiesStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioCapabilitiesStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AudioCapabilitiesStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioCapabilitiesStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
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
    pub fn audio_stream_id (mut self , value : u16 ) -> Result < AudioStreamStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_usage (mut self , value : StreamUsageEnum ) -> Result < AudioStreamStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "streamUsage" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_codec (mut self , value : AudioCodecEnum ) -> Result < AudioStreamStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel_count (mut self , value : u8 ) -> Result < AudioStreamStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "channelCount" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sample_rate (mut self , value : u32 ) -> Result < AudioStreamStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sampleRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn bit_rate (mut self , value : u32 ) -> Result < AudioStreamStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "bitRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn bit_depth (mut self , value : u8 ) -> Result < AudioStreamStructBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "bitDepth" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn reference_count (mut self , value : u8 ) -> Result < AudioStreamStructBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "referenceCount" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (AudioStreamStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamStructBuilder < P , 8usize > where P : crate :: tlv :: TLVBuilderParent , {
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
pub struct VideoSensorParamsStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VideoSensorParamsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VideoSensorParamsStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sensor_width (mut self , value : u16 ) -> Result < VideoSensorParamsStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sensorWidth" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoSensorParamsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoSensorParamsStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sensor_height (mut self , value : u16 ) -> Result < VideoSensorParamsStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sensorHeight" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (VideoSensorParamsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoSensorParamsStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_fps (mut self , value : u16 ) -> Result < VideoSensorParamsStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxFPS" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (VideoSensorParamsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoSensorParamsStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_hdrfps (mut self , value : Option < u16 > ) -> Result < VideoSensorParamsStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxHDRFPS" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (VideoSensorParamsStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoSensorParamsStructBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VideoSensorParamsStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoSensorParamsStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VideoSensorParamsStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoSensorParamsStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VideoSensorParamsStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoSensorParamsStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoSensorParamsStructArrayBuilder < P > (P ) ;
impl < P > VideoSensorParamsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VideoSensorParamsStructBuilder < VideoSensorParamsStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoSensorParamsStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VideoSensorParamsStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoSensorParamsStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VideoSensorParamsStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoSensorParamsStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VideoSensorParamsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoSensorParamsStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamAllocateRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AudioStreamAllocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AudioStreamAllocateRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_usage (mut self , value : StreamUsageEnum ) -> Result < AudioStreamAllocateRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "streamUsage" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AudioStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamAllocateRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_codec (mut self , value : AudioCodecEnum ) -> Result < AudioStreamAllocateRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AudioStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamAllocateRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn channel_count (mut self , value : u8 ) -> Result < AudioStreamAllocateRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "channelCount" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AudioStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamAllocateRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sample_rate (mut self , value : u32 ) -> Result < AudioStreamAllocateRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sampleRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (AudioStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamAllocateRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn bit_rate (mut self , value : u32 ) -> Result < AudioStreamAllocateRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "bitRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (AudioStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamAllocateRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn bit_depth (mut self , value : u8 ) -> Result < AudioStreamAllocateRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "bitDepth" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (AudioStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamAllocateRequestBuilder < P , 6usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AudioStreamAllocateRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AudioStreamAllocateRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AudioStreamAllocateRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamAllocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamAllocateRequestArrayBuilder < P > (P ) ;
impl < P > AudioStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AudioStreamAllocateRequestBuilder < AudioStreamAllocateRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioStreamAllocateRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AudioStreamAllocateRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AudioStreamAllocateRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AudioStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamAllocateResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AudioStreamAllocateResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AudioStreamAllocateResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : u16 ) -> Result < AudioStreamAllocateResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AudioStreamAllocateResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamAllocateResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AudioStreamAllocateResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AudioStreamAllocateResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AudioStreamAllocateResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamAllocateResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamAllocateResponseArrayBuilder < P > (P ) ;
impl < P > AudioStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AudioStreamAllocateResponseBuilder < AudioStreamAllocateResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioStreamAllocateResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AudioStreamAllocateResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AudioStreamAllocateResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamAllocateResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AudioStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamDeallocateRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AudioStreamDeallocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AudioStreamDeallocateRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : u16 ) -> Result < AudioStreamDeallocateRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AudioStreamDeallocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AudioStreamDeallocateRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AudioStreamDeallocateRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamDeallocateRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AudioStreamDeallocateRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamDeallocateRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AudioStreamDeallocateRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamDeallocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AudioStreamDeallocateRequestArrayBuilder < P > (P ) ;
impl < P > AudioStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AudioStreamDeallocateRequestBuilder < AudioStreamDeallocateRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AudioStreamDeallocateRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AudioStreamDeallocateRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AudioStreamDeallocateRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AudioStreamDeallocateRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AudioStreamDeallocateRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AudioStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AudioStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamAllocateRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VideoStreamAllocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_usage (mut self , value : StreamUsageEnum ) -> Result < VideoStreamAllocateRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "streamUsage" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_codec (mut self , value : VideoCodecEnum ) -> Result < VideoStreamAllocateRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_frame_rate (mut self , value : u16 ) -> Result < VideoStreamAllocateRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minFrameRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_frame_rate (mut self , value : u16 ) -> Result < VideoStreamAllocateRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxFrameRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_resolution (self ) -> Result < VideoResolutionStructBuilder < VideoStreamAllocateRequestBuilder < P , 5usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamAllocateRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (4 ) , ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_resolution (self ) -> Result < VideoResolutionStructBuilder < VideoStreamAllocateRequestBuilder < P , 6usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamAllocateRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (5 ) , ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_bit_rate (mut self , value : u32 ) -> Result < VideoStreamAllocateRequestBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "minBitRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_bit_rate (mut self , value : u32 ) -> Result < VideoStreamAllocateRequestBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxBitRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn key_frame_interval (mut self , value : u16 ) -> Result < VideoStreamAllocateRequestBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "keyFrameInterval" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn watermark_enabled (mut self , value : Option < bool > ) -> Result < VideoStreamAllocateRequestBuilder < P , 10usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "watermarkEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 9 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn osd_enabled (mut self , value : Option < bool > ) -> Result < VideoStreamAllocateRequestBuilder < P , 11usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "OSDEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 10 , & value ) ? ;
        Ok (VideoStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateRequestBuilder < P , 11usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VideoStreamAllocateRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VideoStreamAllocateRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VideoStreamAllocateRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamAllocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamAllocateRequestArrayBuilder < P > (P ) ;
impl < P > VideoStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VideoStreamAllocateRequestBuilder < VideoStreamAllocateRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamAllocateRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VideoStreamAllocateRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VideoStreamAllocateRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VideoStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamAllocateResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VideoStreamAllocateResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VideoStreamAllocateResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : u16 ) -> Result < VideoStreamAllocateResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoStreamAllocateResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamAllocateResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VideoStreamAllocateResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VideoStreamAllocateResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VideoStreamAllocateResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamAllocateResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamAllocateResponseArrayBuilder < P > (P ) ;
impl < P > VideoStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VideoStreamAllocateResponseBuilder < VideoStreamAllocateResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamAllocateResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VideoStreamAllocateResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VideoStreamAllocateResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamAllocateResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VideoStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamModifyRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VideoStreamModifyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VideoStreamModifyRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : u16 ) -> Result < VideoStreamModifyRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoStreamModifyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamModifyRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn watermark_enabled (mut self , value : Option < bool > ) -> Result < VideoStreamModifyRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "watermarkEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (VideoStreamModifyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamModifyRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn osd_enabled (mut self , value : Option < bool > ) -> Result < VideoStreamModifyRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "OSDEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (VideoStreamModifyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamModifyRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VideoStreamModifyRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamModifyRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VideoStreamModifyRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamModifyRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VideoStreamModifyRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamModifyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamModifyRequestArrayBuilder < P > (P ) ;
impl < P > VideoStreamModifyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VideoStreamModifyRequestBuilder < VideoStreamModifyRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamModifyRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VideoStreamModifyRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamModifyRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VideoStreamModifyRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamModifyRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VideoStreamModifyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamModifyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamDeallocateRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VideoStreamDeallocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VideoStreamDeallocateRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : u16 ) -> Result < VideoStreamDeallocateRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VideoStreamDeallocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > VideoStreamDeallocateRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VideoStreamDeallocateRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamDeallocateRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VideoStreamDeallocateRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamDeallocateRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VideoStreamDeallocateRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamDeallocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VideoStreamDeallocateRequestArrayBuilder < P > (P ) ;
impl < P > VideoStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VideoStreamDeallocateRequestBuilder < VideoStreamDeallocateRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VideoStreamDeallocateRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VideoStreamDeallocateRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VideoStreamDeallocateRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VideoStreamDeallocateRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VideoStreamDeallocateRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VideoStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VideoStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamAllocateRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SnapshotStreamAllocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn image_codec (mut self , value : ImageCodecEnum ) -> Result < SnapshotStreamAllocateRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "imageCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SnapshotStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_frame_rate (mut self , value : u16 ) -> Result < SnapshotStreamAllocateRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "maxFrameRate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SnapshotStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn min_resolution (self ) -> Result < VideoResolutionStructBuilder < SnapshotStreamAllocateRequestBuilder < P , 3usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamAllocateRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_resolution (self ) -> Result < VideoResolutionStructBuilder < SnapshotStreamAllocateRequestBuilder < P , 4usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamAllocateRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (3 ) , ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn quality (mut self , value : u8 ) -> Result < SnapshotStreamAllocateRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "quality" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (SnapshotStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn watermark_enabled (mut self , value : Option < bool > ) -> Result < SnapshotStreamAllocateRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "watermarkEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (SnapshotStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn osd_enabled (mut self , value : Option < bool > ) -> Result < SnapshotStreamAllocateRequestBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "OSDEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (SnapshotStreamAllocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateRequestBuilder < P , 7usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SnapshotStreamAllocateRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SnapshotStreamAllocateRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SnapshotStreamAllocateRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamAllocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamAllocateRequestArrayBuilder < P > (P ) ;
impl < P > SnapshotStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SnapshotStreamAllocateRequestBuilder < SnapshotStreamAllocateRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamAllocateRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SnapshotStreamAllocateRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SnapshotStreamAllocateRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SnapshotStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamAllocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamAllocateResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SnapshotStreamAllocateResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn snapshot_stream_id (mut self , value : u16 ) -> Result < SnapshotStreamAllocateResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "snapshotStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SnapshotStreamAllocateResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamAllocateResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SnapshotStreamAllocateResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SnapshotStreamAllocateResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SnapshotStreamAllocateResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamAllocateResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamAllocateResponseArrayBuilder < P > (P ) ;
impl < P > SnapshotStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SnapshotStreamAllocateResponseBuilder < SnapshotStreamAllocateResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamAllocateResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SnapshotStreamAllocateResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SnapshotStreamAllocateResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamAllocateResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SnapshotStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamAllocateResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamModifyRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SnapshotStreamModifyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SnapshotStreamModifyRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn snapshot_stream_id (mut self , value : u16 ) -> Result < SnapshotStreamModifyRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "snapshotStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SnapshotStreamModifyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamModifyRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn watermark_enabled (mut self , value : Option < bool > ) -> Result < SnapshotStreamModifyRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "watermarkEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SnapshotStreamModifyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamModifyRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn osd_enabled (mut self , value : Option < bool > ) -> Result < SnapshotStreamModifyRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "OSDEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SnapshotStreamModifyRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamModifyRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SnapshotStreamModifyRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamModifyRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SnapshotStreamModifyRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamModifyRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SnapshotStreamModifyRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamModifyRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamModifyRequestArrayBuilder < P > (P ) ;
impl < P > SnapshotStreamModifyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SnapshotStreamModifyRequestBuilder < SnapshotStreamModifyRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamModifyRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SnapshotStreamModifyRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamModifyRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SnapshotStreamModifyRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamModifyRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SnapshotStreamModifyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamModifyRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamDeallocateRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SnapshotStreamDeallocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SnapshotStreamDeallocateRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn snapshot_stream_id (mut self , value : u16 ) -> Result < SnapshotStreamDeallocateRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "snapshotStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SnapshotStreamDeallocateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SnapshotStreamDeallocateRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SnapshotStreamDeallocateRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamDeallocateRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SnapshotStreamDeallocateRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamDeallocateRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SnapshotStreamDeallocateRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamDeallocateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SnapshotStreamDeallocateRequestArrayBuilder < P > (P ) ;
impl < P > SnapshotStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SnapshotStreamDeallocateRequestBuilder < SnapshotStreamDeallocateRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SnapshotStreamDeallocateRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SnapshotStreamDeallocateRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamDeallocateRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SnapshotStreamDeallocateRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SnapshotStreamDeallocateRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SnapshotStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SnapshotStreamDeallocateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetStreamPrioritiesRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetStreamPrioritiesRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetStreamPrioritiesRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_priorities (self ) -> Result < crate :: tlv :: ToTLVArrayBuilder < SetStreamPrioritiesRequestBuilder < P , 1usize > , StreamUsageEnum > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetStreamPrioritiesRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (0 ) , ) 
    }
    
}
impl < P > SetStreamPrioritiesRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetStreamPrioritiesRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetStreamPrioritiesRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetStreamPrioritiesRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetStreamPrioritiesRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetStreamPrioritiesRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetStreamPrioritiesRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetStreamPrioritiesRequestArrayBuilder < P > (P ) ;
impl < P > SetStreamPrioritiesRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetStreamPrioritiesRequestBuilder < SetStreamPrioritiesRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetStreamPrioritiesRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetStreamPrioritiesRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetStreamPrioritiesRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetStreamPrioritiesRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetStreamPrioritiesRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetStreamPrioritiesRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetStreamPrioritiesRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CaptureSnapshotRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CaptureSnapshotRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CaptureSnapshotRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn snapshot_stream_id (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < CaptureSnapshotRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "snapshotStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CaptureSnapshotRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > CaptureSnapshotRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn requested_resolution (self ) -> Result < VideoResolutionStructBuilder < CaptureSnapshotRequestBuilder < P , 2usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CaptureSnapshotRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > CaptureSnapshotRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CaptureSnapshotRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CaptureSnapshotRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CaptureSnapshotRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CaptureSnapshotRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CaptureSnapshotRequestArrayBuilder < P > (P ) ;
impl < P > CaptureSnapshotRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CaptureSnapshotRequestBuilder < CaptureSnapshotRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CaptureSnapshotRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CaptureSnapshotRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CaptureSnapshotRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CaptureSnapshotRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CaptureSnapshotRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CaptureSnapshotResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CaptureSnapshotResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CaptureSnapshotResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn data (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < CaptureSnapshotResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "data" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CaptureSnapshotResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > CaptureSnapshotResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn image_codec (mut self , value : ImageCodecEnum ) -> Result < CaptureSnapshotResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "imageCodec" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (CaptureSnapshotResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > CaptureSnapshotResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn resolution (self ) -> Result < VideoResolutionStructBuilder < CaptureSnapshotResponseBuilder < P , 3usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CaptureSnapshotResponseBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (2 ) , ) 
    }
    
}
impl < P > CaptureSnapshotResponseBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CaptureSnapshotResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CaptureSnapshotResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CaptureSnapshotResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CaptureSnapshotResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CaptureSnapshotResponseArrayBuilder < P > (P ) ;
impl < P > CaptureSnapshotResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CaptureSnapshotResponseBuilder < CaptureSnapshotResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CaptureSnapshotResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CaptureSnapshotResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CaptureSnapshotResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CaptureSnapshotResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CaptureSnapshotResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CaptureSnapshotResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    MaxConcurrentEncoders = 0 , MaxEncodedPixelRate = 1 , VideoSensorParams = 2 , NightVisionUsesInfrared = 3 , MinViewportResolution = 4 , RateDistortionTradeOffPoints = 5 , MaxContentBufferSize = 6 , MicrophoneCapabilities = 7 , SpeakerCapabilities = 8 , TwoWayTalkSupport = 9 , SnapshotCapabilities = 10 , MaxNetworkBandwidth = 11 , CurrentFrameRate = 12 , HDRModeEnabled = 13 , SupportedStreamUsages = 14 , AllocatedVideoStreams = 15 , AllocatedAudioStreams = 16 , AllocatedSnapshotStreams = 17 , StreamUsagePriorities = 18 , SoftRecordingPrivacyModeEnabled = 19 , SoftLivestreamPrivacyModeEnabled = 20 , HardPrivacyModeOn = 21 , NightVision = 22 , NightVisionIllum = 23 , Viewport = 24 , SpeakerMuted = 25 , SpeakerVolumeLevel = 26 , SpeakerMaxLevel = 27 , SpeakerMinLevel = 28 , MicrophoneMuted = 29 , MicrophoneVolumeLevel = 30 , MicrophoneMaxLevel = 31 , MicrophoneMinLevel = 32 , MicrophoneAGCEnabled = 33 , ImageRotation = 34 , ImageFlipHorizontal = 35 , ImageFlipVertical = 36 , LocalVideoRecordingEnabled = 37 , LocalSnapshotRecordingEnabled = 38 , StatusLightEnabled = 39 , StatusLightBrightness = 40 , ImageRotationDiscreteAngles = 41 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: MaxConcurrentEncoders => write ! (f , "{}(0x{:02x})" , "MaxConcurrentEncoders" , AttributeId :: MaxConcurrentEncoders as u32 ) ? , AttributeId :: MaxEncodedPixelRate => write ! (f , "{}(0x{:02x})" , "MaxEncodedPixelRate" , AttributeId :: MaxEncodedPixelRate as u32 ) ? , AttributeId :: VideoSensorParams => write ! (f , "{}(0x{:02x})" , "VideoSensorParams" , AttributeId :: VideoSensorParams as u32 ) ? , AttributeId :: NightVisionUsesInfrared => write ! (f , "{}(0x{:02x})" , "NightVisionUsesInfrared" , AttributeId :: NightVisionUsesInfrared as u32 ) ? , AttributeId :: MinViewportResolution => write ! (f , "{}(0x{:02x})" , "MinViewportResolution" , AttributeId :: MinViewportResolution as u32 ) ? , AttributeId :: RateDistortionTradeOffPoints => write ! (f , "{}(0x{:02x})" , "RateDistortionTradeOffPoints" , AttributeId :: RateDistortionTradeOffPoints as u32 ) ? , AttributeId :: MaxContentBufferSize => write ! (f , "{}(0x{:02x})" , "MaxContentBufferSize" , AttributeId :: MaxContentBufferSize as u32 ) ? , AttributeId :: MicrophoneCapabilities => write ! (f , "{}(0x{:02x})" , "MicrophoneCapabilities" , AttributeId :: MicrophoneCapabilities as u32 ) ? , AttributeId :: SpeakerCapabilities => write ! (f , "{}(0x{:02x})" , "SpeakerCapabilities" , AttributeId :: SpeakerCapabilities as u32 ) ? , AttributeId :: TwoWayTalkSupport => write ! (f , "{}(0x{:02x})" , "TwoWayTalkSupport" , AttributeId :: TwoWayTalkSupport as u32 ) ? , AttributeId :: SnapshotCapabilities => write ! (f , "{}(0x{:02x})" , "SnapshotCapabilities" , AttributeId :: SnapshotCapabilities as u32 ) ? , AttributeId :: MaxNetworkBandwidth => write ! (f , "{}(0x{:02x})" , "MaxNetworkBandwidth" , AttributeId :: MaxNetworkBandwidth as u32 ) ? , AttributeId :: CurrentFrameRate => write ! (f , "{}(0x{:02x})" , "CurrentFrameRate" , AttributeId :: CurrentFrameRate as u32 ) ? , AttributeId :: HDRModeEnabled => write ! (f , "{}(0x{:02x})" , "HDRModeEnabled" , AttributeId :: HDRModeEnabled as u32 ) ? , AttributeId :: SupportedStreamUsages => write ! (f , "{}(0x{:02x})" , "SupportedStreamUsages" , AttributeId :: SupportedStreamUsages as u32 ) ? , AttributeId :: AllocatedVideoStreams => write ! (f , "{}(0x{:02x})" , "AllocatedVideoStreams" , AttributeId :: AllocatedVideoStreams as u32 ) ? , AttributeId :: AllocatedAudioStreams => write ! (f , "{}(0x{:02x})" , "AllocatedAudioStreams" , AttributeId :: AllocatedAudioStreams as u32 ) ? , AttributeId :: AllocatedSnapshotStreams => write ! (f , "{}(0x{:02x})" , "AllocatedSnapshotStreams" , AttributeId :: AllocatedSnapshotStreams as u32 ) ? , AttributeId :: StreamUsagePriorities => write ! (f , "{}(0x{:02x})" , "StreamUsagePriorities" , AttributeId :: StreamUsagePriorities as u32 ) ? , AttributeId :: SoftRecordingPrivacyModeEnabled => write ! (f , "{}(0x{:02x})" , "SoftRecordingPrivacyModeEnabled" , AttributeId :: SoftRecordingPrivacyModeEnabled as u32 ) ? , AttributeId :: SoftLivestreamPrivacyModeEnabled => write ! (f , "{}(0x{:02x})" , "SoftLivestreamPrivacyModeEnabled" , AttributeId :: SoftLivestreamPrivacyModeEnabled as u32 ) ? , AttributeId :: HardPrivacyModeOn => write ! (f , "{}(0x{:02x})" , "HardPrivacyModeOn" , AttributeId :: HardPrivacyModeOn as u32 ) ? , AttributeId :: NightVision => write ! (f , "{}(0x{:02x})" , "NightVision" , AttributeId :: NightVision as u32 ) ? , AttributeId :: NightVisionIllum => write ! (f , "{}(0x{:02x})" , "NightVisionIllum" , AttributeId :: NightVisionIllum as u32 ) ? , AttributeId :: Viewport => write ! (f , "{}(0x{:02x})" , "Viewport" , AttributeId :: Viewport as u32 ) ? , AttributeId :: SpeakerMuted => write ! (f , "{}(0x{:02x})" , "SpeakerMuted" , AttributeId :: SpeakerMuted as u32 ) ? , AttributeId :: SpeakerVolumeLevel => write ! (f , "{}(0x{:02x})" , "SpeakerVolumeLevel" , AttributeId :: SpeakerVolumeLevel as u32 ) ? , AttributeId :: SpeakerMaxLevel => write ! (f , "{}(0x{:02x})" , "SpeakerMaxLevel" , AttributeId :: SpeakerMaxLevel as u32 ) ? , AttributeId :: SpeakerMinLevel => write ! (f , "{}(0x{:02x})" , "SpeakerMinLevel" , AttributeId :: SpeakerMinLevel as u32 ) ? , AttributeId :: MicrophoneMuted => write ! (f , "{}(0x{:02x})" , "MicrophoneMuted" , AttributeId :: MicrophoneMuted as u32 ) ? , AttributeId :: MicrophoneVolumeLevel => write ! (f , "{}(0x{:02x})" , "MicrophoneVolumeLevel" , AttributeId :: MicrophoneVolumeLevel as u32 ) ? , AttributeId :: MicrophoneMaxLevel => write ! (f , "{}(0x{:02x})" , "MicrophoneMaxLevel" , AttributeId :: MicrophoneMaxLevel as u32 ) ? , AttributeId :: MicrophoneMinLevel => write ! (f , "{}(0x{:02x})" , "MicrophoneMinLevel" , AttributeId :: MicrophoneMinLevel as u32 ) ? , AttributeId :: MicrophoneAGCEnabled => write ! (f , "{}(0x{:02x})" , "MicrophoneAGCEnabled" , AttributeId :: MicrophoneAGCEnabled as u32 ) ? , AttributeId :: ImageRotation => write ! (f , "{}(0x{:02x})" , "ImageRotation" , AttributeId :: ImageRotation as u32 ) ? , AttributeId :: ImageFlipHorizontal => write ! (f , "{}(0x{:02x})" , "ImageFlipHorizontal" , AttributeId :: ImageFlipHorizontal as u32 ) ? , AttributeId :: ImageFlipVertical => write ! (f , "{}(0x{:02x})" , "ImageFlipVertical" , AttributeId :: ImageFlipVertical as u32 ) ? , AttributeId :: LocalVideoRecordingEnabled => write ! (f , "{}(0x{:02x})" , "LocalVideoRecordingEnabled" , AttributeId :: LocalVideoRecordingEnabled as u32 ) ? , AttributeId :: LocalSnapshotRecordingEnabled => write ! (f , "{}(0x{:02x})" , "LocalSnapshotRecordingEnabled" , AttributeId :: LocalSnapshotRecordingEnabled as u32 ) ? , AttributeId :: StatusLightEnabled => write ! (f , "{}(0x{:02x})" , "StatusLightEnabled" , AttributeId :: StatusLightEnabled as u32 ) ? , AttributeId :: StatusLightBrightness => write ! (f , "{}(0x{:02x})" , "StatusLightBrightness" , AttributeId :: StatusLightBrightness as u32 ) ? , AttributeId :: ImageRotationDiscreteAngles => write ! (f , "{}(0x{:02x})" , "ImageRotationDiscreteAngles" , AttributeId :: ImageRotationDiscreteAngles as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: MaxConcurrentEncoders => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxConcurrentEncoders" , AttributeId :: MaxConcurrentEncoders as u32 ) , AttributeId :: MaxEncodedPixelRate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxEncodedPixelRate" , AttributeId :: MaxEncodedPixelRate as u32 ) , AttributeId :: VideoSensorParams => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "VideoSensorParams" , AttributeId :: VideoSensorParams as u32 ) , AttributeId :: NightVisionUsesInfrared => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NightVisionUsesInfrared" , AttributeId :: NightVisionUsesInfrared as u32 ) , AttributeId :: MinViewportResolution => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MinViewportResolution" , AttributeId :: MinViewportResolution as u32 ) , AttributeId :: RateDistortionTradeOffPoints => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RateDistortionTradeOffPoints" , AttributeId :: RateDistortionTradeOffPoints as u32 ) , AttributeId :: MaxContentBufferSize => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxContentBufferSize" , AttributeId :: MaxContentBufferSize as u32 ) , AttributeId :: MicrophoneCapabilities => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MicrophoneCapabilities" , AttributeId :: MicrophoneCapabilities as u32 ) , AttributeId :: SpeakerCapabilities => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeakerCapabilities" , AttributeId :: SpeakerCapabilities as u32 ) , AttributeId :: TwoWayTalkSupport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TwoWayTalkSupport" , AttributeId :: TwoWayTalkSupport as u32 ) , AttributeId :: SnapshotCapabilities => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SnapshotCapabilities" , AttributeId :: SnapshotCapabilities as u32 ) , AttributeId :: MaxNetworkBandwidth => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MaxNetworkBandwidth" , AttributeId :: MaxNetworkBandwidth as u32 ) , AttributeId :: CurrentFrameRate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentFrameRate" , AttributeId :: CurrentFrameRate as u32 ) , AttributeId :: HDRModeEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "HDRModeEnabled" , AttributeId :: HDRModeEnabled as u32 ) , AttributeId :: SupportedStreamUsages => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SupportedStreamUsages" , AttributeId :: SupportedStreamUsages as u32 ) , AttributeId :: AllocatedVideoStreams => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AllocatedVideoStreams" , AttributeId :: AllocatedVideoStreams as u32 ) , AttributeId :: AllocatedAudioStreams => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AllocatedAudioStreams" , AttributeId :: AllocatedAudioStreams as u32 ) , AttributeId :: AllocatedSnapshotStreams => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AllocatedSnapshotStreams" , AttributeId :: AllocatedSnapshotStreams as u32 ) , AttributeId :: StreamUsagePriorities => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StreamUsagePriorities" , AttributeId :: StreamUsagePriorities as u32 ) , AttributeId :: SoftRecordingPrivacyModeEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SoftRecordingPrivacyModeEnabled" , AttributeId :: SoftRecordingPrivacyModeEnabled as u32 ) , AttributeId :: SoftLivestreamPrivacyModeEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SoftLivestreamPrivacyModeEnabled" , AttributeId :: SoftLivestreamPrivacyModeEnabled as u32 ) , AttributeId :: HardPrivacyModeOn => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "HardPrivacyModeOn" , AttributeId :: HardPrivacyModeOn as u32 ) , AttributeId :: NightVision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NightVision" , AttributeId :: NightVision as u32 ) , AttributeId :: NightVisionIllum => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NightVisionIllum" , AttributeId :: NightVisionIllum as u32 ) , AttributeId :: Viewport => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Viewport" , AttributeId :: Viewport as u32 ) , AttributeId :: SpeakerMuted => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeakerMuted" , AttributeId :: SpeakerMuted as u32 ) , AttributeId :: SpeakerVolumeLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeakerVolumeLevel" , AttributeId :: SpeakerVolumeLevel as u32 ) , AttributeId :: SpeakerMaxLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeakerMaxLevel" , AttributeId :: SpeakerMaxLevel as u32 ) , AttributeId :: SpeakerMinLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SpeakerMinLevel" , AttributeId :: SpeakerMinLevel as u32 ) , AttributeId :: MicrophoneMuted => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MicrophoneMuted" , AttributeId :: MicrophoneMuted as u32 ) , AttributeId :: MicrophoneVolumeLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MicrophoneVolumeLevel" , AttributeId :: MicrophoneVolumeLevel as u32 ) , AttributeId :: MicrophoneMaxLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MicrophoneMaxLevel" , AttributeId :: MicrophoneMaxLevel as u32 ) , AttributeId :: MicrophoneMinLevel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MicrophoneMinLevel" , AttributeId :: MicrophoneMinLevel as u32 ) , AttributeId :: MicrophoneAGCEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "MicrophoneAGCEnabled" , AttributeId :: MicrophoneAGCEnabled as u32 ) , AttributeId :: ImageRotation => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ImageRotation" , AttributeId :: ImageRotation as u32 ) , AttributeId :: ImageFlipHorizontal => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ImageFlipHorizontal" , AttributeId :: ImageFlipHorizontal as u32 ) , AttributeId :: ImageFlipVertical => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ImageFlipVertical" , AttributeId :: ImageFlipVertical as u32 ) , AttributeId :: LocalVideoRecordingEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LocalVideoRecordingEnabled" , AttributeId :: LocalVideoRecordingEnabled as u32 ) , AttributeId :: LocalSnapshotRecordingEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "LocalSnapshotRecordingEnabled" , AttributeId :: LocalSnapshotRecordingEnabled as u32 ) , AttributeId :: StatusLightEnabled => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StatusLightEnabled" , AttributeId :: StatusLightEnabled as u32 ) , AttributeId :: StatusLightBrightness => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StatusLightBrightness" , AttributeId :: StatusLightBrightness as u32 ) , AttributeId :: ImageRotationDiscreteAngles => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ImageRotationDiscreteAngles" , AttributeId :: ImageRotationDiscreteAngles as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    AudioStreamAllocate = 0 , AudioStreamDeallocate = 2 , VideoStreamAllocate = 3 , VideoStreamModify = 5 , VideoStreamDeallocate = 6 , SnapshotStreamAllocate = 7 , SnapshotStreamModify = 9 , SnapshotStreamDeallocate = 10 , SetStreamPriorities = 11 , CaptureSnapshot = 12 
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
            CommandId :: AudioStreamAllocate => write ! (f , "{}(0x{:02x})" , "AudioStreamAllocate" , CommandId :: AudioStreamAllocate as u32 ) ? , CommandId :: AudioStreamDeallocate => write ! (f , "{}(0x{:02x})" , "AudioStreamDeallocate" , CommandId :: AudioStreamDeallocate as u32 ) ? , CommandId :: VideoStreamAllocate => write ! (f , "{}(0x{:02x})" , "VideoStreamAllocate" , CommandId :: VideoStreamAllocate as u32 ) ? , CommandId :: VideoStreamModify => write ! (f , "{}(0x{:02x})" , "VideoStreamModify" , CommandId :: VideoStreamModify as u32 ) ? , CommandId :: VideoStreamDeallocate => write ! (f , "{}(0x{:02x})" , "VideoStreamDeallocate" , CommandId :: VideoStreamDeallocate as u32 ) ? , CommandId :: SnapshotStreamAllocate => write ! (f , "{}(0x{:02x})" , "SnapshotStreamAllocate" , CommandId :: SnapshotStreamAllocate as u32 ) ? , CommandId :: SnapshotStreamModify => write ! (f , "{}(0x{:02x})" , "SnapshotStreamModify" , CommandId :: SnapshotStreamModify as u32 ) ? , CommandId :: SnapshotStreamDeallocate => write ! (f , "{}(0x{:02x})" , "SnapshotStreamDeallocate" , CommandId :: SnapshotStreamDeallocate as u32 ) ? , CommandId :: SetStreamPriorities => write ! (f , "{}(0x{:02x})" , "SetStreamPriorities" , CommandId :: SetStreamPriorities as u32 ) ? , CommandId :: CaptureSnapshot => write ! (f , "{}(0x{:02x})" , "CaptureSnapshot" , CommandId :: CaptureSnapshot as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: AudioStreamAllocate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AudioStreamAllocate" , CommandId :: AudioStreamAllocate as u32 ) , CommandId :: AudioStreamDeallocate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AudioStreamDeallocate" , CommandId :: AudioStreamDeallocate as u32 ) , CommandId :: VideoStreamAllocate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "VideoStreamAllocate" , CommandId :: VideoStreamAllocate as u32 ) , CommandId :: VideoStreamModify => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "VideoStreamModify" , CommandId :: VideoStreamModify as u32 ) , CommandId :: VideoStreamDeallocate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "VideoStreamDeallocate" , CommandId :: VideoStreamDeallocate as u32 ) , CommandId :: SnapshotStreamAllocate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SnapshotStreamAllocate" , CommandId :: SnapshotStreamAllocate as u32 ) , CommandId :: SnapshotStreamModify => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SnapshotStreamModify" , CommandId :: SnapshotStreamModify as u32 ) , CommandId :: SnapshotStreamDeallocate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SnapshotStreamDeallocate" , CommandId :: SnapshotStreamDeallocate as u32 ) , CommandId :: SetStreamPriorities => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetStreamPriorities" , CommandId :: SetStreamPriorities as u32 ) , CommandId :: CaptureSnapshot => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CaptureSnapshot" , CommandId :: CaptureSnapshot as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    AudioStreamAllocateResponse = 1 , VideoStreamAllocateResponse = 4 , SnapshotStreamAllocateResponse = 8 , CaptureSnapshotResponse = 13 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: AudioStreamAllocateResponse => write ! (f , "{}(0x{:02x})" , "AudioStreamAllocateResponse" , CommandResponseId :: AudioStreamAllocateResponse as u32 ) ? , CommandResponseId :: VideoStreamAllocateResponse => write ! (f , "{}(0x{:02x})" , "VideoStreamAllocateResponse" , CommandResponseId :: VideoStreamAllocateResponse as u32 ) ? , CommandResponseId :: SnapshotStreamAllocateResponse => write ! (f , "{}(0x{:02x})" , "SnapshotStreamAllocateResponse" , CommandResponseId :: SnapshotStreamAllocateResponse as u32 ) ? , CommandResponseId :: CaptureSnapshotResponse => write ! (f , "{}(0x{:02x})" , "CaptureSnapshotResponse" , CommandResponseId :: CaptureSnapshotResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: AudioStreamAllocateResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AudioStreamAllocateResponse" , CommandResponseId :: AudioStreamAllocateResponse as u32 ) , CommandResponseId :: VideoStreamAllocateResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "VideoStreamAllocateResponse" , CommandResponseId :: VideoStreamAllocateResponse as u32 ) , CommandResponseId :: SnapshotStreamAllocateResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SnapshotStreamAllocateResponse" , CommandResponseId :: SnapshotStreamAllocateResponse as u32 ) , CommandResponseId :: CaptureSnapshotResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CaptureSnapshotResponse" , CommandResponseId :: CaptureSnapshotResponse as u32 ) , 
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
use super :: globals :: * ;
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (1361 , 2 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: MaxConcurrentEncoders as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MaxEncodedPixelRate as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: VideoSensorParams as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NightVisionUsesInfrared as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MinViewportResolution as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: RateDistortionTradeOffPoints as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: MaxContentBufferSize as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: MicrophoneCapabilities as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SpeakerCapabilities as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: TwoWayTalkSupport as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SnapshotCapabilities as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: MaxNetworkBandwidth as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentFrameRate as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: HDRModeEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SupportedStreamUsages as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AllocatedVideoStreams as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: AllocatedAudioStreams as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: AllocatedSnapshotStreams as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: OA ) , crate :: dm :: Attribute :: new (AttributeId :: StreamUsagePriorities as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: SoftRecordingPrivacyModeEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SoftLivestreamPrivacyModeEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_OPERATE . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: HardPrivacyModeOn as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NightVision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: NightVisionIllum as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: Viewport as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SpeakerMuted as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SpeakerVolumeLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SpeakerMaxLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: SpeakerMinLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MicrophoneMuted as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MicrophoneVolumeLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MicrophoneMaxLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MicrophoneMinLevel as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: MicrophoneAGCEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ImageRotation as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ImageFlipHorizontal as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ImageFlipVertical as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: LocalVideoRecordingEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: LocalSnapshotRecordingEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: StatusLightEnabled as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: StatusLightBrightness as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: ImageRotationDiscreteAngles as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: O ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: AudioStreamAllocate as _ , Some (CommandResponseId :: AudioStreamAllocateResponse as _ ) , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: AudioStreamDeallocate as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: VideoStreamAllocate as _ , Some (CommandResponseId :: VideoStreamAllocateResponse as _ ) , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: VideoStreamModify as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: VideoStreamDeallocate as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: SnapshotStreamAllocate as _ , Some (CommandResponseId :: SnapshotStreamAllocateResponse as _ ) , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: SnapshotStreamModify as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: SnapshotStreamDeallocate as _ , None , crate :: dm :: Access :: WM , ) , crate :: dm :: Command :: new (CommandId :: SetStreamPriorities as _ , None , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: CaptureSnapshot as _ , Some (CommandResponseId :: CaptureSnapshotResponse as _ ) , crate :: dm :: Access :: WO , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn max_concurrent_encoders (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_encoded_pixel_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn video_sensor_params < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoSensorParamsStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn night_vision_uses_infrared (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn min_viewport_resolution < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoResolutionStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn rate_distortion_trade_off_points < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RateDistortionTradeOffPointsStructArrayBuilder < P > , RateDistortionTradeOffPointsStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_content_buffer_size (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > ;
    fn microphone_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn speaker_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn two_way_talk_support (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TwoWayTalkSupportTypeEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn snapshot_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotCapabilitiesStructArrayBuilder < P > , SnapshotCapabilitiesStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn max_network_bandwidth (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > ;
    fn current_frame_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn hdr_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn supported_stream_usages < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> Result < P , crate :: error :: Error > ;
    fn allocated_video_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < VideoStreamStructArrayBuilder < P > , VideoStreamStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn allocated_audio_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < AudioStreamStructArrayBuilder < P > , AudioStreamStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn allocated_snapshot_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotStreamStructArrayBuilder < P > , SnapshotStreamStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn stream_usage_priorities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> Result < P , crate :: error :: Error > ;
    fn soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn hard_privacy_mode_on (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn night_vision (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TriStateAutoEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn night_vision_illum (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TriStateAutoEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn viewport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : ViewportStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn speaker_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn speaker_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn speaker_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn speaker_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn microphone_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn microphone_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn microphone_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn microphone_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn microphone_agc_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn image_rotation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn image_flip_horizontal (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn image_flip_vertical (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn local_video_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn status_light_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn status_light_brightness (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ThreeLevelAutoEnum , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_hdr_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_night_vision (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_night_vision_illum (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_viewport (& self , ctx : impl crate :: dm :: WriteContext , value : ViewportStruct < '_ > ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_speaker_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_speaker_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_microphone_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_microphone_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_microphone_agc_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_image_rotation (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_image_flip_horizontal (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_image_flip_vertical (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_local_video_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_status_light_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_status_light_brightness (& self , ctx : impl crate :: dm :: WriteContext , value : ThreeLevelAutoEnum ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn set_image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) 
    }
    fn handle_audio_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamAllocateRequest < '_ > , response : AudioStreamAllocateResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_audio_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamDeallocateRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_video_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamAllocateRequest < '_ > , response : VideoStreamAllocateResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_video_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamModifyRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_video_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamDeallocateRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_snapshot_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamAllocateRequest < '_ > , response : SnapshotStreamAllocateResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_snapshot_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamModifyRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_snapshot_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamDeallocateRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_set_stream_priorities (& self , ctx : impl crate :: dm :: InvokeContext , request : SetStreamPrioritiesRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_capture_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CaptureSnapshotRequest < '_ > , response : CaptureSnapshotResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
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
    fn max_concurrent_encoders (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: max_concurrent_encoders (self , ctx ) 
    }
    fn max_encoded_pixel_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: max_encoded_pixel_rate (self , ctx ) 
    }
    fn video_sensor_params < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoSensorParamsStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: video_sensor_params (self , ctx , builder ) 
    }
    fn night_vision_uses_infrared (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: night_vision_uses_infrared (self , ctx ) 
    }
    fn min_viewport_resolution < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoResolutionStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: min_viewport_resolution (self , ctx , builder ) 
    }
    fn rate_distortion_trade_off_points < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RateDistortionTradeOffPointsStructArrayBuilder < P > , RateDistortionTradeOffPointsStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: rate_distortion_trade_off_points (self , ctx , builder ) 
    }
    fn max_content_buffer_size (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: max_content_buffer_size (self , ctx ) 
    }
    fn microphone_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: microphone_capabilities (self , ctx , builder ) 
    }
    fn speaker_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: speaker_capabilities (self , ctx , builder ) 
    }
    fn two_way_talk_support (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TwoWayTalkSupportTypeEnum , crate :: error :: Error > {
        T :: two_way_talk_support (self , ctx ) 
    }
    fn snapshot_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotCapabilitiesStructArrayBuilder < P > , SnapshotCapabilitiesStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: snapshot_capabilities (self , ctx , builder ) 
    }
    fn max_network_bandwidth (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u32 , crate :: error :: Error > {
        T :: max_network_bandwidth (self , ctx ) 
    }
    fn current_frame_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: current_frame_rate (self , ctx ) 
    }
    fn hdr_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: hdr_mode_enabled (self , ctx ) 
    }
    fn supported_stream_usages < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: supported_stream_usages (self , ctx , builder ) 
    }
    fn allocated_video_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < VideoStreamStructArrayBuilder < P > , VideoStreamStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: allocated_video_streams (self , ctx , builder ) 
    }
    fn allocated_audio_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < AudioStreamStructArrayBuilder < P > , AudioStreamStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: allocated_audio_streams (self , ctx , builder ) 
    }
    fn allocated_snapshot_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotStreamStructArrayBuilder < P > , SnapshotStreamStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: allocated_snapshot_streams (self , ctx , builder ) 
    }
    fn stream_usage_priorities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> Result < P , crate :: error :: Error > {
        T :: stream_usage_priorities (self , ctx , builder ) 
    }
    fn soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: soft_recording_privacy_mode_enabled (self , ctx ) 
    }
    fn soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: soft_livestream_privacy_mode_enabled (self , ctx ) 
    }
    fn hard_privacy_mode_on (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: hard_privacy_mode_on (self , ctx ) 
    }
    fn night_vision (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TriStateAutoEnum , crate :: error :: Error > {
        T :: night_vision (self , ctx ) 
    }
    fn night_vision_illum (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < TriStateAutoEnum , crate :: error :: Error > {
        T :: night_vision_illum (self , ctx ) 
    }
    fn viewport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : ViewportStructBuilder < P > ) -> Result < P , crate :: error :: Error > {
        T :: viewport (self , ctx , builder ) 
    }
    fn speaker_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: speaker_muted (self , ctx ) 
    }
    fn speaker_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: speaker_volume_level (self , ctx ) 
    }
    fn speaker_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: speaker_max_level (self , ctx ) 
    }
    fn speaker_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: speaker_min_level (self , ctx ) 
    }
    fn microphone_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: microphone_muted (self , ctx ) 
    }
    fn microphone_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: microphone_volume_level (self , ctx ) 
    }
    fn microphone_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: microphone_max_level (self , ctx ) 
    }
    fn microphone_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: microphone_min_level (self , ctx ) 
    }
    fn microphone_agc_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: microphone_agc_enabled (self , ctx ) 
    }
    fn image_rotation (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: image_rotation (self , ctx ) 
    }
    fn image_flip_horizontal (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: image_flip_horizontal (self , ctx ) 
    }
    fn image_flip_vertical (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: image_flip_vertical (self , ctx ) 
    }
    fn local_video_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: local_video_recording_enabled (self , ctx ) 
    }
    fn local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: local_snapshot_recording_enabled (self , ctx ) 
    }
    fn status_light_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: status_light_enabled (self , ctx ) 
    }
    fn status_light_brightness (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < ThreeLevelAutoEnum , crate :: error :: Error > {
        T :: status_light_brightness (self , ctx ) 
    }
    fn image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u16 , crate :: error :: Error > {
        T :: image_rotation_discrete_angles (self , ctx ) 
    }
    fn set_hdr_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_hdr_mode_enabled (self , ctx , value ) 
    }
    fn set_soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_soft_recording_privacy_mode_enabled (self , ctx , value ) 
    }
    fn set_soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_soft_livestream_privacy_mode_enabled (self , ctx , value ) 
    }
    fn set_night_vision (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_night_vision (self , ctx , value ) 
    }
    fn set_night_vision_illum (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_night_vision_illum (self , ctx , value ) 
    }
    fn set_viewport (& self , ctx : impl crate :: dm :: WriteContext , value : ViewportStruct < '_ > ) -> Result < () , crate :: error :: Error > {
        T :: set_viewport (self , ctx , value ) 
    }
    fn set_speaker_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_speaker_muted (self , ctx , value ) 
    }
    fn set_speaker_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> Result < () , crate :: error :: Error > {
        T :: set_speaker_volume_level (self , ctx , value ) 
    }
    fn set_microphone_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_microphone_muted (self , ctx , value ) 
    }
    fn set_microphone_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> Result < () , crate :: error :: Error > {
        T :: set_microphone_volume_level (self , ctx , value ) 
    }
    fn set_microphone_agc_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_microphone_agc_enabled (self , ctx , value ) 
    }
    fn set_image_rotation (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        T :: set_image_rotation (self , ctx , value ) 
    }
    fn set_image_flip_horizontal (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_image_flip_horizontal (self , ctx , value ) 
    }
    fn set_image_flip_vertical (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_image_flip_vertical (self , ctx , value ) 
    }
    fn set_local_video_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_local_video_recording_enabled (self , ctx , value ) 
    }
    fn set_local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_local_snapshot_recording_enabled (self , ctx , value ) 
    }
    fn set_status_light_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> Result < () , crate :: error :: Error > {
        T :: set_status_light_enabled (self , ctx , value ) 
    }
    fn set_status_light_brightness (& self , ctx : impl crate :: dm :: WriteContext , value : ThreeLevelAutoEnum ) -> Result < () , crate :: error :: Error > {
        T :: set_status_light_brightness (self , ctx , value ) 
    }
    fn set_image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> Result < () , crate :: error :: Error > {
        T :: set_image_rotation_discrete_angles (self , ctx , value ) 
    }
    fn handle_audio_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamAllocateRequest < '_ > , response : AudioStreamAllocateResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_audio_stream_allocate (self , ctx , request , response ) 
    }
    fn handle_audio_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamDeallocateRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_audio_stream_deallocate (self , ctx , request ) 
    }
    fn handle_video_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamAllocateRequest < '_ > , response : VideoStreamAllocateResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_video_stream_allocate (self , ctx , request , response ) 
    }
    fn handle_video_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamModifyRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_video_stream_modify (self , ctx , request ) 
    }
    fn handle_video_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamDeallocateRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_video_stream_deallocate (self , ctx , request ) 
    }
    fn handle_snapshot_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamAllocateRequest < '_ > , response : SnapshotStreamAllocateResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_snapshot_stream_allocate (self , ctx , request , response ) 
    }
    fn handle_snapshot_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamModifyRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_snapshot_stream_modify (self , ctx , request ) 
    }
    fn handle_snapshot_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamDeallocateRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_snapshot_stream_deallocate (self , ctx , request ) 
    }
    fn handle_set_stream_priorities (& self , ctx : impl crate :: dm :: InvokeContext , request : SetStreamPrioritiesRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_set_stream_priorities (self , ctx , request ) 
    }
    fn handle_capture_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CaptureSnapshotRequest < '_ > , response : CaptureSnapshotResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_capture_snapshot (self , ctx , request , response ) 
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
                    AttributeId :: MaxConcurrentEncoders => {
                        let attr_read_result = self . 0. max_concurrent_encoders (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxConcurrentEncoders , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxEncodedPixelRate => {
                        let attr_read_result = self . 0. max_encoded_pixel_rate (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxEncodedPixelRate , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: VideoSensorParams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: VideoSensorParams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. video_sensor_params (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: VideoSensorParams , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: VideoSensorParams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: NightVisionUsesInfrared => {
                        let attr_read_result = self . 0. night_vision_uses_infrared (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVisionUsesInfrared , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinViewportResolution => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinViewportResolution , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. min_viewport_resolution (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinViewportResolution , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinViewportResolution , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RateDistortionTradeOffPoints => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RateDistortionTradeOffPoints , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. rate_distortion_trade_off_points (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RateDistortionTradeOffPoints , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RateDistortionTradeOffPoints , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxContentBufferSize => {
                        let attr_read_result = self . 0. max_content_buffer_size (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxContentBufferSize , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneCapabilities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneCapabilities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. microphone_capabilities (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneCapabilities , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneCapabilities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SpeakerCapabilities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerCapabilities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. speaker_capabilities (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerCapabilities , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerCapabilities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: TwoWayTalkSupport => {
                        let attr_read_result = self . 0. two_way_talk_support (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TwoWayTalkSupport , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SnapshotCapabilities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SnapshotCapabilities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. snapshot_capabilities (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SnapshotCapabilities , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SnapshotCapabilities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxNetworkBandwidth => {
                        let attr_read_result = self . 0. max_network_bandwidth (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxNetworkBandwidth , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentFrameRate => {
                        let attr_read_result = self . 0. current_frame_rate (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentFrameRate , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HDRModeEnabled => {
                        let attr_read_result = self . 0. hdr_mode_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HDRModeEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SupportedStreamUsages => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedStreamUsages , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. supported_stream_usages (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedStreamUsages , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedStreamUsages , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: AllocatedVideoStreams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedVideoStreams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. allocated_video_streams (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedVideoStreams , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedVideoStreams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: AllocatedAudioStreams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedAudioStreams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. allocated_audio_streams (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedAudioStreams , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedAudioStreams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: AllocatedSnapshotStreams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedSnapshotStreams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. allocated_snapshot_streams (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedSnapshotStreams , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedSnapshotStreams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: StreamUsagePriorities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StreamUsagePriorities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. stream_usage_priorities (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StreamUsagePriorities , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StreamUsagePriorities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SoftRecordingPrivacyModeEnabled => {
                        let attr_read_result = self . 0. soft_recording_privacy_mode_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftRecordingPrivacyModeEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SoftLivestreamPrivacyModeEnabled => {
                        let attr_read_result = self . 0. soft_livestream_privacy_mode_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftLivestreamPrivacyModeEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HardPrivacyModeOn => {
                        let attr_read_result = self . 0. hard_privacy_mode_on (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HardPrivacyModeOn , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NightVision => {
                        let attr_read_result = self . 0. night_vision (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVision , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NightVisionIllum => {
                        let attr_read_result = self . 0. night_vision_illum (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVisionIllum , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Viewport => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. viewport (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) , tw ) , tag , ) ? ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SpeakerMuted => {
                        let attr_read_result = self . 0. speaker_muted (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMuted , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeakerVolumeLevel => {
                        let attr_read_result = self . 0. speaker_volume_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerVolumeLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeakerMaxLevel => {
                        let attr_read_result = self . 0. speaker_max_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMaxLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeakerMinLevel => {
                        let attr_read_result = self . 0. speaker_min_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMinLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneMuted => {
                        let attr_read_result = self . 0. microphone_muted (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMuted , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneVolumeLevel => {
                        let attr_read_result = self . 0. microphone_volume_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneVolumeLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneMaxLevel => {
                        let attr_read_result = self . 0. microphone_max_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMaxLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneMinLevel => {
                        let attr_read_result = self . 0. microphone_min_level (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMinLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneAGCEnabled => {
                        let attr_read_result = self . 0. microphone_agc_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneAGCEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageRotation => {
                        let attr_read_result = self . 0. image_rotation (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageFlipHorizontal => {
                        let attr_read_result = self . 0. image_flip_horizontal (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipHorizontal , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageFlipVertical => {
                        let attr_read_result = self . 0. image_flip_vertical (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipVertical , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocalVideoRecordingEnabled => {
                        let attr_read_result = self . 0. local_video_recording_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalVideoRecordingEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocalSnapshotRecordingEnabled => {
                        let attr_read_result = self . 0. local_snapshot_recording_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalSnapshotRecordingEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StatusLightEnabled => {
                        let attr_read_result = self . 0. status_light_enabled (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StatusLightBrightness => {
                        let attr_read_result = self . 0. status_light_brightness (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightBrightness , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageRotationDiscreteAngles => {
                        let attr_read_result = self . 0. image_rotation_discrete_angles (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotationDiscreteAngles , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: HDRModeEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_hdr_mode_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HDRModeEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SoftRecordingPrivacyModeEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_soft_recording_privacy_mode_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftRecordingPrivacyModeEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SoftLivestreamPrivacyModeEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_soft_livestream_privacy_mode_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftLivestreamPrivacyModeEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: NightVision => {
                let attr_data : TriStateAutoEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_night_vision (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVision , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: NightVisionIllum => {
                let attr_data : TriStateAutoEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_night_vision_illum (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVisionIllum , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: Viewport => {
                let attr_data : ViewportStruct < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_viewport (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SpeakerMuted => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_speaker_muted (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMuted , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SpeakerVolumeLevel => {
                let attr_data : u8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_speaker_volume_level (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerVolumeLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MicrophoneMuted => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_microphone_muted (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMuted , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MicrophoneVolumeLevel => {
                let attr_data : u8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_microphone_volume_level (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneVolumeLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MicrophoneAGCEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_microphone_agc_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneAGCEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageRotation => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_rotation (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotation , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageFlipHorizontal => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_flip_horizontal (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipHorizontal , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageFlipVertical => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_flip_vertical (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipVertical , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: LocalVideoRecordingEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_local_video_recording_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalVideoRecordingEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: LocalSnapshotRecordingEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_local_snapshot_recording_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalSnapshotRecordingEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: StatusLightEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_status_light_enabled (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: StatusLightBrightness => {
                let attr_data : ThreeLevelAutoEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_status_light_brightness (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightBrightness , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageRotationDiscreteAngles => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_rotation_discrete_angles (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotationDiscreteAngles , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: AudioStreamAllocate => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamAllocate ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_audio_stream_allocate (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamAllocate ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamAllocate ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AudioStreamDeallocate => {
                let cmd_data : AudioStreamDeallocateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_audio_stream_deallocate (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamDeallocate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: VideoStreamAllocate => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamAllocate ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (4u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_video_stream_allocate (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamAllocate ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamAllocate ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: VideoStreamModify => {
                let cmd_data : VideoStreamModifyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_video_stream_modify (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamModify ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: VideoStreamDeallocate => {
                let cmd_data : VideoStreamDeallocateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_video_stream_deallocate (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamDeallocate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SnapshotStreamAllocate => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamAllocate ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_snapshot_stream_allocate (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamAllocate ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamAllocate ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: SnapshotStreamModify => {
                let cmd_data : SnapshotStreamModifyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_snapshot_stream_modify (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamModify ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SnapshotStreamDeallocate => {
                let cmd_data : SnapshotStreamDeallocateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_snapshot_stream_deallocate (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamDeallocate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetStreamPriorities => {
                let cmd_data : SetStreamPrioritiesRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_stream_priorities (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetStreamPriorities ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: CaptureSnapshot => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CaptureSnapshot ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (13u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_capture_snapshot (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CaptureSnapshot ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CaptureSnapshot ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 1361u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "CameraAvStreamManagement" , 1361u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "CameraAvStreamManagement" , 1361u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `CameraAvStreamManagement` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct CameraAvStreamManagementCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > CameraAvStreamManagementCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_allocate (self , endpoint : crate :: dm :: EndptId , ) -> Result < AudioStreamAllocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 0 ) ? . data_builder () 
    }
    pub fn audio_stream_deallocate (self , endpoint : crate :: dm :: EndptId , ) -> Result < AudioStreamDeallocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 2 ) ? . data_builder () 
    }
    pub fn video_stream_allocate (self , endpoint : crate :: dm :: EndptId , ) -> Result < VideoStreamAllocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 3 ) ? . data_builder () 
    }
    pub fn video_stream_modify (self , endpoint : crate :: dm :: EndptId , ) -> Result < VideoStreamModifyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 5 ) ? . data_builder () 
    }
    pub fn video_stream_deallocate (self , endpoint : crate :: dm :: EndptId , ) -> Result < VideoStreamDeallocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 6 ) ? . data_builder () 
    }
    pub fn snapshot_stream_allocate (self , endpoint : crate :: dm :: EndptId , ) -> Result < SnapshotStreamAllocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 7 ) ? . data_builder () 
    }
    pub fn snapshot_stream_modify (self , endpoint : crate :: dm :: EndptId , ) -> Result < SnapshotStreamModifyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 9 ) ? . data_builder () 
    }
    pub fn snapshot_stream_deallocate (self , endpoint : crate :: dm :: EndptId , ) -> Result < SnapshotStreamDeallocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 10 ) ? . data_builder () 
    }
    pub fn set_stream_priorities (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetStreamPrioritiesRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 11 ) ? . data_builder () 
    }
    pub fn capture_snapshot (self , endpoint : crate :: dm :: EndptId , ) -> Result < CaptureSnapshotRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1361 , 12 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `CameraAvStreamManagement` cluster's commands. `use` this trait to call `.camera_av_stream_management_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`CameraAvStreamManagementCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait CameraAvStreamManagementCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn camera_av_stream_management_inv (self ) -> CameraAvStreamManagementCmdRequestsView < P > ;
    
}
impl < P > CameraAvStreamManagementCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn camera_av_stream_management_inv (self ) -> CameraAvStreamManagementCmdRequestsView < P > {
        CameraAvStreamManagementCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `CameraAvStreamManagement` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct CameraAvStreamManagementAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > CameraAvStreamManagementAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn max_concurrent_encoders (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_encoded_pixel_rate (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn video_sensor_params (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn night_vision_uses_infrared (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn min_viewport_resolution (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn rate_distortion_trade_off_points (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_content_buffer_size (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (6 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_capabilities (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (7 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speaker_capabilities (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (8 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn two_way_talk_support (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (9 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn snapshot_capabilities (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (10 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn max_network_bandwidth (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (11 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_frame_rate (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (12 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn hdr_mode_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (13 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn supported_stream_usages (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (14 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn allocated_video_streams (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (15 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn allocated_audio_streams (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (16 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn allocated_snapshot_streams (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (17 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn stream_usage_priorities (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (18 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn soft_recording_privacy_mode_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (19 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn soft_livestream_privacy_mode_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (20 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn hard_privacy_mode_on (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (21 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn night_vision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (22 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn night_vision_illum (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (23 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn viewport (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (24 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speaker_muted (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (25 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speaker_volume_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (26 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speaker_max_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (27 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speaker_min_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (28 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_muted (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (29 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_volume_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (30 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_max_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (31 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_min_level (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (32 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_agc_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (33 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_rotation (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (34 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_flip_horizontal (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (35 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_flip_vertical (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (36 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn local_video_recording_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (37 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn local_snapshot_recording_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (38 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn status_light_enabled (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (39 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn status_light_brightness (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (40 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_rotation_discrete_angles (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (41 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1361 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `CameraAvStreamManagement` cluster's attribute reads. `use` this trait to call `.camera_av_stream_management_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`CameraAvStreamManagementAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait CameraAvStreamManagementAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn camera_av_stream_management_read (self ) -> CameraAvStreamManagementAttrReadsView < P > ;
    
}
impl < P > CameraAvStreamManagementAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn camera_av_stream_management_read (self ) -> CameraAvStreamManagementAttrReadsView < P > {
        CameraAvStreamManagementAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `CameraAvStreamManagement` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct CameraAvStreamManagementAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > CameraAvStreamManagementAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn hdr_mode_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 13 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn soft_recording_privacy_mode_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 19 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn soft_livestream_privacy_mode_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 20 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn night_vision (self , endpoint : crate :: dm :: EndptId , value : TriStateAutoEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 22 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn night_vision_illum (self , endpoint : crate :: dm :: EndptId , value : TriStateAutoEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 23 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn viewport (self , endpoint : crate :: dm :: EndptId , ) -> Result < ViewportStructBuilder < crate :: im :: AttrDataBuilder < crate :: im :: AttrDataArrayBuilder < P > , 3 > > , crate :: error :: Error > {
        self . array . push () ? . path (endpoint , 1361 , 24 ) ? . data_builder () 
    }
    pub fn speaker_muted (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 25 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn speaker_volume_level (self , endpoint : crate :: dm :: EndptId , value : u8 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 26 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_muted (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 29 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_volume_level (self , endpoint : crate :: dm :: EndptId , value : u8 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 30 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn microphone_agc_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 33 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_rotation (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 34 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_flip_horizontal (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 35 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_flip_vertical (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 36 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn local_video_recording_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 37 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn local_snapshot_recording_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 38 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn status_light_enabled (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 39 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn status_light_brightness (self , endpoint : crate :: dm :: EndptId , value : ThreeLevelAutoEnum , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 40 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn image_rotation_discrete_angles (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . path (endpoint , 1361 , 41 ) ? . data (| w | crate :: tlv :: ToTLV :: to_tlv (& value , & crate :: tlv :: TLVTag :: Context (crate :: im :: AttrDataTag :: Data as u8 , ) , w , ) ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `CameraAvStreamManagement` cluster's attribute writes. `use` this trait to call `.camera_av_stream_management_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`CameraAvStreamManagementAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait CameraAvStreamManagementAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn camera_av_stream_management_write (self ) -> CameraAvStreamManagementAttrWritesView < P > ;
    
}
impl < P > CameraAvStreamManagementAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn camera_av_stream_management_write (self ) -> CameraAvStreamManagementAttrWritesView < P > {
        CameraAvStreamManagementAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `CameraAvStreamManagement` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct CameraAvStreamManagementCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > CameraAvStreamManagementCmdResponsesView < 'a , 'r > {
    pub fn audio_stream_allocate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AudioStreamAllocateResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < AudioStreamAllocateResponse < 'a >> (1361 , 0 ) 
    }
    pub fn audio_stream_deallocate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 2 ) 
    }
    pub fn video_stream_allocate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < VideoStreamAllocateResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < VideoStreamAllocateResponse < 'a >> (1361 , 3 ) 
    }
    pub fn video_stream_modify (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 5 ) 
    }
    pub fn video_stream_deallocate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 6 ) 
    }
    pub fn snapshot_stream_allocate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < SnapshotStreamAllocateResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < SnapshotStreamAllocateResponse < 'a >> (1361 , 7 ) 
    }
    pub fn snapshot_stream_modify (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 9 ) 
    }
    pub fn snapshot_stream_deallocate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 10 ) 
    }
    pub fn set_stream_priorities (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 11 ) 
    }
    pub fn capture_snapshot (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < CaptureSnapshotResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < CaptureSnapshotResponse < 'a >> (1361 , 12 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `CameraAvStreamManagement`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.camera_av_stream_management_inv_resp()` on an `InvokeResp`; the returned [`CameraAvStreamManagementCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait CameraAvStreamManagementCmdResponses < 'a > {
    fn camera_av_stream_management_inv_resp (& self ) -> CameraAvStreamManagementCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > CameraAvStreamManagementCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn camera_av_stream_management_inv_resp (& self ) -> CameraAvStreamManagementCmdResponsesView < 'a , '_ > {
        CameraAvStreamManagementCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `CameraAvStreamManagement` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct CameraAvStreamManagementAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > CameraAvStreamManagementAttrResponsesView < 'a , 'r > {
    pub fn max_concurrent_encoders (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (1361 , 0 ) 
    }
    pub fn max_encoded_pixel_rate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (1361 , 1 ) 
    }
    pub fn video_sensor_params (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < VideoSensorParamsStruct < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < VideoSensorParamsStruct < 'a > > (1361 , 2 ) 
    }
    pub fn night_vision_uses_infrared (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 3 ) 
    }
    pub fn min_viewport_resolution (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < VideoResolutionStruct < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < VideoResolutionStruct < 'a > > (1361 , 4 ) 
    }
    pub fn rate_distortion_trade_off_points (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , RateDistortionTradeOffPointsStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , RateDistortionTradeOffPointsStruct < 'a > > > (1361 , 5 ) 
    }
    pub fn max_content_buffer_size (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (1361 , 6 ) 
    }
    pub fn microphone_capabilities (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AudioCapabilitiesStruct < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < AudioCapabilitiesStruct < 'a > > (1361 , 7 ) 
    }
    pub fn speaker_capabilities (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AudioCapabilitiesStruct < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < AudioCapabilitiesStruct < 'a > > (1361 , 8 ) 
    }
    pub fn two_way_talk_support (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < TwoWayTalkSupportTypeEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < TwoWayTalkSupportTypeEnum > (1361 , 9 ) 
    }
    pub fn snapshot_capabilities (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , SnapshotCapabilitiesStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , SnapshotCapabilitiesStruct < 'a > > > (1361 , 10 ) 
    }
    pub fn max_network_bandwidth (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (1361 , 11 ) 
    }
    pub fn current_frame_rate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (1361 , 12 ) 
    }
    pub fn hdr_mode_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 13 ) 
    }
    pub fn supported_stream_usages (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , StreamUsageEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , StreamUsageEnum > > (1361 , 14 ) 
    }
    pub fn allocated_video_streams (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , VideoStreamStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , VideoStreamStruct < 'a > > > (1361 , 15 ) 
    }
    pub fn allocated_audio_streams (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , AudioStreamStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , AudioStreamStruct < 'a > > > (1361 , 16 ) 
    }
    pub fn allocated_snapshot_streams (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , SnapshotStreamStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , SnapshotStreamStruct < 'a > > > (1361 , 17 ) 
    }
    pub fn stream_usage_priorities (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , StreamUsageEnum > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , StreamUsageEnum > > (1361 , 18 ) 
    }
    pub fn soft_recording_privacy_mode_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 19 ) 
    }
    pub fn soft_livestream_privacy_mode_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 20 ) 
    }
    pub fn hard_privacy_mode_on (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 21 ) 
    }
    pub fn night_vision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < TriStateAutoEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < TriStateAutoEnum > (1361 , 22 ) 
    }
    pub fn night_vision_illum (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < TriStateAutoEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < TriStateAutoEnum > (1361 , 23 ) 
    }
    pub fn viewport (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ViewportStruct < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ViewportStruct < 'a > > (1361 , 24 ) 
    }
    pub fn speaker_muted (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 25 ) 
    }
    pub fn speaker_volume_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (1361 , 26 ) 
    }
    pub fn speaker_max_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (1361 , 27 ) 
    }
    pub fn speaker_min_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (1361 , 28 ) 
    }
    pub fn microphone_muted (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 29 ) 
    }
    pub fn microphone_volume_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (1361 , 30 ) 
    }
    pub fn microphone_max_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (1361 , 31 ) 
    }
    pub fn microphone_min_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (1361 , 32 ) 
    }
    pub fn microphone_agc_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 33 ) 
    }
    pub fn image_rotation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (1361 , 34 ) 
    }
    pub fn image_flip_horizontal (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 35 ) 
    }
    pub fn image_flip_vertical (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 36 ) 
    }
    pub fn local_video_recording_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 37 ) 
    }
    pub fn local_snapshot_recording_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 38 ) 
    }
    pub fn status_light_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (1361 , 39 ) 
    }
    pub fn status_light_brightness (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ThreeLevelAutoEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < ThreeLevelAutoEnum > (1361 , 40 ) 
    }
    pub fn image_rotation_discrete_angles (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (1361 , 41 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1361 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1361 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (1361 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (1361 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (1361 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `CameraAvStreamManagement`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.camera_av_stream_management_read_resp()` on a `ReportDataResp`; the returned [`CameraAvStreamManagementAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait CameraAvStreamManagementAttrResponses < 'a > {
    fn camera_av_stream_management_read_resp (& self ) -> CameraAvStreamManagementAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > CameraAvStreamManagementAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn camera_av_stream_management_read_resp (& self ) -> CameraAvStreamManagementAttrResponsesView < 'a , '_ > {
        CameraAvStreamManagementAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `CameraAvStreamManagement` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct CameraAvStreamManagementWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > CameraAvStreamManagementWriteResponsesView < 'a , 'r > {
    pub fn hdr_mode_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 13 ) 
    }
    pub fn soft_recording_privacy_mode_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 19 ) 
    }
    pub fn soft_livestream_privacy_mode_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 20 ) 
    }
    pub fn night_vision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 22 ) 
    }
    pub fn night_vision_illum (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 23 ) 
    }
    pub fn viewport (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 24 ) 
    }
    pub fn speaker_muted (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 25 ) 
    }
    pub fn speaker_volume_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 26 ) 
    }
    pub fn microphone_muted (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 29 ) 
    }
    pub fn microphone_volume_level (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 30 ) 
    }
    pub fn microphone_agc_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 33 ) 
    }
    pub fn image_rotation (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 34 ) 
    }
    pub fn image_flip_horizontal (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 35 ) 
    }
    pub fn image_flip_vertical (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 36 ) 
    }
    pub fn local_video_recording_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 37 ) 
    }
    pub fn local_snapshot_recording_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 38 ) 
    }
    pub fn status_light_enabled (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 39 ) 
    }
    pub fn status_light_brightness (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 40 ) 
    }
    pub fn image_rotation_discrete_angles (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1361 , 41 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `CameraAvStreamManagement`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.camera_av_stream_management_write_resp()` on a `WriteResp`; the returned [`CameraAvStreamManagementWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait CameraAvStreamManagementWriteResponses < 'a > {
    fn camera_av_stream_management_write_resp (& self ) -> CameraAvStreamManagementWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > CameraAvStreamManagementWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn camera_av_stream_management_write_resp (& self ) -> CameraAvStreamManagementWriteResponsesView < 'a , '_ > {
        CameraAvStreamManagementWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `AudioStreamAllocateResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`AudioStreamAllocateResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct AudioStreamAllocateResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > AudioStreamAllocateResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < AudioStreamAllocateResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `VideoStreamAllocateResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`VideoStreamAllocateResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct VideoStreamAllocateResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > VideoStreamAllocateResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < VideoStreamAllocateResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `SnapshotStreamAllocateResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`SnapshotStreamAllocateResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct SnapshotStreamAllocateResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > SnapshotStreamAllocateResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < SnapshotStreamAllocateResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `CaptureSnapshotResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`CaptureSnapshotResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct CaptureSnapshotResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > CaptureSnapshotResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < CaptureSnapshotResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `CameraAvStreamManagement` cluster's client operations. Returned by [`CameraAvStreamManagementClient::camera_av_stream_management`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct CameraAvStreamManagementClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > CameraAvStreamManagementClientView < T > {
    pub async fn audio_stream_allocate < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < AudioStreamAllocateResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AudioStreamAllocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . audio_stream_allocate (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (AudioStreamAllocateResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn audio_stream_deallocate < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (AudioStreamDeallocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . audio_stream_deallocate (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn video_stream_allocate < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < VideoStreamAllocateResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (VideoStreamAllocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . video_stream_allocate (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (VideoStreamAllocateResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn video_stream_modify < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (VideoStreamModifyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . video_stream_modify (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn video_stream_deallocate < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (VideoStreamDeallocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . video_stream_deallocate (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn snapshot_stream_allocate < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < SnapshotStreamAllocateResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (SnapshotStreamAllocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . snapshot_stream_allocate (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (SnapshotStreamAllocateResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn snapshot_stream_modify < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SnapshotStreamModifyRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . snapshot_stream_modify (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn snapshot_stream_deallocate < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SnapshotStreamDeallocateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . snapshot_stream_deallocate (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn set_stream_priorities < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SetStreamPrioritiesRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . set_stream_priorities (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn capture_snapshot < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < CaptureSnapshotResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (CaptureSnapshotRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: CameraAvStreamManagementCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . camera_av_stream_management_inv () ;
            let req_builder = view . capture_snapshot (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (CaptureSnapshotResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn max_concurrent_encoders_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . max_concurrent_encoders (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn max_encoded_pixel_rate_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . max_encoded_pixel_rate (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn video_sensor_params_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < VideoSensorParamsStruct < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . video_sensor_params (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn night_vision_uses_infrared_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . night_vision_uses_infrared (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn min_viewport_resolution_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < VideoResolutionStruct < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . min_viewport_resolution (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn rate_distortion_trade_off_points_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , RateDistortionTradeOffPointsStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . rate_distortion_trade_off_points (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn max_content_buffer_size_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . max_content_buffer_size (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn microphone_capabilities_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < AudioCapabilitiesStruct < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . microphone_capabilities (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn speaker_capabilities_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < AudioCapabilitiesStruct < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . speaker_capabilities (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn two_way_talk_support_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < TwoWayTalkSupportTypeEnum , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . two_way_talk_support (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn snapshot_capabilities_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , SnapshotCapabilitiesStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . snapshot_capabilities (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn max_network_bandwidth_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . max_network_bandwidth (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn current_frame_rate_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . current_frame_rate (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn hdr_mode_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . hdr_mode_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn supported_stream_usages_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , StreamUsageEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . supported_stream_usages (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn allocated_video_streams_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , VideoStreamStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . allocated_video_streams (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn allocated_audio_streams_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , AudioStreamStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . allocated_audio_streams (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn allocated_snapshot_streams_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , SnapshotStreamStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . allocated_snapshot_streams (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn stream_usage_priorities_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , StreamUsageEnum > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . stream_usage_priorities (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn soft_recording_privacy_mode_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . soft_recording_privacy_mode_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn soft_livestream_privacy_mode_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . soft_livestream_privacy_mode_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn hard_privacy_mode_on_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . hard_privacy_mode_on (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn night_vision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < TriStateAutoEnum , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . night_vision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn night_vision_illum_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < TriStateAutoEnum , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . night_vision_illum (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn viewport_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < ViewportStruct < '_ > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . viewport (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn speaker_muted_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . speaker_muted (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn speaker_volume_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . speaker_volume_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn speaker_max_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . speaker_max_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn speaker_min_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . speaker_min_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn microphone_muted_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . microphone_muted (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn microphone_volume_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . microphone_volume_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn microphone_max_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . microphone_max_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn microphone_min_level_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . microphone_min_level (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn microphone_agc_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . microphone_agc_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn image_rotation_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . image_rotation (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn image_flip_horizontal_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . image_flip_horizontal (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn image_flip_vertical_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . image_flip_vertical (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn local_video_recording_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . local_video_recording_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn local_snapshot_recording_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . local_snapshot_recording_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn status_light_enabled_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . status_light_enabled (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn status_light_brightness_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < ThreeLevelAutoEnum , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . status_light_brightness (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn image_rotation_discrete_angles_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . image_rotation_discrete_angles (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . camera_av_stream_management_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn hdr_mode_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . hdr_mode_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn soft_recording_privacy_mode_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . soft_recording_privacy_mode_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn soft_livestream_privacy_mode_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . soft_livestream_privacy_mode_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn night_vision_write (self , endpoint : crate :: dm :: EndptId , value : TriStateAutoEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . night_vision (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn night_vision_illum_write (self , endpoint : crate :: dm :: EndptId , value : TriStateAutoEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . night_vision_illum (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn speaker_muted_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . speaker_muted (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn speaker_volume_level_write (self , endpoint : crate :: dm :: EndptId , value : u8 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . speaker_volume_level (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn microphone_muted_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . microphone_muted (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn microphone_volume_level_write (self , endpoint : crate :: dm :: EndptId , value : u8 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . microphone_volume_level (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn microphone_agc_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . microphone_agc_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn image_rotation_write (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . image_rotation (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn image_flip_horizontal_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . image_flip_horizontal (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn image_flip_vertical_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . image_flip_vertical (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn local_video_recording_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . local_video_recording_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn local_snapshot_recording_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . local_snapshot_recording_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn status_light_enabled_write (self , endpoint : crate :: dm :: EndptId , value : bool , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . status_light_enabled (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn status_light_brightness_write (self , endpoint : crate :: dm :: EndptId , value : ThreeLevelAutoEnum , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . status_light_brightness (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    pub fn image_rotation_discrete_angles_write (self , endpoint : crate :: dm :: EndptId , value : u16 , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        use self :: CameraAvStreamManagementAttrWrites as _Writes ;
        crate :: im :: write_attr (self . tx , None , move | msg | {
            msg . write_requests () ? . camera_av_stream_management_write () . image_rotation_discrete_angles (endpoint , value . clone () ) ? . end () ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `CameraAvStreamManagement` cluster. `use` this trait to call `.camera_av_stream_management()` on any [`crate::im::ImClient`], such as an exchange; the returned [`CameraAvStreamManagementClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `CameraAvStreamManagementClientView` keeps each cluster's API surface narrow: at the call site `exchange.camera_av_stream_management().` shows only this cluster's methods in IDE completion." ] pub trait CameraAvStreamManagementClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn camera_av_stream_management (self ) -> CameraAvStreamManagementClientView < Self > {
        CameraAvStreamManagementClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > CameraAvStreamManagementClient for T {
    
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
    fn max_concurrent_encoders (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_encoded_pixel_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn video_sensor_params < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoSensorParamsStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn night_vision_uses_infrared (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn min_viewport_resolution < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoResolutionStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn rate_distortion_trade_off_points < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RateDistortionTradeOffPointsStructArrayBuilder < P > , RateDistortionTradeOffPointsStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_content_buffer_size (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn microphone_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn speaker_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn two_way_talk_support (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TwoWayTalkSupportTypeEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn snapshot_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotCapabilitiesStructArrayBuilder < P > , SnapshotCapabilitiesStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn max_network_bandwidth (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn current_frame_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn hdr_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn supported_stream_usages < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn allocated_video_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < VideoStreamStructArrayBuilder < P > , VideoStreamStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn allocated_audio_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < AudioStreamStructArrayBuilder < P > , AudioStreamStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn allocated_snapshot_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotStreamStructArrayBuilder < P > , SnapshotStreamStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn stream_usage_priorities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn hard_privacy_mode_on (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn night_vision (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TriStateAutoEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn night_vision_illum (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TriStateAutoEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn viewport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : ViewportStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn speaker_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn speaker_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn speaker_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn speaker_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn microphone_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn microphone_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn microphone_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn microphone_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn microphone_agc_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn image_rotation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn image_flip_horizontal (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn image_flip_vertical (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn local_video_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn status_light_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn status_light_brightness (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ThreeLevelAutoEnum , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_hdr_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_night_vision (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_night_vision_illum (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_viewport (& self , ctx : impl crate :: dm :: WriteContext , value : ViewportStruct < '_ > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_speaker_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_speaker_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_microphone_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_microphone_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_microphone_agc_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_image_rotation (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_image_flip_horizontal (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_image_flip_vertical (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_local_video_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_status_light_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_status_light_brightness (& self , ctx : impl crate :: dm :: WriteContext , value : ThreeLevelAutoEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn set_image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> + crate :: utils :: sync :: MaybeSend {
        core :: future :: ready (Err (crate :: error :: ErrorCode :: AttributeNotFound . into () ) ) 
    }
    fn handle_audio_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamAllocateRequest < '_ > , response : AudioStreamAllocateResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_audio_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamDeallocateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_video_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamAllocateRequest < '_ > , response : VideoStreamAllocateResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_video_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamModifyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_video_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamDeallocateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_snapshot_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamAllocateRequest < '_ > , response : SnapshotStreamAllocateResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_snapshot_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamModifyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_snapshot_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamDeallocateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_stream_priorities (& self , ctx : impl crate :: dm :: InvokeContext , request : SetStreamPrioritiesRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_capture_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CaptureSnapshotRequest < '_ > , response : CaptureSnapshotResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn max_concurrent_encoders (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: max_concurrent_encoders (self , ctx ) 
    }
    fn max_encoded_pixel_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: max_encoded_pixel_rate (self , ctx ) 
    }
    fn video_sensor_params < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoSensorParamsStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: video_sensor_params (self , ctx , builder ) 
    }
    fn night_vision_uses_infrared (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: night_vision_uses_infrared (self , ctx ) 
    }
    fn min_viewport_resolution < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : VideoResolutionStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: min_viewport_resolution (self , ctx , builder ) 
    }
    fn rate_distortion_trade_off_points < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < RateDistortionTradeOffPointsStructArrayBuilder < P > , RateDistortionTradeOffPointsStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: rate_distortion_trade_off_points (self , ctx , builder ) 
    }
    fn max_content_buffer_size (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: max_content_buffer_size (self , ctx ) 
    }
    fn microphone_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: microphone_capabilities (self , ctx , builder ) 
    }
    fn speaker_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : AudioCapabilitiesStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: speaker_capabilities (self , ctx , builder ) 
    }
    fn two_way_talk_support (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TwoWayTalkSupportTypeEnum , crate :: error :: Error >> {
        T :: two_way_talk_support (self , ctx ) 
    }
    fn snapshot_capabilities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotCapabilitiesStructArrayBuilder < P > , SnapshotCapabilitiesStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: snapshot_capabilities (self , ctx , builder ) 
    }
    fn max_network_bandwidth (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        T :: max_network_bandwidth (self , ctx ) 
    }
    fn current_frame_rate (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: current_frame_rate (self , ctx ) 
    }
    fn hdr_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: hdr_mode_enabled (self , ctx ) 
    }
    fn supported_stream_usages < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: supported_stream_usages (self , ctx , builder ) 
    }
    fn allocated_video_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < VideoStreamStructArrayBuilder < P > , VideoStreamStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: allocated_video_streams (self , ctx , builder ) 
    }
    fn allocated_audio_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < AudioStreamStructArrayBuilder < P > , AudioStreamStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: allocated_audio_streams (self , ctx , builder ) 
    }
    fn allocated_snapshot_streams < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < SnapshotStreamStructArrayBuilder < P > , SnapshotStreamStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: allocated_snapshot_streams (self , ctx , builder ) 
    }
    fn stream_usage_priorities < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: ToTLVArrayBuilder < P , StreamUsageEnum > , crate :: tlv :: ToTLVBuilder < P , StreamUsageEnum > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: stream_usage_priorities (self , ctx , builder ) 
    }
    fn soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: soft_recording_privacy_mode_enabled (self , ctx ) 
    }
    fn soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: soft_livestream_privacy_mode_enabled (self , ctx ) 
    }
    fn hard_privacy_mode_on (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: hard_privacy_mode_on (self , ctx ) 
    }
    fn night_vision (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TriStateAutoEnum , crate :: error :: Error >> {
        T :: night_vision (self , ctx ) 
    }
    fn night_vision_illum (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < TriStateAutoEnum , crate :: error :: Error >> {
        T :: night_vision_illum (self , ctx ) 
    }
    fn viewport < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : ViewportStructBuilder < P > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: viewport (self , ctx , builder ) 
    }
    fn speaker_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: speaker_muted (self , ctx ) 
    }
    fn speaker_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: speaker_volume_level (self , ctx ) 
    }
    fn speaker_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: speaker_max_level (self , ctx ) 
    }
    fn speaker_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: speaker_min_level (self , ctx ) 
    }
    fn microphone_muted (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: microphone_muted (self , ctx ) 
    }
    fn microphone_volume_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: microphone_volume_level (self , ctx ) 
    }
    fn microphone_max_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: microphone_max_level (self , ctx ) 
    }
    fn microphone_min_level (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: microphone_min_level (self , ctx ) 
    }
    fn microphone_agc_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: microphone_agc_enabled (self , ctx ) 
    }
    fn image_rotation (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: image_rotation (self , ctx ) 
    }
    fn image_flip_horizontal (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: image_flip_horizontal (self , ctx ) 
    }
    fn image_flip_vertical (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: image_flip_vertical (self , ctx ) 
    }
    fn local_video_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: local_video_recording_enabled (self , ctx ) 
    }
    fn local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: local_snapshot_recording_enabled (self , ctx ) 
    }
    fn status_light_enabled (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: status_light_enabled (self , ctx ) 
    }
    fn status_light_brightness (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < ThreeLevelAutoEnum , crate :: error :: Error >> {
        T :: status_light_brightness (self , ctx ) 
    }
    fn image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        T :: image_rotation_discrete_angles (self , ctx ) 
    }
    fn set_hdr_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_hdr_mode_enabled (self , ctx , value ) 
    }
    fn set_soft_recording_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_soft_recording_privacy_mode_enabled (self , ctx , value ) 
    }
    fn set_soft_livestream_privacy_mode_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_soft_livestream_privacy_mode_enabled (self , ctx , value ) 
    }
    fn set_night_vision (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_night_vision (self , ctx , value ) 
    }
    fn set_night_vision_illum (& self , ctx : impl crate :: dm :: WriteContext , value : TriStateAutoEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_night_vision_illum (self , ctx , value ) 
    }
    fn set_viewport (& self , ctx : impl crate :: dm :: WriteContext , value : ViewportStruct < '_ > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_viewport (self , ctx , value ) 
    }
    fn set_speaker_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_speaker_muted (self , ctx , value ) 
    }
    fn set_speaker_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_speaker_volume_level (self , ctx , value ) 
    }
    fn set_microphone_muted (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_microphone_muted (self , ctx , value ) 
    }
    fn set_microphone_volume_level (& self , ctx : impl crate :: dm :: WriteContext , value : u8 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_microphone_volume_level (self , ctx , value ) 
    }
    fn set_microphone_agc_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_microphone_agc_enabled (self , ctx , value ) 
    }
    fn set_image_rotation (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_image_rotation (self , ctx , value ) 
    }
    fn set_image_flip_horizontal (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_image_flip_horizontal (self , ctx , value ) 
    }
    fn set_image_flip_vertical (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_image_flip_vertical (self , ctx , value ) 
    }
    fn set_local_video_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_local_video_recording_enabled (self , ctx , value ) 
    }
    fn set_local_snapshot_recording_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_local_snapshot_recording_enabled (self , ctx , value ) 
    }
    fn set_status_light_enabled (& self , ctx : impl crate :: dm :: WriteContext , value : bool ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_status_light_enabled (self , ctx , value ) 
    }
    fn set_status_light_brightness (& self , ctx : impl crate :: dm :: WriteContext , value : ThreeLevelAutoEnum ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_status_light_brightness (self , ctx , value ) 
    }
    fn set_image_rotation_discrete_angles (& self , ctx : impl crate :: dm :: WriteContext , value : u16 ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_image_rotation_discrete_angles (self , ctx , value ) 
    }
    fn handle_audio_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamAllocateRequest < '_ > , response : AudioStreamAllocateResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_audio_stream_allocate (self , ctx , request , response ) 
    }
    fn handle_audio_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : AudioStreamDeallocateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_audio_stream_deallocate (self , ctx , request ) 
    }
    fn handle_video_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamAllocateRequest < '_ > , response : VideoStreamAllocateResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_video_stream_allocate (self , ctx , request , response ) 
    }
    fn handle_video_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamModifyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_video_stream_modify (self , ctx , request ) 
    }
    fn handle_video_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : VideoStreamDeallocateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_video_stream_deallocate (self , ctx , request ) 
    }
    fn handle_snapshot_stream_allocate < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamAllocateRequest < '_ > , response : SnapshotStreamAllocateResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_snapshot_stream_allocate (self , ctx , request , response ) 
    }
    fn handle_snapshot_stream_modify (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamModifyRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_snapshot_stream_modify (self , ctx , request ) 
    }
    fn handle_snapshot_stream_deallocate (& self , ctx : impl crate :: dm :: InvokeContext , request : SnapshotStreamDeallocateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_snapshot_stream_deallocate (self , ctx , request ) 
    }
    fn handle_set_stream_priorities (& self , ctx : impl crate :: dm :: InvokeContext , request : SetStreamPrioritiesRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_set_stream_priorities (self , ctx , request ) 
    }
    fn handle_capture_snapshot < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CaptureSnapshotRequest < '_ > , response : CaptureSnapshotResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_capture_snapshot (self , ctx , request , response ) 
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
                    AttributeId :: MaxConcurrentEncoders => {
                        let attr_read_result = self . 0. max_concurrent_encoders (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxConcurrentEncoders , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MaxEncodedPixelRate => {
                        let attr_read_result = self . 0. max_encoded_pixel_rate (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxEncodedPixelRate , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: VideoSensorParams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: VideoSensorParams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. video_sensor_params (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: VideoSensorParams , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: VideoSensorParams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: NightVisionUsesInfrared => {
                        let attr_read_result = self . 0. night_vision_uses_infrared (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVisionUsesInfrared , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MinViewportResolution => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinViewportResolution , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. min_viewport_resolution (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinViewportResolution , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MinViewportResolution , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: RateDistortionTradeOffPoints => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RateDistortionTradeOffPoints , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. rate_distortion_trade_off_points (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RateDistortionTradeOffPoints , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: RateDistortionTradeOffPoints , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxContentBufferSize => {
                        let attr_read_result = self . 0. max_content_buffer_size (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxContentBufferSize , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneCapabilities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneCapabilities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. microphone_capabilities (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneCapabilities , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneCapabilities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SpeakerCapabilities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerCapabilities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. speaker_capabilities (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerCapabilities , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerCapabilities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: TwoWayTalkSupport => {
                        let attr_read_result = self . 0. two_way_talk_support (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TwoWayTalkSupport , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SnapshotCapabilities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SnapshotCapabilities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. snapshot_capabilities (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SnapshotCapabilities , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SnapshotCapabilities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: MaxNetworkBandwidth => {
                        let attr_read_result = self . 0. max_network_bandwidth (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MaxNetworkBandwidth , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CurrentFrameRate => {
                        let attr_read_result = self . 0. current_frame_rate (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentFrameRate , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HDRModeEnabled => {
                        let attr_read_result = self . 0. hdr_mode_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HDRModeEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SupportedStreamUsages => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedStreamUsages , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. supported_stream_usages (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedStreamUsages , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedStreamUsages , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: AllocatedVideoStreams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedVideoStreams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. allocated_video_streams (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedVideoStreams , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedVideoStreams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: AllocatedAudioStreams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedAudioStreams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. allocated_audio_streams (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedAudioStreams , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedAudioStreams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: AllocatedSnapshotStreams => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedSnapshotStreams , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. allocated_snapshot_streams (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedSnapshotStreams , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: AllocatedSnapshotStreams , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: StreamUsagePriorities => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StreamUsagePriorities , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. stream_usage_priorities (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StreamUsagePriorities , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StreamUsagePriorities , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SoftRecordingPrivacyModeEnabled => {
                        let attr_read_result = self . 0. soft_recording_privacy_mode_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftRecordingPrivacyModeEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SoftLivestreamPrivacyModeEnabled => {
                        let attr_read_result = self . 0. soft_livestream_privacy_mode_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftLivestreamPrivacyModeEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: HardPrivacyModeOn => {
                        let attr_read_result = self . 0. hard_privacy_mode_on (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HardPrivacyModeOn , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NightVision => {
                        let attr_read_result = self . 0. night_vision (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVision , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: NightVisionIllum => {
                        let attr_read_result = self . 0. night_vision_illum (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVisionIllum , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: Viewport => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. viewport (& ctx , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) , tw ) , tag , ) ? ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SpeakerMuted => {
                        let attr_read_result = self . 0. speaker_muted (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMuted , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeakerVolumeLevel => {
                        let attr_read_result = self . 0. speaker_volume_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerVolumeLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeakerMaxLevel => {
                        let attr_read_result = self . 0. speaker_max_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMaxLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: SpeakerMinLevel => {
                        let attr_read_result = self . 0. speaker_min_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMinLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneMuted => {
                        let attr_read_result = self . 0. microphone_muted (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMuted , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneVolumeLevel => {
                        let attr_read_result = self . 0. microphone_volume_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneVolumeLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneMaxLevel => {
                        let attr_read_result = self . 0. microphone_max_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMaxLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneMinLevel => {
                        let attr_read_result = self . 0. microphone_min_level (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMinLevel , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: MicrophoneAGCEnabled => {
                        let attr_read_result = self . 0. microphone_agc_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneAGCEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageRotation => {
                        let attr_read_result = self . 0. image_rotation (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotation , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageFlipHorizontal => {
                        let attr_read_result = self . 0. image_flip_horizontal (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipHorizontal , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageFlipVertical => {
                        let attr_read_result = self . 0. image_flip_vertical (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipVertical , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocalVideoRecordingEnabled => {
                        let attr_read_result = self . 0. local_video_recording_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalVideoRecordingEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: LocalSnapshotRecordingEnabled => {
                        let attr_read_result = self . 0. local_snapshot_recording_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalSnapshotRecordingEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StatusLightEnabled => {
                        let attr_read_result = self . 0. status_light_enabled (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightEnabled , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: StatusLightBrightness => {
                        let attr_read_result = self . 0. status_light_brightness (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightBrightness , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: ImageRotationDiscreteAngles => {
                        let attr_read_result = self . 0. image_rotation_discrete_angles (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotationDiscreteAngles , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: HDRModeEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_hdr_mode_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: HDRModeEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SoftRecordingPrivacyModeEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_soft_recording_privacy_mode_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftRecordingPrivacyModeEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SoftLivestreamPrivacyModeEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_soft_livestream_privacy_mode_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SoftLivestreamPrivacyModeEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: NightVision => {
                let attr_data : TriStateAutoEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_night_vision (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVision , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: NightVisionIllum => {
                let attr_data : TriStateAutoEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_night_vision_illum (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NightVisionIllum , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: Viewport => {
                let attr_data : ViewportStruct < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_viewport (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Viewport , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SpeakerMuted => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_speaker_muted (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerMuted , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: SpeakerVolumeLevel => {
                let attr_data : u8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_speaker_volume_level (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SpeakerVolumeLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MicrophoneMuted => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_microphone_muted (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneMuted , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MicrophoneVolumeLevel => {
                let attr_data : u8 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_microphone_volume_level (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneVolumeLevel , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: MicrophoneAGCEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_microphone_agc_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: MicrophoneAGCEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageRotation => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_rotation (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotation , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageFlipHorizontal => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_flip_horizontal (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipHorizontal , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageFlipVertical => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_flip_vertical (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageFlipVertical , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: LocalVideoRecordingEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_local_video_recording_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalVideoRecordingEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: LocalSnapshotRecordingEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_local_snapshot_recording_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: LocalSnapshotRecordingEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: StatusLightEnabled => {
                let attr_data : bool = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_status_light_enabled (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightEnabled , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: StatusLightBrightness => {
                let attr_data : ThreeLevelAutoEnum = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_status_light_brightness (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: StatusLightBrightness , false ) ) ) ) , & attr_data , & attr_write_result ) ;
                attr_write_result ? ;
                
            }
            AttributeId :: ImageRotationDiscreteAngles => {
                let attr_data : u16 = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let attr_write_result = self . 0. set_image_rotation_discrete_angles (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: ImageRotationDiscreteAngles , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: AudioStreamAllocate => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamAllocate ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_audio_stream_allocate (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamAllocate ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamAllocate ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AudioStreamDeallocate => {
                let cmd_data : AudioStreamDeallocateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_audio_stream_deallocate (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AudioStreamDeallocate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: VideoStreamAllocate => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamAllocate ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (4u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_video_stream_allocate (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamAllocate ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamAllocate ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: VideoStreamModify => {
                let cmd_data : VideoStreamModifyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_video_stream_modify (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamModify ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: VideoStreamDeallocate => {
                let cmd_data : VideoStreamDeallocateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_video_stream_deallocate (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: VideoStreamDeallocate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SnapshotStreamAllocate => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamAllocate ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_snapshot_stream_allocate (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamAllocate ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamAllocate ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: SnapshotStreamModify => {
                let cmd_data : SnapshotStreamModifyRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_snapshot_stream_modify (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamModify ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SnapshotStreamDeallocate => {
                let cmd_data : SnapshotStreamDeallocateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_snapshot_stream_deallocate (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SnapshotStreamDeallocate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetStreamPriorities => {
                let cmd_data : SetStreamPrioritiesRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_stream_priorities (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetStreamPriorities ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: CaptureSnapshot => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CaptureSnapshot ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (13u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_capture_snapshot (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CaptureSnapshot ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CaptureSnapshot ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 1361u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "CameraAvStreamManagement" , 1361u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "CameraAvStreamManagement" , 1361u32 , self . 0.2 ) 
    }
    
}
