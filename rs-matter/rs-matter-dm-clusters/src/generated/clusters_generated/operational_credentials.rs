# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CertificateChainTypeEnum {
    # [enumval (1 ) ] DACCertificate = 1 , # [enumval (2 ) ] PAICertificate = 2 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , crate :: tlv :: FromTLV , crate :: tlv :: ToTLV ) ] # [tlvargs (datatype = "u8" ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NodeOperationalCertStatusEnum {
    # [enumval (0 ) ] OK = 0 , # [enumval (1 ) ] InvalidPublicKey = 1 , # [enumval (2 ) ] InvalidNodeOpId = 2 , # [enumval (3 ) ] InvalidNOC = 3 , # [enumval (4 ) ] MissingCsr = 4 , # [enumval (5 ) ] TableFull = 5 , # [enumval (6 ) ] InvalidAdminSubject = 6 , # [enumval (9 ) ] FabricConflict = 9 , # [enumval (10 ) ] LabelConflict = 10 , # [enumval (11 ) ] InvalidFabricIndex = 11 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum FabricDescriptorStructTag {
    RootPublicKey = 1 , VendorId = 2 , FabricId = 3 , NodeId = 4 , Label = 5 , VidVerificationStatement = 6 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NOCStructTag {
    Noc = 1 , Icac = 2 , Vvsc = 3 , FabricIndex = 254 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AttestationRequestRequestTag {
    AttestationNonce = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AttestationResponseTag {
    AttestationElements = 0 , AttestationSignature = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CertificateChainRequestRequestTag {
    CertificateType = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CertificateChainResponseTag {
    Certificate = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CSRRequestRequestTag {
    CsrNonce = 0 , IsForUpdateNoc = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum CSRResponseTag {
    NocsrElements = 0 , AttestationSignature = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AddNOCRequestTag {
    NocValue = 0 , IcacValue = 1 , IpkValue = 2 , CaseAdminSubject = 3 , AdminVendorId = 4 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum UpdateNOCRequestTag {
    NocValue = 0 , IcacValue = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum NOCResponseTag {
    StatusCode = 0 , FabricIndex = 1 , DebugText = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum UpdateFabricLabelRequestTag {
    Label = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum RemoveFabricRequestTag {
    FabricIndex = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum AddTrustedRootCertificateRequestTag {
    RootCaCertificate = 0 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SetVIDVerificationStatementRequestTag {
    VendorId = 0 , VidVerificationStatement = 1 , Vvsc = 2 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SignVIDVerificationRequestRequestTag {
    FabricIndex = 0 , ClientChallenge = 1 , 
}
# [derive (Debug , PartialEq , Eq , Copy , Clone , Hash ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u8 ) ] pub enum SignVIDVerificationResponseTag {
    FabricIndex = 0 , FabricBindingVersion = 1 , Signature = 2 , 
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct FabricDescriptorStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > FabricDescriptorStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn root_public_key (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn vendor_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn fabric_id (& self ) -> Result < crate :: im :: FabricId , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn node_id (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    pub fn label (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (5 ) 
    }
    pub fn vid_verification_statement (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (6 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for FabricDescriptorStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for FabricDescriptorStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for FabricDescriptorStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "FabricDescriptorStruct" ) ? ;
        match self . root_public_key () {
            Ok (value ) => write ! (f , "{}: {:?}," , "root_public_key" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "root_public_key" , e . code () ) ? , 
        }
        match self . vendor_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "vendor_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "vendor_id" , e . code () ) ? , 
        }
        match self . fabric_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "fabric_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_id" , e . code () ) ? , 
        }
        match self . node_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "node_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "node_id" , e . code () ) ? , 
        }
        match self . label () {
            Ok (value ) => write ! (f , "{}: {:?}," , "label" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "label" , e . code () ) ? , 
        }
        match self . vid_verification_statement () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "vid_verification_statement" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "vid_verification_statement" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "vid_verification_statement" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for FabricDescriptorStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "FabricDescriptorStruct" ) ;
        match self . root_public_key () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "root_public_key" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "root_public_key" , e . code () ) , 
        }
        match self . vendor_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "vendor_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "vendor_id" , e . code () ) , 
        }
        match self . fabric_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "fabric_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_id" , e . code () ) , 
        }
        match self . node_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "node_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "node_id" , e . code () ) , 
        }
        match self . label () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "label" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "label" , e . code () ) , 
        }
        match self . vid_verification_statement () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "vid_verification_statement" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "vid_verification_statement" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "vid_verification_statement" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct NOCStruct < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > NOCStruct < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn noc (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn icac (& self ) -> Result < crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn vvsc (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (3 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (254 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for NOCStruct < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for NOCStruct < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for NOCStruct < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "NOCStruct" ) ? ;
        match self . noc () {
            Ok (value ) => write ! (f , "{}: {:?}," , "noc" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "noc" , e . code () ) ? , 
        }
        match self . icac () {
            Ok (value ) => write ! (f , "{}: {:?}," , "icac" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "icac" , e . code () ) ? , 
        }
        match self . vvsc () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "vvsc" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "vvsc" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "vvsc" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for NOCStruct < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "NOCStruct" ) ;
        match self . noc () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "noc" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "noc" , e . code () ) , 
        }
        match self . icac () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "icac" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "icac" , e . code () ) , 
        }
        match self . vvsc () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "vvsc" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "vvsc" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "vvsc" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AttestationRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AttestationRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn attestation_nonce (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AttestationRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AttestationRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AttestationRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AttestationRequestRequest" ) ? ;
        match self . attestation_nonce () {
            Ok (value ) => write ! (f , "{}: {:?}," , "attestation_nonce" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "attestation_nonce" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AttestationRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AttestationRequestRequest" ) ;
        match self . attestation_nonce () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "attestation_nonce" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "attestation_nonce" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AttestationResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AttestationResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn attestation_elements (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn attestation_signature (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AttestationResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AttestationResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AttestationResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AttestationResponse" ) ? ;
        match self . attestation_elements () {
            Ok (value ) => write ! (f , "{}: {:?}," , "attestation_elements" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "attestation_elements" , e . code () ) ? , 
        }
        match self . attestation_signature () {
            Ok (value ) => write ! (f , "{}: {:?}," , "attestation_signature" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "attestation_signature" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AttestationResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AttestationResponse" ) ;
        match self . attestation_elements () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "attestation_elements" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "attestation_elements" , e . code () ) , 
        }
        match self . attestation_signature () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "attestation_signature" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "attestation_signature" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CertificateChainRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CertificateChainRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn certificate_type (& self ) -> Result < CertificateChainTypeEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CertificateChainRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CertificateChainRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CertificateChainRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CertificateChainRequestRequest" ) ? ;
        match self . certificate_type () {
            Ok (value ) => write ! (f , "{}: {:?}," , "certificate_type" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "certificate_type" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CertificateChainRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CertificateChainRequestRequest" ) ;
        match self . certificate_type () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "certificate_type" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "certificate_type" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CertificateChainResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CertificateChainResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn certificate (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CertificateChainResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CertificateChainResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CertificateChainResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CertificateChainResponse" ) ? ;
        match self . certificate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "certificate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "certificate" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CertificateChainResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CertificateChainResponse" ) ;
        match self . certificate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "certificate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "certificate" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CSRRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CSRRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn csr_nonce (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn is_for_update_noc (& self ) -> Result < Option < bool > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CSRRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CSRRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CSRRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CSRRequestRequest" ) ? ;
        match self . csr_nonce () {
            Ok (value ) => write ! (f , "{}: {:?}," , "csr_nonce" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "csr_nonce" , e . code () ) ? , 
        }
        match self . is_for_update_noc () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "is_for_update_noc" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "is_for_update_noc" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "is_for_update_noc" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CSRRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CSRRequestRequest" ) ;
        match self . csr_nonce () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "csr_nonce" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "csr_nonce" , e . code () ) , 
        }
        match self . is_for_update_noc () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "is_for_update_noc" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "is_for_update_noc" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "is_for_update_noc" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct CSRResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > CSRResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn nocsr_elements (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn attestation_signature (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for CSRResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for CSRResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for CSRResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "CSRResponse" ) ? ;
        match self . nocsr_elements () {
            Ok (value ) => write ! (f , "{}: {:?}," , "nocsr_elements" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "nocsr_elements" , e . code () ) ? , 
        }
        match self . attestation_signature () {
            Ok (value ) => write ! (f , "{}: {:?}," , "attestation_signature" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "attestation_signature" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for CSRResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "CSRResponse" ) ;
        match self . nocsr_elements () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "nocsr_elements" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "nocsr_elements" , e . code () ) , 
        }
        match self . attestation_signature () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "attestation_signature" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "attestation_signature" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AddNOCRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AddNOCRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn noc_value (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn icac_value (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn ipk_value (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    pub fn case_admin_subject (& self ) -> Result < u64 , crate :: error :: Error > {
        self . 0 . read (3 ) 
    }
    pub fn admin_vendor_id (& self ) -> Result < u16 , crate :: error :: Error > {
        self . 0 . read (4 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AddNOCRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AddNOCRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AddNOCRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AddNOCRequest" ) ? ;
        match self . noc_value () {
            Ok (value ) => write ! (f , "{}: {:?}," , "noc_value" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "noc_value" , e . code () ) ? , 
        }
        match self . icac_value () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "icac_value" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "icac_value" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "icac_value" , e . code () ) ? , 
        }
        match self . ipk_value () {
            Ok (value ) => write ! (f , "{}: {:?}," , "ipk_value" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "ipk_value" , e . code () ) ? , 
        }
        match self . case_admin_subject () {
            Ok (value ) => write ! (f , "{}: {:?}," , "case_admin_subject" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "case_admin_subject" , e . code () ) ? , 
        }
        match self . admin_vendor_id () {
            Ok (value ) => write ! (f , "{}: {:?}," , "admin_vendor_id" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "admin_vendor_id" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AddNOCRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AddNOCRequest" ) ;
        match self . noc_value () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "noc_value" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "noc_value" , e . code () ) , 
        }
        match self . icac_value () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "icac_value" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "icac_value" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "icac_value" , e . code () ) , 
        }
        match self . ipk_value () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "ipk_value" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "ipk_value" , e . code () ) , 
        }
        match self . case_admin_subject () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "case_admin_subject" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "case_admin_subject" , e . code () ) , 
        }
        match self . admin_vendor_id () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "admin_vendor_id" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "admin_vendor_id" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct UpdateNOCRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > UpdateNOCRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn noc_value (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn icac_value (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for UpdateNOCRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for UpdateNOCRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for UpdateNOCRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "UpdateNOCRequest" ) ? ;
        match self . noc_value () {
            Ok (value ) => write ! (f , "{}: {:?}," , "noc_value" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "noc_value" , e . code () ) ? , 
        }
        match self . icac_value () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "icac_value" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "icac_value" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "icac_value" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for UpdateNOCRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "UpdateNOCRequest" ) ;
        match self . noc_value () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "noc_value" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "noc_value" , e . code () ) , 
        }
        match self . icac_value () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "icac_value" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "icac_value" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "icac_value" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct NOCResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > NOCResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn status_code (& self ) -> Result < NodeOperationalCertStatusEnum , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn fabric_index (& self ) -> Result < Option < crate :: im :: FabricIndex > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn debug_text (& self ) -> Result < Option < crate :: tlv :: Utf8Str < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for NOCResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for NOCResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for NOCResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "NOCResponse" ) ? ;
        match self . status_code () {
            Ok (value ) => write ! (f , "{}: {:?}," , "status_code" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "status_code" , e . code () ) ? , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "fabric_index" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "fabric_index" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "debug_text" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "debug_text" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for NOCResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "NOCResponse" ) ;
        match self . status_code () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "status_code" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "status_code" , e . code () ) , 
        }
        match self . fabric_index () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "fabric_index" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "fabric_index" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        match self . debug_text () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "debug_text" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "debug_text" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "debug_text" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct UpdateFabricLabelRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > UpdateFabricLabelRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn label (& self ) -> Result < crate :: tlv :: Utf8Str < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for UpdateFabricLabelRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for UpdateFabricLabelRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for UpdateFabricLabelRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "UpdateFabricLabelRequest" ) ? ;
        match self . label () {
            Ok (value ) => write ! (f , "{}: {:?}," , "label" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "label" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for UpdateFabricLabelRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "UpdateFabricLabelRequest" ) ;
        match self . label () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "label" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "label" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct RemoveFabricRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > RemoveFabricRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn fabric_index (& self ) -> Result < crate :: im :: FabricIndex , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for RemoveFabricRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for RemoveFabricRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for RemoveFabricRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "RemoveFabricRequest" ) ? ;
        match self . fabric_index () {
            Ok (value ) => write ! (f , "{}: {:?}," , "fabric_index" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for RemoveFabricRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "RemoveFabricRequest" ) ;
        match self . fabric_index () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "fabric_index" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct AddTrustedRootCertificateRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > AddTrustedRootCertificateRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn root_ca_certificate (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for AddTrustedRootCertificateRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for AddTrustedRootCertificateRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for AddTrustedRootCertificateRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "AddTrustedRootCertificateRequest" ) ? ;
        match self . root_ca_certificate () {
            Ok (value ) => write ! (f , "{}: {:?}," , "root_ca_certificate" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "root_ca_certificate" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for AddTrustedRootCertificateRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "AddTrustedRootCertificateRequest" ) ;
        match self . root_ca_certificate () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "root_ca_certificate" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "root_ca_certificate" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SetVIDVerificationStatementRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SetVIDVerificationStatementRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn vendor_id (& self ) -> Result < Option < u16 > , crate :: error :: Error > {
        self . 0 . read_opt (0 ) 
    }
    pub fn vid_verification_statement (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (1 ) 
    }
    pub fn vvsc (& self ) -> Result < Option < crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > {
        self . 0 . read_opt (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SetVIDVerificationStatementRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SetVIDVerificationStatementRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SetVIDVerificationStatementRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SetVIDVerificationStatementRequest" ) ? ;
        match self . vendor_id () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "vendor_id" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "vendor_id" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "vendor_id" , e . code () ) ? , 
        }
        match self . vid_verification_statement () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "vid_verification_statement" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "vid_verification_statement" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "vid_verification_statement" , e . code () ) ? , 
        }
        match self . vvsc () {
            Ok (Some (value ) ) => write ! (f , "{}: Some({:?})," , "vvsc" , value ) ? , Ok (None ) => write ! (f , "{}: None," , "vvsc" ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "vvsc" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SetVIDVerificationStatementRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SetVIDVerificationStatementRequest" ) ;
        match self . vendor_id () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "vendor_id" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "vendor_id" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "vendor_id" , e . code () ) , 
        }
        match self . vid_verification_statement () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "vid_verification_statement" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "vid_verification_statement" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "vid_verification_statement" , e . code () ) , 
        }
        match self . vvsc () {
            Ok (Some (value ) ) => crate :: reexport :: defmt :: write ! (f , "{}: Some({:?})," , "vvsc" , value ) , Ok (None ) => crate :: reexport :: defmt :: write ! (f , "{}: None," , "vvsc" ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "vvsc" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SignVIDVerificationRequestRequest < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SignVIDVerificationRequestRequest < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn fabric_index (& self ) -> Result < crate :: im :: FabricIndex , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn client_challenge (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SignVIDVerificationRequestRequest < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SignVIDVerificationRequestRequest < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SignVIDVerificationRequestRequest < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SignVIDVerificationRequestRequest" ) ? ;
        match self . fabric_index () {
            Ok (value ) => write ! (f , "{}: {:?}," , "fabric_index" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        match self . client_challenge () {
            Ok (value ) => write ! (f , "{}: {:?}," , "client_challenge" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "client_challenge" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SignVIDVerificationRequestRequest < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SignVIDVerificationRequestRequest" ) ;
        match self . fabric_index () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "fabric_index" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        match self . client_challenge () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "client_challenge" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "client_challenge" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
# [derive (PartialEq , Eq , Clone , Hash ) ] pub struct SignVIDVerificationResponse < 'a > (crate :: tlv :: TLVElement < 'a > ) ;
impl < 'a > SignVIDVerificationResponse < 'a > {
    # [doc = "Create a new instance" ] pub const fn new (element : crate :: tlv :: TLVElement < 'a > ) -> Self {
        Self (element ) 
    }
    # [doc = "Return the underlying TLV element" ] pub const fn tlv_element (& self ) -> & crate :: tlv :: TLVElement < 'a > {
        & self . 0 
    }
    pub fn fabric_index (& self ) -> Result < crate :: im :: FabricIndex , crate :: error :: Error > {
        self . 0 . read (0 ) 
    }
    pub fn fabric_binding_version (& self ) -> Result < u8 , crate :: error :: Error > {
        self . 0 . read (1 ) 
    }
    pub fn signature (& self ) -> Result < crate :: tlv :: OctetStr < '_ > , crate :: error :: Error > {
        self . 0 . read (2 ) 
    }
    
}
impl < 'a > crate :: tlv :: FromTLV < 'a > for SignVIDVerificationResponse < 'a > {
    fn from_tlv (element : & crate :: tlv :: TLVElement < 'a > ) -> Result < Self , crate :: error :: Error > {
        Ok (Self :: new (element . clone () ) ) 
    }
    
}
impl crate :: tlv :: ToTLV for SignVIDVerificationResponse < '_ > {
    fn to_tlv < W : crate :: tlv :: TLVWrite > (& self , tag : & crate :: tlv :: TLVTag , tw : W ) -> Result < () , crate :: error :: Error > {
        self . 0 . to_tlv (tag , tw ) 
    }
    fn tlv_iter (& self , tag : crate :: tlv :: TLVTag ) -> impl Iterator < Item = Result < crate :: tlv :: TLV , crate :: error :: Error >> {
        self . 0 . tlv_iter (tag ) 
    }
    
}
impl core :: fmt :: Debug for SignVIDVerificationResponse < '_ > {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{} {{" , "SignVIDVerificationResponse" ) ? ;
        match self . fabric_index () {
            Ok (value ) => write ! (f , "{}: {:?}," , "fabric_index" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) ? , 
        }
        match self . fabric_binding_version () {
            Ok (value ) => write ! (f , "{}: {:?}," , "fabric_binding_version" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "fabric_binding_version" , e . code () ) ? , 
        }
        match self . signature () {
            Ok (value ) => write ! (f , "{}: {:?}," , "signature" , value ) ? , Err (e ) => write ! (f , "{}: ??? {:?}," , "signature" , e . code () ) ? , 
        }
        write ! (f , "}}" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for SignVIDVerificationResponse < '_ > {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{} {{" , "SignVIDVerificationResponse" ) ;
        match self . fabric_index () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "fabric_index" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_index" , e . code () ) , 
        }
        match self . fabric_binding_version () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "fabric_binding_version" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "fabric_binding_version" , e . code () ) , 
        }
        match self . signature () {
            Ok (value ) => crate :: reexport :: defmt :: write ! (f , "{}: {:?}," , "signature" , value ) , Err (e ) => crate :: reexport :: defmt :: write ! (f , "{}: ??? {:?}," , "signature" , e . code () ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "}}" ) 
    }
    
}
pub struct FabricDescriptorStructBuilder < P , const F : usize = 1usize > (P ) ;
impl < P > FabricDescriptorStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn root_public_key (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < FabricDescriptorStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rootPublicKey" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (FabricDescriptorStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn vendor_id (mut self , value : u16 ) -> Result < FabricDescriptorStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "vendorID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (FabricDescriptorStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_id (mut self , value : crate :: im :: FabricId ) -> Result < FabricDescriptorStructBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (FabricDescriptorStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn node_id (mut self , value : u64 ) -> Result < FabricDescriptorStructBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "nodeID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (FabricDescriptorStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 5 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn label (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < FabricDescriptorStructBuilder < P , 6usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "label" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 5 , & value ) ? ;
        Ok (FabricDescriptorStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 6 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn vid_verification_statement (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < FabricDescriptorStructBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "VIDVerificationStatement" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 6 , & value ) ? ;
        Ok (FabricDescriptorStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < FabricDescriptorStructBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (FabricDescriptorStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > FabricDescriptorStructBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for FabricDescriptorStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "FabricDescriptorStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for FabricDescriptorStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "FabricDescriptorStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for FabricDescriptorStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for FabricDescriptorStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct FabricDescriptorStructArrayBuilder < P > (P ) ;
impl < P > FabricDescriptorStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < FabricDescriptorStructBuilder < FabricDescriptorStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (FabricDescriptorStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for FabricDescriptorStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "FabricDescriptorStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for FabricDescriptorStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "FabricDescriptorStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for FabricDescriptorStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for FabricDescriptorStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NOCStructBuilder < P , const F : usize = 1usize > (P ) ;
impl < P > NOCStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > NOCStructBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn noc (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < NOCStructBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "noc" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (NOCStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NOCStructBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn icac (mut self , value : crate :: tlv :: Nullable < crate :: tlv :: OctetStr < '_ > > ) -> Result < NOCStructBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "icac" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (NOCStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NOCStructBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn vvsc (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < NOCStructBuilder < P , 254usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "vvsc" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (NOCStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NOCStructBuilder < P , 254 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < NOCStructBuilder < P , 255usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 254 , & value ) ? ;
        Ok (NOCStructBuilder (self . 0 ) ) 
    }
    
}
impl < P > NOCStructBuilder < P , 255usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for NOCStructBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NOCStruct" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for NOCStructBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NOCStruct" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for NOCStructBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NOCStructBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NOCStructArrayBuilder < P > (P ) ;
impl < P > NOCStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < NOCStructBuilder < NOCStructArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NOCStructArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for NOCStructArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NOCStruct[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for NOCStructArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NOCStruct[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for NOCStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NOCStructArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AttestationRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AttestationRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AttestationRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn attestation_nonce (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AttestationRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "attestationNonce" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AttestationRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AttestationRequestRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AttestationRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AttestationRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AttestationRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AttestationRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AttestationRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AttestationRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AttestationRequestRequestArrayBuilder < P > (P ) ;
impl < P > AttestationRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AttestationRequestRequestBuilder < AttestationRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AttestationRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AttestationRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AttestationRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AttestationRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AttestationRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AttestationRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AttestationRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AttestationResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AttestationResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AttestationResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn attestation_elements (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AttestationResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "attestationElements" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AttestationResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > AttestationResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn attestation_signature (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AttestationResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "attestationSignature" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AttestationResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > AttestationResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AttestationResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AttestationResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AttestationResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AttestationResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AttestationResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AttestationResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AttestationResponseArrayBuilder < P > (P ) ;
impl < P > AttestationResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AttestationResponseBuilder < AttestationResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AttestationResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AttestationResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AttestationResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AttestationResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AttestationResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AttestationResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AttestationResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CertificateChainRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CertificateChainRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CertificateChainRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn certificate_type (mut self , value : CertificateChainTypeEnum ) -> Result < CertificateChainRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "certificateType" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CertificateChainRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > CertificateChainRequestRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CertificateChainRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CertificateChainRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CertificateChainRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CertificateChainRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CertificateChainRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CertificateChainRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CertificateChainRequestRequestArrayBuilder < P > (P ) ;
impl < P > CertificateChainRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CertificateChainRequestRequestBuilder < CertificateChainRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CertificateChainRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CertificateChainRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CertificateChainRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CertificateChainRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CertificateChainRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CertificateChainRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CertificateChainRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CertificateChainResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CertificateChainResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CertificateChainResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn certificate (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < CertificateChainResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "certificate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CertificateChainResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > CertificateChainResponseBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CertificateChainResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CertificateChainResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CertificateChainResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CertificateChainResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CertificateChainResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CertificateChainResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CertificateChainResponseArrayBuilder < P > (P ) ;
impl < P > CertificateChainResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CertificateChainResponseBuilder < CertificateChainResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CertificateChainResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CertificateChainResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CertificateChainResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CertificateChainResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CertificateChainResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CertificateChainResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CertificateChainResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CSRRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CSRRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CSRRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn csr_nonce (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < CSRRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "CSRNonce" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CSRRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > CSRRequestRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn is_for_update_noc (mut self , value : Option < bool > ) -> Result < CSRRequestRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "isForUpdateNOC" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (CSRRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > CSRRequestRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CSRRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CSRRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CSRRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CSRRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CSRRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CSRRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CSRRequestRequestArrayBuilder < P > (P ) ;
impl < P > CSRRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CSRRequestRequestBuilder < CSRRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CSRRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CSRRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CSRRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CSRRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CSRRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CSRRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CSRRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CSRResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > CSRResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > CSRResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn nocsr_elements (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < CSRResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "NOCSRElements" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (CSRResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > CSRResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn attestation_signature (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < CSRResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "attestationSignature" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (CSRResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > CSRResponseBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for CSRResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CSRResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for CSRResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CSRResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for CSRResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CSRResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct CSRResponseArrayBuilder < P > (P ) ;
impl < P > CSRResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < CSRResponseBuilder < CSRResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (CSRResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for CSRResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "CSRResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for CSRResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "CSRResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for CSRResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for CSRResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddNOCRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AddNOCRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AddNOCRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn noc_value (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AddNOCRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "NOCValue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AddNOCRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddNOCRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn icac_value (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < AddNOCRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ICACValue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (AddNOCRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddNOCRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn ipk_value (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AddNOCRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "IPKValue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (AddNOCRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddNOCRequestBuilder < P , 3 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn case_admin_subject (mut self , value : u64 ) -> Result < AddNOCRequestBuilder < P , 4usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "caseAdminSubject" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 3 , & value ) ? ;
        Ok (AddNOCRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddNOCRequestBuilder < P , 4 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn admin_vendor_id (mut self , value : u16 ) -> Result < AddNOCRequestBuilder < P , 5usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "adminVendorId" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 4 , & value ) ? ;
        Ok (AddNOCRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddNOCRequestBuilder < P , 5usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AddNOCRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddNOCRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AddNOCRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddNOCRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AddNOCRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddNOCRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddNOCRequestArrayBuilder < P > (P ) ;
impl < P > AddNOCRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AddNOCRequestBuilder < AddNOCRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AddNOCRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AddNOCRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddNOCRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AddNOCRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddNOCRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AddNOCRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddNOCRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct UpdateNOCRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > UpdateNOCRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > UpdateNOCRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn noc_value (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < UpdateNOCRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "NOCValue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (UpdateNOCRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > UpdateNOCRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn icac_value (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < UpdateNOCRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "ICACValue" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (UpdateNOCRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > UpdateNOCRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for UpdateNOCRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "UpdateNOCRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for UpdateNOCRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "UpdateNOCRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for UpdateNOCRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for UpdateNOCRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct UpdateNOCRequestArrayBuilder < P > (P ) ;
impl < P > UpdateNOCRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < UpdateNOCRequestBuilder < UpdateNOCRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (UpdateNOCRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for UpdateNOCRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "UpdateNOCRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for UpdateNOCRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "UpdateNOCRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for UpdateNOCRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for UpdateNOCRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NOCResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > NOCResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > NOCResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn status_code (mut self , value : NodeOperationalCertStatusEnum ) -> Result < NOCResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "statusCode" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (NOCResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > NOCResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : Option < crate :: im :: FabricIndex > ) -> Result < NOCResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (NOCResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > NOCResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn debug_text (mut self , value : Option < crate :: tlv :: Utf8Str < '_ > > ) -> Result < NOCResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "debugText" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (NOCResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > NOCResponseBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for NOCResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NOCResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for NOCResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NOCResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for NOCResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NOCResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct NOCResponseArrayBuilder < P > (P ) ;
impl < P > NOCResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < NOCResponseBuilder < NOCResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (NOCResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for NOCResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "NOCResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for NOCResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "NOCResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for NOCResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for NOCResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct UpdateFabricLabelRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > UpdateFabricLabelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > UpdateFabricLabelRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn label (mut self , value : crate :: tlv :: Utf8Str < '_ > ) -> Result < UpdateFabricLabelRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "label" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (UpdateFabricLabelRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > UpdateFabricLabelRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for UpdateFabricLabelRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "UpdateFabricLabelRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for UpdateFabricLabelRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "UpdateFabricLabelRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for UpdateFabricLabelRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for UpdateFabricLabelRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct UpdateFabricLabelRequestArrayBuilder < P > (P ) ;
impl < P > UpdateFabricLabelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < UpdateFabricLabelRequestBuilder < UpdateFabricLabelRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (UpdateFabricLabelRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for UpdateFabricLabelRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "UpdateFabricLabelRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for UpdateFabricLabelRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "UpdateFabricLabelRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for UpdateFabricLabelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for UpdateFabricLabelRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RemoveFabricRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > RemoveFabricRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > RemoveFabricRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : crate :: im :: FabricIndex ) -> Result < RemoveFabricRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (RemoveFabricRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > RemoveFabricRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for RemoveFabricRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RemoveFabricRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for RemoveFabricRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RemoveFabricRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for RemoveFabricRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RemoveFabricRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct RemoveFabricRequestArrayBuilder < P > (P ) ;
impl < P > RemoveFabricRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < RemoveFabricRequestBuilder < RemoveFabricRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (RemoveFabricRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for RemoveFabricRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "RemoveFabricRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for RemoveFabricRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "RemoveFabricRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for RemoveFabricRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for RemoveFabricRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddTrustedRootCertificateRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > AddTrustedRootCertificateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > AddTrustedRootCertificateRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn root_ca_certificate (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < AddTrustedRootCertificateRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "rootCACertificate" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (AddTrustedRootCertificateRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > AddTrustedRootCertificateRequestBuilder < P , 1usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for AddTrustedRootCertificateRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddTrustedRootCertificateRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for AddTrustedRootCertificateRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddTrustedRootCertificateRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for AddTrustedRootCertificateRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddTrustedRootCertificateRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct AddTrustedRootCertificateRequestArrayBuilder < P > (P ) ;
impl < P > AddTrustedRootCertificateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < AddTrustedRootCertificateRequestBuilder < AddTrustedRootCertificateRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (AddTrustedRootCertificateRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for AddTrustedRootCertificateRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "AddTrustedRootCertificateRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for AddTrustedRootCertificateRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "AddTrustedRootCertificateRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for AddTrustedRootCertificateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for AddTrustedRootCertificateRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetVIDVerificationStatementRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SetVIDVerificationStatementRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SetVIDVerificationStatementRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn vendor_id (mut self , value : Option < u16 > ) -> Result < SetVIDVerificationStatementRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "vendorID" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SetVIDVerificationStatementRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetVIDVerificationStatementRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn vid_verification_statement (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < SetVIDVerificationStatementRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "VIDVerificationStatement" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SetVIDVerificationStatementRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetVIDVerificationStatementRequestBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn vvsc (mut self , value : Option < crate :: tlv :: OctetStr < '_ > > ) -> Result < SetVIDVerificationStatementRequestBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "vvsc" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SetVIDVerificationStatementRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SetVIDVerificationStatementRequestBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SetVIDVerificationStatementRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetVIDVerificationStatementRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SetVIDVerificationStatementRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetVIDVerificationStatementRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SetVIDVerificationStatementRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetVIDVerificationStatementRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SetVIDVerificationStatementRequestArrayBuilder < P > (P ) ;
impl < P > SetVIDVerificationStatementRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SetVIDVerificationStatementRequestBuilder < SetVIDVerificationStatementRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SetVIDVerificationStatementRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SetVIDVerificationStatementRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SetVIDVerificationStatementRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SetVIDVerificationStatementRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SetVIDVerificationStatementRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SetVIDVerificationStatementRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SetVIDVerificationStatementRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SignVIDVerificationRequestRequestBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SignVIDVerificationRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SignVIDVerificationRequestRequestBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : crate :: im :: FabricIndex ) -> Result < SignVIDVerificationRequestRequestBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SignVIDVerificationRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SignVIDVerificationRequestRequestBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn client_challenge (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < SignVIDVerificationRequestRequestBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "clientChallenge" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SignVIDVerificationRequestRequestBuilder (self . 0 ) ) 
    }
    
}
impl < P > SignVIDVerificationRequestRequestBuilder < P , 2usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SignVIDVerificationRequestRequestBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationRequestRequest" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SignVIDVerificationRequestRequestBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationRequestRequest" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SignVIDVerificationRequestRequestBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SignVIDVerificationRequestRequestBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SignVIDVerificationRequestRequestArrayBuilder < P > (P ) ;
impl < P > SignVIDVerificationRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SignVIDVerificationRequestRequestBuilder < SignVIDVerificationRequestRequestArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SignVIDVerificationRequestRequestArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SignVIDVerificationRequestRequestArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationRequestRequest[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SignVIDVerificationRequestRequestArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationRequestRequest[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SignVIDVerificationRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SignVIDVerificationRequestRequestArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SignVIDVerificationResponseBuilder < P , const F : usize = 0usize > (P ) ;
impl < P > SignVIDVerificationResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_struct (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    
}
impl < P > SignVIDVerificationResponseBuilder < P , 0 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_index (mut self , value : crate :: im :: FabricIndex ) -> Result < SignVIDVerificationResponseBuilder < P , 1usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricIndex" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 0 , & value ) ? ;
        Ok (SignVIDVerificationResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SignVIDVerificationResponseBuilder < P , 1 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn fabric_binding_version (mut self , value : u8 ) -> Result < SignVIDVerificationResponseBuilder < P , 2usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "fabricBindingVersion" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 1 , & value ) ? ;
        Ok (SignVIDVerificationResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SignVIDVerificationResponseBuilder < P , 2 > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn signature (mut self , value : crate :: tlv :: OctetStr < '_ > ) -> Result < SignVIDVerificationResponseBuilder < P , 3usize > , crate :: error :: Error > {
        crate :: dm :: log_field (& self , "signature" , & value ) ;
        crate :: tlv :: TLVWrite :: write_ctx (self . 0 . writer () , 2 , & value ) ? ;
        Ok (SignVIDVerificationResponseBuilder (self . 0 ) ) 
    }
    
}
impl < P > SignVIDVerificationResponseBuilder < P , 3usize > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Finish the struct and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P , const F : usize > core :: fmt :: Debug for SignVIDVerificationResponseBuilder < P , F > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationResponse" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P , const F : usize > crate :: reexport :: defmt :: Format for SignVIDVerificationResponseBuilder < P , F > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationResponse" ) 
    }
    
}
impl < P , const F : usize > crate :: tlv :: TLVBuilderParent for SignVIDVerificationResponseBuilder < P , F > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SignVIDVerificationResponseBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
pub struct SignVIDVerificationResponseArrayBuilder < P > (P ) ;
impl < P > SignVIDVerificationResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = "Create a new instance" ] pub fn new (mut parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        parent . writer () . start_array (tag ) ? ;
        Ok (Self (parent ) ) 
    }
    # [doc = "Push a new element into the array" ] pub fn push (self ) -> Result < SignVIDVerificationResponseBuilder < SignVIDVerificationResponseArrayBuilder < P >> , crate :: error :: Error > {
        crate :: tlv :: TLVBuilder :: new (SignVIDVerificationResponseArrayBuilder (self . 0 ) , & crate :: tlv :: TLVTag :: Anonymous ) 
    }
    # [doc = "Finish the array and return the parent" ] pub fn end (mut self ) -> Result < P , crate :: error :: Error > {
        use crate :: tlv :: TLVWrite ;
        self . 0 . writer () . end_container () ? ;
        Ok (self . 0 ) 
    }
    
}
impl < P > core :: fmt :: Debug for SignVIDVerificationResponseArrayBuilder < P > where P : core :: fmt :: Debug , {
    fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationResponse[]" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < P > crate :: reexport :: defmt :: Format for SignVIDVerificationResponseArrayBuilder < P > where P : crate :: reexport :: defmt :: Format , {
    fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "{:?}::{}" , self . 0 , "SignVIDVerificationResponse[]" ) 
    }
    
}
impl < P > crate :: tlv :: TLVBuilderParent for SignVIDVerificationResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    type Write = P :: Write ;
    fn writer (& mut self ) -> & mut P :: Write {
        self . 0 . writer () 
    }
    
}
impl < P > crate :: tlv :: TLVBuilder < P > for SignVIDVerificationResponseArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn new (parent : P , tag : & crate :: tlv :: TLVTag ) -> Result < Self , crate :: error :: Error > {
        Self :: new (parent , tag ) 
    }
    fn unchecked_into_parent (self ) -> P {
        self . 0 
    }
    
}
# [doc = "The attribute IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum AttributeId {
    NOCs = 0 , Fabrics = 1 , SupportedFabrics = 2 , CommissionedFabrics = 3 , TrustedRootCertificates = 4 , CurrentFabricIndex = 5 , GeneratedCommandList = 65528 , AcceptedCommandList = 65529 , AttributeList = 65531 , FeatureMap = 65532 , ClusterRevision = 65533 
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
            AttributeId :: NOCs => write ! (f , "{}(0x{:02x})" , "NOCs" , AttributeId :: NOCs as u32 ) ? , AttributeId :: Fabrics => write ! (f , "{}(0x{:02x})" , "Fabrics" , AttributeId :: Fabrics as u32 ) ? , AttributeId :: SupportedFabrics => write ! (f , "{}(0x{:02x})" , "SupportedFabrics" , AttributeId :: SupportedFabrics as u32 ) ? , AttributeId :: CommissionedFabrics => write ! (f , "{}(0x{:02x})" , "CommissionedFabrics" , AttributeId :: CommissionedFabrics as u32 ) ? , AttributeId :: TrustedRootCertificates => write ! (f , "{}(0x{:02x})" , "TrustedRootCertificates" , AttributeId :: TrustedRootCertificates as u32 ) ? , AttributeId :: CurrentFabricIndex => write ! (f , "{}(0x{:02x})" , "CurrentFabricIndex" , AttributeId :: CurrentFabricIndex as u32 ) ? , AttributeId :: GeneratedCommandList => write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) ? , AttributeId :: AcceptedCommandList => write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) ? , AttributeId :: AttributeList => write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) ? , AttributeId :: FeatureMap => write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) ? , AttributeId :: ClusterRevision => write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) ? , 
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
            AttributeId :: NOCs => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NOCs" , AttributeId :: NOCs as u32 ) , AttributeId :: Fabrics => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "Fabrics" , AttributeId :: Fabrics as u32 ) , AttributeId :: SupportedFabrics => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SupportedFabrics" , AttributeId :: SupportedFabrics as u32 ) , AttributeId :: CommissionedFabrics => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CommissionedFabrics" , AttributeId :: CommissionedFabrics as u32 ) , AttributeId :: TrustedRootCertificates => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "TrustedRootCertificates" , AttributeId :: TrustedRootCertificates as u32 ) , AttributeId :: CurrentFabricIndex => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CurrentFabricIndex" , AttributeId :: CurrentFabricIndex as u32 ) , AttributeId :: GeneratedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "GeneratedCommandList" , AttributeId :: GeneratedCommandList as u32 ) , AttributeId :: AcceptedCommandList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AcceptedCommandList" , AttributeId :: AcceptedCommandList as u32 ) , AttributeId :: AttributeList => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttributeList" , AttributeId :: AttributeList as u32 ) , AttributeId :: FeatureMap => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "FeatureMap" , AttributeId :: FeatureMap as u32 ) , AttributeId :: ClusterRevision => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "ClusterRevision" , AttributeId :: ClusterRevision as u32 ) , 
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
    AttestationRequest = 0 , CertificateChainRequest = 2 , CSRRequest = 4 , AddNOC = 6 , UpdateNOC = 7 , UpdateFabricLabel = 9 , RemoveFabric = 10 , AddTrustedRootCertificate = 11 , SetVIDVerificationStatement = 12 , SignVIDVerificationRequest = 13 
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
            CommandId :: AttestationRequest => write ! (f , "{}(0x{:02x})" , "AttestationRequest" , CommandId :: AttestationRequest as u32 ) ? , CommandId :: CertificateChainRequest => write ! (f , "{}(0x{:02x})" , "CertificateChainRequest" , CommandId :: CertificateChainRequest as u32 ) ? , CommandId :: CSRRequest => write ! (f , "{}(0x{:02x})" , "CSRRequest" , CommandId :: CSRRequest as u32 ) ? , CommandId :: AddNOC => write ! (f , "{}(0x{:02x})" , "AddNOC" , CommandId :: AddNOC as u32 ) ? , CommandId :: UpdateNOC => write ! (f , "{}(0x{:02x})" , "UpdateNOC" , CommandId :: UpdateNOC as u32 ) ? , CommandId :: UpdateFabricLabel => write ! (f , "{}(0x{:02x})" , "UpdateFabricLabel" , CommandId :: UpdateFabricLabel as u32 ) ? , CommandId :: RemoveFabric => write ! (f , "{}(0x{:02x})" , "RemoveFabric" , CommandId :: RemoveFabric as u32 ) ? , CommandId :: AddTrustedRootCertificate => write ! (f , "{}(0x{:02x})" , "AddTrustedRootCertificate" , CommandId :: AddTrustedRootCertificate as u32 ) ? , CommandId :: SetVIDVerificationStatement => write ! (f , "{}(0x{:02x})" , "SetVIDVerificationStatement" , CommandId :: SetVIDVerificationStatement as u32 ) ? , CommandId :: SignVIDVerificationRequest => write ! (f , "{}(0x{:02x})" , "SignVIDVerificationRequest" , CommandId :: SignVIDVerificationRequest as u32 ) ? , 
        }
        write ! (f , "::Invoke" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandId :: AttestationRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttestationRequest" , CommandId :: AttestationRequest as u32 ) , CommandId :: CertificateChainRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CertificateChainRequest" , CommandId :: CertificateChainRequest as u32 ) , CommandId :: CSRRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CSRRequest" , CommandId :: CSRRequest as u32 ) , CommandId :: AddNOC => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AddNOC" , CommandId :: AddNOC as u32 ) , CommandId :: UpdateNOC => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UpdateNOC" , CommandId :: UpdateNOC as u32 ) , CommandId :: UpdateFabricLabel => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "UpdateFabricLabel" , CommandId :: UpdateFabricLabel as u32 ) , CommandId :: RemoveFabric => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "RemoveFabric" , CommandId :: RemoveFabric as u32 ) , CommandId :: AddTrustedRootCertificate => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AddTrustedRootCertificate" , CommandId :: AddTrustedRootCertificate as u32 ) , CommandId :: SetVIDVerificationStatement => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SetVIDVerificationStatement" , CommandId :: SetVIDVerificationStatement as u32 ) , CommandId :: SignVIDVerificationRequest => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SignVIDVerificationRequest" , CommandId :: SignVIDVerificationRequest as u32 ) , 
        }
        crate :: reexport :: defmt :: write ! (f , "::Invoke" ) 
    }
    
}
# [doc = "The command response IDs for the cluster." ] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , crate :: reexport :: strum :: FromRepr ) ] # [cfg_attr (feature = "defmt" , derive (crate :: reexport :: defmt :: Format ) ) ] # [repr (u32 ) ] pub enum CommandResponseId {
    AttestationResponse = 1 , CertificateChainResponse = 3 , CSRResponse = 5 , NOCResponse = 8 , SignVIDVerificationResponse = 14 
}
impl core :: fmt :: Debug for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ > ) -> core :: fmt :: Result {
        write ! (f , "Cmd::" ) ? ;
        match self . 0 {
            CommandResponseId :: AttestationResponse => write ! (f , "{}(0x{:02x})" , "AttestationResponse" , CommandResponseId :: AttestationResponse as u32 ) ? , CommandResponseId :: CertificateChainResponse => write ! (f , "{}(0x{:02x})" , "CertificateChainResponse" , CommandResponseId :: CertificateChainResponse as u32 ) ? , CommandResponseId :: CSRResponse => write ! (f , "{}(0x{:02x})" , "CSRResponse" , CommandResponseId :: CSRResponse as u32 ) ? , CommandResponseId :: NOCResponse => write ! (f , "{}(0x{:02x})" , "NOCResponse" , CommandResponseId :: NOCResponse as u32 ) ? , CommandResponseId :: SignVIDVerificationResponse => write ! (f , "{}(0x{:02x})" , "SignVIDVerificationResponse" , CommandResponseId :: SignVIDVerificationResponse as u32 ) ? , 
        }
        write ! (f , "::Response" ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl crate :: reexport :: defmt :: Format for MetadataDebug < CommandResponseId > {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Cmd::" ) ;
        match self . 0 {
            CommandResponseId :: AttestationResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "AttestationResponse" , CommandResponseId :: AttestationResponse as u32 ) , CommandResponseId :: CertificateChainResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CertificateChainResponse" , CommandResponseId :: CertificateChainResponse as u32 ) , CommandResponseId :: CSRResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "CSRResponse" , CommandResponseId :: CSRResponse as u32 ) , CommandResponseId :: NOCResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "NOCResponse" , CommandResponseId :: NOCResponse as u32 ) , CommandResponseId :: SignVIDVerificationResponse => crate :: reexport :: defmt :: write ! (f , "{}(0x{:02x})" , "SignVIDVerificationResponse" , CommandResponseId :: SignVIDVerificationResponse as u32 ) , 
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
# [doc = "The cluster metadata. By default, all cluster attributes, commands and events are allowed, and the revision is the latest one. Use `Cluster::with_*` to reconfigure." ] pub const FULL_CLUSTER : crate :: dm :: Cluster < 'static > = crate :: dm :: Cluster :: new (62 , 2 , 0 , & [crate :: dm :: Attribute :: new (AttributeId :: NOCs as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_ADMIN ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: Fabrics as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: SupportedFabrics as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: CommissionedFabrics as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: TrustedRootCertificates as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: CurrentFabricIndex as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: GeneratedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AcceptedCommandList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: AttributeList as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: A ) , crate :: dm :: Attribute :: new (AttributeId :: FeatureMap as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , crate :: dm :: Attribute :: new (AttributeId :: ClusterRevision as _ , crate :: dm :: Access :: READ . union (crate :: dm :: Access :: NEED_VIEW ) , crate :: dm :: Quality :: NONE ) , ] , & [crate :: dm :: Command :: new (CommandId :: AttestationRequest as _ , Some (CommandResponseId :: AttestationResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: CertificateChainRequest as _ , Some (CommandResponseId :: CertificateChainResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: CSRRequest as _ , Some (CommandResponseId :: CSRResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: AddNOC as _ , Some (CommandResponseId :: NOCResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: UpdateNOC as _ , Some (CommandResponseId :: NOCResponse as _ ) , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: UpdateFabricLabel as _ , Some (CommandResponseId :: NOCResponse as _ ) , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: RemoveFabric as _ , Some (CommandResponseId :: NOCResponse as _ ) , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: AddTrustedRootCertificate as _ , None , crate :: dm :: Access :: WA , ) , crate :: dm :: Command :: new (CommandId :: SetVIDVerificationStatement as _ , None , crate :: dm :: Access :: WA . union (crate :: dm :: Access :: FAB_SCOPED ) , ) , crate :: dm :: Command :: new (CommandId :: SignVIDVerificationRequest as _ , Some (CommandResponseId :: SignVIDVerificationResponse as _ ) , crate :: dm :: Access :: WA , ) , ] , & [] , | _ , _ , _ | true , | _ , _ , _ | true , | _ , _ , _ | true , ) ;
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
    fn nocs < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NOCStructArrayBuilder < P > , NOCStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn fabrics < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < FabricDescriptorStructArrayBuilder < P > , FabricDescriptorStructBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn supported_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn commissioned_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn trusted_root_certificates < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: OctetsArrayBuilder < P > , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > ;
    fn current_fabric_index (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > ;
    fn handle_attestation_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AttestationRequestRequest < '_ > , response : AttestationResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_certificate_chain_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CertificateChainRequestRequest < '_ > , response : CertificateChainResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_csr_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CSRRequestRequest < '_ > , response : CSRResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_add_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_update_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_update_fabric_label < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateFabricLabelRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_remove_fabric < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveFabricRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    fn handle_add_trusted_root_certificate (& self , ctx : impl crate :: dm :: InvokeContext , request : AddTrustedRootCertificateRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_set_vid_verification_statement (& self , ctx : impl crate :: dm :: InvokeContext , request : SetVIDVerificationStatementRequest < '_ > , ) -> Result < () , crate :: error :: Error > ;
    fn handle_sign_vid_verification_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SignVIDVerificationRequestRequest < '_ > , response : SignVIDVerificationResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > ;
    
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
    fn nocs < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NOCStructArrayBuilder < P > , NOCStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: nocs (self , ctx , builder ) 
    }
    fn fabrics < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < FabricDescriptorStructArrayBuilder < P > , FabricDescriptorStructBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: fabrics (self , ctx , builder ) 
    }
    fn supported_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: supported_fabrics (self , ctx ) 
    }
    fn commissioned_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: commissioned_fabrics (self , ctx ) 
    }
    fn trusted_root_certificates < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: OctetsArrayBuilder < P > , crate :: tlv :: OctetsBuilder < P > > ) -> Result < P , crate :: error :: Error > {
        T :: trusted_root_certificates (self , ctx , builder ) 
    }
    fn current_fabric_index (& self , ctx : impl crate :: dm :: ReadContext ) -> Result < u8 , crate :: error :: Error > {
        T :: current_fabric_index (self , ctx ) 
    }
    fn handle_attestation_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AttestationRequestRequest < '_ > , response : AttestationResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_attestation_request (self , ctx , request , response ) 
    }
    fn handle_certificate_chain_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CertificateChainRequestRequest < '_ > , response : CertificateChainResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_certificate_chain_request (self , ctx , request , response ) 
    }
    fn handle_csr_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CSRRequestRequest < '_ > , response : CSRResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_csr_request (self , ctx , request , response ) 
    }
    fn handle_add_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_add_noc (self , ctx , request , response ) 
    }
    fn handle_update_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_update_noc (self , ctx , request , response ) 
    }
    fn handle_update_fabric_label < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateFabricLabelRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_update_fabric_label (self , ctx , request , response ) 
    }
    fn handle_remove_fabric < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveFabricRequest < '_ > , response : NOCResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_remove_fabric (self , ctx , request , response ) 
    }
    fn handle_add_trusted_root_certificate (& self , ctx : impl crate :: dm :: InvokeContext , request : AddTrustedRootCertificateRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_add_trusted_root_certificate (self , ctx , request ) 
    }
    fn handle_set_vid_verification_statement (& self , ctx : impl crate :: dm :: InvokeContext , request : SetVIDVerificationStatementRequest < '_ > , ) -> Result < () , crate :: error :: Error > {
        T :: handle_set_vid_verification_statement (self , ctx , request ) 
    }
    fn handle_sign_vid_verification_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SignVIDVerificationRequestRequest < '_ > , response : SignVIDVerificationResponseBuilder < P > , ) -> Result < P , crate :: error :: Error > {
        T :: handle_sign_vid_verification_request (self , ctx , request , response ) 
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
                    AttributeId :: NOCs => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NOCs , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. nocs (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NOCs , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NOCs , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Fabrics => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Fabrics , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. fabrics (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Fabrics , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Fabrics , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SupportedFabrics => {
                        let attr_read_result = self . 0. supported_fabrics (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFabrics , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CommissionedFabrics => {
                        let attr_read_result = self . 0. commissioned_fabrics (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CommissionedFabrics , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TrustedRootCertificates => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TrustedRootCertificates , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. trusted_root_certificates (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TrustedRootCertificates , false ) ) ) ) , tw ) , tag , ) ? , ) ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TrustedRootCertificates , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentFabricIndex => {
                        let attr_read_result = self . 0. current_fabric_index (& ctx ) ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentFabricIndex , false ) ) ) ) , & attr_read_result ) ;
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
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: AttestationRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AttestationRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_attestation_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AttestationRequest ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AttestationRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: CertificateChainRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CertificateChainRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (3u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_certificate_chain_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CertificateChainRequest ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CertificateChainRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: CSRRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CSRRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_csr_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CSRRequest ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CSRRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddNOC => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddNOC ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_noc (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddNOC ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddNOC ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: UpdateNOC => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateNOC ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_update_noc (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateNOC ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateNOC ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: UpdateFabricLabel => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateFabricLabel ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_update_fabric_label (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateFabricLabel ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateFabricLabel ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RemoveFabric => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveFabric ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_remove_fabric (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveFabric ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveFabric ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddTrustedRootCertificate => {
                let cmd_data : AddTrustedRootCertificateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_add_trusted_root_certificate (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddTrustedRootCertificate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetVIDVerificationStatement => {
                let cmd_data : SetVIDVerificationStatementRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_vid_verification_statement (& ctx , cmd_data . clone () ) ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetVIDVerificationStatement ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SignVIDVerificationRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SignVIDVerificationRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (14u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_sign_vid_verification_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SignVIDVerificationRequest ) ) ) , tw ) , tag , ) ? ) ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SignVIDVerificationRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 62u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OperationalCredentials" , 62u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAdaptor < T > , Q ) > where T : ClusterHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OperationalCredentials" , 62u32 , self . 0.2 ) 
    }
    
}
impl < T > crate :: dm :: NonBlockingHandler for HandlerAdaptor < T > where T : ClusterHandler , {
    
}
# [doc = "Cluster-scoped view onto a [`crate::im::CmdDataArrayBuilder`] for the `OperationalCredentials` cluster. Empty-request commands push and return `Self`; parameterized commands return the codegen-emitted typed request builder (whose parent chain bypasses the view — close back to the array via Data + CmdData `.end()?`s). `.end()` closes the wrapped array. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen — currently `end`, which the WebRTC cluster uses) get a `cmd_` prefix." ] pub struct OperationalCredentialsCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: CmdDataArrayBuilder < P > , 
}
impl < P > OperationalCredentialsCmdRequestsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn attestation_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < AttestationRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 0 ) ? . data_builder () 
    }
    pub fn certificate_chain_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < CertificateChainRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 2 ) ? . data_builder () 
    }
    pub fn csr_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < CSRRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 4 ) ? . data_builder () 
    }
    pub fn add_noc (self , endpoint : crate :: dm :: EndptId , ) -> Result < AddNOCRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 6 ) ? . data_builder () 
    }
    pub fn update_noc (self , endpoint : crate :: dm :: EndptId , ) -> Result < UpdateNOCRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 7 ) ? . data_builder () 
    }
    pub fn update_fabric_label (self , endpoint : crate :: dm :: EndptId , ) -> Result < UpdateFabricLabelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 9 ) ? . data_builder () 
    }
    pub fn remove_fabric (self , endpoint : crate :: dm :: EndptId , ) -> Result < RemoveFabricRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 10 ) ? . data_builder () 
    }
    pub fn add_trusted_root_certificate (self , endpoint : crate :: dm :: EndptId , ) -> Result < AddTrustedRootCertificateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 11 ) ? . data_builder () 
    }
    pub fn set_vid_verification_statement (self , endpoint : crate :: dm :: EndptId , ) -> Result < SetVIDVerificationStatementRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 12 ) ? . data_builder () 
    }
    pub fn sign_vid_verification_request (self , endpoint : crate :: dm :: EndptId , ) -> Result < SignVIDVerificationRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < P > , 2 , > , 0 , > , crate :: error :: Error , > {
        self . array . push () ? . path (endpoint , 62 , 13 ) ? . data_builder () 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `OperationalCredentials` cluster's commands. `use` this trait to call `.operational_credentials_inv()` on a [`crate::im::CmdDataArrayBuilder`]; the returned [`OperationalCredentialsCmdRequestsView`] exposes one method per command (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait OperationalCredentialsCmdRequests < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn operational_credentials_inv (self ) -> OperationalCredentialsCmdRequestsView < P > ;
    
}
impl < P > OperationalCredentialsCmdRequests < P > for crate :: im :: CmdDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn operational_credentials_inv (self ) -> OperationalCredentialsCmdRequestsView < P > {
        OperationalCredentialsCmdRequestsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrPathArrayBuilder`] for the `OperationalCredentials` cluster. Each method pushes one `AttrPath` (cluster ID baked in) and returns `Self` for chaining; `.end()` closes the underlying array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OperationalCredentialsAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrPathArrayBuilder < P > , 
}
impl < P > OperationalCredentialsAttrReadsView < P > where P : crate :: tlv :: TLVBuilderParent , {
    pub fn nocs (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (0 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn fabrics (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (1 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn supported_fabrics (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (2 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn commissioned_fabrics (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (3 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn trusted_root_certificates (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (4 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn current_fabric_index (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (5 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn generated_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (65528 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn accepted_command_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (65529 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn attribute_list (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (65531 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn feature_map (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (65532 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    pub fn cluster_revision (self , endpoint : crate :: dm :: EndptId , ) -> Result < Self , crate :: error :: Error > {
        let array = self . array . push () ? . endpoint (endpoint ) ? . cluster (62 ) ? . attr (65533 ) ? . end () ? ;
        Ok (Self {
            array 
        }
        ) 
    }
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `OperationalCredentials` cluster's attribute reads. `use` this trait to call `.operational_credentials_read()` on an [`crate::im::AttrPathArrayBuilder`]; the returned [`OperationalCredentialsAttrReadsView`] exposes one method per attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait OperationalCredentialsAttrReads < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn operational_credentials_read (self ) -> OperationalCredentialsAttrReadsView < P > ;
    
}
impl < P > OperationalCredentialsAttrReads < P > for crate :: im :: AttrPathArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn operational_credentials_read (self ) -> OperationalCredentialsAttrReadsView < P > {
        OperationalCredentialsAttrReadsView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped view onto an [`crate::im::AttrDataArrayBuilder`] for the `OperationalCredentials` cluster. Scalar-valued attrs push and return `Self` for chaining; struct/list-valued attrs return the codegen-emitted typed value builder (whose parent chain bypasses the view — close back to the array via Data + AttrData `.end()?`s). `.end()` closes the wrapped array. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OperationalCredentialsAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    array : crate :: im :: AttrDataArrayBuilder < P > , 
}
impl < P > OperationalCredentialsAttrWritesView < P > where P : crate :: tlv :: TLVBuilderParent , {
    # [doc = r" Close the wrapped array and return its parent." ] pub fn end (self ) -> Result < P , crate :: error :: Error > {
        self . array . end () 
    }
    
}
# [doc = "IM-client extension trait for the `OperationalCredentials` cluster's attribute writes. `use` this trait to call `.operational_credentials_write()` on an [`crate::im::AttrDataArrayBuilder`]; the returned [`OperationalCredentialsAttrWritesView`] exposes one method per writable attribute (cluster-prefix-free). `.end()` on the view closes the wrapped array." ] pub trait OperationalCredentialsAttrWrites < P > : Sized where P : crate :: tlv :: TLVBuilderParent , {
    fn operational_credentials_write (self ) -> OperationalCredentialsAttrWritesView < P > ;
    
}
impl < P > OperationalCredentialsAttrWrites < P > for crate :: im :: AttrDataArrayBuilder < P > where P : crate :: tlv :: TLVBuilderParent , {
    fn operational_credentials_write (self ) -> OperationalCredentialsAttrWritesView < P > {
        OperationalCredentialsAttrWritesView {
            array : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::InvokeResp`] for the `OperationalCredentials` cluster. Each method returns an iterator of `(EndptId, Result<R, Error>)` over the entries in `invoke_responses` whose path matches that command. Command names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get a `cmd_` prefix." ] pub struct OperationalCredentialsCmdResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: InvokeResp < 'a > , 
}
impl < 'a , 'r > OperationalCredentialsCmdResponsesView < 'a , 'r > {
    pub fn attestation_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < AttestationResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < AttestationResponse < 'a >> (62 , 0 ) 
    }
    pub fn certificate_chain_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < CertificateChainResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < CertificateChainResponse < 'a >> (62 , 2 ) 
    }
    pub fn csr_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < CSRResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < CSRResponse < 'a >> (62 , 4 ) 
    }
    pub fn add_noc (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NOCResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NOCResponse < 'a >> (62 , 6 ) 
    }
    pub fn update_noc (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NOCResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NOCResponse < 'a >> (62 , 7 ) 
    }
    pub fn update_fabric_label (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NOCResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NOCResponse < 'a >> (62 , 9 ) 
    }
    pub fn remove_fabric (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < NOCResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < NOCResponse < 'a >> (62 , 10 ) 
    }
    pub fn add_trusted_root_certificate (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (62 , 11 ) 
    }
    pub fn set_vid_verification_statement (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < () , crate :: error :: Error > , ) > + '_ {
        self . resp . statuses (62 , 12 ) 
    }
    pub fn sign_vid_verification_request (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < SignVIDVerificationResponse < 'a > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . responses :: < SignVIDVerificationResponse < 'a >> (62 , 13 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `OperationalCredentials`-cluster command responses out of a generic [`crate::im::InvokeResp`]. `use` this trait to call `.operational_credentials_inv_resp()` on an `InvokeResp`; the returned [`OperationalCredentialsCmdResponsesView`] exposes one iterator method per command, each yielding `(EndptId, Result<<Output>, Error>)` — see [`crate::im::InvokeResp::responses`] / [`crate::im::InvokeResp::statuses`] for the per-entry semantics." ] pub trait OperationalCredentialsCmdResponses < 'a > {
    fn operational_credentials_inv_resp (& self ) -> OperationalCredentialsCmdResponsesView < 'a , '_ > ;
    
}
impl < 'a > OperationalCredentialsCmdResponses < 'a > for crate :: im :: InvokeResp < 'a > {
    fn operational_credentials_inv_resp (& self ) -> OperationalCredentialsCmdResponsesView < 'a , '_ > {
        OperationalCredentialsCmdResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped response view onto a [`crate::im::ReportDataResp`] for the `OperationalCredentials` cluster. Each method returns an iterator of `(EndptId, Result<T, Error>)` over the entries in `attr_reports` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OperationalCredentialsAttrResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: ReportDataResp < 'a > , 
}
impl < 'a , 'r > OperationalCredentialsAttrResponsesView < 'a , 'r > {
    pub fn nocs (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , NOCStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , NOCStruct < 'a > > > (62 , 0 ) 
    }
    pub fn fabrics (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , FabricDescriptorStruct < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , FabricDescriptorStruct < 'a > > > (62 , 1 ) 
    }
    pub fn supported_fabrics (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (62 , 2 ) 
    }
    pub fn commissioned_fabrics (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (62 , 3 ) 
    }
    pub fn trusted_root_certificates (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: tlv :: OctetStr < 'a > > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: tlv :: OctetStr < 'a > > > (62 , 4 ) 
    }
    pub fn current_fabric_index (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u8 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u8 > (62 , 5 ) 
    }
    pub fn generated_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (62 , 65528 ) 
    }
    pub fn accepted_command_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: CmdId > > (62 , 65529 ) 
    }
    pub fn attribute_list (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < crate :: tlv :: TLVArray < 'a , crate :: im :: AttrId > > (62 , 65531 ) 
    }
    pub fn feature_map (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u32 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u32 > (62 , 65532 ) 
    }
    pub fn cluster_revision (& self , ) -> impl Iterator < Item = (crate :: dm :: EndptId , Result < u16 , crate :: error :: Error > , ) > + use < '_ , 'a , 'r > {
        self . resp . attrs :: < u16 > (62 , 65533 ) 
    }
    
}
# [doc = "IM-client extension trait for extracting `OperationalCredentials`-cluster attribute reports out of a generic [`crate::im::ReportDataResp`]. `use` this trait to call `.operational_credentials_read_resp()` on a `ReportDataResp`; the returned [`OperationalCredentialsAttrResponsesView`] exposes one iterator method per attribute (scalar, string, struct and list alike), each yielding `(EndptId, Result<T, Error>)`. The item type borrows the response, so string/struct/list attributes come back as `Utf8Str<'a>`/`OctetStr<'a>`, the codegen struct wrapper `Foo<'a>`, or `TLVArray<'a, _>` respectively." ] pub trait OperationalCredentialsAttrResponses < 'a > {
    fn operational_credentials_read_resp (& self ) -> OperationalCredentialsAttrResponsesView < 'a , '_ > ;
    
}
impl < 'a > OperationalCredentialsAttrResponses < 'a > for crate :: im :: ReportDataResp < 'a > {
    fn operational_credentials_read_resp (& self ) -> OperationalCredentialsAttrResponsesView < 'a , '_ > {
        OperationalCredentialsAttrResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Cluster-scoped write-status view onto a [`crate::im::WriteResp`] for the `OperationalCredentials` cluster. Each method returns an iterator of `(EndptId, Result<(), Error>)` over the entries in `write_responses` whose path matches that attribute. Attribute names that would collide with the view's own inherent methods (see `RESERVED_VIEW_METHOD_NAMES` in the codegen) get an `attr_` prefix." ] pub struct OperationalCredentialsWriteResponsesView < 'a , 'r > {
    resp : & 'r crate :: im :: WriteResp < 'a > , 
}
impl < 'a , 'r > OperationalCredentialsWriteResponsesView < 'a , 'r > {
    
}
# [doc = "IM-client extension trait for extracting `OperationalCredentials`-cluster per-attribute write statuses out of a generic [`crate::im::WriteResp`]. `use` this trait to call `.operational_credentials_write_resp()` on a `WriteResp`; the returned [`OperationalCredentialsWriteResponsesView`] exposes one iterator method per writable attribute, each yielding `(EndptId, Result<(), Error>)`." ] pub trait OperationalCredentialsWriteResponses < 'a > {
    fn operational_credentials_write_resp (& self ) -> OperationalCredentialsWriteResponsesView < 'a , '_ > ;
    
}
impl < 'a > OperationalCredentialsWriteResponses < 'a > for crate :: im :: WriteResp < 'a > {
    fn operational_credentials_write_resp (& self ) -> OperationalCredentialsWriteResponsesView < 'a , '_ > {
        OperationalCredentialsWriteResponsesView {
            resp : self 
        }
        
    }
    
}
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `AttestationResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`AttestationResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct AttestationResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > AttestationResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < AttestationResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `CertificateChainResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`CertificateChainResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct CertificateChainResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > CertificateChainResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < CertificateChainResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `CSRResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`CSRResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct CSRResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > CSRResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < CSRResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `NOCResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`NOCResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct NOCResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > NOCResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < NOCResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Single-shot handle wrapping the Invoke response chunk of a `SignVIDVerificationResponse` response. Holds the transaction's RX buffer alive; `response()` parses the embedded `CommandDataIB` into a borrowed [`SignVIDVerificationResponse`]. The trailing `StatusResponse(Success)` is sent when the caller `.complete().await?`s the handle.\n\nSingle-command invokes never chunk per Matter Core spec §10.7.10, so a single `response()` call is sufficient." ] pub struct SignVIDVerificationResponseHandle < C > {
    chunk : C , 
}
impl < C : crate :: im :: ImInvokeChunk > SignVIDVerificationResponseHandle < C > {
    # [doc = r" Borrowed access to the parsed response. The returned" ] # [doc = r" value points into the transaction's RX buffer, which" ] # [doc = r" stays valid until this handle is dropped or" ] # [doc = r" `.complete()`d." ] pub fn response (& self ) -> Result < SignVIDVerificationResponse < '_ > , crate :: error :: Error > {
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
# [doc = "Per-transaction view onto the `OperationalCredentials` cluster's client operations. Returned by [`OperationalCredentialsClient::operational_credentials`]. Each method consumes the view (and therefore the underlying [`crate::im::ImClient`])." ] pub struct OperationalCredentialsClientView < T > {
    tx : T , 
}
impl < T : crate :: im :: ImClient > OperationalCredentialsClientView < T > {
    pub async fn attestation_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < AttestationResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AttestationRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . attestation_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (AttestationResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn certificate_chain_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < CertificateChainResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (CertificateChainRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . certificate_chain_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (CertificateChainResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn csr_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < CSRResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (CSRRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . csr_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (CSRResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn add_noc < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NOCResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (AddNOCRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . add_noc (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NOCResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn update_noc < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NOCResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (UpdateNOCRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . update_noc (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NOCResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn update_fabric_label < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NOCResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (UpdateFabricLabelRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . update_fabric_label (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NOCResponseHandle {
            chunk 
        }
        ) 
    }
    pub async fn remove_fabric < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < NOCResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (RemoveFabricRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . remove_fabric (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (NOCResponseHandle {
            chunk 
        }
        ) 
    }
    pub fn add_trusted_root_certificate < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (AddTrustedRootCertificateRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . add_trusted_root_certificate (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub fn set_vid_verification_statement < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> where F : FnMut (SetVIDVerificationStatementRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        crate :: im :: invoke_cmd (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . set_vid_verification_statement (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) 
    }
    pub async fn sign_vid_verification_request < F > (self , endpoint : crate :: dm :: EndptId , mut request : F , ) -> Result < SignVIDVerificationResponseHandle < T :: InvokeChunk > , crate :: error :: Error > where F : FnMut (SignVIDVerificationRequestRequestBuilder < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , 0 , > , ) -> Result < crate :: im :: CmdDataBuilder < crate :: im :: CmdDataArrayBuilder < crate :: im :: InvReqBuilder < T :: InvokeSender , 3 > , > , 2 , > , crate :: error :: Error , > + crate :: utils :: sync :: MaybeSend , {
        use self :: OperationalCredentialsCmdRequests as _Cmds ;
        let chunk = crate :: im :: ImClient :: invoke_with (self . tx , None , move | msg | {
            let view = msg . invoke_requests () ? . operational_credentials_inv () ;
            let req_builder = view . sign_vid_verification_request (endpoint ) ? ;
            let cmd_data = request (req_builder ) ? ;
            cmd_data . end () ? . end () ? . end () 
        }
        ) . await ? ;
        Ok (SignVIDVerificationResponseHandle {
            chunk 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn nocs_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , NOCStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . nocs (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
                         attribute surfaces as `Err` to `f`." ] pub fn fabrics_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , FabricDescriptorStruct < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . fabrics (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn supported_fabrics_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . supported_fabrics (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn commissioned_fabrics_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . commissioned_fabrics (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn trusted_root_certificates_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: tlv :: OctetStr < '_ > > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . trusted_root_certificates (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn current_fabric_index_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . current_fabric_index (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    # [doc = "Read a non-scalar attribute (string, octet string, struct or list) \
                         whose value *borrows* the response buffer. Decodes the value and \
                         passes `Result<Value, Error>` to `f` while the borrow is live, \
                         returning the owned `R` that `f` produces (copy out what you need, \
                         e.g. `|v| v.map(|s| s.to_string())`). A non-served optional \
                         attribute surfaces as `Err` to `f`." ] pub fn generated_command_list_read_with < R > (self , endpoint : crate :: dm :: EndptId , f : impl FnOnce (Result < crate :: tlv :: TLVArray < '_ , crate :: im :: CmdId > , crate :: error :: Error > ) -> R , ) -> impl core :: future :: Future < Output = Result < R , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . generated_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . accepted_command_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
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
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr_with (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . attribute_list (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        , move | data | {
            f (data . and_then (| data | crate :: tlv :: FromTLV :: from_tlv (& data ) ) ) 
        }
        ) 
    }
    pub fn feature_map_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u32 , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . feature_map (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    pub fn cluster_revision_read (self , endpoint : crate :: dm :: EndptId , ) -> impl core :: future :: Future < Output = Result < u16 , crate :: error :: Error >> {
        use self :: OperationalCredentialsAttrReads as _Reads ;
        crate :: im :: read_attr (self . tx , move | msg | {
            msg . attr_requests () ? . operational_credentials_read () . cluster_revision (endpoint ) ? . end () ? . fabric_filtered (true ) ? . end () 
        }
        ) 
    }
    
}
# [doc = "Single-shot IM-client convenience trait for the `OperationalCredentials` cluster. `use` this trait to call `.operational_credentials()` on any [`crate::im::ImClient`], such as an exchange; the returned [`OperationalCredentialsClientView`] exposes one method per command and per attribute. Scalar attributes get a by-value `<attr>_read(endpoint)`; non-scalar ones (string/octet/struct/list, whose value borrows the response) get `<attr>_read_with(endpoint, |v| …)`, which hands the borrowed value to the closure and returns its owned result. The cluster ID, command/attribute ID, request opcode, retransmit loop, response-chunk iteration, and status-only handling are all baked in. DefaultSuccess commands return `Result<(), Error>` and drain the response internally; response-bearing commands return `Result<<RespStruct>Handle<_>, Error>` — the handle keeps the RX buffer alive so the caller can read the borrowed response via `.response()?` before `.complete().await?`ing the transaction.\n\nThe indirection through `OperationalCredentialsClientView` keeps each cluster's API surface narrow: at the call site `exchange.operational_credentials().` shows only this cluster's methods in IDE completion." ] pub trait OperationalCredentialsClient : crate :: im :: ImClient {
    # [doc = r" Enter this cluster's client view. Consumes the" ] # [doc = r" transaction — call methods on the returned view to drive" ] # [doc = r" a single IM transaction (one command invoke or one" ] # [doc = r" attribute read/write)." ] fn operational_credentials (self ) -> OperationalCredentialsClientView < Self > {
        OperationalCredentialsClientView {
            tx : self 
        }
        
    }
    
}
impl < T : crate :: im :: ImClient > OperationalCredentialsClient for T {
    
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
    fn nocs < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NOCStructArrayBuilder < P > , NOCStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn fabrics < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < FabricDescriptorStructArrayBuilder < P > , FabricDescriptorStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn supported_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn commissioned_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn trusted_root_certificates < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: OctetsArrayBuilder < P > , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn current_fabric_index (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_attestation_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AttestationRequestRequest < '_ > , response : AttestationResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_certificate_chain_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CertificateChainRequestRequest < '_ > , response : CertificateChainResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_csr_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CSRRequestRequest < '_ > , response : CSRResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_add_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_update_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_update_fabric_label < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateFabricLabelRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_remove_fabric < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveFabricRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_add_trusted_root_certificate (& self , ctx : impl crate :: dm :: InvokeContext , request : AddTrustedRootCertificateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_set_vid_verification_statement (& self , ctx : impl crate :: dm :: InvokeContext , request : SetVIDVerificationStatementRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    fn handle_sign_vid_verification_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SignVIDVerificationRequestRequest < '_ > , response : SignVIDVerificationResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error > > + crate :: utils :: sync :: MaybeSend ;
    
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
    fn nocs < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < NOCStructArrayBuilder < P > , NOCStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: nocs (self , ctx , builder ) 
    }
    fn fabrics < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < FabricDescriptorStructArrayBuilder < P > , FabricDescriptorStructBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: fabrics (self , ctx , builder ) 
    }
    fn supported_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: supported_fabrics (self , ctx ) 
    }
    fn commissioned_fabrics (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: commissioned_fabrics (self , ctx ) 
    }
    fn trusted_root_certificates < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: ReadContext , builder : crate :: dm :: ArrayAttributeRead < crate :: tlv :: OctetsArrayBuilder < P > , crate :: tlv :: OctetsBuilder < P > > ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: trusted_root_certificates (self , ctx , builder ) 
    }
    fn current_fabric_index (& self , ctx : impl crate :: dm :: ReadContext ) -> impl core :: future :: Future < Output = Result < u8 , crate :: error :: Error >> {
        T :: current_fabric_index (self , ctx ) 
    }
    fn handle_attestation_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AttestationRequestRequest < '_ > , response : AttestationResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_attestation_request (self , ctx , request , response ) 
    }
    fn handle_certificate_chain_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CertificateChainRequestRequest < '_ > , response : CertificateChainResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_certificate_chain_request (self , ctx , request , response ) 
    }
    fn handle_csr_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : CSRRequestRequest < '_ > , response : CSRResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_csr_request (self , ctx , request , response ) 
    }
    fn handle_add_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : AddNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_add_noc (self , ctx , request , response ) 
    }
    fn handle_update_noc < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateNOCRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_update_noc (self , ctx , request , response ) 
    }
    fn handle_update_fabric_label < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : UpdateFabricLabelRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_update_fabric_label (self , ctx , request , response ) 
    }
    fn handle_remove_fabric < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : RemoveFabricRequest < '_ > , response : NOCResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_remove_fabric (self , ctx , request , response ) 
    }
    fn handle_add_trusted_root_certificate (& self , ctx : impl crate :: dm :: InvokeContext , request : AddTrustedRootCertificateRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_add_trusted_root_certificate (self , ctx , request ) 
    }
    fn handle_set_vid_verification_statement (& self , ctx : impl crate :: dm :: InvokeContext , request : SetVIDVerificationStatementRequest < '_ > , ) -> impl core :: future :: Future < Output = Result < () , crate :: error :: Error >> {
        T :: handle_set_vid_verification_statement (self , ctx , request ) 
    }
    fn handle_sign_vid_verification_request < P : crate :: tlv :: TLVBuilderParent > (& self , ctx : impl crate :: dm :: InvokeContext , request : SignVIDVerificationRequestRequest < '_ > , response : SignVIDVerificationResponseBuilder < P > , ) -> impl core :: future :: Future < Output = Result < P , crate :: error :: Error >> {
        T :: handle_sign_vid_verification_request (self , ctx , request , response ) 
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
                    AttributeId :: NOCs => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NOCs , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. nocs (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NOCs , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: NOCs , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: Fabrics => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Fabrics , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. fabrics (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Fabrics , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: Fabrics , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: SupportedFabrics => {
                        let attr_read_result = self . 0. supported_fabrics (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: SupportedFabrics , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: CommissionedFabrics => {
                        let attr_read_result = self . 0. commissioned_fabrics (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CommissionedFabrics , false ) ) ) ) , & attr_read_result ) ;
                        crate :: dm :: Reply :: set (writer , attr_read_result ? ) 
                    }
                    AttributeId :: TrustedRootCertificates => {
                        crate :: dm :: log_build_start (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TrustedRootCertificates , false ) ) ) ) ) ;
                        let tag = crate :: dm :: Reply :: tag (& writer ) ;
                        let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                        let attr_read_result = self . 0. trusted_root_certificates (& ctx , crate :: dm :: ArrayAttributeRead :: new (ctx . attr () . list_index . clone () , crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TrustedRootCertificates , false ) ) ) ) , tw ) , tag , ) ? , ) . await ;
                        crate :: dm :: log_build_end (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: TrustedRootCertificates , false ) ) ) ) , & attr_read_result . as_ref () . map (| _ | () ) ) ;
                        attr_read_result ? ;
                        crate :: dm :: Reply :: complete (writer ) 
                    }
                    AttributeId :: CurrentFabricIndex => {
                        let attr_read_result = self . 0. current_fabric_index (& ctx ) . await ;
                        crate :: dm :: log_result (& MetadataDebug ((ctx . attr () . endpoint_id , self , MetadataDebug ((AttributeId :: CurrentFabricIndex , false ) ) ) ) , & attr_read_result ) ;
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
        match CommandId :: try_from (ctx . cmd () . cmd_id ) ? {
            CommandId :: AttestationRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AttestationRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (1u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_attestation_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AttestationRequest ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AttestationRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: CertificateChainRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CertificateChainRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (3u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_certificate_chain_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CertificateChainRequest ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CertificateChainRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: CSRRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CSRRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (5u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_csr_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CSRRequest ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: CSRRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddNOC => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddNOC ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_add_noc (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddNOC ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddNOC ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: UpdateNOC => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateNOC ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_update_noc (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateNOC ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateNOC ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: UpdateFabricLabel => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateFabricLabel ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_update_fabric_label (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateFabricLabel ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: UpdateFabricLabel ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: RemoveFabric => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveFabric ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (8u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_remove_fabric (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveFabric ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: RemoveFabric ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
                cmd_invoke_result ? ;
                crate :: dm :: Reply :: complete (writer ) ? 
            }
            CommandId :: AddTrustedRootCertificate => {
                let cmd_data : AddTrustedRootCertificateRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_add_trusted_root_certificate (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: AddTrustedRootCertificate ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SetVIDVerificationStatement => {
                let cmd_data : SetVIDVerificationStatementRequest < '_ > = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                let cmd_invoke_result = self . 0. handle_set_vid_verification_statement (& ctx , cmd_data . clone () ) . await ;
                crate :: dm :: log_data_result (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SetVIDVerificationStatement ) ) ) , & cmd_data , & cmd_invoke_result ) ;
                cmd_invoke_result ? ;
                
            }
            CommandId :: SignVIDVerificationRequest => {
                let cmd_data = crate :: tlv :: FromTLV :: from_tlv (ctx . data () ) ? ;
                crate :: dm :: log_data_build_start (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SignVIDVerificationRequest ) ) ) , & cmd_data ) ;
                let mut writer = reply . with_command (14u32 ) ? ;
                let tag = crate :: dm :: Reply :: tag (& writer ) ;
                let tw = crate :: dm :: Reply :: writer (& mut writer ) ;
                let cmd_invoke_result = self . 0. handle_sign_vid_verification_request (& ctx , cmd_data , crate :: tlv :: TLVBuilder :: new (crate :: tlv :: TLVWriteParent :: new (MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SignVIDVerificationRequest ) ) ) , tw ) , tag , ) ? ) . await ;
                crate :: dm :: log_build_end (& MetadataDebug ((ctx . cmd () . endpoint_id , self , MetadataDebug (CommandId :: SignVIDVerificationRequest ) ) ) , & cmd_invoke_result . as_ref () . map (| _ | () ) ) ;
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
        if ctx . cluster () . map (| c | c == 62u32 ) . unwrap_or (true ) {
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
        write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OperationalCredentials" , 62u32 , self . 0.2 ) 
    }
    
}
# [cfg (feature = "defmt" ) ] impl < T , Q > crate :: reexport :: defmt :: Format for MetadataDebug < (u16 , & HandlerAsyncAdaptor < T > , Q ) > where T : ClusterAsyncHandler , Q : crate :: reexport :: defmt :: Format , {
    # [allow (unreachable_code ) ] fn format (& self , f : crate :: reexport :: defmt :: Formatter < '_ > ) {
        crate :: reexport :: defmt :: write ! (f , "Endpt(0x{:02x})::Cluster::{}(0x{:04x})::{:?}" , self . 0.0 , "OperationalCredentials" , 62u32 , self . 0.2 ) 
    }
    
}
