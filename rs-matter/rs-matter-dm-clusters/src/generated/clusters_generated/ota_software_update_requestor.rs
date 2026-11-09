# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AnnouncementReasonEnum {
    # [enumval (0 ) ] SimpleAnnouncement = 0 , # [enumval (1 ) ] UpdateAvailable = 1 , # [enumval (2 ) ] UrgentUpdateAvailable = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ChangeReasonEnum {
    # [enumval (0 ) ] Unknown = 0 , # [enumval (1 ) ] Success = 1 , # [enumval (2 ) ] Failure = 2 , # [enumval (3 ) ] TimeOut = 3 , # [enumval (4 ) ] DelayByProvider = 4 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum UpdateStateEnum {
    # [enumval (0 ) ] Unknown = 0 , # [enumval (1 ) ] Idle = 1 , # [enumval (2 ) ] Querying = 2 , # [enumval (3 ) ] DelayedOnQuery = 3 , # [enumval (4 ) ] Downloading = 4 , # [enumval (5 ) ] Applying = 5 , # [enumval (6 ) ] DelayedOnApply = 6 , # [enumval (7 ) ] RollingBack = 7 , # [enumval (8 ) ] DelayedOnUserConsent = 8 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum ProviderLocationTag {
    ProviderNodeId = 1 , Endpoint = 2 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AnnounceOTAProviderRequestTag {
    ProviderNodeId = 0 , VendorId = 1 , AnnouncementReason = 2 , MetadataForNode = 3 , Endpoint = 4 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct ProviderLocation < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > ProviderLocation < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn provider_node_id (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn endpoint (& self ) -> Result < crate :: im :: EndptId , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for ProviderLocation < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for ProviderLocation < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for ProviderLocation < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "ProviderLocation" ) ? ;
        match self . provider_node_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "provider_node_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "provider_node_id" , e . code () ) ? , 
        }
        match self . endpoint () {
            Ok (value ) => write ! (f , "{}: {:?}," , "endpoint" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoint" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for ProviderLocation < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "ProviderLocation" ) ;
        match self . provider_node_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "provider_node_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "provider_node_id" , e . code () ) , 
        }
        match self . endpoint () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "endpoint" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoint" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AnnounceOTAProviderRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AnnounceOTAProviderRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn provider_node_id (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn vendor_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn announcement_reason (& self ) -> Result < AnnouncementReasonEnum , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn metadata_for_node (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn endpoint (& self ) -> Result < crate :: im :: EndptId , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AnnounceOTAProviderRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AnnounceOTAProviderRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AnnounceOTAProviderRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AnnounceOTAProviderRequest" ) ? ;
        match self . provider_node_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "provider_node_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "provider_node_id" , e . code () ) ? , 
        }
        match self . vendor_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "vendor_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "vendor_id" , e . code () ) ? , 
        }
        match self . announcement_reason () {
            Ok (value ) => write ! (f , "{}: {:?}," , "announcement_reason" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "announcement_reason" , e . code () ) ? , 
        }
        match self . metadata_for_node () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "metadata_for_node" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "metadata_for_node" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "metadata_for_node" , e . code () ) ? , 
        }
        match self . endpoint () {
            Ok (value ) => write ! (f , "{}: {:?}," , "endpoint" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "endpoint" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AnnounceOTAProviderRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AnnounceOTAProviderRequest" ) ;
        match self . provider_node_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "provider_node_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "provider_node_id" , e . code () ) , 
        }
        match self . vendor_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "vendor_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "vendor_id" , e . code () ) , 
        }
        match self . announcement_reason () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "announcement_reason" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "announcement_reason" , e . code () ) , 
        }
        match self . metadata_for_node () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "metadata_for_node" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "metadata_for_node" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "metadata_for_node" , e . code () ) , 
        }
        match self . endpoint () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "endpoint" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "endpoint" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct ProviderLocationBuilder < P , const F : usize = 1usize > (P ) ;
impl < P > ProviderLocationBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > ProviderLocationBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn provider_node_id (mut self , value : u64 ) -> Result < ProviderLocationBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "providerNodeID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (ProviderLocationBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProviderLocationBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoint (mut self , value : crate :: im :: EndptId ) -> Result < ProviderLocationBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "endpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (ProviderLocationBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProviderLocationBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < ProviderLocationBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (ProviderLocationBuilder (self . 0 ) ) 
    }
    
}
impl < P > ProviderLocationBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for ProviderLocationBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProviderLocation" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for ProviderLocationBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProviderLocation" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for ProviderLocationBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProviderLocationBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct ProviderLocationArrayBuilder < P > (P ) ;
impl < P > ProviderLocationArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < ProviderLocationBuilder < ProviderLocationArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (ProviderLocationArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for ProviderLocationArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "ProviderLocation[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for ProviderLocationArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "ProviderLocation[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for ProviderLocationArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for ProviderLocationArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AnnounceOTAProviderRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AnnounceOTAProviderRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AnnounceOTAProviderRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn provider_node_id (mut self , value : u64 ) -> Result < AnnounceOTAProviderRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "providerNodeID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AnnounceOTAProviderRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AnnounceOTAProviderRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn vendor_id (mut self , value : u16 ) -> Result < AnnounceOTAProviderRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "vendorID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AnnounceOTAProviderRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AnnounceOTAProviderRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn announcement_reason (mut self , value : AnnouncementReasonEnum ) -> Result < AnnounceOTAProviderRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "announcementReason" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AnnounceOTAProviderRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AnnounceOTAProviderRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn metadata_for_node (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < AnnounceOTAProviderRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "metadataForNode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (AnnounceOTAProviderRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AnnounceOTAProviderRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn endpoint (mut self , value : crate :: im :: EndptId ) -> Result < AnnounceOTAProviderRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "endpoint" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (AnnounceOTAProviderRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AnnounceOTAProviderRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AnnounceOTAProviderRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AnnounceOTAProviderRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AnnounceOTAProviderRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AnnounceOTAProviderRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AnnounceOTAProviderRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AnnounceOTAProviderRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AnnounceOTAProviderRequestArrayBuilder < P > (P ) ;
impl < P > AnnounceOTAProviderRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AnnounceOTAProviderRequestBuilder < AnnounceOTAProviderRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AnnounceOTAProviderRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AnnounceOTAProviderRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AnnounceOTAProviderRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AnnounceOTAProviderRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AnnounceOTAProviderRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AnnounceOTAProviderRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AnnounceOTAProviderRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum StateTransitionTag {
    PreviousState = 0 , NewState = 1 , Reason = 2 , TargetSoftwareVersion = 3 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum VersionAppliedTag {
    SoftwareVersion = 0 , ProductId = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum DownloadErrorTag {
    SoftwareVersion = 0 , BytesDownloaded = 1 , ProgressPercent = 2 , PlatformCode = 3 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct StateTransition < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > StateTransition < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn previous_state (& self ) -> Result < UpdateStateEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn new_state (& self ) -> Result < UpdateStateEnum , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn reason (& self ) -> Result < ChangeReasonEnum , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn target_software_version (& self ) -> Result < crate :: tlv :: Nullable < u32 > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for StateTransition < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for StateTransition < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for StateTransition < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "StateTransition" ) ? ;
        match self . previous_state () {
            Ok (value ) => write ! (f , "{}: {:?}," , "previous_state" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "previous_state" , e . code () ) ? , 
        }
        match self . new_state () {
            Ok (value ) => write ! (f , "{}: {:?}," , "new_state" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "new_state" , e . code () ) ? , 
        }
        match self . reason () {
            Ok (value ) => write ! (f , "{}: {:?}," , "reason" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "reason" , e . code () ) ? , 
        }
        match self . target_software_version () {
            Ok (value ) => write ! (f , "{}: {:?}," , "target_software_version" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "target_software_version" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for StateTransition < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "StateTransition" ) ;
        match self . previous_state () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "previous_state" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "previous_state" , e . code () ) , 
        }
        match self . new_state () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "new_state" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "new_state" , e . code () ) , 
        }
        match self . reason () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "reason" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "reason" , e . code () ) , 
        }
        match self . target_software_version () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "target_software_version" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "target_software_version" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct VersionApplied < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > VersionApplied < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn software_version (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn product_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for VersionApplied < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for VersionApplied < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for VersionApplied < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "VersionApplied" ) ? ;
        match self . software_version () {
            Ok (value ) => write ! (f , "{}: {:?}," , "software_version" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "software_version" , e . code () ) ? , 
        }
        match self . product_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "product_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "product_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for VersionApplied < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "VersionApplied" ) ;
        match self . software_version () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "software_version" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "software_version" , e . code () ) , 
        }
        match self . product_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "product_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "product_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct DownloadError < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > DownloadError < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn software_version (& self ) -> Result < u32 , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn bytes_downloaded (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn progress_percent (& self ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn platform_code (& self ) -> Result < crate :: tlv :: Nullable < i64 > , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for DownloadError < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for DownloadError < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for DownloadError < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "DownloadError" ) ? ;
        match self . software_version () {
            Ok (value ) => write ! (f , "{}: {:?}," , "software_version" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "software_version" , e . code () ) ? , 
        }
        match self . bytes_downloaded () {
            Ok (value ) => write ! (f , "{}: {:?}," , "bytes_downloaded" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "bytes_downloaded" , e . code () ) ? , 
        }
        match self . progress_percent () {
            Ok (value ) => write ! (f , "{}: {:?}," , "progress_percent" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "progress_percent" , e . code () ) ? , 
        }
        match self . platform_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "platform_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "platform_code" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for DownloadError < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "DownloadError" ) ;
        match self . software_version () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "software_version" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "software_version" , e . code () ) , 
        }
        match self . bytes_downloaded () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "bytes_downloaded" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "bytes_downloaded" , e . code () ) , 
        }
        match self . progress_percent () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "progress_percent" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "progress_percent" , e . code () ) , 
        }
        match self . platform_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "platform_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "platform_code" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct StateTransitionBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > StateTransitionBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > StateTransitionBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn previous_state (mut self , value : UpdateStateEnum ) -> Result < StateTransitionBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "previousState" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (StateTransitionBuilder (self . 0 ) ) 
    }
    
}
impl < P > StateTransitionBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn new_state (mut self , value : UpdateStateEnum ) -> Result < StateTransitionBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "newState" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (StateTransitionBuilder (self . 0 ) ) 
    }
    
}
impl < P > StateTransitionBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn reason (mut self , value : ChangeReasonEnum ) -> Result < StateTransitionBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "reason" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (StateTransitionBuilder (self . 0 ) ) 
    }
    
}
impl < P > StateTransitionBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn target_software_version (mut self , value : crate :: tlv :: Nullable < u32 > ) -> Result < StateTransitionBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "targetSoftwareVersion" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (StateTransitionBuilder (self . 0 ) ) 
    }
    
}
impl < P > StateTransitionBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for StateTransitionBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StateTransition" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for StateTransitionBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StateTransition" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for StateTransitionBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StateTransitionBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct StateTransitionArrayBuilder < P > (P ) ;
impl < P > StateTransitionArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < StateTransitionBuilder < StateTransitionArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (StateTransitionArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for StateTransitionArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "StateTransition[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for StateTransitionArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "StateTransition[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for StateTransitionArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for StateTransitionArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VersionAppliedBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > VersionAppliedBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > VersionAppliedBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn software_version (mut self , value : u32 ) -> Result < VersionAppliedBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "softwareVersion" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (VersionAppliedBuilder (self . 0 ) ) 
    }
    
}
impl < P > VersionAppliedBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn product_id (mut self , value : u16 ) -> Result < VersionAppliedBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "productID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (VersionAppliedBuilder (self . 0 ) ) 
    }
    
}
impl < P > VersionAppliedBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for VersionAppliedBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VersionApplied" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for VersionAppliedBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VersionApplied" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for VersionAppliedBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VersionAppliedBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct VersionAppliedArrayBuilder < P > (P ) ;
impl < P > VersionAppliedArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < VersionAppliedBuilder < VersionAppliedArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (VersionAppliedArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for VersionAppliedArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "VersionApplied[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for VersionAppliedArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "VersionApplied[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for VersionAppliedArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for VersionAppliedArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct DownloadErrorBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > DownloadErrorBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > DownloadErrorBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn software_version (mut self , value : u32 ) -> Result < DownloadErrorBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "softwareVersion" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (DownloadErrorBuilder (self . 0 ) ) 
    }
    
}
impl < P > DownloadErrorBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn bytes_downloaded (mut self , value : u64 ) -> Result < DownloadErrorBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "bytesDownloaded" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (DownloadErrorBuilder (self . 0 ) ) 
    }
    
}
impl < P > DownloadErrorBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn progress_percent (mut self , value : crate :: tlv :: Nullable < u8 > ) -> Result < DownloadErrorBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "progressPercent" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (DownloadErrorBuilder (self . 0 ) ) 
    }
    
}
impl < P > DownloadErrorBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn platform_code (mut self , value : crate :: tlv :: Nullable < i64 > ) -> Result < DownloadErrorBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "platformCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (DownloadErrorBuilder (self . 0 ) ) 
    }
    
}
impl < P > DownloadErrorBuilder < P , 4usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for DownloadErrorBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "DownloadError" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for DownloadErrorBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "DownloadError" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for DownloadErrorBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for DownloadErrorBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct DownloadErrorArrayBuilder < P > (P ) ;
impl < P > DownloadErrorArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < DownloadErrorBuilder < DownloadErrorArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (DownloadErrorArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for DownloadErrorArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "DownloadError[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for DownloadErrorArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "DownloadError[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for DownloadErrorArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for DownloadErrorArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
impl StateTransition < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (StateTransitionBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 42 , EventId :: StateTransition as _ , Self :: PRIORITY , | tw | {
            f (StateTransitionBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (StateTransitionBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: StateTransition as _ , Self :: PRIORITY , | tw | {
            f (StateTransitionBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl VersionApplied < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Critical ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (VersionAppliedBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 42 , EventId :: VersionApplied as _ , Self :: PRIORITY , | tw | {
            f (VersionAppliedBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (VersionAppliedBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: VersionApplied as _ , Self :: PRIORITY , | tw | {
            f (VersionAppliedBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
impl DownloadError < '_ > {
    const PRIORITY : crate :: im :: EventPriority = crate :: im :: EventPriority :: Info ;
    pub fn emit_for < F > (emitter : impl crate :: dm :: EventEmitter , endpoint_id : crate :: dm :: EndptId , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (DownloadErrorBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_event (endpoint_id , 42 , EventId :: DownloadError as _ , Self :: PRIORITY , | tw | {
            f (DownloadErrorBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    pub fn emit < F > (emitter : impl crate :: dm :: OwnEventEmitter , f : F ) -> Result < crate :: dm :: EventNumber , crate :: error :: Error > where F : FnOnce (DownloadErrorBuilder < crate :: dm :: EventTLVWrite < '_ >> ) -> Result < crate :: dm :: EventTLVWrite < '_ > , crate :: error :: Error > , {
        emitter . emit_own_event (EventId :: DownloadError as _ , Self :: PRIORITY , | tw | {
            f (DownloadErrorBuilder :: new (tw , & crate :: dm :: EVENT_DATA_TAG ) ? ) ? ;
            Ok (() ) 
        }
        , ) 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    DefaultOTAProviders = 0 , UpdatePossible = 1 , UpdateState = 2 , UpdateStateProgress = 3 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: DefaultOTAProviders => write ! (f , "{}(0x{:02x})" , "DefaultOTAProviders" , AttributeId :: DefaultOTAProviders as u32 ) ? , AttributeId :: UpdatePossible => write ! (f , "{}(0x{:02x})" , "UpdatePossible" , AttributeId :: UpdatePossible as u32 ) ? , AttributeId :: UpdateState => write ! (f , "{}(0x{:02x})" , "UpdateState" , AttributeId :: UpdateState as u32 ) ? , AttributeId :: UpdateStateProgress => write ! (f , "{}(0x{:02x})" , "UpdateStateProgress" , AttributeId :: UpdateStateProgress as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: DefaultOTAProviders => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DefaultOTAProviders" , AttributeId :: DefaultOTAProviders as u32 ) , AttributeId :: UpdatePossible => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UpdatePossible" , AttributeId :: UpdatePossible as u32 ) , AttributeId :: UpdateState => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UpdateState" , AttributeId :: UpdateState as u32 ) , AttributeId :: UpdateStateProgress => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UpdateStateProgress" , AttributeId :: UpdateStateProgress as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    AnnounceOTAProvider = 0 
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
            CommandId :: AnnounceOTAProvider => write ! (f , "{}(0x{:02x})" , "AnnounceOTAProvider" , CommandId :: AnnounceOTAProvider as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: AnnounceOTAProvider => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AnnounceOTAProvider" , CommandId :: AnnounceOTAProvider as u32 ) , 
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
    StateTransition = 0 , VersionApplied = 1 , DownloadError = 2 
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
            EventId :: StateTransition => write ! (f , "{}(0x{:02x})" , "StateTransition" , EventId :: StateTransition as u32 ) ? , EventId :: VersionApplied => write ! (f , "{}(0x{:02x})" , "VersionApplied" , EventId :: VersionApplied as u32 ) ? , EventId :: DownloadError => write ! (f , "{}(0x{:02x})" , "DownloadError" , EventId :: DownloadError as u32 ) ? , 
        }
        write ! (f , "::Emit" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < EventId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Event::" ) ;
        match self . 0 {
            EventId :: StateTransition => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "StateTransition" , EventId :: StateTransition as u32 ) , EventId :: VersionApplied => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "VersionApplied" , EventId :: VersionApplied as u32 ) , EventId :: DownloadError => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "DownloadError" , EventId :: DownloadError as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Emit" ) 
    }
    
}
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (42 , 1 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: DefaultOTAProviders as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: WRITE ) . union (crate :: dm :: Access :: NEED_ADMIN . union (crate :: dm :: Access :: NEED_VIEW ) ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: UpdatePossible as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: UpdateState as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: UpdateStateProgress as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: AnnounceOTAProvider as _ , None , crate :: dm :: Access :: WA , ) , ] , & [crate :: dm :: Event :: new (EventId :: StateTransition as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: VersionApplied as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , crate :: dm :: Event :: new (EventId :: DownloadError as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , ) , ] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn default_ota_providers < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ProviderLocationArrayBuilder < P > , ProviderLocationBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn update_possible (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > ;
    fn update_state (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < UpdateStateEnum , crate :: error :: Error > ;
    fn update_state_progress (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > ;
    fn set_default_ota_providers (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ProviderLocation < '_ > > , ProviderLocation < '_ > > ) -> Result < () , crate :: error :: Error > ;
    fn handle_announce_ota_provider (& self , ctx : impl crate :: dm :: InvokeContext , request : AnnounceOTAProviderRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    
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
    fn default_ota_providers < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ProviderLocationArrayBuilder < P > , ProviderLocationBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: default_ota_providers (self , ctx , builder ) 
    }
    fn update_possible (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < bool , crate :: error :: Error > {
        T :: update_possible (self , ctx ) 
    }
    fn update_state (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < UpdateStateEnum , crate :: error :: Error > {
        T :: update_state (self , ctx ) 
    }
    fn update_state_progress (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > {
        T :: update_state_progress (self , ctx ) 
    }
    fn set_default_ota_providers (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ProviderLocation < '_ > > , ProviderLocation < '_ > > ) -> Result < () , crate :: error :: Error > {
        T :: set_default_ota_providers (self , ctx , value ) 
    }
    fn handle_announce_ota_provider (& self , ctx : impl crate :: dm :: InvokeContext , request : AnnounceOTAProviderRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_announce_ota_provider (self , ctx , request ) 
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
                    AttributeId :: DefaultOTAProviders => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. default_ota_providers (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: UpdatePossible => {
                        let attr_read_result = self . 0. update_possible (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpdatePossible , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UpdateState => {
                        let attr_read_result = self . 0. update_state (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpdateState , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UpdateStateProgress => {
                        let attr_read_result = self . 0. update_state_progress (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpdateStateProgress , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: DefaultOTAProviders => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_default_ota_providers (& ctx , attr_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: AnnounceOTAProvider => {
                let cmd_data : AnnounceOTAProviderRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_announce_ota_provider (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AnnounceOTAProvider ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 42u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OtaSoftwareUpdateRequestor" , 42u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OtaSoftwareUpdateRequestor" , 42u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `OtaSoftwareUpdateRequestor` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct OtaSoftwareUpdateRequestorCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > OtaSoftwareUpdateRequestorCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn announce_ota_provider (self , endpoint : crate :: dm :: EndptId , ) -> Result < AnnounceOTAProviderRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 42 , 0 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `OtaSoftwareUpdateRequestor` cluster's commands. `use` this trait to call `.ota_software_update_requestor_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`OtaSoftwareUpdateRequestorCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait OtaSoftwareUpdateRequestorCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn ota_software_update_requestor_inv (self ) -> OtaSoftwareUpdateRequestorCmdRequestsView < P > ;
    
}
impl < P > OtaSoftwareUpdateRequestorCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn ota_software_update_requestor_inv (self ) -> OtaSoftwareUpdateRequestorCmdRequestsView < P > {
        OtaSoftwareUpdateRequestorCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `OtaSoftwareUpdateRequestor` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OtaSoftwareUpdateRequestorAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > OtaSoftwareUpdateRequestorAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn default_ota_providers (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn update_possible (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn update_state (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn update_state_progress (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (42 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `OtaSoftwareUpdateRequestor` cluster's attribute reads. `use` this trait to call `.ota_software_update_requestor_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`OtaSoftwareUpdateRequestorAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait OtaSoftwareUpdateRequestorAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn ota_software_update_requestor_read (self ) -> OtaSoftwareUpdateRequestorAttrReadsView < P > ;
    
}
impl < P > OtaSoftwareUpdateRequestorAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn ota_software_update_requestor_read (self ) -> OtaSoftwareUpdateRequestorAttrReadsView < P > {
        OtaSoftwareUpdateRequestorAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `OtaSoftwareUpdateRequestor` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OtaSoftwareUpdateRequestorAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > OtaSoftwareUpdateRequestorAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn default_ota_providers (self , endpoint : crate :: dm :: EndptId , ) -> Result < ProviderLocationArrayBuilder < crate :: im :: AttrDataBuilder < crate :: im :: AttrDataArrayBuilder < P > , 3 > > , crate :: error :: Error > {
        self . array . push () ? . path (endpoint , 42 , 0 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `OtaSoftwareUpdateRequestor` cluster's attribute writes. `use` this trait to call `.ota_software_update_requestor_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`OtaSoftwareUpdateRequestorAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait OtaSoftwareUpdateRequestorAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn ota_software_update_requestor_write (self ) -> OtaSoftwareUpdateRequestorAttrWritesView < P > ;
    
}
impl < P > OtaSoftwareUpdateRequestorAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn ota_software_update_requestor_write (self ) -> OtaSoftwareUpdateRequestorAttrWritesView < P > {
        OtaSoftwareUpdateRequestorAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `OtaSoftwareUpdateRequestor` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct OtaSoftwareUpdateRequestorCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > OtaSoftwareUpdateRequestorCmdResponsesView < 'a , 'r > {
    pub fn announce_ota_provider (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (42 , 0 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `OtaSoftwareUpdateRequestor`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.ota_software_update_requestor_inv_resp()` on an `InvokeResp`; the returned [`OtaSoftwareUpdateRequestorCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait OtaSoftwareUpdateRequestorCmdResponses < 'a > {
    fn ota_software_update_requestor_inv_resp (& self ) -> OtaSoftwareUpdateRequestorCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > OtaSoftwareUpdateRequestorCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn ota_software_update_requestor_inv_resp (& self ) -> OtaSoftwareUpdateRequestorCmdResponsesView < 'a , '_ > {
        OtaSoftwareUpdateRequestorCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `OtaSoftwareUpdateRequestor` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OtaSoftwareUpdateRequestorAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > OtaSoftwareUpdateRequestorAttrResponsesView < 'a , 'r > {
    pub fn default_ota_providers (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , ProviderLocation < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , ProviderLocation < 'a > > > (42 , 0 ) 
    }
    pub fn update_possible (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < bool , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < bool > (42 , 1 ) 
    }
    pub fn update_state (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < UpdateStateEnum , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < UpdateStateEnum > (42 , 2 ) 
    }
    pub fn update_state_progress (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: Nullable < u8 > > (42 , 3 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (42 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (42 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (42 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (42 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (42 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `OtaSoftwareUpdateRequestor`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.ota_software_update_requestor_read_resp()` on a `ReportDataResp`; the returned [`OtaSoftwareUpdateRequestorAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait OtaSoftwareUpdateRequestorAttrResponses < 'a > {
    fn ota_software_update_requestor_read_resp (& self ) -> OtaSoftwareUpdateRequestorAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > OtaSoftwareUpdateRequestorAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn ota_software_update_requestor_read_resp (& self ) -> OtaSoftwareUpdateRequestorAttrResponsesView < 'a , '_ > {
        OtaSoftwareUpdateRequestorAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `OtaSoftwareUpdateRequestor` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OtaSoftwareUpdateRequestorWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > OtaSoftwareUpdateRequestorWriteResponsesView < 'a , 'r > {
    pub fn default_ota_providers (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (42 , 0 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `OtaSoftwareUpdateRequestor`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.ota_software_update_requestor_write_resp()` on a `WriteResp`; the returned [`OtaSoftwareUpdateRequestorWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait OtaSoftwareUpdateRequestorWriteResponses < 'a > {
    fn ota_software_update_requestor_write_resp (& self ) -> OtaSoftwareUpdateRequestorWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > OtaSoftwareUpdateRequestorWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn ota_software_update_requestor_write_resp (& self ) -> OtaSoftwareUpdateRequestorWriteResponsesView < 'a , '_ > {
        OtaSoftwareUpdateRequestorWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Per-transaction view onto the `OtaSoftwareUpdateRequestor` cluster's client operations. Returned by [`OtaSoftwareUpdateRequestorClient::ota_software_update_requestor`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct OtaSoftwareUpdateRequestorClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > OtaSoftwareUpdateRequestorClientView < T > {
    pub fn announce_ota_provider < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (AnnounceOTAProviderRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OtaSoftwareUpdateRequestorCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . ota_software_update_requestor_inv () ;
            let req_builder = view . announce_ota_provider (endpoint ) ? ;
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
                         attribute surfaces as `Err` to `f`." ] pub fn default_ota_providers_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , ProviderLocation < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . default_ota_providers (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn update_possible_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . update_possible (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn update_state_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < UpdateStateEnum , crate :: error :: Error >> {
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . update_state (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn update_state_progress_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . update_state_progress (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: OtaSoftwareUpdateRequestorAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . ota_software_update_requestor_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `OtaSoftwareUpdateRequestor` cluster. `use` this trait to call `.ota_software_update_requestor()` on any [`crate::im::ImClient`], such as an exchange; the returned [`OtaSoftwareUpdateRequestorClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `OtaSoftwareUpdateRequestorClientView` keeps each cluster's API surface narrow: at the call site `exchange.ota_software_update_requestor().` shows only this cluster's methods in IDE completion." ] pub trait OtaSoftwareUpdateRequestorClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn ota_software_update_requestor (self ) -> OtaSoftwareUpdateRequestorClientView < Self > {
        OtaSoftwareUpdateRequestorClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > OtaSoftwareUpdateRequestorClient for T {
    
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
    fn default_ota_providers < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ProviderLocationArrayBuilder < P > , ProviderLocationBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn update_possible (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn update_state (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < UpdateStateEnum , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn update_state_progress (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn set_default_ota_providers (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ProviderLocation < '_ > > , ProviderLocation < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_announce_ota_provider (& self , ctx : impl crate :: dm :: InvokeContext , request : AnnounceOTAProviderRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn default_ota_providers < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < ProviderLocationArrayBuilder < P > , ProviderLocationBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: default_ota_providers (self , ctx , builder ) 
    }
    fn update_possible (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < bool , crate :: error :: Error >> {
        T :: update_possible (self , ctx ) 
    }
    fn update_state (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < UpdateStateEnum , crate :: error :: Error >> {
        T :: update_state (self , ctx ) 
    }
    fn update_state_progress (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < crate :: tlv :: Nullable < u8 > , crate :: error :: Error >> {
        T :: update_state_progress (self , ctx ) 
    }
    fn set_default_ota_providers (& self , ctx : impl crate :: dm :: WriteContext , value : crate :: dm :: ArrayAttributeWrite < crate :: tlv :: TLVArray < '_ , ProviderLocation < '_ > > , ProviderLocation < '_ > > ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: set_default_ota_providers (self , ctx , value ) 
    }
    fn handle_announce_ota_provider (& self , ctx : impl crate :: dm :: InvokeContext , request : AnnounceOTAProviderRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_announce_ota_provider (self , ctx , request ) 
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
                    AttributeId :: DefaultOTAProviders => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. default_ota_providers (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: UpdatePossible => {
                        let attr_read_result = self . 0. update_possible (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpdatePossible , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UpdateState => {
                        let attr_read_result = self . 0. update_state (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpdateState , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: UpdateStateProgress => {
                        let attr_read_result = self . 0. update_state_progress (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: UpdateStateProgress , false ) ) ) ) , & attr_read_result ) ;
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
            AttributeId :: DefaultOTAProviders => {
                let attr_data = crate :: dm :: ArrayAttributeWrite :: new (ctx . attr () . list_index . clone () , ctx . data () ) ? ;
                let attr_write_result = self . 0. set_default_ota_providers (& ctx , attr_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: DefaultOTAProviders , false ) ) ) ) , & attr_data , & attr_write_result ) ;
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
            CommandId :: AnnounceOTAProvider => {
                let cmd_data : AnnounceOTAProviderRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_announce_ota_provider (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AnnounceOTAProvider ) ) ) , & cmd_data , & cmd_invoke_result ) ;
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
        if ctx . cluster () . map (| c | c == 42u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OtaSoftwareUpdateRequestor" , 42u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OtaSoftwareUpdateRequestor" , 42u32 , self . 0.2 ) 
    }
    
}
