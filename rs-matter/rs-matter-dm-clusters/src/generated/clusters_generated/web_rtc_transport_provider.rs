# [cfg (not (feature = "defmt" ) ) ] crate :: reexport :: bitflags :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq , Hash ) ] pub struct Feature : u32 {
        const METADATA = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
# [cfg (feature = "defmt" ) ] crate :: reexport :: defmt :: bitflags ! {
    # [repr (transparent ) ] # [derive (Default ) ] pub struct Feature : u32 {
        const METADATA = 1 ;
        const _INTERNAL_ALL_BITS = ! 0 ;
        
    }
    
}
crate :: bitflags_tlv ! (Feature , u32 ) ;
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SFrameStructTag {
    CipherSuite = 0 , BaseKey = 1 , Kid = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SolicitOfferRequestTag {
    StreamUsage = 0 , OriginatingEndpointId = 1 , VideoStreamId = 2 , AudioStreamId = 3 , IceServers = 4 , IceTransportPolicy = 5 , MetadataEnabled = 6 , SFrameConfig = 7 , VideoStreams = 8 , AudioStreams = 9 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SolicitOfferResponseTag {
    WebRtcSessionId = 0 , DeferredOffer = 1 , VideoStreamId = 2 , AudioStreamId = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProvideOfferRequestTag {
    WebRtcSessionId = 0 , Sdp = 1 , StreamUsage = 2 , OriginatingEndpointId = 3 , VideoStreamId = 4 , AudioStreamId = 5 , IceServers = 6 , IceTransportPolicy = 7 , MetadataEnabled = 8 , SFrameConfig = 9 , VideoStreams = 10 , AudioStreams = 11 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProvideOfferResponseTag {
    WebRtcSessionId = 0 , VideoStreamId = 1 , AudioStreamId = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProvideAnswerRequestTag {
    WebRtcSessionId = 0 , Sdp = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProvideICECandidatesRequestTag {
    WebRtcSessionId = 0 , IceCandidates = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum EndSessionRequestTag {
    WebRtcSessionId = 0 , Reason = 1 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SFrameStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SFrameStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn cipher_suite (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn base_key (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn kid (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SFrameStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SFrameStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SFrameStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SFrameStruct" ) ? ;
        match self . cipher_suite () {
            Ok (value ) => write ! (f , "{}: {:?}," , "cipher_suite" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "cipher_suite" , e . code () ) ? , 
        }
        match self . base_key () {
            Ok (value ) => write ! (f , "{}: {:?}," , "base_key" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "base_key" , e . code () ) ? , 
        }
        match self . kid () {
            Ok (value ) => write ! (f , "{}: {:?}," , "kid" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "kid" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SFrameStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SFrameStruct" ) ;
        match self . cipher_suite () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "cipher_suite" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "cipher_suite" , e . code () ) , 
        }
        match self . base_key () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "base_key" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "base_key" , e . code () ) , 
        }
        match self . kid () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "kid" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "kid" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SolicitOfferRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SolicitOfferRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn stream_usage (& self ) -> Result < StreamUsageEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn originating_endpoint_id (& self ) -> Result < crate :: im :: EndptId , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn video_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn audio_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn ice_servers (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , ICEServerStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn ice_transport_policy (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn metadata_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    pub fn s_frame_config (& self ) -> Result < Option < SFrameStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (7 ) 
    }
    pub fn video_streams (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (8 ) 
    }
    pub fn audio_streams (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (9 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SolicitOfferRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SolicitOfferRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SolicitOfferRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SolicitOfferRequest" ) ? ;
        match self . stream_usage () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_usage" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) ? , 
        }
        match self . originating_endpoint_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "originating_endpoint_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "originating_endpoint_id" , e . code () ) ? , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "video_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "audio_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        match self . ice_servers () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "ice_servers" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "ice_servers" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ice_servers" , e . code () ) ? , 
        }
        match self . ice_transport_policy () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "ice_transport_policy" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "ice_transport_policy" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ice_transport_policy" , e . code () ) ? , 
        }
        match self . metadata_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "metadata_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "metadata_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "metadata_enabled" , e . code () ) ? , 
        }
        match self . s_frame_config () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "s_frame_config" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "s_frame_config" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "s_frame_config" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SolicitOfferRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SolicitOfferRequest" ) ;
        match self . stream_usage () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_usage" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) , 
        }
        match self . originating_endpoint_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "originating_endpoint_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "originating_endpoint_id" , e . code () ) , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "video_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "audio_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        match self . ice_servers () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "ice_servers" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "ice_servers" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ice_servers" , e . code () ) , 
        }
        match self . ice_transport_policy () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "ice_transport_policy" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "ice_transport_policy" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ice_transport_policy" , e . code () ) , 
        }
        match self . metadata_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "metadata_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "metadata_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "metadata_enabled" , e . code () ) , 
        }
        match self . s_frame_config () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "s_frame_config" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "s_frame_config" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "s_frame_config" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SolicitOfferResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SolicitOfferResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn web_rtc_session_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn deferred_offer (& self ) -> Result < bool , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn video_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    pub fn audio_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SolicitOfferResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SolicitOfferResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SolicitOfferResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SolicitOfferResponse" ) ? ;
        match self . web_rtc_session_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) ? , 
        }
        match self . deferred_offer () {
            Ok (value ) => write ! (f , "{}: {:?}," , "deferred_offer" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "deferred_offer" , e . code () ) ? , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "video_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "audio_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SolicitOfferResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SolicitOfferResponse" ) ;
        match self . web_rtc_session_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) , 
        }
        match self . deferred_offer () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "deferred_offer" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "deferred_offer" , e . code () ) , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "video_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "audio_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProvideOfferRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProvideOfferRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn web_rtc_session_id (& self ) -> Result < crate :: tlv :: Nullable < u16 > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn sdp (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn stream_usage (& self ) -> Result < StreamUsageEnum , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn originating_endpoint_id (& self ) -> Result < crate :: im :: EndptId , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn video_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (4 ) 
    }
    pub fn audio_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (5 ) 
    }
    pub fn ice_servers (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , ICEServerStruct < '_ > > > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    pub fn ice_transport_policy (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (7 ) 
    }
    pub fn metadata_enabled (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (8 ) 
    }
    pub fn s_frame_config (& self ) -> Result < Option < SFrameStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (9 ) 
    }
    pub fn video_streams (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (10 ) 
    }
    pub fn audio_streams (& self ) -> Result < Option < crate :: tlv :: TLVArray < '_ , u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (11 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProvideOfferRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProvideOfferRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProvideOfferRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProvideOfferRequest" ) ? ;
        match self . web_rtc_session_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) ? , 
        }
        match self . sdp () {
            Ok (value ) => write ! (f , "{}: {:?}," , "sdp" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sdp" , e . code () ) ? , 
        }
        match self . stream_usage () {
            Ok (value ) => write ! (f , "{}: {:?}," , "stream_usage" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) ? , 
        }
        match self . originating_endpoint_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "originating_endpoint_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "originating_endpoint_id" , e . code () ) ? , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "video_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "audio_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        match self . ice_servers () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "ice_servers" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "ice_servers" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ice_servers" , e . code () ) ? , 
        }
        match self . ice_transport_policy () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "ice_transport_policy" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "ice_transport_policy" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ice_transport_policy" , e . code () ) ? , 
        }
        match self . metadata_enabled () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "metadata_enabled" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "metadata_enabled" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "metadata_enabled" , e . code () ) ? , 
        }
        match self . s_frame_config () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "s_frame_config" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "s_frame_config" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "s_frame_config" , e . code () ) ? , 
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
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProvideOfferRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProvideOfferRequest" ) ;
        match self . web_rtc_session_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) , 
        }
        match self . sdp () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "sdp" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sdp" , e . code () ) , 
        }
        match self . stream_usage () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "stream_usage" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "stream_usage" , e . code () ) , 
        }
        match self . originating_endpoint_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "originating_endpoint_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "originating_endpoint_id" , e . code () ) , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "video_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "audio_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        match self . ice_servers () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "ice_servers" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "ice_servers" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ice_servers" , e . code () ) , 
        }
        match self . ice_transport_policy () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "ice_transport_policy" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "ice_transport_policy" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ice_transport_policy" , e . code () ) , 
        }
        match self . metadata_enabled () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "metadata_enabled" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "metadata_enabled" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "metadata_enabled" , e . code () ) , 
        }
        match self . s_frame_config () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "s_frame_config" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "s_frame_config" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "s_frame_config" , e . code () ) , 
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
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProvideOfferResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProvideOfferResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn web_rtc_session_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn video_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn audio_stream_id (& self ) -> Result < Option < crate :: tlv :: Nullable < u16 > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProvideOfferResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProvideOfferResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProvideOfferResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProvideOfferResponse" ) ? ;
        match self . web_rtc_session_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) ? , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "video_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) ? , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "audio_stream_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProvideOfferResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProvideOfferResponse" ) ;
        match self . web_rtc_session_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) , 
        }
        match self . video_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "video_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "video_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "video_stream_id" , e . code () ) , 
        }
        match self . audio_stream_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "audio_stream_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "audio_stream_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "audio_stream_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProvideAnswerRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProvideAnswerRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn web_rtc_session_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn sdp (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProvideAnswerRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProvideAnswerRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProvideAnswerRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProvideAnswerRequest" ) ? ;
        match self . web_rtc_session_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) ? , 
        }
        match self . sdp () {
            Ok (value ) => write ! (f , "{}: {:?}," , "sdp" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "sdp" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProvideAnswerRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProvideAnswerRequest" ) ;
        match self . web_rtc_session_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) , 
        }
        match self . sdp () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "sdp" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "sdp" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProvideICECandidatesRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProvideICECandidatesRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn web_rtc_session_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn ice_candidates (& self ) -> Result < crate :: tlv :: TLVArray < '_ , ICECandidateStruct < '_ > > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProvideICECandidatesRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProvideICECandidatesRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProvideICECandidatesRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProvideICECandidatesRequest" ) ? ;
        match self . web_rtc_session_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) ? , 
        }
        match self . ice_candidates () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ice_candidates" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ice_candidates" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProvideICECandidatesRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProvideICECandidatesRequest" ) ;
        match self . web_rtc_session_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) , 
        }
        match self . ice_candidates () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ice_candidates" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ice_candidates" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct EndSessionRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > EndSessionRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn web_rtc_session_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn reason (& self ) -> Result < WebRTCEndReasonEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for EndSessionRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for EndSessionRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for EndSessionRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "EndSessionRequest" ) ? ;
        match self . web_rtc_session_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) ? , 
        }
        match self . reason () {
            Ok (value ) => write ! (f , "{}: {:?}," , "reason" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "reason" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for EndSessionRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "EndSessionRequest" ) ;
        match self . web_rtc_session_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "web_rtc_session_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "web_rtc_session_id" , e . code () ) , 
        }
        match self . reason () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "reason" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "reason" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct SFrameStructBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SFrameStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SFrameStructBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn cipher_suite (mut self , value : u16 ) -> Result < SFrameStructBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "cipherSuite" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SFrameStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SFrameStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn base_key (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < SFrameStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "baseKey" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SFrameStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SFrameStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn kid (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < SFrameStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "kid" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SFrameStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > SFrameStructBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SFrameStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SFrameStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SFrameStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SFrameStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SFrameStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SFrameStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SFrameStructArrayBuilder < P > (P ) ;
impl < P > SFrameStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SFrameStructBuilder < SFrameStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SFrameStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SFrameStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SFrameStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SFrameStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SFrameStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SFrameStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SFrameStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SolicitOfferRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SolicitOfferRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_usage (mut self , value : StreamUsageEnum ) -> Result < SolicitOfferRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "streamUsage" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SolicitOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn originating_endpoint_id (mut self , value : crate :: im :: EndptId ) -> Result < SolicitOfferRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "originatingEndpointID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SolicitOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < SolicitOfferRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SolicitOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < SolicitOfferRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (SolicitOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ice_servers (self ) -> Result < crate :: tlv :: OptionalBuilder < SolicitOfferRequestBuilder < P , 5usize > , ICEServerStructArrayBuilder < SolicitOfferRequestBuilder < P , 5usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SolicitOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (4 ) , ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ice_transport_policy (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < SolicitOfferRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ICETransportPolicy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (SolicitOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn metadata_enabled (mut self , value : Option < bool > ) -> Result < SolicitOfferRequestBuilder < P , 7usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "metadataEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (SolicitOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn s_frame_config (self ) -> Result < crate :: tlv :: OptionalBuilder < SolicitOfferRequestBuilder < P , 8usize > , SFrameStructBuilder < SolicitOfferRequestBuilder < P , 8usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SolicitOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (7 ) , ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_streams (self ) -> Result < crate :: tlv :: OptionalBuilder < SolicitOfferRequestBuilder < P , 9usize > , crate :: tlv :: ToTLVArrayBuilder < SolicitOfferRequestBuilder < P , 9usize > , u16 > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SolicitOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (8 ) , ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_streams (self ) -> Result < crate :: tlv :: OptionalBuilder < SolicitOfferRequestBuilder < P , 10usize > , crate :: tlv :: ToTLVArrayBuilder < SolicitOfferRequestBuilder < P , 10usize > , u16 > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SolicitOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (9 ) , ) 
    }
    
}
impl < P > SolicitOfferRequestBuilder < P , 10usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SolicitOfferRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SolicitOfferRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SolicitOfferRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SolicitOfferRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SolicitOfferRequestArrayBuilder < P > (P ) ;
impl < P > SolicitOfferRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SolicitOfferRequestBuilder < SolicitOfferRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SolicitOfferRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SolicitOfferRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SolicitOfferRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SolicitOfferRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SolicitOfferRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SolicitOfferResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SolicitOfferResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SolicitOfferResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn web_rtc_session_id (mut self , value : u16 ) -> Result < SolicitOfferResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "webRTCSessionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SolicitOfferResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn deferred_offer (mut self , value : bool ) -> Result < SolicitOfferResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "deferredOffer" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SolicitOfferResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < SolicitOfferResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SolicitOfferResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferResponseBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < SolicitOfferResponseBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (SolicitOfferResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SolicitOfferResponseBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SolicitOfferResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SolicitOfferResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SolicitOfferResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SolicitOfferResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SolicitOfferResponseArrayBuilder < P > (P ) ;
impl < P > SolicitOfferResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SolicitOfferResponseBuilder < SolicitOfferResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SolicitOfferResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SolicitOfferResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SolicitOfferResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SolicitOfferResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SolicitOfferResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SolicitOfferResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideOfferRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProvideOfferRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn web_rtc_session_id (mut self , value : crate :: tlv :: Nullable < u16 > ) -> Result < ProvideOfferRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "webRTCSessionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sdp (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ProvideOfferRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sdp" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn stream_usage (mut self , value : StreamUsageEnum ) -> Result < ProvideOfferRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "streamUsage" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn originating_endpoint_id (mut self , value : crate :: im :: EndptId ) -> Result < ProvideOfferRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "originatingEndpointID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < ProvideOfferRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < ProvideOfferRequestBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ice_servers (self ) -> Result < crate :: tlv :: OptionalBuilder < ProvideOfferRequestBuilder < P , 7usize > , ICEServerStructArrayBuilder < ProvideOfferRequestBuilder < P , 7usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (6 ) , ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 7 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ice_transport_policy (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < ProvideOfferRequestBuilder < P , 8usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ICETransportPolicy" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 7 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 8 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn metadata_enabled (mut self , value : Option < bool > ) -> Result < ProvideOfferRequestBuilder < P , 9usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "metadataEnabled" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 8 , & value ) ? ;
        Ok (ProvideOfferRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 9 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn s_frame_config (self ) -> Result < crate :: tlv :: OptionalBuilder < ProvideOfferRequestBuilder < P , 10usize > , SFrameStructBuilder < ProvideOfferRequestBuilder < P , 10usize > > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (9 ) , ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 10 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_streams (self ) -> Result < crate :: tlv :: OptionalBuilder < ProvideOfferRequestBuilder < P , 11usize > , crate :: tlv :: ToTLVArrayBuilder < ProvideOfferRequestBuilder < P , 11usize > , u16 > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (10 ) , ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 11 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_streams (self ) -> Result < crate :: tlv :: OptionalBuilder < ProvideOfferRequestBuilder < P , 12usize > , crate :: tlv :: ToTLVArrayBuilder < ProvideOfferRequestBuilder < P , 12usize > , u16 > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideOfferRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (11 ) , ) 
    }
    
}
impl < P > ProvideOfferRequestBuilder < P , 12usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProvideOfferRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProvideOfferRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProvideOfferRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideOfferRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideOfferRequestArrayBuilder < P > (P ) ;
impl < P > ProvideOfferRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProvideOfferRequestBuilder < ProvideOfferRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideOfferRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProvideOfferRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProvideOfferRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProvideOfferRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideOfferRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideOfferResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProvideOfferResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProvideOfferResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn web_rtc_session_id (mut self , value : u16 ) -> Result < ProvideOfferResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "webRTCSessionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ProvideOfferResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn video_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < ProvideOfferResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "videoStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ProvideOfferResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn audio_stream_id (mut self , value : Option < crate :: tlv :: Nullable < u16 > > ) -> Result < ProvideOfferResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "audioStreamID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ProvideOfferResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideOfferResponseBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProvideOfferResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProvideOfferResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProvideOfferResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideOfferResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideOfferResponseArrayBuilder < P > (P ) ;
impl < P > ProvideOfferResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProvideOfferResponseBuilder < ProvideOfferResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideOfferResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProvideOfferResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProvideOfferResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideOfferResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProvideOfferResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideOfferResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideAnswerRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProvideAnswerRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProvideAnswerRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn web_rtc_session_id (mut self , value : u16 ) -> Result < ProvideAnswerRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "webRTCSessionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ProvideAnswerRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideAnswerRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn sdp (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < ProvideAnswerRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "sdp" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ProvideAnswerRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideAnswerRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProvideAnswerRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideAnswerRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProvideAnswerRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideAnswerRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProvideAnswerRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideAnswerRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideAnswerRequestArrayBuilder < P > (P ) ;
impl < P > ProvideAnswerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProvideAnswerRequestBuilder < ProvideAnswerRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideAnswerRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProvideAnswerRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideAnswerRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProvideAnswerRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideAnswerRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProvideAnswerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideAnswerRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideICECandidatesRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > ProvideICECandidatesRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProvideICECandidatesRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn web_rtc_session_id (mut self , value : u16 ) -> Result < ProvideICECandidatesRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "webRTCSessionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (ProvideICECandidatesRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProvideICECandidatesRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ice_candidates (self ) -> Result < ICECandidateStructArrayBuilder < ProvideICECandidatesRequestBuilder < P , 2usize > > , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideICECandidatesRequestBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Context (1 ) , ) 
    }
    
}
impl < P > ProvideICECandidatesRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProvideICECandidatesRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideICECandidatesRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProvideICECandidatesRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideICECandidatesRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProvideICECandidatesRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideICECandidatesRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProvideICECandidatesRequestArrayBuilder < P > (P ) ;
impl < P > ProvideICECandidatesRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProvideICECandidatesRequestBuilder < ProvideICECandidatesRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProvideICECandidatesRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProvideICECandidatesRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProvideICECandidatesRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProvideICECandidatesRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProvideICECandidatesRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProvideICECandidatesRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProvideICECandidatesRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EndSessionRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > EndSessionRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > EndSessionRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn web_rtc_session_id (mut self , value : u16 ) -> Result < EndSessionRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "webRTCSessionID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (EndSessionRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EndSessionRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn reason (mut self , value : WebRTCEndReasonEnum ) -> Result < EndSessionRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "reason" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (EndSessionRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > EndSessionRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for EndSessionRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EndSessionRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for EndSessionRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EndSessionRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for EndSessionRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EndSessionRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct EndSessionRequestArrayBuilder < P > (P ) ;
impl < P > EndSessionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < EndSessionRequestBuilder < EndSessionRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (EndSessionRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for EndSessionRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "EndSessionRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for EndSessionRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "EndSessionRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for EndSessionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for EndSessionRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    CurrentSessions = 0 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: CurrentSessions => write ! (f , "{}(0x{:02x})" , "CurrentSessions" , AttributeId :: CurrentSessions as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: CurrentSessions => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentSessions" , AttributeId :: CurrentSessions as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    SolicitOffer = 0 , ProvideOffer = 2 , ProvideAnswer = 4 , ProvideICECandidates = 5 , EndSession = 6 
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
            CommandId :: SolicitOffer => write ! (f , "{}(0x{:02x})" , "SolicitOffer" , CommandId :: SolicitOffer as u32 ) ? , CommandId :: ProvideOffer => write ! (f , "{}(0x{:02x})" , "ProvideOffer" , CommandId :: ProvideOffer as u32 ) ? , CommandId :: ProvideAnswer => write ! (f , "{}(0x{:02x})" , "ProvideAnswer" , CommandId :: ProvideAnswer as u32 ) ? , CommandId :: ProvideICECandidates => write ! (f , "{}(0x{:02x})" , "ProvideICECandidates" , CommandId :: ProvideICECandidates as u32 ) ? , CommandId :: EndSession => write ! (f , "{}(0x{:02x})" , "EndSession" , CommandId :: EndSession as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: SolicitOffer => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SolicitOffer" , CommandId :: SolicitOffer as u32 ) , CommandId :: ProvideOffer => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ProvideOffer" , CommandId :: ProvideOffer as u32 ) , CommandId :: ProvideAnswer => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ProvideAnswer" , CommandId :: ProvideAnswer as u32 ) , CommandId :: ProvideICECandidates => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ProvideICECandidates" , CommandId :: ProvideICECandidates as u32 ) , CommandId :: EndSession => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "EndSession" , CommandId :: EndSession as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    SolicitOfferResponse = 1 , ProvideOfferResponse = 3 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: SolicitOfferResponse => write ! (f , "{}(0x{:02x})" , "SolicitOfferResponse" , CommandResponseId :: SolicitOfferResponse as u32 ) ? , CommandResponseId :: ProvideOfferResponse => write ! (f , "{}(0x{:02x})" , "ProvideOfferResponse" , CommandResponseId :: ProvideOfferResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: SolicitOfferResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SolicitOfferResponse" , CommandResponseId :: SolicitOfferResponse as u32 ) , CommandResponseId :: ProvideOfferResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ProvideOfferResponse" , CommandResponseId :: ProvideOfferResponse as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (1363 , 2 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: CurrentSessions as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_MANAGE . union (crate :: dm :: Access :: NEED_ADMIN ) ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: SolicitOffer as _ , Some (CommandResponseId :: SolicitOfferResponse as _ ) , crate :: dm :: Access :: WO . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: ProvideOffer as _ , Some (CommandResponseId :: ProvideOfferResponse as _ ) , crate :: dm :: Access :: WO . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: ProvideAnswer as _ , None , crate :: dm :: Access :: WO . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: ProvideICECandidates as _ , None , crate :: dm :: Access :: WO . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: EndSession as _ , None , crate :: dm :: Access :: WO . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn current_sessions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < WebRTCSessionStructArrayBuilder < P > , WebRTCSessionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn handle_solicit_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SolicitOfferRequest < '_ > , response : SolicitOfferResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_provide_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideOfferRequest < '_ > , response : ProvideOfferResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_provide_answer (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideAnswerRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_provide_ice_candidates (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideICECandidatesRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_end_session (& self , ctx : impl crate :: dm :: InvokeContext , request : EndSessionRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn current_sessions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < WebRTCSessionStructArrayBuilder < P > , WebRTCSessionStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: current_sessions (self , ctx , builder ) 
    }
    fn handle_solicit_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SolicitOfferRequest < '_ > , response : SolicitOfferResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_solicit_offer (self , ctx , request , response ) 
    }
    fn handle_provide_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideOfferRequest < '_ > , response : ProvideOfferResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_provide_offer (self , ctx , request , response ) 
    }
    fn handle_provide_answer (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideAnswerRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_provide_answer (self , ctx , request ) 
    }
    fn handle_provide_ice_candidates (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideICECandidatesRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_provide_ice_candidates (self , ctx , request ) 
    }
    fn handle_end_session (& self , ctx : impl crate :: dm :: InvokeContext , request : EndSessionRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_end_session (self , ctx , request ) 
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
                    AttributeId :: CurrentSessions => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSessions , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_sessions (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSessions , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSessions , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
            CommandId :: SolicitOffer => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SolicitOffer ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_solicit_offer (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SolicitOffer ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SolicitOffer ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ProvideOffer => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideOffer ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (3u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_provide_offer (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideOffer ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideOffer ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ProvideAnswer => {
                let cmd_data : ProvideAnswerRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_provide_answer (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideAnswer ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ProvideICECandidates => {
                let cmd_data : ProvideICECandidatesRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_provide_ice_candidates (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideICECandidates ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EndSession => {
                let cmd_data : EndSessionRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_end_session (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EndSession ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 1363u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "WebRTCTransportProvider" , 1363u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "WebRTCTransportProvider" , 1363u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `WebRTCTransportProvider` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct WebRtcTransportProviderCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > WebRtcTransportProviderCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn solicit_offer (self , endpoint : crate :: dm :: EndptId , ) -> Result < SolicitOfferRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1363 , 0 ) ? . data_builder () 
    }
    pub fn provide_offer (self , endpoint : crate :: dm :: EndptId , ) -> Result < ProvideOfferRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1363 , 2 ) ? . data_builder () 
    }
    pub fn provide_answer (self , endpoint : crate :: dm :: EndptId , ) -> Result < ProvideAnswerRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1363 , 4 ) ? . data_builder () 
    }
    pub fn provide_ice_candidates (self , endpoint : crate :: dm :: EndptId , ) -> Result < ProvideICECandidatesRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1363 , 5 ) ? . data_builder () 
    }
    pub fn end_session (self , endpoint : crate :: dm :: EndptId , ) -> Result < EndSessionRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 1363 , 6 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `WebRTCTransportProvider` cluster's commands. `use` this trait to call `.web_rtc_transport_provider_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`WebRtcTransportProviderCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait WebRtcTransportProviderCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn web_rtc_transport_provider_inv (self ) -> WebRtcTransportProviderCmdRequestsView < P > ;
    
}
impl < P > WebRtcTransportProviderCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn web_rtc_transport_provider_inv (self ) -> WebRtcTransportProviderCmdRequestsView < P > {
        WebRtcTransportProviderCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `WebRTCTransportProvider` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct WebRtcTransportProviderAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > WebRtcTransportProviderAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn current_sessions (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1363 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1363 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1363 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1363 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1363 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (1363 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `WebRTCTransportProvider` cluster's attribute reads. `use` this trait to call `.web_rtc_transport_provider_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`WebRtcTransportProviderAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait WebRtcTransportProviderAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn web_rtc_transport_provider_read (self ) -> WebRtcTransportProviderAttrReadsView < P > ;
    
}
impl < P > WebRtcTransportProviderAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn web_rtc_transport_provider_read (self ) -> WebRtcTransportProviderAttrReadsView < P > {
        WebRtcTransportProviderAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `WebRTCTransportProvider` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct WebRtcTransportProviderAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > WebRtcTransportProviderAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `WebRTCTransportProvider` cluster's attribute writes. `use` this trait to call `.web_rtc_transport_provider_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`WebRtcTransportProviderAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait WebRtcTransportProviderAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn web_rtc_transport_provider_write (self ) -> WebRtcTransportProviderAttrWritesView < P > ;
    
}
impl < P > WebRtcTransportProviderAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn web_rtc_transport_provider_write (self ) -> WebRtcTransportProviderAttrWritesView < P > {
        WebRtcTransportProviderAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `WebRTCTransportProvider` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct WebRtcTransportProviderCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > WebRtcTransportProviderCmdResponsesView < 'a , 'r > {
    pub fn solicit_offer (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < SolicitOfferResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < SolicitOfferResponse < 'a >> (1363 , 0 ) 
    }
    pub fn provide_offer (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < ProvideOfferResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < ProvideOfferResponse < 'a >> (1363 , 2 ) 
    }
    pub fn provide_answer (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1363 , 4 ) 
    }
    pub fn provide_ice_candidates (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1363 , 5 ) 
    }
    pub fn end_session (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (1363 , 6 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `WebRTCTransportProvider`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.web_rtc_transport_provider_inv_resp()` on an `InvokeResp`; the returned [`WebRtcTransportProviderCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait WebRtcTransportProviderCmdResponses < 'a > {
    fn web_rtc_transport_provider_inv_resp (& self ) -> WebRtcTransportProviderCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > WebRtcTransportProviderCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn web_rtc_transport_provider_inv_resp (& self ) -> WebRtcTransportProviderCmdResponsesView < 'a , '_ > {
        WebRtcTransportProviderCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `WebRTCTransportProvider` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct WebRtcTransportProviderAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > WebRtcTransportProviderAttrResponsesView < 'a , 'r > {
    pub fn current_sessions (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , WebRTCSessionStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , WebRTCSessionStruct < 'a > > > (1363 , 0 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1363 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (1363 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (1363 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (1363 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (1363 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `WebRTCTransportProvider`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.web_rtc_transport_provider_read_resp()` on a `ReportDataResp`; the returned [`WebRtcTransportProviderAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait WebRtcTransportProviderAttrResponses < 'a > {
    fn web_rtc_transport_provider_read_resp (& self ) -> WebRtcTransportProviderAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > WebRtcTransportProviderAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn web_rtc_transport_provider_read_resp (& self ) -> WebRtcTransportProviderAttrResponsesView < 'a , '_ > {
        WebRtcTransportProviderAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `WebRTCTransportProvider` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct WebRtcTransportProviderWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > WebRtcTransportProviderWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `WebRTCTransportProvider`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.web_rtc_transport_provider_write_resp()` on a `WriteResp`; the returned [`WebRtcTransportProviderWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait WebRtcTransportProviderWriteResponses < 'a > {
    fn web_rtc_transport_provider_write_resp (& self ) -> WebRtcTransportProviderWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > WebRtcTransportProviderWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn web_rtc_transport_provider_write_resp (& self ) -> WebRtcTransportProviderWriteResponsesView < 'a , '_ > {
        WebRtcTransportProviderWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `SolicitOfferResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`SolicitOfferResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct SolicitOfferResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > SolicitOfferResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < SolicitOfferResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `ProvideOfferResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`ProvideOfferResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct ProvideOfferResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > ProvideOfferResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < ProvideOfferResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `WebRTCTransportProvider` cluster's client operations. Returned by [`WebRtcTransportProviderClient::web_rtc_transport_provider`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct WebRtcTransportProviderClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > WebRtcTransportProviderClientView < T > {
    pub async fn solicit_offer < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < SolicitOfferResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (SolicitOfferRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: WebRtcTransportProviderCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . web_rtc_transport_provider_inv () ;
            let req_builder = view . solicit_offer (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (SolicitOfferResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn provide_offer < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < ProvideOfferResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (ProvideOfferRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: WebRtcTransportProviderCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . web_rtc_transport_provider_inv () ;
            let req_builder = view . provide_offer (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (ProvideOfferResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn provide_answer < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (ProvideAnswerRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: WebRtcTransportProviderCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . web_rtc_transport_provider_inv () ;
            let req_builder = view . provide_answer (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn provide_ice_candidates < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (ProvideICECandidatesRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: WebRtcTransportProviderCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . web_rtc_transport_provider_inv () ;
            let req_builder = view . provide_ice_candidates (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn end_session < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (EndSessionRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: WebRtcTransportProviderCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . web_rtc_transport_provider_inv () ;
            let req_builder = view . end_session (endpoint ) ? ;
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
                         attribute surfaces as `Err` to `f`." ] pub fn current_sessions_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , WebRTCSessionStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: WebRtcTransportProviderAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . web_rtc_transport_provider_read () . current_sessions (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: WebRtcTransportProviderAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . web_rtc_transport_provider_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: WebRtcTransportProviderAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . web_rtc_transport_provider_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: WebRtcTransportProviderAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . web_rtc_transport_provider_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: WebRtcTransportProviderAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . web_rtc_transport_provider_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: WebRtcTransportProviderAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . web_rtc_transport_provider_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `WebRTCTransportProvider` cluster. `use` this trait to call `.web_rtc_transport_provider()` on any [`crate::im::ImClient`], such as an exchange; the returned [`WebRtcTransportProviderClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `WebRtcTransportProviderClientView` keeps each cluster's API surface narrow: at the call site `exchange.web_rtc_transport_provider().` shows only this cluster's methods in IDE completion." ] pub trait WebRtcTransportProviderClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn web_rtc_transport_provider (self ) -> WebRtcTransportProviderClientView < Self > {
        WebRtcTransportProviderClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > WebRtcTransportProviderClient for T {
    
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
    fn current_sessions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < WebRTCSessionStructArrayBuilder < P > , WebRTCSessionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_solicit_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SolicitOfferRequest < '_ > , response : SolicitOfferResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_provide_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideOfferRequest < '_ > , response : ProvideOfferResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_provide_answer (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideAnswerRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_provide_ice_candidates (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideICECandidatesRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_end_session (& self , ctx : impl crate :: dm :: InvokeContext , request : EndSessionRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn current_sessions < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < WebRTCSessionStructArrayBuilder < P > , WebRTCSessionStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: current_sessions (self , ctx , builder ) 
    }
    fn handle_solicit_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SolicitOfferRequest < '_ > , response : SolicitOfferResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_solicit_offer (self , ctx , request , response ) 
    }
    fn handle_provide_offer < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideOfferRequest < '_ > , response : ProvideOfferResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_provide_offer (self , ctx , request , response ) 
    }
    fn handle_provide_answer (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideAnswerRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_provide_answer (self , ctx , request ) 
    }
    fn handle_provide_ice_candidates (& self , ctx : impl crate :: dm :: InvokeContext , request : ProvideICECandidatesRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_provide_ice_candidates (self , ctx , request ) 
    }
    fn handle_end_session (& self , ctx : impl crate :: dm :: InvokeContext , request : EndSessionRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_end_session (self , ctx , request ) 
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
                    AttributeId :: CurrentSessions => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSessions , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. current_sessions (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSessions , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentSessions , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
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
            CommandId :: SolicitOffer => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SolicitOffer ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_solicit_offer (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SolicitOffer ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SolicitOffer ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ProvideOffer => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideOffer ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (3u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_provide_offer (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideOffer ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideOffer ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: ProvideAnswer => {
                let cmd_data : ProvideAnswerRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_provide_answer (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideAnswer ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: ProvideICECandidates => {
                let cmd_data : ProvideICECandidatesRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_provide_ice_candidates (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: ProvideICECandidates ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: EndSession => {
                let cmd_data : EndSessionRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_end_session (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: EndSession ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 1363u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "WebRTCTransportProvider" , 1363u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "WebRTCTransportProvider" , 1363u32 , self . 0.2 ) 
    }
    
}
